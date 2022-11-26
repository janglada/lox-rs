#[derive(Clone, Debug)]
pub struct UpValue {
    pub(crate) index: u8,
    pub(crate) is_local: bool,
}
