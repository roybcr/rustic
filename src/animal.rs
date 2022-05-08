pub(crate) trait Animal {
    fn get_name(&self) -> Option<String>;
    fn get_noise(&self) -> Option<String>;
    fn make_noise(&self);
}

pub struct Human {
    name: String,
    pet: Option<Pet>,
}

pub struct Pet {
    name: String,
    noise: String,
}

pub(crate) trait PetOwner {
    fn go_for_a_walk(&self);
}

impl Human {
    pub fn new(name: String, pet: Option<Pet>) -> Self {
        Self { name, pet }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_pet(&self) -> String {
        match &self.pet {
            Option::None => "".to_owned(),
            Option::Some(pet) => pet.get(),
        }
    }
}

impl PetOwner for Human {
    fn go_for_a_walk(&self) {
        println!(
            "{} and their pet, {} goes for a walk!",
            self.get_name(),
            &self.get_pet(),
        );
    }
}

impl Pet {
    pub fn new(name: String, noise: String) -> Pet {
        Pet { name, noise }
    }

    fn get(&self) -> String {
        self.name.to_owned() + " , " + &self.noise.to_owned()
    }
}

impl Animal for Option<Pet> {
    fn get_name(&self) -> Option<String> {
        match self {
            Option::Some(animal) => Some(animal.name.to_owned()),
            Option::None => None,
        }
    }

    fn get_noise(&self) -> Option<String> {
        match self {
            Option::Some(animal) => Some(animal.noise.to_owned()),
            Option::None => None,
        }
    }

    fn make_noise(&self) {
        match self {
            Option::Some(animal) => {
                println!("I'm a {}, and my noise is {}", animal.name, animal.noise)
            }
            Option::None => (),
        }
    }
}
