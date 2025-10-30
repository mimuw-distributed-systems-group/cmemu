//! Disassembler generator.

use crate::utils::MetaDict;
use instr_macro::{Body, IdentifiersMarker, InOrderMarker, Pattern};
use proc_macro2::TokenStream;
use quote::quote;
use std::mem::swap;
use syn::spanned::Spanned;

mod instr_macro;

/// Idea:
/// look at src/component/core/decode/instruction.rs
//--------------
// todo: optimization: (ALWAYS MEASURE to learn if given approach is really faster and how much faster)
//  - todo: use hashmap instead of series of ifs? (stupid approach takes too much memory;
//          the idea is to create a jump table instead of series of if-else; it should only check specified bits to save memory
//          and thus be nested like in the docs [ARM-ARM] - and like currently series of if-else are nested)
//  - todo: implement decode cache? (most likely u16/u32 + ctx (itstate?) -> Instruction)
// todo: consider: allow to ignore some bindings like: ("10x01", _, "1111")
// todo: consider: enforce every `match instr!` arm be either `match instr!` or `(if) instr_bind!`; currently it can be neither of them
// todo: consider: should instr_bind! on u16 instruction create u16 bindings?
// todo: consider: forbid "_ =>" and assume some default (from i.e. #[decode_instr(default = ...)])
//                 (if there's a need, maybe make the "_ =>" optional temporarily?)
//                 Note: once we know we covered all instructions, this default should be undefined instruction
pub fn decode_instr(attr: TokenStream, item: syn::ItemFn) -> TokenStream {
    use syn::parse::Parser;

    let args = match MetaDict::new_parser(&attr, &["unpredictable"])
        .parse2(attr)
        .and_then(DecodeInstrArgs::try_from)
    {
        Ok(args) => args,
        Err(e) => return e.to_compile_error(),
    };
    decode_instr_expand(item, &args).unwrap_or_else(|e| e.to_compile_error())
}

#[derive(Clone)]
struct DecodeContext<'a> {
    /// Identifier of binding containing raw instruction bits
    instr_ident: &'a syn::Ident,
    instr_len: InstructionLength,
    args: &'a DecodeInstrArgs,
}

/// Number of instruction bits: 16 or 32
#[derive(Clone, Copy)]
enum InstructionLength {
    Bits16,
    Bits32,
}

struct DecodeInstrArgs {
    unpredictable: syn::Path,
}

impl InstructionLength {
    fn from_path(path: &syn::Path) -> Option<Self> {
        if path.is_ident("u16") {
            Some(InstructionLength::Bits16)
        } else if path.is_ident("u32") {
            Some(InstructionLength::Bits32)
        } else {
            None
        }
    }

    fn as_u8(self) -> u8 {
        match self {
            Self::Bits16 => 16,
            Self::Bits32 => 32,
        }
    }
}

impl TryFrom<MetaDict> for DecodeInstrArgs {
    type Error = syn::Error;

    fn try_from(mdict: MetaDict) -> syn::Result<Self> {
        let unpredictable = mdict.get_value("unpredictable")?.clone();
        Ok(Self { unpredictable })
    }
}

fn decode_instr_expand(mut item: syn::ItemFn, args: &DecodeInstrArgs) -> syn::Result<TokenStream> {
    if item.block.stmts.len() != 1 {
        return Err(syn::Error::new_spanned(
            item.block,
            "There must be one only one statement in #[decode_instr] function",
        ));
    }

    let syn::Stmt::Expr(expr_match @ syn::Expr::Match(_), None) = &mut item.block.stmts[0] else {
        return Err(syn::Error::new_spanned(
            item.block,
            "The only statement in #[decode_instr] function must be a match expression.",
        ));
    };

    let Some(syn::FnArg::Typed(first_arg)) = item.sig.inputs.first() else {
        return Err(syn::Error::new_spanned(
            item.sig.inputs,
            "There must be at least one argument in #[decode_instr] function \
             and the first one must be either u16 or u32.",
        ));
    };
    ensure_no_attributes(&first_arg.attrs)?;

    let ctx = match (&*first_arg.pat, &*first_arg.ty) {
        (
            syn::Pat::Ident(syn::PatIdent {
                attrs,
                by_ref: None,
                mutability: None,
                ident,
                subpat: None,
            }),
            syn::Type::Path(syn::TypePath { qself: None, path }),
        ) if InstructionLength::from_path(path).is_some() => {
            ensure_no_attributes(attrs)?;
            DecodeContext {
                instr_ident: ident,
                instr_len: InstructionLength::from_path(path).expect("already checked it's some"),
                args,
            }
        }
        _ => {
            return Err(syn::Error::new_spanned(
                first_arg,
                "There first argument in #[decode_instr] function must be either u16 or u32.",
            ));
        }
    };

    expand_match_arm_expr(&ctx, expr_match)?;

    Ok(quote! { #item })
}

fn expand_match_arm_expr(ctx: &DecodeContext, body: &mut syn::Expr) -> syn::Result<()> {
    // if block, "unwrap" first expression
    if let syn::Expr::Block(syn::ExprBlock {
        attrs,
        label: None,
        block,
    }) = body
    {
        let replaced = match &mut block.stmts[..] {
            // match instr! requires one instruction and no semicolon
            [syn::Stmt::Expr(expr @ syn::Expr::Match(_), None)] => expand_instr(ctx, expr)?,
            // instr_bind! requires a block with only if expression
            [syn::Stmt::Expr(syn::Expr::If(expr_if), None)] => expand_if_instr_bind(ctx, expr_if)?,
            // or block with first statement being macro call
            [stmt @ syn::Stmt::Macro(_), ..] => expand_instr_bind(ctx, stmt)?,
            _ => false,
        };
        if replaced {
            ensure_no_attributes(attrs)?;
        }
    } else if let syn::Expr::If(expr_if) = body {
        // if no block, instr_bind! can be only in the condition of an if expr
        expand_if_instr_bind(ctx, expr_if)?;
    } else {
        expand_instr(ctx, body)?;
    }

    Ok(())
}

fn expand_if_instr_bind(ctx: &DecodeContext, expr_if: &mut syn::ExprIf) -> syn::Result<bool> {
    if let Some(expr_macro) = extract_macro_call(&mut expr_if.cond, "instr_bind") {
        // ensure no attributes
        ensure_no_attributes(&expr_if.attrs)?;
        ensure_no_attributes(&expr_macro.attrs)?;

        // ensure no else block
        if let Some((else_token, _)) = expr_if.else_branch {
            return Err(syn::Error::new_spanned(
                else_token,
                "invalid `if instr_bind!': else is not permitted, it's automatically generated",
            ));
        }

        // parse the patterns
        let parsed_body = expr_macro.mac.parse_body::<Body<IdentifiersMarker>>()?;
        let pattern = Pattern::new(ctx, &parsed_body, true, expr_macro.span())?;

        // expect some unpredictable bits
        if !pattern.has_unpredictable_bits() {
            return Err(syn::Error::new_spanned(
                expr_if.if_token,
                "invalid `if instr_bind!': no (0) or (1) found in the pattern, remove the unnecessary `if'",
            ));
        }

        // generate if condition & bindings
        let mut if_cond = pattern.generate_unpredictable_bits_match_condition();
        let bindings = pattern.generate_bindings();

        // patch the code
        let unpredictable = &ctx.args.unpredictable;
        let if_span = expr_if.if_token.span;

        swap(&mut *expr_if.cond, &mut if_cond);
        expr_if.then_branch.stmts.insert(0, bindings);
        expr_if.else_branch = Some((
            syn::Token![else](if_span),
            syn::parse_quote_spanned! {if_span=> { #unpredictable }},
        ));

        Ok(true)
    } else {
        Ok(false)
    }
}

fn expand_instr_bind(ctx: &DecodeContext, body: &mut syn::Stmt) -> syn::Result<bool> {
    if let Some(stmt_macro) = extract_macro_stmt(body, "instr_bind") {
        // ensure no attributes
        ensure_no_attributes(&stmt_macro.attrs)?;

        // parse the patterns
        let parsed_body = stmt_macro.mac.parse_body::<Body<IdentifiersMarker>>()?;
        let pattern = Pattern::new(ctx, &parsed_body, false, stmt_macro.span())?;

        // replace the code
        *body = pattern.generate_bindings();

        Ok(true)
    } else {
        Ok(false)
    }
}

fn expand_instr(ctx: &DecodeContext, body: &mut syn::Expr) -> syn::Result<bool> {
    if let syn::Expr::Match(expr_match) = body
        && let Some(expr_macro) = extract_macro_call(&mut expr_match.expr, "instr")
    {
        // ensure no attributes
        ensure_no_attributes(&expr_macro.attrs)?;
        ensure_no_attributes(&expr_match.attrs)?;

        // check if there are enough arms
        if expr_match.arms.is_empty() {
            return Err(syn::Error::new_spanned(
                expr_match.match_token,
                "there should be at least one arm in the match body",
            ));
        }

        // parse the patterns
        let parsed_body = expr_macro.mac.parse_body::<Body<InOrderMarker>>()?;
        let pattern = Pattern::new(ctx, &parsed_body, false, expr_macro.span())?;
        let conds = expr_match.arms[..expr_match.arms.len() - 1]
            .iter()
            .map(|arm| pattern.match_arm_to_condition(arm, false))
            .collect::<syn::Result<Vec<_>>>()?;
        let _no_value = pattern.match_arm_to_condition(
            expr_match
                .arms
                .last()
                .expect("there should be at least one arm in the match body"),
            true,
        )?;

        // generate the `if` structure
        let mut if_structure: syn::Expr =
            syn::parse_quote_spanned! {expr_match.span()=> { #( if #conds { () } else )* { () } } };
        let if_structure = block_to_last_expression(&mut if_structure);
        expand_fuse_arms_with_bodies(ctx, if_structure, &mut expr_match.arms)?;
        swap(body, if_structure);
        return Ok(true);
    }
    Ok(false)
}

// helper method for expand_*instr*
fn extract_macro_call<'a>(
    body: &'a mut syn::Expr,
    macro_name: &str,
) -> Option<&'a mut syn::ExprMacro> {
    // check if special macro call
    if let syn::Expr::Macro(em @ syn::ExprMacro { .. }) = body
        && em.mac.path.is_ident(macro_name)
    {
        Some(em)
    } else {
        None
    }
}

fn extract_macro_stmt<'a>(
    body: &'a mut syn::Stmt,
    macro_name: &str,
) -> Option<&'a mut syn::StmtMacro> {
    // check if special macro call
    if let syn::Stmt::Macro(em @ syn::StmtMacro { .. }) = body
        && em.mac.path.is_ident(macro_name)
    {
        Some(em)
    } else {
        None
    }
}

fn expand_fuse_arms_with_bodies(
    ctx: &DecodeContext,
    if_structure: &mut syn::Expr,
    match_arms: &mut [syn::Arm],
) -> syn::Result<()> {
    assert!(!match_arms.is_empty());
    let (expr, else_branch) = if match_arms.len() > 1 {
        if let syn::Expr::If(expr_if) = if_structure {
            if let syn::Stmt::Expr(expr, _) = expr_if
                .then_branch
                .stmts
                .last_mut()
                .expect("invalid if-structure")
            {
                use std::borrow::BorrowMut;
                (expr, expr_if.else_branch.as_mut().map(|e| e.1.borrow_mut()))
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    } else {
        (block_to_last_expression(if_structure), None)
    };

    swap(expr, &mut match_arms[0].body); // fill proper expr
    expand_match_arm_expr(ctx, expr)?; // expand body

    // continue if possible
    if let Some(expr) = else_branch {
        expand_fuse_arms_with_bodies(ctx, expr, &mut match_arms[1..])?;
    }

    Ok(())
}

//-----------------------------------------------------
// Helper methods
//-----------------------------------------------------

// helper method with some assumptions!
fn block_to_last_expression(expr: &mut syn::Expr) -> &mut syn::Expr {
    if let syn::Expr::Block(eb) = expr {
        if let syn::Stmt::Expr(expr, None) =
            eb.block.stmts.last_mut().expect("invalid if-structure")
        {
            expr
        } else {
            unreachable!()
        }
    } else {
        unreachable!()
    }
}

// in case someone wants i.e., allow some clippy lint, but the attribute will get lost
fn ensure_no_attributes(attrs: &[syn::Attribute]) -> syn::Result<()> {
    if attrs.is_empty() {
        Ok(())
    } else {
        Err(syn::Error::new_spanned(
            &attrs[0],
            "no attributes allowed here",
        ))
    }
}
