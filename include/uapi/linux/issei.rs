//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/issei.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Copyright (C) 2023-2026 Intel Corporation
// Intel Silicon Security Engine Interface (ISSEI) Linux driver:
// ISSEI Interface Header
//

//
// This ioctl is used to associate the current file descriptor with a
// FW Client (given by UUID). This opens a communication channel
// between a host client and a FW client. From this point every read and write
// will communicate with the associated FW client.
// The communication between the clients can be terminated by
// IOCTL_ISSEI_DISCONNECT_CLIENT IOCTL or by
// closing the file descriptor (file_operation release()).
//
// The ioctl argument is a struct with a union that contains
// the input parameter and the output parameter for this ioctl.
//
// The input parameter is UUID of the FW Client.
// The output parameter is the properties of the FW client
// (FW protocol version, max message size and client flags).
//

//
// struct issei_client - ISSEI client information structure
// @max_msg_length: maximum message length supported by the firmware client (in bytes)
// @protocol_version: protocol version reported by the firmware client
// @reserved1: reserved
// @flags: flag bitmask reported by the firmware client
// @reserved2: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct issei_client {
    pub max_msg_length: __u32,
    pub protocol_version: __u8,
    pub reserved1: [__u8; 3],
    pub flags: __u32,
    pub reserved2: __u32,
}

pub const ISSEI_IOCTL_UUID_LEN: c_int = 16;
//
// struct issei_connect_client_data - ioctl Connect Client Data structure
// @in_client_uuid: unique id of the firmware client to connect to (from user space to kernel)
// @out_client_properties: connected firmware client properties (from kernel to user space)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct issei_connect_client_data {
    pub in_client_uuid: [__u8; ISSEI_IOCTL_UUID_LEN],
    pub out_client_properties: issei_client,
}

//
// This ioctl is used to terminate association between
// the host client and the FW client.
//

