//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/virtio_can.h
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
//
// Copyright (C) 2021-2023 OpenSynergy GmbH
// Copyright Red Hat, Inc. 2025
//

// Feature bit numbers
pub const VIRTIO_CAN_F_CAN_CLASSIC: c_int = 0;
pub const VIRTIO_CAN_F_CAN_FD: c_int = 1;
pub const VIRTIO_CAN_F_RTR_FRAMES: c_int = 2;
pub const VIRTIO_CAN_F_LATE_TX_ACK: c_int = 3;
// CAN Result Types
pub const VIRTIO_CAN_RESULT_OK: c_int = 0;
pub const VIRTIO_CAN_RESULT_NOT_OK: c_int = 1;
// CAN flags to determine type of CAN Id
pub const VIRTIO_CAN_FLAGS_EXTENDED: c_uint = 0x8000;
pub const VIRTIO_CAN_FLAGS_FD: c_uint = 0x4000;
pub const VIRTIO_CAN_FLAGS_RTR: c_uint = 0x2000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_can_config {

// CAN controller status
    pub status: __le16,
}

// TX queue message types
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_can_tx_out {
pub const VIRTIO_CAN_TX: c_uint = 0x0001;
    pub msg_type: __le16,
    pub /: *mut *mut __le16 length; / 0..8 CC, 0..64 CAN-FD, 0..2048 CAN-XL, 12 bits,
    pub /: *mut *mut __u8 reserved_classic_dlc; / If CAN classic length = 8 then DLC can be 8..15,
    pub padding: __u8,
    pub /: *mut *mut __le16 reserved_xl_priority; / May be needed for CAN XL priority,
    pub flags: __le32,
    pub can_id: __le32,
    pub __counted_by_le(length): __u8 sdu[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_can_tx_in {
    pub result: __u8,
}

// RX queue message types
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_can_rx {
pub const VIRTIO_CAN_RX: c_uint = 0x0101;
    pub msg_type: __le16,
    pub /: *mut *mut __le16 length; / 0..8 CC, 0..64 CAN-FD, 0..2048 CAN-XL, 12 bits,
    pub /: *mut *mut __u8 reserved_classic_dlc; / If CAN classic length = 8 then DLC can be 8..15,
    pub padding: __u8,
    pub /: *mut *mut __le16 reserved_xl_priority; / May be needed for CAN XL priority,
    pub flags: __le32,
    pub can_id: __le32,
    pub __counted_by_le(length): __u8 sdu[],
}

// Control queue message types
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_can_control_out {
pub const VIRTIO_CAN_SET_CTRL_MODE_START: c_uint = 0x0201;
pub const VIRTIO_CAN_SET_CTRL_MODE_STOP: c_uint = 0x0202;
    pub msg_type: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_can_control_in {
    pub result: __u8,
}
