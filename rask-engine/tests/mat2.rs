use core::f32::consts::FRAC_PI_2;

use rask_engine::math::{Mat2, Vec2};

const MAT: Mat2 = Mat2::new(0.0, 1.0, 2.0, 3.0);

fn mat_eq(mat1: &Mat2, mat2: &Mat2, precision: f32) -> bool {
    mat1.as_ref()
        .iter()
        .zip(mat2.as_ref())
        .all(|(a, b)| f32::abs(a - b) < precision)
}

#[test]
fn test_add_mat2() {
    assert_eq!(MAT + MAT, Mat2::new(0.0, 2.0, 4.0, 6.0));
}

#[test]
#[allow(clippy::eq_op)]
fn test_sub_mat2() {
    assert_eq!(MAT - MAT, Mat2::zero());
}

#[test]
fn test_neg_mat2() {
    assert_eq!(-MAT, Mat2::new(0.0, -1.0, -2.0, -3.0));
}

#[test]
fn test_mul_f32() {
    assert_eq!(MAT * 2.0, 2.0 * MAT);
    assert_eq!(MAT * 2.0, Mat2::new(0.0, 2.0, 4.0, 6.0));
}

#[test]
fn test_mul_vec3() {
    let a = Vec2::new(1.0, 2.0);

    assert_eq!(MAT * a, Vec2::new(2.0, 8.0));
}

#[test]
fn test_mul_mat2() {
    assert_eq!(MAT * MAT, Mat2::new(2.0, 3.0, 6.0, 11.0));
}

#[test]
fn test_div_f32() {
    assert_eq!(MAT / 0.5, Mat2::new(0.0, 2.0, 4.0, 6.0));
}

#[test]
fn test_zero() {
    let mat1 = Mat2::zero();
    let mat2 = Mat2::new(0.0, 0.0, 0.0, 0.0);

    assert_eq!(mat1.as_ref(), mat2.as_ref());
}

#[test]
fn test_identity() {
    let mat1 = Mat2::identity();
    let mat2 = Mat2::new(1.0, 0.0, 0.0, 1.0);
    assert_eq!(mat1.as_ref(), mat2.as_ref());
}

#[test]
fn test_identity_mul() {
    let mat2 = Mat2::identity();
    println!("ident: {:?}", mat2);
    println!("mat1(): {:?}", MAT);
    assert_eq!((MAT * mat2).as_ref(), MAT.as_ref());
}

#[test]
fn test_new() {
    let mat2 = [0.0, 2.0, 1.0, 3.0];
    assert_eq!(MAT.as_ref(), &mat2);
}

#[test]
fn test_transpose() {
    let mat2 = Mat2::new(0.0, 2.0, 1.0, 3.0);
    assert_eq!(&MAT.transpose(), &mat2);
}

#[test]
fn test_rotation() {
    let mat2 = Mat2::new(0.0, -1.0, 1.0, 0.0);
    assert!(mat_eq(
        &Mat2::rotation(FRAC_PI_2),
        &mat2,
        1e-7,
    ));
}
