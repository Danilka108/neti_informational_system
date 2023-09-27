use super::RoleChecker;

macro_rules! role {
    ($raw_name:literal as $v:vis $name:ident) => {
        $v struct $name;

        const RAW_ROLE_NAME: &str = concat!($raw_name);

        impl RoleChecker for $name {
            fn can_access(role: &str) -> bool {
                role == RAW_ROLE_NAME
            }
        }
    };
}

role!("ADMIN" as pub AdminRole);
