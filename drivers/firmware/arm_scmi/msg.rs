//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/arm_scmi/msg.c
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
// For transports using message passing.
//
// Derived from shm.c.
//
// Copyright (C) 2019-2024 ARM Ltd.
// Copyright (C) 2020-2021 OpenSynergy GmbH
//

//
// struct scmi_msg_payld - Transport SDU layout
//
// The SCMI specification requires all parameters, message headers, return
// arguments or any protocol data to be expressed in little endian format only.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_msg_payld {
    pub msg_header: __le32,
    pub msg_payload: [__le32; ],
}

//
// msg_command_size() - Actual size of transport SDU for command.
//
// @xfer: message which core has prepared for sending
//
// Return: transport SDU size.
//
#[no_mangle]
unsafe extern "C" fn msg_command_size(xfer: *mut scmi_xfer) -> usize {
    static size_t msg_command_size(struct scmi_xfer *xfer)
    {
    return sizeof(struct scmi_msg_payld) + xfer.tx.len;
    }
//
// msg_response_size() - Maximum size of transport SDU for response.
//
// @xfer: message which core has prepared for sending
//
// Return: transport SDU size.
//
#[no_mangle]
unsafe extern "C" fn msg_response_size(xfer: *mut scmi_xfer) -> usize {
    static size_t msg_response_size(struct scmi_xfer *xfer)
    {
    return sizeof(struct scmi_msg_payld) + sizeof(__le32) + xfer.rx.len;
    }
//
// msg_tx_prepare() - Set up transport SDU for command.
//
// @msg: transport SDU for command
// @xfer: message which is being sent
//
#[no_mangle]
unsafe extern "C" fn msg_tx_prepare(msg: *mut scmi_msg_payld, xfer: *mut scmi_xfer) {
    static void msg_tx_prepare(struct scmi_msg_payld *msg, struct scmi_xfer *xfer)
    {
    msg.msg_header = cpu_to_le32(pack_scmi_header(&xfer.hdr));
    if (xfer.tx.buf)
    memcpy(msg.msg_payload, xfer.tx.buf, xfer.tx.len);
    }
//
// msg_read_header() - Read SCMI header from transport SDU.
//
// @msg: transport SDU
//
// Return: SCMI header
//
#[no_mangle]
unsafe extern "C" fn msg_read_header(msg: *mut scmi_msg_payld) -> u32 {
    static u32 msg_read_header(struct scmi_msg_payld *msg)
    {
    return le32_to_cpu(msg.msg_header);
    }
//
// msg_fetch_response() - Fetch response SCMI payload from transport SDU.
//
// @msg: transport SDU with response
// @len: transport SDU size
// @xfer: message being responded to
//
    static void msg_fetch_response(struct scmi_msg_payld *msg,
    size_t len, struct scmi_xfer *xfer)
    {
    let mut prefix_len: usize = sizeof(*msg) + sizeof(msg.msg_payload[0]);
    xfer.hdr.status = le32_to_cpu(msg.msg_payload[0]);
    xfer.rx.len = min_t(size_t, xfer.rx.len,
    len >= prefix_len ? len - prefix_len : 0);
// Take a copy to the rx buffer..
    memcpy(xfer.rx.buf, &msg.msg_payload[1], xfer.rx.len);
    }
//
// msg_fetch_notification() - Fetch notification payload from transport SDU.
//
// @msg: transport SDU with notification
// @len: transport SDU size
// @max_len: maximum SCMI payload size to fetch
// @xfer: notification message
//
    static void msg_fetch_notification(struct scmi_msg_payld *msg, size_t len,
    size_t max_len, struct scmi_xfer *xfer)
    {
    xfer.rx.len = min_t(size_t, max_len,
    len >= sizeof(*msg) ? len - sizeof(*msg) : 0);
// Take a copy to the rx buffer..
    memcpy(xfer.rx.buf, msg.msg_payload, xfer.rx.len);
    }
    static const struct scmi_message_operations scmi_msg_ops = {
    .tx_prepare = msg_tx_prepare,
    .command_size = msg_command_size,
    .response_size = msg_response_size,
    .read_header = msg_read_header,
    .fetch_response = msg_fetch_response,
    .fetch_notification = msg_fetch_notification,
    };
    const struct scmi_message_operations *scmi_message_operations_get(void)
    {
    return &scmi_msg_ops;
    }
