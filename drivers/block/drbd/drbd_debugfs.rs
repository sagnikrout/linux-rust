//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/block/drbd/drbd_debugfs.h
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

extern "C" {
    pub fn drbd_debugfs_init() -> void __init;
}
extern "C" {
    pub fn drbd_debugfs_cleanup();
}
extern "C" {
    pub fn drbd_debugfs_resource_add(resource: *mut drbd_resource);
}
extern "C" {
    pub fn drbd_debugfs_resource_cleanup(resource: *mut drbd_resource);
}
extern "C" {
    pub fn drbd_debugfs_connection_add(connection: *mut drbd_connection);
}
extern "C" {
    pub fn drbd_debugfs_connection_cleanup(connection: *mut drbd_connection);
}
extern "C" {
    pub fn drbd_debugfs_device_add(device: *mut drbd_device);
}
extern "C" {
    pub fn drbd_debugfs_device_cleanup(device: *mut drbd_device);
}
extern "C" {
    pub fn drbd_debugfs_peer_device_add(peer_device: *mut drbd_peer_device);
}
extern "C" {
    pub fn drbd_debugfs_peer_device_cleanup(peer_device: *mut drbd_peer_device);
}

