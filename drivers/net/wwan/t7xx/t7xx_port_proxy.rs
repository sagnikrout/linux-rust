//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/t7xx/t7xx_port_proxy.h
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
// Copyright (c) 2021, MediaTek Inc.
// Copyright (c) 2021-2022, Intel Corporation.
//
// Authors:
// Amir Hanania <amir.hanania@intel.com>
// Haijun Liu <haijun.liu@mediatek.com>
// Moises Veleta <moises.veleta@intel.com>
// Ricardo Martinez <ricardo.martinez@linux.intel.com>
//
// Contributors:
// Chiranjeevi Rapolu <chiranjeevi.rapolu@intel.com>
// Eliot Lee <eliot.lee@intel.com>
// Sreehari Kancharla <sreehari.kancharla@intel.com>
//

pub const MTK_QUEUES: c_int = 16;
pub const RX_QUEUE_MAXLEN: c_int = 32;
pub const CTRL_QUEUE_MAXLEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum port_cfg_id {
    PORT_CFG_ID_INVALID,
    PORT_CFG_ID_NORMAL,
    PORT_CFG_ID_EARLY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct port_proxy {
    pub port_count: c_int,
    pub 1]: list_head rx_ch_ports[PORT_CH_ID_MASK +,
    pub queue_ports: [list_head; CLDMA_NUM][MTK_QUEUES],
    pub dev: *mut device,
    pub cfg_id: port_cfg_id,
    pub ports: [t7xx_port; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccci_header {
    pub packet_header: __le32,
    pub packet_len: __le32,
    pub status: __le32,
    pub ex_msg: __le32,
}

// Coupled with HW - indicates if there is data following the CCCI header or not
pub const CCCI_HEADER_NO_DATA: c_uint = 0xffffffff;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctrl_msg_header {
    pub ctrl_msg_id: __le32,
    pub ex_msg: __le32,
    pub data_length: __le32,
}

// Control identification numbers for AP<->MD messages
pub const CTL_ID_HS1_MSG: c_uint = 0x0;
pub const CTL_ID_HS2_MSG: c_uint = 0x1;
pub const CTL_ID_HS3_MSG: c_uint = 0x2;
pub const CTL_ID_MD_EX: c_uint = 0x4;
pub const CTL_ID_DRV_VER_ERROR: c_uint = 0x5;
pub const CTL_ID_MD_EX_ACK: c_uint = 0x6;
pub const CTL_ID_MD_EX_PASS: c_uint = 0x8;
pub const CTL_ID_PORT_ENUM: c_uint = 0x9;
// Modem exception check identification code - "EXCP"
pub const MD_EX_CHK_ID: c_uint = 0x45584350;
// Modem exception check acknowledge identification code - "EREC"
pub const MD_EX_CHK_ACK_ID: c_uint = 0x45524543;

pub const PORT_ENUM_VER: c_int = 0;
pub const PORT_ENUM_HEAD_PATTERN: c_uint = 0x5a5a5a5a;
pub const PORT_ENUM_TAIL_PATTERN: c_uint = 0xa5a5a5a5;
pub const PORT_ENUM_VER_MISMATCH: c_uint = 0x00657272;
// Port operations mapping

extern "C" {
    pub fn t7xx_proxy_debug_ports_show(t7xx_dev: *mut t7xx_pci_dev, show: bool);
}
extern "C" {
    pub fn t7xx_port_proxy_reset(port_prox: *mut port_proxy);
}
extern "C" {
    pub fn t7xx_port_proxy_uninit(port_prox: *mut port_proxy);
}
extern "C" {
    pub fn t7xx_port_proxy_init(md: *mut t7xx_modem) -> c_int;
}
extern "C" {
    pub fn t7xx_port_proxy_md_status_notify(port_prox: *mut port_proxy, state: c_uint);
}
extern "C" {
    pub fn t7xx_port_enum_msg_handler(md: *mut t7xx_modem, msg: *mut c_void, msg_len: usize) -> c_int;
}
extern "C" {
    pub fn t7xx_port_proxy_set_cfg(md: *mut t7xx_modem, cfg_id: port_cfg_id);
}
extern "C" {
    pub fn t7xx_port_proxy_recv_skb(queue: *mut cldma_queue, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn t7xx_port_proxy_recv_skb_from_dedicated_queue(queue: *mut cldma_queue, skb: *mut sk_buff) -> c_int;
}
