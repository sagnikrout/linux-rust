//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dpll/zl3073x/synth.h
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
// struct zl3073x_synth - synthesizer state
// @freq_mult: frequency multiplier
// @freq_base: frequency base
// @freq_m: frequency numerator
// @freq_n: frequency denominator
// @ctrl: synth control
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zl3073x_synth {
    pub freq_mult: u32,
    pub freq_base: u16,
    pub freq_m: u16,
    pub freq_n: u16,
    pub ctrl: u8,
}

extern "C" {
    pub fn zl3073x_synth_state_fetch(zldev: *mut zl3073x_dev, synth_id: u8) -> c_int;
}
//
// zl3073x_synth_dpll_get - get DPLL ID the synth is driven by
// @synth: pointer to synth state
//
// Return: ID of DPLL the given synthetizer is driven by
//
extern "C" {
    pub fn FIELD_GET(_arg: ZL_SYNTH_CTRL_DPLL_SEL, _arg: synth->ctrl) -> return;
}
//
// zl3073x_synth_freq_get - get synth current freq
// @synth: pointer to synth state
//
// Return: frequency of given synthetizer
//
// zl3073x_synth_is_enabled - check if the given synth is enabled
// @synth: pointer to synth state
//
// Return: true if synth is enabled, false otherwise
//
extern "C" {
    pub fn FIELD_GET(_arg: ZL_SYNTH_CTRL_EN, _arg: synth->ctrl) -> return;
}
