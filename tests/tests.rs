
#[macro_use]
extern crate ordermap;
extern crate itertools;
extern crate serde_test;

use serde_test::{Token, assert_tokens};

#[test]
fn test_sort() {
    let m = ordermap! {
        1 => 2,
        7 => 1,
        2 => 2,
        3 => 3,
    };

    itertools::assert_equal(m.sorted_by(|_k1, v1, _k2, v2| v1.cmp(v2)),
                            vec![(7, 1), (1, 2), (2, 2), (3, 3)]);
}

#[test]
fn test_serde() {
    // NOTE: Needs the `serde` feature to work
    let cities = ordermap! {
        "Arizona" => ("Phoenix", 1.615f64),
        "California" => ("Los Angeles", 3.976),
        "New York" => ("New York City", 8.538)
    };
    let mut expected_tokens = vec![Token::Map { len: Some(3) }];
    for (state, &(city, population)) in &cities {
        expected_tokens.extend(&[
            Token::BorrowedStr(state),
            Token::Tuple { len: 2 },
            Token::BorrowedStr(city),
            Token::F64(population),
            Token::TupleEnd
        ])
    }
    expected_tokens.push(Token::MapEnd);
    assert_tokens(&cities, &expected_tokens);
}

