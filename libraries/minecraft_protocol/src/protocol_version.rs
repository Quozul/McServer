use macros::Pvn;
use std::cmp::Ordering;

#[derive(Default, Clone, Debug, Pvn, Hash)]
pub enum ProtocolVersion {
    #[default]
    #[pvn(769)]
    V1_21_4,
    #[pvn(768)]
    V1_21_2,
    #[pvn(767)]
    V1_21,
    #[pvn(766)]
    V1_20_5,
    #[pvn(765)]
    V1_20_3,
    #[pvn(764)]
    V1_20_2,
    #[pvn(763)]
    V1_20,
    #[pvn(762)]
    V1_19_4,
    #[pvn(761)]
    V1_19_3,
    #[pvn(760)]
    V1_19_1,
    #[pvn(759)]
    V1_19,
    #[pvn(758)]
    V1_18_2,
    #[pvn(757)]
    V1_18,
}

impl PartialEq for ProtocolVersion {
    fn eq(&self, other: &Self) -> bool {
        self.version_number() == other.version_number()
    }
}

impl Eq for ProtocolVersion {}

impl PartialOrd for ProtocolVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ProtocolVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        self.version_number().cmp(&other.version_number())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_version_ordering() {
        let v1_21 = ProtocolVersion::V1_21;
        let v1_21_2 = ProtocolVersion::V1_21_2;
        let v1_21_4 = ProtocolVersion::V1_21_4;

        assert!(v1_21 < v1_21_2);
        assert!(v1_21_2 < v1_21_4);
        assert!(v1_21_4 > v1_21_2);
        assert_eq!(v1_21_4, v1_21_4);
        assert_ne!(v1_21_2, v1_21_4);
    }
}
