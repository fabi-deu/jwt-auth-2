pub mod handlers  {
    pub mod user {
        pub mod new;
        pub mod login;
        pub mod auth_test;
    }
}

pub mod middleware {
    pub mod user {
        pub mod auth;
    }
}

pub mod models {
    pub mod user;
    pub mod auth_user;
    pub mod user_permission;
    pub mod appstate;
}

pub(crate) mod util {
    pub(crate) mod cookies;
    pub(crate) mod jwt {
        pub(crate) mod general;
        pub(crate) mod access_token;
        pub(crate) mod refresh_token;
        pub(crate) mod claims;
    }

    pub mod validation;
    pub(crate) mod hashing;
}
