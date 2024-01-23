use winapi::shared::minwindef::BOOL;
use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef::HINSTANCE;
use winapi::shared::minwindef::LPVOID;
use winapi::shared::minwindef::TRUE;
use winapi::um::winnt::DLL_PROCESS_ATTACH;

use crate::*;

#[no_mangle]
pub unsafe extern "system" fn DllMain(
    _hinst_dll: HINSTANCE,
    fdw_reason: DWORD,
    _lpv_reserved: LPVOID,
) -> BOOL
{
    if fdw_reason == DLL_PROCESS_ATTACH {
        init_driver();
    }

    return TRUE;
}
