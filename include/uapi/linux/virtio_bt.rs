//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/virtio_bt.h
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


// SPDX-License-Identifier: BSD-3-Clause

// Feature bits

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtio_bt_config_type {
    VIRTIO_BT_CONFIG_TYPE_PRIMARY	= 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtio_bt_config_vendor {
    VIRTIO_BT_CONFIG_VENDOR_NONE	= 0,
    VIRTIO_BT_CONFIG_VENDOR_ZEPHYR	= 1,
    VIRTIO_BT_CONFIG_VENDOR_INTEL	= 2,
    VIRTIO_BT_CONFIG_VENDOR_REALTEK	= 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_bt_config {
    pub type: __u8,
    pub vendor: __u16,
    pub msft_opcode: __u16,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_bt_config_v2 {
    pub type: __u8,
    pub alignment: __u8,
    pub vendor: __u16,
    pub msft_opcode: __u16,
}
