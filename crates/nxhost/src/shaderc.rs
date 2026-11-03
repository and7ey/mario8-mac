// Mush8 - shaderc.rs
// On-device GLSL -> SPIR-V through libshaderc's C API.
use anyhow::{Result, anyhow, bail};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::sync::OnceLock;

#[repr(C)]
pub struct Compiler {
    _p: [u8; 0],
}
#[repr(C)]
pub struct CompileOptions {
    _p: [u8; 0],
}
#[repr(C)]
pub struct CompilationResult {
    _p: [u8; 0],
}

pub const VERTEX: i32 = 0;
pub const FRAGMENT: i32 = 1;
pub const COMPUTE: i32 = 2;

const ENV_VULKAN: i32 = 0;
const ENV_VERSION_VULKAN_1_1: u32 = (1 << 22) | (1 << 12);

unsafe extern "C" {
    fn shaderc_compiler_initialize() -> *mut Compiler;
    fn shaderc_compiler_release(compiler: *mut Compiler);
    fn shaderc_compile_options_initialize() -> *mut CompileOptions;
    fn shaderc_compile_options_release(options: *mut CompileOptions);
    fn shaderc_compile_options_set_target_env(options: *mut CompileOptions, target: c_int, version: u32);
    fn shaderc_compile_into_spv(
        compiler: *const Compiler,
        source: *const c_char,
        source_len: usize,
        kind: c_int,
        file: *const c_char,
        entry: *const c_char,
        options: *const CompileOptions,
    ) -> *mut CompilationResult;
    fn shaderc_result_get_compilation_status(result: *const CompilationResult) -> c_int;
    fn shaderc_result_get_error_message(result: *const CompilationResult) -> *const c_char;
    fn shaderc_result_get_bytes(result: *const CompilationResult) -> *const c_char;
    fn shaderc_result_get_length(result: *const CompilationResult) -> usize;
    fn shaderc_result_release(result: *mut CompilationResult);
}

struct Handle<T>(*mut T);
unsafe impl<T> Send for Handle<T> {}
unsafe impl<T> Sync for Handle<T> {}

fn compiler() -> &'static Handle<Compiler> {
    static C: OnceLock<Handle<Compiler>> = OnceLock::new();
    C.get_or_init(|| Handle(unsafe { shaderc_compiler_initialize() }))
}

fn options() -> &'static Handle<CompileOptions> {
    static O: OnceLock<Handle<CompileOptions>> = OnceLock::new();
    O.get_or_init(|| unsafe {
        let o = shaderc_compile_options_initialize();
        shaderc_compile_options_set_target_env(o, ENV_VULKAN, ENV_VERSION_VULKAN_1_1);
        Handle(o)
    })
}

pub fn compile(kind: i32, source: &str, name: &str) -> Result<Vec<u32>> {
    let compiler = compiler().0;
    let options = options().0;
    if compiler.is_null() || options.is_null() {
        bail!("shaderc initialization failed");
    }
    let src = CString::new(source).map_err(|_| anyhow!("GLSL source contains a NUL byte"))?;
    let file = CString::new(name).map_err(|_| anyhow!("shader name contains a NUL byte"))?;
    let result = unsafe {
        shaderc_compile_into_spv(
            compiler,
            src.as_ptr(),
            source.len(),
            kind,
            file.as_ptr(),
            c"main".as_ptr(),
            options,
        )
    };
    if result.is_null() {
        bail!("shaderc: out of memory");
    }
    let status = unsafe { shaderc_result_get_compilation_status(result) };
    let out = if status == 0 {
        let len = unsafe { shaderc_result_get_length(result) };
        let ptr = unsafe { shaderc_result_get_bytes(result) } as *const u8;
        if len % 4 != 0 || ptr.is_null() {
            Err(anyhow!("shaderc: bad SPIR-V output"))
        } else {
            let mut words = vec![0u32; len / 4];
            unsafe { std::ptr::copy_nonoverlapping(ptr, words.as_mut_ptr() as *mut u8, len) };
            Ok(words)
        }
    } else {
        let msg = unsafe { CStr::from_ptr(shaderc_result_get_error_message(result)) };
        Err(anyhow!("{}", msg.to_string_lossy()))
    };
    unsafe { shaderc_result_release(result) };
    out
}
