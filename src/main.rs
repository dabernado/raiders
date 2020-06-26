#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

use raiders::{
    state::*,
    system::{
        control::ControllerSystem,
    },
};
use amethyst::{
    core::transform::TransformBundle,
    assets::PrefabLoaderSystemDesc,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{UiBundle, RenderUi},
    utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let binding_path = config_dir.join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_system_desc(PrefabLoaderSystemDesc::<ScenePrefabData>::default(), "", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(ControllerSystem, "controller_system", &["input_system"])
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat3D::default()),
        )?;

    let mut game = Application::new(assets_dir, MainMenuState::default(), game_data)?;
    game.run();

    Ok(())
}
