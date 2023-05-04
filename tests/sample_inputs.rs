use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let input = r#""#;
    let output = r#""#;
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), output);
    assert!(output.stderr_str().is_empty());
}

