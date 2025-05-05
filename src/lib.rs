use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DataEnum, DeriveInput, parse_macro_input};

/// A procedural macro to generate `from_*` methods for various integer types
/// for enums with explicit or implicit integer discriminants.
///
/// Apply this macro to an enum. It will generate `impl` blocks for the enum
/// with `from_i8`, `from_u8`, `from_i16`, `from_u16`, `from_i32`, `from_u32`,
/// `from_i64`, and `from_u64` functions. Each function takes a value of the
/// corresponding integer type and returns an `Option<Self>`.
///
/// Inside each `from_<type>` method, the macro generates a `const` for each
/// enum variant. This constant holds the variant's discriminant value cast to
/// the specific `<type>` of the method. The function then matches the input
/// integer value against these locally defined constants.
///
/// This correctly handles variants with implicit discriminants (starting from 0
/// or the previous variant's value + 1).
///
/// Returns `Some(variant)` if the input integer value matches the discriminant
/// (when cast to the method's type) of a variant. Returns `None` otherwise.
/// Note that casting the discriminant to a smaller type might result in
/// wrapping or truncation, which affects the values being matched against.
///
/// # Example
///
/// ```rust
/// use rawenum_macro::rawenum;
///
/// #[rawenum]
/// #[derive(Debug, PartialEq)] // Add derives if needed for testing/usage
/// enum MyEnum {
///     VariantA = 1, // Explicit discriminant
///     VariantB,     // Implicit discriminant (will be 2)
///     VariantC = 256, // Explicit discriminant (test casting implications)
///     VariantD,     // Implicit discriminant (will be 257)
/// }
///
/// // from_i8(1) -> Some(VariantA) (1 as i8 == 1 as i8)
/// let a_i8: Option<MyEnum> = MyEnum::from_i8(1);
/// assert_eq!(a_i8, Some(MyEnum::VariantA));
///
/// // from_u32(2) -> Some(VariantB) (2 as u32 == 2 as u32)
/// let b_u32: Option<MyEnum> = MyEnum::from_u32(2); // Testing implicit discriminant
/// assert_eq!(b_u32, Some(MyEnum::VariantB));
///
/// // from_i64(256) -> Some(VariantC) (256 as i64 == 256 as i64)
/// let c_i64: Option<MyEnum> = MyEnum::from_i64(256);
/// assert_eq!(c_i64, Some(MyEnum::VariantC));
///
/// // from_u8(0) -> Some(VariantC) because 256 as u8 is 0.
/// // The match is against the discriminant value *after* casting to u8.
/// let wrapped_c_u8: Option<MyEnum> = MyEnum::from_u8(0);
/// assert_eq!(wrapped_c_u8, Some(MyEnum::VariantC));
///
/// // from_i16(257) -> Some(VariantD) (257 as i16 == 257 as i16)
/// let d_i16: Option<MyEnum> = MyEnum::from_i16(257); // Testing implicit discriminant after explicit
/// assert_eq!(d_i16, Some(MyEnum::VariantD));
///
/// let e_i16: Option<MyEnum> = MyEnum::from_i16(99); // No variant with discriminant 99 (as i16)
/// assert_eq!(e_i16, None);
/// ```
#[proc_macro_attribute]
pub fn rawenum(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident; // The name of the enum

    // Ensure the input is an enum, otherwise return a compile error.
    let Data::Enum(DataEnum { variants, .. }) = &input.data else {
        return syn::Error::new_spanned(input, "rawenum can only be applied to enums")
            .to_compile_error()
            .into();
    };

    // Define the integer types for which we want to generate `from_*` methods
    let integer_types = vec![
        ("i8", quote! { i8 }),
        ("u8", quote! { u8 }),
        ("i16", quote! { i16 }),
        ("u16", quote! { u16 }),
        ("i32", quote! { i32 }),
        ("u32", quote! { u32 }),
        ("i64", quote! { i64 }),
        ("u64", quote! { u64 }),
    ];

    // This vector will collect the code for all generated methods.
    let mut all_generated_methods = Vec::new();

    // Generate `impl` block and all the `from_*` functions
    for (type_str, type_tokens) in integer_types.iter() {
        // Create the function name identifier, e.g., `from_i8`
        let fn_name = format_ident!("from_{}", type_str);

        // Vectors to hold const declarations and match arms *for this specific method*
        let mut local_generated_consts = Vec::new();
        let mut local_match_arms = Vec::new();

        // Generate `const` declarations for each variant *within this method*,
        // casting to the current target integer type.
        for variant in variants {
            let variant_name = &variant.ident; // Name of the variant

            // Create a unique const name for each variant *and* type,
            // e.g., `__RAWENUM_MYENUM_VARIANTA_DISCRIMINANT_I8`
            let const_name = format_ident!(
                "__RAWENUM_{}_DISCRIMINANT_{}_{}",
                name.to_string().to_uppercase(),
                variant_name.to_string().to_uppercase(),
                type_str.to_uppercase()
            );

            // Generate the const declaration:
            // `const ENUM_VARIANT_DISCRIMINANT_TYPE: TargetType = EnumName::VariantName as TargetType;`
            // This cast is crucial and happens here, local to the method.
            local_generated_consts.push(quote! {
                const #const_name: #type_tokens = #name::#variant_name as #type_tokens;
            });

            // Generate the match arm using the generated const: `CONST_NAME_TYPE => Some(Self::VariantName),`
            local_match_arms.push(quote! {
                #const_name => Some(Self::#variant_name),
            });
        }

        // Add the catch-all arm for any value that doesn't match any discriminant
        // (within the range of the target type after casting the discriminant).
        local_match_arms.push(quote! {
            _ => None,
        });

        // Generate the code for a single `from_*` function
        let method_code = quote! {
            #[allow(dead_code)] // Allow this function to be unused without a warning
            /// Converts a raw #type_tokens integer value to an Option<Self>.
            ///
            /// Returns `Some(variant)` if the value matches the discriminant
            /// (when cast to #type_tokens) of a variant. Returns `None` otherwise.
            pub fn #fn_name(value: #type_tokens) -> Option<Self> {
                // Include the locally generated consts here
                #( #local_generated_consts )*

                // Match the input value directly against the constants of the same type.
                match value {
                    // Expand all the collected local match arms
                    #( #local_match_arms )*
                }
            }
        };

        // Add the completed method code to the list.
        all_generated_methods.push(method_code);
    }

    // Combine the original enum definition and the generated methods within the impl block.
    let expanded = quote! {
        #input // Include the original enum definition

        impl #name {
            #( #all_generated_methods )* // Expand all the generated from_* methods
        }
    };

    // Convert the generated code back to a TokenStream and return it
    expanded.into()
}
