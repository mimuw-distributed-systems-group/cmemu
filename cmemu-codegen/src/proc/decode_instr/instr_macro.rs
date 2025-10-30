//! Handling of `instr!(...)` and `instr_bind!(...)` for `#[decode_instr]`

use super::{DecodeContext, InstructionLength, ensure_no_attributes};
use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;
use std::marker::PhantomData;

//-----------------------------------------------------
// High level parsing of arguments of `instr!(...)` and `instr_bind!(...)`
//-----------------------------------------------------

// Clippy finding caused by the `custom_keyword!` macro, which is external
// and we have no control over it.
#[allow(clippy::expl_impl_clone_on_copy)]
mod kw {
    syn::custom_keyword!(order);
    syn::custom_keyword!(identifiers);
}

pub(super) struct Body<M> {
    pat: syn::Lit,
    order_idents: syn::Expr,
    marker_type: PhantomData<M>,
}

pub(super) struct InOrderMarker;
pub(super) struct IdentifiersMarker;

impl syn::parse::Parse for Body<InOrderMarker> {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let pat: syn::ExprLit = input.parse()?;
        ensure_no_attributes(&pat.attrs)?;
        input.parse::<syn::Token![in]>()?;
        input.parse::<kw::order>()?;
        let order_idents: syn::Expr = input.parse()?;
        Ok(Body {
            pat: pat.lit,
            order_idents,
            marker_type: PhantomData,
        })
    }
}

// It's useful to have identifier spans - the errors are more accurate.
// Alternatively, we could set span of these identifiers to whole bit pattern string.
impl syn::parse::Parse for Body<IdentifiersMarker> {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let pat: syn::ExprLit = input.parse()?;
        ensure_no_attributes(&pat.attrs)?;
        input.parse::<kw::identifiers>()?;
        let order_idents: syn::Expr = input.parse()?;
        Ok(Body {
            pat: pat.lit,
            order_idents,
            marker_type: PhantomData,
        })
    }
}

//-----------------------------------------------------
// Bit patterns
//-----------------------------------------------------

pub(super) struct Pattern<'a> {
    ctx: DecodeContext<'a>,
    bits: BitsPack,
    subpatterns: Vec<Subpattern>,
    span: Span,
}

struct BitsPack {
    fixed: u32,
    fixed_mask: u32,
    unpredictable: u32,
    unpredictable_mask: u32,
}

struct Subpattern {
    bits_width: u8,
    bits_shift: u8,
    pat_name: syn::Ident,
}

struct ParsedPattern {
    bits: BitsPack,
    bits_cnt: u8,
    subpats_raw: Vec<(u8, u8, String)>,
    not_flag: bool,
}

#[derive(PartialEq, Clone, Copy)]
enum BitPatternType {
    MainPattern { allow_unpredictable_bits: bool },
    MatchArmSubpattern,
}

impl<'a> Pattern<'a> {
    pub(super) fn new<M>(
        ctx: &DecodeContext<'a>,
        body: &Body<M>,
        allow_unpredictable_bits: bool,
        span: Span,
    ) -> syn::Result<Self> {
        let mut parsed_pattern = Self::parse_bit_pattern(
            &body.pat,
            BitPatternType::MainPattern {
                allow_unpredictable_bits,
            },
        )?;

        // check if we've got proper number of bits
        if parsed_pattern.bits_cnt != ctx.instr_len.as_u8() {
            return Err(syn::Error::new_spanned(
                &body.pat,
                format!(
                    "invalid bit pattern: expected {} bits, got {} bits instead",
                    ctx.instr_len.as_u8(),
                    parsed_pattern.bits_cnt,
                ),
            ));
        }

        // check if order_idents is a tuple of ids
        let ordered_idents =
            if let syn::Expr::Tuple(syn::ExprTuple { attrs, elems, .. }) = &body.order_idents {
                ensure_no_attributes(attrs)?;
                elems
                    .iter()
                    .map(|e| {
                        if let syn::Expr::Path(syn::ExprPath {
                            attrs,
                            qself: None,
                            path,
                        }) = e
                        {
                            ensure_no_attributes(attrs)?;
                            path.get_ident().map_or_else(
                                || {
                                    Err(syn::Error::new_spanned(
                                    e,
                                    "invalid in order/identifiers: expected a simple identifier",
                                ))
                                },
                                |ident| Ok(ident.clone()),
                            )
                        } else {
                            Err(syn::Error::new_spanned(
                                e,
                                "invalid in order/identifiers: expected a simple identifier",
                            ))
                        }
                    })
                    .collect::<syn::Result<Vec<_>>>()?
            } else {
                return Err(syn::Error::new_spanned(
                    &body.order_idents,
                    "invalid in order/identifiers: expected a tuple of identifiers",
                ));
            };

        // check ids uniqueness
        for idx in 0..ordered_idents.len() {
            if ordered_idents[idx + 1..]
                .iter()
                .any(|ident| *ident == ordered_idents[idx])
            {
                return Err(syn::Error::new_spanned(
                    &ordered_idents[idx],
                    "invalid in order/identifiers: identifiers must be unique",
                ));
            }
        }

        // construct subpatterns
        let subpatterns = ordered_idents
            .into_iter()
            .map(|ident| {
                let ident_str = format!("{ident}");
                parsed_pattern.subpats_raw
                    .iter()
                    .position(|(_, _, pat_name)| *pat_name == ident_str)
                    .map_or_else(
                        || {
                            Err(syn::Error::new_spanned(
                                &ident,
                                "invalid in order/identifiers: identifier not found in the bit pattern",
                            ))
                        },
                        |subpat_idx| {
                            let (bits_width, bits_shift, _) = parsed_pattern.subpats_raw.remove(subpat_idx);
                            Ok(Subpattern {
                                bits_width,
                                bits_shift,
                                pat_name: ident.clone(),
                            })
                        },
                    )
            })
            .collect::<syn::Result<Vec<_>>>()?;

        // check if all ids covered
        if !parsed_pattern.subpats_raw.is_empty() {
            return Err(syn::Error::new_spanned(
                &body.order_idents,
                "invalid in order/identifiers: all identifiers present in bit pattern must occur here",
            ));
        }

        // return the value
        Ok(Pattern {
            ctx: ctx.clone(),
            bits: parsed_pattern.bits,
            subpatterns,
            span,
        })
    }

    pub(super) fn match_arm_to_condition(
        &self,
        arm: &syn::Arm,
        expect_wildcard: bool,
    ) -> syn::Result<syn::Expr> {
        // validate match arm
        ensure_no_attributes(&arm.attrs)?;
        if let Some((if_, _)) = &arm.guard {
            return Err(syn::Error::new_spanned(if_, "guard not allowed"));
        }

        // validate the pattern: check if wildcard if expected
        if expect_wildcard {
            return if let syn::Pat::Wild(syn::PatWild { attrs, .. }) = &arm.pat {
                ensure_no_attributes(attrs)?;
                Ok(syn::parse_quote! { () }) // random value to discard
            } else {
                Err(syn::Error::new_spanned(
                    &arm.pat,
                    "expected a wildcard `_` as last branch of match",
                ))
            };
        }

        // convert patterns into conditions
        let conds = match &arm.pat {
            syn::Pat::Or(syn::PatOr { attrs, cases, .. }) => {
                ensure_no_attributes(attrs)?;
                cases
                    .iter()
                    .map(|pat| self.match_arm_tuple_pattern_to_condition(pat))
                    .collect::<Result<Vec<_>, _>>()?
            }
            // assume it's tuple pattern - it's validated later
            _ => vec![self.match_arm_tuple_pattern_to_condition(&arm.pat)?],
        };

        // return the expression
        Ok(syn::parse_quote! { #(#conds)||* })
    }

    fn match_arm_tuple_pattern_to_condition(&self, pat: &syn::Pat) -> syn::Result<TokenStream> {
        // validate the pattern: tuple of string literals
        fn validate_pat(p: &syn::Pat) -> syn::Result<&syn::Lit> {
            if let syn::Pat::Lit(syn::PatLit { attrs, lit }) = &p {
                ensure_no_attributes(attrs)?;
                Ok(lit)
            } else {
                Err(syn::Error::new_spanned(p, "expected string literal"))
            }
        }
        let bit_patterns = if let syn::Pat::Tuple(syn::PatTuple { attrs, elems, .. }) = pat {
            ensure_no_attributes(attrs)?;
            elems
                .iter()
                .map(validate_pat)
                .collect::<syn::Result<Vec<_>>>()?
        } else if let syn::Pat::Paren(syn::PatParen { attrs, pat, .. }) = pat {
            ensure_no_attributes(attrs)?;
            vec![validate_pat(pat)?]
        } else {
            return Err(syn::Error::new_spanned(
                pat,
                "expected tuple of string literals",
            ));
        };

        // parse the bit patterns
        let bit_patterns = bit_patterns
            .into_iter()
            .map(|e| {
                Self::parse_bit_pattern(e, BitPatternType::MatchArmSubpattern)
                    .map(|parsed| (parsed, e))
            })
            .collect::<syn::Result<Vec<_>>>()?;

        // validate bit patterns number
        if bit_patterns.len() != self.subpatterns.len() {
            return Err(syn::Error::new_spanned(
                pat,
                format!(
                    "invalid number of bit patterns, expected {}, got {}",
                    self.subpatterns.len(),
                    bit_patterns.len()
                ),
            ));
        }

        // construct expected bits
        let mut fixed_bits = self.bits.fixed;
        let mut fixed_bits_mask = self.bits.fixed_mask;
        let mut fixed_not_bits = vec![];
        let mut fixed_not_bits_mask = vec![];
        for ((pat, expr), subpat) in bit_patterns.iter().zip(self.subpatterns.iter()) {
            // validate number of bits
            if pat.bits_cnt != subpat.bits_width {
                return Err(syn::Error::new_spanned(
                    expr,
                    format!(
                        "invalid number of bits in the pattern, expected {}, got {}",
                        subpat.bits_width, pat.bits_cnt
                    ),
                ));
            }

            // accumulate more known bits
            if pat.not_flag {
                fixed_not_bits.push(pat.bits.fixed << subpat.bits_shift);
                fixed_not_bits_mask.push(pat.bits.fixed_mask << subpat.bits_shift);
            } else {
                fixed_bits |= pat.bits.fixed << subpat.bits_shift;
                fixed_bits_mask |= pat.bits.fixed_mask << subpat.bits_shift;
            }
        }

        // return the condition
        let (instr_bits, instr_span) = self.get_instr_bits_as_u32_tokens_and_span();
        Ok(quote_spanned! { instr_span =>
            #instr_bits & #fixed_bits_mask == #fixed_bits
            #( && #instr_bits & #fixed_not_bits_mask != #fixed_not_bits )*
        })
    }

    #[allow(clippy::too_many_lines)] // it's hard to shorten this function and it doesn't seem good to split it even more
    fn parse_bit_pattern(expr: &syn::Lit, pat_type: BitPatternType) -> syn::Result<ParsedPattern> {
        const DELIMITER: char = '|';

        // helper closure
        let err_msg = |msg| Err(syn::Error::new_spanned(expr, msg));

        // check if bit pattern is string literal
        let pat_str = if let syn::Lit::Str(lit_str) = expr {
            lit_str.value()
        } else {
            return err_msg("expected string literal");
        };

        // parse the pattern
        let mut subpats_raw = vec![];
        let mut bits = BitsPack {
            fixed: 0_u32,
            fixed_mask: 0_u32,
            unpredictable: 0_u32,
            unpredictable_mask: 0_u32,
        };
        let mut bits_cnt = 0;
        let mut pat_it = pat_str.chars().rev();
        let mut not_flag = false;
        let mut last_char_delim = true;
        #[cfg(debug_assertions)]
        let mut delim_found = false;

        while let Some(ch) = pat_it.next() {
            if bits_cnt == 32 {
                return err_msg("bit pattern can't be longer than 32 bits");
            }

            match ch {
                '0' | '1' => {
                    bits.fixed |= (ch as u32 - '0' as u32) << bits_cnt;
                    bits.fixed_mask |= 1 << bits_cnt;
                    bits_cnt += 1;
                }
                'x' => {
                    bits_cnt += 1;
                }
                '>' if matches!(pat_type, BitPatternType::MainPattern { .. }) => {
                    let subpat_len = Self::take_while_mut(&mut pat_it, ':', expr)?
                        .parse::<u8>()
                        .map_err(|err| {
                            syn::Error::new_spanned(
                                expr,
                                format!(
                                    "invalid bit pattern: inside <id:num>: failed to parse num: {err}"
                                ),
                            )
                        })?;
                    if subpat_len == 0 {
                        return err_msg(
                            "invalid bit pattern: inside <id:num>: num must be positive",
                        );
                    }
                    let subpat_ident = Self::take_while_mut(&mut pat_it, '<', expr)?;

                    subpats_raw.push((subpat_len, bits_cnt, subpat_ident));
                    bits_cnt += subpat_len;
                }
                ')' if pat_type
                    == BitPatternType::MainPattern {
                        allow_unpredictable_bits: true,
                    } =>
                {
                    let ch = pat_it.next();
                    let opening = pat_it.next();
                    match (ch, opening) {
                        (Some(ch), Some('(')) if matches!(ch, '0' | '1') => {
                            bits.unpredictable |= (ch as u32 - '0' as u32) << bits_cnt;
                            bits.unpredictable_mask |= 1 << bits_cnt;
                            bits_cnt += 1;
                        }
                        _ => {
                            return err_msg(
                                "invalid bit pattern: use (0) or (1) for \"unpredictable\" bits check",
                            );
                        }
                    }
                }
                ' ' if pat_type == BitPatternType::MatchArmSubpattern => {
                    if pat_it.eq("not".chars().rev()) {
                        not_flag = true;
                        break;
                    }
                    return err_msg("invalid bit pattern: expected `(not )?[01x]+'");
                }
                DELIMITER if matches!(pat_type, BitPatternType::MainPattern { .. }) => {
                    if last_char_delim {
                        return err_msg(&format!(
                            "invalid bit pattern: delimiter `{DELIMITER}' not allowed at start, end and after other delimiter."
                        ));
                    }
                    #[cfg(debug_assertions)]
                    {
                        delim_found = true;
                    }
                }
                _ => {
                    let msg = match pat_type {
                        // it seems there's a bug in rustfmt keeping it from formatting this code,
                        // so please keep it nicely formatted manually
                        BitPatternType::MainPattern {
                            allow_unpredictable_bits: true,
                        } => {
                            "invalid bit pattern: only 0, 1, (0), (1), x, | and <id:num> allowed at global level"
                        }
                        BitPatternType::MainPattern {
                            allow_unpredictable_bits: false,
                        } => {
                            "invalid bit pattern: only 0, 1, x, | and <id:num> allowed at global level"
                        }
                        BitPatternType::MatchArmSubpattern => {
                            "invalid bit pattern: expected `(not )?[01x]+'"
                        }
                    };
                    return err_msg(msg);
                }
            }

            last_char_delim = ch == DELIMITER;
        }

        if last_char_delim {
            return err_msg(&format!(
                "invalid bit pattern: delimiter `{DELIMITER}` not allowed at start, end and after another delimiter."
            ));
        }

        #[cfg(debug_assertions)]
        {
            if matches!(pat_type, BitPatternType::MainPattern { .. }) && !delim_found {
                return err_msg(&format!(
                    "invalid bit pattern: assert: are you sure you haven't missed any delimiters `{DELIMITER}'?"
                ));
            }
        }

        Ok(ParsedPattern {
            bits,
            bits_cnt,
            subpats_raw,
            not_flag,
        })
    }

    // helper method for Self::parse_bit_pattern
    fn take_while_mut(
        pat_it: &mut impl Iterator<Item = char>,
        until: char,
        expr: &syn::Lit,
    ) -> syn::Result<String> {
        let mut acc = vec![];
        let mut finished = false;
        // let's assume clippy::while_let_on_iterator knows what it suggests...
        for ch in pat_it {
            if ch == until {
                finished = true;
                break;
            }
            acc.push(ch);
        }

        if finished {
            Ok(acc.into_iter().rev().collect::<String>())
        } else {
            Err(syn::Error::new_spanned(
                expr,
                "invalid bit pattern: not closed <id:num> group",
            ))
        }
    }

    pub(super) fn generate_unpredictable_bits_match_condition(&self) -> syn::Expr {
        let unpredictable_bits = self.bits.unpredictable;
        let unpredictable_bits_mask = self.bits.unpredictable_mask;
        let (instr_bits, instr_span) = self.get_instr_bits_as_u32_tokens_and_span();
        let cond_spanned = quote_spanned! { instr_span => {
            #[allow(clippy::verbose_bit_mask)]
            let cond = #instr_bits & #unpredictable_bits_mask == #unpredictable_bits;
            cond
        }};
        syn::parse_quote_spanned! {self.span=> #cond_spanned }
    }

    pub(super) fn generate_bindings(&self) -> syn::Stmt {
        let names = self.subpatterns.iter().map(|sp| &sp.pat_name);
        let vals = self.subpatterns.iter().map(|sp| {
            let (instr_bits, instr_span) = self.get_instr_bits_as_u32_tokens_and_span();
            let shift = sp.bits_shift;
            let mask = u32::try_from((1_u64 << sp.bits_width) - 1).unwrap();
            // quote! adds type to literals, and we don't want it
            let n: TokenStream = sp.bits_width.to_string().parse().unwrap();
            quote_spanned! { instr_span => {
                let val = (#instr_bits >> #shift) & #mask;
                <crate::Bitstring![#n]>::try_from(val).unwrap()
            }}
        });

        syn::parse_quote_spanned! {self.span=> let ( #(#names, )* ) = ( #(#vals, )* ); }
    }

    fn get_instr_bits_as_u32_tokens_and_span(&self) -> (TokenStream, Span) {
        let instr_bits = &self.ctx.instr_ident;
        let instr_span = instr_bits.span();
        let instr_bits2 = match self.ctx.instr_len {
            InstructionLength::Bits16 => {
                quote_spanned! { instr_span => u32::from(#instr_bits) }
            }
            InstructionLength::Bits32 => quote_spanned! { instr_span => #instr_bits },
        };
        (instr_bits2, instr_span)
    }

    pub(super) fn has_unpredictable_bits(&self) -> bool {
        self.bits.unpredictable_mask != 0
    }
}
