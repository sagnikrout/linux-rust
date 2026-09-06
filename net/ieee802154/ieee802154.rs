//! Automatically rewritten from C Header to Rust Module
//! Source: net/ieee802154/ieee802154.h
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
// Copyright (C) 2007, 2008, 2009 Siemens AG
//
extern "C" {
    pub fn ieee802154_nl_init() -> int __init;
}
extern "C" {
    pub fn ieee802154_nl_exit();
}

extern "C" {
    pub fn ieee802154_nl_mcast(msg: *mut sk_buff, group: c_uint) -> c_int;
}
extern "C" {
    pub fn ieee802154_nl_reply(msg: *mut sk_buff, info: *mut genl_info) -> c_int;
}
// genetlink ops/groups
extern "C" {
    pub fn ieee802154_list_phy(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ieee802154_dump_phy(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn ieee802154_add_iface(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ieee802154_del_iface(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee802154_mcgrp_ids {
    IEEE802154_COORD_MCGRP,
    IEEE802154_BEACON_MCGRP,
}

extern "C" {
    pub fn ieee802154_associate_req(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ieee802154_associate_resp(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ieee802154_disassociate_req(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ieee802154_scan_req(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ieee802154_start_req(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ieee802154_list_iface(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ieee802154_dump_iface(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn ieee802154_set_macparams(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ieee802154_llsec_getparams(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ieee802154_llsec_setparams(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ieee802154_llsec_add_key(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ieee802154_llsec_del_key(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ieee802154_llsec_add_dev(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ieee802154_llsec_del_dev(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ieee802154_llsec_add_devkey(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ieee802154_llsec_del_devkey(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ieee802154_llsec_add_seclevel(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ieee802154_llsec_del_seclevel(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
