use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Data, DataEnum, DeriveInput, Token, Type,
    parse::{Parse, ParseStream},
    parse_macro_input,
    spanned::Spanned,
};

// Helper struct to parse the attribute arguments (the specified types)
struct AllowedTypes {
    types: Vec<Type>,
}

impl Parse for AllowedTypes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut types = Vec::new();
        // Parse a comma-separated list of types
        while !input.is_empty() {
            let ty: Type = input.parse()?;
            types.push(ty);
            // If there's more input, expect a comma
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(AllowedTypes { types })
    }
}

/// A procedural macro to generate `from_*` methods for specific integer types
/// for enums with explicit or implicit integer discriminants.
///
/// Apply this macro to an enum, providing a comma-separated list of integer
/// types for which `from_*` methods should be generated.
///
/// The macro will generate `impl` blocks for the enum with `from_<type>`
/// functions for each specified type (e.g., `from_i32`, `from_u8`). Each
/// function takes a value of the corresponding integer type and returns an
/// `Option<Self>`.
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
/// use rawenum::rawenum;
///
/// #[rawenum(i32, u8)] // Specify the desired integer types
/// #[derive(Debug, PartialEq)] // Add derives if needed for testing/usage
/// enum MyEnum {
///     VariantA = 1, // Explicit discriminant
///     VariantB,     // Implicit discriminant (will be 2)
///     VariantC = 256, // Explicit discriminant (test casting implications)
///     VariantD,     // Implicit discriminant (will be 257)
/// }
///
/// // Only from_i32 and from_u8 methods are generated
/// let a_i32: Option<MyEnum> = MyEnum::from_i32(1);
/// assert_eq!(a_i32, Some(MyEnum::VariantA));
///
/// let b_u8: Option<MyEnum> = MyEnum::from_u8(2); // Testing implicit discriminant
/// assert_eq!(b_u8, Some(MyEnum::VariantB));
///
/// // Attempting to call a non-generated method like from_i64 would be a compile error
/// // let c_i64: Option<MyEnum> = MyEnum::from_i64(256); // This line would cause a compile error
///
/// // Test wrapping behavior for u8
/// let wrapped_c_u8: Option<MyEnum> = MyEnum::from_u8(0); // 256 as u8 is 0
/// assert_eq!(wrapped_c_u8, Some(MyEnum::VariantC));
///
/// let e_i32: Option<MyEnum> = MyEnum::from_i32(99); // No variant with discriminant 99 (as i32)
/// assert_eq!(e_i32, None);
/// ```
#[proc_macro_attribute]
pub fn rawenum(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident; // The name of the enum

    // Parse the specified integer types from the attribute arguments
    let allowed_types = parse_macro_input!(attr as AllowedTypes);
    let specified_types = allowed_types.types;

    // Ensure at least one type was specified
    if specified_types.is_empty() {
        return syn::Error::new_spanned(
            input,
            "at least one integer type must be specified, e.g., #[rawenum(i32)]",
        )
        .to_compile_error()
        .into();
    }

    // Ensure the input is an enum, otherwise return a compile error.
    let Data::Enum(DataEnum { variants, .. }) = &input.data else {
        return syn::Error::new_spanned(input, "rawenum can only be applied to enums")
            .to_compile_error()
            .into();
    };

    // This vector will collect the code for all generated methods.
    let mut all_generated_methods = Vec::new();

    // Supported integer types for validation
    const SUPPORTED_TYPES: &[&str] = &["i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64"];

    // Generate `impl` block and the `from_*` functions only for specified types
    for specified_type in specified_types {
        // Extract the identifier and span from the specified type
        let (type_ident, type_span) = match &specified_type {
            Type::Path(type_path) => {
                if let Some(segment) = type_path.path.segments.last() {
                    (segment.ident.clone(), segment.span())
                } else {
                    return syn::Error::new_spanned(specified_type, "invalid type specified")
                        .to_compile_error()
                        .into();
                }
            }
            _ => {
                return syn::Error::new_spanned(
                    specified_type,
                    "expected an integer type identifier (e.g., i32)",
                )
                .to_compile_error()
                .into();
            }
        };

        let type_str = type_ident.to_string();

        // Validate that the specified type is one of the supported integer types
        if !SUPPORTED_TYPES.contains(&type_str.as_str()) {
            return syn::Error::new_spanned(
                specified_type,
                format!(
                    "unsupported integer type '{}'. Supported types are {}.",
                    type_str,
                    SUPPORTED_TYPES.join(", ")
                ),
            )
            .to_compile_error()
            .into();
        }

        // Create the function name identifier with the correct span
        let fn_name = format_ident!("from_{}", type_str, span = type_span);

        // Vectors to hold const declarations and match arms *for this specific method*
        let mut local_generated_consts = Vec::new();
        let mut local_match_arms = Vec::new();

        // Generate `const` declarations for each variant *within this method*,
        // casting to the current target integer type.
        for variant in variants {
            let variant_name = &variant.ident; // Name of the variant
            let variant_span = variant_name.span(); // Span of the variant name

            // Create a unique const name for each variant *and* type, with the correct span
            let const_name = format_ident!(
                "__RAWENUM_{}_DISCRIMINANT_{}_{}",
                name.to_string().to_uppercase(),
                variant_name.to_string().to_uppercase(),
                type_str.to_uppercase(),
                span = variant_span
            );

            // Generate the const declaration:
            // `const ENUM_VARIANT_DISCRIMINANT_TYPE: TargetType = EnumName::VariantName as TargetType;`
            // Use #specified_type directly to preserve its span.
            local_generated_consts.push(quote! {
                const #const_name: #specified_type = #name::#variant_name as #specified_type;
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
            /// Converts a raw #specified_type integer value to an Option<Self>.
            ///
            /// Returns `Some(variant)` if the value matches the discriminant
            /// (when cast to #specified_type) of a variant. Returns `None` otherwise.
            pub fn #fn_name(value: #specified_type) -> Option<Self> {
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
