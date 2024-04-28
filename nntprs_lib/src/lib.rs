pub mod errors;

mod private {
    pub trait Sealed {}
    impl Sealed for super::UnconfiguredAuthentication {}
    impl Sealed for super::LoginAuthenticated {}
    impl Sealed for super::LoginAnonymous {}
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct NewsLogin {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub enum ConnectionType {
    Plain,
    StarTls,
    Tls,
    #[default]
    Unspecified,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct UnconfiguredAuthentication;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct LoginAuthenticated(NewsLogin);

#[derive(Debug, Default, Eq, PartialEq)]
pub struct LoginAnonymous;

pub trait AuthenticationStatus: private::Sealed {}

impl AuthenticationStatus for UnconfiguredAuthentication {}
impl AuthenticationStatus for LoginAuthenticated {}
impl AuthenticationStatus for LoginAnonymous {}

pub struct Connection<A: AuthenticationStatus = UnconfiguredAuthentication> {
    connection_type: ConnectionType,
    auth: A,
}

impl Connection {
    pub fn new() -> Self {
        Connection {
            auth: UnconfiguredAuthentication,
            connection_type: ConnectionType::default(),
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.auth != UnconfiguredAuthentication
    }

    pub fn is_plain(&self) -> bool {
        self.connection_type == ConnectionType::Plain
    }

    pub fn is_tls(&self) -> bool {
        self.connection_type == ConnectionType::Tls
    }
}

impl Default for Connection {
    fn default() -> Self {
        Self::new()
    }
}

impl Connection<LoginAuthenticated> {
    pub fn get_login(&self) -> Option<&NewsLogin> {
        match self.auth {
            LoginAuthenticated(ref login) => Some(login),
        }
    }
}

pub struct NetNewsItem<'x> {
    pub newsgroup: Option<String>,
    pub parsed: Option<mail_parser::Message<'x>>,
    pub raw: Option<String>,
}

impl<'x> NetNewsItem<'x> {
    pub fn new(newsgroup: &str, raw: &'x str) -> Self {
        let parsed = mail_parser::MessageParser::new().parse(raw.as_bytes());

        NetNewsItem {
            newsgroup: Some(newsgroup.to_string()),
            parsed,
            raw: Some(raw.to_string()),
        }
    }

    pub fn is_parsed(&self) -> bool {
        self.parsed.is_some()
    }

    pub fn raw(&self) -> Option<&str> {
        self.raw.as_deref()
    }

    pub fn news_group(&self) -> Option<&str> {
        self.newsgroup.as_deref()
    }

    pub fn parsed(&self) -> Option<&mail_parser::Message> {
        self.parsed.as_ref()
    }
}
