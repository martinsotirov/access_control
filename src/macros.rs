#[macro_export]
macro_rules! check_rights {
    ($roles:expr => $rights:tt) => (
        for right in &$rights {
            let parts = Regex::new(r"(\w+):(\w+)/(\w+)").unwrap();
            for cap in parts.captures_iter(right) {
                if !$roles.access(&cap[1], &cap[2], &cap[3]) {
                    return Err(Status::Unauthorized)
                }
            }
        }
    )
}
