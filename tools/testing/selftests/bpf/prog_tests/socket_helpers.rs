//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/prog_tests/socket_helpers.h
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

// include/linux/net.h
pub const SOCK_TYPE_MASK: c_uint = 0xf;
pub const IO_TIMEOUT_SEC: c_int = 30;
pub const MAX_STRERR_LEN: c_int = 256;
// workaround for older vm_sockets.h

pub const VMADDR_CID_LOCAL: c_int = 1;

// include/linux/compiler_types.h

// include/linux/cleanup.h

// __ptr = nullvalue;                                            \

// Wrappers that fail the test on error and report it.

// len = sizeof(*addr4);
// len = sizeof(*addr6);
// len = sizeof(sa_family_t);
// len = sizeof(*addr);
extern "C" {
    pub fn socket_loopback_reuseport(_arg: family, _arg: sotype, _arg: -1) -> return;
}
extern "C" {
    pub fn accept(_arg: fd, _arg: addr, _arg: len) -> return;
}
extern "C" {
    pub fn recv(_arg: fd, _arg: buf, _arg: len, _arg: flags) -> return;
}
// p0 = take_fd(s);
// p0 = take_fd(p);
// p1 = take_fd(c);
