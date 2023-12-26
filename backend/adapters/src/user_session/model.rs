use app::user_session;
use sqlx::FromRow;
use utils::entity::Id;

#[derive(Clone, Debug, FromRow)]
#[sea_query::enum_def]
pub struct UserSessions {
    pub user_id: i32,
    pub metadata: String,
    pub refresh_token: String,
    pub expires_at: i64,
}

impl From<UserSessions> for user_session::Entity {
    fn from(value: UserSessions) -> Self {
        user_session::Entity {
            id: Id::new(user_session::Id {
                metadata: value.metadata,
                user_id: Id::new(value.user_id),
            }),
            refresh_token: value.refresh_token,
            expires_at: user_session::SecondsFromUnixEpoch {
                seconds: user_session::Seconds {
                    val: value.expires_at.try_into().unwrap(),
                },
            },
        }
    }
}
