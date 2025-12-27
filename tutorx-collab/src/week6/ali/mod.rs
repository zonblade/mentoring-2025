mod traits;
mod admin;
mod staff;

use traits::{Identity, AccessControl};
use admin::SuperAdmin;
use staff::Staff;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rbac() {
        let staff = Staff {
            username: String::from("Zkh"),
            departement: String::from("ITDEV"),
        };
        
        assert_eq!(staff.get_name(), "Zkh");
        assert_eq!(staff.get_role(), "STAFF");
        assert!(staff.can_read());
        assert!(!staff.can_delete());
    }
}