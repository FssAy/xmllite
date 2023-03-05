//! # Spoofed xmllite library
//!
//! Boilerplate prepared by [FssAy](https://github.com/DmitrijVC)

#[cfg(not(target_os = "windows"))]
compile_error!("This crate requires Windows target to compile");

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::os::raw::c_void;
use std::ptr;
use memexec::memexec_dll;
use memexec::peloader::def::DLL_PROCESS_ATTACH;
use winapi::ctypes::c_void as void;
use winapi::shared::guiddef::REFIID;
use winapi::shared::minwindef::*;
use winapi::um::objidlbase::{IMalloc, ISequentialStream};
use winapi::um::processthreadsapi::*;
use winapi::um::unknwnbase::IUnknown;
use winapi::um::winnt::{HRESULT, WCHAR};

/// Alias for an undocumented type that hold no meaning.
type UnknownType = c_void;

#[cfg(not(target_arch = "x86_64"))]
const IMAGE: &'static [u8] = include_bytes!("../bin/xmllite_32.dll");

#[cfg(target_arch = "x86_64")]
const IMAGE: &'static [u8] = include_bytes!("../bin/xmllite_64.dll");

lazy_static! {
    /// Loades the in-memory DLL and returns all the exported function addresses
    pub static ref EXPORTS: HashMap<String, usize> = unsafe {
        memexec_dll(IMAGE, 0 as _, DLL_PROCESS_ATTACH, 0 as _).unwrap()
    };
}

/// Macro for easier calling of the exported function from the original library.
macro_rules! originate {
    ($name:literal, $F:ty, $( $arg:ident ),* ) => {
        {
            let addr = EXPORTS.get($name).unwrap();
            let ptr = *addr as *const c_void;
            let f: $F = unsafe { std::mem::transmute(ptr) };
            let result = f(
                $($arg,)*
            );
            result
        }
    };
}

/// Your code goes here.
/// This function is called in a new thread on an every load.
unsafe extern "system" fn main(_lp_param: LPVOID) -> DWORD {
    // uncomment to allocate a console to the process
    /* winapi::um::consoleapi::AllocConsole(); */

    // remove the runtime if you are not planning on using async code
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async move {
        // async code
    });

    // Do not return "259" as it can put the application into an infinite loop.
    // read more at:
    // https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodethread?redirectedfrom=MSDN
    0
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn DllMain(hinst_dll: HINSTANCE, fdw_reason: DWORD, _lpv_reserved: LPVOID) -> BOOL {
    match fdw_reason {
        DLL_PROCESS_ATTACH => unsafe {
            CreateThread(
                ptr::null_mut(),
                0,
                Some(main),
                hinst_dll as LPVOID,
                0,
                ptr::null_mut(),
            );

            let _ = EXPORTS.len();
        },
        _ => {}
    }

    TRUE
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CreateXmlReader(
    riid: REFIID,
    ppv_object: *mut *mut void,
    p_malloc: *mut IMalloc,
) -> HRESULT {
    originate!(
        "CreateXmlReader",
        extern "C" fn(REFIID, *mut *mut void, *mut IMalloc) -> HRESULT,
        riid,
        ppv_object,
        p_malloc
    )
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CreateXmlReaderInputWithEncodingCodePage(
    p_input_stream: *mut IUnknown,
    p_malloc: *mut IMalloc,
    n_encoding_code_page: UINT,
    f_encoding_hint: BOOL,
    pwsz_base_uri: *const WCHAR,
    pp_input: *mut *mut UnknownType,
) -> HRESULT {
    originate!(
        "CreateXmlReaderInputWithEncodingCodePage",
        extern "C" fn(
            *mut IUnknown,
            *mut IMalloc,
            UINT,
            BOOL,
            *const WCHAR,
            *mut *mut UnknownType,
        ) -> HRESULT,
        p_input_stream,
        p_malloc,
        n_encoding_code_page,
        f_encoding_hint,
        pwsz_base_uri,
        pp_input
    )
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CreateXmlReaderInputWithEncodingName(
    p_input_stream: *mut IUnknown,
    p_malloc: *mut IMalloc,
    pwsz_encoding_name: *const WCHAR,
    f_encoding_hint: BOOL,
    pwsz_base_uri: *const WCHAR,
    pp_input: *mut *mut UnknownType,
) -> HRESULT {
    originate!(
        "CreateXmlReaderInputWithEncodingName",
        extern "C" fn(
            *mut IUnknown,
            *mut IMalloc,
            *const WCHAR,
            BOOL,
            *const WCHAR,
            *mut *mut UnknownType,
        ) -> HRESULT,
        p_input_stream,
        p_malloc,
        pwsz_encoding_name,
        f_encoding_hint,
        pwsz_base_uri,
        pp_input
    )
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CreateXmlWriter(
    riid: REFIID,
    ppv_object: *mut *mut void,
    p_malloc: *mut IMalloc,
) -> HRESULT {
    originate!(
        "CreateXmlWriter",
        extern "C" fn(REFIID, *mut *mut void, *mut IMalloc) -> HRESULT,
        riid,
        ppv_object,
        p_malloc
    )
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CreateXmlWriterOutputWithEncodingCodePage(
    p_stream: *mut ISequentialStream,
    p_malloc: *mut IMalloc,
    n_encoding_code_page: UINT,
    pp_output: *mut *mut UnknownType,
) -> HRESULT {
    originate!(
        "CreateXmlWriterOutputWithEncodingCodePage",
        extern "C" fn(*mut ISequentialStream, *mut IMalloc, UINT, *mut *mut UnknownType) -> HRESULT,
        p_stream,
        p_malloc,
        n_encoding_code_page,
        pp_output
    )
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CreateXmlWriterOutputWithEncodingName(
    p_stream: *mut ISequentialStream,
    p_malloc: *mut IMalloc,
    pwsz_encoding_name: *const WCHAR,
    pp_output: *mut *mut UnknownType,
) -> HRESULT {
    originate!(
        "CreateXmlWriterOutputWithEncodingName",
        extern "C" fn(
            *mut ISequentialStream,
            *mut IMalloc,
            *const WCHAR,
            *mut *mut UnknownType,
        ) -> HRESULT,
        p_stream,
        p_malloc,
        pwsz_encoding_name,
        pp_output
    )
}
