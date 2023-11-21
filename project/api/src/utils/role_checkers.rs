use app::user::Role;

pub trait RoleChecker {
    fn can_access(role: Role) -> bool;
}

macro_rules! gen_composed_role_checkers {
    ($head:ident, $($tail:ident),+) => {
        impl<$head: RoleChecker, $($tail: RoleChecker),+> RoleChecker for ($head, $($tail),+) {
            fn can_access(role: Role) -> bool {
                if $head::can_access(role) {
                    return true;
                }
                $(
                    if $tail::can_access(role) {
                        return true;
                    }
                )+
                return false;
            }
        }

        gen_composed_role_checkers!($($tail),+);
    };
    ($element:ident) => {
        impl<$element: RoleChecker> RoleChecker for ($element,) {
            fn can_access(role: Role) -> bool {
                if $element::can_access(role) {
                    return true;
                }

                return false;
            }
        }
    };
}

gen_composed_role_checkers!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);

pub struct Admin;

impl RoleChecker for Admin {
    fn can_access(role: Role) -> bool {
        matches!(role, Role::Admin)
    }
}
