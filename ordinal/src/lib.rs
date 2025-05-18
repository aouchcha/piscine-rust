#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    // expected public fields
    pub items : Vec<(String, f32)>,
    pub receipt : Vec<f32>
}
impl Cart {
    pub fn new() -> Self {
        Cart {
            items: Vec::new(),
            receipt: Vec::new()
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        for item in &s.products {
            if item.0 == ele {
                // println!("{:?}",item);
                self.items.push(item.clone());
                self.receipt.push(item.1.clone());
            }
        }
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        // let result = Vec::new();
        // let mut cheap_one = self.items[0].1;
        // let mut top_one = self.items[0].1;
        let mut sorted_prices = &mut self.receipt;
        sorted_prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let nbr_of_free = self.items.len()/3;
        if nbr_of_free == 0 {
            return self.receipt.clone()
        }
        let mut total_price = 0.0;
        for item in &self.receipt {
            total_price += *item
        }
        let mut total_price_of_cheap = 0.0;
        for item in &self.receipt[..nbr_of_free] {
            total_price_of_cheap += *item
        }
        let new_total = total_price - total_price_of_cheap;
        let discount = new_total / total_price;

        // println!("{:?}",(cheap_one/3.0));
        for item in &mut self.receipt {
            // *item =( *item-(cheap_one/20.0) * 100.0).round() / 100.0
            *item = ((*item * discount) * 100.0).round() / 100.0;
        }
        // for item in &mut self.receipt {
        //     *item = (*item*100.0).round()/100.0
        // }
        self.receipt.clone()
    }
}