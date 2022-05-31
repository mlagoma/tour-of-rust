pub struct SeaCreature {
    pub name: String,
    noise: String,
}

impl SeaCreature {
    pub fn get_sound(&self) -> &str {
        &self.noise
    }
}

pub fn main() -> SeaCreature {
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    println!("{} goes {}", creature.name, creature.get_sound());
    println!("{} goes {}", creature.name, creature.noise);
    creature
}
