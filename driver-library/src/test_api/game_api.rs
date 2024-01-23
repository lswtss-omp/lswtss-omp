use crate::*;

pub unsafe fn is_game_api_ready() -> bool
{
    return RAW_AS_C_SCRIPT_ENGINE_HANDLE_OPTION.is_some()
        && RAW_AS_I_SCRIPT_CONTEXT_HANDLE_OPTION.is_some()
        && RAW_NTT_CURRENT_UNIVERSE_HANDLE_OPTION.is_some();
}
