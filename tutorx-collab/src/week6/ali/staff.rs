use super::traits::{Identity, AccessControl};

pub struct Staff {
    pub username: String,
    pub departement: String,
}

impl Identity for Staff {
    fn get_name(&self) -> &str {
        &self.username
    }

    fn get_role(&self) -> &str {
        "STAFF"
    }
}

impl AccessControl for Staff {
    fn can_read(&self) -> bool { true }
    fn can_delete(&self) -> bool { false }
}