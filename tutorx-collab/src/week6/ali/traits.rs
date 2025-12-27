pub trait Identity {
    fn get_name(&self) -> &str;
    fn get_role(&self) -> &str;
}

pub trait AccessControl {
    fn can_read(&self) -> &bool;
    fn can_delete(&self) -> &bool;
}
