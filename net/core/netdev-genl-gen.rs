//! Automatically rewritten from C Header to Rust Module
//! Source: net/core/netdev-genl-gen.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
// Do not edit directly, auto-generated from:
// Documentation/netlink/specs/netdev.yaml
// YNL-GEN kernel header
// To regenerate run: tools/net/ynl/ynl-regen.sh

// Common nested types
extern "C" {
    pub fn netdev_nl_dev_get_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn netdev_nl_dev_get_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn netdev_nl_page_pool_get_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn netdev_nl_queue_get_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn netdev_nl_napi_get_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn netdev_nl_napi_get_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn netdev_nl_bind_rx_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn netdev_nl_napi_set_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn netdev_nl_bind_tx_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn netdev_nl_queue_create_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn netdev_nl_sock_priv_init(priv: *mut netdev_nl_sock);
}
extern "C" {
    pub fn netdev_nl_sock_priv_destroy(priv: *mut netdev_nl_sock);
}
