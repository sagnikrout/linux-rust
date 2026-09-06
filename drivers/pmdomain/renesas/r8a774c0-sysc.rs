//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/renesas/r8a774c0-sysc.c
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
// Renesas RZ/G2E System Controller
// Copyright (C) 2018 Renesas Electronics Corp.
//
// Based on Renesas R-Car E3 System Controller
//

    static struct rcar_sysc_area r8a774c0_areas[] __initdata = {
    { "always-on",	    0, 0, R8A774C0_PD_ALWAYS_ON, -1, PD_ALWAYS_ON },
    { "ca53-scu",	0x140, 0, R8A774C0_PD_CA53_SCU,  R8A774C0_PD_ALWAYS_ON,
    PD_SCU },
    { "ca53-cpu0",	0x200, 0, R8A774C0_PD_CA53_CPU0, R8A774C0_PD_CA53_SCU,
    PD_CPU_NOCR },
    { "ca53-cpu1",	0x200, 1, R8A774C0_PD_CA53_CPU1, R8A774C0_PD_CA53_SCU,
    PD_CPU_NOCR },
    { "a3vc",	0x380, 0, R8A774C0_PD_A3VC,	R8A774C0_PD_ALWAYS_ON },
    { "a2vc1",	0x3c0, 1, R8A774C0_PD_A2VC1,	R8A774C0_PD_A3VC },
    { "3dg-a",	0x100, 0, R8A774C0_PD_3DG_A,	R8A774C0_PD_ALWAYS_ON },
    { "3dg-b",	0x100, 1, R8A774C0_PD_3DG_B,	R8A774C0_PD_3DG_A },
    };
// Fixups for RZ/G2E ES1.0 revision
    static const struct soc_device_attribute r8a774c0[] __initconst = {
    { .soc_id = "r8a774c0", .revision = "ES1.0" },
    { /* sentinel */ }
    };
#[no_mangle]
unsafe extern "C" fn r8a774c0_sysc_init() -> int __init {
    static int __init r8a774c0_sysc_init(void)
    {
    if (soc_device_match(r8a774c0)) {
// Fix incorrect 3DG hierarchy
    swap(r8a774c0_areas[6], r8a774c0_areas[7]);
    r8a774c0_areas[6].parent = R8A774C0_PD_ALWAYS_ON;
    r8a774c0_areas[7].parent = R8A774C0_PD_3DG_B;
    }
    return 0;
    }
    const struct rcar_sysc_info r8a774c0_sysc_info __initconst = {
    .init = r8a774c0_sysc_init,
    .areas = r8a774c0_areas,
    .num_areas = ARRAY_SIZE(r8a774c0_areas),
    .extmask_offs = 0x2f8,
    .extmask_val = BIT(0),
    };
