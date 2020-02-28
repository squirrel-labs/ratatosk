use std::f32::consts::FRAC_PI_2;

use rask_engine::math::{Mat3, Vec3};

const MAT: Mat3 = Mat3::new(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);

fn mat_eq(mat1: &Mat3, mat2: &Mat3, precision: f32) -> bool {
    mat1.as_ref()
        .iter()
        .zip(mat2.as_ref())
        .all(|(a, b)| f32::abs(a - b) < precision)
}

#[test]
fn test_add_mat3() {
    assert_eq!(MAT + MAT, Mat3::new(0.0, 2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0));
}

#[test]
#[allow(clippy::eq_op)]
fn test_sub_mat3() {
    assert_eq!(MAT - MAT, Mat3::zero());
}

#[test]
fn test_neg_mat3() {
    assert_eq!(-MAT, Mat3::new(0.0, -1.0, -2.0, -3.0, -4.0, -5.0, -6.0, -7.0, -8.0));
}

#[test]
fn test_mul_f32() {
    assert_eq!(MAT * 2.0, 2.0 * MAT);
    assert_eq!(MAT * 2.0, Mat3::new(0.0, 2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0));
}

#[test]
fn test_mul_vec3() {
    let a = Vec3::new(1.0, 2.0, 3.0);

    assert_eq!(MAT * a, Vec3::new(8.0, 26.0, 44.0));
}

#[test]
fn test_mul_mat3() {
    assert_eq!(MAT * MAT, Mat3::new(15.0, 18.0, 21.0, 42.0, 54.0, 66.0, 69.0, 90.0, 111.0));
}

#[test]
fn test_div_f32() {
    assert_eq!(MAT / 0.5, Mat3::new(0.0, 2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0));
}

#[test]
fn test_zero() {
    let mat1 = Mat3::zero();
    let mat2 = Mat3::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    assert_eq!(mat1.as_ref(), mat2.as_ref());
}

#[test]
fn test_identity() {
    let mat1 = Mat3::identity();
    let mat2 = Mat3::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0);
    assert_eq!(mat1.as_ref(), mat2.as_ref());
}

#[test]
fn test_identity_mul() {
    let mat2 = Mat3::identity();
    println!("ident: {:?}", mat2);
    println!("mat1(): {:?}", MAT);
    assert_eq!((MAT * mat2).as_ref(), MAT.as_ref());
}

#[test]
fn test_new() {
    let mat2 = [0.0, 3.0, 6.0, 1.0, 4.0, 7.0, 2.0, 5.0, 8.0];
    assert_eq!(MAT.as_ref(), &mat2);
}

#[test]
fn test_transpose() {
    let mat2 = Mat3::new(0.0, 3.0, 6.0, 1.0, 4.0, 7.0, 2.0, 5.0, 8.0);
    assert_eq!(&MAT.transpose(), &mat2);
}

#[test]
fn test_rotation() {
    let mat2 = Mat3::new(0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    assert!(mat_eq(
        &Mat3::rotation(FRAC_PI_2),
        &mat2,
        1e-7,
    ));
}
