#![feature(pointer_byte_offsets)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::needless_return)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::ffi::c_char;
use std::ffi::c_void;
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr::null_mut;

use winapi::shared::minwindef::BOOL;
use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef::HINSTANCE;
use winapi::shared::minwindef::LPVOID;
use winapi::shared::minwindef::TRUE;
use winapi::um::libloaderapi::GetProcAddress;
use winapi::um::libloaderapi::LoadLibraryA;
use winapi::um::sysinfoapi::GetSystemDirectoryA;
use winapi::um::winnt::DLL_PROCESS_ATTACH;

type DirectInput8Create = unsafe extern "system" fn(
    hinst: HINSTANCE,
    dw_version: DWORD,
    riidltf: *mut c_void,
    ppv_out: *mut *mut c_void,
    punk_outer: *mut c_void,
) -> BOOL;

static mut DIRECT_INPUT_8_CREATE_PTR: DirectInput8Create = DirectInput8Create;

static mut DINPUT8_LIBRARY_HANDLE: HINSTANCE = null_mut();

#[no_mangle]
pub unsafe extern "system" fn DllMain(
    _hinst_dll: HINSTANCE,
    fdw_reason: DWORD,
    _lpv_reserved: LPVOID,
) -> BOOL
{
    if fdw_reason == DLL_PROCESS_ATTACH {
        let mut dinput8_library_dir_path_as_c_char_array: [c_char; 256] = [0; 256];

        GetSystemDirectoryA(
            dinput8_library_dir_path_as_c_char_array.as_mut_ptr(),
            256,
        );

        let dinput8_library_dir_path_as_c_str =
            CStr::from_ptr(dinput8_library_dir_path_as_c_char_array.as_mut_ptr());

        let dinput8_library_path_as_c_string = CString::new(
            format!(
                "{}\\{}",
                dinput8_library_dir_path_as_c_str
                    .to_str()
                    .unwrap(),
                "dinput8.dll"
            ),
        )
        .unwrap();

        DINPUT8_LIBRARY_HANDLE = LoadLibraryA(dinput8_library_path_as_c_string.as_ptr());

        let direct_input_8_create_name_as_c_string = CString::new("DirectInput8Create").unwrap();

        DIRECT_INPUT_8_CREATE_PTR = std::mem::transmute(
            GetProcAddress(
                DINPUT8_LIBRARY_HANDLE,
                direct_input_8_create_name_as_c_string.as_ptr(),
            ),
        );

        let driver_library_name_as_c_string = CString::new("lswtss-omp-driver.dll").unwrap();

        LoadLibraryA(driver_library_name_as_c_string.as_ptr());
    }

    return TRUE;
}

#[no_mangle]
unsafe extern "system" fn DirectInput8Create(
    hinst: HINSTANCE,
    dw_version: DWORD,
    riidltf: *mut c_void,
    ppv_out: *mut *mut c_void,
    punk_outer: *mut c_void,
) -> BOOL
{
    return DIRECT_INPUT_8_CREATE_PTR(
        hinst, dw_version, riidltf, ppv_out, punk_outer,
    );
}
