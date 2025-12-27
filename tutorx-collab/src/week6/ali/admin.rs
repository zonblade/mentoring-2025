use super::traits::{Identity, AccessControl};

pub struct SuperAdmin {
    pub username: String,
    pub admin_code: u32,
}

impl Identity for SuperAdmin {
    fn get_name(&self) -> &str {
        &self.username
    }

    fn get_role(&self) -> &str {
        "SUPER_ADMIN"
    }
}

impl AccessControl for SuperAdmin {
    fn can_read(&self) -> bool { true }
    fn can_delete(&self) -> bool { true }
}