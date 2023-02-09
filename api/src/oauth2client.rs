use oauth2::{
    basic::BasicClient, url::Url, AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenUrl,
};

#[derive(Clone, Debug)]
pub struct OAuth2Client {
    pub client: BasicClient,
    pub redirect_uri: String,
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub scopes: Vec<String>,
}

impl OAuth2Client {
    pub fn new(
        client_id: String,
        client_secret: String,
        redirect_uri: String,
        auth_url: String,
        token_url: String,
        scopes: Vec<String>,
    ) -> Self {
        let client = BasicClient::new(
            ClientId::new(client_id.clone()),
            Some(ClientSecret::new(client_secret.clone())),
            AuthUrl::new(auth_url.clone()).expect("invalid auth url"),
            Some(TokenUrl::new(token_url.clone()).expect("invalid token url")),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_uri.clone()).expect("invalid redirect uri"));

        Self {
            client,
            redirect_uri,
            client_id,
            client_secret,
            auth_url,
            token_url,
            scopes,
        }
    }

    pub fn create_pkce_auth_url(&self) -> PkceAuth {
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
        let (auth_url, csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .set_pkce_challenge(pkce_code_challenge)
            .add_scopes(
                self.scopes
                    .iter()
                    .map(|s| Scope::new(s.to_string()))
                    .collect::<Vec<_>>(),
            )
            .url();
        PkceAuth {
            auth_url,
            csrf_token,
            verifier: pkce_code_verifier,
        }
    }
}

#[derive(Debug)]
pub struct PkceAuth {
    pub verifier: PkceCodeVerifier,
    pub auth_url: Url,
    pub csrf_token: CsrfToken,
}
