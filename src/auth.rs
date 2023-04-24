use actix_web::dev::ServiceRequest;
use actix_web::web::Data;
use actix_web::Error;
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::headers::www_authenticate::basic::Basic;

pub(crate) struct ApiCredentials {
    pub user_id: String,
    pub password: String,
}

impl ApiCredentials {
    pub fn from_env() -> Self {
        ApiCredentials {
            user_id: std::env::var("API_USERNAME").expect("Forgot to set API_USERNAME."),
            password: std::env::var("API_PASSWORD").expect("Forgot to set API_PASSWORD."),
        }
    }

    pub fn validate(&self, user_credentials: BasicAuth) -> bool {
        user_credentials.password().map_or(false, |user_password| {
            user_credentials.user_id() == self.user_id && user_password == self.password
        })
    }

    #[cfg(test)]
    pub fn new(user_id: &str, password: &str) -> Self {
        ApiCredentials {
            user_id: user_id.to_owned(),
            password: password.to_owned(),
        }
    }
}

pub(crate) async fn basic_auth_validator(
    request: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let api_credentials = request
        .app_data::<Data<ApiCredentials>>()
        .expect("App should've panicked if no credentials provided.");
    let challenge = Basic::with_realm("Access to the API");

    match api_credentials.validate(credentials) {
        true => Ok(request),
        false => Err((AuthenticationError::new(challenge).into(), request)),
    }
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use actix_web::{get, web::Data};
    use actix_web_httpauth::{
        headers::authorization::{Authorization, Basic},
        middleware::HttpAuthentication,
    };

    use super::{basic_auth_validator, ApiCredentials};

    #[get("/")]
    async fn test_handler() -> &'static str {
        "Hello, world!"
    }

    #[actix_web::test]
    async fn test_auth() {
        let app = actix_web::test::init_service(
            actix_web::App::new()
                .app_data(Data::new(ApiCredentials::new("test", "test")))
                .service(test_handler)
                .wrap(HttpAuthentication::basic(basic_auth_validator)),
        )
        .await;

        let basic_auth_header = Basic::new("test", Some("test"));
        let req = actix_web::test::TestRequest::get()
            .uri("/")
            .insert_header(Authorization::<Basic>::from(basic_auth_header))
            .to_request();

        let resp = actix_web::test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    #[should_panic(expected = "Forgot to set API_USERNAME.")]
    async fn test_panics_when_no_api_credentials_set() {
        actix_web::test::init_service(
            actix_web::App::new()
                .service(test_handler)
                .app_data(Data::new(ApiCredentials::from_env()))
                .wrap(HttpAuthentication::basic(basic_auth_validator)),
        )
        .await;
    }

    #[actix_web::test]
    async fn test_returns_401_if_no_credentials_provided() {
        let app = actix_web::test::init_service(
            actix_web::App::new()
                .app_data(Data::new(ApiCredentials::new("test", "test")))
                .service(test_handler)
                .wrap(HttpAuthentication::basic(basic_auth_validator)),
        )
        .await;

        let req = actix_web::test::TestRequest::get().uri("/").to_request();

        let resp = actix_web::test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[actix_web::test]
    async fn test_returns_401_if_incorrect_credentials_provided() {
        let app = actix_web::test::init_service(
            actix_web::App::new()
                .app_data(Data::new(ApiCredentials::new("test", "test")))
                .service(test_handler)
                .wrap(HttpAuthentication::basic(basic_auth_validator)),
        )
        .await;

        let basic_auth_header = Basic::new("test", Some("wrong_password"));
        let req = actix_web::test::TestRequest::get()
            .uri("/")
            .insert_header(Authorization::<Basic>::from(basic_auth_header))
            .to_request();

        let resp = actix_web::test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
}
