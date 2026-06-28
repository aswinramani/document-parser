pub struct Code {
    pub code: Option<String>,
    pub display_name: Option<String>,
    pub code_system: Option<String>,
    pub code_system_name: Option<String>,
    pub null_flavor: Option<String>,
    pub xsi_type: Option<String>,
    pub translations: Vec<Translation>,
}

pub struct Translation {
    pub xsi_type: Option<String>,
    pub code: Option<String>,
    pub display_name: Option<String>,
    pub code_system: Option<String>,
    pub code_system_name: Option<String>,
    pub null_flavor: Option<String>,
}

pub struct EffectiveTime {
    pub low: Option<String>,
    pub high: Option<String>,
    pub null_flavor: Option<String>,
    pub value: Option<String>,
}

pub struct BaseIdentifier {
    pub root: Option<String>,
    pub extension: Option<String>,
}

pub struct PatientIdentifier {
    pub base_id: Option<BaseIdentifier>,
    pub assigning_authority_name: Option<String>,
}

pub struct PersonName {
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub family: Option<String>,
    pub name_use: Option<String>,
    pub given: Vec<String>,
}

pub struct Address {
    pub address_use: Option<String>,
    pub street_address_line: Vec<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

pub struct Telecom {
    pub telecom_use: Option<String>,
    pub value: Option<String>,
}

pub struct Patient {
    pub ids: Vec<PatientIdentifier>,
    pub names: Vec<PersonName>,
    pub gender: Option<Code>,
    pub dob: Option<String>,
    pub addresses: Vec<Address>,
    pub telecoms: Vec<Telecom>,
    pub marital_status: Option<Code>,
    pub race: Option<Code>,
    pub ethnicity: Option<Code>,
}

pub struct Organization {
    pub ids: Vec<BaseIdentifier>,
    pub name: Option<String>,
    pub telecoms: Vec<Telecom>,
    pub addresses: Vec<Address>,
}

pub struct Author {
    pub template_id: Option<BaseIdentifier>,
    pub time: Option<EffectiveTime>,
    pub assigned_author: Option<AssignedAuthor>,
}

pub struct AssignedAuthor {
    pub id: Option<BaseIdentifier>,
    pub code: Option<Code>,
    pub address: Option<Address>,
    pub telecom: Option<Telecom>,
    pub assigned_person: Option<PersonName>,
}

pub struct DocumentMetadata {
    pub type_id: Option<BaseIdentifier>,
    pub template_ids: Vec<BaseIdentifier>,
    pub id: Option<BaseIdentifier>,
    pub code: Option<Code>,
    pub title: Option<String>,
    pub effective_time: Option<EffectiveTime>,
    pub confidentiality_code: Option<Code>,
    pub language_code: Option<String>,
    pub version_number: Option<String>,
    pub realm_code: Option<String>,
    pub authors: Vec<Author>,
    pub custodian: Option<Organization>,
}

pub struct Reference {
    pub value: Option<String>,
}
