use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub(super) fn deserialize<'de, D, G>(deserializer: D) -> Result<Option<G>, D::Error>
where
    D: Deserializer<'de>,
    G: Deserialize<'de>,
{
    let opt: G = G::deserialize(deserializer)?;
    Ok(Some(opt))
}

pub(super) fn serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Serialize + std::fmt::Debug,
    S: Serializer,
{
    match value.as_ref() {
        Some(value) => value.serialize(serializer),
        None => serializer.serialize_none(),
    }
}
