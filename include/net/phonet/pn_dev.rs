//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/phonet/pn_dev.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// File: pn_dev.h
//
// Phonet network device
//
// Copyright (C) 2008 Nokia Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phonet_device_list {
    pub list: list_head,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phonet_device {
    pub list: list_head,
    pub netdev: *mut net_device,
    pub 64): DECLARE_BITMAP(addrs,,
    pub rcu: rcu_head,
}

extern "C" {
    pub fn phonet_device_init() -> c_int;
}
extern "C" {
    pub fn phonet_device_exit();
}
extern "C" {
    pub fn phonet_netlink_register() -> c_int;
}
extern "C" {
    pub fn phonet_address_add(dev: *mut net_device, addr: u8) -> c_int;
}
extern "C" {
    pub fn phonet_address_del(dev: *mut net_device, addr: u8) -> c_int;
}
extern "C" {
    pub fn phonet_address_get(dev: *mut net_device, addr: u8) -> u8;
}
extern "C" {
    pub fn phonet_address_lookup(net: *mut net, addr: u8) -> c_int;
}
extern "C" {
    pub fn phonet_address_notify(net: *mut net, event: c_int, ifindex: u32, addr: u8);
}
extern "C" {
    pub fn phonet_route_add(dev: *mut net_device, daddr: u8) -> c_int;
}
extern "C" {
    pub fn phonet_route_del(dev: *mut net_device, daddr: u8) -> c_int;
}
extern "C" {
    pub fn rtm_phonet_notify(net: *mut net, event: c_int, ifindex: u32, dst: u8);
}
pub const PN_NO_ADDR: c_uint = 0xff;
