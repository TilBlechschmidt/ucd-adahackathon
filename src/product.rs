use std::fmt::Display;

use crate::{Material, Price};

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum HammerType {
    Sledge,
    Framing,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum NailType {
    Roofing,
    Framing,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum SawType {
    Jig,
    Circular,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum ProductType {
    Hammer(HammerType),
    Saw(SawType),
    // Nails come in a pack, second tuple value is the amount of nails
    Nail(NailType, usize),
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Product {
    pub product_type: ProductType,
    pub material: Material,
    price: Price,
}

impl Product {
    pub const fn new(product_type: ProductType, material: Material, price: Price) -> Self {
        Self {
            product_type,
            material,
            price,
        }
    }

    pub const fn price(&self) -> Price {
        self.price + self.material.price()
    }
}

impl Display for ProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use HammerType::Sledge;
        use NailType::*;
        use ProductType::*;
        use SawType::*;

        match self {
            Hammer(Sledge) => f.write_str("sledge hammer"),
            Hammer(HammerType::Framing) => f.write_str("framing hammer"),
            Saw(Jig) => f.write_str("jigsaw"),
            Saw(Circular) => f.write_str("circular saw"),
            Nail(Roofing, count) => f.write_fmt(format_args!("roofing nails (x{})", count)),
            Nail(Framing, count) => f.write_fmt(format_args!("framing nails (x{})", count)),
        }
    }
}

impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.material, self.product_type)
    }
}
