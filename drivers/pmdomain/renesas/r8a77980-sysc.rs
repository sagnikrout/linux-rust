//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/renesas/r8a77980-sysc.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Renesas R-Car V3H System Controller
//
// Copyright (C) 2018 Renesas Electronics Corp.
// Copyright (C) 2018 Cogent Embedded, Inc.
//

    static const struct rcar_sysc_area r8a77980_areas[] __initconst = {
    { "always-on",	    0, 0, R8A77980_PD_ALWAYS_ON, -1, PD_ALWAYS_ON },
    { "ca53-scu",	0x140, 0, R8A77980_PD_CA53_SCU,	R8A77980_PD_ALWAYS_ON,
    PD_SCU },
    { "ca53-cpu0",	0x200, 0, R8A77980_PD_CA53_CPU0, R8A77980_PD_CA53_SCU,
    PD_CPU_NOCR },
    { "ca53-cpu1",	0x200, 1, R8A77980_PD_CA53_CPU1, R8A77980_PD_CA53_SCU,
    PD_CPU_NOCR },
    { "ca53-cpu2",	0x200, 2, R8A77980_PD_CA53_CPU2, R8A77980_PD_CA53_SCU,
    PD_CPU_NOCR },
    { "ca53-cpu3",	0x200, 3, R8A77980_PD_CA53_CPU3, R8A77980_PD_CA53_SCU,
    PD_CPU_NOCR },
    { "cr7",	0x240, 0, R8A77980_PD_CR7,	R8A77980_PD_ALWAYS_ON,
    PD_CPU_NOCR },
    { "a3ir",	0x180, 0, R8A77980_PD_A3IR,	R8A77980_PD_ALWAYS_ON },
    { "a2ir0",	0x400, 0, R8A77980_PD_A2IR0,	R8A77980_PD_A3IR },
    { "a2ir1",	0x400, 1, R8A77980_PD_A2IR1,	R8A77980_PD_A3IR },
    { "a2ir2",	0x400, 2, R8A77980_PD_A2IR2,	R8A77980_PD_A3IR },
    { "a2ir3",	0x400, 3, R8A77980_PD_A2IR3,	R8A77980_PD_A3IR },
    { "a2ir4",	0x400, 4, R8A77980_PD_A2IR4,	R8A77980_PD_A3IR },
    { "a2ir5",	0x400, 5, R8A77980_PD_A2IR5,	R8A77980_PD_A3IR },
    { "a2sc0",	0x400, 6, R8A77980_PD_A2SC0,	R8A77980_PD_A3IR },
    { "a2sc1",	0x400, 7, R8A77980_PD_A2SC1,	R8A77980_PD_A3IR },
    { "a2sc2",	0x400, 8, R8A77980_PD_A2SC2,	R8A77980_PD_A3IR },
    { "a2sc3",	0x400, 9, R8A77980_PD_A2SC3,	R8A77980_PD_A3IR },
    { "a2sc4",	0x400, 10, R8A77980_PD_A2SC4,	R8A77980_PD_A3IR },
    { "a2dp0",	0x400, 11, R8A77980_PD_A2DP0,	R8A77980_PD_A3IR },
    { "a2dp1",	0x400, 12, R8A77980_PD_A2DP1,	R8A77980_PD_A3IR },
    { "a2cn",	0x400, 13, R8A77980_PD_A2CN,	R8A77980_PD_A3IR },
    { "a3vip0",	0x2c0, 0, R8A77980_PD_A3VIP0,	R8A77980_PD_ALWAYS_ON },
    { "a3vip1",	0x300, 0, R8A77980_PD_A3VIP1,	R8A77980_PD_ALWAYS_ON },
    { "a3vip2",	0x280, 0, R8A77980_PD_A3VIP2,	R8A77980_PD_ALWAYS_ON },
    };
    const struct rcar_sysc_info r8a77980_sysc_info __initconst = {
    .areas = r8a77980_areas,
    .num_areas = ARRAY_SIZE(r8a77980_areas),
    .extmask_offs = 0x138,
    .extmask_val = BIT(0),
    };
