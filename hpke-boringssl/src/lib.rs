#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
mod bindings_linux_x86_64;
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
mod bindings_macos_aarch64;
#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
mod bindings_windows_x86_64;
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub use bindings_linux_x86_64::*;
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub use bindings_macos_aarch64::*;
#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
pub use bindings_windows_x86_64::*;
#[allow(clippy::len_without_is_empty)]
pub trait FfiSlice {
    fn as_ptr(&self) -> *const u8;
    fn len(&self) -> usize;
}
pub trait FfiSliceMut {
    fn as_mut_ptr(&mut self) -> *mut u8;
    fn capacity(&self) -> usize;
    /// # Safety
    unsafe fn set_len(&mut self, new_len: usize);
}
impl FfiSlice for &[u8] {
    fn as_ptr(&self) -> *const u8 {
        if <Self as FfiSlice>::len(self) == 0 {
            std::ptr::null()
        } else {
            <[u8]>::as_ptr(self)
        }
    }
    fn len(&self) -> usize {
        <[u8]>::len(self)
    }
}
impl FfiSliceMut for Vec<u8> {
    fn as_mut_ptr(&mut self) -> *mut u8 {
        if <Self as FfiSliceMut>::capacity(self) == 0 {
            std::ptr::null_mut()
        } else {
            Vec::as_mut_ptr(self)
        }
    }
    fn capacity(&self) -> usize {
        Vec::capacity(self)
    }
    unsafe fn set_len(&mut self, new_len: usize) {
        unsafe { Vec::set_len(self, new_len) }
    }
}
