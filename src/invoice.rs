use crate::{Bundle, Material::Steel, Price, ProductType::*};
use std::{collections::HashMap, fmt::Display};

pub struct Statistics {
    pub hammers_sold: usize,
    pub steel_required: usize,
    pub nails_dispensed: usize,
}

pub struct Invoice {
    items: HashMap<Bundle, usize>,
}

impl Invoice {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn add_many(&mut self, item: impl Into<Bundle>, count: usize) {
        self.items
            .entry(item.into())
            .and_modify(|c| *c += count)
            .or_insert(count);
    }

    pub fn add(&mut self, item: impl Into<Bundle>) {
        self.add_many(item, 1);
    }

    pub fn price(&self) -> Price {
        self.items
            .iter()
            .map(|(item, count)| item.price() * count)
            .sum()
    }

    pub fn statistics(&self) -> Statistics {
        let mut hammers_sold = 0;
        let mut steel_required = 0;
        let mut nails_dispensed = 0;

        for (bundle, count) in self.items.iter() {
            for product in bundle.products() {
                match product.product_type {
                    Hammer(_) => hammers_sold += count,
                    Nail(_, box_size) => nails_dispensed += count * box_size,
                    _ => {}
                }

                match product.material {
                    Steel => steel_required += count,
                    _ => {}
                }
            }
        }

        Statistics {
            hammers_sold,
            steel_required,
            nails_dispensed,
        }
    }
}

impl Display for Invoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (bundle, count) in self.items.iter() {
            for _ in 0..*count {
                // TODO Cache this? :)
                write!(f, "{}\n", bundle)?;
            }
        }

        Ok(())
    }
}
