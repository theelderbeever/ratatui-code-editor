use ratatui_code_editor::{code::Code, register_language};

#[test]
fn registered_language_is_parsed_and_highlighted() {
    // Reuse a bundled grammar under a new name to stand in for an external one.
    register_language(
        "custom-json",
        tree_sitter_json::LANGUAGE.into(),
        "(string) @string\n(number) @number",
    );
    let code = Code::new(r#"{"a": 1}"#, "custom-json", None).unwrap();
    assert_eq!(code.lang(), "custom-json");
    assert!(code.is_highlight());
    let theme =
        std::collections::HashMap::from([("string".to_owned(), 1), ("number".to_owned(), 2)]);
    let highlights = code.highlight_interval(0, code.len_chars(), &theme);
    assert!(
        !highlights.is_empty(),
        "registered query should produce highlights"
    );
}

#[test]
fn unknown_language_still_falls_back_to_plain_text() {
    assert!(Code::new("text", "not-registered", None).is_ok());
}
