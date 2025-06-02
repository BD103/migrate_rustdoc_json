//! <https://github.com/rust-lang/rustdoc-types/blob/trunk/CHANGELOG.md#v0390---2025-03-24>
//!
//! # `#[repr(...)]` Debug Format
//!
//! > **Syntax**
//! >
//! > _ReprAttrs_: `#[attr = Repr([` _ReprAttr_ (`, ` _ReprAttr_)<sup>\*</sup> `])]`\n
//! >
//! > _ReprAttr_:
//! >     `ReprInt(` _IntType_ `)`
//! >   | `ReprRust`
//! >   | `ReprC`
//! >   | `ReprPacked(` _Align_ `)`
//! >   | `ReprSimd`
//! >   | `ReprTransparent`
//! >   | `ReprAlign(` _Align_ `)`
//! >   | `ReprEmpty`
//! >
//! > _IntType_:
//! >     `SignedInt(` _IntTy_ `)`
//! >   | `UnsignedInt(` _UintTy_ `)`
//! >
//! > _IntTy_:
//! >     `Isize`
//! >   | `I8`
//! >   | `I16`
//! >   | `I32`
//! >   | `I64`
//! >   | `I128`
//! >
//! > _UintTy_:
//! >     `Usize`
//! >   | `U8`
//! >   | `U16`
//! >   | `U32`
//! >   | `U64`
//! >   | `U128`
//! >
//! > _Align_: `Align(` [`0`-`9`]<sup>+</sup> ` bytes)`
//!
//! Note that _Align_'s value is a [`u8`] and is always a power of 2.

use rustdoc_types_42 as current;
use rustdoc_types_43 as up;
use winnow::Parser;

use crate::{declare_migrate_up, declare_serialize_deserialize, traits::MigrateUp};

declare_migrate_up!(42, 43);
declare_serialize_deserialize!();

impl MigrateUp for current::Item {
    type Up = up::Item;

    fn migrate_up(self) -> Self::Up {
        let current::Item {
            id,
            crate_id,
            name,
            span,
            visibility,
            docs,
            links,
            attrs,
            deprecation,
            inner,
        } = self;

        let attrs = attrs
            .into_iter()
            .filter_map(
                |debug_attr| match parser::parse_repr_attrs.parse(&debug_attr) {
                    Ok(pretty_attr) => pretty_attr,
                    // If the attribute could not be parsed, usually meaning it wasn't a
                    // `#[attr = Repr(...)]`, simply return the original value.
                    Err(_) => Some(debug_attr),
                },
            )
            .collect();

        up::Item {
            id: id.migrate_up(),
            crate_id,
            name,
            span: span.migrate_up(),
            visibility: visibility.migrate_up(),
            docs,
            links: links.migrate_up(),
            attrs,
            deprecation: deprecation.migrate_up(),
            inner: inner.migrate_up(),
        }
    }
}

crate::impl_unchanged_migrations! {
    Crate,
    AssocItemConstraint,
    Constant,
    Deprecation,
    Discriminant,
    DynTrait,
    Enum,
    ExternalCrate,
    Function,
    FunctionHeader,
    FunctionPointer,
    FunctionSignature,
    GenericParamDef,
    Generics,
    Id,
    Impl,
    ItemSummary,
    Module,
    Path,
    PolyTrait,
    Primitive,
    ProcMacro,
    Span,
    Static,
    Struct,
    Trait,
    TraitAlias,
    TypeAlias,
    Union,
    Use,
    Variant,
    Abi,
    AssocItemConstraintKind,
    GenericArg,
    GenericArgs@v42,
    GenericBound,
    GenericParamDefKind,
    ItemEnum,
    ItemKind,
    MacroKind,
    PreciseCapturingArg,
    StructKind,
    Term,
    TraitBoundModifier,
    Type,
    VariantKind,
    Visibility,
    WherePredicate,
}

mod parser {
    use winnow::{
        Parser,
        ascii::{alpha1, alphanumeric1, digit1},
        combinator::{delimited, dispatch, empty, fail, separated},
        error::Result,
    };

    pub fn parse_repr_attrs(input: &mut &str) -> Result<Option<String>> {
        let mut reprs: Vec<_> = delimited(
            "#[attr = Repr([",
            separated(1.., parse_repr_attr, ", "),
            "])]\n",
        )
        .parse_next(input)?;

        // Only keep non-empty strings, thus removing an `ReprEmpty`s.
        reprs.retain(|s| !s.is_empty());

        // If there are no reprs, usually because they got filtered out, return `None`.
        if reprs.is_empty() {
            return Ok(None);
        }

        Ok(Some(format!("#[repr({})]", reprs.join(", "))))
    }

    fn parse_repr_attr(input: &mut &str) -> Result<String> {
        dispatch! { alpha1;
            "ReprInt" => delimited("(", parse_int_type, ")").map(String::from),
            "ReprRust" => empty.value(String::from("Rust")),
            "ReprC" => empty.value(String::from("C")),
            "ReprPacked" => delimited("(", parse_align, ")").map(|s| format!("packed({s})")),
            "ReprSimd" => empty.value(String::from("simd")),
            "ReprTransparent" => empty.value(String::from("transparent")),
            "ReprAlign" => delimited("(", parse_align, ")").map(|s| format!("align({s})")),
            // This shouldn't realistically ever be hit. If it is, we return an empty string.
            // (`parse_repr_attrs()` has custom logic for detecting this and removing it.)
            "ReprEmpty" => empty.default_value(),
            _ => fail::<_, _, _>,
        }
        .parse_next(input)
    }

    fn parse_int_type<'s>(input: &mut &'s str) -> Result<&'s str> {
        dispatch! { alpha1;
            "SignedInt" => delimited("(", parse_int_ty, ")"),
            "UnsignedInt" => delimited("(", parse_uint_ty, ")"),
            _ => fail::<_, _, _>,
        }
        .parse_next(input)
    }

    fn parse_int_ty<'s>(input: &mut &'s str) -> Result<&'s str> {
        dispatch! { alphanumeric1;
            "Isize" => empty.value("isize"),
            "I8" => empty.value("i8"),
            "I16" => empty.value("i16"),
            "I32" => empty.value("i32"),
            "I64" => empty.value("i64"),
            "I128" => empty.value("i128"),
            _ => fail::<_, _, _>,
        }
        .parse_next(input)
    }

    fn parse_uint_ty<'s>(input: &mut &'s str) -> Result<&'s str> {
        dispatch! { alphanumeric1;
            "Usize" => empty.value("usize"),
            "U8" => empty.value("u8"),
            "U16" => empty.value("u16"),
            "U32" => empty.value("u32"),
            "U64" => empty.value("u64"),
            "U128" => empty.value("u128"),
            _ => fail::<_, _, _>,
        }
        .parse_next(input)
    }

    fn parse_align<'s>(input: &mut &'s str) -> Result<&'s str> {
        delimited("Align(", digit1, " bytes)").parse_next(input)
    }
}
