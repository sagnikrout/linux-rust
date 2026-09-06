//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/eventfd.h
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
//
// include/linux/eventfd.h
//
// Copyright (C) 2007  Davide Libenzi <davidel@xmailserver.org>
//

//
// CAREFUL: Check include/uapi/asm-generic/fcntl.h when defining
// new flags, since they might collide with O_* ones. We want
// to re-use O_* flags that couldn't possibly have a meaning
// from eventfd, in order to leave a free define-space for
// shared O_* flags.
//

extern "C" {
    pub fn eventfd_ctx_put(ctx: *mut eventfd_ctx);
}
extern "C" {
    pub fn eventfd_signal_mask(ctx: *mut eventfd_ctx, mask: __poll_t);
}
extern "C" {
    pub fn eventfd_ctx_do_read(ctx: *mut eventfd_ctx, cnt: *mut __u64);
}

//
// Ugly ugly ugly error layer to support modules that uses eventfd but
// pretend to work in !CONFIG_EVENTFD configurations. Namely, AIO.
//
extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}

