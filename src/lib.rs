use std::collections::hash_set::HashSet;
use std::collections::hash_map::HashMap;

use query::Query;
use right::Right;

pub mod query;
pub mod right;
pub mod macros;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Action {
    Create,
    Read,
    Update,
    Delete,
    Custom(String)
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Possession {
    Any,
    Own,
    Custom(String)
}

pub type Rights = HashMap<String, HashSet<Right>>;

#[derive(Debug)]
pub struct AccessControl {
    rights: Rights
}

impl AccessControl {
    pub fn new() -> Self {
        AccessControl{rights: HashMap::new()}
    }

    pub fn with_rights(rights: Rights) -> Self {
        AccessControl{rights}
    }

    pub fn get_rights(&self) -> &Rights {
        &self.rights
    }

    pub fn grant<S: Into<String>>(&mut self, role: S, right: Right) {
        let grant_role = &role.into();
        match self.rights.get_mut(grant_role) {
            Some(r) => {
                r.insert(right);
            },
            None => {
                let mut rights: HashSet<Right> = HashSet::new();
                rights.insert(right);
                self.rights.insert(grant_role.clone(), rights);
            }
        }
    }

    pub fn can_role<S: Into<String>>(&self, role: S) -> Query {
        Query::new(vec![role.into()], self.rights.clone())
    }

    pub fn can_roles<I, T>(&self, roles: I) -> Query where I: IntoIterator<Item = T>, T: Into<String> {
        Query::new(roles.into_iter().map(Into::into).collect(), self.rights.clone())
    }
}

#[cfg(test)]
mod tests {
    fn init_acl() -> crate::AccessControl {
        let mut acl = crate::AccessControl::new();
        acl.grant("user", crate::Right::read_own("post"));
        acl.grant("admin", crate::Right::delete_any("post"));
        acl
    }

    #[test]
    fn can_grant_rights() {
        let acl = init_acl();
        assert_eq!(acl.get_rights().len(), 2);
    }

    #[test]
    fn can_grant_custom_rights() {
        let mut acl = crate::AccessControl::new();
        acl.grant("team_lead", crate::Right::new("publish", "team", "news"));
        assert_eq!(acl.can_role("team_lead").access("publish", "team", "news"), true);
    }

    #[test]
    fn can_check_single_role_rights() {
        let acl = init_acl();
        assert_eq!(acl.can_role("user").read_own("post"), true);
        assert_eq!(acl.can_role("user").delete_any("post"), false);
    }

    #[test]
    fn can_check_multiple_role_rights() {
        let acl = init_acl();
        assert_eq!(acl.can_roles(vec!["user", "admin"]).read_own("post"), true);
        assert_eq!(acl.can_roles(vec!["user", "admin"]).delete_any("post"), true);
    }
}
