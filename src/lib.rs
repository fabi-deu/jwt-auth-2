

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
    pub mod validation;
    pub(crate) mod hashing;
}