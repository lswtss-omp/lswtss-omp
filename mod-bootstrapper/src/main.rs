#![feature(ascii_char)]
#![feature(pointer_byte_offsets)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::needless_return)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::enum_variant_names)]

use std::path::Path;
use std::path::PathBuf;

use eframe::egui;
use lswtss_open_modding_platform_foundation::*;
use lswtss_open_modding_platform_mod_bootstrapper_core::*;

fn main() -> Result<(), eframe::Error>
{
    eframe::run_native(
        "LSW:TSS OMP Mod Bootstrapper",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size(
                [
                    800.0, 600.0,
                ],
            ),
            centered: true,
            ..Default::default()
        },
        Box::new(|_| Box::<AppState>::default()),
    )
}

struct AppLoadingActionState
{
    pub next_state_thread: Option<std::thread::JoinHandle<AppActionState>>,
}

struct AppSelectRootDirActionState {}

#[derive(Clone)]
struct AppBaseCharacterSelectInfo
{
    pub prefab_baked_resource_src_path: PathBuf,
    pub is_selected: bool,
}

#[derive(Clone)]
struct AppSelectBaseCharactersActionState
{
    pub root_dir_path: PathBuf,
    pub base_characters_select_info: Vec<AppBaseCharacterSelectInfo>,
}

#[derive(Clone)]
struct AppBaseCharacterPrefabBakedResourceDependencySelectInfo
{
    pub base: ResourceInfo,
    pub is_selected: bool,
}

#[derive(Clone)]
struct AppBaseCharacterPrefabBakedResourceSelectDependenciesInfo
{
    pub base: ResourceInfo,
    pub dependencies: Vec<AppBaseCharacterPrefabBakedResourceDependencySelectInfo>,
}

struct AppSelectBaseCharacterPrefabBakedResourcesDependenciesActionState
{
    pub base_character_prefab_baked_resources_select_dependencies_info: Vec<AppBaseCharacterPrefabBakedResourceSelectDependenciesInfo>,
}

struct AppModCustomCharacterSelectPropertiesInfo
{
    pub id: String,
    pub class: ModV1CharacterClass,
    pub base_character_prefab_baked_resource_info: PrefabBakedResourceInfo,
}

struct AppSelectModCustomCharactersPropertiesActionState
{
    pub mod_custom_characters_select_properties_info: Vec<AppModCustomCharacterSelectPropertiesInfo>,
}

enum AppActionState
{
    Loading(AppLoadingActionState),
    SelectRootDir(AppSelectRootDirActionState),
    SelectBaseCharacters(AppSelectBaseCharactersActionState),
    SelectBaseCharacterPrefabBakedResourcesDependencies(AppSelectBaseCharacterPrefabBakedResourcesDependenciesActionState),
    SelectModCustomCharactersProperties(AppSelectModCustomCharactersPropertiesActionState),
}

struct AppState
{
    action: AppActionState,
}

impl Default for AppState
{
    fn default() -> Self
    {
        Self {
            action: AppActionState::SelectRootDir(AppSelectRootDirActionState {}),
        }
    }
}

impl eframe::App for AppState
{
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    )
    {
        egui::TopBottomPanel::top("header")
            .frame(egui::Frame::default().outer_margin(12.0))
            .show(
                ctx,
                |ui| {
                    ui.spacing();
                    ui.vertical_centered_justified(
                        |ui| {
                            ui.heading("Mod Bootstrapper");
                            ui.separator();
                            ui.label("LSW:TSS Open Modding Platform");
                        },
                    );
                    ui.spacing();
                },
            );

        egui::TopBottomPanel::bottom("footer")
            .frame(egui::Frame::default().outer_margin(12.0))
            .show(
                ctx,
                |ui| {
                    ui.vertical_centered_justified(
                        |ui| {
                            match &mut self.action {
                                AppActionState::SelectBaseCharacters(app_select_base_characters_action_state) => {
                                    if ui
                                        .button("Continue")
                                        .clicked()
                                    {
                                        let root_dir_path = app_select_base_characters_action_state
                                            .root_dir_path
                                            .clone();
                                        let base_characters_select_info = app_select_base_characters_action_state
                                            .base_characters_select_info
                                            .clone();

                                        self.action = AppActionState::Loading(
                                            AppLoadingActionState {
                                                next_state_thread: Some(
                                                    std::thread::spawn(
                                                        move || {
                                                            return AppActionState::SelectBaseCharacterPrefabBakedResourcesDependencies(
                                                                AppSelectBaseCharacterPrefabBakedResourcesDependenciesActionState {
                                                                    base_character_prefab_baked_resources_select_dependencies_info:
                                                                        get_app_base_character_prefab_baked_resources_select_dependencies_info(
                                                                            &root_dir_path,
                                                                            &base_characters_select_info,
                                                                        ),
                                                                },
                                                            );
                                                        },
                                                    ),
                                                ),
                                            },
                                        );
                                    }
                                },
                                AppActionState::SelectBaseCharacterPrefabBakedResourcesDependencies(app_select_base_character_prefab_baked_resources_dependencies_action_state) => {
                                    if ui
                                        .button("Continue")
                                        .clicked()
                                    {
                                        let base_character_prefab_baked_resources_select_dependencies_info = app_select_base_character_prefab_baked_resources_dependencies_action_state
                                            .base_character_prefab_baked_resources_select_dependencies_info
                                            .clone();

                                        self.action = AppActionState::Loading(
                                            AppLoadingActionState {
                                                next_state_thread: Some(
                                                    std::thread::spawn(
                                                        move || {
                                                            return AppActionState::SelectModCustomCharactersProperties(
                                                                AppSelectModCustomCharactersPropertiesActionState {
                                                                    mod_custom_characters_select_properties_info: get_app_mod_custom_characters_select_properties_info(
                                                                        &base_character_prefab_baked_resources_select_dependencies_info,
                                                                    ),
                                                                },
                                                            );
                                                        },
                                                    ),
                                                ),
                                            },
                                        );
                                    }
                                },
                                AppActionState::SelectModCustomCharactersProperties(app_select_mod_custom_characters_properties_action_state) => {
                                    if ui
                                        .button("Bootstrap!")
                                        .clicked()
                                    {
                                        if let Some(app_mod_dir_path) = rfd::FileDialog::new().pick_folder() {
                                            bootstrap_app_mod(
                                                &app_mod_dir_path,
                                                &app_select_mod_custom_characters_properties_action_state.mod_custom_characters_select_properties_info,
                                            );

                                            self.action = AppActionState::Loading(
                                                AppLoadingActionState {
                                                    next_state_thread: Some(
                                                        std::thread::spawn(
                                                            move || {
                                                                return AppActionState::SelectRootDir(AppSelectRootDirActionState {});
                                                            },
                                                        ),
                                                    ),
                                                },
                                            );
                                        }
                                    }
                                },
                                _ => {},
                            };
                        },
                    );
                },
            );

        match &mut self.action {
            AppActionState::Loading(app_loading_action_state) => {
                egui::CentralPanel::default()
                    .frame(egui::Frame::default().outer_margin(12.0))
                    .show(
                        ctx,
                        |ui| {
                            ui.vertical_centered_justified(
                                |ui| {
                                    ui.label("Loading...");
                                },
                            )
                        },
                    );

                if let Some(next_state_thread) = app_loading_action_state
                    .next_state_thread
                    .as_mut()
                {
                    if next_state_thread.is_finished() {
                        self.action = app_loading_action_state
                            .next_state_thread
                            .take()
                            .unwrap()
                            .join()
                            .unwrap_or(AppActionState::SelectRootDir(AppSelectRootDirActionState {}));
                    }
                } else {
                    self.action = AppActionState::SelectRootDir(AppSelectRootDirActionState {});
                }
            },
            AppActionState::SelectRootDir(_app_select_root_dir_action_state) => {
                egui::CentralPanel::default()
                    .frame(egui::Frame::default().outer_margin(12.0))
                    .show(
                        ctx,
                        |ui| {
                            ui.vertical_centered_justified(
                                |ui| {
                                    if ui
                                        .button("Select root directory")
                                        .clicked()
                                    {
                                        if let Some(app_root_dir_path) = rfd::FileDialog::new().pick_folder() {
                                            self.action = AppActionState::Loading(
                                                AppLoadingActionState {
                                                    next_state_thread: Some(
                                                        std::thread::spawn(
                                                            move || {
                                                                return AppActionState::SelectBaseCharacters(
                                                                    AppSelectBaseCharactersActionState {
                                                                        root_dir_path: app_root_dir_path.clone(),
                                                                        base_characters_select_info: get_app_base_characters_select_info(&app_root_dir_path),
                                                                    },
                                                                );
                                                            },
                                                        ),
                                                    ),
                                                },
                                            );
                                        }
                                    }
                                },
                            );
                        },
                    );
            },
            AppActionState::SelectBaseCharacters(_app_select_base_character_action_state) => {
                egui::CentralPanel::default()
                    .frame(egui::Frame::default().outer_margin(12.0))
                    .show(
                        ctx,
                        |ui| {
                            egui::ScrollArea::vertical()
                                .max_height(f32::INFINITY)
                                .auto_shrink(false)
                                .show(
                                    ui,
                                    |ui| {
                                        for app_base_character_select_info in &mut _app_select_base_character_action_state.base_characters_select_info {
                                            ui.horizontal(
                                                |ui| {
                                                    ui.checkbox(
                                                        &mut app_base_character_select_info.is_selected,
                                                        app_base_character_select_info
                                                            .prefab_baked_resource_src_path
                                                            .file_name()
                                                            .unwrap()
                                                            .to_str()
                                                            .unwrap(),
                                                    );
                                                },
                                            );
                                        }
                                    },
                                );
                        },
                    );
            },
            AppActionState::SelectBaseCharacterPrefabBakedResourcesDependencies(app_select_base_character_prefab_baked_resources_dependencies_action_state) => {
                egui::CentralPanel::default()
                    .frame(egui::Frame::default().outer_margin(12.0))
                    .show(
                        ctx,
                        |ui| {
                            egui::ScrollArea::vertical()
                                .max_height(f32::INFINITY)
                                .auto_shrink(false)
                                .show(
                                    ui,
                                    |ui| {
                                        for app_base_character_prefab_baked_resource_select_dependencies_info in
                                            &mut app_select_base_character_prefab_baked_resources_dependencies_action_state.base_character_prefab_baked_resources_select_dependencies_info
                                        {
                                            ui.label(
                                                &app_base_character_prefab_baked_resource_select_dependencies_info
                                                    .base
                                                    .canon_name,
                                            );

                                            for app_base_character_prefab_baked_resource_dependency_select_info in &mut app_base_character_prefab_baked_resource_select_dependencies_info.dependencies {
                                                ui.horizontal(
                                                    |ui| {
                                                        ui.checkbox(
                                                            &mut app_base_character_prefab_baked_resource_dependency_select_info.is_selected,
                                                            &app_base_character_prefab_baked_resource_dependency_select_info
                                                                .base
                                                                .canon_name,
                                                        );
                                                    },
                                                );
                                            }
                                        }
                                    },
                                );
                        },
                    );
            },
            AppActionState::SelectModCustomCharactersProperties(app_select_mod_custom_characters_properties_action_state) => {
                egui::CentralPanel::default()
                    .frame(egui::Frame::default().outer_margin(12.0))
                    .show(
                        ctx,
                        |ui| {
                            egui::ScrollArea::vertical()
                                .max_height(f32::INFINITY)
                                .auto_shrink(false)
                                .show(
                                    ui,
                                    |ui| {
                                        for app_mod_custom_character_select_properties_info in
                                            &mut app_select_mod_custom_characters_properties_action_state.mod_custom_characters_select_properties_info
                                        {
                                            ui.label(
                                                &app_mod_custom_character_select_properties_info
                                                    .base_character_prefab_baked_resource_info
                                                    .base
                                                    .canon_name,
                                            );
                                            ui.text_edit_singleline(&mut app_mod_custom_character_select_properties_info.id);
                                            egui::ComboBox::from_id_source(ui.next_auto_id())
                                                .selected_text(
                                                    format!(
                                                        "{:?}",
                                                        app_mod_custom_character_select_properties_info.class
                                                    ),
                                                )
                                                .show_ui(
                                                    ui,
                                                    |ui| {
                                                        ui.selectable_value(
                                                            &mut app_mod_custom_character_select_properties_info.class,
                                                            ModV1CharacterClass::Jedi,
                                                            "Jedi",
                                                        );
                                                        ui.selectable_value(
                                                            &mut app_mod_custom_character_select_properties_info.class,
                                                            ModV1CharacterClass::Sith,
                                                            "Sith",
                                                        );
                                                        ui.selectable_value(
                                                            &mut app_mod_custom_character_select_properties_info.class,
                                                            ModV1CharacterClass::RebelResistance,
                                                            "RebelResistance",
                                                        );
                                                        ui.selectable_value(
                                                            &mut app_mod_custom_character_select_properties_info.class,
                                                            ModV1CharacterClass::BountyHunter,
                                                            "BountyHunter",
                                                        );
                                                        ui.selectable_value(
                                                            &mut app_mod_custom_character_select_properties_info.class,
                                                            ModV1CharacterClass::AstromechDroid,
                                                            "AstromechDroid",
                                                        );
                                                        ui.selectable_value(
                                                            &mut app_mod_custom_character_select_properties_info.class,
                                                            ModV1CharacterClass::ProtocolDroid,
                                                            "ProtocolDroid",
                                                        );
                                                        ui.selectable_value(
                                                            &mut app_mod_custom_character_select_properties_info.class,
                                                            ModV1CharacterClass::Scoundrel,
                                                            "Scoundrel",
                                                        );
                                                        ui.selectable_value(
                                                            &mut app_mod_custom_character_select_properties_info.class,
                                                            ModV1CharacterClass::GalacticEmpire,
                                                            "GalacticEmpire",
                                                        );
                                                        ui.selectable_value(
                                                            &mut app_mod_custom_character_select_properties_info.class,
                                                            ModV1CharacterClass::Scavenger,
                                                            "Scavenger",
                                                        );
                                                        ui.selectable_value(
                                                            &mut app_mod_custom_character_select_properties_info.class,
                                                            ModV1CharacterClass::Civilian,
                                                            "Civilian",
                                                        );
                                                    },
                                                );
                                        }
                                    },
                                );
                        },
                    );
            },
        };
    }
}

fn get_app_base_characters_select_info(app_root_dir_path: &Path) -> Vec<AppBaseCharacterSelectInfo>
{
    let mut app_base_characters_select_info = Vec::new();

    let app_root_mod_chars_dir_path = app_root_dir_path
        .join("chars")
        .join("minifig");

    for app_root_chars_minifig_dir_dir_info in std::fs::read_dir(app_root_mod_chars_dir_path).unwrap() {
        let app_root_chars_minifig_dir_dir_info = app_root_chars_minifig_dir_dir_info.unwrap();
        let app_root_chars_minifig_dir_dir_path = app_root_chars_minifig_dir_dir_info.path();

        for app_root_chars_minifig_dir_dir_file_info in std::fs::read_dir(app_root_chars_minifig_dir_dir_path).unwrap() {
            let app_root_chars_minifig_dir_dir_file_info = app_root_chars_minifig_dir_dir_file_info.unwrap();
            let app_root_chars_minifig_dir_dir_file_path = app_root_chars_minifig_dir_dir_file_info.path();

            if let Some(app_root_chars_minifig_dir_dir_file_extension) = app_root_chars_minifig_dir_dir_file_path.extension() {
                if app_root_chars_minifig_dir_dir_file_extension.to_ascii_lowercase() == "prefab_baked" {
                    println!(
                        "found: {:?}",
                        app_root_chars_minifig_dir_dir_file_path
                    );

                    app_base_characters_select_info.push(
                        AppBaseCharacterSelectInfo {
                            prefab_baked_resource_src_path: app_root_chars_minifig_dir_dir_file_path,
                            is_selected: false,
                        },
                    );
                }
            }
        }
    }

    return app_base_characters_select_info;
}

fn get_app_base_character_prefab_baked_resources_select_dependencies_info(
    app_root_dir_path: &Path,
    app_base_characters_select_info: &Vec<AppBaseCharacterSelectInfo>,
) -> Vec<AppBaseCharacterPrefabBakedResourceSelectDependenciesInfo>
{
    let mut app_base_character_prefab_baked_resources_select_dependencies_info = Vec::new();

    for app_base_characters_select_info in app_base_characters_select_info {
        if !app_base_characters_select_info.is_selected {
            continue;
        }

        let app_base_character_prefab_baked_resource_info = get_prefab_baked_resource_info(
            app_root_dir_path,
            &app_base_characters_select_info.prefab_baked_resource_src_path,
        );

        app_base_character_prefab_baked_resources_select_dependencies_info.push(
            AppBaseCharacterPrefabBakedResourceSelectDependenciesInfo {
                base: app_base_character_prefab_baked_resource_info.base,
                dependencies: app_base_character_prefab_baked_resource_info
                    .dependencies
                    .iter()
                    .map(
                        |dependency| AppBaseCharacterPrefabBakedResourceDependencySelectInfo {
                            base: dependency.clone(),
                            is_selected: false,
                        },
                    )
                    .collect(),
            },
        );
    }

    return app_base_character_prefab_baked_resources_select_dependencies_info;
}

fn get_app_mod_custom_characters_select_properties_info(
    app_base_character_prefab_baked_resources_select_dependencies_info: &Vec<AppBaseCharacterPrefabBakedResourceSelectDependenciesInfo>
) -> Vec<AppModCustomCharacterSelectPropertiesInfo>
{
    let mut app_mod_custom_characters_select_properties_info = Vec::new();

    for app_base_character_prefab_baked_resource_select_dependencies_info in app_base_character_prefab_baked_resources_select_dependencies_info {
        app_mod_custom_characters_select_properties_info.push(
            AppModCustomCharacterSelectPropertiesInfo {
                id: "ID_OF_CHARACTER".to_string(),
                class: ModV1CharacterClass::Civilian,
                base_character_prefab_baked_resource_info: PrefabBakedResourceInfo {
                    base: app_base_character_prefab_baked_resource_select_dependencies_info
                        .base
                        .clone(),
                    dependencies: app_base_character_prefab_baked_resource_select_dependencies_info
                        .dependencies
                        .iter()
                        .filter(|dependency| dependency.is_selected)
                        .map(
                            |dependency| {
                                dependency
                                    .base
                                    .clone()
                            },
                        )
                        .collect(),
                },
            },
        );
    }

    return app_mod_custom_characters_select_properties_info;
}

fn bootstrap_app_mod(
    app_mod_dir_path: &Path,
    app_mod_custom_characters_select_properties_info: &Vec<AppModCustomCharacterSelectPropertiesInfo>,
)
{
    let mut app_mod_info = ModInfo {
        name: "Test mod".to_string(),
        actions: Vec::new(),
    };

    for app_mod_custom_character_select_properties_info in app_mod_custom_characters_select_properties_info {
        app_mod_info
            .actions
            .append(
                &mut bootstrap_mod_custom_character(
                    app_mod_dir_path,
                    &app_mod_custom_character_select_properties_info.id,
                    &app_mod_custom_character_select_properties_info.class,
                    &app_mod_custom_character_select_properties_info.base_character_prefab_baked_resource_info,
                ),
            );
    }

    std::fs::write(
        app_mod_dir_path.join("mod.json"),
        serde_json::to_string_pretty(&app_mod_info).unwrap(),
    )
    .unwrap();
}
