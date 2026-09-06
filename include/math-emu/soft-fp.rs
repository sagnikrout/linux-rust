//! Automatically rewritten from C Header to Rust Module
//! Source: include/math-emu/soft-fp.h
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


// Software floating-point emulation.

// Allow sfp-machine to have its own byte order definitions.

pub const _FP_WORKBITS: c_int = 3;

// By default don't care about exceptions.

pub const FP_EX_INVALID: c_int = 0;

pub const FP_EX_INVALID_SNAN: c_int = 0;

// inf - inf

pub const FP_EX_INVALID_ISI: c_int = 0;

// inf / inf

pub const FP_EX_INVALID_IDI: c_int = 0;

// 0 / 0

pub const FP_EX_INVALID_ZDZ: c_int = 0;

// inf * 0

pub const FP_EX_INVALID_IMZ: c_int = 0;

pub const FP_EX_OVERFLOW: c_int = 0;

// Macro flag: #define FP_EX_UNDERFLOW

pub const FP_EX_DIVZERO: c_int = 0;

pub const FP_EX_INEXACT: c_int = 0;

pub const FP_EX_DENORM: c_int = 0;

// By default we never flush denormal input operands to signed zero.

pub const FP_DENORM_ZERO: c_int = 0;

// By default we write the results always.
// sfp-machine may override this and e.g.
// check if some exceptions are unmasked
// and inhibit it in such a case.
//
pub const FP_INHIBIT_RESULTS: c_int = 0;

pub const FP_TRAPPING_EXCEPTIONS: c_int = 0;

pub const FP_CLS_NORMAL: c_int = 0;
pub const FP_CLS_ZERO: c_int = 1;
pub const FP_CLS_INF: c_int = 2;
pub const FP_CLS_NAN: c_int = 3;

// Sigh.  Silly things longlong.h needs.

extern "C" {
    pub fn __attribute__(_arg: (mode(SI))) -> typedef int SItype;
}
extern "C" {
    pub fn __attribute__(_arg: (mode(DI))) -> typedef int DItype;
}
extern "C" {
    pub fn __attribute__(_arg: (mode(SI))) -> typedef unsigned int USItype;
}
extern "C" {
    pub fn __attribute__(_arg: (mode(DI))) -> typedef unsigned int UDItype;
}

extern "C" {
    pub fn __attribute__(_arg: (mode(HI))) -> typedef unsigned int UHWtype;
}

pub type UHWtype = USItype;

