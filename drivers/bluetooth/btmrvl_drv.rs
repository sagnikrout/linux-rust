//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/bluetooth/btmrvl_drv.h
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
//
// Marvell Bluetooth driver: global definitions & declarations
//
// Copyright (C) 2009, Marvell International Ltd.
//

pub const BTM_HEADER_LEN: c_int = 4;
pub const BTM_UPLD_SIZE: c_int = 2312;
// Time to wait until Host Sleep state change in millisecond

// Time to wait for command response in millisecond

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdwr_status {
    RDWR_STATUS_SUCCESS = 0,
    RDWR_STATUS_FAILURE = 1,
    RDWR_STATUS_DONE = 2
}

pub const FW_DUMP_MAX_NAME_LEN: c_int = 8;
pub const FW_DUMP_HOST_READY: c_uint = 0xEE;
pub const FW_DUMP_DONE: c_uint = 0xFF;
pub const FW_DUMP_READ_DONE: c_uint = 0xFE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct memory_type_mapping {
    pub mem_name: [u8; FW_DUMP_MAX_NAME_LEN],
    pub mem_ptr: *mut u8,
    pub mem_size: u32,
    pub done_flag: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btmrvl_thread {
    pub task: *mut task_struct,
    pub wait_q: wait_queue_head_t,
    pub priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btmrvl_device {
    pub card: *mut c_void,
    pub hcidev: *mut hci_dev,
    pub dev_type: u8,
    pub tx_dnld_rdy: u8,
    pub psmode: u8,
    pub pscmd: u8,
    pub hsmode: u8,
    pub hscmd: u8,
// Low byte is gap, high byte is GPIO
    pub gpio_gap: u16,
    pub hscfgcmd: u8,
    pub sendcmdflag: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btmrvl_adapter {
    pub hw_regs_buf: *mut c_void,
    pub hw_regs: *mut u8,
    pub int_count: u32,
    pub tx_queue: sk_buff_head,
    pub psmode: u8,
    pub ps_state: u8,
    pub hs_state: u8,
    pub wakeup_tries: u8,
    pub cmd_wait_q: wait_queue_head_t,
    pub event_hs_wait_q: wait_queue_head_t,
    pub cmd_complete: u8,
    pub is_suspended: bool,
    pub is_suspending: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btmrvl_private {
    pub btmrvl_dev: btmrvl_device,
    pub adapter: *mut btmrvl_adapter,
    pub main_thread: btmrvl_thread,
    pub nb): *mut *mut u8 payload, u16,
    pub priv): *mut *mut int (hw_wakeup_firmware)(struct btmrvl_private,
    pub priv): *mut *mut int (hw_process_int_status)(struct btmrvl_private,
    pub /: *mut *mut spinlock_t driver_lock; / spinlock used by driver,

    pub debugfs_data: *mut c_void,

    pub surprise_removed: bool,
}

pub const MRVL_VENDOR_PKT: c_uint = 0xFE;
// Vendor specific Bluetooth commands
pub const BT_CMD_PSCAN_WIN_REPORT_ENABLE: c_uint = 0xFC03;
pub const BT_CMD_ROUTE_SCO_TO_HOST: c_uint = 0xFC1D;
pub const BT_CMD_SET_BDADDR: c_uint = 0xFC22;
pub const BT_CMD_AUTO_SLEEP_MODE: c_uint = 0xFC23;
pub const BT_CMD_HOST_SLEEP_CONFIG: c_uint = 0xFC59;
pub const BT_CMD_HOST_SLEEP_ENABLE: c_uint = 0xFC5A;
pub const BT_CMD_MODULE_CFG_REQ: c_uint = 0xFC5B;
pub const BT_CMD_LOAD_CONFIG_DATA: c_uint = 0xFC61;
// Sub-commands: Module Bringup/Shutdown Request/Response
pub const MODULE_BRINGUP_REQ: c_uint = 0xF1;
pub const MODULE_BROUGHT_UP: c_uint = 0x00;
pub const MODULE_ALREADY_UP: c_uint = 0x0C;
pub const MODULE_SHUTDOWN_REQ: c_uint = 0xF2;
// Vendor specific Bluetooth events
pub const BT_EVENT_AUTO_SLEEP_MODE: c_uint = 0x23;
pub const BT_EVENT_HOST_SLEEP_CONFIG: c_uint = 0x59;
pub const BT_EVENT_HOST_SLEEP_ENABLE: c_uint = 0x5A;
pub const BT_EVENT_MODULE_CFG_REQ: c_uint = 0x5B;
pub const BT_EVENT_POWER_STATE: c_uint = 0x20;
// Bluetooth Power States
pub const BT_PS_ENABLE: c_uint = 0x02;
pub const BT_PS_DISABLE: c_uint = 0x03;
pub const BT_PS_SLEEP: c_uint = 0x01;
// Host Sleep states
pub const HS_ACTIVATED: c_uint = 0x01;
pub const HS_DEACTIVATED: c_uint = 0x00;
// Power Save modes
pub const PS_SLEEP: c_uint = 0x01;
pub const PS_AWAKE: c_uint = 0x00;
pub const BT_CAL_HDR_LEN: c_int = 4;
pub const BT_CAL_DATA_SIZE: c_int = 28;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btmrvl_event {
    pub /: *mut *mut u8 ec; / event counter,
    pub length: u8,
    pub data: [u8; 4],
    pub __packed: },
// Prototype of global function
    pub priv): *mut int btmrvl_register_hdev(struct btmrvl_private,
    pub card): *mut *mut btmrvl_private btmrvl_add_card(void,
    pub priv): *mut int btmrvl_remove_card(struct btmrvl_private,
    pub priv): *mut void btmrvl_interrupt(struct btmrvl_private,
    pub skb): *mut *mut bool btmrvl_check_evtpkt(struct btmrvl_private priv, struct sk_buff,
    pub skb): *mut *mut int btmrvl_process_event(struct btmrvl_private priv, struct sk_buff,
    pub subcmd): *mut *mut int btmrvl_send_module_cfg_cmd(struct btmrvl_private priv, u8,
    pub subcmd): *mut *mut int btmrvl_pscan_window_reporting(struct btmrvl_private priv, u8,
    pub priv): *mut int btmrvl_send_hscfg_cmd(struct btmrvl_private,
    pub priv): *mut int btmrvl_enable_ps(struct btmrvl_private,
    pub priv): *mut int btmrvl_prepare_command(struct btmrvl_private,
    pub priv): *mut int btmrvl_enable_hs(struct btmrvl_private,

    pub hdev): *mut void btmrvl_debugfs_init(struct hci_dev,
    pub hdev): *mut void btmrvl_debugfs_remove(struct hci_dev,
