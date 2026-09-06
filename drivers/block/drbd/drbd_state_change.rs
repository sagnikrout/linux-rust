//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/block/drbd/drbd_state_change.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbd_resource_state_change {
    pub resource: *mut drbd_resource,
    pub role: [drbd_role; 2],
    pub susp: [bool; 2],
    pub susp_nod: [bool; 2],
    pub susp_fen: [bool; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbd_device_state_change {
    pub device: *mut drbd_device,
    pub disk_state: [drbd_disk_state; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbd_connection_state_change {
    pub connection: *mut drbd_connection,
    pub /: *mut *mut drbd_conns cstate[2]; / drbd9: drbd_conn_state,
    pub peer_role: [drbd_role; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbd_peer_device_state_change {
    pub peer_device: *mut drbd_peer_device,
    pub disk_state: [drbd_disk_state; 2],
    pub /: *mut *mut drbd_conns repl_state[2]; / drbd9: drbd_repl_state,
    pub resync_susp_user: [bool; 2],
    pub resync_susp_peer: [bool; 2],
    pub resync_susp_dependency: [bool; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbd_state_change {
    pub list: list_head,
    pub n_devices: c_uint,
    pub n_connections: c_uint,
    pub resource: [drbd_resource_state_change; 1],
    pub devices: *mut drbd_device_state_change,
    pub connections: *mut drbd_connection_state_change,
    pub peer_devices: *mut drbd_peer_device_state_change,
}

extern "C" {
    pub fn copy_old_to_new_state_change(: *mut drbd_state_change);
}
extern "C" {
    pub fn forget_state_change(: *mut drbd_state_change);
}
