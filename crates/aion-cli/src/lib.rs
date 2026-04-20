use libloading::{Library, Symbol};
use std::env;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::PathBuf;

type RunExecuteFn = unsafe extern "C" fn(*const c_char) -> *const c_char;
type RunDiffFn = unsafe extern "C" fn(*const c_char, *const c_char) -> *const c_char;
type RunStoreFn = unsafe extern "C" fn(*const c_char, *const c_char) -> *const c_char;
type RunLoadFn = unsafe extern "C" fn(*const c_char) -> *const c_char;

pub struct KernelApi {
    _lib: Library,
    run_execute: RunExecuteFn,
    run_diff: RunDiffFn,
    run_store: RunStoreFn,
    run_load: RunLoadFn,
}

impl KernelApi {
    pub fn load() -> Result<Self, String> {
        let path = kernel_path();
        // SAFETY: loading dynamic library from explicit path.
        let lib = unsafe { Library::new(&path) }.map_err(|_| {
            "AION Kernel not found. Install aion-kernel or set AION_KERNEL_PATH.".to_string()
        })?;

        // SAFETY: symbol names and signatures are part of the fixed C-ABI contract.
        let run_execute: RunExecuteFn = unsafe { *load_symbol(&lib, b"run_execute\0")? };
        // SAFETY: symbol names and signatures are part of the fixed C-ABI contract.
        let run_diff: RunDiffFn = unsafe { *load_symbol(&lib, b"run_diff\0")? };
        // SAFETY: symbol names and signatures are part of the fixed C-ABI contract.
        let run_store: RunStoreFn = unsafe { *load_symbol(&lib, b"run_store\0")? };
        // SAFETY: symbol names and signatures are part of the fixed C-ABI contract.
        let run_load: RunLoadFn = unsafe { *load_symbol(&lib, b"run_load\0")? };

        Ok(Self {
            _lib: lib,
            run_execute,
            run_diff,
            run_store,
            run_load,
        })
    }

    pub fn run_execute(&self, spec_json: &str) -> Result<String, String> {
        let c_spec = CString::new(spec_json).map_err(|e| e.to_string())?;
        // SAFETY: pointer remains valid for call duration; callee returns C string pointer.
        let ptr = unsafe { (self.run_execute)(c_spec.as_ptr()) };
        read_c_result(ptr)
    }

    pub fn run_diff(&self, a_json: &str, b_json: &str) -> Result<String, String> {
        let c_a = CString::new(a_json).map_err(|e| e.to_string())?;
        let c_b = CString::new(b_json).map_err(|e| e.to_string())?;
        // SAFETY: pointers remain valid for call duration; callee returns C string pointer.
        let ptr = unsafe { (self.run_diff)(c_a.as_ptr(), c_b.as_ptr()) };
        read_c_result(ptr)
    }

    pub fn run_store(&self, path: &str, artifact_json: &str) -> Result<String, String> {
        let c_path = CString::new(path).map_err(|e| e.to_string())?;
        let c_artifact = CString::new(artifact_json).map_err(|e| e.to_string())?;
        // SAFETY: pointers remain valid for call duration; callee returns C string pointer.
        let ptr = unsafe { (self.run_store)(c_path.as_ptr(), c_artifact.as_ptr()) };
        read_c_result(ptr)
    }

    pub fn run_load(&self, path: &str) -> Result<String, String> {
        let c_path = CString::new(path).map_err(|e| e.to_string())?;
        // SAFETY: pointer remains valid for call duration; callee returns C string pointer.
        let ptr = unsafe { (self.run_load)(c_path.as_ptr()) };
        read_c_result(ptr)
    }
}

unsafe fn load_symbol<'a, T>(lib: &'a Library, name: &[u8]) -> Result<Symbol<'a, T>, String> {
    // SAFETY: caller ensures the requested symbol type matches ABI contract.
    unsafe { lib.get::<T>(name) }.map_err(|_| {
        "AION Kernel not found. Install aion-kernel or set AION_KERNEL_PATH.".to_string()
    })
}

fn read_c_result(ptr: *const c_char) -> Result<String, String> {
    if ptr.is_null() {
        return Err("AION Kernel returned null response".to_string());
    }
    // SAFETY: kernel ABI guarantees a valid, NUL-terminated UTF-8 string.
    let text = unsafe { CStr::from_ptr(ptr) }
        .to_str()
        .map_err(|e| e.to_string())?
        .to_string();
    if text.starts_with("error:") {
        return Err(text);
    }
    Ok(text)
}

fn kernel_path() -> PathBuf {
    if let Ok(path) = env::var("AION_KERNEL_PATH") {
        return PathBuf::from(path);
    }
    if cfg!(target_os = "windows") {
        PathBuf::from("aion-kernel.dll")
    } else if cfg!(target_os = "macos") {
        PathBuf::from("libaion-kernel.dylib")
    } else {
        PathBuf::from("libaion-kernel.so")
    }
}
