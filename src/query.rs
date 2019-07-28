use crate::Action;
use crate::Possession;
use crate::Rights;
use crate::right::Right;

#[derive(Debug)]
pub struct Query {
    roles: Vec<String>, rights: Rights
}

impl Query {
    pub fn new(roles: Vec<String>, rights: Rights) -> Self {
        Query{roles, rights}
    }

    pub fn check(&self, action: Action, possession: Possession, resource: String) -> bool {
        let check_role = Right{action, resource, possession};
        for role in &self.roles {
            if let Some(r) =  self.rights.get(role) {
                if r.contains(&check_role.clone()) {
                    return true
                }
            }
        }
        false
    }

    pub fn execute_right(&self, right: Right) -> bool {
        self.check(right.action, right.possession, right.resource)
    }

    pub fn access<S: Into<String>>(&self, action: S, possession: S, resource: S) -> bool {
        let action = match action.into().as_ref() {
            "create" => Action::Create,
            "read" => Action::Read,
            "update" => Action::Update,
            "delete" => Action::Delete,
            custom => Action::Custom(String::from(custom))
        };

        let possession = match possession.into().as_ref() {
            "own" => Possession::Own,
            "any" => Possession::Any,
            custom => Possession::Custom(String::from(custom))
        };

        self.check(action, possession, resource.into())
    }

    pub fn create_own<S: Into<String>>(&self, resource: S) -> bool {
        self.check(Action::Create, Possession::Own, resource.into())
    }

    pub fn create_any<S: Into<String>>(&self, resource: S) -> bool {
        self.check(Action::Create, Possession::Any, resource.into())
    }

    pub fn create<S: Into<String>>(&self, resource: S) -> bool {
        self.create_any(resource)
    }

    pub fn read_own<S: Into<String>>(&self, resource: S) -> bool {
        self.check(Action::Read, Possession::Own, resource.into())
    }

    pub fn read_any<S: Into<String>>(&self, resource: S) -> bool {
        self.check(Action::Read, Possession::Any, resource.into())
    }

    pub fn read<S: Into<String>>(&self, resource: S) -> bool {
        self.read_any(resource)
    }

    pub fn update_own<S: Into<String>>(&self, resource: S) -> bool {
        self.check(Action::Update, Possession::Own, resource.into())
    }

    pub fn update_any<S: Into<String>>(&self, resource: S) -> bool {
        self.check(Action::Update, Possession::Any, resource.into())
    }

    pub fn update<S: Into<String>>(&self, resource: S) -> bool {
        self.update_any(resource)
    }

    pub fn delete_own<S: Into<String>>(&self, resource: S) -> bool {
        self.check(Action::Delete, Possession::Own, resource.into())
    }

    pub fn delete_any<S: Into<String>>(&self, resource: S) -> bool {
        self.check(Action::Delete, Possession::Any, resource.into())
    }

    pub fn delete<S: Into<String>>(&self, resource: S) -> bool {
        self.delete_any(resource)
    }
}

#[cfg(test)]
mod tests {
    use crate::Rights;
    use crate::right::Right;
    use crate::query::Query;
    use std::collections::hash_set::HashSet;

    fn init_query(right: Right) -> Query {
        let mut query = Query{roles: vec![String::from("user")], rights: Rights::new()};
        let mut rights: HashSet<Right> = HashSet::new();
        rights.insert(right);
        query.rights.insert(String::from("user"), rights);
        query
    }

    #[test]
    fn can_check_custom_access() {
        let query = init_query(Right::new("publish", "team", "news"));
        assert_eq!(query.access("publish", "team", "news"), true);
    }

    #[test]
    fn can_check_custom_access_by_right() {
        let query = init_query(Right::new("create", "any", "page"));
        assert_eq!(query.execute_right(Right::create_any("page")), true);
    }

    #[test]
    fn can_check_create_own() {
        let query = init_query(Right::create_own("post"));
        assert_eq!(query.create_own("post"), true);
    }

    #[test]
    fn can_check_create_any() {
        let query = init_query(Right::create_any("post"));
        assert_eq!(query.create_any("post"), true);
    }

    #[test]
    fn can_check_create() {
        let query = init_query(Right::create_any("post"));
        assert_eq!(query.create("post"), true);
    }

    #[test]
    fn can_check_read_own() {
        let query = init_query(Right::read_own("post"));
        assert_eq!(query.read_own("post"), true);
    }

    #[test]
    fn can_check_read_any() {
        let query = init_query(Right::read_any("post"));
        assert_eq!(query.read_any("post"), true);
    }

    #[test]
    fn can_check_read() {
        let query = init_query(Right::read_any("post"));
        assert_eq!(query.read("post"), true);
    }

    #[test]
    fn can_check_update_own() {
        let query = init_query(Right::update_own("post"));
        assert_eq!(query.update_own("post"), true);
    }

    #[test]
    fn can_check_update_any() {
        let query = init_query(Right::update_any("post"));
        assert_eq!(query.update_any("post"), true);
    }

    #[test]
    fn can_check_update() {
        let query = init_query(Right::update_any("post"));
        assert_eq!(query.update("post"), true);
    }

    #[test]
    fn can_check_delete_own() {
        let query = init_query(Right::delete_own("post"));
        assert_eq!(query.delete_own("post"), true);
    }

    #[test]
    fn can_check_delete_any() {
        let query = init_query(Right::delete_any("post"));
        assert_eq!(query.delete_any("post"), true);
    }

    #[test]
    fn can_check_delete() {
        let query = init_query(Right::delete_any("post"));
        assert_eq!(query.delete("post"), true);
    }
}
