//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath10k/usb.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2004-2011 Atheros Communications Inc.
// Copyright (c) 2011-2012 Qualcomm Atheros, Inc.
// Copyright (c) 2016-2017 Erik Stromdahl <erik.stromdahl@gmail.com>
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//
// constants
pub const TX_URB_COUNT: c_int = 32;
pub const RX_URB_COUNT: c_int = 32;
pub const ATH10K_USB_RX_BUFFER_SIZE: c_int = 4096;

// USB endpoint definitions
pub const ATH10K_USB_EP_ADDR_APP_CTRL_IN: c_uint = 0x81;
pub const ATH10K_USB_EP_ADDR_APP_DATA_IN: c_uint = 0x82;
pub const ATH10K_USB_EP_ADDR_APP_DATA2_IN: c_uint = 0x83;
pub const ATH10K_USB_EP_ADDR_APP_INT_IN: c_uint = 0x84;
pub const ATH10K_USB_EP_ADDR_APP_CTRL_OUT: c_uint = 0x01;
pub const ATH10K_USB_EP_ADDR_APP_DATA_LP_OUT: c_uint = 0x02;
pub const ATH10K_USB_EP_ADDR_APP_DATA_MP_OUT: c_uint = 0x03;
pub const ATH10K_USB_EP_ADDR_APP_DATA_HP_OUT: c_uint = 0x04;
// diagnostic command definitions
pub const ATH10K_USB_CONTROL_REQ_SEND_BMI_CMD: c_int = 1;
pub const ATH10K_USB_CONTROL_REQ_RECV_BMI_RESP: c_int = 2;
pub const ATH10K_USB_CONTROL_REQ_DIAG_CMD: c_int = 3;
pub const ATH10K_USB_CONTROL_REQ_DIAG_RESP: c_int = 4;
pub const ATH10K_USB_CTRL_DIAG_CC_READ: c_int = 0;
pub const ATH10K_USB_CTRL_DIAG_CC_WRITE: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_usb_ctrl_diag_cmd_write {
    pub cmd: __le32,
    pub address: __le32,
    pub value: __le32,
    pub padding: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_usb_ctrl_diag_cmd_read {
    pub cmd: __le32,
    pub address: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_usb_ctrl_diag_resp_read {
    pub value: [u8; 4],
    pub __packed: },
// tx/rx pipes for usb
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_usb_pipe_id {
    ATH10K_USB_PIPE_TX_CTRL = 0,
    ATH10K_USB_PIPE_TX_DATA_LP,
    ATH10K_USB_PIPE_TX_DATA_MP,
    ATH10K_USB_PIPE_TX_DATA_HP,
    ATH10K_USB_PIPE_RX_CTRL,
    ATH10K_USB_PIPE_RX_DATA,
    ATH10K_USB_PIPE_RX_DATA2,
    ATH10K_USB_PIPE_RX_INT,
    ATH10K_USB_PIPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_usb_pipe {
    pub urb_list_head: list_head,
    pub urb_submitted: usb_anchor,
    pub urb_alloc: u32,
    pub urb_cnt: u32,
    pub urb_cnt_thresh: u32,
    pub usb_pipe_handle: c_uint,
    pub flags: u32,
    pub ep_address: u8,
    pub logical_pipe_num: u8,
    pub ar_usb: *mut ath10k_usb,
    pub max_packet_size: u16,
    pub io_complete_work: work_struct,
    pub io_comp_queue: sk_buff_head,
    pub ep_desc: *mut usb_endpoint_descriptor,
}

// usb device object
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_usb {
// protects pipe->urb_list_head and  pipe->urb_cnt
    pub cs_lock: spinlock_t,
    pub udev: *mut usb_device,
    pub interface: *mut usb_interface,
    pub pipes: [ath10k_usb_pipe; ATH10K_USB_PIPE_MAX],
    pub diag_cmd_buffer: *mut u8,
    pub diag_resp_buffer: *mut u8,
    pub ar: *mut ath10k,
}

// usb urb object
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_urb_context {
    pub link: list_head,
    pub pipe: *mut ath10k_usb_pipe,
    pub skb: *mut sk_buff,
    pub ar: *mut ath10k,
}
