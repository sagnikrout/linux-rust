//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/iw_portmap.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2014 Intel Corporation. All rights reserved.
// Copyright (c) 2014 Chelsio, Inc. All rights reserved.
//

pub const IWPM_ULIBNAME_SIZE: c_int = 32;
pub const IWPM_DEVNAME_SIZE: c_int = 32;
pub const IWPM_IFNAME_SIZE: c_int = 16;
pub const IWPM_IPADDR_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwpm_dev_data {
    pub dev_name: [c_char; IWPM_DEVNAME_SIZE],
    pub if_name: [c_char; IWPM_IFNAME_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwpm_sa_data {
    pub loc_addr: sockaddr_storage,
    pub mapped_loc_addr: sockaddr_storage,
    pub rem_addr: sockaddr_storage,
    pub mapped_rem_addr: sockaddr_storage,
    pub flags: u32,
}

extern "C" {
    pub fn iwpm_init(_arg: u8) -> c_int;
}
extern "C" {
    pub fn iwpm_exit(_arg: u8) -> c_int;
}
extern "C" {
    pub fn iwpm_valid_pid() -> c_int;
}
extern "C" {
    pub fn iwpm_register_pid(pm_msg: *mut iwpm_dev_data, nl_client: u8) -> c_int;
}
extern "C" {
    pub fn iwpm_add_mapping(pm_msg: *mut iwpm_sa_data, nl_client: u8) -> c_int;
}
extern "C" {
    pub fn iwpm_add_and_query_mapping(pm_msg: *mut iwpm_sa_data, nl_client: u8) -> c_int;
}
extern "C" {
    pub fn iwpm_remove_mapping(local_addr: *mut sockaddr_storage, nl_client: u8) -> c_int;
}
extern "C" {
    pub fn iwpm_register_pid_cb(: *mut sk_buff, : *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn iwpm_add_mapping_cb(: *mut sk_buff, : *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn iwpm_add_and_query_mapping_cb(: *mut sk_buff, : *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn iwpm_remote_info_cb(: *mut sk_buff, : *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn iwpm_mapping_error_cb(: *mut sk_buff, : *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn iwpm_mapping_info_cb(: *mut sk_buff, : *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn iwpm_ack_mapping_info_cb(: *mut sk_buff, : *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn iwpm_hello_cb(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
