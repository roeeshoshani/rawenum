use rawenum::rawenum;

// --- Test Case 1: Enum with only explicit discriminants ---
#[rawenum]
#[derive(Debug, PartialEq)]
enum ExplicitEnum {
    Zero = 0,
    One = 1,
    Ten = 10,
    NegativeFive = -5,
}

#[test]
fn test_explicit_enum() {
    // Test from_i8
    assert_eq!(ExplicitEnum::from_i8(0), Some(ExplicitEnum::Zero));
    assert_eq!(ExplicitEnum::from_i8(1), Some(ExplicitEnum::One));
    assert_eq!(ExplicitEnum::from_i8(10), Some(ExplicitEnum::Ten));
    assert_eq!(ExplicitEnum::from_i8(-5), Some(ExplicitEnum::NegativeFive));
    assert_eq!(ExplicitEnum::from_i8(99), None); // No match

    // Test from_u8 (should not match negative values)
    assert_eq!(ExplicitEnum::from_u8(0), Some(ExplicitEnum::Zero));
    assert_eq!(ExplicitEnum::from_u8(1), Some(ExplicitEnum::One));
    assert_eq!(ExplicitEnum::from_u8(10), Some(ExplicitEnum::Ten));
    assert_eq!(ExplicitEnum::from_u8(255), None); // No match (discriminant -5 as u8 is 251)
    assert_eq!(ExplicitEnum::from_u8(251), Some(ExplicitEnum::NegativeFive)); // -5 as u8 is 251
    assert_eq!(ExplicitEnum::from_u8(99), None); // No match

    // Test from_i16
    assert_eq!(ExplicitEnum::from_i16(0), Some(ExplicitEnum::Zero));
    assert_eq!(ExplicitEnum::from_i16(1), Some(ExplicitEnum::One));
    assert_eq!(ExplicitEnum::from_i16(10), Some(ExplicitEnum::Ten));
    assert_eq!(ExplicitEnum::from_i16(-5), Some(ExplicitEnum::NegativeFive));
    assert_eq!(ExplicitEnum::from_i16(1000), None); // No match

    // Test from_u16
    assert_eq!(ExplicitEnum::from_u16(0), Some(ExplicitEnum::Zero));
    assert_eq!(ExplicitEnum::from_u16(1), Some(ExplicitEnum::One));
    assert_eq!(ExplicitEnum::from_u16(10), Some(ExplicitEnum::Ten));
    assert_eq!(
        ExplicitEnum::from_u16(65531),
        Some(ExplicitEnum::NegativeFive)
    ); // -5 as u16 is 65531
    assert_eq!(ExplicitEnum::from_u16(1000), None); // No match

    // Test from_i32
    assert_eq!(ExplicitEnum::from_i32(0), Some(ExplicitEnum::Zero));
    assert_eq!(ExplicitEnum::from_i32(1), Some(ExplicitEnum::One));
    assert_eq!(ExplicitEnum::from_i32(10), Some(ExplicitEnum::Ten));
    assert_eq!(ExplicitEnum::from_i32(-5), Some(ExplicitEnum::NegativeFive));
    assert_eq!(ExplicitEnum::from_i32(100000), None); // No match

    // Test from_u32
    assert_eq!(ExplicitEnum::from_u32(0), Some(ExplicitEnum::Zero));
    assert_eq!(ExplicitEnum::from_u32(1), Some(ExplicitEnum::One));
    assert_eq!(ExplicitEnum::from_u32(10), Some(ExplicitEnum::Ten));
    assert_eq!(
        ExplicitEnum::from_u32(4294967291),
        Some(ExplicitEnum::NegativeFive)
    ); // -5 as u32
    assert_eq!(ExplicitEnum::from_u32(100000), None); // No match

    // Test from_i64
    assert_eq!(ExplicitEnum::from_i64(0), Some(ExplicitEnum::Zero));
    assert_eq!(ExplicitEnum::from_i64(1), Some(ExplicitEnum::One));
    assert_eq!(ExplicitEnum::from_i64(10), Some(ExplicitEnum::Ten));
    assert_eq!(ExplicitEnum::from_i64(-5), Some(ExplicitEnum::NegativeFive));
    assert_eq!(ExplicitEnum::from_i64(10000000000), None); // No match

    // Test from_u64
    assert_eq!(ExplicitEnum::from_u64(0), Some(ExplicitEnum::Zero));
    assert_eq!(ExplicitEnum::from_u64(1), Some(ExplicitEnum::One));
    assert_eq!(ExplicitEnum::from_u64(10), Some(ExplicitEnum::Ten));
    assert_eq!(
        ExplicitEnum::from_u64(18446744073709551611),
        Some(ExplicitEnum::NegativeFive)
    ); // -5 as u64
    assert_eq!(ExplicitEnum::from_u64(10000000000), None); // No match
}

// --- Test Case 2: Enum with only implicit discriminants ---
#[rawenum]
#[derive(Debug, PartialEq)]
enum ImplicitEnum {
    A, // 0
    B, // 1
    C, // 2
}

#[test]
fn test_implicit_enum() {
    // Test from_i8
    assert_eq!(ImplicitEnum::from_i8(0), Some(ImplicitEnum::A));
    assert_eq!(ImplicitEnum::from_i8(1), Some(ImplicitEnum::B));
    assert_eq!(ImplicitEnum::from_i8(2), Some(ImplicitEnum::C));
    assert_eq!(ImplicitEnum::from_i8(3), None); // No match
    assert_eq!(ImplicitEnum::from_i8(-1), None); // No match

    // Test from_u8
    assert_eq!(ImplicitEnum::from_u8(0), Some(ImplicitEnum::A));
    assert_eq!(ImplicitEnum::from_u8(1), Some(ImplicitEnum::B));
    assert_eq!(ImplicitEnum::from_u8(2), Some(ImplicitEnum::C));
    assert_eq!(ImplicitEnum::from_u8(3), None); // No match

    // Test from_i32
    assert_eq!(ImplicitEnum::from_i32(0), Some(ImplicitEnum::A));
    assert_eq!(ImplicitEnum::from_i32(1), Some(ImplicitEnum::B));
    assert_eq!(ImplicitEnum::from_i32(2), Some(ImplicitEnum::C));
    assert_eq!(ImplicitEnum::from_i32(3), None); // No match
    assert_eq!(ImplicitEnum::from_i32(-1), None); // No match

    // Test from_u64
    assert_eq!(ImplicitEnum::from_u64(0), Some(ImplicitEnum::A));
    assert_eq!(ImplicitEnum::from_u64(1), Some(ImplicitEnum::B));
    assert_eq!(ImplicitEnum::from_u64(2), Some(ImplicitEnum::C));
    assert_eq!(ImplicitEnum::from_u64(3), None); // No match
}

// --- Test Case 3: Enum with mixed explicit and implicit discriminants ---
#[rawenum]
#[derive(Debug, PartialEq)]
enum MixedEnum {
    Start = 100, // 100
    Next,        // 101
    Jump = 200,  // 200
    Another,     // 201
    End,         // 202
}

#[test]
fn test_mixed_enum() {
    // Test from_i16
    assert_eq!(MixedEnum::from_i16(100), Some(MixedEnum::Start));
    assert_eq!(MixedEnum::from_i16(101), Some(MixedEnum::Next));
    assert_eq!(MixedEnum::from_i16(200), Some(MixedEnum::Jump));
    assert_eq!(MixedEnum::from_i16(201), Some(MixedEnum::Another));
    assert_eq!(MixedEnum::from_i16(202), Some(MixedEnum::End));
    assert_eq!(MixedEnum::from_i16(99), None); // Before range
    assert_eq!(MixedEnum::from_i16(150), None); // Between ranges
    assert_eq!(MixedEnum::from_i16(300), None); // After range

    // Test from_u8 (checking wrapping behavior)
    // 100 as u8 is 100
    // 101 as u8 is 101
    // 200 as u8 is 200
    // 201 as u8 is 201
    // 202 as u8 is 202
    assert_eq!(MixedEnum::from_u8(100), Some(MixedEnum::Start));
    assert_eq!(MixedEnum::from_u8(101), Some(MixedEnum::Next));
    assert_eq!(MixedEnum::from_u8(200), Some(MixedEnum::Jump));
    assert_eq!(MixedEnum::from_u8(201), Some(MixedEnum::Another));
    assert_eq!(MixedEnum::from_u8(202), Some(MixedEnum::End));
    assert_eq!(MixedEnum::from_u8(50), None); // No match

    // Test from_i8 (checking values outside i8 range)
    // 100 as i8 is 100
    // 101 as i8 is 101
    // 200 as i8 is -56 (wrapping)
    // 201 as i8 is -55 (wrapping)
    // 202 as i8 is -54 (wrapping)
    assert_eq!(MixedEnum::from_i8(100), Some(MixedEnum::Start));
    assert_eq!(MixedEnum::from_i8(101), Some(MixedEnum::Next));
    assert_eq!(MixedEnum::from_i8(-56), Some(MixedEnum::Jump)); // 200 wraps to -56
    assert_eq!(MixedEnum::from_i8(-55), Some(MixedEnum::Another)); // 201 wraps to -55
    assert_eq!(MixedEnum::from_i8(-54), Some(MixedEnum::End)); // 202 wraps to -54
    assert_eq!(MixedEnum::from_i8(50), None); // No match
}

// --- Test Case 4: Enum with large discriminants (checking i64/u64) ---
#[rawenum]
#[derive(Debug, PartialEq)]
enum LargeDiscriminantEnum {
    Small = 10,
    Large = 1_000_000_000,          // 1 Billion
    Larger = 2_000_000_000,         // 2 Billion
    NegativeLarge = -1_000_000_000, // Negative 1 Billion
}

#[test]
fn test_large_discriminant_enum() {
    // Test from_i32 (Large and NegativeLarge are outside i32 range)
    assert_eq!(
        LargeDiscriminantEnum::from_i32(10),
        Some(LargeDiscriminantEnum::Small)
    );
    // 1_000_000_000 as i32 is 1_000_000_000
    assert_eq!(
        LargeDiscriminantEnum::from_i32(1_000_000_000),
        Some(LargeDiscriminantEnum::Large)
    );
    // 2_000_000_000 as i32 is 2_000_000_000
    assert_eq!(
        LargeDiscriminantEnum::from_i32(2_000_000_000),
        Some(LargeDiscriminantEnum::Larger)
    );
    // -1_000_000_000 as i32 is -1_000_000_000
    assert_eq!(
        LargeDiscriminantEnum::from_i32(-1_000_000_000),
        Some(LargeDiscriminantEnum::NegativeLarge)
    );
    assert_eq!(LargeDiscriminantEnum::from_i32(500), None); // No match

    // Test from_u32 (NegativeLarge is outside u32 range)
    assert_eq!(
        LargeDiscriminantEnum::from_u32(10),
        Some(LargeDiscriminantEnum::Small)
    );
    assert_eq!(
        LargeDiscriminantEnum::from_u32(1_000_000_000),
        Some(LargeDiscriminantEnum::Large)
    );
    assert_eq!(
        LargeDiscriminantEnum::from_u32(2_000_000_000),
        Some(LargeDiscriminantEnum::Larger)
    );
    // -1_000_000_000 as u32 wraps
    assert_eq!(
        LargeDiscriminantEnum::from_u32(3294967296u32),
        Some(LargeDiscriminantEnum::NegativeLarge)
    ); // -1_000_000_000 as u32
    assert_eq!(LargeDiscriminantEnum::from_u32(500), None); // No match

    // Test from_i64
    assert_eq!(
        LargeDiscriminantEnum::from_i64(10),
        Some(LargeDiscriminantEnum::Small)
    );
    assert_eq!(
        LargeDiscriminantEnum::from_i64(1_000_000_000),
        Some(LargeDiscriminantEnum::Large)
    );
    assert_eq!(
        LargeDiscriminantEnum::from_i64(2_000_000_000),
        Some(LargeDiscriminantEnum::Larger)
    );
    assert_eq!(
        LargeDiscriminantEnum::from_i64(-1_000_000_000),
        Some(LargeDiscriminantEnum::NegativeLarge)
    );
    assert_eq!(LargeDiscriminantEnum::from_i64(5_000_000_000), None); // No match

    // Test from_u64
    assert_eq!(
        LargeDiscriminantEnum::from_u64(10),
        Some(LargeDiscriminantEnum::Small)
    );
    assert_eq!(
        LargeDiscriminantEnum::from_u64(1_000_000_000),
        Some(LargeDiscriminantEnum::Large)
    );
    assert_eq!(
        LargeDiscriminantEnum::from_u64(2_000_000_000),
        Some(LargeDiscriminantEnum::Larger)
    );
    // -1_000_000_000 as u64 wraps
    assert_eq!(
        LargeDiscriminantEnum::from_u64(18446744072709551616u64),
        Some(LargeDiscriminantEnum::NegativeLarge)
    ); // -1_000_000_000 as u64
    assert_eq!(LargeDiscriminantEnum::from_u64(5_000_000_000), None); // No match
}

// --- Test Case 5: Enum with zero discriminant ---
#[rawenum]
#[derive(Debug, PartialEq)]
enum ZeroEnum {
    First = 0,
    Second, // 1
}

#[test]
fn test_zero_enum() {
    assert_eq!(ZeroEnum::from_i8(0), Some(ZeroEnum::First));
    assert_eq!(ZeroEnum::from_i8(1), Some(ZeroEnum::Second));
    assert_eq!(ZeroEnum::from_u8(0), Some(ZeroEnum::First));
    assert_eq!(ZeroEnum::from_u8(1), Some(ZeroEnum::Second));
    assert_eq!(ZeroEnum::from_i32(0), Some(ZeroEnum::First));
    assert_eq!(ZeroEnum::from_i32(1), Some(ZeroEnum::Second));
    assert_eq!(ZeroEnum::from_i8(2), None);
    assert_eq!(ZeroEnum::from_u8(2), None);
}

// --- Test Case 6: Enum with negative discriminants ---
#[rawenum]
#[derive(Debug, PartialEq)]
enum NegativeEnum {
    NegOne = -1,
    NegTwo = -2,
    Zero = 0,
    One = 1,
}

#[test]
fn test_negative_enum() {
    // Test from_i8
    assert_eq!(NegativeEnum::from_i8(-1), Some(NegativeEnum::NegOne));
    assert_eq!(NegativeEnum::from_i8(-2), Some(NegativeEnum::NegTwo));
    assert_eq!(NegativeEnum::from_i8(0), Some(NegativeEnum::Zero));
    assert_eq!(NegativeEnum::from_i8(1), Some(NegativeEnum::One));
    assert_eq!(NegativeEnum::from_i8(99), None);

    // Test from_u8 (checking wrapping)
    // -1 as u8 is 255
    // -2 as u8 is 254
    // 0 as u8 is 0
    // 1 as u8 is 1
    assert_eq!(NegativeEnum::from_u8(255), Some(NegativeEnum::NegOne));
    assert_eq!(NegativeEnum::from_u8(254), Some(NegativeEnum::NegTwo));
    assert_eq!(NegativeEnum::from_u8(0), Some(NegativeEnum::Zero));
    assert_eq!(NegativeEnum::from_u8(1), Some(NegativeEnum::One));
    assert_eq!(NegativeEnum::from_u8(99), None);

    // Test from_i32
    assert_eq!(NegativeEnum::from_i32(-1), Some(NegativeEnum::NegOne));
    assert_eq!(NegativeEnum::from_i32(-2), Some(NegativeEnum::NegTwo));
    assert_eq!(NegativeEnum::from_i32(0), Some(NegativeEnum::Zero));
    assert_eq!(NegativeEnum::from_i32(1), Some(NegativeEnum::One));
    assert_eq!(NegativeEnum::from_i32(999), None);
}
