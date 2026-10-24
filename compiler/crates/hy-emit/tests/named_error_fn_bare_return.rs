use hy_emit::emit_go;
use hy_syntax::parse_file;

fn emit_src(src: &str) -> String {
    let ast = parse_file("test.hy", src).expect("parses");
    emit_go(&ast).expect("emits")
}

#[test]
fn named_error_fn_bare_return_preserves_named_results_issue_1450() {
    let src = "\
import \"errors\"

fn compute(bad bool) (total int, err error) {
    total = 42
    if bad {
        err = errors.new(\"boom\")
        return
    }
    return
}

fn main() {}
";
    let go = emit_src(src);
    assert!(
        go.contains("func compute(bad bool) (total int, err error) {\n\ttotal = 42\n\tif bad {\n\t\terr = errors.New(\"boom\")\n\t\treturn\n\t}\n\treturn\n"),
        "expected naked return statements in named result error function, got:\n{go}"
    );
    assert!(
        !go.contains("return 0, nil"),
        "naked return must not be clobbered with zero-filled values:\n{go}"
    );
}

#[test]
fn mixed_named_error_fn_bare_return_emits_naked_return_issue_1450() {
    let src = "\
fn compute() (total int, error) {
    total = 100
    return
}

fn main() {}
";
    let go = emit_src(src);
    assert!(
        go.contains("\treturn\n"),
        "expected naked return statement, got:\n{go}"
    );
    assert!(
        !go.contains("return 0, nil"),
        "must not clobber named total:\n{go}"
    );
}

#[test]
fn named_error_fn_explicit_short_return_still_zero_fills() {
    let src = "\
fn compute() (total int, err error) {
    return 99
}

fn main() {}
";
    let go = emit_src(src);
    assert!(
        go.contains("return 99, nil"),
        "explicit short return must fill error slot with nil, got:\n{go}"
    );
}

#[test]
fn unnamed_error_fn_bare_return_still_zero_fills() {
    let src = "\
fn compute() (int, error) {
    return
}

fn compute_err_only() error {
    return
}

fn main() {}
";
    let go = emit_src(src);
    assert!(
        go.contains("func compute() (int, error) {\n\treturn 0, nil\n}"),
        "unnamed results must zero-fill, got:\n{go}"
    );
    assert!(
        go.contains("func compute_err_only() error {\n\treturn nil\n}"),
        "unnamed single error result must zero-fill with nil, got:\n{go}"
    );
}
