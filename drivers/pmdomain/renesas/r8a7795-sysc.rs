//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/renesas/r8a7795-sysc.c
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
// Renesas R-Car H3 System Controller
//
// Copyright (C) 2016-2017 Glider bvba
//

    static struct rcar_sysc_area r8a7795_areas[] __initdata = {
    { "always-on",	    0, 0, R8A7795_PD_ALWAYS_ON,	-1, PD_ALWAYS_ON },
    { "ca57-scu",	0x1c0, 0, R8A7795_PD_CA57_SCU,	R8A7795_PD_ALWAYS_ON,
    PD_SCU },
    { "ca57-cpu0",	 0x80, 0, R8A7795_PD_CA57_CPU0,	R8A7795_PD_CA57_SCU,
    PD_CPU_NOCR },
    { "ca57-cpu1",	 0x80, 1, R8A7795_PD_CA57_CPU1,	R8A7795_PD_CA57_SCU,
    PD_CPU_NOCR },
    { "ca57-cpu2",	 0x80, 2, R8A7795_PD_CA57_CPU2,	R8A7795_PD_CA57_SCU,
    PD_CPU_NOCR },
    { "ca57-cpu3",	 0x80, 3, R8A7795_PD_CA57_CPU3,	R8A7795_PD_CA57_SCU,
    PD_CPU_NOCR },
    { "ca53-scu",	0x140, 0, R8A7795_PD_CA53_SCU,	R8A7795_PD_ALWAYS_ON,
    PD_SCU },
    { "ca53-cpu0",	0x200, 0, R8A7795_PD_CA53_CPU0,	R8A7795_PD_CA53_SCU,
    PD_CPU_NOCR },
    { "ca53-cpu1",	0x200, 1, R8A7795_PD_CA53_CPU1,	R8A7795_PD_CA53_SCU,
    PD_CPU_NOCR },
    { "ca53-cpu2",	0x200, 2, R8A7795_PD_CA53_CPU2,	R8A7795_PD_CA53_SCU,
    PD_CPU_NOCR },
    { "ca53-cpu3",	0x200, 3, R8A7795_PD_CA53_CPU3,	R8A7795_PD_CA53_SCU,
    PD_CPU_NOCR },
    { "a3vp",	0x340, 0, R8A7795_PD_A3VP,	R8A7795_PD_ALWAYS_ON },
    { "cr7",	0x240, 0, R8A7795_PD_CR7,	R8A7795_PD_ALWAYS_ON },
    { "a3vc",	0x380, 0, R8A7795_PD_A3VC,	R8A7795_PD_ALWAYS_ON },
    { "a2vc1",	0x3c0, 1, R8A7795_PD_A2VC1,	R8A7795_PD_A3VC },
    { "3dg-a",	0x100, 0, R8A7795_PD_3DG_A,	R8A7795_PD_ALWAYS_ON },
    { "3dg-b",	0x100, 1, R8A7795_PD_3DG_B,	R8A7795_PD_3DG_A },
    { "3dg-c",	0x100, 2, R8A7795_PD_3DG_C,	R8A7795_PD_3DG_B },
    { "3dg-d",	0x100, 3, R8A7795_PD_3DG_D,	R8A7795_PD_3DG_C },
    { "3dg-e",	0x100, 4, R8A7795_PD_3DG_E,	R8A7795_PD_3DG_D },
    { "a3ir",	0x180, 0, R8A7795_PD_A3IR,	R8A7795_PD_ALWAYS_ON },
    };
//
// Fixups for R-Car H3 revisions
//

    static const struct soc_device_attribute r8a7795_quirks_match[] __initconst = {
    {
    .soc_id = "r8a7795", .revision = "ES2.*",
    .data = (void *)(NO_EXTMASK),
    },
    { /* sentinel */ }
    };
#[no_mangle]
unsafe extern "C" fn r8a7795_sysc_init() -> int __init {
    static int __init r8a7795_sysc_init(void)
    {
    const struct soc_device_attribute *attr;
    let mut quirks: u32 = 0;
    attr = soc_device_match(r8a7795_quirks_match);
    if (attr)
    quirks = (uintptr_t)attr.data;
    if (quirks & NO_EXTMASK)
    r8a7795_sysc_info.extmask_val = 0;
    return 0;
    }
    struct rcar_sysc_info r8a7795_sysc_info __initdata = {
    .init = r8a7795_sysc_init,
    .areas = r8a7795_areas,
    .num_areas = ARRAY_SIZE(r8a7795_areas),
    .extmask_offs = 0x2f8,
    .extmask_val = BIT(0),
    };
