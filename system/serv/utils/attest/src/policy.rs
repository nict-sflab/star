use crate::Measurement;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuotePolicy {
    pub expected_mrenclave: Option<Measurement>,
    pub expected_mrsigner: Option<Measurement>,
    pub allow_debug: bool,
    pub allow_advisory: bool,
}
