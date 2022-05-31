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

fn static_make_noise(creature: &SeaCreature) {
    // we know the real type
    print_type_of(&creature);
    print!(" goes ");
    creature.make_noise();
}

fn dynamic_make_noise(noise_maker: &dyn NoiseMaker) {
    // we don't know the real type
    print_type_of(&noise_maker);
    print!(" goes ");
    noise_maker.make_noise();
}

fn print_type_of<T>(_: &T) {
    print!("{}", std::any::type_name::<T>())
}

// fn generic_make_noise<T>(creature: &T)
// where
//     T: NoiseMaker,
// {
//     // we know the real type at compile-time
//     print_type_of(&creature);
//     print!(" goes ");
//     creature.make_noise();
// }

fn generic_make_noise(creature: &impl NoiseMaker)
{
    // we know the real type at compile-time
    print_type_of(&creature);
    print!(" goes ");
    creature.make_noise();
}

pub fn main() -> SeaCreature {
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    println!("{} goes {}", creature.name, creature.get_sound());
    print!("{} goes ", creature.name);
    creature.make_alot_of_noise();
    static_make_noise(&creature);
    dynamic_make_noise(&creature);
    generic_make_noise(&creature);
    creature
}
