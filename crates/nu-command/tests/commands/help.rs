use nu_test_support::fs::Stub::FileWithContent;
use nu_test_support::playground::Playground;
use nu_test_support::{nu, nu_repl_code, pipeline};

#[test]
fn help_commands_length() {
    let actual = nu!(
    cwd: ".", pipeline(
        r#"
        help commands | length
        "#
    ));

    let output = actual.out;
    let output_int: i32 = output.parse().unwrap();
    let is_positive = output_int.is_positive();
    assert!(is_positive);
}

#[test]
fn help_shows_signature() {
    let actual = nu!(cwd: ".", pipeline("help str distance"));
    assert!(actual
        .out
        .contains("<string> | str distance <string> -> <int>"));

    // don't show signature for parser keyword
    let actual = nu!(cwd: ".", pipeline("help alias"));
    assert!(!actual.out.contains("Signatures"));
}

#[ignore = "TODO: Need to decide how to do help messages of new aliases"]
#[test]
fn help_aliases() {
    let code = &[
        "alias SPAM = print 'spam'",
        "help aliases | where name == SPAM | length",
    ];
    let actual = nu!(cwd: ".", nu_repl_code(code));

    assert_eq!(actual.out, "1");
}

#[ignore = "TODO: Need to decide how to do help messages of new aliases"]
#[test]
fn help_alias_usage_1() {
    Playground::setup("help_alias_usage_1", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContent(
            "spam.nu",
            r#"
                # line1
                alias SPAM = print 'spam'
            "#,
        )]);

        let code = &[
            "source spam.nu",
            "help aliases | where name == SPAM | get 0.usage",
        ];
        let actual = nu!(cwd: dirs.test(), nu_repl_code(code));

        assert_eq!(actual.out, "line1");
    })
}

#[ignore = "TODO: Need to decide how to do help messages of new aliases"]
#[test]
fn help_alias_usage_2() {
    let code = &[
        "alias SPAM = print 'spam'  # line2",
        "help aliases | where name == SPAM | get 0.usage",
    ];
    let actual = nu!(cwd: ".", nu_repl_code(code));

    assert_eq!(actual.out, "line2");
}

#[ignore = "TODO: Need to decide how to do help messages of new aliases"]
#[test]
fn help_alias_usage_3() {
    Playground::setup("help_alias_usage_3", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContent(
            "spam.nu",
            r#"
                # line1
                alias SPAM = print 'spam' # line2
            "#,
        )]);

        let code = &[
            "source spam.nu",
            "help aliases | where name == SPAM | get 0.usage",
        ];
        let actual = nu!(cwd: dirs.test(), nu_repl_code(code));

        assert!(actual.out.contains("line1"));
        assert!(actual.out.contains("line2"));
    })
}

#[ignore = "TODO: Need to decide how to do help messages of new aliases"]
#[test]
fn help_alias_name() {
    Playground::setup("help_alias_name", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContent(
            "spam.nu",
            r#"
                # line1
                alias SPAM = print 'spam' # line2
            "#,
        )]);

        let code = &["source spam.nu", "help aliases SPAM"];
        let actual = nu!(cwd: dirs.test(), nu_repl_code(code));

        assert!(actual.out.contains("line1"));
        assert!(actual.out.contains("line2"));
        assert!(actual.out.contains("SPAM"));
        assert!(actual.out.contains("print 'spam'"));
    })
}

#[ignore = "TODO: Need to decide how to do help messages of new aliases"]
#[test]
fn help_alias_name_f() {
    Playground::setup("help_alias_name_f", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContent(
            "spam.nu",
            r#"
                # line1
                alias SPAM = print 'spam' # line2
            "#,
        )]);

        let code = &["source spam.nu", "help aliases -f SPAM | get 0.usage"];
        let actual = nu!(cwd: dirs.test(), nu_repl_code(code));

        assert!(actual.out.contains("line1"));
        assert!(actual.out.contains("line2"));
    })
}

#[ignore = "TODO: Need to decide how to do help messages of new aliases"]
#[test]
fn help_export_alias_name_single_word() {
    Playground::setup("help_export_alias_name_single_word", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContent(
            "spam.nu",
            r#"
                # line1
                export alias SPAM = print 'spam' # line2
            "#,
        )]);

        let code = &["use spam.nu SPAM", "help aliases SPAM"];
        let actual = nu!(cwd: dirs.test(), nu_repl_code(code));

        assert!(actual.out.contains("line1"));
        assert!(actual.out.contains("line2"));
        assert!(actual.out.contains("SPAM"));
        assert!(actual.out.contains("print 'spam'"));
    })
}

#[ignore = "TODO: Need to decide how to do help messages of new aliases"]
#[test]
fn help_export_alias_name_multi_word() {
    Playground::setup("help_export_alias_name_multi_word", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContent(
            "spam.nu",
            r#"
                # line1
                export alias SPAM = print 'spam' # line2
            "#,
        )]);

        let code = &["use spam.nu", "help aliases spam SPAM"];
        let actual = nu!(cwd: dirs.test(), nu_repl_code(code));

        assert!(actual.out.contains("line1"));
        assert!(actual.out.contains("line2"));
        assert!(actual.out.contains("SPAM"));
        assert!(actual.out.contains("print 'spam'"));
    })
}

#[test]
fn help_module_usage_1() {
    Playground::setup("help_module_usage", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContent(
            "spam.nu",
            r#"
                # line1
                module SPAM {
                    # line2
                } #line3
            "#,
        )]);

        let code = &[
            "source spam.nu",
            "help modules | where name == SPAM | get 0.usage",
        ];
        let actual = nu!(cwd: dirs.test(), nu_repl_code(code));

        assert!(actual.out.contains("line1"));
        assert!(actual.out.contains("line2"));
        assert!(actual.out.contains("line3"));
    })
}

#[test]
fn help_module_name() {
    Playground::setup("help_module_name", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContent(
            "spam.nu",
            r#"
                # line1
                module SPAM {
                    # line2
                } #line3
            "#,
        )]);

        let code = &["source spam.nu", "help modules SPAM"];
        let actual = nu!(cwd: dirs.test(), nu_repl_code(code));

        assert!(actual.out.contains("line1"));
        assert!(actual.out.contains("line2"));
        assert!(actual.out.contains("line3"));
        assert!(actual.out.contains("SPAM"));
    })
}

#[test]
fn help_module_sorted_decls() {
    Playground::setup("help_module_sorted_decls", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContent(
            "spam.nu",
            r#"
                module SPAM {
                    export def z [] {}
                    export def a [] {}
                }
            "#,
        )]);

        let code = &["source spam.nu", "help modules SPAM"];
        let actual = nu!(cwd: dirs.test(), nu_repl_code(code));

        assert!(actual.out.contains("a, z"));
    })
}

#[test]
fn help_module_sorted_aliases() {
    Playground::setup("help_module_sorted_aliases", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContent(
            "spam.nu",
            r#"
                module SPAM {
                    export alias z = echo 'z'
                    export alias a = echo 'a'
                }
            "#,
        )]);

        let code = &["source spam.nu", "help modules SPAM"];
        let actual = nu!(cwd: dirs.test(), nu_repl_code(code));

        assert!(actual.out.contains("a, z"));
    })
}

#[test]
fn help_usage_extra_usage_command() {
    Playground::setup("help_usage_extra_usage_command", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContent(
            "spam.nu",
            r#"
                # module_line1
                #
                # module_line2

                # def_line1
                #
                # def_line2
                export def foo [] {}
            "#,
        )]);

        let actual = nu!(cwd: dirs.test(), pipeline("use spam.nu *; help modules spam"));
        assert!(actual.out.contains("module_line1"));
        assert!(actual.out.contains("module_line2"));

        let actual = nu!(cwd: dirs.test(),
            pipeline("use spam.nu *; help modules | where name == spam | get 0.usage"));
        assert!(actual.out.contains("module_line1"));
        assert!(!actual.out.contains("module_line2"));

        let actual = nu!(cwd: dirs.test(), pipeline("use spam.nu *; help commands foo"));
        assert!(actual.out.contains("def_line1"));
        assert!(actual.out.contains("def_line2"));

        let actual = nu!(cwd: dirs.test(),
            pipeline("use spam.nu *; help commands | where name == foo | get 0.usage"));
        assert!(actual.out.contains("def_line1"));
        assert!(!actual.out.contains("def_line2"));
    })
}

#[ignore = "TODO: Need to decide how to do help messages of new aliases"]
#[test]
fn help_usage_extra_usage_alias() {
    Playground::setup("help_usage_extra_usage_alias", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContent(
            "spam.nu",
            r#"
                # module_line1
                #
                # module_line2

                # alias_line1
                #
                # alias_line2
                export alias bar = echo 'bar'
            "#,
        )]);

        let actual = nu!(cwd: dirs.test(), pipeline("use spam.nu *; help modules spam"));
        assert!(actual.out.contains("module_line1"));
        assert!(actual.out.contains("module_line2"));

        let actual = nu!(cwd: dirs.test(),
            pipeline("use spam.nu *; help modules | where name == spam | get 0.usage"));
        assert!(actual.out.contains("module_line1"));
        assert!(!actual.out.contains("module_line2"));

        let actual = nu!(cwd: dirs.test(), pipeline("use spam.nu *; help aliases bar"));
        assert!(actual.out.contains("alias_line1"));
        assert!(actual.out.contains("alias_line2"));

        let actual = nu!(cwd: dirs.test(),
            pipeline("use spam.nu *; help aliases | where name == bar | get 0.usage"));
        assert!(actual.out.contains("alias_line1"));
        assert!(!actual.out.contains("alias_line2"));
    })
}

#[test]
fn help_modules_main_1() {
    let inp = &[
        r#"module spam {
            export def main [] { 'foo' };
        }"#,
        "help spam",
    ];

    let actual = nu!(cwd: ".", pipeline(&inp.join("; ")));

    assert!(actual.out.contains("  spam"));
}

#[test]
fn help_modules_main_2() {
    let inp = &[
        r#"module spam {
            export def main [] { 'foo' };
        }"#,
        "help modules | where name == spam | get 0.commands.0",
    ];

    let actual = nu!(cwd: ".", pipeline(&inp.join("; ")));

    assert_eq!(actual.out, "spam");
}

#[ignore = "TODO: Can't have alias with the same name as command"]
#[test]
fn help_alias_before_command() {
    let code = &[
        "alias SPAM = echo 'spam'",
        "def SPAM [] { 'spam' }",
        "help SPAM",
    ];
    let actual = nu!(cwd: ".", nu_repl_code(code));

    assert!(actual.out.contains("Alias"));
}
