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

fn static_make_noise(ferris: &SeaCreature) {
    // we know the real type
    print_type_of(&ferris);
    print!(" goes ");
    ferris.make_noise();
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

// fn generic_make_noise<T>(ferris: &T)
// where
//     T: NoiseMaker,
// {
//     // we know the real type at compile-time
//     print_type_of(&ferris);
//     print!(" goes ");
//     ferris.make_noise();
// }

fn generic_make_noise(ferris: &impl NoiseMaker)
{
    // we know the real type at compile-time
    print_type_of(&ferris);
    print!(" goes ");
    ferris.make_noise();
}

struct Ocean {
    animals: Vec<Box<dyn NoiseMaker>>,
}

pub fn main() -> SeaCreature {
// pub fn main() -> impl NoiseMaker {
// pub fn main() -> &'static Box<dyn NoiseMaker> {
// pub fn main() -> &'static Box<(dyn NoiseMaker + 'static)> {
    let ferris = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    let sarah = SeaCreature {
        name: String::from("Sarah"),
        noise: String::from("swish"),
    };

    println!("{} goes {}", ferris.name, ferris.get_sound());
    print!("{} goes ", ferris.name);
    ferris.make_alot_of_noise();
    static_make_noise(&ferris);
    dynamic_make_noise(&ferris);
    generic_make_noise(&ferris);

    let ocean = Ocean {
        animals: vec![Box::new(ferris), Box::new(sarah)],
    };

    for a in ocean.animals.iter() {
        a.make_noise();
        // let creature = *a;
    }

    // let sarah = *&ocean.animals[1];
    // // sarah.make_noise();
    // sarah
    let watisthis = &ocean.animals[1];
    print_type_of(&watisthis);
    println!();
    // creature
    // How to return the Box?
    SeaCreature {
        name: String::from("Mike"),
        noise: String::from("derp"),
    }
}
