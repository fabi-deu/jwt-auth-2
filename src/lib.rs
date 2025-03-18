pub mod handlers  {
    pub mod user {
        pub mod change_credentials {
            pub mod change_password;
            pub mod change_username;
        }
        pub mod refresh {
            pub mod access_token;
            pub mod refresh_token;
        }
        pub mod delete;
        pub mod new;
        pub mod login;
        pub mod auth_test;
    }
}

pub mod middleware {
    pub mod user {
        pub mod auth;
        pub mod refresh_auth;
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




pub mod route {
    use axum::{middleware, Extension, Router};
    use axum::routing::{delete, get, post, put};
    use tower::ServiceBuilder;
    use crate::handlers::user::auth_test::auth_test;
    use crate::handlers::user::change_credentials::change_password::change_password;
    use crate::handlers::user::change_credentials::change_username::change_username;
    use crate::handlers::user::delete::delete_user;
    use crate::handlers::user::login::login;
    use crate::handlers::user::new::create_new_user;
    use crate::handlers::user::refresh::access_token::refresh_access_token;
    use crate::handlers::user::refresh::refresh_token::refresh_refresh_token;
    use crate::middleware::user::auth::auth_middleware;
    use crate::middleware::user::refresh_auth::refresh_token_auth_middleware;
    use crate::models::appstate::AppstateWrapper;

    /// returns the default router with all routes
    /// - `version`: specifies the api version, for example 'v1' or 'v2'
    pub fn get_default_router(appstate: AppstateWrapper, version: &str) -> Router {
        // public routes are accessible without any authentication or authorization
        let pub_routes = Router::new()
            .route("/new", post(create_new_user))
            .route("/login", post(login))
            .route("/refresh/refresh_token", post(refresh_refresh_token))
            .with_state(appstate.clone());

        // protected routes require access-token-authentication
        let protected_routes = Router::new()
            .route("/auth_test", get(auth_test))
            .route("/delete", delete(delete_user))
            .route("/change/password", put(change_password))
            .route("/change/username", put(change_username))
            .layer(
                ServiceBuilder::new()
                    .layer(middleware::from_fn(auth_middleware))
                    .layer(Extension(appstate.clone()))
            );

        // refresh token protected routes require - as the name implies - refresh-token-authentication
        let refresh_token_protected_routes = Router::new()
            .route("/refresh/access_token", get(refresh_access_token))
            .layer(
                ServiceBuilder::new()
                    .layer(middleware::from_fn(refresh_token_auth_middleware))
                    .layer(Extension(appstate.clone()))
            );

        // put them together
        let prefix = format!("/{}/user", version);
        let app = Router::new()
            .nest(&prefix, protected_routes)
            .nest(&prefix, refresh_token_protected_routes)
            .layer(Extension(appstate.clone()))
            .nest(&prefix, pub_routes)
            .with_state(appstate);

        app
    }
}



// router


