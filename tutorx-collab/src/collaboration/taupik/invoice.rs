struct Invoice {
    id: String,
    invoice_number: String,
    date: String,
    items: Vec<InvoiceItem>,
}

struct InvoiceItem {
    product: Product,
    quantity: f64,
    unit_price: f64,
}

struct Product {
    id: String,
    name: String,
}

impl Invoice {
    fn add_item(&mut self, product: Product, quantity: f64, unit_price: f64){
        self.items.push(InvoiceItem{
            product,
            quantity,
            unit_price,
        });
    }

    fn new(invoice_number: String, date: String) -> Self {
        Self { 
            id: format!("INV-"), 
            invoice_number, 
            date, 
            items: Vec::new(),
        }
    }
}

fn main(){
    // Data Product
    let _laptop = Product {
        id: "89a29924-3109-433d-b1f5-a36deed103b3".to_string(),
        name: "Macbook Pro 2020".to_string(),
    };

    // Buat Invoice Baru
    let mut invoice = Invoice::new(
        "INV-2024-001".to_string(),
        "20 Desember 2025".to_string(),
    );

    invoice.add_item(_laptop, 10, 17000000);

}