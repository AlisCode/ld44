use crate::components::register_components;
use crate::ld_entities::LDEntities;
use crate::level::{Level, LevelLoader};
use crate::renderer::{LDImages, Renderer};
use crate::resources::*;
use crate::states::main_state::LD44States;
use crate::systems::*;

use quicksilver::graphics::Background;
use quicksilver::lifecycle::State;
use quicksilver::prelude::*;
use quicksilver::saving::load as qs_load;
use specs::{Dispatcher, DispatcherBuilder, World};

pub struct GameState {
    world: World,
    logic: Dispatcher<'static, 'static>,
    graphics: Dispatcher<'static, 'static>,
    turn_player: Dispatcher<'static, 'static>,
    turn_env: Dispatcher<'static, 'static>,
    turn_input: Dispatcher<'static, 'static>,
    assets: Assets,
    level_loader: LevelLoader,
    background_image: Asset<Image>,
}

impl GameState {
    pub fn wants_to_switch(&self) -> Option<LD44States> {
        None
    }

    fn create_world() -> (World, LevelLoader) {
        let mut world = World::new();
        register_components(&mut world);

        // TODO: Load level (from saving profile)
        let _cursor = LDEntities::cursor(&mut world);
        let level = qs_load::<Level>("LD44", "curr_level").unwrap_or_default();
        let ll = LevelLoader::load(level.get_url());

        // Create various resources
        let delta_time: DeltaTime = Default::default();
        let mouse_wrapper: MouseWrapper = Default::default();
        let camera: Camera = Default::default();
        let nav_mesh: NavMesh = Default::default();
        let turn_locker: TurnLocker = Default::default();
        let selection: Selection = Default::default();

        // Adds resources to the world
        world.add_resource(delta_time);
        world.add_resource(mouse_wrapper);
        world.add_resource(camera);
        world.add_resource(nav_mesh);
        world.add_resource(turn_locker);
        world.add_resource(selection);

        (world, ll)
    }

    pub fn reload(&mut self) {
        let (world, ll) = GameState::create_world();
        self.world = world;
        self.level_loader = ll;
    }
}

impl State for GameState {
    fn new() -> Result<Self> {
        let background_image = Asset::new(Image::load("background.png"));
        let mut assets = Assets::default();
        assets.add_image(LDImages::Cursor);
        assets.add_image(LDImages::CursorSelected);
        assets.add_image(LDImages::Ground);
        assets.add_image(LDImages::RobotRight);
        assets.add_image(LDImages::RobotLeft);
        assets.add_image(LDImages::RobotUp);
        assets.add_image(LDImages::Laser);
        assets.add_image(LDImages::LaserOn);
        assets.add_image(LDImages::LaserOff);

        let (world, ll) = GameState::create_world();

        // Create dispatchers
        let logic = DispatcherBuilder::new()
            .with(SysCursor, "sys_cursor", &[])
            .build();

        let graphics = DispatcherBuilder::new()
            .with(SysMoving, "sys_moving", &[])
            .with(SysCamFollowCursor, "sys_cam_follow_cursor", &["sys_moving"])
            .with(SysRenderable, "sys_renderable", &["sys_moving"])
            .with(SysTurnLock, "sys_turn_lock", &["sys_moving"])
            .build();

        let turn_player = DispatcherBuilder::new()
            .with(SysNavigation, "sys_navigation", &[])
            .with(SysNavAgent, "sys_nav_agent", &["sys_navigation"])
            .build();

        let turn_env = DispatcherBuilder::new().build();

        let turn_input = DispatcherBuilder::new()
            .with(SysSelectable, "sys_selectable", &[])
            .with(
                SysSelectWalkable,
                "sys_select_walkable",
                &["sys_selectable"],
            )
            .build();

        Ok(GameState {
            world,
            logic,
            graphics,
            turn_player,
            turn_env,
            turn_input,
            assets,
            level_loader: ll,
            background_image,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if !self.level_loader.poll(&mut self.world) {
            return Ok(());
        }

        // Updates mouse
        self.world.write_resource::<MouseWrapper>().mouse = Some(window.mouse());

        self.logic.dispatch(&self.world.res);

        let mut turn_lock = self.world.write_resource::<TurnLocker>();
        if turn_lock.next_sys {
            turn_lock.trigger_next_sys();
        }
        if turn_lock.end_turn {
            turn_lock.end_turn = false;
            let lock = turn_lock.lock;
            drop(turn_lock);
            match lock {
                TurnLockMode::PlayerInput => self.turn_input.dispatch(&self.world.res),
                TurnLockMode::PlayerTurn => self.turn_player.dispatch(&self.world.res),
                TurnLockMode::WorldTurn => {
                    self.turn_env.dispatch(&self.world.res);
                }
            }
        }

        // Updates camera
        window.set_view(self.world.read_resource::<Camera>().as_view());

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        let view = &self.world.read_resource::<Camera>().view.clone();
        self.background_image.execute(|bkg| {
            window.draw(view, Background::Img(&bkg));
            Ok(())
        })?;

        if !self.level_loader.poll(&mut self.world) {
            return Ok(());
        }
        // Ticks delta
        self.world.write_resource::<DeltaTime>().tick(window);
        self.world.add_resource(Renderer::default());

        self.graphics.dispatch(&self.world.res);

        let mut renderer = self.world.write_resource::<Renderer>();
        renderer.render(window, &mut self.assets);
        Ok(())
    }
}
