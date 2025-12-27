pub trait PaymentActions {
    fn pay(&mut self);
    fn submit(&mut self);
}
