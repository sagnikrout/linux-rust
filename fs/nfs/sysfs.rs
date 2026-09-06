//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfs/sysfs.h
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
// Copyright (c) 2019 Hammerspace Inc
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_netns_client {
    pub kobject: kobject,
    pub nfs_net_kobj: kobject,
    pub net: *mut net,
    pub identifier: *const char __rcu,
}

extern "C" {
    pub fn nfs_sysfs_init() -> c_int;
}
extern "C" {
    pub fn nfs_sysfs_exit();
}
extern "C" {
    pub fn nfs_netns_sysfs_setup(netns: *mut nfs_net, net: *mut net);
}
extern "C" {
    pub fn nfs_netns_sysfs_destroy(netns: *mut nfs_net);
}
extern "C" {
    pub fn nfs_sysfs_add_server(s: *mut nfs_server);
}
extern "C" {
    pub fn nfs_sysfs_move_server_to_sb(s: *mut super_block);
}
extern "C" {
    pub fn nfs_sysfs_move_sb_to_server(s: *mut nfs_server);
}
extern "C" {
    pub fn nfs_sysfs_remove_server(s: *mut nfs_server);
}
