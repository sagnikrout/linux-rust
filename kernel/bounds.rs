//! Automatically rewritten from C to Rust
//! Source: kernel/bounds.c
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
// Generate definitions needed by the preprocessor.
// This code generates raw asm output which is post-processed
// to extract and format the required data.
//
// Macro flag: #define __GENERATING_BOUNDS_H
// Macro flag: #define COMPILE_OFFSETS
// Include headers that define the enum constants of interest

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
// The enum constants to put into include/generated/bounds.h
    DEFINE(NR_PAGEFLAGS, __NR_PAGEFLAGS);
    DEFINE(MAX_NR_ZONES, __MAX_NR_ZONES);

    DEFINE(NR_CPUS_BITS, order_base_2(CONFIG_NR_CPUS));

    DEFINE(SPINLOCK_SIZE, sizeof(spinlock_t));

    DEFINE(LRU_GEN_WIDTH, order_base_2(MAX_NR_GENS + 1));
    DEFINE(__LRU_REFS_WIDTH, MAX_NR_TIERS - 2);

    DEFINE(LRU_GEN_WIDTH, 0);
    DEFINE(__LRU_REFS_WIDTH, 0);

// End of constants
    return 0;
    }
