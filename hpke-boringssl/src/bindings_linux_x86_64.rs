/* 1742349759.7570436 */
#![allow(clippy::all)]
#![allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct env_md_st {
    _unused: [u8; 0],
}
pub type EVP_MD = env_md_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct evp_aead_st {
    _unused: [u8; 0],
}
pub type EVP_AEAD = evp_aead_st;
pub type EVP_AEAD_CTX = evp_aead_ctx_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct evp_hpke_aead_st {
    _unused: [u8; 0],
}
pub type EVP_HPKE_AEAD = evp_hpke_aead_st;
pub type EVP_HPKE_CTX = evp_hpke_ctx_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct evp_hpke_kdf_st {
    _unused: [u8; 0],
}
pub type EVP_HPKE_KDF = evp_hpke_kdf_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct evp_hpke_kem_st {
    _unused: [u8; 0],
}
pub type EVP_HPKE_KEM = evp_hpke_kem_st;
pub type EVP_HPKE_KEY = evp_hpke_key_st;
#[repr(C)]
#[derive(Copy, Clone)]
pub union evp_aead_ctx_st_state {
    pub opaque: [u8; 564usize],
    pub alignment: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of evp_aead_ctx_st_state",
    ][::std::mem::size_of::<evp_aead_ctx_st_state>() - 568usize];
    [
        "Alignment of evp_aead_ctx_st_state",
    ][::std::mem::align_of::<evp_aead_ctx_st_state>() - 8usize];
    [
        "Offset of field: evp_aead_ctx_st_state::opaque",
    ][::std::mem::offset_of!(evp_aead_ctx_st_state, opaque) - 0usize];
    [
        "Offset of field: evp_aead_ctx_st_state::alignment",
    ][::std::mem::offset_of!(evp_aead_ctx_st_state, alignment) - 0usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evp_aead_ctx_st {
    pub aead: *const EVP_AEAD,
    pub state: evp_aead_ctx_st_state,
    pub tag_len: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of evp_aead_ctx_st"][::std::mem::size_of::<evp_aead_ctx_st>() - 584usize];
    ["Alignment of evp_aead_ctx_st"][::std::mem::align_of::<evp_aead_ctx_st>() - 8usize];
    [
        "Offset of field: evp_aead_ctx_st::aead",
    ][::std::mem::offset_of!(evp_aead_ctx_st, aead) - 0usize];
    [
        "Offset of field: evp_aead_ctx_st::state",
    ][::std::mem::offset_of!(evp_aead_ctx_st, state) - 8usize];
    [
        "Offset of field: evp_aead_ctx_st::tag_len",
    ][::std::mem::offset_of!(evp_aead_ctx_st, tag_len) - 576usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evp_hpke_ctx_st {
    pub kem: *const EVP_HPKE_KEM,
    pub aead: *const EVP_HPKE_AEAD,
    pub kdf: *const EVP_HPKE_KDF,
    pub aead_ctx: EVP_AEAD_CTX,
    pub base_nonce: [u8; 24usize],
    pub exporter_secret: [u8; 64usize],
    pub seq: u64,
    pub is_sender: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of evp_hpke_ctx_st"][::std::mem::size_of::<evp_hpke_ctx_st>() - 712usize];
    ["Alignment of evp_hpke_ctx_st"][::std::mem::align_of::<evp_hpke_ctx_st>() - 8usize];
    [
        "Offset of field: evp_hpke_ctx_st::kem",
    ][::std::mem::offset_of!(evp_hpke_ctx_st, kem) - 0usize];
    [
        "Offset of field: evp_hpke_ctx_st::aead",
    ][::std::mem::offset_of!(evp_hpke_ctx_st, aead) - 8usize];
    [
        "Offset of field: evp_hpke_ctx_st::kdf",
    ][::std::mem::offset_of!(evp_hpke_ctx_st, kdf) - 16usize];
    [
        "Offset of field: evp_hpke_ctx_st::aead_ctx",
    ][::std::mem::offset_of!(evp_hpke_ctx_st, aead_ctx) - 24usize];
    [
        "Offset of field: evp_hpke_ctx_st::base_nonce",
    ][::std::mem::offset_of!(evp_hpke_ctx_st, base_nonce) - 608usize];
    [
        "Offset of field: evp_hpke_ctx_st::exporter_secret",
    ][::std::mem::offset_of!(evp_hpke_ctx_st, exporter_secret) - 632usize];
    [
        "Offset of field: evp_hpke_ctx_st::seq",
    ][::std::mem::offset_of!(evp_hpke_ctx_st, seq) - 696usize];
    [
        "Offset of field: evp_hpke_ctx_st::is_sender",
    ][::std::mem::offset_of!(evp_hpke_ctx_st, is_sender) - 704usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct evp_hpke_key_st {
    pub kem: *const EVP_HPKE_KEM,
    pub private_key: [u8; 32usize],
    pub public_key: [u8; 65usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of evp_hpke_key_st"][::std::mem::size_of::<evp_hpke_key_st>() - 112usize];
    ["Alignment of evp_hpke_key_st"][::std::mem::align_of::<evp_hpke_key_st>() - 8usize];
    [
        "Offset of field: evp_hpke_key_st::kem",
    ][::std::mem::offset_of!(evp_hpke_key_st, kem) - 0usize];
    [
        "Offset of field: evp_hpke_key_st::private_key",
    ][::std::mem::offset_of!(evp_hpke_key_st, private_key) - 8usize];
    [
        "Offset of field: evp_hpke_key_st::public_key",
    ][::std::mem::offset_of!(evp_hpke_key_st, public_key) - 40usize];
};
unsafe extern "C" {
    fn EVP_hpke_x25519_hkdf_sha256() -> *const EVP_HPKE_KEM;
    fn EVP_hpke_p256_hkdf_sha256() -> *const EVP_HPKE_KEM;
    fn EVP_HPKE_KEM_id(kem: *const EVP_HPKE_KEM) -> u16;
    fn EVP_HPKE_KEM_public_key_len(kem: *const EVP_HPKE_KEM) -> usize;
    fn EVP_HPKE_KEM_private_key_len(kem: *const EVP_HPKE_KEM) -> usize;
    fn EVP_HPKE_KEM_enc_len(kem: *const EVP_HPKE_KEM) -> usize;
    fn EVP_hpke_hkdf_sha256() -> *const EVP_HPKE_KDF;
    fn EVP_HPKE_KDF_id(kdf: *const EVP_HPKE_KDF) -> u16;
    fn EVP_HPKE_KDF_hkdf_md(kdf: *const EVP_HPKE_KDF) -> *const EVP_MD;
    fn EVP_hpke_aes_128_gcm() -> *const EVP_HPKE_AEAD;
    fn EVP_hpke_aes_256_gcm() -> *const EVP_HPKE_AEAD;
    fn EVP_hpke_chacha20_poly1305() -> *const EVP_HPKE_AEAD;
    fn EVP_HPKE_AEAD_id(aead: *const EVP_HPKE_AEAD) -> u16;
    fn EVP_HPKE_AEAD_aead(aead: *const EVP_HPKE_AEAD) -> *const EVP_AEAD;
    fn EVP_HPKE_KEY_zero(key: *mut EVP_HPKE_KEY);
    fn EVP_HPKE_KEY_cleanup(key: *mut EVP_HPKE_KEY);
    fn EVP_HPKE_KEY_new() -> *mut EVP_HPKE_KEY;
    fn EVP_HPKE_KEY_free(key: *mut EVP_HPKE_KEY);
    fn EVP_HPKE_KEY_copy(
        dst: *mut EVP_HPKE_KEY,
        src: *const EVP_HPKE_KEY,
    ) -> ::std::os::raw::c_int;
    fn EVP_HPKE_KEY_move(out: *mut EVP_HPKE_KEY, in_: *mut EVP_HPKE_KEY);
    fn EVP_HPKE_KEY_init(
        key: *mut EVP_HPKE_KEY,
        kem: *const EVP_HPKE_KEM,
        priv_key: *const u8,
        priv_key_len: usize,
    ) -> ::std::os::raw::c_int;
    fn EVP_HPKE_KEY_generate(
        key: *mut EVP_HPKE_KEY,
        kem: *const EVP_HPKE_KEM,
    ) -> ::std::os::raw::c_int;
    fn EVP_HPKE_KEY_kem(key: *const EVP_HPKE_KEY) -> *const EVP_HPKE_KEM;
    fn EVP_HPKE_KEY_public_key(
        key: *const EVP_HPKE_KEY,
        out: *mut u8,
        out_len: *mut usize,
        max_out: usize,
    ) -> ::std::os::raw::c_int;
    fn EVP_HPKE_KEY_private_key(
        key: *const EVP_HPKE_KEY,
        out: *mut u8,
        out_len: *mut usize,
        max_out: usize,
    ) -> ::std::os::raw::c_int;
    fn EVP_HPKE_CTX_zero(ctx: *mut EVP_HPKE_CTX);
    fn EVP_HPKE_CTX_cleanup(ctx: *mut EVP_HPKE_CTX);
    fn EVP_HPKE_CTX_new() -> *mut EVP_HPKE_CTX;
    fn EVP_HPKE_CTX_free(ctx: *mut EVP_HPKE_CTX);
    fn EVP_HPKE_CTX_setup_sender(
        ctx: *mut EVP_HPKE_CTX,
        out_enc: *mut u8,
        out_enc_len: *mut usize,
        max_enc: usize,
        kem: *const EVP_HPKE_KEM,
        kdf: *const EVP_HPKE_KDF,
        aead: *const EVP_HPKE_AEAD,
        peer_public_key: *const u8,
        peer_public_key_len: usize,
        info: *const u8,
        info_len: usize,
    ) -> ::std::os::raw::c_int;
    fn EVP_HPKE_CTX_setup_sender_with_seed_for_testing(
        ctx: *mut EVP_HPKE_CTX,
        out_enc: *mut u8,
        out_enc_len: *mut usize,
        max_enc: usize,
        kem: *const EVP_HPKE_KEM,
        kdf: *const EVP_HPKE_KDF,
        aead: *const EVP_HPKE_AEAD,
        peer_public_key: *const u8,
        peer_public_key_len: usize,
        info: *const u8,
        info_len: usize,
        seed: *const u8,
        seed_len: usize,
    ) -> ::std::os::raw::c_int;
    fn EVP_HPKE_CTX_setup_recipient(
        ctx: *mut EVP_HPKE_CTX,
        key: *const EVP_HPKE_KEY,
        kdf: *const EVP_HPKE_KDF,
        aead: *const EVP_HPKE_AEAD,
        enc: *const u8,
        enc_len: usize,
        info: *const u8,
        info_len: usize,
    ) -> ::std::os::raw::c_int;
    fn EVP_HPKE_CTX_setup_auth_sender(
        ctx: *mut EVP_HPKE_CTX,
        out_enc: *mut u8,
        out_enc_len: *mut usize,
        max_enc: usize,
        key: *const EVP_HPKE_KEY,
        kdf: *const EVP_HPKE_KDF,
        aead: *const EVP_HPKE_AEAD,
        peer_public_key: *const u8,
        peer_public_key_len: usize,
        info: *const u8,
        info_len: usize,
    ) -> ::std::os::raw::c_int;
    fn EVP_HPKE_CTX_setup_auth_sender_with_seed_for_testing(
        ctx: *mut EVP_HPKE_CTX,
        out_enc: *mut u8,
        out_enc_len: *mut usize,
        max_enc: usize,
        key: *const EVP_HPKE_KEY,
        kdf: *const EVP_HPKE_KDF,
        aead: *const EVP_HPKE_AEAD,
        peer_public_key: *const u8,
        peer_public_key_len: usize,
        info: *const u8,
        info_len: usize,
        seed: *const u8,
        seed_len: usize,
    ) -> ::std::os::raw::c_int;
    fn EVP_HPKE_CTX_setup_auth_recipient(
        ctx: *mut EVP_HPKE_CTX,
        key: *const EVP_HPKE_KEY,
        kdf: *const EVP_HPKE_KDF,
        aead: *const EVP_HPKE_AEAD,
        enc: *const u8,
        enc_len: usize,
        info: *const u8,
        info_len: usize,
        peer_public_key: *const u8,
        peer_public_key_len: usize,
    ) -> ::std::os::raw::c_int;
    fn EVP_HPKE_CTX_open(
        ctx: *mut EVP_HPKE_CTX,
        out: *mut u8,
        out_len: *mut usize,
        max_out_len: usize,
        in_: *const u8,
        in_len: usize,
        ad: *const u8,
        ad_len: usize,
    ) -> ::std::os::raw::c_int;
    fn EVP_HPKE_CTX_seal(
        ctx: *mut EVP_HPKE_CTX,
        out: *mut u8,
        out_len: *mut usize,
        max_out_len: usize,
        in_: *const u8,
        in_len: usize,
        ad: *const u8,
        ad_len: usize,
    ) -> ::std::os::raw::c_int;
    fn EVP_HPKE_CTX_export(
        ctx: *const EVP_HPKE_CTX,
        out: *mut u8,
        secret_len: usize,
        context: *const u8,
        context_len: usize,
    ) -> ::std::os::raw::c_int;
    fn EVP_HPKE_CTX_max_overhead(ctx: *const EVP_HPKE_CTX) -> usize;
    fn EVP_HPKE_CTX_kem(ctx: *const EVP_HPKE_CTX) -> *const EVP_HPKE_KEM;
    fn EVP_HPKE_CTX_aead(ctx: *const EVP_HPKE_CTX) -> *const EVP_HPKE_AEAD;
    fn EVP_HPKE_CTX_kdf(ctx: *const EVP_HPKE_CTX) -> *const EVP_HPKE_KDF;
}
pub unsafe fn rust_evp_hpke_x25519_hkdf_sha256() -> *const EVP_HPKE_KEM {
    unsafe {
        let __out = EVP_hpke_x25519_hkdf_sha256();
        __out
    }
}
pub unsafe fn rust_evp_hpke_p256_hkdf_sha256() -> *const EVP_HPKE_KEM {
    unsafe {
        let __out = EVP_hpke_p256_hkdf_sha256();
        __out
    }
}
pub unsafe fn rust_evp_hpke_kem_id(kem: *const EVP_HPKE_KEM) -> u16 {
    unsafe {
        let __out = EVP_HPKE_KEM_id(kem);
        __out
    }
}
pub unsafe fn rust_evp_hpke_kem_public_key_len(kem: *const EVP_HPKE_KEM) -> usize {
    unsafe {
        let __out = EVP_HPKE_KEM_public_key_len(kem);
        __out
    }
}
pub unsafe fn rust_evp_hpke_kem_private_key_len(kem: *const EVP_HPKE_KEM) -> usize {
    unsafe {
        let __out = EVP_HPKE_KEM_private_key_len(kem);
        __out
    }
}
pub unsafe fn rust_evp_hpke_kem_enc_len(kem: *const EVP_HPKE_KEM) -> usize {
    unsafe {
        let __out = EVP_HPKE_KEM_enc_len(kem);
        __out
    }
}
pub unsafe fn rust_evp_hpke_hkdf_sha256() -> *const EVP_HPKE_KDF {
    unsafe {
        let __out = EVP_hpke_hkdf_sha256();
        __out
    }
}
pub unsafe fn rust_evp_hpke_kdf_id(kdf: *const EVP_HPKE_KDF) -> u16 {
    unsafe {
        let __out = EVP_HPKE_KDF_id(kdf);
        __out
    }
}
pub unsafe fn rust_evp_hpke_kdf_hkdf_md(kdf: *const EVP_HPKE_KDF) -> *const EVP_MD {
    unsafe {
        let __out = EVP_HPKE_KDF_hkdf_md(kdf);
        __out
    }
}
pub unsafe fn rust_evp_hpke_aes_128_gcm() -> *const EVP_HPKE_AEAD {
    unsafe {
        let __out = EVP_hpke_aes_128_gcm();
        __out
    }
}
pub unsafe fn rust_evp_hpke_aes_256_gcm() -> *const EVP_HPKE_AEAD {
    unsafe {
        let __out = EVP_hpke_aes_256_gcm();
        __out
    }
}
pub unsafe fn rust_evp_hpke_chacha20_poly1305() -> *const EVP_HPKE_AEAD {
    unsafe {
        let __out = EVP_hpke_chacha20_poly1305();
        __out
    }
}
pub unsafe fn rust_evp_hpke_aead_id(aead: *const EVP_HPKE_AEAD) -> u16 {
    unsafe {
        let __out = EVP_HPKE_AEAD_id(aead);
        __out
    }
}
pub unsafe fn rust_evp_hpke_aead_aead(aead: *const EVP_HPKE_AEAD) -> *const EVP_AEAD {
    unsafe {
        let __out = EVP_HPKE_AEAD_aead(aead);
        __out
    }
}
pub unsafe fn rust_evp_hpke_key_zero(key: *mut EVP_HPKE_KEY) {
    unsafe {
        let __out = EVP_HPKE_KEY_zero(key);
        __out
    }
}
pub unsafe fn rust_evp_hpke_key_cleanup(key: *mut EVP_HPKE_KEY) {
    unsafe {
        let __out = EVP_HPKE_KEY_cleanup(key);
        __out
    }
}
pub unsafe fn rust_evp_hpke_key_new() -> *mut EVP_HPKE_KEY {
    unsafe {
        let __out = EVP_HPKE_KEY_new();
        __out
    }
}
pub unsafe fn rust_evp_hpke_key_free(key: *mut EVP_HPKE_KEY) {
    unsafe {
        let __out = EVP_HPKE_KEY_free(key);
        __out
    }
}
pub unsafe fn rust_evp_hpke_key_copy(
    dst: *mut EVP_HPKE_KEY,
    src: *const EVP_HPKE_KEY,
) -> ::std::os::raw::c_int {
    unsafe {
        let __out = EVP_HPKE_KEY_copy(dst, src);
        __out
    }
}
pub unsafe fn rust_evp_hpke_key_move(out: *mut EVP_HPKE_KEY, in_: *mut EVP_HPKE_KEY) {
    unsafe {
        let __out = EVP_HPKE_KEY_move(out, in_);
        __out
    }
}
pub unsafe fn rust_evp_hpke_key_init(
    key: *mut EVP_HPKE_KEY,
    kem: *const EVP_HPKE_KEM,
    priv_key: &impl crate::FfiSlice,
) -> ::std::os::raw::c_int {
    unsafe {
        let __out = EVP_HPKE_KEY_init(key, kem, priv_key.as_ptr(), priv_key.len());
        __out
    }
}
pub unsafe fn rust_evp_hpke_key_generate(
    key: *mut EVP_HPKE_KEY,
    kem: *const EVP_HPKE_KEM,
) -> ::std::os::raw::c_int {
    unsafe {
        let __out = EVP_HPKE_KEY_generate(key, kem);
        __out
    }
}
pub unsafe fn rust_evp_hpke_key_kem(key: *const EVP_HPKE_KEY) -> *const EVP_HPKE_KEM {
    unsafe {
        let __out = EVP_HPKE_KEY_kem(key);
        __out
    }
}
pub unsafe fn rust_evp_hpke_key_public_key(
    key: *const EVP_HPKE_KEY,
    out: &mut impl crate::FfiSliceMut,
) -> ::std::os::raw::c_int {
    unsafe {
        let mut __len1 = 0;
        let __out = EVP_HPKE_KEY_public_key(
            key,
            out.as_mut_ptr(),
            &mut __len1,
            out.capacity(),
        );
        out.set_len(__len1);
        __out
    }
}
pub unsafe fn rust_evp_hpke_key_private_key(
    key: *const EVP_HPKE_KEY,
    out: &mut impl crate::FfiSliceMut,
) -> ::std::os::raw::c_int {
    unsafe {
        let mut __len1 = 0;
        let __out = EVP_HPKE_KEY_private_key(
            key,
            out.as_mut_ptr(),
            &mut __len1,
            out.capacity(),
        );
        out.set_len(__len1);
        __out
    }
}
pub unsafe fn rust_evp_hpke_ctx_zero(ctx: *mut EVP_HPKE_CTX) {
    unsafe {
        let __out = EVP_HPKE_CTX_zero(ctx);
        __out
    }
}
pub unsafe fn rust_evp_hpke_ctx_cleanup(ctx: *mut EVP_HPKE_CTX) {
    unsafe {
        let __out = EVP_HPKE_CTX_cleanup(ctx);
        __out
    }
}
pub unsafe fn rust_evp_hpke_ctx_new() -> *mut EVP_HPKE_CTX {
    unsafe {
        let __out = EVP_HPKE_CTX_new();
        __out
    }
}
pub unsafe fn rust_evp_hpke_ctx_free(ctx: *mut EVP_HPKE_CTX) {
    unsafe {
        let __out = EVP_HPKE_CTX_free(ctx);
        __out
    }
}
pub unsafe fn rust_evp_hpke_ctx_setup_sender(
    ctx: *mut EVP_HPKE_CTX,
    out_enc: &mut impl crate::FfiSliceMut,
    kem: *const EVP_HPKE_KEM,
    kdf: *const EVP_HPKE_KDF,
    aead: *const EVP_HPKE_AEAD,
    peer_public_key: &impl crate::FfiSlice,
    info: &impl crate::FfiSlice,
) -> ::std::os::raw::c_int {
    unsafe {
        let mut __len1 = 0;
        let __out = EVP_HPKE_CTX_setup_sender(
            ctx,
            out_enc.as_mut_ptr(),
            &mut __len1,
            out_enc.capacity(),
            kem,
            kdf,
            aead,
            peer_public_key.as_ptr(),
            peer_public_key.len(),
            info.as_ptr(),
            info.len(),
        );
        out_enc.set_len(__len1);
        __out
    }
}
pub unsafe fn rust_evp_hpke_ctx_setup_sender_with_seed_for_testing(
    ctx: *mut EVP_HPKE_CTX,
    out_enc: &mut impl crate::FfiSliceMut,
    kem: *const EVP_HPKE_KEM,
    kdf: *const EVP_HPKE_KDF,
    aead: *const EVP_HPKE_AEAD,
    peer_public_key: &impl crate::FfiSlice,
    info: &impl crate::FfiSlice,
    seed: &impl crate::FfiSlice,
) -> ::std::os::raw::c_int {
    unsafe {
        let mut __len1 = 0;
        let __out = EVP_HPKE_CTX_setup_sender_with_seed_for_testing(
            ctx,
            out_enc.as_mut_ptr(),
            &mut __len1,
            out_enc.capacity(),
            kem,
            kdf,
            aead,
            peer_public_key.as_ptr(),
            peer_public_key.len(),
            info.as_ptr(),
            info.len(),
            seed.as_ptr(),
            seed.len(),
        );
        out_enc.set_len(__len1);
        __out
    }
}
pub unsafe fn rust_evp_hpke_ctx_setup_recipient(
    ctx: *mut EVP_HPKE_CTX,
    key: *const EVP_HPKE_KEY,
    kdf: *const EVP_HPKE_KDF,
    aead: *const EVP_HPKE_AEAD,
    enc: &impl crate::FfiSlice,
    info: &impl crate::FfiSlice,
) -> ::std::os::raw::c_int {
    unsafe {
        let __out = EVP_HPKE_CTX_setup_recipient(
            ctx,
            key,
            kdf,
            aead,
            enc.as_ptr(),
            enc.len(),
            info.as_ptr(),
            info.len(),
        );
        __out
    }
}
pub unsafe fn rust_evp_hpke_ctx_setup_auth_sender(
    ctx: *mut EVP_HPKE_CTX,
    out_enc: &mut impl crate::FfiSliceMut,
    key: *const EVP_HPKE_KEY,
    kdf: *const EVP_HPKE_KDF,
    aead: *const EVP_HPKE_AEAD,
    peer_public_key: &impl crate::FfiSlice,
    info: &impl crate::FfiSlice,
) -> ::std::os::raw::c_int {
    unsafe {
        let mut __len1 = 0;
        let __out = EVP_HPKE_CTX_setup_auth_sender(
            ctx,
            out_enc.as_mut_ptr(),
            &mut __len1,
            out_enc.capacity(),
            key,
            kdf,
            aead,
            peer_public_key.as_ptr(),
            peer_public_key.len(),
            info.as_ptr(),
            info.len(),
        );
        out_enc.set_len(__len1);
        __out
    }
}
pub unsafe fn rust_evp_hpke_ctx_setup_auth_sender_with_seed_for_testing(
    ctx: *mut EVP_HPKE_CTX,
    out_enc: &mut impl crate::FfiSliceMut,
    key: *const EVP_HPKE_KEY,
    kdf: *const EVP_HPKE_KDF,
    aead: *const EVP_HPKE_AEAD,
    peer_public_key: &impl crate::FfiSlice,
    info: &impl crate::FfiSlice,
    seed: &impl crate::FfiSlice,
) -> ::std::os::raw::c_int {
    unsafe {
        let mut __len1 = 0;
        let __out = EVP_HPKE_CTX_setup_auth_sender_with_seed_for_testing(
            ctx,
            out_enc.as_mut_ptr(),
            &mut __len1,
            out_enc.capacity(),
            key,
            kdf,
            aead,
            peer_public_key.as_ptr(),
            peer_public_key.len(),
            info.as_ptr(),
            info.len(),
            seed.as_ptr(),
            seed.len(),
        );
        out_enc.set_len(__len1);
        __out
    }
}
pub unsafe fn rust_evp_hpke_ctx_setup_auth_recipient(
    ctx: *mut EVP_HPKE_CTX,
    key: *const EVP_HPKE_KEY,
    kdf: *const EVP_HPKE_KDF,
    aead: *const EVP_HPKE_AEAD,
    enc: &impl crate::FfiSlice,
    info: &impl crate::FfiSlice,
    peer_public_key: &impl crate::FfiSlice,
) -> ::std::os::raw::c_int {
    unsafe {
        let __out = EVP_HPKE_CTX_setup_auth_recipient(
            ctx,
            key,
            kdf,
            aead,
            enc.as_ptr(),
            enc.len(),
            info.as_ptr(),
            info.len(),
            peer_public_key.as_ptr(),
            peer_public_key.len(),
        );
        __out
    }
}
pub unsafe fn rust_evp_hpke_ctx_open(
    ctx: *mut EVP_HPKE_CTX,
    out: &mut impl crate::FfiSliceMut,
    in_: &impl crate::FfiSlice,
    ad: &impl crate::FfiSlice,
) -> ::std::os::raw::c_int {
    unsafe {
        let mut __len1 = 0;
        let __out = EVP_HPKE_CTX_open(
            ctx,
            out.as_mut_ptr(),
            &mut __len1,
            out.capacity(),
            in_.as_ptr(),
            in_.len(),
            ad.as_ptr(),
            ad.len(),
        );
        out.set_len(__len1);
        __out
    }
}
pub unsafe fn rust_evp_hpke_ctx_seal(
    ctx: *mut EVP_HPKE_CTX,
    out: &mut impl crate::FfiSliceMut,
    in_: &impl crate::FfiSlice,
    ad: &impl crate::FfiSlice,
) -> ::std::os::raw::c_int {
    unsafe {
        let mut __len1 = 0;
        let __out = EVP_HPKE_CTX_seal(
            ctx,
            out.as_mut_ptr(),
            &mut __len1,
            out.capacity(),
            in_.as_ptr(),
            in_.len(),
            ad.as_ptr(),
            ad.len(),
        );
        out.set_len(__len1);
        __out
    }
}
pub unsafe fn rust_evp_hpke_ctx_export(
    ctx: *const EVP_HPKE_CTX,
    out: *mut u8,
    secret_len: usize,
    context: &impl crate::FfiSlice,
) -> ::std::os::raw::c_int {
    unsafe {
        let __out = EVP_HPKE_CTX_export(
            ctx,
            out,
            secret_len,
            context.as_ptr(),
            context.len(),
        );
        __out
    }
}
pub unsafe fn rust_evp_hpke_ctx_max_overhead(ctx: *const EVP_HPKE_CTX) -> usize {
    unsafe {
        let __out = EVP_HPKE_CTX_max_overhead(ctx);
        __out
    }
}
pub unsafe fn rust_evp_hpke_ctx_kem(ctx: *const EVP_HPKE_CTX) -> *const EVP_HPKE_KEM {
    unsafe {
        let __out = EVP_HPKE_CTX_kem(ctx);
        __out
    }
}
pub unsafe fn rust_evp_hpke_ctx_aead(ctx: *const EVP_HPKE_CTX) -> *const EVP_HPKE_AEAD {
    unsafe {
        let __out = EVP_HPKE_CTX_aead(ctx);
        __out
    }
}
pub unsafe fn rust_evp_hpke_ctx_kdf(ctx: *const EVP_HPKE_CTX) -> *const EVP_HPKE_KDF {
    unsafe {
        let __out = EVP_HPKE_CTX_kdf(ctx);
        __out
    }
}
