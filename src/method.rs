use serde::de::{Error as DeError, Unexpected};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{self, Display};
use std::str::FromStr;

/// HTTP request methods.
///
/// [Read more](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods)
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Method {
    /// The GET method requests a representation of the specified resource. Requests using GET
    /// should only retrieve data.
    Get,

    /// The HEAD method asks for a response identical to that of a GET request, but without the response body.
    Head,

    /// The POST method is used to submit an entity to the specified resource, often causing a
    /// change in state or side effects on the server.
    Post,

    /// The PUT method replaces all current representations of the target resource with the request
    /// payload.
    Put,

    /// The DELETE method deletes the specified resource.
    Delete,

    /// The CONNECT method establishes a tunnel to the server identified by the target resource.
    Connect,

    /// The OPTIONS method is used to describe the communication options for the target resource.
    Options,

    /// The TRACE method performs a message loop-back test along the path to the target resource.
    Trace,

    /// The PATCH method is used to apply partial modifications to a resource.
    Patch,
}

impl Method {
    /// Whether a method is considered "safe", meaning the request is
    /// essentially read-only.
    ///
    /// See [the spec](https://tools.ietf.org/html/rfc7231#section-4.2.1) for more details.
    pub fn is_safe(&self) -> bool {
        matches!(
            self,
            Method::Get | Method::Head | Method::Options | Method::Trace
        )
    }
}

struct MethodVisitor;

impl<'de> Visitor<'de> for MethodVisitor {
    type Value = Method;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a HTTP method &str")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        match Method::from_str(v) {
            Ok(method) => Ok(method),
            Err(_) => Err(DeError::invalid_value(Unexpected::Str(v), &self)),
        }
    }
}

impl<'de> Deserialize<'de> for Method {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(MethodVisitor)
    }
}

impl Serialize for Method {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Get => write!(f, "GET"),
            Self::Head => write!(f, "HEAD"),
            Self::Post => write!(f, "POST"),
            Self::Put => write!(f, "PUT"),
            Self::Delete => write!(f, "DELETE"),
            Self::Connect => write!(f, "CONNECT"),
            Self::Options => write!(f, "OPTIONS"),
            Self::Trace => write!(f, "TRACE"),
            Self::Patch => write!(f, "PATCH"),
        }
    }
}

impl FromStr for Method {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_ascii_uppercase() {
            "GET" => Ok(Self::Get),
            "HEAD" => Ok(Self::Head),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "DELETE" => Ok(Self::Delete),
            "CONNECT" => Ok(Self::Connect),
            "OPTIONS" => Ok(Self::Options),
            "TRACE" => Ok(Self::Trace),
            "PATCH" => Ok(Self::Patch),
            _ => crate::bail!("Invalid HTTP method"),
        }
    }
}

impl<'a> std::convert::TryFrom<&'a str> for Method {
    type Error = crate::Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl AsRef<str> for Method {
    fn as_ref(&self) -> &str {
        match self {
            Self::Get => "GET",
            Self::Head => "HEAD",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Delete => "DELETE",
            Self::Connect => "CONNECT",
            Self::Options => "OPTIONS",
            Self::Trace => "TRACE",
            Self::Patch => "PATCH",
        }
    }
}

#[cfg(test)]
mod test {
    use super::Method;

    #[test]
    fn serde() -> Result<(), serde_json::Error> {
        assert_eq!(Method::Get, serde_json::from_str("\"GET\"")?);
        assert_eq!(Some("PATCH"), serde_json::to_value(Method::Patch)?.as_str());
        Ok(())
    }
    #[test]
    fn serde_fail() {
        serde_json::from_str::<Method>("\"ABC\"").expect_err("Did deserialize from invalid string");
    }
}
