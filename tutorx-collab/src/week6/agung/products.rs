use super::BusinessLogic;

// local scoped struct
pub struct Product {
    pub id: i32
}

pub trait Products {
    fn get_items()->Vec<Product>;
}

impl Products for BusinessLogic {
    fn get_items()->Vec<Product>{
        vec![
            Product{
                id:1
            }
        ]
    }
}