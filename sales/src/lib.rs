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
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    // Insert item from the store into the cart
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some(product) = s.products.iter().find(|(name, _)| name == &ele) {
            self.items.push(product.clone());
        }
    }

    // Generate the receipt with the promotion applied
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        // Sort the items by price in ascending order
        let mut sorted_items = self.items.iter().map(|(_, price)| *price).collect::<Vec<f32>>();
        sorted_items.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Apply the "Buy three, get one free" promotion
        for chunk in sorted_items.chunks_mut(3) {
            if let Some(min_item) = chunk.iter_mut().min_by(|a, b| a.partial_cmp(b).unwrap()) {
                *min_item = 0.0;  // Make the cheapest one free
            }
        }

        // To avoid zero values (as per the requirement), we'll adjust the prices slightly
        let total: f32 = sorted_items.iter().sum();
        let adjusted_prices: Vec<f32> = sorted_items
            .into_iter()
            .map(|price| {
                // If price is zero, add a small amount to make it non-zero
                if price == 0.0 {
                    0.01
                } else {
                    (price * total / sorted_items.iter().sum::<f32>()).round() // Proportional adjustment
                }
            })
            .collect();

        self.receipt = adjusted_prices.clone();
        adjusted_prices
    }
}
