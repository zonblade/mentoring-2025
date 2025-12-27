pub struct Payment {
    pub amount: f64,
    pub currency: String,
    pub status: String,
}

impl Payment {
    pub fn new(
        amount: f64, 
        currency: String, 
        status: String
    ) -> Self {
        Self {
            amount,
            currency,
            status,
        }
    }

    pub fn is_successful(&self) -> bool {
        self.status == "successful"
    }
}