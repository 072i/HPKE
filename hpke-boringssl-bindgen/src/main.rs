use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use syn::{parse_quote, Expr, File, FnArg, ForeignItemFn, Pat, Stmt, Type, Visibility};

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
        let param_patterns: Vec<_> = orig_params
            .windows(2)
            .filter_map(filter_slice)
            .chain(orig_params.windows(3).filter_map(filter_slice_mut))
            .collect();
        let (new_params, callee_args, let_stmts, set_stmts) =
            make_args(orig_params, param_patterns);
        let new_name = format_ident!("rust_{}", orig_name);
        let new_output = i.sig.output.clone();
        let tokens = quote! {
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
fn make_args(
    orig_params: Vec<&FnArg>,
    param_patterns: Vec<Vec<&FnArg>>,
) -> (
    Punctuated<FnArg, Comma>,
    Punctuated<Expr, Comma>,
    Vec<Stmt>,
    Vec<Stmt>,
) {
    let mut counter = 0usize;
    orig_params.into_iter().fold(
        (Punctuated::new(), Punctuated::new(), Vec::new(), Vec::new()),
        |(mut arg_list, mut expr_list, mut let_list, mut set_list), orig_param| {
            match param_patterns.iter().find_map(|pattern| {
                pattern
                    .iter()
                    .position(|&param| param == orig_param)
                    .map(|pos| (pos, pattern.len(), pattern[0], pattern[0].ident()))
            }) {
                Some((0, 2, p0, i)) => {
                    let mut new_param = p0.clone();
                    new_param.set_ty(parse_quote!(&impl crate::FfiSlice));
                    arg_list.push(new_param);

                    expr_list.push(parse_quote!(#i.as_ptr()));
                }
                Some((1, 2, _, i)) => expr_list.push(parse_quote!(#i.len())),
                Some((0, 3, p0, i)) => {
                    let mut new_param = p0.clone();
                    new_param.set_ty(parse_quote!(&mut impl crate::FfiSliceMut));
                    arg_list.push(new_param);

                    expr_list.push(parse_quote!(#i.as_mut_ptr()));
                }
                Some((1, 3, _, i)) => {
                    counter += 1;
                    let c = format_ident!("__len{}", counter);
                    let_list.push(parse_quote!(let mut #c = 0;));
                    expr_list.push(parse_quote!(&mut #c));
                    set_list.push(parse_quote!(#i.set_len(#c);));
                }
                Some((2, 3, _, i)) => expr_list.push(parse_quote!(#i.capacity())),
                None => {
                    arg_list.push(orig_param.clone());
                    let i = orig_param.ident();
                    expr_list.push(parse_quote!(#i));
                }
                _ => panic!(),
            }
            (arg_list, expr_list, let_list, set_list)
        },
    )
}
fn filter_slice<'a>(params: &[&'a FnArg]) -> Option<Vec<&'a FnArg>> {
    let [p1, p2] = params else { return None };
    if p2.name().ends_with("_len")
        && *p1.ty() == parse_quote!(*const u8)
        && *p2.ty() == parse_quote!(usize)
    {
        Some(Vec::from(params))
    } else {
        None
    }
}
fn filter_slice_mut<'a>(params: &[&'a FnArg]) -> Option<Vec<&'a FnArg>> {
    let [p1, p2, p3] = params else { return None };
    if p2.name().ends_with("_len")
        && p3.name().starts_with("max_")
        && *p1.ty() == parse_quote!(*mut u8)
        && *p2.ty() == parse_quote!(*mut usize)
        && *p3.ty() == parse_quote!(usize)
    {
        Some(Vec::from(params))
    } else {
        None
    }
}
trait FnArgExt {
    fn ident(&self) -> &Ident;
    fn ty(&self) -> &Type;
    fn set_ty(&mut self, ty: Type);
    fn name(&self) -> String;
}
impl FnArgExt for FnArg {
    fn ident(&self) -> &Ident {
        if let FnArg::Typed(v) = self {
            if let Pat::Ident(v) = &*v.pat {
                &v.ident
            } else {
                panic!()
            }
        } else {
            panic!()
        }
    }
    fn ty(&self) -> &Type {
        if let FnArg::Typed(v) = self {
            &v.ty
        } else {
            panic!()
        }
    }
    fn set_ty(&mut self, ty: Type) {
        if let FnArg::Typed(v) = self {
            *v.ty = ty
        } else {
            panic!()
        }
    }
    fn name(&self) -> String {
        self.ident().to_string()
    }
}
