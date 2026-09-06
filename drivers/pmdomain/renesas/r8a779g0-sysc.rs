//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/renesas/r8a779g0-sysc.c
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
// Renesas R-Car V4H System Controller
//
// Copyright (C) 2022 Renesas Electronics Corp.
//

    static struct rcar_gen4_sysc_area r8a779g0_areas[] __initdata = {
    { "always-on",	R8A779G0_PD_ALWAYS_ON, -1, PD_ALWAYS_ON },
    { "a3e0",	R8A779G0_PD_A3E0, R8A779G0_PD_ALWAYS_ON, PD_SCU },
    { "a2e0d0",	R8A779G0_PD_A2E0D0, R8A779G0_PD_A3E0, PD_SCU },
    { "a2e0d1",	R8A779G0_PD_A2E0D1, R8A779G0_PD_A3E0, PD_SCU },
    { "a1e0d0c0",	R8A779G0_PD_A1E0D0C0, R8A779G0_PD_A2E0D0, PD_CPU_NOCR },
    { "a1e0d0c1",	R8A779G0_PD_A1E0D0C1, R8A779G0_PD_A2E0D0, PD_CPU_NOCR },
    { "a1e0d1c0",	R8A779G0_PD_A1E0D1C0, R8A779G0_PD_A2E0D1, PD_CPU_NOCR },
    { "a1e0d1c1",	R8A779G0_PD_A1E0D1C1, R8A779G0_PD_A2E0D1, PD_CPU_NOCR },
    { "a33dga",	R8A779G0_PD_A33DGA, R8A779G0_PD_ALWAYS_ON },
    { "a23dgb",	R8A779G0_PD_A23DGB, R8A779G0_PD_A33DGA },
    { "a3vip0",	R8A779G0_PD_A3VIP0, R8A779G0_PD_ALWAYS_ON },
    { "a3vip1",	R8A779G0_PD_A3VIP1, R8A779G0_PD_ALWAYS_ON },
    { "a3vip2",	R8A779G0_PD_A3VIP2, R8A779G0_PD_ALWAYS_ON },
    { "a3dul",	R8A779G0_PD_A3DUL, R8A779G0_PD_ALWAYS_ON },
    { "a3isp0",	R8A779G0_PD_A3ISP0, R8A779G0_PD_ALWAYS_ON },
    { "a3isp1",	R8A779G0_PD_A3ISP1, R8A779G0_PD_ALWAYS_ON },
    { "a3ir",	R8A779G0_PD_A3IR, R8A779G0_PD_ALWAYS_ON },
    { "a2cn0",	R8A779G0_PD_A2CN0, R8A779G0_PD_A3IR },
    { "a1cnn0",	R8A779G0_PD_A1CNN0, R8A779G0_PD_A2CN0 },
    { "a1dsp0",	R8A779G0_PD_A1DSP0, R8A779G0_PD_A2CN0 },
    { "a1dsp1",	R8A779G0_PD_A1DSP1, R8A779G0_PD_A2CN0 },
    { "a1dsp2",	R8A779G0_PD_A1DSP2, R8A779G0_PD_A2CN0 },
    { "a1dsp3",	R8A779G0_PD_A1DSP3, R8A779G0_PD_A2CN0 },
    { "a2imp01",	R8A779G0_PD_A2IMP01, R8A779G0_PD_A3IR },
    { "a2imp23",	R8A779G0_PD_A2IMP23, R8A779G0_PD_A3IR },
    { "a2psc",	R8A779G0_PD_A2PSC, R8A779G0_PD_A3IR },
    { "a2dma",	R8A779G0_PD_A2DMA, R8A779G0_PD_A3IR },
    { "a2cv0",	R8A779G0_PD_A2CV0, R8A779G0_PD_A3IR },
    { "a2cv1",	R8A779G0_PD_A2CV1, R8A779G0_PD_A3IR },
    { "a2cv2",	R8A779G0_PD_A2CV2, R8A779G0_PD_A3IR },
    { "a2cv3",	R8A779G0_PD_A2CV3, R8A779G0_PD_A3IR },
    };
    const struct rcar_gen4_sysc_info r8a779g0_sysc_info __initconst = {
    .areas = r8a779g0_areas,
    .num_areas = ARRAY_SIZE(r8a779g0_areas),
    };
