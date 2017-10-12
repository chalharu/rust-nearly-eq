#![cfg_attr(feature = "i128", feature(i128_type))]

#[cfg(feature = "num-complex")]
extern crate num_complex;

#[cfg(feature = "ndarray")]
extern crate ndarray;

#[cfg(feature = "num-rational")]
extern crate num_rational;

#[macro_use]
extern crate nearly_eq;

#[cfg(feature = "num-complex")]
use num_complex::Complex;

#[cfg(feature = "num-rational")]
use num_rational::Rational;

#[cfg(feature = "ndarray")]
use ndarray::arr1;

#[cfg(feature = "ndarray")]
use ndarray::arr2;

#[cfg(feature = "ndarray")]
use ndarray::arr3;

#[test]
fn it_should_not_panic_if_values_are_nearly_equal() {
    assert_nearly_eq!(8f32, 8f32 + 1e-7);
}

#[test]
fn it_should_not_panic_if_values_are_nearly_equal_f64() {
    assert_nearly_eq!(0f64, 1e-12 as f64);
}

#[test]
#[should_panic]
fn it_should_panic_if_values_are_not_nearly_equal() {
    assert_nearly_eq!(8f32, 8f32 - 1e-5);
}

#[test]
fn compare_with_explicit_eps() {
    assert_nearly_eq!(3f64, 4f64, 2f64);
}

#[test]
#[should_panic]
fn bad_compare_with_explicit_eps() {
    assert_nearly_eq!(3f64, 4f64, 1e-3f64);
}

#[test]
fn compare_with_vector() {
    let left = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let right = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_nearly_eq!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_vector() {
    let left = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.01];
    let right = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_nearly_eq!(left, right);
}

#[test]
#[should_panic]
fn bad_len_compare_with_vector() {
    let left = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let right = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_nearly_eq!(left, right);
}

#[test]
fn compare_with_slice() {
    let left = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let right = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_nearly_eq!(&left as &[f32], &right as &[f32]);
}

#[test]
#[should_panic]
fn bad_compare_with_slice() {
    let left = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.01];
    let right = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_nearly_eq!(&left as &[f32], &right as &[f32]);
}

#[test]
#[should_panic]
fn bad_len_compare_with_slice() {
    let left = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let right = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_nearly_eq!(&left as &[f32], &right as &[f32]);
}

#[test]
fn compare_with_array() {
    let left = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let right = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_nearly_eq!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_array() {
    let left = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.01];
    let right = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_nearly_eq!(left, right);
}

#[test]
#[cfg(feature = "num-complex")]
fn compare_with_complex() {
    let left = Complex::new(1.0f64, 0.0);
    let right = Complex::new(1.0f64, 1e-12);
    assert_nearly_eq!(left, right);
}

#[test]
#[should_panic]
#[cfg(feature = "num-complex")]
fn bad_compare_with_complex() {
    let left = Complex::new(1.0f64, 0.0);
    let right = Complex::new(1.0f64, 1e-8);
    assert_nearly_eq!(left, right);
}

#[test]
#[cfg(feature = "ndarray")]
fn compare_with_ndarray1d() {
    let left = arr1(&[1f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
    let right = arr1(&[1f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
    assert_nearly_eq!(left, right);
}

#[test]
#[should_panic]
#[cfg(feature = "ndarray")]
fn bad_compare_with_ndarray1d() {
    let left = arr1(&[1f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0001]);
    let right = arr1(&[1f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
    assert_nearly_eq!(left, right);
}


#[test]
#[cfg(feature = "ndarray")]
fn compare_with_ndarray2d() {
    let left = arr2(&[[1f64, 2.0, 3.0, 4.0, 5.0], [6.0, 7.0, 8.0, 9.0, 10.0]]);
    let right = arr2(&[[1f64, 2.0, 3.0, 4.0, 5.0], [6.0, 7.0, 8.0, 9.0, 10.0]]);
    assert_nearly_eq!(left, right);
}

#[test]
#[should_panic]
#[cfg(feature = "ndarray")]
fn bad_compare_with_ndarray2d_val() {
    let left = arr2(&[[1f64, 2.0, 3.0, 4.0, 5.0], [6.0, 7.0, 8.0, 9.0, 10.0001]]);
    let right = arr2(&[[1f64, 2.0, 3.0, 4.0, 5.0], [6.0, 7.0, 8.0, 9.0, 10.0]]);
    assert_nearly_eq!(left, right);
}

#[test]
#[should_panic]
#[cfg(feature = "ndarray")]
fn bad_compare_with_ndarray2d_len() {
    let left = arr2(&[[1f64, 2.0, 3.0, 4.0, 5.0], [6.0, 7.0, 8.0, 9.0, 10.0]]);
    let right = arr2(&[
        [1f64, 2.0],
        [3.0, 4.0],
        [5.0, 6.0],
        [7.0, 8.0],
        [9.0, 10.0],
    ]);
    assert_nearly_eq!(left, right);
}

#[test]
#[cfg(feature = "ndarray")]
fn compare_with_ndarray3d() {
    let left = arr3(&[[[1f64, 2.0], [4.0, 5.0]], [[6.0, 7.0], [9.0, 10.0]]]);
    let right = arr3(&[[[1f64, 2.0], [4.0, 5.0]], [[6.0, 7.0], [9.0, 10.0]]]);
    assert_nearly_eq!(left, right);
}

#[test]
#[should_panic]
#[cfg(feature = "ndarray")]
fn bad_compare_with_ndarray3d() {
    let left = arr3(&[[[1f64, 2.0], [4.0, 5.0]], [[6.0, 7.0], [9.0, 10.0001]]]);
    let right = arr3(&[[[1f64, 2.0], [4.0, 5.0]], [[6.0, 7.0], [9.0, 10.0]]]);
    assert_nearly_eq!(left, right);
}

macro_rules! type_impls {
    ($($T:ident)+) => {
        $(
            mod $T {
                #[test]
                fn it_should_not_panic_if_values_are_nearly_equal() {
                    assert_nearly_eq!(0 as $T, 0 as $T);
                }

                #[test]
                #[should_panic]
                fn it_should_panic_if_values_are_not_nearly_equal() {
                    assert_nearly_eq!(0 as $T, 1 as $T);
                }
            }
        )+
    }
}

type_impls! { i8 i16 i32 i64 u8 u16 u32 u64 }

#[cfg(feature = "i128")]
type_impls! { i128 u128 }

#[test]
#[cfg(feature = "num-rational")]
fn compare_with_ratio() {
    let left = Rational::new(1, 1000);
    let right = Rational::new(1, 1001);
    let eps = Rational::new(1, 10000);
    assert_nearly_eq!(left, right, eps);
}

#[test]
#[should_panic]
#[cfg(feature = "num-rational")]
fn bad_compare_with_ratio() {
    let left = Rational::new(1, 1000);
    let right = Rational::new(1, 1001);
    let eps = Rational::new(1, 1000000000);
    assert_nearly_eq!(left, right, eps);
}
