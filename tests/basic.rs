use rawenum::rawenum;

// Note on testing for absence of methods:
// To rigorously test that methods for *other* types are *not* generated,
// you would typically use a tool like `trybuild`. This tool allows you
// to write small Rust files that you expect to either compile successfully
// or fail with specific compilation errors (like "no method named `from_i64`").
// This cannot be easily demonstrated within a single `#[test]` function
// in the same file where the macro is used successfully.

// --- Test Case 1: Enum with only explicit discriminants, specifying i32 and u8 ---
#[rawenum(i32, u8)]
#[derive(Debug, PartialEq)]
enum ExplicitEnum {
    Zero = 0,
    One = 1,
    Ten = 10,
    NegativeFive = -5, // -5 as u8 is 251
}

#[test]
fn test_explicit_enum_i32_u8() {
    // Test from_i32 (should be generated)
    assert_eq!(ExplicitEnum::from_i32(0), Some(ExplicitEnum::Zero));
    assert_eq!(ExplicitEnum::from_i32(1), Some(ExplicitEnum::One));
    assert_eq!(ExplicitEnum::from_i32(10), Some(ExplicitEnum::Ten));
    assert_eq!(ExplicitEnum::from_i32(-5), Some(ExplicitEnum::NegativeFive));
    assert_eq!(ExplicitEnum::from_i32(99), None); // No match

    // Test from_u8 (should be generated)
    assert_eq!(ExplicitEnum::from_u8(0), Some(ExplicitEnum::Zero));
    assert_eq!(ExplicitEnum::from_u8(1), Some(ExplicitEnum::One));
    assert_eq!(ExplicitEnum::from_u8(10), Some(ExplicitEnum::Ten));
    assert_eq!(ExplicitEnum::from_u8(251), Some(ExplicitEnum::NegativeFive)); // -5 as u8 is 251
    assert_eq!(ExplicitEnum::from_u8(99), None); // No match
}

// --- Test Case 2: Enum with only implicit discriminants, specifying i16 and u64 ---
#[rawenum(i16, u64)]
#[derive(Debug, PartialEq)]
enum ImplicitEnum {
    A, // 0
    B, // 1
    C, // 2
}

#[test]
fn test_implicit_enum_i16_u64() {
    // Test from_i16 (should be generated)
    assert_eq!(ImplicitEnum::from_i16(0), Some(ImplicitEnum::A));
    assert_eq!(ImplicitEnum::from_i16(1), Some(ImplicitEnum::B));
    assert_eq!(ImplicitEnum::from_i16(2), Some(ImplicitEnum::C));
    assert_eq!(ImplicitEnum::from_i16(3), None); // No match
    assert_eq!(ImplicitEnum::from_i16(-1), None); // No match

    // Test from_u64 (should be generated)
    assert_eq!(ImplicitEnum::from_u64(0), Some(ImplicitEnum::A));
    assert_eq!(ImplicitEnum::from_u64(1), Some(ImplicitEnum::B));
    assert_eq!(ImplicitEnum::from_u64(2), Some(ImplicitEnum::C));
    assert_eq!(ImplicitEnum::from_u64(3), None); // No match
}

// --- Test Case 3: Enum with mixed explicit and implicit discriminants, specifying i8 and i64 ---
#[rawenum(i8, i64)]
#[derive(Debug, PartialEq)]
enum MixedEnum {
    Start = 100, // 100 as i8 is 100
    Next,        // 101 as i8 is 101
    Jump = 200,  // 200 as i8 is -56 (wrapping)
    Another,     // 201 as i8 is -55 (wrapping)
    End,         // 202 as i8 is -54 (wrapping)
}

#[test]
fn test_mixed_enum_i8_i64() {
    // Test from_i8 (should be generated)
    assert_eq!(MixedEnum::from_i8(100), Some(MixedEnum::Start));
    assert_eq!(MixedEnum::from_i8(101), Some(MixedEnum::Next));
    assert_eq!(MixedEnum::from_i8(-56), Some(MixedEnum::Jump)); // 200 wraps to -56
    assert_eq!(MixedEnum::from_i8(-55), Some(MixedEnum::Another)); // 201 wraps to -55
    assert_eq!(MixedEnum::from_i8(-54), Some(MixedEnum::End)); // 202 wraps to -54
    assert_eq!(MixedEnum::from_i8(50), None); // No match

    // Test from_i64 (should be generated)
    assert_eq!(MixedEnum::from_i64(100), Some(MixedEnum::Start));
    assert_eq!(MixedEnum::from_i64(101), Some(MixedEnum::Next));
    assert_eq!(MixedEnum::from_i64(200), Some(MixedEnum::Jump));
    assert_eq!(MixedEnum::from_i64(201), Some(MixedEnum::Another));
    assert_eq!(MixedEnum::from_i64(202), Some(MixedEnum::End));
    assert_eq!(MixedEnum::from_i64(99), None); // Before range
    assert_eq!(MixedEnum::from_i64(150), None); // Between ranges
    assert_eq!(MixedEnum::from_i64(300), None); // After range
}

// --- Test Case 4: Enum with zero discriminant, specifying i32 ---
#[rawenum(i32)]
#[derive(Debug, PartialEq)]
enum ZeroEnum {
    First = 0,
    Second, // 1
}

#[test]
fn test_zero_enum_i32() {
    assert_eq!(ZeroEnum::from_i32(0), Some(ZeroEnum::First));
    assert_eq!(ZeroEnum::from_i32(1), Some(ZeroEnum::Second));
    assert_eq!(ZeroEnum::from_i32(2), None);
}

// --- Test Case 5: Enum with negative discriminants, specifying i8 and u8 ---
#[rawenum(i8, u8)]
#[derive(Debug, PartialEq)]
enum NegativeEnum {
    NegOne = -1, // -1 as u8 is 255
    NegTwo = -2, // -2 as u8 is 254
    Zero = 0,    // 0 as u8 is 0
    One = 1,     // 1 as u8 is 1
}

#[test]
fn test_negative_enum_i8_u8() {
    // Test from_i8 (should be generated)
    assert_eq!(NegativeEnum::from_i8(-1), Some(NegativeEnum::NegOne));
    assert_eq!(NegativeEnum::from_i8(-2), Some(NegativeEnum::NegTwo));
    assert_eq!(NegativeEnum::from_i8(0), Some(NegativeEnum::Zero));
    assert_eq!(NegativeEnum::from_i8(1), Some(NegativeEnum::One));
    assert_eq!(NegativeEnum::from_i8(99), None);

    // Test from_u8 (should be generated, checking wrapping)
    assert_eq!(NegativeEnum::from_u8(255), Some(NegativeEnum::NegOne));
    assert_eq!(NegativeEnum::from_u8(254), Some(NegativeEnum::NegTwo));
    assert_eq!(NegativeEnum::from_u8(0), Some(NegativeEnum::Zero));
    assert_eq!(NegativeEnum::from_u8(1), Some(NegativeEnum::One));
    assert_eq!(NegativeEnum::from_u8(99), None);
}
