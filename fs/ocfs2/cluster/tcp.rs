//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/cluster/tcp.h
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
// tcp.h
//
// Function prototypes
//
// Copyright (C) 2004 Oracle.  All rights reserved.
//

// same as hb delay, we're waiting for another node to recognize our hb
pub const O2NET_RECONNECT_DELAY_MS_DEFAULT: c_int = 2000;
pub const O2NET_KEEPALIVE_DELAY_MS_DEFAULT: c_int = 2000;
pub const O2NET_IDLE_TIMEOUT_MS_DEFAULT: c_int = 30000;
pub const O2NET_TCP_USER_TIMEOUT: c_uint = 0x7fffffff;
// TODO: figure this out....
// ?????????????????????????
// When the server has died, an ICMP port unreachable
// message prompts ECONNREFUSED.
extern "C" {
    pub fn o2net_unregister_handler_list(list: *mut list_head);
}
extern "C" {
    pub fn o2net_unregister_and_flush_handler_list(list: *mut list_head);
}
extern "C" {
    pub fn o2net_fill_node_map(map: *mut c_ulong, bytes: unsigned);
}
extern "C" {
    pub fn o2net_register_hb_callbacks() -> c_int;
}
extern "C" {
    pub fn o2net_unregister_hb_callbacks();
}
extern "C" {
    pub fn o2net_start_listening(node: *mut o2nm_node) -> c_int;
}
extern "C" {
    pub fn o2net_complete_start_listening(node: *mut o2nm_node);
}
extern "C" {
    pub fn o2net_stop_listening(node: *mut o2nm_node);
}
extern "C" {
    pub fn o2net_disconnect_node(node: *mut o2nm_node);
}
extern "C" {
    pub fn o2net_num_connected_peers() -> c_int;
}
extern "C" {
    pub fn o2net_init() -> c_int;
}
extern "C" {
    pub fn o2net_exit();
}

extern "C" {
    pub fn o2net_debugfs_init();
}
extern "C" {
    pub fn o2net_debugfs_exit();
}
extern "C" {
    pub fn o2net_debug_add_nst(nst: *mut o2net_send_tracking);
}
extern "C" {
    pub fn o2net_debug_del_nst(nst: *mut o2net_send_tracking);
}
extern "C" {
    pub fn o2net_debug_add_sc(sc: *mut o2net_sock_container);
}
extern "C" {
    pub fn o2net_debug_del_sc(sc: *mut o2net_sock_container);
}

