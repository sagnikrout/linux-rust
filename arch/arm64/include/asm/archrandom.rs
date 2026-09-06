//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/archrandom.h
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

pub const ARM_SMCCC_TRNG_MIN_VERSION: c_uint = 0x10000UL;
//
// Reads of RNDR set PSTATE.NZCV to 0b0000 on success,
// and set PSTATE.NZCV to 0b0100 otherwise.
//
// Reads of RNDRRS set PSTATE.NZCV to 0b0000 on success,
// and set PSTATE.NZCV to 0b0100 otherwise.
//
extern "C" {
    pub fn this_cpu_has_cap(_arg: ARM64_HAS_RNG) -> return;
}
extern "C" {
    pub fn alternative_has_cap_unlikely(_arg: ARM64_HAS_RNG) -> return;
}
//
// Only support the generic interface after we have detected
// the system wide capability, avoiding complexity with the
// cpufeature code and with potential scheduling between CPUs
// with and without the feature.
//
// We prefer the SMCCC call, since its semantics (return actual
// hardware backed entropy) is closer to the idea behind this
// function here than what even the RNDRSS register provides
// (the output of a pseudo RNG freshly seeded by a TRNG).
//
// v++ = res.a1;
// v++ = res.a2;
// v++ = res.a3;
//
// RNDRRS is not backed by an entropy source but by a DRBG that is
// reseeded after each invocation. This is not a 100% fit but good
// enough to implement this API if no other entropy source exists.
//
// Open code as we run prior to the first call to cpufeature.
