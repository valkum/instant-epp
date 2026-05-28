use std::{borrow::Cow, fmt::Debug};

use instant_xml::ToXml;

use crate::{
    common::{NoExtension, ServiceExtension, EPP_XMLNS},
    contact, domain, host,
    request::{Command, Transaction, EPP_LANG, EPP_VERSION},
};

impl Transaction<NoExtension> for Login<'_> {}

/// Type corresponding to the `<login>` tag in an EPP XML login request
#[derive(Debug, Eq, PartialEq, ToXml)]
#[xml(rename = "login", ns(EPP_XMLNS))]
pub struct Login<'a> {
    /// The username to use for the login
    #[xml(rename = "clID")]
    username: &'a str,
    /// The password to use for the login
    #[xml(rename = "pw")]
    password: &'a str,
    /// A new password which should be set
    #[xml(rename = "newPW")]
    new_password: Option<&'a str>,
    /// Data under the `<options>` tag
    options: Options<'a>,
    /// Data under the `<svcs>` tag
    #[xml(rename = "svcs")]
    services: Services<'a>,
}

impl<'a> Login<'a> {
    pub fn new(
        username: &'a str,
        password: &'a str,
        new_password: Option<&'a str>,
        ext_uris: Option<&'_ [&'a str]>,
    ) -> Self {
        Self {
            username,
            password,
            new_password,
            options: Options {
                version: EPP_VERSION.into(),
                lang: EPP_LANG.into(),
            },
            services: Services {
                obj_uris: vec![
                    host::XMLNS.into(),
                    contact::XMLNS.into(),
                    domain::XMLNS.into(),
                ],
                svc_ext: ext_uris.and_then(|uris| {
                    (!uris.is_empty()).then(|| ServiceExtension {
                        ext_uris: uris.iter().map(|&u| u.into()).collect(),
                    })
                }),
            },
        }
    }

    /// Sets the `<options>` tag data
    pub fn options(&mut self, options: Options<'a>) {
        self.options = options;
    }

    /// Sets the `<svcs>` tag data
    pub fn services(&mut self, services: Services<'a>) {
        self.services = services;
    }
}

/// The `<options>` type in EPP XML login requests
///
/// Implements the `epp:credsOptionsType` type defined in RFC 5730.
#[derive(Debug, Eq, PartialEq, ToXml)]
#[xml(rename = "options", ns(EPP_XMLNS))]
pub struct Options<'a> {
    /// The EPP version being used
    pub version: Cow<'a, str>,
    /// The language that will be used during EPP transactions
    pub lang: Cow<'a, str>,
}

impl<'a> Options<'a> {
    /// Creates a LoginOptions object with version and lang data
    pub fn build(version: &'a str, lang: &'a str) -> Self {
        Self {
            version: version.into(),
            lang: lang.into(),
        }
    }
}

/// The `<svcs>` type in EPP XML
///
/// Implements the `epp:loginSvcType` type defined in RFC 5730.
#[derive(Debug, Eq, PartialEq, ToXml)]
#[xml(rename = "svcs", ns(EPP_XMLNS))]
pub struct Services<'a> {
    /// The service URIs being used by this EPP session represented by `<objURI>` in EPP XML
    #[xml(rename = "objURI")]
    pub obj_uris: Vec<Cow<'a, str>>,
    // The `<svcExtension>` being used in this EPP session
    #[xml(rename = "svcExtension")]
    pub svc_ext: Option<ServiceExtension<'a>>,
}

impl Command for Login<'_> {
    type Response = ();
    const COMMAND: &'static str = "login";
}

#[cfg(test)]
mod tests {
    use super::Login;
    use crate::response::ResultCode;
    use crate::tests::{assert_serialized, response_from_file, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let ext_uris = Some(&["http://schema.ispapi.net/epp/xml/keyvalue-1.0"][..]);
        let object = Login::new("username", "password", Some("new-password"), ext_uris);
        assert_serialized("request/login.xml", &object);
    }

    #[test]
    fn command_no_extension() {
        let object = Login::new("username", "password", None, None);
        assert_serialized("request/login_no_extension.xml", &object);
        let object = Login::new("username", "password", None, Some(&[]));
        assert_serialized("request/login_no_extension.xml", &object);
    }

    #[test]
    fn response() {
        let object = response_from_file::<Login>("response/login.xml");
        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG);
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID);
        assert_eq!(object.tr_ids.server_tr_id, SVTRID);
    }
}
