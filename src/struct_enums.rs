enum Species { Crab, Octopus, Fish, Clam }
enum PoisonType { Acidic, Painful, Lethal }
enum Size { Big, Small }
enum Weapon {
    Claw(i32, Size),
    Poison(PoisonType),
    None
}

struct SeaCreature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: Weapon,
}


enum Stuff {
    Sword,
    Shield,
}

enum Item {
    StuffInventory(Stuff),
    StringInventory(String),
    // None represents the absence of an item
    None,
}

struct BagOfHolding {
    item: Item,
}


pub fn main() {
    // SeaCreature's data is on stack
    let ferris = SeaCreature {
        // String struct is also on stack,
        // but holds a reference to data on heap
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: Weapon::Claw(2, Size::Small),
    };

    match ferris.species {
        Species::Crab => {
            match ferris.weapon {
                Weapon::Claw(num_claws,size) => {
                    let size_description = match size {
                        Size::Big => "big",
                        Size::Small => "small"
                    };
                    println!("ferris is a crab with {} {} claws", num_claws, size_description)
                },
                _ => println!("ferris is a crab with some other weapon")
            }
        },
        _ => println!("ferris is some other animal"),
    }

    // let item = Item::Inventory(String::from("test"));
    // let item = Item::Inventory("test".to_string());
    // let item = Item::Inventory(Stuff::Sword);
    let mut stuff = Item::None;
    match stuff {
        // Item::None => {
        //     let stuff_description = match stuff {
        //         Stuff::Sword => "sword",
        //         Stuff::Shield => "shield"
        //     };
        //     println!("{}", stuff_description)
        // },
        _ => println!("no stuff")
    }
    stuff = Item::StuffInventory(Stuff::Sword);
    match stuff {
        Item::StuffInventory(stuff) => {
            let stuff_description = match stuff {
                Stuff::Sword => "sword",
                Stuff::Shield => "shield"
            };
            println!("{}", stuff_description)
        },
        _ => println!("no stuff")
    }
    stuff = Item::StringInventory(String::from("shield"));
    match stuff {
        Item::StringInventory(s) => {
            println!("{}", s)
        },
        _ => println!("no stuff")
    }
    let bag = BagOfHolding {
        // item: Item::StringInventory(String::from("sword"))
        item: Item::None
    };
    match bag.item {
        Item::StringInventory(s) => {
            println!("{} in the bag", s)
        },
        _ => println!("no stuff")
    }
    // println!("{}", Item::StringInventory
    // let item = Item::Inventory("test".to_string());
    // println!("{}", item.0.len())
    // println!("{}", stuff_description)
    // let bag = BagOfHolding { item: Inventory("test") };
    // // let bag = BagOfHolding {  };
    // println!("{}", String::to_string(bag.item.0));
}

