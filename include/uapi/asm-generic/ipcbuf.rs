//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/asm-generic/ipcbuf.h
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

//
// The generic ipc64_perm structure:
// Note extra padding because this structure is passed back and forth
// between kernel and user space.
//
// ipc64_perm was originally meant to be architecture specific, but
// everyone just ended up making identical copies without specific
// optimizations, so we may just as well all use the same one.
//
// Pad space is left for:
// - 32-bit mode_t on architectures that only had 16 bit
// - 32-bit seq
// - 2 miscellaneous 32-bit values
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm {
    pub key: __kernel_key_t,
    pub uid: __kernel_uid32_t,
    pub gid: __kernel_gid32_t,
    pub cuid: __kernel_uid32_t,
    pub cgid: __kernel_gid32_t,
    pub mode: __kernel_mode_t,
// pad if mode_t is u16:
    pub sizeof(__kernel_mode_t)]: unsigned char __pad1[4 -,
    pub seq: c_ushort,
    pub __pad2: c_ushort,
    pub __unused1: __kernel_ulong_t,
    pub __unused2: __kernel_ulong_t,
}
