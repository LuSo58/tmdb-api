use std::borrow::Cow;

/// Command to get details of a company
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::client::Client;
/// use tmdb_api::client::reqwest::ReqwestExecutor;
/// use tmdb_api::company::alternative_names::CompanyAlternativeNames;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::<ReqwestExecutor>::new("this-is-my-secret-token".into());
///     let cmd = CompanyAlternativeNames::new(1);
///     let result = cmd.execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {:#?}", res),
///         Err(err) => eprintln!("error: {:?}", err),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct CompanyAlternativeNames {
    /// ID of the Company
    pub company_id: u64,
}

impl CompanyAlternativeNames {
    pub fn new(company_id: u64) -> Self {
        Self { company_id }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CompanyAlternativeName {
    pub name: String,
    #[serde(
        deserialize_with = "crate::util::empty_string::deserialize",
        rename = "type"
    )]
    pub kind: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CompanyAlternativeNamesResult {
    pub id: u64,
    pub results: Vec<CompanyAlternativeName>,
}

impl crate::prelude::Command for CompanyAlternativeNames {
    type Output = CompanyAlternativeNamesResult;

    fn path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("/company/{}/alternative_names", self.company_id))
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::CompanyAlternativeNames;
    use crate::client::Client;
    use crate::client::reqwest::ReqwestExecutor;
    use crate::prelude::Command;
    use mockito::Matcher;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/company/1/alternative_names")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/company-alternative-names.json"))
            .create_async()
            .await;

        let result = CompanyAlternativeNames::new(1)
            .execute(&client)
            .await
            .unwrap();
        assert_eq!(result.id, 1);
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/company/1/alternative_names")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;

        let err = CompanyAlternativeNames::new(1)
            .execute(&client)
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 7);
    }

    #[tokio::test]
    async fn resource_not_found() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/company/1/alternative_names")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;

        let err = CompanyAlternativeNames::new(1)
            .execute(&client)
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::CompanyAlternativeNames;
    use crate::client::Client;
    use crate::client::reqwest::ReqwestExecutor;
    use crate::prelude::Command;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestExecutor>::new(secret);

        let result = CompanyAlternativeNames::new(1)
            .execute(&client)
            .await
            .unwrap();
        assert_eq!(result.id, 1);
    }
}
