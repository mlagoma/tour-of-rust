pub struct SeaCreature {
    pub name: String,
    noise: String,
}

impl SeaCreature {
    pub fn get_sound(&self) -> &str {
        &self.noise
    }
}

pub trait NoiseMaker {
    fn make_noise(&self);
}

trait LoudNoiseMaker: NoiseMaker {
    fn make_alot_of_noise(&self) {
        self.make_noise();
        self.make_noise();
        self.make_noise();
    }
}

impl NoiseMaker for SeaCreature {
    fn make_noise(&self) {
        println!("{}", &self.get_sound());
    }
}

impl LoudNoiseMaker for SeaCreature {}

pub fn main() -> SeaCreature {
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    println!("{} goes {}", creature.name, creature.get_sound());
    print!("{} goes ", creature.name);
    creature.make_noise();
    creature.make_alot_of_noise();
    creature
}
