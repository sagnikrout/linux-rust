//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/renesas/r8a779a0-sysc.c
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
// Renesas R-Car V3U System Controller
//
// Copyright (C) 2020 Renesas Electronics Corp.
//

    static struct rcar_gen4_sysc_area r8a779a0_areas[] __initdata = {
    { "always-on",	R8A779A0_PD_ALWAYS_ON, -1, PD_ALWAYS_ON },
    { "a3e0",	R8A779A0_PD_A3E0, R8A779A0_PD_ALWAYS_ON, PD_SCU },
    { "a3e1",	R8A779A0_PD_A3E1, R8A779A0_PD_ALWAYS_ON, PD_SCU },
    { "a2e0d0",	R8A779A0_PD_A2E0D0, R8A779A0_PD_A3E0, PD_SCU },
    { "a2e0d1",	R8A779A0_PD_A2E0D1, R8A779A0_PD_A3E0, PD_SCU },
    { "a2e1d0",	R8A779A0_PD_A2E1D0, R8A779A0_PD_A3E1, PD_SCU },
    { "a2e1d1",	R8A779A0_PD_A2E1D1, R8A779A0_PD_A3E1, PD_SCU },
    { "a1e0d0c0",	R8A779A0_PD_A1E0D0C0, R8A779A0_PD_A2E0D0, PD_CPU_NOCR },
    { "a1e0d0c1",	R8A779A0_PD_A1E0D0C1, R8A779A0_PD_A2E0D0, PD_CPU_NOCR },
    { "a1e0d1c0",	R8A779A0_PD_A1E0D1C0, R8A779A0_PD_A2E0D1, PD_CPU_NOCR },
    { "a1e0d1c1",	R8A779A0_PD_A1E0D1C1, R8A779A0_PD_A2E0D1, PD_CPU_NOCR },
    { "a1e1d0c0",	R8A779A0_PD_A1E1D0C0, R8A779A0_PD_A2E1D0, PD_CPU_NOCR },
    { "a1e1d0c1",	R8A779A0_PD_A1E1D0C1, R8A779A0_PD_A2E1D0, PD_CPU_NOCR },
    { "a1e1d1c0",	R8A779A0_PD_A1E1D1C0, R8A779A0_PD_A2E1D1, PD_CPU_NOCR },
    { "a1e1d1c1",	R8A779A0_PD_A1E1D1C1, R8A779A0_PD_A2E1D1, PD_CPU_NOCR },
    { "3dg-a",	R8A779A0_PD_3DG_A, R8A779A0_PD_ALWAYS_ON },
    { "3dg-b",	R8A779A0_PD_3DG_B, R8A779A0_PD_3DG_A },
    { "a3vip0",	R8A779A0_PD_A3VIP0, R8A779A0_PD_ALWAYS_ON },
    { "a3vip1",	R8A779A0_PD_A3VIP1, R8A779A0_PD_ALWAYS_ON },
    { "a3vip3",	R8A779A0_PD_A3VIP3, R8A779A0_PD_ALWAYS_ON },
    { "a3vip2",	R8A779A0_PD_A3VIP2, R8A779A0_PD_ALWAYS_ON },
    { "a3isp01",	R8A779A0_PD_A3ISP01, R8A779A0_PD_ALWAYS_ON },
    { "a3isp23",	R8A779A0_PD_A3ISP23, R8A779A0_PD_ALWAYS_ON },
    { "a3ir",	R8A779A0_PD_A3IR, R8A779A0_PD_ALWAYS_ON },
    { "a2cn0",	R8A779A0_PD_A2CN0, R8A779A0_PD_A3IR },
    { "a2imp01",	R8A779A0_PD_A2IMP01, R8A779A0_PD_A3IR },
    { "a2dp0",	R8A779A0_PD_A2DP0, R8A779A0_PD_A3IR },
    { "a2cv0",	R8A779A0_PD_A2CV0, R8A779A0_PD_A3IR },
    { "a2cv1",	R8A779A0_PD_A2CV1, R8A779A0_PD_A3IR },
    { "a2cv4",	R8A779A0_PD_A2CV4, R8A779A0_PD_A3IR },
    { "a2cv6",	R8A779A0_PD_A2CV6, R8A779A0_PD_A3IR },
    { "a2cn2",	R8A779A0_PD_A2CN2, R8A779A0_PD_A3IR },
    { "a2imp23",	R8A779A0_PD_A2IMP23, R8A779A0_PD_A3IR },
    { "a2dp1",	R8A779A0_PD_A2DP1, R8A779A0_PD_A3IR },
    { "a2cv2",	R8A779A0_PD_A2CV2, R8A779A0_PD_A3IR },
    { "a2cv3",	R8A779A0_PD_A2CV3, R8A779A0_PD_A3IR },
    { "a2cv5",	R8A779A0_PD_A2CV5, R8A779A0_PD_A3IR },
    { "a2cv7",	R8A779A0_PD_A2CV7, R8A779A0_PD_A3IR },
    { "a2cn1",	R8A779A0_PD_A2CN1, R8A779A0_PD_A3IR },
    { "a1cnn0",	R8A779A0_PD_A1CNN0, R8A779A0_PD_A2CN0 },
    { "a1cnn2",	R8A779A0_PD_A1CNN2, R8A779A0_PD_A2CN2 },
    { "a1dsp0",	R8A779A0_PD_A1DSP0, R8A779A0_PD_A2CN2 },
    { "a1cnn1",	R8A779A0_PD_A1CNN1, R8A779A0_PD_A2CN1 },
    { "a1dsp1",	R8A779A0_PD_A1DSP1, R8A779A0_PD_A2CN1 },
    };
    const struct rcar_gen4_sysc_info r8a779a0_sysc_info __initconst = {
    .areas = r8a779a0_areas,
    .num_areas = ARRAY_SIZE(r8a779a0_areas),
    };
