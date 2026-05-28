//! Common data types included in EPP Requests and Responses

use std::borrow::Cow;

use instant_xml::{FromXml, ToXml};

use crate::request::Extension;

pub(crate) const EPP_XMLNS: &str = "urn:ietf:params:xml:ns:epp-1.0";

#[derive(Debug, Eq, PartialEq, ToXml)]
pub struct NoExtension;

impl<'xml> FromXml<'xml> for NoExtension {
    fn matches(_: instant_xml::Id<'_>, _: Option<instant_xml::Id<'_>>) -> bool {
        false
    }

    fn deserialize<'cx>(
        _: &mut Self::Accumulator,
        _: &'static str,
        _: &mut instant_xml::Deserializer<'cx, 'xml>,
    ) -> Result<(), instant_xml::Error> {
        unreachable!()
    }

    type Accumulator = Option<Self>;
    const KIND: instant_xml::Kind = instant_xml::Kind::Element;
}

impl Extension for NoExtension {
    type Response = Self;
}

/// The `<svcExtension>` type in EPP XML
#[derive(Debug, Eq, FromXml, PartialEq, ToXml)]
#[xml(rename = "svcExtension", ns(EPP_XMLNS))]
pub struct ServiceExtension<'a> {
    /// The service extension URIs being represented by `<extURI>` in EPP XML
    #[xml(rename = "extURI")]
    pub ext_uris: Vec<Cow<'a, str>>,
}
