pub struct Book {
    pub title: String,
    pub price: f64,
    pub final_price: f64
}

fn get_book_price(title: String, price: f64) -> Result<Book, String>{
    if price < 0.0 {
        return Err("Price can't be negative".to_string());
    }

    let taxed_price = price * (price / 10.0);
    Ok(
        Book {
            title,
            price,
            final_price: taxed_price
        }
    )
}

fn main() {
    // gunakan match
    let brevity_book: Result<Book, String> = get_book_price("Write with Brevity".to_string(), 10.0);
    match brevity_book {
        Ok(book) => {
            println!("Book: {}", book.title);
            println!("Price: ${:.2}", book.price);
            println!("Final Price: ${:.2}", book.final_price);
        }
        Err(e) => {
            println!("err get book price: {}", e);
        }
    }
}