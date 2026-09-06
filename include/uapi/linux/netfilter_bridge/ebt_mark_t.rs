//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter_bridge/ebt_mark_t.h
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
// The target member is reused for adding new actions, the
// value of the real target is -1 to -NUM_STANDARD_TARGETS.
// For backward compatibility, the 4 lsb (2 would be enough,
// but let's play it safe) are kept to designate this target.
// The remaining bits designate the action. By making the set
// action 0xfffffff0, the result will look ok for older
// versions. [September 2006]

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_mark_t_info {
    pub mark: c_ulong,
// EBT_ACCEPT, EBT_DROP, EBT_CONTINUE or EBT_RETURN
    pub target: c_int,
}

