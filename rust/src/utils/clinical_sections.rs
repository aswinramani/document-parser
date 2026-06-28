use crate::utils::common_structs::{Address, Author, BaseIdentifier, Code, EffectiveTime, PersonName, Telecom, Reference};

pub struct Section {
    pub template_ids: Vec<BaseIdentifier>,
    pub code: Option<Code>,
    pub title: Option<String>,
    pub text: Option<String>,
    pub entries: Vec<Entry>,
}

pub struct Entry {
    pub template_ids: Vec<BaseIdentifier>,
    pub id: Option<BaseIdentifier>,
    pub code: Option<Code>,
    pub status_code: Option<String>,
    pub effective_time: Option<EffectiveTime>,
    pub entry_relationship: Vec<EntryRelationship>,
}

pub struct EntryRelationship {
    pub type_code: Option<String>,
    pub observation: Option<Observation>,
}

pub struct Observation {
    pub class_code: Option<String>,
    pub mood_code: Option<String>,
    pub template_ids: Vec<BaseIdentifier>,
    pub id: Option<BaseIdentifier>,
    pub code: Option<Code>,
    pub status_code: Option<String>,
    pub text: Option<Reference>,
    pub effective_time: Option<EffectiveTime>,
    pub value: Option<Value>,
    pub author: Option<Author>,
}

pub struct Value {
    pub code: Option<Code> ,
    pub original_text: Option<Reference>,
}


