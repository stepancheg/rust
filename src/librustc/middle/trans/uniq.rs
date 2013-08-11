// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use lib::llvm::ValueRef;
use middle::trans::base::*;
use middle::trans::build::*;
use middle::trans::common::*;
use middle::trans::datum::immediate_rvalue;
use middle::trans::glue;
use middle::ty;
use middle::lang_items::*;

pub fn make_free_glue(bcx: @mut Block, vptrptr: ValueRef, box_ty: ty::t)
    -> @mut Block {
    let _icx = push_ctxt("uniq::make_free_glue");
    let box_datum = immediate_rvalue(Load(bcx, vptrptr), box_ty);

    /*
    match ty::get(box_ty).sty {
        ty::ty_uniq(v) => {
            println(fmt!("inside uniq %?", ty::get(v.ty).sty));
        },
        _ => fail!()
    };
    */

    let contains_vec = match ty::get(box_ty).sty {
        ty::ty_uniq(v) => ty::type_is_vec(v.ty),
        _ => bcx.tcx().sess.bug("unknown type")
    };

    let not_null = IsNotNull(bcx, box_datum.val);
    do with_cond(bcx, not_null) |bcx| {
        let body_datum = box_datum.box_body(bcx);
        let bcx = glue::drop_ty(bcx, body_datum.to_ref_llval(bcx),
                                body_datum.ty);
        let contains_managed = ty::type_contents(bcx.tcx(), box_ty).contains_managed();
        //println(fmt!("types detected as %? %?", contains_managed, contains_vec));
        let free_func = match (contains_managed, contains_vec) {
            (true,  _)     => FreeFnLangItem,
            (false, false) => ExchangeFreeFnLangItem,
            (false, true)  => VecExchangeFreeFnLangItem,
        };
        glue::trans_free(bcx, box_datum.val, free_func)
    }
}
