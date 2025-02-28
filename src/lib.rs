pub mod handlers  {
    pub mod user {
        pub mod new;
    }
}

pub mod models {
    pub mod user;
    pub mod user_permission;
    pub mod appstate;
}

pub mod util {
    pub(crate) mod jwt {
        pub(crate) mod access_token;
        pub(crate) mod refresh_token;
        pub(crate) mod claims;
    }

    pub mod validation;
    pub(crate) mod hashing;
}
