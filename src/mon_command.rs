use std::collections::HashMap;

use serde_json;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_a_mon_command() {
        let command = MonCommand::new()
            .with_prefix("osd set")
            .with("key", "osdout");

        let actual: HashMap<String, String> = serde_json::from_str(&command.as_json()).unwrap();
        let expected: HashMap<String, String> =
            serde_json::from_str(r#"{"prefix":"osd set","format":"json","key":"osdout"}"#).unwrap();

        assert_eq!(expected, actual);
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum MaybeList<'a> {
    String(&'a str),
    List(Vec<&'a str>)
}

impl<'a> From<&'a String> for MaybeList<'a> {
    fn from(s: &'a String) -> MaybeList<'a> {
        MaybeList::String(s.as_ref())
    }
}

impl<'a> From<&'a str> for MaybeList<'a> {
    fn from(s: &'a str) -> MaybeList<'a> {
        MaybeList::String(s)
    }
}

impl<'a> From<Vec<&'a String>> for MaybeList<'a> {
    fn from(s: Vec<&'a String>) -> MaybeList<'a> {
        MaybeList::List(s.into_iter().map(|s| s.as_ref()).collect())
    }
}

impl<'a> From<Vec<&'a str>> for MaybeList<'a> {
    fn from(s: Vec<&'a str>) -> MaybeList<'a> {
        MaybeList::List(s)
    }
}

pub struct MonCommand<'a> {
    map: HashMap<&'a str, MaybeList<'a>>,
}

impl<'a> Default for MonCommand<'a> {
    fn default() -> Self {
        MonCommand {
            map: {
                let mut map = HashMap::new();
                map.insert("format", "json".into());
                map
            },
        }
    }
}

impl<'a> MonCommand<'a> {
    pub fn new() -> MonCommand<'a> {
        MonCommand::default()
    }

    pub fn with_format(self, format: &'a str) -> MonCommand<'a> {
        self.with("format", MaybeList::String(format))
    }

    pub fn with_name(self, name: &'a str) -> MonCommand<'a> {
        self.with("name", MaybeList::String(name))
    }

    pub fn with_prefix(self, prefix: &'a str) -> MonCommand<'a> {
        self.with("prefix", MaybeList::String(prefix))
    }

    pub fn with<V: Into<MaybeList<'a>>>(mut self, name: &'a str, value: V) -> MonCommand<'a> {
        self.map.insert(name, value.into());
        self
    }

    pub fn as_json(&self) -> String {
        serde_json::to_string(&self.map).unwrap()
    }
}
