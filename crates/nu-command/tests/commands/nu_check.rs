use nu_test_support::fs::Stub::FileWithContentToBeTrimmed;
use nu_test_support::playground::Playground;
use nu_test_support::{nu, pipeline};

#[test]
fn parse_script_success() {
    Playground::setup("nu_check_test_1", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "script.nu",
            r#"
                greet "world"

                def greet [name] {
                  echo "hello" $name
                }
            "#,
        )]);

        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                nu-check script.nu
            "#
        ));

        assert!(actual.err.is_empty());
    })
}

#[test]
fn parse_script_with_wrong_type() {
    Playground::setup("nu_check_test_2", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "script.nu",
            r#"
                greet "world"

                def greet [name] {
                  echo "hello" $name
                }
            "#,
        )]);

        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                nu-check -d --as-module script.nu
            "#
        ));

        assert!(actual.err.contains("Failed to parse content"));
    })
}
#[test]
fn parse_script_failure() {
    Playground::setup("nu_check_test_3", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "script.nu",
            r#"
                greet "world"

                def greet [name {
                  echo "hello" $name
                }
            "#,
        )]);

        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                nu-check -d script.nu
            "#
        ));

        assert!(actual.err.contains("Unexpected end of code"));
    })
}

#[test]
fn parse_module_success() {
    Playground::setup("nu_check_test_4", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "foo.nu",
            r#"
                # foo.nu

                export def hello [name: string] {
                    $"hello ($name)!"
                }

                export def hi [where: string] {
                    $"hi ($where)!"
                }
            "#,
        )]);

        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                nu-check --as-module foo.nu
            "#
        ));

        assert!(actual.err.is_empty());
    })
}

#[test]
fn parse_module_with_wrong_type() {
    Playground::setup("nu_check_test_5", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "foo.nu",
            r#"
                # foo.nu

                export def hello [name: string {
                    $"hello ($name)!"
                }

                export def hi [where: string] {
                    $"hi ($where)!"
                }
            "#,
        )]);

        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                nu-check -d foo.nu
            "#
        ));

        assert!(actual.err.contains("Failed to parse content"));
    })
}
#[test]
fn parse_module_failure() {
    Playground::setup("nu_check_test_6", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "foo.nu",
            r#"
                # foo.nu

                export def hello [name: string {
                    $"hello ($name)!"
                }

                export def hi [where: string] {
                    $"hi ($where)!"
                }
            "#,
        )]);

        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                nu-check -d --as-module foo.nu
            "#
        ));

        assert!(actual.err.contains("Unexpected end of code"));
    })
}

#[test]
fn file_not_exist() {
    Playground::setup("nu_check_test_7", |dirs, _sandbox| {
        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                nu-check --as-module foo.nu
            "#
        ));

        assert!(actual.err.contains("file not found"));
    })
}

#[test]
fn parse_unsupported_file() {
    Playground::setup("nu_check_test_8", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "foo.txt",
            r#"
                # foo.nu

                export def hello [name: string {
                    $"hello ($name)!"
                }

                export def hi [where: string] {
                    $"hi ($where)!"
                }
            "#,
        )]);

        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                nu-check --as-module foo.txt
            "#
        ));

        assert!(actual
            .err
            .contains("File extension must be the type of .nu"));
    })
}
#[test]
fn parse_dir_failure() {
    Playground::setup("nu_check_test_9", |dirs, _sandbox| {
        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                nu-check --as-module ~
            "#
        ));

        assert!(actual.err.contains("Path is not a file"));
    })
}

#[test]
fn parse_module_success_2() {
    Playground::setup("nu_check_test_10", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "foo.nu",
            r#"
                # foo.nu

                export env MYNAME { "Arthur, King of the Britons" }
            "#,
        )]);

        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                nu-check --as-module foo.nu
            "#
        ));

        assert!(actual.err.is_empty());
    })
}

#[test]
fn parse_script_success_with_raw_stream() {
    Playground::setup("nu_check_test_11", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "script.nu",
            r#"
                greet "world"

                def greet [name] {
                  echo "hello" $name
                }
            "#,
        )]);

        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                open script.nu | nu-check 
            "#
        ));

        assert!(actual.err.is_empty());
    })
}

#[test]
fn parse_module_success_with_raw_stream() {
    Playground::setup("nu_check_test_12", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "foo.nu",
            r#"
                # foo.nu

                export def hello [name: string] {
                    $"hello ($name)!"
                }

                export def hi [where: string] {
                    $"hi ($where)!"
                }
            "#,
        )]);

        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                open foo.nu | nu-check --as-module
            "#
        ));

        assert!(actual.err.is_empty());
    })
}

#[test]
fn parse_string_as_script_success() {
    Playground::setup("nu_check_test_13", |dirs, _sandbox| {
        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                echo $'two(char nl)lines' | nu-check
            "#
        ));

        assert!(actual.err.is_empty());
    })
}

#[test]
fn parse_string_as_script() {
    Playground::setup("nu_check_test_14", |dirs, _sandbox| {
        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                echo $'two(char nl)lines' | nu-check -d --as-module
            "#
        ));

        println!("the out put is {}", actual.err);
        assert!(actual.err.contains("Failed to parse content"));
    })
}

#[test]
fn parse_module_success_with_internal_stream() {
    Playground::setup("nu_check_test_15", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "foo.nu",
            r#"
                # foo.nu

                export def hello [name: string] {
                    $"hello ($name)!"
                }

                export def hi [where: string] {
                    $"hi ($where)!"
                }
            "#,
        )]);

        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                open foo.nu | lines | nu-check --as-module
            "#
        ));

        assert!(actual.err.is_empty());
    })
}

#[test]
fn parse_script_success_with_complex_internal_stream() {
    Playground::setup("nu_check_test_16", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "grep.nu",
            r#"
                #grep for nu
                def grep-nu [
                  search   #search term
                  entrada?  #file or pipe
                  #
                  #Examples
                  #grep-nu search file.txt
                  #ls **/* | some_filter | grep-nu search 
                  #open file.txt | grep-nu search
                ] {
                  if ($entrada | empty?) {
                    if ($in | column? name) {
                      grep -ihHn $search ($in | get name)
                    } else {
                      ($in | into string) | grep -ihHn $search
                    }
                  } else {
                      grep -ihHn $search $entrada
                  }
                  | lines 
                  | parse "{file}:{line}:{match}"
                  | str trim
                  | update match {|f| 
                      $f.match 
                      | nu-highlight
                    }
                  | rename "source file" "line number"
                }

            "#,
        )]);

        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                open grep.nu | lines | nu-check
            "#
        ));

        assert!(actual.err.is_empty());
    })
}

#[test]
fn parse_script_failure_with_complex_internal_stream() {
    Playground::setup("nu_check_test_17", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "grep.nu",
            r#"
                #grep for nu
                def grep-nu [
                  search   #search term
                  entrada?  #file or pipe
                  #
                  #Examples
                  #grep-nu search file.txt
                  #ls **/* | some_filter | grep-nu search 
                  #open file.txt | grep-nu search
                ] 
                  if ($entrada | empty?) {
                    if ($in | column? name) {
                      grep -ihHn $search ($in | get name)
                    } else {
                      ($in | into string) | grep -ihHn $search
                    }
                  } else {
                      grep -ihHn $search $entrada
                  }
                  | lines 
                  | parse "{file}:{line}:{match}"
                  | str trim
                  | update match {|f| 
                      $f.match 
                      | nu-highlight
                    }
                  | rename "source file" "line number"
                }

            "#,
        )]);

        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                open grep.nu | lines | nu-check
            "#
        ));

        assert_eq!(actual.out, "false".to_string());
    })
}

#[test]
fn parse_script_success_with_complex_external_stream() {
    Playground::setup("nu_check_test_18", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "grep.nu",
            r#"
                #grep for nu
                def grep-nu [
                  search   #search term
                  entrada?  #file or pipe
                  #
                  #Examples
                  #grep-nu search file.txt
                  #ls **/* | some_filter | grep-nu search 
                  #open file.txt | grep-nu search
                ] {
                  if ($entrada | empty?) {
                    if ($in | column? name) {
                      grep -ihHn $search ($in | get name)
                    } else {
                      ($in | into string) | grep -ihHn $search
                    }
                  } else {
                      grep -ihHn $search $entrada
                  }
                  | lines 
                  | parse "{file}:{line}:{match}"
                  | str trim
                  | update match {|f| 
                      $f.match 
                      | nu-highlight
                    }
                  | rename "source file" "line number"
                }

            "#,
        )]);

        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                open grep.nu | nu-check
            "#
        ));

        assert!(actual.err.is_empty());
    })
}

#[test]
fn parse_module_success_with_complex_external_stream() {
    Playground::setup("nu_check_test_19", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "grep.nu",
            r#"
                #grep for nu
                def grep-nu [
                  search   #search term
                  entrada?  #file or pipe
                  #
                  #Examples
                  #grep-nu search file.txt
                  #ls **/* | some_filter | grep-nu search 
                  #open file.txt | grep-nu search
                ] {
                  if ($entrada | empty?) {
                    if ($in | column? name) {
                      grep -ihHn $search ($in | get name)
                    } else {
                      ($in | into string) | grep -ihHn $search
                    }
                  } else {
                      grep -ihHn $search $entrada
                  }
                  | lines 
                  | parse "{file}:{line}:{match}"
                  | str trim
                  | update match {|f| 
                      $f.match 
                      | nu-highlight
                    }
                  | rename "source file" "line number"
                }

            "#,
        )]);

        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                open grep.nu | nu-check -d --as-module
            "#
        ));

        assert!(actual.err.is_empty());
    })
}
