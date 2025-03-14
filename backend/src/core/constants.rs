pub mod openapi {
    pub const DEFAULT_SECURITY_SCHEME: &'static str = "Session ID";

    pub mod tags {
        pub const AUTH_TAG: &'static str = "Auth";
        pub const FORGOT_PASSWORD_TAG: &'static str = "Forgot Password";
        pub const USER_TAG: &'static str = "Users";

        pub mod admin {
            pub const USER_TAG: &'static str = "Users (Admin)";
        }
    }
}

pub mod session {
    pub mod headers {
        pub const SESSION_HEADER_KEY: &'static str = "X-Session-Id";
    }
}

pub mod admin {
    pub const ADMIN_EMAIL: &'static str = "admin@localhost";
}
