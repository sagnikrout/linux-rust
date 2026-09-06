//! Automatically rewritten from C Header to Rust Module
//! Source: net/mctp/test/utils.h
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

pub const MCTP_DEV_TEST_MTU: c_int = 68;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mctp_test_dev {
    pub ndev: *mut net_device,
    pub mdev: *mut mctp_dev,
    pub lladdr_len: c_ushort,
    pub lladdr: [c_uchar; MAX_ADDR_LEN],
    pub pkts: sk_buff_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mctp_test_route {
    pub rt: mctp_route,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mctp_test_bind_setup {
    pub bind_addr: mctp_eid_t,
    pub bind_net: c_int,
    pub bind_type: u8,
    pub have_peer: bool,
    pub peer_addr: mctp_eid_t,
    pub peer_net: c_int,
// optional name. Used for comparison in "lookup" tests
    pub name: *const c_char,
}

extern "C" {
    pub fn mctp_test_destroy_dev(dev: *mut mctp_test_dev);
}
extern "C" {
    pub fn mctp_test_route_destroy(test: *mut kunit, rt: *mut mctp_test_route);
}
extern "C" {
    pub fn mctp_test_skb_set_dev(skb: *mut sk_buff, dev: *mut mctp_test_dev);
}

