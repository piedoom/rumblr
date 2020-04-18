use crate::data::Meta;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Edit {
    meta: Meta,
}