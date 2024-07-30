pub enum Item {
    First,
    Second,
    Third,
}

impl Item {
    pub fn index(&self) -> usize {
        match self {
            Item::First => 0,
            Item::Second => 1,
            Item::Third => 2,
        }
    }
}

pub trait CommonOps: Sized {
    fn default_values() -> Self;

    fn get_item(&self, item: Item) -> f64;

    fn set_item(&mut self, item: Item, value: f64);

    fn is_default(&self) -> bool {
        let default: Self = Self::default_values();
        self.get_item(Item::First) == default.get_item(Item::First)
            && self.get_item(Item::Second) == default.get_item(Item::Second)
            && self.get_item(Item::Third) == default.get_item(Item::Third)
    }

    fn sum(&self) -> f64 {
        self.get_item(Item::First) + self.get_item(Item::Second) + self.get_item(Item::Third)
    }
}

#[derive(Debug, PartialEq)]
pub struct Tuple(u32, f32, f64);

impl CommonOps for Tuple {
    fn default_values() -> Self {
        Self(0, 0.0, 0.0)
    }

    fn get_item(&self, item: Item) -> f64 {
        match item {
            Item::First => self.0 as _,
            Item::Second => self.1 as _,
            Item::Third => self.2,
        }
    }

    fn set_item(&mut self, item: Item, value: f64) {
        match item {
            Item::First => self.0 = value as _,
            Item::Second => self.1 = value as _,
            Item::Third => self.2 = value,
        };
    }
}

#[derive(Debug, PartialEq)]
pub struct Array([f64; 3]);

impl CommonOps for Array {
    fn default_values() -> Self {
        Self([0.0; 3])
    }

    fn get_item(&self, item: Item) -> f64 {
        self.0[item.index()]
    }

    fn set_item(&mut self, item: Item, value: f64) {
        self.0[item.index()] = value
    }
}

#[cfg(test)]
mod test {

    use super::*;

    fn sum_should_work_correctly(mut v: impl CommonOps) {
        v.set_item(Item::First, 1.0);
        v.set_item(Item::Second, 2.0);
        v.set_item(Item::Third, 3.0);
        assert!(v.sum() == 6.0)
    }

    fn is_default_should_work_correctly<T: CommonOps + PartialEq>(v: T) {
        let d: T = T::default_values();
        assert!(d.is_default());
        if v == d {
            assert!(v.is_default())
        } else {
            assert!(!v.is_default())
        }
    }

    #[test]
    pub fn sum_should_work_correctly_for_tuple() {
        sum_should_work_correctly(Tuple::default_values())
    }

    #[test]
    pub fn sum_should_work_correctly_for_array() {
        sum_should_work_correctly(Array::default_values())
    }

    #[test]
    pub fn is_default_should_work_correctly_for_tuple() {
        is_default_should_work_correctly(Tuple::default_values());
        is_default_should_work_correctly(Tuple(1, 2_f32, 3_f64));
    }

    #[test]
    pub fn is_default_should_work_correctly_for_array() {
        is_default_should_work_correctly(Array::default_values());
        is_default_should_work_correctly(Array([1_f64, 2_f64, 3.0]));
    }
}
