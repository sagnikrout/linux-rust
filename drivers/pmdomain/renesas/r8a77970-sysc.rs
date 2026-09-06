//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/renesas/r8a77970-sysc.c
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
// Renesas R-Car V3M System Controller
//
// Copyright (C) 2017 Cogent Embedded Inc.
//

    static const struct rcar_sysc_area r8a77970_areas[] __initconst = {
    { "always-on",	    0, 0, R8A77970_PD_ALWAYS_ON, -1, PD_ALWAYS_ON },
    { "ca53-scu",	0x140, 0, R8A77970_PD_CA53_SCU,	R8A77970_PD_ALWAYS_ON,
    PD_SCU },
    { "ca53-cpu0",	0x200, 0, R8A77970_PD_CA53_CPU0, R8A77970_PD_CA53_SCU,
    PD_CPU_NOCR },
    { "ca53-cpu1",	0x200, 1, R8A77970_PD_CA53_CPU1, R8A77970_PD_CA53_SCU,
    PD_CPU_NOCR },
    { "a3ir",	0x180, 0, R8A77970_PD_A3IR,	R8A77970_PD_ALWAYS_ON },
    { "a2ir0",	0x400, 0, R8A77970_PD_A2IR0,	R8A77970_PD_A3IR },
    { "a2ir1",	0x400, 1, R8A77970_PD_A2IR1,	R8A77970_PD_A3IR },
    { "a2dp",	0x400, 2, R8A77970_PD_A2DP,	R8A77970_PD_A3IR },
    { "a2cn",	0x400, 3, R8A77970_PD_A2CN,	R8A77970_PD_A3IR },
    { "a2sc0",	0x400, 4, R8A77970_PD_A2SC0,	R8A77970_PD_A3IR },
    { "a2sc1",	0x400, 5, R8A77970_PD_A2SC1,	R8A77970_PD_A3IR },
    };
    const struct rcar_sysc_info r8a77970_sysc_info __initconst = {
    .areas = r8a77970_areas,
    .num_areas = ARRAY_SIZE(r8a77970_areas),
    .extmask_offs = 0x1b0,
    .extmask_val = BIT(0),
    };
