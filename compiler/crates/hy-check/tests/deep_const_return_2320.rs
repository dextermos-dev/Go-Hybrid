#[test]
fn deep_const_return_roots_regression_2320() {
    // Program A: const map returned through 5 wrappers, moved to task, then mutated (HYB-E3004 / HYB-E3006)
    let prog_a = r#"
const shared = map[string]int{"a": 1}

fn leaf() map[string]int { return shared }
fn h1() map[string]int { return leaf() }
fn h2() map[string]int { return h1() }
fn h3() map[string]int { return h2() }
fn h4() map[string]int { return h3() }
fn h5() map[string]int { return h4() }

fn worker(m map[string]int, done chan int) {
    _ = m["a"]
    done <- 1
}

fn main() {
    m := h5()
    done := make(chan int, 1)
    task worker(move m, done)
    m2 := h5()
    m2["a"] = 99
    <-done
}
"#;
    let r_a = hy_check::check_file(&hy_syntax::parse_file("prog_a.hy", prog_a).unwrap());
    assert_eq!(r_a.verdict, hy_check::Verdict::Reject, "expected reject for Program A: {:?}", r_a.diagnostics);
    assert!(
        r_a.diagnostics.iter().any(|d| d.code == "HYB-E3004" || d.code == "HYB-E3006"),
        "expected HYB-E3004 or HYB-E3006: {:?}",
        r_a.diagnostics
    );

    // Program B: const map returned through local fn value, moved to task, then mutated (HYB-E3004 / HYB-E3006)
    let prog_b = r#"
const shared = map[string]int{"a": 1}

fn leaf() map[string]int { return shared }

fn worker(m map[string]int, done chan int) {
    _ = m["a"]
    done <- 1
}

fn main() {
    cb := leaf
    m := cb()
    done := make(chan int, 1)
    task worker(move m, done)
    m2 := cb()
    m2["a"] = 99
    <-done
}
"#;
    let r_b = hy_check::check_file(&hy_syntax::parse_file("prog_b.hy", prog_b).unwrap());
    assert_eq!(r_b.verdict, hy_check::Verdict::Reject, "expected reject for Program B: {:?}", r_b.diagnostics);
    assert!(
        r_b.diagnostics.iter().any(|d| d.code == "HYB-E3004" || d.code == "HYB-E3006"),
        "expected HYB-E3004 or HYB-E3006: {:?}",
        r_b.diagnostics
    );
}
