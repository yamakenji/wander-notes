pub const API_HOST: &str = "api.wandernotes.cc";
pub const PLANNED_FEATURES: &[&str] = &[
    "お問い合わせ",
    "閲覧数・いいね",
    "散歩記録API",
    "将来の検索・AI機能",
];

pub fn planned_features() -> &'static [&'static str] {
    PLANNED_FEATURES
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_expected_api_host() {
        assert_eq!(API_HOST, "api.wandernotes.cc");
    }

    #[test]
    fn keeps_all_planned_features() {
        assert_eq!(planned_features().len(), 4);
        assert!(planned_features().contains(&"お問い合わせ"));
        assert!(planned_features().contains(&"閲覧数・いいね"));
        assert!(planned_features().contains(&"散歩記録API"));
        assert!(planned_features().contains(&"将来の検索・AI機能"));
    }
}
