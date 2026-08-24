use academiagpu_community::quota::QuotaRule;

#[test]
fn test_default_quota_limits() {
    let rule = QuotaRule::default();
    // Default community quota should be 8GB
    assert_eq!(rule.max_vram_mb, 8192);
    assert_eq!(rule.max_gpus, 1);
}

#[test]
fn test_custom_quota_serialization() {
    let rule = QuotaRule {
        max_vram_mb: 24576,
        max_gpus: 4,
    };
    
    let json = serde_json::to_string(&rule).unwrap();
    let deserialized: QuotaRule = serde_json::from_str(&json).unwrap();
    
    assert_eq!(deserialized.max_vram_mb, 24576);
    assert_eq!(deserialized.max_gpus, 4);
}
