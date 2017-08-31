use data::*;
use super::assert_almost_eq;

// Metric
#[test]
fn kilooctet() {
    let i1 = Data::from_octets(100);
    let r1 = i1.as_kilooctets();

    let i2 = Data::from_kilooctets(100.0);
    let r2 = i2.as_octets();

    assert_almost_eq(r1, 0.1);
    assert_eq!(r2, 100000);
}

#[test]
fn megaoctet() {
    let i1 = Data::from_octets(100);
    let r1 = i1.as_megaoctets();

    let i2 = Data::from_megaoctets(100.0);
    let r2 = i2.as_octets();

    assert_almost_eq(r1, 0.0001);
    assert_eq!(r2, 100000000);
}

#[test]
fn gigaoctet() {
    let i1 = Data::from_octets(100);
    let r1 = i1.as_gigaoctets();

    let i2 = Data::from_gigaoctets(100.0);
    let r2 = i2.as_octets();

    assert_almost_eq(r1, 1e-7);
    assert_eq!(r2, 100000000000);
}

#[test]
fn teraoctet() {
    let i1 = Data::from_octets(100);
    let r1 = i1.as_teraoctets();

    let i2 = Data::from_teraoctets(100.0);
    let r2 = i2.as_octets();

    assert_almost_eq(r1, 1e-10);
    assert_eq!(r2, 100000000000000);
}

// Imperial
#[test]
fn kibioctet() {
    let i1 = Data::from_octets(100);
    let r1 = i1.as_kibioctets();

    let i2 = Data::from_kibioctets(100.0);
    let r2 = i2.as_octets();

    assert_almost_eq(r1, 0.09765625);
    assert_eq!(r2, 102400);
}

#[test]
fn mebioctet() {
    let i1 = Data::from_octets(100);
    let r1 = i1.as_mebioctets();

    let i2 = Data::from_mebioctets(100.0);
    let r2 = i2.as_octets();

    assert_almost_eq(r1, 9.536743e-5);
    assert_eq!(r2, 104857600);
}

#[test]
fn gibioctets() {
    let i1 = Data::from_octets(100);
    let r1 = i1.as_gibioctets();

    let i2 = Data::from_gibioctets(100.0);
    let r2 = i2.as_octets();

    assert_almost_eq(r1, 9.313226e-8);
    assert_eq!(r2, 107374182400);
}

#[test]
fn tebioctets() {
    let i1 = Data::from_octets(100);
    let r1 = i1.as_tebioctets();

    let i2 = Data::from_tebioctets(100.0);
    let r2 = i2.as_octets();

    assert_almost_eq(r1, 9.094947e-11);
    assert_eq!(r2, 109951162777600);
}

// Traits
#[test]
fn add() {
    let a = Data::from_octets(2);
    let b = Data::from_octets(4);
    let c = a + b;
    assert_eq!(c.as_octets(), 6);
}

#[test]
fn sub() {
    let a = Data::from_octets(4);
    let b = Data::from_octets(2);
    let c = a - b;
    assert_eq!(c.as_octets(), 2);
}

#[test]
fn mul() {
    let a = Data::from_octets(2);
    let c = a * 2.0;
    assert_eq!(c.as_octets(), 4);
}

#[test]
fn eq() {
    let a = Data::from_octets(2);
    let b = Data::from_octets(2);
    assert_eq!(a == b, true);
}

#[test]
fn neq() {
    let a = Data::from_octets(2);
    let b = Data::from_octets(4);
    assert_eq!(a == b, false);
}

#[test]
fn cmp() {
    let a = Data::from_octets(2);
    let b = Data::from_octets(4);
    assert_eq!(a < b, true);
    assert_eq!(a <= b, true);
    assert_eq!(a > b, false);
    assert_eq!(a >= b, false);
}
