use serde::Serialize;
use serde_json::Value;

fn mask_tail(s: &str) -> String {
    if s.len() <= 4 { "•".repeat(s.len()) }
    else { "•".repeat(3) + &s[s.len()-4..] }
}

pub fn redact_token(tok: &Option<String>, show: &bool) -> Option<String> {
    match (tok.as_deref(), show) {
        (None, _)        => None,                    // omit in JSON
        (Some(t), true)  => Some(t.to_owned()),      // show full
        (Some(t), false) => Some(mask_tail(t)),      // masked
    }
}

// 2) trait for “JSON view” that respects the flag
pub trait RedactableData {
    fn to_redacted_json(&self, show_secrets: &bool) -> Value;
    fn to_redacted_table_row(&self, show_secrets: &bool) -> Vec<String>;
    fn to_redacted_table_rows(&self, show_secrets: &bool) -> Vec<Vec<String>> {
        vec![self.to_redacted_table_row(show_secrets)]
    }
}