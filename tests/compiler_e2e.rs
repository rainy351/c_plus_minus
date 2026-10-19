//! End-to-end tests for C± -> C -> GCC -> executable.
//! Run from the project root with `cargo test --test compiler_e2e`.
//! Requires gcc on PATH. Each test gets its own temp directory, so it cannot
//! overwrite the project's out.c / out.exe.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{self, Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static TEST_ID: AtomicUsize = AtomicUsize::new(0);

struct TestDir(PathBuf);

impl TestDir {
    fn new() -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock before UNIX_EPOCH")
            .as_nanos();
        let id = TEST_ID.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "cpm-e2e-{}-{unique}-{id}",
            process::id()
        ));
        fs::create_dir_all(&dir).expect("cannot create test directory");
        Self(dir)
    }

    fn path(&self) -> &Path {
        &self.0
    }

    fn write(&self, name: &str, text: &str) {
        fs::write(self.path().join(name), text).expect("cannot write test fixture");
    }

    fn compile(&self) -> Output {
        Command::new(env!("CARGO_BIN_EXE_с_plus_minus"))
            .arg("main.cpm")
            .current_dir(self.path())
            .output()
            .expect("cannot start C± compiler")
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        // Cleanup should not turn a test result into a failure.
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn display_output(out: &Output) -> String {
    format!(
        "exit: {:?}\nstdout:\n{}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

fn run_ok_with_files(source: &str, files: &[(&str, &str)], expected: &str) {
    let dir = TestDir::new();
    dir.write("main.cpm", source);
    for (name, content) in files {
        dir.write(name, content);
    }

    let compilation = dir.compile();
    assert!(
        compilation.status.success(),
        "C± or GCC failed:\n{}",
        display_output(&compilation)
    );

    let executable = dir.path().join(if cfg!(windows) { "out.exe" } else { "out" });
    let execution = Command::new(&executable)
        .current_dir(dir.path())
        .output()
        .expect("C± output executable is missing or could not start");
    assert!(
        execution.status.success(),
        "generated program failed:\n{}",
        display_output(&execution)
    );
    let actual = String::from_utf8_lossy(&execution.stdout).replace("\r\n", "\n");
    assert_eq!(actual, expected);
}

fn run_ok(source: &str, expected: &str) {
    run_ok_with_files(source, &[], expected);
}

fn compile_error(source: &str, expected_fragment: &str) {
    let dir = TestDir::new();
    dir.write("main.cpm", source);
    let compilation = dir.compile();
    assert!(
        !compilation.status.success(),
        "expected compilation failure, but it succeeded:\n{}",
        display_output(&compilation)
    );
    let output = display_output(&compilation);
    assert!(
        output.contains(expected_fragment),
        "expected diagnostic {expected_fragment:?}, got:\n{output}"
    );
}

#[test]
fn arithmetic_precedence() {
    run_ok(
        r#"
        idk a = 2+4*5;
        idk b = 2*4+5;
        __builtin_sys_print(a);
        __builtin_sys_print(",");
        __builtin_sys_print(b);
        "#,
        "22,13",
    );
}

#[test]
fn variable_reassignment() {
    run_ok(
        r#"
        idk a = 5;
        __builtin_sys_print(a);
        a = 4;
        __builtin_sys_print(a);
        "#,
        "54",
    );
}

#[test]
fn overloaded_functions_with_nested_calls() {
    run_ok(
        r#"
        func identity(manual int x): int { return x; }
        func identity(manual string x): string { return x; }
        __builtin_sys_print(identity("hello"));
        __builtin_sys_print(identity(42));
        "#,
        "hello42",
    );
}

#[test]
fn forward_declarations_and_recursion() {
    run_ok(
        r#"
        func first(manual int x): int { return factorial(x); }
        func factorial(manual int n): int {
            if(n < 2) { return 1; }
            else { return n * factorial(n-1); }
        }
        __builtin_sys_print(first(5));
        "#,
        "120",
    );
}

#[test]
fn nested_expression_as_function_argument() {
    run_ok(
        r#"
        func double(manual int x): int { return x*2; }
        __builtin_sys_print(double(1+4));
        "#,
        "10",
    );
}

#[test]
fn zero_argument_function() {
    run_ok(
        r#"
        func answer(): int { return 42; }
        __builtin_sys_print(answer());
        "#,
        "42",
    );
}

#[test]
fn if_else_and_shadowed_variables() {
    run_ok(
        r#"
        idk x = 5;
        if(x > 2) {
            idk x = "inner";
            __builtin_sys_print(x);
        } else {
            __builtin_sys_print("wrong");
        }
        __builtin_sys_print(x);
        "#,
        "inner5",
    );
}

#[test]
fn include_library() {
    run_ok_with_files(
        r#"
        include <helpers.xd>;
        __builtin_sys_print(from_lib());
        "#,
        &[("helpers.xd", "func from_lib(): int { return 7; }")],
        "7",
    );
}

#[test]
fn rejects_mismatched_variable_declaration() {
    compile_error(
        r#"manual int x = "wrong";"#,
        "tried to assign value of type",
    );
}

#[test]
fn rejects_assignment_of_wrong_type() {
    compile_error(
        r#"idk x = 5; x = "wrong";"#,
        "tried to assign value of type",
    );
}

#[test]
fn rejects_duplicate_declaration_in_same_scope() {
    compile_error(
        "idk x = 1; idk x = 2;",
        "already declared in this scope",
    );
}

#[test]
fn rejects_unknown_variable() {
    compile_error(
        "__builtin_sys_print(missing);",
        "not found in this scope",
    );
}

#[test]
fn rejects_unknown_overload() {
    compile_error(
        r#"
        func just_int(manual int x): int { return x; }
        just_int("wrong");
        "#,
        "not found",
    );
}

#[test]
fn rejects_wrong_return_type() {
    compile_error(
        r#"func wrong(): int { return "oops"; }"#,
        "return",
    );
}

#[test]
fn rejects_extra_token_after_assignment_value() {
    compile_error(
        "idk val3 = 1; val3 = 5 5; __builtin_sys_print(val3);",
        "parser: no semicolon",
    );
}

#[test]
fn rejects_missing_semicolon_after_assignment() {
    compile_error(
        "idk val3 = 1; val3 = 5",
        "parser: no semicolon",
    );
}

#[test]
fn modulo() {
    run_ok(
        r#"
            __builtin_sys_print(5/2);
            __builtin_sys_print(5%2);
            __builtin_sys_print(5/2*2+5%2);
        "#,
        "215",
    );
}

#[test]
fn while_test() {
    run_ok(
        r#"
            idk a = 0;
            while(a<5) {
                a = a+1;
                __builtin_sys_print(a);
            }
            __builtin_sys_print("end");
        "#,
        "12345end",
    );
}

