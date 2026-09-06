//! Automatically rewritten from C Header to Rust Module
//! Source: net/nfc/hci/hci.h
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
// Copyright (C) 2012  Intel Corporation. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gate_pipe_map {
    pub gate: u8,
    pub pipe: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hcp_message {
    pub /: *mut *mut u8 header; / type -cmd,evt,rsp- + instruction,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hcp_packet {
    pub /: *mut *mut u8 header; / cbit+pipe,
    pub message: hcp_message,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hcp_exec_waiter {
    pub wq: *mut wait_queue_head_t,
    pub exec_complete: bool,
    pub exec_result: c_int,
    pub result_skb: *mut sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_msg {
    pub msg_l: list_head,
    pub msg_frags: sk_buff_head,
    pub wait_response: bool,
    pub cb: data_exchange_cb_t,
    pub cb_context: *mut c_void,
    pub completion_delay: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_create_pipe_params {
    pub src_gate: u8,
    pub dest_host: u8,
    pub dest_gate: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_create_pipe_resp {
    pub src_host: u8,
    pub src_gate: u8,
    pub dest_host: u8,
    pub dest_gate: u8,
    pub pipe: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_delete_pipe_noti {
    pub pipe: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_all_pipe_cleared_noti {
    pub host: u8,
    pub __packed: },
pub const NFC_HCI_FRAGMENT: c_uint = 0x7f;

    pub completion_delay): c_ulong,
    pub skb): *mut u8 instruction, struct sk_buff,
// HCP headers
pub const NFC_HCI_HCP_PACKET_HEADER_LEN: c_int = 1;
pub const NFC_HCI_HCP_MESSAGE_HEADER_LEN: c_int = 1;
pub const NFC_HCI_HCP_HEADER_LEN: c_int = 2;
// HCP types
pub const NFC_HCI_HCP_COMMAND: c_uint = 0x00;
pub const NFC_HCI_HCP_EVENT: c_uint = 0x01;
pub const NFC_HCI_HCP_RESPONSE: c_uint = 0x02;
// Generic commands
pub const NFC_HCI_ANY_SET_PARAMETER: c_uint = 0x01;
pub const NFC_HCI_ANY_GET_PARAMETER: c_uint = 0x02;
pub const NFC_HCI_ANY_OPEN_PIPE: c_uint = 0x03;
pub const NFC_HCI_ANY_CLOSE_PIPE: c_uint = 0x04;
// Reader RF commands
pub const NFC_HCI_WR_XCHG_DATA: c_uint = 0x10;
// Admin commands
pub const NFC_HCI_ADM_CREATE_PIPE: c_uint = 0x10;
pub const NFC_HCI_ADM_DELETE_PIPE: c_uint = 0x11;
pub const NFC_HCI_ADM_NOTIFY_PIPE_CREATED: c_uint = 0x12;
pub const NFC_HCI_ADM_NOTIFY_PIPE_DELETED: c_uint = 0x13;
pub const NFC_HCI_ADM_CLEAR_ALL_PIPE: c_uint = 0x14;
pub const NFC_HCI_ADM_NOTIFY_ALL_PIPE_CLEARED: c_uint = 0x15;
// Generic responses
pub const NFC_HCI_ANY_OK: c_uint = 0x00;
pub const NFC_HCI_ANY_E_NOT_CONNECTED: c_uint = 0x01;
pub const NFC_HCI_ANY_E_CMD_PAR_UNKNOWN: c_uint = 0x02;
pub const NFC_HCI_ANY_E_NOK: c_uint = 0x03;
pub const NFC_HCI_ANY_E_PIPES_FULL: c_uint = 0x04;
pub const NFC_HCI_ANY_E_REG_PAR_UNKNOWN: c_uint = 0x05;
pub const NFC_HCI_ANY_E_PIPE_NOT_OPENED: c_uint = 0x06;
pub const NFC_HCI_ANY_E_CMD_NOT_SUPPORTED: c_uint = 0x07;
pub const NFC_HCI_ANY_E_INHIBITED: c_uint = 0x08;
pub const NFC_HCI_ANY_E_TIMEOUT: c_uint = 0x09;
pub const NFC_HCI_ANY_E_REG_ACCESS_DENIED: c_uint = 0x0a;
pub const NFC_HCI_ANY_E_PIPE_ACCESS_DENIED: c_uint = 0x0b;
