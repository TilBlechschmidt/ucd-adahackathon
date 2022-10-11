pub type Price = usize;

mod bundle;
mod invoice;
mod material;
mod product;
pub mod warehouse;

pub use bundle::{Bundle, ConstBundle};
pub use invoice::{Invoice, Statistics};
pub use material::Material;
pub use product::*;

pub fn build_invoice(n: usize) -> Invoice {
    let mut invoice = Invoice::new();
    invoice.add_many(warehouse::STARTER_BUNDLE, n);
    invoice
}
