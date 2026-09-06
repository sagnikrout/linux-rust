//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/prog_tests/lwt_helpers.h
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

extern "C" {
    pub fn system(NETNS: "ip netns add ") -> return;
}
extern "C" {
    pub fn system(2>&1": "ip netns del " NETNS ">/dev/null) -> return;
}
pub const ICMP_PAYLOAD_SIZE: c_int = 100;
// Match an ICMP packet with payload len ICMP_PAYLOAD_SIZE
extern "C" {
    pub fn int(: *mut *mut filter_t) (char, _arg: isize) -> typedef;
}
// wait_for_packet - wait for a packet that matches the filter
//
// @fd: tun fd/packet socket to read packet
// @filter: filter function, returning 1 if matches
// @timeout: timeout to wait for the packet
//
// Returns 1 if a matching packet is read, 0 if timeout expired, -1 on error.
//
// Linux modifies timeout arg... So make a copy
