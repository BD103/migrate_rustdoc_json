//! **v42 to v43 migration.**
//!
//! The format of the stringified attributes in `Item::attrs` was changed for `#[repr(...)]`
//! attributes. In v42 `#[repr(...)]` attributes were represented in their [`Debug`] form, such as
//! `#[attr = Repr([ReprC, ReprInt(SignedInt(I8))])])`. v43 changes this to be the pretty printed
//! form, such as `#[repr(C,i8)]`.
//!
//! This migration parses the [`Debug`] representation using [`winnow`] in order to convert it to
//! the pretty-printed form. The syntax is listed below, and was found through experimentation and
//! by reading `AttributeKind`'s `PrintAttribute` implementation in
//! [commit `d93f678f`](https://github.com/rust-lang/rust/blob/d93f678fa55842cccd2f091deccd93e9494b3764/compiler/rustc_attr_data_structures/src/attributes.rs#L193)
//! of the Rust compiler.
//!
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
    use std::fmt;

    use winnow::{
        Parser,
        ascii::{alpha1, alphanumeric1, digit1},
        combinator::{delimited, dispatch, empty, fail, separated},
        error::Result,
    };

    #[derive(Clone)]
    enum ReprAttr<'s> {
        Int(IntType),
        Rust,
        C,
        Packed(&'s str),
        Simd,
        Transparent,
        Align(&'s str),
        Empty,
    }

    impl<'s> fmt::Display for ReprAttr<'s> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Int(int_type) => write!(f, "{int_type}"),
                Self::Rust => write!(f, "Rust"),
                Self::C => write!(f, "C"),
                Self::Packed(align) => write!(f, "packed({align})"),
                Self::Simd => write!(f, "simd"),
                Self::Transparent => write!(f, "transparent"),
                Self::Align(align) => write!(f, "align({align})"),
                Self::Empty => Err(fmt::Error),
            }
        }
    }

    #[derive(Clone)]
    enum IntType {
        Signed(IntSize),
        Unsigned(IntSize),
    }

    impl fmt::Display for IntType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Signed(int_size) => write!(f, "i{int_size}"),
                Self::Unsigned(int_size) => write!(f, "u{int_size}"),
            }
        }
    }

    #[derive(Clone)]
    enum IntSize {
        XSize,
        X8,
        X16,
        X32,
        X64,
        X128,
    }

    impl fmt::Display for IntSize {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                IntSize::XSize => write!(f, "size"),
                IntSize::X8 => write!(f, "8"),
                IntSize::X16 => write!(f, "16"),
                IntSize::X32 => write!(f, "32"),
                IntSize::X64 => write!(f, "64"),
                IntSize::X128 => write!(f, "128"),
            }
        }
    }

    pub fn parse_repr_attrs(input: &mut &str) -> Result<Option<String>> {
        let mut reprs: Vec<_> = delimited(
            "#[attr = Repr([",
            separated(1.., parse_repr_attr, ", "),
            "])]\n",
        )
        .parse_next(input)?;

        // Only keep non-empty reprs.
        reprs.retain(|s| !matches!(s, ReprAttr::Empty));

        // If there are no reprs, usually because they got filtered out, return `None`.
        if reprs.is_empty() {
            return Ok(None);
        }

        let reprs: Vec<_> = reprs.into_iter().map(|repr| format!("{repr}")).collect();

        Ok(Some(format!("#[repr({})]", reprs.join(", "))))
    }

    fn parse_repr_attr<'s>(input: &mut &'s str) -> Result<ReprAttr<'s>> {
        dispatch! { alpha1;
            "ReprInt" => delimited("(", parse_int_type, ")").map(ReprAttr::Int),
            "ReprRust" => empty.value(ReprAttr::Rust),
            "ReprC" => empty.value(ReprAttr::C),
            "ReprPacked" => delimited("(", parse_align, ")").map(ReprAttr::Packed),
            "ReprSimd" => empty.value(ReprAttr::Simd),
            "ReprTransparent" => empty.value(ReprAttr::Transparent),
            "ReprAlign" => delimited("(", parse_align, ")").map(ReprAttr::Align),
            // This shouldn't realistically ever be hit.
            "ReprEmpty" => empty.value(ReprAttr::Empty),
            _ => fail::<_, _, _>,
        }
        .parse_next(input)
    }

    fn parse_int_type(input: &mut &str) -> Result<IntType> {
        dispatch! { alpha1;
            "SignedInt" => delimited("(", parse_int_ty, ")").map(IntType::Signed),
            "UnsignedInt" => delimited("(", parse_uint_ty, ")").map(IntType::Unsigned),
            _ => fail::<_, _, _>,
        }
        .parse_next(input)
    }

    fn parse_int_ty(input: &mut &str) -> Result<IntSize> {
        dispatch! { alphanumeric1;
            "Isize" => empty.value(IntSize::XSize),
            "I8" => empty.value(IntSize::X8),
            "I16" => empty.value(IntSize::X16),
            "I32" => empty.value(IntSize::X32),
            "I64" => empty.value(IntSize::X64),
            "I128" => empty.value(IntSize::X128),
            _ => fail::<_, _, _>,
        }
        .parse_next(input)
    }

    fn parse_uint_ty(input: &mut &str) -> Result<IntSize> {
        dispatch! { alphanumeric1;
            "Usize" => empty.value(IntSize::XSize),
            "U8" => empty.value(IntSize::X8),
            "U16" => empty.value(IntSize::X16),
            "U32" => empty.value(IntSize::X32),
            "U64" => empty.value(IntSize::X64),
            "U128" => empty.value(IntSize::X128),
            _ => fail::<_, _, _>,
        }
        .parse_next(input)
    }

    fn parse_align<'s>(input: &mut &'s str) -> Result<&'s str> {
        delimited("Align(", digit1, " bytes)").parse_next(input)
    }
}
