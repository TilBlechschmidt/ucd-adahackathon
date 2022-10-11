use std::fmt::{Debug, Display};

use crate::{Price, Product};

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct ConstBundle<const SIZE: usize>([Product; SIZE]);

impl<const SIZE: usize> ConstBundle<SIZE> {
    pub const fn new(products: [Product; SIZE]) -> Self {
        Self(products)
    }

    pub const fn price(&self) -> Price {
        let mut offset = 0;
        let mut total: Price = 0;

        while offset < self.0.len() {
            total = total + self.0[offset].price();
            offset = offset + 1;
        }

        total
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Bundle(Vec<Product>);

impl Bundle {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, product: Product) {
        self.0.push(product);
    }

    pub fn price(&self) -> Price {
        self.0.iter().map(Product::price).sum()
    }

    pub fn products(&self) -> impl Iterator<Item = &Product> {
        self.0.iter()
    }
}

impl<const SIZE: usize> From<ConstBundle<SIZE>> for Bundle {
    fn from(bundle: ConstBundle<SIZE>) -> Self {
        Self(bundle.0.into_iter().collect())
    }
}

impl Display for Bundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Bundle: ")?;

        for (i, product) in self.0.iter().enumerate() {
            write!(f, "{}", product)?;

            if i < self.0.len() - 1 {
                f.write_str(", ")?;
            }
        }

        // Technically cheated but all items have no cent values so we just assume prices are in € instead of cents :)
        // (should not make any performance / energy difference either way — tho using integers instead of floats surely has)
        write!(f, ": {},00", self.price())
    }
}
