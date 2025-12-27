use super::payment::Payment;
use super::payment_actions::PaymentActions;

impl PaymentActions for Payment {
    fn pay(&mut self) {
        self.status = "paid".to_string();
        println!("Payment of {} {} processed.", self.amount, self.currency);
    }

    fn submit(&mut self) {
        if self.status == "paid" {
            self.status = "submitted".to_string();
            println!("Payment submitted.");
        } else {
            println!("Cannot submit unpaid payment.");
        }
    }
}