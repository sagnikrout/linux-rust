//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/pwm.h
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


// SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note

//
// struct pwmchip_waveform - Describe a PWM waveform for a pwm_chip's PWM channel
// @hwpwm: per-chip relative index of the PWM device
// @__pad: padding, must be zero
// @period_length_ns: duration of the repeating period.
// A value of 0 represents a disabled PWM.
// @duty_length_ns: duration of the active part in each period
// @duty_offset_ns: offset of the rising edge from a period's start
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwmchip_waveform {
    pub hwpwm: __u32,
    pub __pad: __u32,
    pub period_length_ns: __u64,
    pub duty_length_ns: __u64,
    pub duty_offset_ns: __u64,
}

// Reserves the passed hwpwm for exclusive control.

// counter part to PWM_IOCTL_REQUEST

//
// Modifies the passed wf according to hardware constraints. All parameters are
// rounded down to the next possible value, unless there is no such value, then
// values are rounded up. Note that zero isn't considered for rounding down
// period_length_ns.
//

// Get the currently implemented waveform

// Like PWM_IOCTL_ROUNDWF + PWM_IOCTL_SETEXACTWF in one go.

//
// Program the PWM to emit exactly the passed waveform, subject only to rounding
// down each value less than 1 ns. Returns 0 on success, -EDOM if the waveform
// cannot be implemented exactly, or other negative error codes.
//

