//! `GET /_matrix/identity/*/validate/msisdn/submitToken`
//!
//! Endpoint to validate the ownership of a phone number by the end user.

pub mod v2 {
    //! `/v2/` ([spec])
    //!
    //! [spec]: https://spec.matrix.org/v1.2/identity-service-api/#get_matrixidentityv2validatemsisdnsubmittoken

    use ruma_api::ruma_api;
    use ruma_identifiers::{ClientSecret, SessionId};

    ruma_api! {
        metadata: {
            description: "Validate ownership of an email address.",
            method: GET,
            name: "validate_email_by_end_user",
            stable_path: "/_matrix/identity/v2/validate/msisdn/submitToken",
            authentication: AccessToken,
            rate_limited: false,
            added: 1.0,
        }

        request: {
            /// The session ID, generated by the `requestToken` call.
            #[ruma_api(query)]
            pub sid: &'a SessionId,

            /// The client secret that was supplied to the `requestToken` call.
            #[ruma_api(query)]
            pub client_secret: &'a ClientSecret,

            /// The token generated by the `requestToken` call and sent to the user.
            #[ruma_api(query)]
            pub token: &'a str,
        }

        #[derive(Default)]
        response: {}
    }

    impl<'a> Request<'a> {
        /// Create a new `Request` with the given session ID, client secret and token.
        pub fn new(sid: &'a SessionId, client_secret: &'a ClientSecret, token: &'a str) -> Self {
            Self { sid, client_secret, token }
        }
    }

    impl Response {
        /// Create a new empty `Response`.
        pub fn new() -> Self {
            Self {}
        }
    }
}
