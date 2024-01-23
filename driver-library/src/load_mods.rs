use crate::*;

pub fn load_mods()
{
    for mod_state in unsafe { MODS_STATE.iter_mut() } {
        load_mod(mod_state);
    }
}
