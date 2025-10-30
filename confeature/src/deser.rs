use crate::optional;
use indexmap::IndexMap;
use serde::de::{self, IgnoredAny, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::cmp::min;
use std::fmt;
use std::marker::PhantomData;
use std::ops::Range;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ConfigSpec {
    #[serde(rename = ".meta")]
    #[serde(default, skip_serializing)]
    pub _meta: IndexMap<String, IgnoredAny>,

    #[serde(rename = ".confeature")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conf: Option<ConfeatureConfig>,

    #[serde(flatten)]
    pub ns: IndexMap<String, Namespace>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub(crate) struct ConfeatureConfig {
    #[serde(flatten)]
    pub scope: ScopeConfig,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ScopeConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<Mode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode_cap: Option<Mode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct Namespace {
    #[serde(rename = ".meta")]
    #[serde(default, skip_serializing)]
    pub _meta: IndexMap<String, IgnoredAny>,

    #[serde(rename = ".confeature")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<ScopeConfig>,

    #[serde(flatten)]
    pub ns: IndexMap<String, FauxConfigurable>,
}

impl Namespace {
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Configurable)> {
        self.ns.iter().map(|(k, fc)| (k, &fc.obj))
    }
}

impl IntoIterator for Namespace {
    type Item = (String, Configurable);
    type IntoIter = XXX;

    fn into_iter(self) -> XXX {
        XXX(self.ns.into_iter())
    }
}
#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub struct XXX(<IndexMap<String, FauxConfigurable> as IntoIterator>::IntoIter);
impl Iterator for XXX {
    type Item = (String, Configurable);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(k, v)| (k, v.obj))
    }
}

// This is needed for string_or_struct to work on Configurable...
// https://stackoverflow.com/questions/54761790/how-to-deserialize-with-for-a-container-using-serde-in-rust
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
#[non_exhaustive]
pub struct FauxConfigurable {
    #[serde(deserialize_with = "string_or_struct", flatten)]
    pub obj: Configurable,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Mode {
    /// Only from the spec file, this doesn't need to add any dependencies.
    Fixed,
    /// Bound at compile time (it may be used to drive conditional compilation).
    Comptime,
    /// (Unstable) Binds at comptime, but may be patched in the binary (access is ultra-fast).
    Patchable,
    /// If configured at compile time, this should optimize to a constant. Can be configured at runtime.
    #[default]
    Mixed,
    /// Simple preference runtime > comptime > spec file.
    Anytime,
    /// Can be configured only at runtime to override the default.
    /// It deliberately ignores comptime environment.
    Runtime,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct SharedAttrs {
    #[serde(rename = ".meta")]
    #[serde(default, skip_serializing)]
    pub _meta: IndexMap<String, IgnoredAny>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub enum Configurable {
    Feature {
        #[serde(flatten)]
        attr: SharedAttrs,
    },
    Cfg {
        expr: String,

        #[serde(flatten)]
        attr: SharedAttrs,
    },
    Bool {
        #[serde(default)]
        default: bool,
        #[serde(default)]
        mode: Option<Mode>,

        #[serde(flatten)]
        attr: SharedAttrs,
    },
    Int {
        default: Option<i64>,
        #[serde(default)]
        optional: bool,

        #[serde(default)]
        range: Option<Range<i64>>,
        #[serde(default)]
        mode: Option<Mode>,

        #[serde(flatten)]
        attr: SharedAttrs,
    },
    Enum {
        default: Option<String>,
        #[serde(default)]
        optional: bool,

        variants: Vec<String>,
        #[serde(default)]
        mode: Option<Mode>,

        #[serde(flatten)]
        attr: SharedAttrs,
    },
    Str {
        default: Option<String>,
        #[serde(default)]
        optional: bool,

        #[serde(default)]
        mode: Option<Mode>,

        #[serde(flatten)]
        attr: SharedAttrs,
    },
    Namespace {
        ns: Namespace,
    },
    Struct {
        #[serde(default)]
        mode: Option<Mode>,
        #[serde(flatten)]
        attr: SharedAttrs,

        /// Instantiate this struct (path) instead of defining a new one.
        #[serde(default)]
        instance: Option<String>,

        fields: Namespace,
    },
    #[serde(alias = "bit_field")]
    Bitfield {
        fields: IndexMap<String, u8>,
        #[serde(default)]
        mode: Option<Mode>,

        #[serde(flatten)]
        attr: SharedAttrs,
    },
}

/*
Ideas for mode: struct/namespace

1. Just nested module (and join with __)
2. Support for an enforcing type with assoc constants:
```
    // User code
    trait Mob { const HP: u32; const MANA: u32}
    // Generated code
    struct Goblin;
    impl Mob for Goblin {
        const HP: u32 = ....;
        const MANA: u32 = ...;
    }
    // Usage:
    Goblin::HP
3. Const struct instance (may be singleton or instantiation of user-defined)
```
    // Either auto-generated or user code
    struct Mob{ hp: u32, mana: u32 }
    // Generated code
    const/static GOBLIN: Mob = Mob { hp: ..., mana: ... }
    // Usage:
    GOBLIN.hp // autoderef
```

1. and 3. seems natural for respectively namespace and struct
2. is limited to only comptime (implicit mode_cap: comptime)
3. nicely extends to a bitfield

 */

impl Configurable {
    #[must_use]
    pub fn get_mode(&self, default: Mode) -> Mode {
        match self {
            Configurable::Feature { .. } | Configurable::Cfg { .. } => min(Mode::Comptime, default),
            Configurable::Bool { mode, .. }
            | Configurable::Int { mode, .. }
            | Configurable::Enum { mode, .. }
            | Configurable::Str { mode, .. }
            | Configurable::Struct { mode, .. }
            | Configurable::Bitfield { mode, .. } => mode.unwrap_or(default),
            Configurable::Namespace { ns, .. } => {
                optional!(ns.scope.as_ref()?.default_mode?).unwrap_or(default)
            }
        }
    }
    #[must_use]
    pub fn get_attrs(&self) -> &SharedAttrs {
        match self {
            Configurable::Feature { attr, .. }
            | Configurable::Cfg { attr, .. }
            | Configurable::Bool { attr, .. }
            | Configurable::Int { attr, .. }
            | Configurable::Enum { attr, .. }
            | Configurable::Str { attr, .. }
            | Configurable::Struct { attr, .. }
            | Configurable::Bitfield { attr, .. } => attr,
            Configurable::Namespace { .. } => unimplemented!(),
            // Configurable::Namespace { ns, .. } => &SharedAttrs{_meta: ns._meta, doc: optional!(ns.scope.as_ref()?.doc?)},
        }
    }
    #[must_use]
    pub fn is_optional(&self) -> bool {
        match self {
            Configurable::Feature { .. }
            | Configurable::Cfg { .. }
            | Configurable::Bool { .. }
            | Configurable::Struct { .. }
            | Configurable::Bitfield { .. } => false,
            Configurable::Int { optional, .. }
            | Configurable::Enum { optional, .. }
            | Configurable::Str { optional, .. } => *optional,
            Configurable::Namespace { .. } => unimplemented!(),
        }
    }
    #[must_use]
    pub fn has_default(&self) -> bool {
        match self {
            Configurable::Feature { .. }
            | Configurable::Cfg { .. }
            | Configurable::Bool { .. }
            | Configurable::Bitfield { .. } => true,
            Configurable::Int { default, .. } => default.is_some(),
            Configurable::Enum { default, .. } => default.is_some(),
            Configurable::Str { default, .. } => default.is_some(),
            Configurable::Struct { fields, .. } => fields
                .iter()
                .all(|(_, c)| c.has_default() && !c.is_optional()),
            Configurable::Namespace { .. } => unimplemented!(),
        }
    }
}

impl FromStr for Configurable {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "bool" | "flag" | "false" => Self::Bool {
                default: false,
                mode: None,
                attr: Default::default(),
            },
            "true" => Self::Bool {
                default: true,
                mode: None,
                attr: Default::default(),
            },
            "feature" => Self::Feature {
                attr: Default::default(),
            },
            "int" => Self::Int {
                default: None,
                optional: true,
                range: None,
                mode: None,
                attr: Default::default(),
            },
            _ if i64::from_str(s).is_ok() => Self::Int {
                default: Some(i64::from_str(s).unwrap()),
                optional: false,
                range: None,
                mode: None,
                attr: Default::default(),
            },
            _ => return Err("Invalid shortcut notation"),
        })
    }
}

// Copied verbatim from https://serde.rs/string-or-struct.html
pub fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = &'static str>,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = &'static str>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_bool<E>(self, v: bool) -> Result<T, E>
        where
            E: de::Error,
        {
            // Convert back, so we have a single source of truth
            FromStr::from_str(&v.to_string()).map_err(de::Error::custom)
        }

        fn visit_i64<E>(self, v: i64) -> Result<T, E>
        where
            E: de::Error,
        {
            FromStr::from_str(&v.to_string()).map_err(de::Error::custom)
        }

        fn visit_u64<E>(self, v: u64) -> Result<T, E>
        where
            E: de::Error,
        {
            FromStr::from_str(&v.to_string()).map_err(de::Error::custom)
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            FromStr::from_str(value).map_err(de::Error::custom)
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}
