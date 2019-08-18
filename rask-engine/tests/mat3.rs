use rask_game_engine::math::*;

fn mat1() -> Mat3 {
    Mat3::new(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0)
}

fn mat_eq(mat1: &Mat3, mat2: &Mat3, precision: f32) -> bool {
    mat1.as_ref()
        .iter()
        .zip(mat2.as_ref())
        .all(|(a, b)| f32::abs(a - b) < precision)
}

#[test]
fn test_ident() {
    let mat1 = Mat3::identity();
    let mat2 = Mat3::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0);
    assert_eq!(mat1.as_ref(), mat2.as_ref());
}

#[test]
fn test_ident_mult() {
    let mat2 = Mat3::identity();
    println!("ident: {:?}", mat2);
    println!("mat1(): {:?}", mat1());
    assert_eq!((mat1() * mat2).as_ref(), mat1().as_ref());
}

#[test]
fn test_new() {
    let mat2 = [0.0, 3.0, 6.0, 1.0, 4.0, 7.0, 2.0, 5.0, 8.0];
    assert_eq!(mat1().as_ref(), &mat2);
}

#[test]
fn test_transpose() {
    let mat2 = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    assert_eq!(mat1().transpose().as_ref(), &mat2);
}
#[test]
fn test_rotation() {
    let mat2 = Mat3::new(-1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0);
    assert!(mat_eq(
        &Mat3::rotation(std::f32::consts::PI),
        &mat2,
        10f32.powi(-7)
    ));
}
