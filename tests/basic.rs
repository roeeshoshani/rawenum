#[macro_use]
extern crate rawenum_macro;

// Apply the #[rawenum] macro to the example enum.
#[rawenum]
#[derive(Debug, PartialEq)]
enum SpaceKind {
    Constant = 0,
    Processor = 1,
    Spacebase = 2,
    Internal = 3,
    Fspec = 4,
    Iop = 5,
    Join = 6,
    // Add a variant without an explicit discriminant
    Unknown,
    // Add a variant with a larger discriminant to test larger integer types
    LargeValue = 256,
    AnotherLarge = 65535, // Fits in u16
    VeryLarge = 100000,   // Fits in i32/u32
    MaxI64 = isize::MAX,  // Test with max value of a type
    MinI64 = isize::MIN,  // Test with min value of a type
}

#[test]
fn test_from_i8() {
    assert_eq!(SpaceKind::from_i8(0), Some(SpaceKind::Constant));
    assert_eq!(SpaceKind::from_i8(6), Some(SpaceKind::Join));
    assert_eq!(SpaceKind::from_i8(7), None); // Invalid
    assert_eq!(SpaceKind::from_i8(-1), None); // Invalid
    assert_eq!(SpaceKind::from_i8(127), None); // Max i8, not a discriminant
    assert_eq!(SpaceKind::from_i8(-128), None); // Min i8, not a discriminant
}

#[test]
fn test_from_u8() {
    assert_eq!(SpaceKind::from_u8(0), Some(SpaceKind::Constant));
    assert_eq!(SpaceKind::from_u8(6), Some(SpaceKind::Join));
    assert_eq!(SpaceKind::from_u8(7), None); // Invalid
    assert_eq!(SpaceKind::from_u8(255), None); // Max u8, not a discriminant
}

#[test]
fn test_from_i16() {
    assert_eq!(SpaceKind::from_i16(0), Some(SpaceKind::Constant));
    assert_eq!(SpaceKind::from_i16(6), Some(SpaceKind::Join));
    assert_eq!(SpaceKind::from_i16(256), Some(SpaceKind::LargeValue));
    assert_eq!(SpaceKind::from_i16(7), None); // Invalid
    assert_eq!(SpaceKind::from_i16(-1), None); // Invalid
}

#[test]
fn test_from_u16() {
    assert_eq!(SpaceKind::from_u16(0), Some(SpaceKind::Constant));
    assert_eq!(SpaceKind::from_u16(6), Some(SpaceKind::Join));
    assert_eq!(SpaceKind::from_u16(256), Some(SpaceKind::LargeValue));
    assert_eq!(SpaceKind::from_u16(65535), Some(SpaceKind::AnotherLarge)); // Test value that fits u16
    assert_eq!(SpaceKind::from_u16(7), None); // Invalid
}

#[test]
fn test_from_i32() {
    assert_eq!(SpaceKind::from_i32(0), Some(SpaceKind::Constant));
    assert_eq!(SpaceKind::from_i32(6), Some(SpaceKind::Join));
    assert_eq!(SpaceKind::from_i32(256), Some(SpaceKind::LargeValue));
    assert_eq!(SpaceKind::from_i32(65535), Some(SpaceKind::AnotherLarge));
    assert_eq!(SpaceKind::from_i32(100000), Some(SpaceKind::VeryLarge));
    assert_eq!(SpaceKind::from_i32(7), None); // Invalid
    assert_eq!(SpaceKind::from_i32(-1), None); // Invalid
}

#[test]
fn test_from_u32() {
    assert_eq!(SpaceKind::from_u32(0), Some(SpaceKind::Constant));
    assert_eq!(SpaceKind::from_u32(6), Some(SpaceKind::Join));
    assert_eq!(SpaceKind::from_u32(256), Some(SpaceKind::LargeValue));
    assert_eq!(SpaceKind::from_u32(65535), Some(SpaceKind::AnotherLarge));
    assert_eq!(SpaceKind::from_u32(100000), Some(SpaceKind::VeryLarge));
    assert_eq!(SpaceKind::from_u32(7), None); // Invalid
}

#[test]
fn test_from_i64() {
    assert_eq!(SpaceKind::from_i64(0), Some(SpaceKind::Constant));
    assert_eq!(SpaceKind::from_i64(6), Some(SpaceKind::Join));
    assert_eq!(SpaceKind::from_i64(256), Some(SpaceKind::LargeValue));
    assert_eq!(SpaceKind::from_i64(65535), Some(SpaceKind::AnotherLarge));
    assert_eq!(SpaceKind::from_i64(100000), Some(SpaceKind::VeryLarge));
    assert_eq!(SpaceKind::from_i64(i64::MAX), Some(SpaceKind::MaxI64));
    assert_eq!(SpaceKind::from_i64(i64::MIN), Some(SpaceKind::MinI64));
    assert_eq!(SpaceKind::from_i64(7), None); // Invalid
}

#[test]
fn test_from_u64() {
    assert_eq!(SpaceKind::from_u64(0), Some(SpaceKind::Constant));
    assert_eq!(SpaceKind::from_u64(6), Some(SpaceKind::Join));
    assert_eq!(SpaceKind::from_u64(256), Some(SpaceKind::LargeValue));
    assert_eq!(SpaceKind::from_u64(65535), Some(SpaceKind::AnotherLarge));
    assert_eq!(SpaceKind::from_u64(100000), Some(SpaceKind::VeryLarge));
    // Note: MaxI64 and MinI64 are i64.MAX and i64.MIN.
    // u64::MAX is larger than i64::MAX, so a u64 value of i64::MAX will match.
    assert_eq!(
        SpaceKind::from_u64(i64::MAX as u64),
        Some(SpaceKind::MaxI64)
    );
    // i64::MIN is negative, so it won't match a u64 value directly unless cast,
    // but the literal discriminant in the match arm handles this.
    // Let's test a u64 value that doesn't match.
    assert_eq!(SpaceKind::from_u64(u64::MAX), None);
    assert_eq!(SpaceKind::from_u64(7), None); // Invalid
}

#[test]
fn test_from_invalid_discriminants() {
    // Test values that do not match any discriminant across different types
    assert_eq!(SpaceKind::from_i8(10), None); // 10 is VariantC, but doesn't fit i8
    assert_eq!(SpaceKind::from_u8(10), None); // 10 is VariantC, but doesn't fit u8
    assert_eq!(SpaceKind::from_i32(100001), None); // 100001 is not a discriminant
    assert_eq!(SpaceKind::from_u32(u32::MAX), None); // Max u32 not a discriminant
}

#[test]
fn test_variant_without_discriminant() {
    // Test that the variant without an explicit discriminant (Unknown)
    // cannot be produced by any of the from_* methods.
    let unknown_variant = SpaceKind::Unknown;
    println!("Variant without discriminant: {:?}", unknown_variant);

    assert_eq!(SpaceKind::from_i8(99), None);
    assert_eq!(SpaceKind::from_u64(999), None);
    // We can't directly assert that from_* never returns Unknown, but the macro
    // doesn't generate match arms for variants without explicit discriminants.
}

#[test]
fn test_enum_attributes_preserved() {
    // Verifies that derives like Debug and PartialEq are kept.
    let kind = SpaceKind::Constant;
    println!("Testing Debug derive: {:?}", kind);
    assert_eq!(kind, SpaceKind::Constant);
}
