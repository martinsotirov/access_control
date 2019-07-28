use regex::Regex;
use crate::Action;
use crate::Possession;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Right {
    pub action: Action,
    pub resource: String,
    pub possession: Possession
}

impl Right {
    pub fn new<S: Into<String>>(action: S, possession: S, resource: S) -> Self {
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

        Right{
            action,
            possession,
            resource: resource.into()
        }
    }

    pub fn from_pattern<S: AsRef<str>>(pattern: S) -> Self {
        let re = Regex::new(r"(\w+):(\w+)/(\w+)").unwrap();
        let caps = re.captures(pattern.as_ref()).unwrap();
        Right::new(&caps[1], &caps[2], &caps[3])
    }

    pub fn create_own<S: Into<String>>(resource: S) -> Self {
        Right{action: Action::Create, resource: resource.into(), possession: Possession::Own}
    }

    pub fn create_any<S: Into<String>>(resource: S) -> Self {
        Right{action: Action::Create, resource: resource.into(), possession: Possession::Any}
    }

    pub fn read_own<S: Into<String>>(resource: S) -> Self {
        Right{action: Action::Read, resource: resource.into(), possession: Possession::Own}
    }

    pub fn read_any<S: Into<String>>(resource: S) -> Self {
        Right{action: Action::Read, resource: resource.into(), possession: Possession::Any}
    }

    pub fn update_own<S: Into<String>>(resource: S) -> Self {
        Right{action: Action::Update, resource: resource.into(), possession: Possession::Own}
    }

    pub fn update_any<S: Into<String>>(resource: S) -> Self {
        Right{action: Action::Update, resource: resource.into(), possession: Possession::Any}
    }

    pub fn delete_own<S: Into<String>>(resource: S) -> Self {
        Right{action: Action::Delete, resource: resource.into(), possession: Possession::Own}
    }

    pub fn delete_any<S: Into<String>>(resource: S) -> Self {
        Right{action: Action::Delete, resource: resource.into(), possession: Possession::Any}
    }
}

#[cfg(test)]
mod tests {
    use crate::Rights;
    use crate::Action;
    use crate::Possession;
    use crate::right::Right;
    use crate::query::Query;
    use std::collections::hash_set::HashSet;

    #[test]
    fn can_create_right() {
        let right = Right::new("publish", "team", "news");
        assert_eq!(right.action, Action::Custom(String::from("publish")));
        assert_eq!(right.possession, Possession::Custom(String::from("team")));
        assert_eq!(right.resource, String::from("news"));
    }

    #[test]
    fn can_make_create_own_right() {
        let right = Right::create_own("post");
        assert_eq!(right.action, Action::Create);
        assert_eq!(right.possession, Possession::Own);
        assert_eq!(right.resource, String::from("post"));
    }

    #[test]
    fn can_make_right_from_pattern() {
        let right = Right::from_pattern("delete:own/profile");
        assert_eq!(right.action, Action::Delete);
        assert_eq!(right.possession, Possession::Own);
        assert_eq!(right.resource, String::from("profile"));
    }

    #[test]
    fn can_make_create_any_right() {
        let right = Right::create_any("post");
        assert_eq!(right.action, Action::Create);
        assert_eq!(right.possession, Possession::Any);
        assert_eq!(right.resource, String::from("post"));
    }

    #[test]
    fn can_make_read_own_right() {
        let right = Right::read_own("post");
        assert_eq!(right.action, Action::Read);
        assert_eq!(right.possession, Possession::Own);
        assert_eq!(right.resource, String::from("post"));
    }

    #[test]
    fn can_make_read_any_right() {
        let right = Right::read_any("post");
        assert_eq!(right.action, Action::Read);
        assert_eq!(right.possession, Possession::Any);
        assert_eq!(right.resource, String::from("post"));
    }

    #[test]
    fn can_make_update_own_right() {
        let right = Right::update_own("post");
        assert_eq!(right.action, Action::Update);
        assert_eq!(right.possession, Possession::Own);
        assert_eq!(right.resource, String::from("post"));
    }

    #[test]
    fn can_make_update_any_right() {
        let right = Right::update_any("post");
        assert_eq!(right.action, Action::Update);
        assert_eq!(right.possession, Possession::Any);
        assert_eq!(right.resource, String::from("post"));
    }

    #[test]
    fn can_make_delete_own_right() {
        let right = Right::delete_own("post");
        assert_eq!(right.action, Action::Delete);
        assert_eq!(right.possession, Possession::Own);
        assert_eq!(right.resource, String::from("post"));
    }

    #[test]
    fn can_make_delete_any_right() {
        let right = Right::delete_any("post");
        assert_eq!(right.action, Action::Delete);
        assert_eq!(right.possession, Possession::Any);
        assert_eq!(right.resource, String::from("post"));
    }
}
