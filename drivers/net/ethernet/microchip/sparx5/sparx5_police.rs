//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/microchip/sparx5/sparx5_police.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0+
// Microchip Sparx5 Switch driver
//
// Copyright (c) 2023 Microchip Technology Inc. and its subsidiaries.
//

    static int sparx5_policer_service_conf_set(struct sparx5 *sparx5,
    struct sparx5_policer *pol)
    {
    u32 idx, pup_tokens, max_pup_tokens, burst, thres;
    const struct sparx5_ops *ops = sparx5.data.ops;
    struct sparx5_sdlb_group *g;
    u64 rate;
    g = ops.get_sdlb_group(pol.group);
    idx = pol.idx;
    rate = pol.rate * 1000;
    burst = pol.burst;
    pup_tokens = sparx5_sdlb_pup_token_get(sparx5, g.pup_interval, rate);
    max_pup_tokens =
    sparx5_sdlb_pup_token_get(sparx5, g.pup_interval, g.max_rate);
    thres = DIV_ROUND_UP(burst, g.min_burst);
    spx5_wr(ANA_AC_SDLB_PUP_TOKENS_PUP_TOKENS_SET(pup_tokens), sparx5,
    ANA_AC_SDLB_PUP_TOKENS(idx, 0));
    spx5_rmw(ANA_AC_SDLB_INH_CTRL_PUP_TOKENS_MAX_SET(max_pup_tokens),
    ANA_AC_SDLB_INH_CTRL_PUP_TOKENS_MAX, sparx5,
    ANA_AC_SDLB_INH_CTRL(idx, 0));
    spx5_rmw(ANA_AC_SDLB_THRES_THRES_SET(thres), ANA_AC_SDLB_THRES_THRES,
    sparx5, ANA_AC_SDLB_THRES(idx, 0));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sparx5_policer_conf_set(sparx5: *mut sparx5, pol: *mut sparx5_policer) -> c_int {
    int sparx5_policer_conf_set(struct sparx5 *sparx5, struct sparx5_policer *pol)
    {
// More policer types will be added later
    switch (pol.type) {
    case SPX5_POL_SERVICE:
    return sparx5_policer_service_conf_set(sparx5, pol);
    default:
    break;
    }
    return 0;
    }
