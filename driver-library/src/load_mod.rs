use crate::*;

pub fn load_mod(mod_state: &mut ModState)
{
    if mod_state.is_loaded {
        return;
    }

    log::info!(
        "Loading {}...",
        mod_state.id
    );

    for mod_action_info in &mod_state
        .info
        .actions
    {
        execute_mod_action(
            &mod_state.id,
            &mod_state.dir_path,
            mod_action_info,
        );
    }

    mod_state.is_loaded = true;

    log::info!(
        "Loaded {}",
        mod_state.id
    );
}
