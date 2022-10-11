use std::fmt::Display;

use crate::Price;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Material {
    Steel,
    Brass,
    Titanium,
    Unknown,
}

impl Material {
    pub const fn price(&self) -> Price {
        match *self {
            Material::Steel => 10,
            Material::Brass => 20,
            Material::Titanium => 50,
            Material::Unknown => 0,
        }
    }
}

impl Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Material::Steel => "steel",
            Material::Brass => "brass",
            Material::Titanium => "titanium",
            Material::Unknown => "unknown",
        })
    }
}
