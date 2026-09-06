//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/renesas/r8a779f0-sysc.c
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
// Renesas R-Car S4-8 System Controller
//
// Copyright (C) 2021 Renesas Electronics Corp.
//

    static struct rcar_gen4_sysc_area r8a779f0_areas[] __initdata = {
    { "always-on",	R8A779F0_PD_ALWAYS_ON, -1, PD_ALWAYS_ON },
    { "a3e0",	R8A779F0_PD_A3E0, R8A779F0_PD_ALWAYS_ON, PD_SCU },
    { "a3e1",	R8A779F0_PD_A3E1, R8A779F0_PD_ALWAYS_ON, PD_SCU },
    { "a2e0d0",	R8A779F0_PD_A2E0D0, R8A779F0_PD_A3E0, PD_SCU },
    { "a2e0d1",	R8A779F0_PD_A2E0D1, R8A779F0_PD_A3E0, PD_SCU },
    { "a2e1d0",	R8A779F0_PD_A2E1D0, R8A779F0_PD_A3E1, PD_SCU },
    { "a2e1d1",	R8A779F0_PD_A2E1D1, R8A779F0_PD_A3E1, PD_SCU },
    { "a1e0d0c0",	R8A779F0_PD_A1E0D0C0, R8A779F0_PD_A2E0D0, PD_CPU_NOCR },
    { "a1e0d0c1",	R8A779F0_PD_A1E0D0C1, R8A779F0_PD_A2E0D0, PD_CPU_NOCR },
    { "a1e0d1c0",	R8A779F0_PD_A1E0D1C0, R8A779F0_PD_A2E0D1, PD_CPU_NOCR },
    { "a1e0d1c1",	R8A779F0_PD_A1E0D1C1, R8A779F0_PD_A2E0D1, PD_CPU_NOCR },
    { "a1e1d0c0",	R8A779F0_PD_A1E1D0C0, R8A779F0_PD_A2E1D0, PD_CPU_NOCR },
    { "a1e1d0c1",	R8A779F0_PD_A1E1D0C1, R8A779F0_PD_A2E1D0, PD_CPU_NOCR },
    { "a1e1d1c0",	R8A779F0_PD_A1E1D1C0, R8A779F0_PD_A2E1D1, PD_CPU_NOCR },
    { "a1e1d1c1",	R8A779F0_PD_A1E1D1C1, R8A779F0_PD_A2E1D1, PD_CPU_NOCR },
    };
    const struct rcar_gen4_sysc_info r8a779f0_sysc_info __initconst = {
    .areas = r8a779f0_areas,
    .num_areas = ARRAY_SIZE(r8a779f0_areas),
    };
