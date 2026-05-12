use crate::models::ingest_model::request_model::request_assistant::RequestAssistant;

pub struct RequestAssistantVisitor;
impl<'de> serde::de::Visitor<'de> for RequestAssistantVisitor {
    type Value = RequestAssistant;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string representing a RequestAssistant (e.g., \"add\", \"get\")")
    }
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value.to_lowercase().as_str() {
            "add" => Ok(RequestAssistant::Add),
            "get" => Ok(RequestAssistant::Get),
            "update" | "upd" => Ok(RequestAssistant::Update),
            "delete" | "del" => Ok(RequestAssistant::Delete),
            "clear" | "clr" => Ok(RequestAssistant::Clear),
            "set" => Ok(RequestAssistant::Set),
            "use" => Ok(RequestAssistant::Use),
            "config" | "cfg" => Ok(RequestAssistant::Config),
            _ => Err(E::unknown_variant(
                value,
                &[
                    "add", "get", "update", "delete", "clear", "set", "use", "config",
                ],
            )),
        }
    }
}
