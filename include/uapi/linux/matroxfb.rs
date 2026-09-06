//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/matroxfb.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct matroxioc_output_mode {
    pub /: *mut *mut __u32 output; / which output,
pub const MATROXFB_OUTPUT_PRIMARY: c_uint = 0x0000;
pub const MATROXFB_OUTPUT_SECONDARY: c_uint = 0x0001;
pub const MATROXFB_OUTPUT_DFP: c_uint = 0x0002;
    pub /: *mut *mut __u32 mode; / which mode,
pub const MATROXFB_OUTPUT_MODE_PAL: c_uint = 0x0001;
pub const MATROXFB_OUTPUT_MODE_NTSC: c_uint = 0x0002;
pub const MATROXFB_OUTPUT_MODE_MONITOR: c_uint = 0x0080;
}

// bitfield

// connect these outputs to this framebuffer

// which outputs are connected to this framebuffer

// which outputs are available for this framebuffer

// which outputs exist on this framebuffer

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum matroxfb_ctrl_id {
    MATROXFB_CID_TESTOUT	 = V4L2_CID_PRIVATE_BASE,
    MATROXFB_CID_DEFLICKER,
    MATROXFB_CID_LAST
}
