use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use rand::{distributions::Uniform, thread_rng, Rng};

lazy_static! {
    static ref ROBOT_NAME_REGISTRY: Arc<Mutex<HashSet<String>>> =
        Arc::new(Mutex::new(HashSet::new()));
}

pub struct Robot {
    _name: String,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            _name: Robot::generate_name(),
        }
    }

    fn generate_name() -> String {
        let mut rng = thread_rng();
        let num = rng.gen_range(0..=999);
        let prefix = rng
            .sample_iter(&Uniform::from('A'..='Z'))
            .take(2)
            .collect::<String>();

        format!("{}{:03}", prefix, num)
    }

    pub fn name(&self) -> &str {
        &self._name
    }

    pub fn reset_name(&mut self) {
        loop {
            let candidate = Robot::generate_name();
            let mut reg = ROBOT_NAME_REGISTRY.lock().unwrap();
            match reg.get(&candidate) {
                None => {
                    self._name = candidate.to_owned();
                    reg.insert(candidate.to_owned());
                    break;
                }
                Some(_) => continue,
            }
        }
    }
}
