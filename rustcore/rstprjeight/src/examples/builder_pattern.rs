//Builder pattern
pub struct Pizza {
    pub size: u8,
    pub cheese: bool,
    pub pepperoni: bool,
    pub mushrooms: bool,
}

pub struct PizzaBuilder {
    pub size: u8,
    pub cheese: bool,
    pub pepperoni: bool,
    pub mushrooms: bool,
}

impl PizzaBuilder {
    pub fn new(size: u8) -> Self {
        PizzaBuilder {
            size,
            cheese: false,
            pepperoni: false,
            mushrooms: false,
        }
    }

    pub fn add_cheese(mut self) -> Self {
        self.cheese = true;
        self
    }

    pub fn add_pepperoni(mut self) -> Self {
        self.pepperoni = true;
        self
    }

    pub fn add_mushrooms(mut self) -> Self {
        self.mushrooms = true;
        self
    }

    pub fn build(self) -> Pizza {
        Pizza {
            size: self.size,
            cheese: self.cheese,
            pepperoni: self.pepperoni,
            mushrooms: self.mushrooms,
        }
    }
}

// let pizza = PizzaBuilder::new(12)
//     .add_cheese()
//     .add_pepperoni()
//     .build();
