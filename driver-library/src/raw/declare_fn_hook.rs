#[macro_export]
macro_rules! declare_raw_fn_hook {
    (
        $raw_fn_type:ty,
        $raw_fn_option_static_name:ident,
        $raw_fn_hook_trampoline_static_name:ident,
        $raw_fn_hook_detour_fn_name:ident,
        $register_raw_fn_hook_fn_name:ident,
    ) => {
        static mut $raw_fn_hook_trampoline_static_name: $raw_fn_type = $raw_fn_hook_detour_fn_name;

        mod declare_raw_fn_hook_macro_mod
        {
            use super::*;

            static mut HANDLE_OPTION: Option<detour::RawDetour> = None;

            pub fn register()
            {
                unsafe {
                    HANDLE_OPTION = Some(
                        detour::RawDetour::new(
                            $raw_fn_option_static_name.unwrap() as *const (),
                            $raw_fn_hook_detour_fn_name as *const (),
                        )
                        .unwrap(),
                    );

                    $raw_fn_hook_trampoline_static_name = std::mem::transmute(
                        HANDLE_OPTION
                            .as_ref()
                            .unwrap()
                            .trampoline(),
                    );

                    HANDLE_OPTION
                        .as_ref()
                        .unwrap()
                        .enable()
                        .unwrap();
                }
            }
        }

        pub fn $register_raw_fn_hook_fn_name()
        {
            declare_raw_fn_hook_macro_mod::register();
        }
    };
}
