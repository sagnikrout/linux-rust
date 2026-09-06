//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/xen/trace_types.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xen_mc_flush_reason {
    XEN_MC_FL_NONE,		/* explicit flush */
    XEN_MC_FL_BATCH,	/* out of hypercall space */
    XEN_MC_FL_ARGS,		/* out of argument space */
    XEN_MC_FL_CALLBACK,	/* out of callback space */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xen_mc_extend_args {
    XEN_MC_XE_OK,
    XEN_MC_XE_BAD_OP,
    XEN_MC_XE_NO_SPACE
}

extern "C" {
    pub fn void(: *mut *mut xen_mc_callback_fn_t)(void) -> typedef;
}
