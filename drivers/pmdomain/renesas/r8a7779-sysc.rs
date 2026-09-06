//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/renesas/r8a7779-sysc.c
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
// Renesas R-Car H1 System Controller
//
// Copyright (C) 2016 Glider bvba
//

    static const struct rcar_sysc_area r8a7779_areas[] __initconst = {
    { "always-on",	    0, 0, R8A7779_PD_ALWAYS_ON,	-1, PD_ALWAYS_ON },
    { "arm1",	 0x40, 1, R8A7779_PD_ARM1,	R8A7779_PD_ALWAYS_ON,
    PD_CPU_CR },
    { "arm2",	 0x40, 2, R8A7779_PD_ARM2,	R8A7779_PD_ALWAYS_ON,
    PD_CPU_CR },
    { "arm3",	 0x40, 3, R8A7779_PD_ARM3,	R8A7779_PD_ALWAYS_ON,
    PD_CPU_CR },
    { "sgx",	 0xc0, 0, R8A7779_PD_SGX,	R8A7779_PD_ALWAYS_ON },
    { "vdp",	0x100, 0, R8A7779_PD_VDP,	R8A7779_PD_ALWAYS_ON },
    { "imp",	0x140, 0, R8A7779_PD_IMP,	R8A7779_PD_ALWAYS_ON },
    };
    const struct rcar_sysc_info r8a7779_sysc_info __initconst = {
    .areas = r8a7779_areas,
    .num_areas = ARRAY_SIZE(r8a7779_areas),
    };
