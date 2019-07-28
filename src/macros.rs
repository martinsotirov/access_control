#[macro_export]
macro_rules! check_rights {
    ($roles:expr => $rights:tt) => (
        for right in &$rights {
            let right = Right::from_pattern(right);
            if !$roles.execute_right(right) {
                return Err(Status::Unauthorized)
            }
        }
    )
}

#[cfg(test)]
mod tests {
    use crate::AccessControl;
    use crate::Right;
    use crate::check_rights;
    use rocket::http::Status;

    fn init_acl() -> AccessControl {
        let mut acl = AccessControl::new();
        acl.grant("user", Right::read_own("post"));
        acl.grant("admin", Right::delete_any("post"));
        acl
    }

    fn authorized() -> Result<&'static str, Status>  {
        let acl = init_acl();
        check_rights!(acl.can_role("admin") => ["delete:any/post"]);
        Ok("Success")
    }

    fn unauthorized() -> Result<&'static str, Status>  {
        let acl = init_acl();
        check_rights!(acl.can_role("user") => ["delete:any/post"]);
        Ok("Success")
    }

    #[test]
    fn can_check_rights_via_macro() {
        assert_eq!(authorized(), Ok("Success"));
        assert_eq!(unauthorized(), Err(Status::Unauthorized));
    }
}
