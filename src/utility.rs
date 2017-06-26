use oauthcli::url;
use oauthcli::{ OAuthAuthorizationHeader, OAuthAuthorizationHeaderBuilder, SignatureMethod };


/// hack since there is no default token
pub fn default_auth_header() -> OAuthAuthorizationHeader {
	OAuthAuthorizationHeaderBuilder::new(
	    "GET",
	    &url::Url::parse("http://tumblr.com").unwrap(),
	    "",
	    "",
	    SignatureMethod::HmacSha1
	)
	.token("", "")
    .finish()
}