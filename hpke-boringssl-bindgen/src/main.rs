use heck::ToSnakeCase;
use proc_macro2::Ident;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use syn::{Expr, File, FnArg, ForeignItemFn, Pat, Stmt, Type, Visibility};

fn main() {
    let mut buffer = Vec::new();
    bindgen::builder()
        .clang_arg("-Iboringssl/include")
        .header("boringssl/include/openssl/hpke.h")
        .allowlist_type("EVP_HPKE_.+")
        .allowlist_function("EVP_HPKE_.+")
        .allowlist_function("EVP_hpke_.+")
        .merge_extern_blocks(true)
        .generate()
        .unwrap()
        .write(Box::new(&mut buffer))
        .unwrap();
    let mut file = syn::parse_file(std::str::from_utf8(&buffer).unwrap()).unwrap();
    VA.visit_file_mut(&mut file);
    let mut vb = VB { files: Vec::new() };
    vb.visit_file(&file);
    let mut items = Vec::new();
    items.extend(file.items);
    vb.files
        .into_iter()
        .for_each(|file| items.extend(file.items));
    let file = File {
        items,
        ..syn::parse_file("#![allow(clippy::all)]\n#![allow(non_camel_case_types)]\n").unwrap()
    };
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    let path = "hpke-boringssl/src/bindings_macos_aarch64.rs";
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    let path = "hpke-boringssl/src/bindings_linux_x86_64.rs";
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    let path = "hpke-boringssl/src/bindings_windows_x86_64.rs";
    std::fs::write(
        path,
        [
            format!(
                "/* {} */\n",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64()
            ),
            prettyplease::unparse(&file),
        ]
        .concat(),
    )
    .unwrap();
}
struct VA;
impl VisitMut for VA {
    fn visit_foreign_item_fn_mut(&mut self, i: &mut ForeignItemFn) {
        i.vis = Visibility::Inherited
    }
}
struct VB {
    files: Vec<File>,
}
impl<'a> Visit<'a> for VB {
    fn visit_foreign_item_fn(&mut self, i: &'a ForeignItemFn) {
        let orig_params: Vec<_> = i.sig.inputs.iter().collect();
        let orig_name = i.sig.ident.clone();
        let slice_params = orig_params.windows(2).filter_map(filter_slice);
        let slice_mut_params = orig_params.windows(3).filter_map(filter_slice_mut);
        let all_params: Vec<_> = slice_params.chain(slice_mut_params).collect();
        let new_params = get_new_params(&orig_params, &all_params);
        let (callee_args, let_stmts, set_stmts) = get_callee_args(&orig_params, &all_params);
        let new_name = get_new_name(&orig_name);
        let new_output = i.sig.output.clone();
        let tokens = quote::quote! {
            pub unsafe fn #new_name (#new_params) #new_output {
                unsafe {
                    #(#let_stmts)*
                    let __out = #orig_name (#callee_args);
                    #(#set_stmts)*
                    __out
                }
            }
        };
        self.files.push(syn::parse2(tokens).unwrap());
    }
}
fn get_new_name(orig_name: &Ident) -> Ident {
    str2ast(&format!("rust_{}", orig_name).to_snake_case())
}
fn get_callee_args(
    orig_params: &[&FnArg],
    all_params: &[Vec<&FnArg>],
) -> (Punctuated<Expr, Comma>, Vec<Stmt>, Vec<Stmt>) {
    let mut out = Punctuated::new();
    let mut counter = 0;
    let mut let_stmts = Vec::new();
    let mut set_stmts = Vec::new();
    orig_params.iter().copied().for_each(|param| {
        match all_params.iter().find(|params| params.contains(&param)) {
            Some(params) => {
                let param0 = arg2str(params[0]);
                out.push(match params.len() {
                    2 if params[0] == param => str2ast(&format!("{}.as_ptr()", param0)),
                    2 if params[1] == param => str2ast(&format!("{}.len()", param0)),
                    3 if params[0] == param => str2ast(&format!("{}.as_mut_ptr()", param0)),
                    3 if params[1] == param => {
                        counter += 1;
                        let_stmts.push(str2ast(&format!("let mut __len{} = 0;", counter)));
                        set_stmts.push(str2ast(&format!("{}.set_len(__len{});", param0, counter)));
                        str2ast(&format!("&mut __len{}", counter))
                    }
                    3 if params[2] == param => str2ast(&format!("{}.capacity()", param0)),
                    _ => panic!(),
                })
            }
            None => out.push(str2ast(&arg2str(param))),
        }
    });
    (out, let_stmts, set_stmts)
}
fn get_new_params(orig_params: &[&FnArg], all_params: &[Vec<&FnArg>]) -> Punctuated<FnArg, Comma> {
    let mut out = Punctuated::new();
    orig_params.iter().copied().for_each(|param| {
        match all_params.iter().find(|params| params.contains(&param)) {
            Some(params) if params[0] == param => {
                let mut new_param = params[0].clone();
                match (params.len(), &mut new_param) {
                    (2, FnArg::Typed(pat)) => {
                        pat.ty = Box::new(str2ast("&impl crate::FfiSlice"));
                    }
                    (3, FnArg::Typed(pat)) => {
                        pat.ty = Box::new(str2ast("&mut impl crate::FfiSliceMut"));
                    }
                    _ => panic!(),
                }
                out.push(new_param);
            }
            None => out.push(param.clone()),
            _ => (),
        }
    });
    out
}
fn filter_slice<'a>(params: &[&'a FnArg]) -> Option<Vec<&'a FnArg>> {
    let [FnArg::Typed(a), FnArg::Typed(b)] = params else {
        return None;
    };
    let (Pat::Ident(ia), Pat::Ident(ib)) = (a.pat.as_ref(), b.pat.as_ref()) else {
        return None;
    };
    let (Type::Ptr(pa), Type::Path(pb)) = (a.ty.as_ref(), b.ty.as_ref()) else {
        return None;
    };
    let (Type::Path(pa), Some(_)) = (pa.elem.as_ref(), pa.const_token) else {
        return None;
    };
    let (na, nb) = (ia.ident.to_string(), ib.ident.to_string());
    if [na.trim_end_matches("_"), "_len"].concat() == nb
        && pa.path.is_ident("u8")
        && pb.path.is_ident("usize")
    {
        return Some(Vec::from(params));
    }
    None
}
fn filter_slice_mut<'a>(params: &[&'a FnArg]) -> Option<Vec<&'a FnArg>> {
    let [FnArg::Typed(a), FnArg::Typed(b), FnArg::Typed(c)] = params else {
        return None;
    };
    let (Pat::Ident(_), Pat::Ident(ib), Pat::Ident(ic)) =
        (a.pat.as_ref(), b.pat.as_ref(), c.pat.as_ref())
    else {
        return None;
    };
    let (Type::Ptr(pa), Type::Ptr(pb), Type::Path(pc)) =
        (a.ty.as_ref(), b.ty.as_ref(), c.ty.as_ref())
    else {
        return None;
    };
    let (Type::Path(pa), Type::Path(pb), Some(_), Some(_)) = (
        pa.elem.as_ref(),
        pb.elem.as_ref(),
        pa.mutability,
        pb.mutability,
    ) else {
        return None;
    };
    let (nb, nc) = (ib.ident.to_string(), ic.ident.to_string());
    if nb.contains("len")
        && nc.contains("max")
        && pa.path.is_ident("u8")
        && pb.path.is_ident("usize")
        && pc.path.is_ident("usize")
    {
        return Some(Vec::from(params));
    }
    None
}
fn str2ast<T: Parse>(s: &str) -> T {
    syn::parse_str(s).unwrap()
}
fn arg2str(arg: &FnArg) -> String {
    if let FnArg::Typed(p) = arg {
        if let Pat::Ident(i) = p.pat.as_ref() {
            return i.ident.to_string();
        }
    }
    panic!()
}
