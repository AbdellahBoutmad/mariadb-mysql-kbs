use serde::Serialize;

pub struct Page<'a> {
    pub url: &'a str,
    pub name: &'a str,
}

#[derive(Clone)]
pub struct PageProcess<'a> {
    pub url: String,
    pub name: String,
    pub data_type: &'a str,
}

pub struct QueryResponse<'a> {
    pub url: &'a str,
    pub body: String,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Range {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i128>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "from")]
    pub from_f: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i128>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "to")]
    pub to_f: Option<f64>,
}

impl Range {
    pub fn try_fill_from(&mut self, val: String) {
        match val.parse::<i128>() {
            Ok(v) => self.from = Some(v),
            _ => match val.parse::<f64>() {
                Ok(v) => self.from_f = Some(v),
                _ => {}
            },
        }
    }

    pub fn try_fill_to(&mut self, val: String) {
        match val.parse::<i128>() {
            Ok(v) => self.to = Some(v),
            _ => match val.parse::<f64>() {
                Ok(v) => self.to_f = Some(v),
                _ => {}
            },
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct KbParsedEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cli: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<bool>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "skip_serialize_range")]
    pub range: Option<Range>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "validValues")]
    pub valid_values: Option<Vec<String>>,
}

impl KbParsedEntry {
    pub fn init_range(&mut self) {
        if self.range.is_none() {
            self.range = Some(Range {
                from: None,
                to: None,
                from_f: None,
                to_f: None,
            });
        }
    }
}

pub fn skip_serialize_range(data: &std::option::Option<Range>) -> bool {
    if data.is_none() {
        return true;
    }
    if data.as_ref().unwrap().from.is_some()
        || data.as_ref().unwrap().from_f.is_some()
        || data.as_ref().unwrap().to.is_some()
        || data.as_ref().unwrap().to_f.is_some()
    {
        return false;
    }
    return true;
}

#[derive(Serialize)]
pub struct DataFile<'a> {
    pub data: Vec<KbParsedEntry>,
    pub name: &'a str,
    pub url: &'a str,
}
