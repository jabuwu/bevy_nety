use ron::{de::from_str, ser::to_string};
use serde::{Deserialize, Serialize};

// use these serializers everywhere, in case we want to swap them out eventually
// TODO: use binary serializer instead of text based ron serializer?
// TODO: remove the need for dangerous unwraps

pub(crate) fn deserialize<'a, T>(string: &'a str) -> T
where
    T: Deserialize<'a>,
{
    from_str(string).unwrap()
}

pub(crate) fn serialize<T>(data: &T) -> String
where
    T: ?Sized + Serialize,
{
    to_string(data).unwrap()
}
