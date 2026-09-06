//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/prandom.h
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
// include/linux/prandom.h
//
// Include file for the fast pseudo-random 32-bit
// generation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnd_state {
    pub s4: __u32 s1, s2, s3,,
}

extern "C" {
    pub fn prandom_u32_state(state: *mut rnd_state) -> u32;
}
extern "C" {
    pub fn prandom_bytes_state(state: *mut rnd_state, buf: *mut c_void, nbytes: usize);
}
extern "C" {
    pub fn prandom_seed_full_state(pcpu_state: *mut rnd_state __percpu);
}

//
// Handle minimum values for seeds
//
// prandom_seed_state - set seed for prandom_u32_state().
// @state: pointer to state structure to receive the seed.
// @seed: arbitrary 64-bit value to use as a seed.
//
