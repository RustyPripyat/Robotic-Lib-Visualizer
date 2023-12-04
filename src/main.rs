// use chrono::Utc;
use robotics_lib::event::events::Event;
use robotics_lib::energy::Energy;
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::World;
use robotics_lib::world::worldgenerator::Generator;

use crate::generator::*;
use crate::visualizer::save_world_image;

pub mod visualizer;
mod content;
mod tiletype;
mod utils;
mod generator;

fn main() {
    struct MyRobot(Robot);

    impl Runnable for MyRobot {
        fn process_tick(&mut self, _world: &mut World) {
            // Do nothing
        }

        fn get_energy(&self) -> &Energy {
            &self.0.energy
        }
        fn get_energy_mut(&mut self) -> &mut Energy {
            &mut self.0.energy
        }

        fn get_coordinate(&self) -> &Coordinate {
            &self.0.coordinate
        }
        fn get_coordinate_mut(&mut self) -> &mut Coordinate {
            &mut self.0.coordinate
        }

        fn get_backpack(&self) -> &BackPack {
            &self.0.backpack
        }
        fn get_backpack_mut(&mut self) -> &mut BackPack {
            &mut self.0.backpack
        }
        fn handle_event(&mut self, _: Event) { todo!() }
    }

    let _r = MyRobot(Robot::new());
    let size = 1000;
    let mut generator = WorldGenerator::new(size, NoiseSettings::default(), Thresholds::default(), LavaSettings::default(size));

    let tiles = generator.gen().0;
    save_world_image(&tiles, (0, 0), format!("o{}-f{}-l{}-p{}-a{}.png", generator.noise_settings.octaves, generator.noise_settings.frequency, generator.noise_settings.lacunarity, generator.noise_settings.persistence, generator.noise_settings.attenuation).as_str());
}
