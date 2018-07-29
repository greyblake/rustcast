#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

// Item   | Price
// -------+------
// Apple  | 0.40
// Tomato | 0.25
// Mango  | 1.80


#[derive(Debug, Clone, Copy)]
pub enum Item {
    Apple,
    Tomato,
    Mango,
}

use Item::*;

pub struct PriceList {
    apple: f64,
    tomato: f64,
    mango: f64,
}

impl PriceList {
    pub fn new() -> Self {
        Self {
            apple: 0.0,
            tomato: 0.0,
            mango: 0.0,
        }
    }

    pub fn get(&self, item: Item) -> f64 {
        match item {
            Apple => self.apple,
            Tomato => self.tomato,
            Mango => self.mango
        }
    }

    pub fn set(&mut self, item: Item, price: f64) {
        match item {
            Apple => self.apple = price,
            Tomato => self.tomato = price,
            Mango => self.mango = price
        }
    }
}


pub struct Checkout {
    price_list: PriceList,
    items: Vec<Item>
}

impl Checkout {
    pub fn new(price_list: PriceList) -> Self {
        Self {
            price_list,
            items: vec![]
        }
    }

    pub fn total(&self) -> f64 {
        self.items.iter()
            .map(|&item| self.price_list.get(item) )
            .sum()
    }

    pub fn scan(&mut self, item: Item) {
        self.items.push(item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_checkout() -> Checkout {
        let mut price_list = PriceList::new();
        price_list.set(Apple, 0.40);
        price_list.set(Tomato, 0.25);
        price_list.set(Mango, 1.80);
        Checkout::new(price_list)
    }

    fn assert_total(items: Vec<Item>, expected_total: f64) {
        let mut checkout = build_checkout();
        for &item in items.iter() {
            checkout.scan(item);
        }
        let actual_total = checkout.total();
        assert_eq!(actual_total, expected_total);
    }

    #[test]
    fn test_checkout_empty() {
        assert_total(vec![], 0.0);
    }

    #[test]
    fn test_checkout_with_1_apple() {
        assert_total(vec![Apple], 0.4);
    }

    #[test]
    fn test_checkout_with_multiple_items() {
        assert_total(
            vec![        // price   total
                Apple,   // 0.40    0.40
                Tomato,  // 0.25    0.65
                Mango,   // 1.80    2.45
                Tomato,  // 0.25    2.70
                Apple    // 0.40    3.10
            ],
            3.10
        )
    }


    #[test]
    fn test_price_list_default() {
        let price_list = PriceList::new();
        assert_eq!(price_list.get(Apple), 0.0);
        assert_eq!(price_list.get(Tomato), 0.0);
        assert_eq!(price_list.get(Mango), 0.0);
    }

    #[test]
    fn test_price_list_set_and_get_prices() {
        let mut price_list = PriceList::new();
        price_list.set(Apple, 0.40);
        price_list.set(Tomato, 0.25);
        price_list.set(Mango, 1.80);

        assert_eq!(price_list.get(Apple), 0.40);
        assert_eq!(price_list.get(Tomato), 0.25);
        assert_eq!(price_list.get(Mango), 1.80);
    }
}
