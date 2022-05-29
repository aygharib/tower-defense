use crate::engine::Engine;

pub struct Interface {
    engine: Engine,
}

impl Interface {
    pub fn run(engine: Engine) {
        let interface =  Self {
            engine,
        };
        todo!("Run the game")
    }
}