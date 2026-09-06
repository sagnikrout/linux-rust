//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/issei/issei_dev.h
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
// Copyright (C) 2023-2026 Intel Corporation

pub const ISSEI_HOST_CLIENTS_MAX: c_int = 255;
pub const ISSEI_SUPPORTED_PROTOCOL_VER: c_int = 1;
pub const ISSEI_MAX_CONSEC_RESET: c_int = 3;

pub const ISSEI_STOP_TIMEOUT_MSEC: c_int = 500;

//
// struct issei_write_buf - write buffer object
// @list: linked list pointer
// @cl: host client that requested this write
// @data: data to write
// @data_size: data size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct issei_write_buf {
    pub list: list_head,
    pub cl: *mut issei_host_client,
    pub data: *const u8,
    pub data_size: usize,
}

//
// struct issei_hw_ops - callbacks for hardware operations
// @irq_clear: clear irq
// @irq_enable: enable irq
// @irq_disable: disable irq
// @irq_sync: sync irq
// @hw_reset: initiate hardware reset
// @hw_config: initial hardware config
// @hw_is_ready: check if hardware is ready
// @hw_reset_release: release hardware from reset
// @host_set_ready: set host ready indicator
// @setup_message_send: send setup message
// @setup_message_recv: receive setup message
// @irq_write_generate: generate interrupt on write complete
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct issei_hw_ops {
    pub idev): *mut *mut void (irq_clear)(struct issei_device,
    pub idev): *mut *mut void (irq_enable)(struct issei_device,
    pub idev): *mut *mut void (irq_disable)(struct issei_device,
    pub idev): *mut *mut void (irq_sync)(struct issei_device,
    pub enable): *mut *mut *mut int (hw_reset)(struct issei_device idev, bool,
    pub idev): *mut *mut int (hw_config)(struct issei_device,
    pub idev): *mut *mut bool (hw_is_ready)(struct issei_device,
    pub idev): *mut *mut void (hw_reset_release)(struct issei_device,
    pub idev): *mut *mut void (host_set_ready)(struct issei_device,
    pub idev): *mut *mut int (setup_message_send)(struct issei_device,
    pub idev): *mut *mut int (setup_message_recv)(struct issei_device,
    pub idev): *mut *mut int (irq_write_generate)(struct issei_device,
}

//
// enum issei_rst_state: driver reset flow states
// @ISSEI_RST_STATE_INIT: initial state
// @ISSEI_RST_STATE_HW_READY: waiting for HW to be ready
// @ISSEI_RST_STATE_SETUP: waiting for channel setup completion
// @ISSEI_RST_STATE_START: waiting for start handshake completion
// @ISSEI_RST_STATE_CLIENT_ENUM: waiting for client enumeration
// @ISSEI_RST_STATE_DONE: reset flow is done
// @ISSEI_RST_STATE_DISABLED: flow is disabled
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum issei_rst_state {
    ISSEI_RST_STATE_INIT,
    ISSEI_RST_STATE_HW_READY,
    ISSEI_RST_STATE_SETUP,
    ISSEI_RST_STATE_START,
    ISSEI_RST_STATE_CLIENT_ENUM,
    ISSEI_RST_STATE_DONE,
    ISSEI_RST_STATE_DISABLED,
}

//
// struct issei_device - issei device
// @parent: parent device object
// @dev: associated device object
// @cdev: character device
// @minor: allocated minor number
// @wait_has_data: wait queue for data
// @has_data: there are data to process
// @power_down: device is powering down
// @wait_rst_state: waitqueue for reset state processing
// @rst_state: reset state
// @fw_protocol_ver: protocol version
// @fw_version: firmware version
// @process_thread: worker thread
// @reset_count: number of consecutive link reset attempts
// @all_reset_count: cumilative number of link reset attempts
// @client_lock: mutex to protect client lists and write queue
// @host_client_list: host clients list
// @host_client_last_id: last allocated host client id
// @host_client_count: number of active host clients
// @fw_client_list: firmware clients list
// @write_queue: write queue
// @last_write_ts: last write timestamp
// @dma: DMA memory configuration
// @ops: hardware operations
// @hw: hw-specific data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct issei_device {
    pub parent: *mut device,
    pub dev: device,
    pub cdev: *mut cdev,
    pub minor: u32,
    pub wait_has_data: wait_queue_head_t,
    pub has_data: bool,
    pub power_down: bool,
    pub wait_rst_state: wait_queue_head_t,
    pub rst_state: issei_rst_state,
    pub fw_protocol_ver: u16,
    pub fw_version: [u16; 4],
// reset flow
    pub process_thread: *mut task_struct,
    pub reset_count: u8,
    pub all_reset_count: u8,
// clients
    pub client_lock: mutex,
    pub host_client_list: list_head,
    pub host_client_last_id: u16,
    pub host_client_count: u8,
    pub fw_clients: *mut kset,
    pub fw_client_list: list_head,
    pub write_queue: list_head,
    pub last_write_ts: ktime_t,
    pub dma: issei_dma,
    pub ops: *const issei_hw_ops,
    pub hw: [c_char; ],
}

extern "C" {
    pub fn issei_start(idev: *mut issei_device) -> c_int;
}
extern "C" {
    pub fn issei_stop(idev: *mut issei_device);
}
