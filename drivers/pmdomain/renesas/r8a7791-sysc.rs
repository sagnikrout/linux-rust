//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/renesas/r8a7791-sysc.c
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
// Renesas R-Car M2-W/N System Controller
//
// Copyright (C) 2016 Glider bvba
//

    static const struct rcar_sysc_area r8a7791_areas[] __initconst = {
    { "always-on",	    0, 0, R8A7791_PD_ALWAYS_ON,	-1, PD_ALWAYS_ON },
    { "ca15-scu",	0x180, 0, R8A7791_PD_CA15_SCU,	R8A7791_PD_ALWAYS_ON,
    PD_SCU },
    { "ca15-cpu0",	 0x40, 0, R8A7791_PD_CA15_CPU0,	R8A7791_PD_CA15_SCU,
    PD_CPU_NOCR },
    { "ca15-cpu1",	 0x40, 1, R8A7791_PD_CA15_CPU1,	R8A7791_PD_CA15_SCU,
    PD_CPU_NOCR },
    { "sh-4a",	 0x80, 0, R8A7791_PD_SH_4A,	R8A7791_PD_ALWAYS_ON },
    { "sgx",	 0xc0, 0, R8A7791_PD_SGX,	R8A7791_PD_ALWAYS_ON },
    };
    const struct rcar_sysc_info r8a7791_sysc_info __initconst = {
    .areas = r8a7791_areas,
    .num_areas = ARRAY_SIZE(r8a7791_areas),
    };
