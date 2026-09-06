//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/inet6_hashtables.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// INET		An implementation of the TCP/IP protocol suite for the LINUX
// operating system.  INET is implemented using the BSD Socket
// interface as the means of communication with the user level.
//
// Authors:	Lotsa people, from code originally in tcp
//

extern "C" {
    pub fn inet6_init_ehash_secret();
}
extern "C" {
    pub fn jhash_3words(_arg: lhash, _arg: fhash, _arg: ports, _arg: initval) -> return;
}
//
// Sockets in TCP_CLOSE state are _always_ taken out of the hash, so
// we need not check it for TCP lookups anymore, thanks Alexey. -DaveM
//
// The sockhash lock must be held as a reader here.
//
// refcounted = true;
// refcounted = false;
// We've chosen a new reuseport sock which is never refcounted. This
// implies that sk also isn't refcounted.
//
// READ_ONCE() paired with WRITE_ONCE() in sock_bindtoindex_locked()

