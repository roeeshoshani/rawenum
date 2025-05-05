extern crate proc_macro;

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
/// The macro works by generating a `const` for each enum variant, casting the
/// variant to `i64` to get its discriminant value (handled by the compiler),
/// and then using these `const`s in the `match` arms. This correctly handles
/// variants with implicit discriminants (starting from 0 or the previous variant's
/// value + 1).
///
/// Returns `Some(variant)` if the input integer value matches the discriminant
/// of a variant. Returns `None` otherwise.
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
///     VariantC = 5, // Explicit discriminant
///     VariantD,     // Implicit discriminant (will be 6)
/// }
///
/// let a_i8: Option<MyEnum> = MyEnum::from_i8(1);
/// assert_eq!(a_i8, Some(MyEnum::VariantA));
///
/// let b_u32: Option<MyEnum> = MyEnum::from_u32(2); // Testing implicit discriminant
/// assert_eq!(b_u32, Some(MyEnum::VariantB));
///
/// let c_i64: Option<MyEnum> = MyEnum::from_i64(5);
/// assert_eq!(c_i64, Some(MyEnum::VariantC));
///
/// let d_i16: Option<MyEnum> = MyEnum::from_i16(6); // Testing implicit discriminant
/// assert_eq!(d_i16, Some(MyEnum::VariantD));
///
/// let e_i16: Option<MyEnum> = MyEnum::from_i16(99); // No variant with discriminant 99
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

    // Generate `const` declarations for each variant by casting to i64.
    // This allows the compiler to figure out the discriminant value,
    // including for variants without explicit discriminants.
    let mut generated_consts = Vec::new();
    let mut match_arms = Vec::new();

    for variant in variants {
        let variant_name = &variant.ident; // Name of the variant
        // Create a unique const name for each variant, e.g., `MYENUM_VARIANTA_DISCRIMINANT`
        let const_name = format_ident!(
            "{}_DISCRIMINANT_{}",
            name.to_string().to_uppercase(),
            variant_name.to_string().to_uppercase()
        );

        // Generate the const declaration: `const ENUM_VARIANT_DISCRIMINANT: i64 = EnumName::VariantName as i64;`
        generated_consts.push(quote! {
            #[allow(non_upper_case_globals)] // Allow the generated const names
            const #const_name: i64 = #name::#variant_name as i64;
        });

        // Generate the match arm using the generated const: `CONST_NAME => Some(Self::VariantName),`
        match_arms.push(quote! {
            #const_name => Some(Self::#variant_name),
        });
    }

    // Add the catch-all arm for any value that doesn't match any discriminant.
    match_arms.push(quote! {
        _ => None,
    });

    // Define the integer types for which we want to generate `from_*` methods
    let integer_types = vec![
        ("i8", quote! { i8 }),
        ("u8", quote! { u8 }),
        ("i16", quote! { i16 }),
        ("u16", quote! { u16 }),
        ("i32", quote! { i32 }),
        ("u32", quote! { u32 }),
        ("i64", quote! { i64 }), // The type we cast to in the consts
        ("u64", quote! { u64 }),
    ];

    // Generate the `impl` block and all the `from_*` functions
    let generated_methods = integer_types.iter().map(|(type_str, type_tokens)| {
        // Create the function name identifier, e.g., `from_i8`
        let fn_name = format_ident!("from_{}", type_str);

        // Generate the code for a single `from_*` function
        quote! {
            /// Converts a raw #type_tokens integer value to an Option<Self>.
            ///
            /// Returns `Some(variant)` if the value matches the discriminant
            /// of a variant. Returns `None` otherwise.
            #[allow(dead_code)] // Allow this function to be unused without a warning
            pub fn #fn_name(value: #type_tokens) -> Option<Self> {
                // Cast the input value to i64 before matching against the i64 consts.
                // This handles potential sign extension or truncation as needed.
                match value as i64 {
                    // Expand all the collected match arms
                    #( #match_arms )*
                }
            }
        }
    });

    // Combine the original enum definition, the generated consts, and the generated methods
    let expanded = quote! {
        #input // Include the original enum definition

        // Include the generated consts within an anonymous block to prevent name conflicts
        // if the macro is applied multiple times in the same scope.
        const _: () = {
            #( #generated_consts )* // Expand all the generated consts
        };

        impl #name {
            #( #generated_methods )* // Expand all the generated from_* methods
        }
    };

    // Convert the generated code back to a TokenStream and return it
    expanded.into()
}
