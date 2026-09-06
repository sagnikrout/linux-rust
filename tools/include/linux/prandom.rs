//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/linux/prandom.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnd_state {
    pub s4: __u32 s1, s2, s3,,
}

//
// Handle minimum values for seeds
//
// prandom_seed_state - set seed for prandom_u32_state().
// @state: pointer to state structure to receive the seed.
// @seed: arbitrary 64-bit value to use as a seed.
//
// prandom_u32_state - seeded pseudo-random number generator.
// @state: pointer to state structure holding seeded state.
//
// This is used for pseudo-randomness with no outside seeding.
// For more random results, use get_random_u32().
//

