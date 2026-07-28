use DA001_linear_search::linear_search::{
    char::{Char, Cordanets, get_position},
    linear_search,
    search::{linear_search_all, linear_search_res},
    x_linear_one,
};
use pretty_assertions::assert_eq;
use test_case::test_case;

#[test_case(Some(2), &["asd", "asdf", "tr"], &"tr" )]
#[test_case(Some(0), &["a"], &"a" )]
#[test_case(Some(3), &["a", "b", "c", "d", "google", "zoo"], &"d" )]
#[test_case(Some(4), &["a", "b", "c", "d", "google", "zoo"], &"google" )]
#[test_case(None, &[""], &"a" )]
#[test_case(Some(1), &[1, 2, 3, 4], &2 )]
#[test_case(Some(0), &[1, 2, 3, 4], &1)]
#[test_case(Some(5), &[1, 2, 3, 4, 0, 11], &11 )]
#[test_case(None, &[1, 2, 3, 4], &5 )]
#[test_case(None, &[1, 2, 3, 4, 11], &5 )]
#[test_case(Some(2), &[Char::N, Char::T, Char::I, Char::Q ], &Char::I )]
#[test_case(Some(0), get_position().as_ref(), &Cordanets{x:0,y:0,z:0} )]
#[test_case(None, get_position().as_ref(), &Cordanets{x:12,y:0,z:0} )]
#[test_case(None, &[ Char::T, Char::I, Char::Q ], &Char::N )]
#[test_case(Some(25) , &[ Char::A, Char::B, Char::C, Char::D, Char::E, Char::F, Char::G, Char::H, Char::I, Char::J, Char::K, Char::L, Char::M, Char::N, Char::O, Char::P, Char::Q, Char::R, Char::S, Char::T, Char::U, Char::V, Char::W, Char::X, Char::Y, Char::Z, Char::ZZ ], &Char::ZZ )]
#[test_case(Some(26) , &[ Char::A, Char::B, Char::C, Char::D, Char::E, Char::F, Char::G, Char::H, Char::I, Char::J, Char::K, Char::L, Char::M, Char::N, Char::O, Char::P, Char::Q, Char::R, Char::S, Char::T, Char::U, Char::V, Char::W, Char::X, Char::Y, Char::Z, Char::ZZ ], &Char::ZZ )]
fn test_linear_search<T: Ord>(expected: Option<usize>, arr: &[T], item: &T) {
    let res = linear_search(item, arr);
    assert_eq!(res, expected)
}

#[test_case(Some(5), &[1, 2, 3, 4, 0, 11], &11 )]
#[test_case(None, &[1, 2, 3, 4], &5 )]
#[test_case(None, &[1, 2, 3, 4, 11], &5 )]
#[test_case(Some(2), &[Char::N, Char::T, Char::I, Char::Q ], &Char::I )]
#[test_case(Some(0), get_position().as_ref(), &Cordanets{x:0,y:0,z:0} )]
#[test_case(None, get_position().as_ref(), &Cordanets{x:12,y:0,z:0} )]
#[test_case(None, &[ Char::T, Char::I, Char::Q ], &Char::N )]
fn test_linear_search_res<T: Ord>(expected: Option<usize>, arr: &[T], item: &T) {
    let res = linear_search_res(item, arr);
    let error_string = "Item not found in array";
    match res {
        Ok(e) => assert_eq!(e, expected.unwrap()),
        Err(err) => {
            assert_eq!(err, error_string)
        }
    };
}

#[test_case(2, &["asd", "asdf", "tr"], &"tr" )]
#[test_case(3, &["a", "b", "c", "d", "google", "zoo"], &"d" )]
#[test_case(0, &[1, 2, 3, 4], &1)]
#[test_case(26 , &[ Char::A, Char::B, Char::C, Char::D, Char::E, Char::F, Char::G, Char::H, Char::I, Char::J, Char::K, Char::L, Char::M, Char::N, Char::O, Char::P, Char::Q, Char::R, Char::S, Char::T, Char::U, Char::V, Char::W, Char::X, Char::Y, Char::Z, Char::ZZ ], &Char::ZZ )]
fn test_x_linear<T: Ord>(expected: usize, arr: &[T], item: &T) {
    let res = x_linear_one(item, arr);
    //
    assert_eq!(res.unwrap(), expected);
}

#[test_case( &[1, 2, 3, 4, 11], &5 )]
#[test_case( &[1, 2, 3, 4], &5 )]
fn test_x_linear_none<T: Ord>(arr: &[T], item: &T) {
    let string = "Not found".to_string();
    let res = x_linear_one(item, arr);

    assert_eq!(res.err().unwrap(), string);
}
