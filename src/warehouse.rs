use crate::product::HammerType::{self, Sledge};
use crate::product::NailType::*;
use crate::product::ProductType::*;
use crate::product::SawType::*;
use crate::ConstBundle;
use crate::Material::*;

macro_rules! warehouse {
    ($($i:ident($t:expr, $m:expr, $p:expr)),*) => {
        $(
            #[allow(dead_code)]
            pub const $i: crate::Product = crate::Product::new($t, $m, $p);
        )*
    };
}

warehouse! {
    HAMMER_SLEDGE_STEEL_LIGHT       (Hammer(Sledge), Steel, 25),
    HAMMER_SLEDGE_STEEL_HEAVY       (Hammer(Sledge), Steel, 40),
    HAMMER_FRAMING_STEEL_LIGHT      (Hammer(HammerType::Framing), Steel, 30),
    HAMMER_FRAMING_TITANIUM_LIGHT   (Hammer(HammerType::Framing), Titanium, 60),

    SAW_JIG_STEEL                   (Saw(Jig), Steel, 50),
    SAW_CIRCULAR                    (Saw(Circular), Unknown, 100),

    NAIL_ROOFING_BRASS_SMALL        (Nail(Roofing, 20), Brass, 10),
    NAIL_ROOFING_BRASS_MEDIUM       (Nail(Roofing, 50), Brass, 20),
    NAIL_ROOFING_BRASS_LARGE        (Nail(Roofing, 100), Brass, 35),
    NAIL_FRAMING_STEEL_SMALL        (Nail(Framing, 50), Steel, 15),
    NAIL_FRAMING_STEEL_MEDIUM       (Nail(Framing, 100), Steel, 25),
    NAIL_FRAMING_STEEL_LARGE        (Nail(Framing, 200), Steel, 45)
}

pub const STARTER_BUNDLE: ConstBundle<5> = ConstBundle::new([
    HAMMER_FRAMING_STEEL_LIGHT,
    NAIL_FRAMING_STEEL_LARGE,
    NAIL_ROOFING_BRASS_MEDIUM,
    SAW_JIG_STEEL,
    HAMMER_SLEDGE_STEEL_HEAVY,
]);
