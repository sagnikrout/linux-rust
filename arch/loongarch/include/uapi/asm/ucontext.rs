//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/uapi/asm/ucontext.h
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
// struct ucontext - user context structure
// @uc_flags:
// @uc_link:
// @uc_stack:
// @uc_mcontext:	holds basic processor state
// @uc_sigmask:
// @uc_extcontext:	holds extended processor state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucontext {
    pub uc_flags: c_ulong,
    pub uc_link: *mut ucontext,
    pub uc_stack: stack_t,
    pub uc_sigmask: sigset_t,
// There's some padding here to allow sigset_t to be expanded in the
// future.  Though this is unlikely, other architectures put uc_sigmask
// at the end of this structure and explicitly state it can be
// expanded, so we didn't want to box ourselves in here.
    pub sizeof(sigset_t)]: __u8 __unused[1024 / 8 -,
// We can't put uc_sigmask at the end of this structure because we need
// to be able to expand sigcontext in the future.  For example, the
// vector ISA extension will almost certainly add ISA state.  We want
// to ensure all user-visible ISA state can be saved and restored via a
// ucontext, so we're putting this at the end in order to allow for
// infinite extensibility.  Since we know this will be extended and we
// assume sigset_t won't be extended an extreme amount, we're
// prioritizing this.
    pub uc_mcontext: sigcontext,
}
