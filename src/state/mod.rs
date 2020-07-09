use amethyst::{
    prelude::*,
    ecs::world::*,
    renderer::{
	rendy::{
            mesh::{Normal, Position, TexCoord},
        },
    },
    assets::{PrefabLoader, RonFormat},
    utils::scene::BasicScenePrefab,
    ui::{UiCreator, UiFinder, UiEvent, UiEventType},
    core::{
        transform::Transform,
    },
};
use crate::{
    gen::*,
};
use rand::random;
use log::info;

const BUTTON_START: &str = "start";
const BUTTON_LOAD: &str = "load";
const BUTTON_OPTIONS: &str = "options";
const BUTTON_CREDITS: &str = "credits";
const CONTAINER: &str = "container";

pub type ScenePrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

/* Main Menu State */
#[derive(Default, Debug)]
pub struct MainMenuState {
    button_start: Option<Entity>,
    button_load: Option<Entity>,
    button_options: Option<Entity>,
    button_credits: Option<Entity>,
    container: Option<Entity>,
}

impl SimpleState for MainMenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, ..} = data;
        
        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/main_menu.ron", ());
        });
        
        let player_handle = world.exec(|loader: PrefabLoader<'_, ScenePrefabData>| {
            loader.load("prefabs/player.ron", RonFormat, ())
        });
        
        let player = world.create_entity().with(player_handle).build();

        world.insert(PlayerEntity(player.id()));
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> Trans<GameData<'static, 'static>, StateEvent> {
        // only search for buttons if they have not been found yet
        let StateData { world, .. } = data;

        if self.button_start.is_none()
            || self.button_load.is_none()
            || self.button_options.is_none()
            || self.button_credits.is_none()
            || self.container.is_none()
        {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.button_start = ui_finder.find(BUTTON_START);
                self.button_load = ui_finder.find(BUTTON_LOAD);
                self.button_options = ui_finder.find(BUTTON_OPTIONS);
                self.button_credits = ui_finder.find(BUTTON_CREDITS);
                self.container = ui_finder.find(CONTAINER);
            });
        }

        Trans::None
    }

    fn handle_event(
        &mut self,
        data: StateData<GameData<'_, '_>>,
        event: StateEvent
        ) -> Trans<GameData<'static, 'static>, StateEvent> {
        let StateData { world, .. } = data;
        match event {
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_start {
                    info!("[Trans::Switch] Switching to LoadingState");
                    world.delete_entity(self.container.expect("[ERROR][raiders::state] Container not found"))
                        .unwrap();
                    
                    return Trans::Switch(Box::new(LoadingState::default()));
                }
                if Some(target) == self.button_load || Some(target) == self.button_options || Some(target) == self.button_credits {
                    info!("This Buttons functionality is not yet implemented!");
                }

                Trans::None
            },
            StateEvent::Window(_event) => Trans::None,
            StateEvent::Input(_event) => Trans::None,
            _ => Trans::None,
        }
    }
}

/* Loading State */
#[derive(Default, Debug)]
pub struct LoadingState {
    screen_loading: Option<Entity>,
    finished: bool,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        
        world.exec(|mut creator: UiCreator<'_>| creator.create("ui/loading.ron", ()) );
        
        let map_type: Terrain = random();
        let mut gen = MapGenerator::new(map_type);
        gen.build_terrain();
        gen.finish();

        let map_handle = world.exec(|loader: PrefabLoader<'_, ScenePrefabData>| {
            loader.load(gen.map_path(), RonFormat, ())
        });
        
        let _map = world.create_entity()
            .with(map_handle)
            .with(Transform::default())
            .build();

        self.finished = true;
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> Trans<GameData<'static, 'static>, StateEvent> {
        // only search for buttons if they have not been found yet
        let StateData { world, .. } = data;

        if self.screen_loading.is_none() {
            world.exec(|ui_finder: UiFinder<'_>| self.screen_loading = ui_finder.find(CONTAINER) );
        }

        if self.finished && !self.screen_loading.is_none() {
            world.delete_entity(self.screen_loading.expect("[ERROR][raiders::state] Loading screen not found"))
                .unwrap();
            self.screen_loading = None;
        }

        Trans::None
    }

    fn handle_event(
        &mut self,
        _data: StateData<GameData<'_, '_>>,
        event: StateEvent
        ) -> Trans<GameData<'static, 'static>, StateEvent> {
        match event {
            StateEvent::Window(_event) => {},
            StateEvent::Ui(_event) => {},
            StateEvent::Input(_event) => {},
        };

        Trans::None
    }
}

/* Gameplat State */
pub struct GameplayState;

impl SimpleState for GameplayState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("Game session started");
    }

    fn handle_event(
        &mut self,
        _data: StateData<GameData<'_, '_>>,
        event: StateEvent
        ) -> Trans<GameData<'static, 'static>, StateEvent> {
        match event {
            StateEvent::Window(_event) => {},
            StateEvent::Ui(_event) => {},
            StateEvent::Input(_event) => {},
        };

        Trans::None
    }
}

/* Pause State */
pub struct PauseState;

impl SimpleState for PauseState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("Game paused");
    }

    fn handle_event(
        &mut self,
        _data: StateData<GameData<'_, '_>>,
        event: StateEvent
        ) -> Trans<GameData<'static, 'static>, StateEvent> {
        match event {
            StateEvent::Window(_event) => {},
            StateEvent::Ui(_event) => {},
            StateEvent::Input(_event) => {},
        };

        Trans::None
    }
}

/* Result State */
pub struct ResultState;

impl SimpleState for ResultState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("Session has ended");
    }

    fn handle_event(
        &mut self,
        _data: StateData<GameData<'_, '_>>,
        event: StateEvent
        ) -> Trans<GameData<'static, 'static>, StateEvent> {
        match event {
            StateEvent::Window(_event) => {},
            StateEvent::Ui(_event) => {},
            StateEvent::Input(_event) => {},
        };

        Trans::None
    }
}

/* Resource wrappers */
#[derive(Default)]
pub struct PlayerEntity(Index);

impl PlayerEntity {
    pub fn index(&self) -> Index { return self.0 }
}
