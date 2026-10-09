use std::num::NonZero;

use super::*;

type I = NonZero<u8>;
type CM = ChooseMin<u8, I>;

fn ix(i: usize) -> I {
    I::from_address(i)
}

fn check_invariants(cm: &CM) {
    let mut seen = vec![false; cm.list.len()];
    for (level, head) in cm.levels.iter().enumerate() {
        let mut prev = None;
        let mut cur = *head;
        while let Some(e) = cur {
            let cell = &cm.list[e.ix()];
            assert_eq!(cell.count as usize, level, "element {e} in wrong level");
            assert_eq!(cell.node.prev, prev, "broken prev link at {e}");
            assert!(!seen[e.address()], "element {e} seen twice");
            seen[e.address()] = true;
            prev = cur;
            cur = cell.node.next;
        }
    }
    let missing: Vec<_> = (0..seen.len()).filter(|&i| !seen[i]).collect();
    assert!(missing.is_empty(), "elements not in any level: {missing:?}");
}

fn check_min(cm: &CM, counts: &[u8]) {
    match (cm.min(), counts.iter().min()) {
        (None, None) => {}
        (Some(e), Some(&m)) => {
            assert_eq!(counts[e.address()], m, "min() = {e}, counts = {counts:?}")
        }
        (got, expected) => panic!("min() = {got:?}, expected count {expected:?}"),
    }
}

fn build(counts: &[u8]) -> CM {
    let cm: CM = counts.iter().copied().collect();
    check_invariants(&cm);
    check_min(&cm, counts);
    cm
}

#[test]
fn empty_has_no_min() {
    build(&[]);
}

#[test]
fn single_element() {
    let cm = build(&[3]);
    assert_eq!(cm.min(), Some(ix(0)));
}

#[test]
fn picks_minimum() {
    let cm = build(&[3, 1, 2]);
    assert_eq!(cm.min(), Some(ix(1)));
}

#[test]
fn picks_zero_count() {
    let cm = build(&[2, 0, 1]);
    assert_eq!(cm.min(), Some(ix(1)));
}

#[test]
fn ties_pick_any_minimum() {
    build(&[2, 1, 3, 1]);
}

#[test]
fn decrease_moves_to_new_min() {
    let mut counts = [3, 2, 3];
    let mut cm = build(&counts);
    for i in 0..2 {
        cm.update_count(ix(0), |c| c.decrease());
        counts[0] -= 1;
        check_invariants(&cm);
        check_min(&cm, &counts);
    }
    assert_eq!(cm.min(), Some(ix(0)));
}

#[test]
fn increase_after_decrease_restores_min() {
    // cover / uncover pattern from dancing links
    let mut cm = build(&[2, 3, 2]);
    cm.update_count(ix(0), |c| c.decrease());
    check_min(&cm, &[1, 3, 2]);
    cm.update_count(ix(0), |c| c.increase());
    check_invariants(&cm);
    check_min(&cm, &[2, 3, 2]);
}

#[test]
fn increase_only_min_element() {
    let mut cm = build(&[1, 2, 3]);
    cm.update_count(ix(0), |c| c.increase());
    check_invariants(&cm);
    check_min(&cm, &[2, 2, 3]);
}

#[test]
fn random_updates_keep_invariants() {
    let mut seed: u64 = 0x2545_F491_4F6C_DD1D;
    let mut next = move |n: u64| {
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (seed >> 33) % n
    };
    for _ in 0..50 {
        let len = 1 + next(12) as usize;
        let mut counts: Vec<u8> = (0..len).map(|_| 1 + next(6) as u8).collect();
        let max = *counts.iter().max().unwrap();
        let mut cm = build(&counts);
        for _ in 0..200 {
            let e = next(len as u64) as usize;
            let up = counts[e] == 0 || (counts[e] < max && next(2) == 0);
            if up {
                cm.update_count(ix(e), |c| c.increase());
                counts[e] += 1;
            } else {
                cm.update_count(ix(e), |c| c.decrease());
                counts[e] -= 1;
            }
            check_invariants(&cm);
            check_min(&cm, &counts);
        }
    }
}
