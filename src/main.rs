use crate::field::FieldElement;
use crate::params::{DU, Q};

mod field;
mod params;
mod ring;

fn main() {
    assert!(DU < 12);
    for x in 0..Q {
        let t = FieldElement::from(x).compress::<DU>();
        assert!(u16::from(t) <= (1 << DU)-1)
    }
}
