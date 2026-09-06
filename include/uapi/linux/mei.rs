//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/mei.h
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
//
// Copyright(c) 2003-2015 Intel Corporation. All rights reserved.
// Intel Management Engine Interface (Intel MEI) Linux driver
// Intel MEI Interface Header
//

//
// This IOCTL is used to associate the current file descriptor with a
// FW Client (given by UUID). This opens a communication channel
// between a host client and a FW client. From this point every read and write
// will communicate with the associated FW client.
// Only in close() (file_operation release()) is the communication between
// the clients disconnected.
//
// The IOCTL argument is a struct with a union that contains
// the input parameter and the output parameter for this IOCTL.
//
// The input parameter is UUID of the FW Client.
// The output parameter is the properties of the FW client
// (FW protocol version and max message size).
//

//
// Intel MEI client information struct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_client {
    pub max_msg_length: __u32,
    pub protocol_version: __u8,
    pub reserved: [__u8; 3],
}

//
// IOCTL Connect Client Data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_connect_client_data {
    pub in_client_uuid: uuid_le,
    pub out_client_properties: mei_client,
}

//
// DOC: set and unset event notification for a connected client
//
// The IOCTL argument is 1 for enabling event notification and 0 for
// disabling the service.
// Return:  -EOPNOTSUPP if the devices doesn't support the feature
//

//
// DOC: retrieve notification
//
// The IOCTL output argument is 1 if an event was pending and 0 otherwise.
// The ioctl has to be called in order to acknowledge pending event.
//
// Return:  -EOPNOTSUPP if the devices doesn't support the feature
//

//
// struct mei_connect_client_vtag - mei client information struct with vtag
//
// @in_client_uuid: UUID of client to connect
// @vtag: virtual tag
// @reserved: reserved for future use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_connect_client_vtag {
    pub in_client_uuid: uuid_le,
    pub vtag: __u8,
    pub reserved: [__u8; 3],
}

//
// struct mei_connect_client_data_vtag - IOCTL connect data union
//
// @connect: input connect data
// @out_client_properties: output client data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_connect_client_data_vtag {
    pub connect: mei_connect_client_vtag,
    pub out_client_properties: mei_client,
}

//
// DOC:
// This IOCTL is used to associate the current file descriptor with a
// FW Client (given by UUID), and virtual tag (vtag).
// The IOCTL opens a communication channel between a host client and
// a FW client on a tagged channel. From this point on, every read
// and write will communicate with the associated FW client
// on the tagged channel.
// Upon close() the communication is terminated.
//
// The IOCTL argument is a struct with a union that contains
// the input parameter and the output parameter for this IOCTL.
//
// The input parameter is UUID of the FW Client, a vtag [0,255].
// The output parameter is the properties of the FW client
// (FW protocol version and max message size).
//
// Clients that do not support tagged connection
// will respond with -EOPNOTSUPP.
//

