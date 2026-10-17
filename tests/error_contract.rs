use fn0rd_lib::error::FnordError;
use std::error::Error;

#[test]
fn contextual_error_messages_remain_stable() {
    for (error, expected) in [
        (
            FnordError::Config("invalid setting".into()),
            "config error: invalid setting",
        ),
        (
            FnordError::Date("invalid date".into()),
            "date error: invalid date",
        ),
        (
            FnordError::Parse("invalid input".into()),
            "parse error: invalid input",
        ),
    ] {
        assert_eq!(error.to_string(), expected);
        assert!(error.source().is_none());
    }
}

#[test]
fn io_conversion_preserves_message_kind_and_source() {
    let error: FnordError =
        std::io::Error::new(std::io::ErrorKind::PermissionDenied, "fixture denied").into();
    assert_eq!(error.to_string(), "io error: fixture denied");
    let source = error
        .source()
        .unwrap()
        .downcast_ref::<std::io::Error>()
        .unwrap();
    assert_eq!(source.kind(), std::io::ErrorKind::PermissionDenied);
    assert_eq!(source.to_string(), "fixture denied");
}
