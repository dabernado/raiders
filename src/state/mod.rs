use amethyst::{
    prelude::*,
    ecs::world::*,
    renderer::{
	rendy::mesh::{Normal, Position, TexCoord},
    },
    assets::{PrefabLoader, RonFormat},
    utils::scene::BasicScenePrefab,
    ui::{UiCreator, UiFinder, UiEvent, UiEventType},
};
use log::info;

const BUTTON_START: &str = "start";
const BUTTON_LOAD: &str = "load";
const BUTTON_OPTIONS: &str = "options";
const BUTTON_CREDITS: &str = "credits";

pub type ScenePrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

/* Main Menu State */
#[derive(Default, Debug)]
pub struct MainMenuState {
    button_start: Option<Entity>,
    button_load: Option<Entity>,
    button_options: Option<Entity>,
    button_credits: Option<Entity>,
}

impl SimpleState for MainMenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, ..} = data;
        
        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/main_menu.ron", ());
        });
        
        let sphere_handle = world.exec(|loader: PrefabLoader<'_, ScenePrefabData>| {
            loader.load("prefabs/sphere.ron", RonFormat, ())
        });
        let player_handle = world.exec(|loader: PrefabLoader<'_, ScenePrefabData>| {
            loader.load("prefabs/player.ron", RonFormat, ())
        });
        
        let _sphere = world.create_entity().with(sphere_handle).build();
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
        {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.button_start = ui_finder.find(BUTTON_START);
                self.button_load = ui_finder.find(BUTTON_LOAD);
                self.button_options = ui_finder.find(BUTTON_OPTIONS);
                self.button_credits = ui_finder.find(BUTTON_CREDITS);
            });
        }

        Trans::None
    }

    fn handle_event(
        &mut self,
        _data: StateData<GameData<'_, '_>>,
        event: StateEvent
        ) -> Trans<GameData<'static, 'static>, StateEvent> {
        match event {
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_start {
                    info!("[Trans::Switch] Switching to LoadingState");
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
pub struct LoadingState;

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        world.delete_all();

        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/loading.ron", ());
        });
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

/*#[derive(Default)]
pub struct CurrentUiElement(Index);

impl CurrentUiElement {
    pub fn index(&self) -> Index { return self.0 }
    pub fn update(&mut self, index: Index) { self.0 = index }
}

pub struct UiIter(Cycle<IntoIter<Entity>>);

impl UiIter {
    pub fn get_mut(&mut self) -> &mut Cycle<IntoIter<Entity>> { return &mut self.0 }
}

#[derive(Default)]
pub struct UiSize(usize);

impl UiSize {
    pub fn unwrap(&self) -> usize { return self.0 }
}*/
