//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/asm-generic/signal-defs.h
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
// SA_FLAGS values:
//
// SA_NOCLDSTOP flag to turn off SIGCHLD when children stop.
// SA_NOCLDWAIT flag on SIGCHLD to inhibit zombies.
// SA_SIGINFO delivers the signal with SIGINFO structs.
// SA_ONSTACK indicates that a registered stack_t will be used.
// SA_RESTART flag to get restarting signals (which were the default long ago)
// SA_NODEFER prevents the current signal from being masked in the handler.
// SA_RESETHAND clears the handler when the signal is delivered.
// SA_UNSUPPORTED is a flag bit that will never be supported. Kernels from
// before the introduction of SA_UNSUPPORTED did not clear unknown bits from
// sa_flags when read using the oldact argument to sigaction and rt_sigaction,
// so this bit allows flag bit support to be detected from userspace while
// allowing an old kernel to be distinguished from a kernel that supports every
// flag bit.
// SA_EXPOSE_TAGBITS exposes an architecture-defined set of tag bits in
// siginfo.si_addr.
//
// SA_ONESHOT and SA_NOMASK are the historical Linux names for the Single
// Unix names RESETHAND and NODEFER respectively.
//

pub const SA_NOCLDSTOP: c_uint = 0x00000001;

pub const SA_NOCLDWAIT: c_uint = 0x00000002;

pub const SA_SIGINFO: c_uint = 0x00000004;

// 0x00000008 used on alpha, mips, parisc
// 0x00000010 used on alpha, parisc
// 0x00000020 used on alpha, parisc, sparc
// 0x00000040 used on alpha, parisc
// 0x00000080 used on parisc
// 0x00000100 used on sparc
// 0x00000200 used on sparc
pub const SA_UNSUPPORTED: c_uint = 0x00000400;
pub const SA_EXPOSE_TAGBITS: c_uint = 0x00000800;
// 0x00010000 used on mips
// 0x00800000 used for internal SA_IMMUTABLE
// 0x01000000 used on x86
// 0x02000000 used on x86
//
// New architectures should not define the obsolete
// SA_RESTORER	0x04000000
//

pub const SA_ONSTACK: c_uint = 0x08000000;

pub const SA_RESTART: c_uint = 0x10000000;

pub const SA_NODEFER: c_uint = 0x40000000;

pub const SA_RESETHAND: c_uint = 0x80000000;

extern "C" {
    pub fn __signalfn_t(_arg: c_int) -> typedef void;
}
extern "C" {
    pub fn __restorefn_t() -> typedef void;
}

