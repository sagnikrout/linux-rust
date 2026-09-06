//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/pi/archrandom_early.c
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


// SPDX-License-Identifier: GPL-2.0-only

//
// To avoid rewriting code include asm/archrandom.h and create macros
// for the functions that won't be included.
//

// Macro flag: #define pr_err_once(...)

#[no_mangle]
pub unsafe extern "C" fn get_kaslr_seed_zkr(dtb_pa: uintptr_t) -> u64 {
    u64 get_kaslr_seed_zkr(const uintptr_t dtb_pa)
    {
    let mut seed: c_ulong = 0;
    if (!fdt_early_match_extension_isa((const void *)dtb_pa, "zkr"))
    return 0;
    if (!csr_seed_long(&seed))
    return 0;
    return seed;
    }
