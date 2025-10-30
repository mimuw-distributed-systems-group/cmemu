use proc_macro2::{Span, TokenStream};

/// Kind of `syn::Meta`.
/// The idea is to parse list in form "key = value, key = value".
/// It's useful when you are only given the "parenthesis contents" of "something(key = value)".
///
/// Note: only `syn::Path`s were needed, thus they are the value types.
///       Howerver, the value type can be obviously generic, if needed.
pub(crate) struct MetaDict {
    // header(key = value, meta = dict); used for error messages when key is not found
    header_span: Span,
    entries: Vec<MetaDictItem>,
}

// internal structs
struct MetaDictItem {
    key: syn::Ident,
    value: syn::Path,
}

struct MetaDictParser<'a> {
    header_span: Span,
    expected_keys: &'a [&'a str],
}

impl MetaDict {
    /// Creates a new parser.
    ///
    /// Arguments:
    /// * `spanned`:
    ///   Span used for "not found" error messages.
    ///   Preferred "header" part of "header(key = value)".
    /// * `expected_keys`:
    ///   Expected set of keys (intention: static set of fields of a structure).
    ///   If the parsed set differs, an error is returned.
    pub(crate) fn new_parser<'input, T>(
        spanned: &T,
        expected_keys: &'input [&str],
    ) -> impl syn::parse::Parser<Output = Self> + 'input
    where
        T: syn::spanned::Spanned,
    {
        // assumption: small number of keys, so O(n^2) algorithms could be even faster than big-O approximately faster algorithms
        let header_span = spanned.span();
        MetaDictParser {
            header_span,
            expected_keys,
        }
    }

    /// Gets an expected value. Returns error for unexpected values.
    // Note: we could use `.expect()`, however this approach is similar,
    //       but more flexible: in case we don't want to check against expected set of keys.
    pub(crate) fn get_value(&self, key: &str) -> syn::Result<&syn::Path> {
        self.entries
            .iter()
            .find_map(|e| (e.key == key).then_some(&e.value))
            .ok_or_else(|| syn::Error::new(self.header_span, format!("key '{key}' not found")))
    }
}

impl syn::parse::Parse for MetaDictItem {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let key = input.parse()?;
        input.parse::<syn::Token![=]>()?;
        let value = input.parse()?;
        Ok(Self { key, value })
    }
}

impl syn::parse::Parser for MetaDictParser<'_> {
    type Output = MetaDict;
    fn parse2(self, tokens: TokenStream) -> syn::Result<Self::Output> {
        let parser = syn::punctuated::Punctuated::<_, syn::Token![,]>::parse_terminated;
        let entries: Vec<_> = parser
            .parse2(tokens)
            .map_err(|mut e| {
                // if parser finds unexpected end of TokenStream,
                // it emits unhelpful Span::call_site(), so let's make error a little bit better
                e.combine(syn::Error::new(
                    self.header_span,
                    "previous error occurred for this call",
                ));
                e
            })?
            .into_iter()
            .collect();

        Self::validate_key_uniqueness(&entries[..])?;
        self.expect_keys(&entries[..])?;

        Ok(MetaDict {
            header_span: self.header_span,
            entries,
        })
    }
}

impl MetaDictParser<'_> {
    fn validate_key_uniqueness(entries: &[MetaDictItem]) -> syn::Result<()> {
        for (i, entry) in entries.iter().enumerate() {
            if let Some(other) = entries[i + 1..].iter().find(|e| e.key == entry.key) {
                let mut err = syn::Error::new_spanned(&entry.key, "key is not unique");
                err.combine(syn::Error::new_spanned(&other.key, "key is not unique"));
                return Err(err);
            }
        }
        Ok(())
    }

    fn expect_keys(&self, entries: &[MetaDictItem]) -> syn::parse::Result<()> {
        debug_assert!(!self.expected_keys.is_empty());
        for key in self.expected_keys {
            if entries.iter().all(|e| e.key != key) {
                return Err(syn::Error::new(
                    self.header_span,
                    format!("expected key '{key}' - not found"),
                ));
            }
        }
        for entry in entries {
            if self.expected_keys.iter().all(|k| entry.key != k) {
                return Err(syn::Error::new_spanned(&entry.key, "found unexpected key"));
            }
        }
        Ok(())
    }
}
