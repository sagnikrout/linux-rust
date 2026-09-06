//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dpll/zl3073x/out.h
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
// struct zl3073x_out - output state
// @div: output divisor
// @width: output pulse width
// @esync_n_period: embedded sync or n-pin period (for n-div formats)
// @esync_n_width: embedded sync or n-pin pulse width
// @phase_comp: phase compensation
// @mode: output mode
// @ctrl: output control
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zl3073x_out {
    pub div: u32,
    pub width: u32,
    pub esync_n_period: u32,
    pub esync_n_width: u32,
    pub phase_comp: i32,
    pub mode: u8,
    pub ctrl: u8,
}

extern "C" {
    pub fn zl3073x_out_state_fetch(zldev: *mut zl3073x_dev, index: u8) -> c_int;
}
//
// zl3073x_out_clock_type_get - get output clock type
// @out: pointer to out state
//
// Return: clock type of given output (ZL_OUTPUT_MODE_CLOCK_TYPE_*)
//
extern "C" {
    pub fn FIELD_GET(_arg: ZL_OUTPUT_MODE_CLOCK_TYPE, _arg: out->mode) -> return;
}
//
// zl3073x_out_clock_type_set - set output clock type
// @out: pointer to out state
// @type: clock type (ZL_OUTPUT_MODE_CLOCK_TYPE_*)
//
// zl3073x_out_signal_format_get - get output signal format
// @out: pointer to out state
//
// Return: signal format of given output
//
extern "C" {
    pub fn FIELD_GET(_arg: ZL_OUTPUT_MODE_SIGNAL_FORMAT, _arg: out->mode) -> return;
}
//
// zl3073x_out_is_diff - check if the given output is differential
// @out: pointer to out state
//
// Return: true if output is differential, false if output is single-ended
//
// zl3073x_out_is_enabled - check if the given output is enabled
// @out: pointer to out state
//
// Return: true if output is enabled, false if output is disabled
//
// zl3073x_out_is_ndiv - check if the given output is in N-div mode
// @out: pointer to out state
//
// Return: true if output is in N-div mode, false otherwise
//
// zl3073x_out_synth_get - get synth connected to given output
// @out: pointer to out state
//
// Return: index of synth connected to given output.
//
extern "C" {
    pub fn FIELD_GET(_arg: ZL_OUTPUT_CTRL_SYNTH_SEL, _arg: out->ctrl) -> return;
}
