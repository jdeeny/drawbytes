// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// force-host

#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]
#![feature(slice_patterns)]

extern crate syntax;
extern crate syntax_pos;
extern crate rustc;
extern crate rustc_plugin;

use syntax::parse::token;
use syntax::tokenstream::TokenTree;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;  // trait for expr_usize
use syntax_pos::Span;
use rustc_plugin::Registry;

// WARNING WARNING WARNING WARNING WARNING
// =======================================
//
// This code also appears in src/doc/guide-plugin.md. Please keep
// the two copies in sync!  FIXME: have rustdoc read this file

fn expand_draw_u8(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
        -> Box<MacResult + 'static> {

    static NUMERALS: &'static [(&'static str, usize)] = &[
        ("_", 0), ("0", 0),
        ("X", 1),
        ];

    if args.len() != 1 {
        cx.span_err(
            sp,
            &format!("argument should be a single identifier, but got {} arguments", args.len()));
        return DummyResult::any(sp);
    }

    let text = match args[0] {
        TokenTree::Token(_, token::Ident(s, false)) => {
            let s = s.to_string();
            if s.len() > 8 {
                cx.span_err(sp, "argument should be 8 characters or less");
                return DummyResult::any(sp);
            }
            s
        },
        _ => {
            cx.span_err(sp, "argument should be a single identifier");
            return DummyResult::any(sp);
        }
    };

    let mut text = &*text;
    let mut total = 0;
    let mut shift = 8;
    while !text.is_empty() {
        match NUMERALS.iter().find(|&&(rn, _)| text.starts_with(rn)) {
            Some(&(rn, val)) => {
                shift -= 1;
                total += val << shift;
                text = &text[rn.len()..];
            }
            None => {
                cx.span_err(sp, "invalid drawbytes");
                return DummyResult::any(sp);
            }
        }
    }

    MacEager::expr(cx.expr_u8(sp, total as u8))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("draw_u8", expand_draw_u8);
}
