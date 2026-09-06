//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/vortex.c
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
// No special init required for Vortex processors.
//
    static const struct cpu_dev vortex_cpu_dev = {
    .c_vendor	= "Vortex",
    .c_ident	= { "Vortex86 SoC" },
    .legacy_models	= {
    {
    .family = 5,
    .model_names = {
    [2] = "Vortex86DX",
    [8] = "Vortex86MX",
    },
    },
    {
    .family = 6,
    .model_names = {
//
// Both the Vortex86EX and the Vortex86EX2
// have the same family and model id.
//
// However, the -EX2 supports the product name
// CPUID call, so this name will only be used
// for the -EX, which does not.
//
    [0] = "Vortex86EX",
    },
    },
    },
    .c_x86_vendor	= X86_VENDOR_VORTEX,
    };
    cpu_dev_register(vortex_cpu_dev);
