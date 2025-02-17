use std::ops::Deref;
use std::sync::Arc;
use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use sqlx::{Pool, Postgres};
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct Appstate {
    pub(crate) db: Arc<Mutex<Pool<Postgres>>>,
    pub(crate) jwt_secret: String,
    pub(crate) cookie_secret: Key,
}

#[derive(Clone, Debug)]
pub struct AppstateWrapper(pub Arc<Appstate>);

impl Appstate {
    pub fn new(db: Pool<Postgres>, jwt_secret: String, cookie_secret: Key) -> Self {
        Self {
            db: Arc::new(Mutex::new(db)),
            jwt_secret,
            cookie_secret,
        }
    }
}


impl Deref for AppstateWrapper {
    type Target = Appstate;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRef<AppstateWrapper> for Key {
    fn from_ref(state: &AppstateWrapper) -> Self {
        state.0.cookie_secret.clone()
    }
}

impl FromRef<Appstate> for Key {
    fn from_ref(state: &Appstate) -> Self {
        state.cookie_secret.clone()
    }
}