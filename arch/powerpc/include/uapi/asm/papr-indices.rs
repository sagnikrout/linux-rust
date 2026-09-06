//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/uapi/asm/papr-indices.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

pub const LOC_CODE_SIZE: c_int = 80;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct papr_indices_io_block {
    pub /: *mut *mut __u8 is_sensor; / 0 for indicator and 1 for sensor,
    pub indice_type: __u32,
    pub indices: },
    pub /: *mut *mut __u32 token; / Sensor or indicator token,
    pub /: *mut *mut __u32 state; / get / set state,
//
// PAPR+ 12.3.2.4 Converged Location Code Rules - Length
// Restrictions. 79 characters plus null.
//
    pub /: *mut *mut char location_code_str[LOC_CODE_SIZE]; / location code,
    pub dynamic_param: },
}

//
// ioctls for /dev/papr-indices.
// PAPR_INDICES_IOC_GET: Returns a get-indices handle fd to read data
// PAPR_DYNAMIC_SENSOR_IOC_GET: Gets the state of the input sensor
// PAPR_DYNAMIC_INDICATOR_IOC_SET: Sets the new state for the input indicator
//

