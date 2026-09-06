//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/t7xx/t7xx_port.h
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
// Haijun Liu <haijun.liu@mediatek.com>
// Moises Veleta <moises.veleta@intel.com>
// Ricardo Martinez <ricardo.martinez@linux.intel.com>
//
// Contributors:
// Amir Hanania <amir.hanania@intel.com>
// Andy Shevchenko <andriy.shevchenko@linux.intel.com>
// Chandrashekar Devegowda <chandrashekar.devegowda@intel.com>
// Eliot Lee <eliot.lee@intel.com>
//

// Channel ID and Message ID definitions.
// The channel number consists of peer_id(15:12) , channel_id(11:0)
// peer_id:
// 0:reserved, 1: to AP, 2: to MD
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum port_ch {
// to AP
    PORT_CH_AP_CONTROL_RX = 0x1000,
    PORT_CH_AP_CONTROL_TX = 0x1001,
    PORT_CH_AP_ADB_RX = 0x100a,
    PORT_CH_AP_ADB_TX = 0x100b,

// to MD
    PORT_CH_CONTROL_RX = 0x2000,
    PORT_CH_CONTROL_TX = 0x2001,
    PORT_CH_UART1_RX = 0x2006,	/* META */
    PORT_CH_UART1_TX = 0x2008,
    PORT_CH_UART2_RX = 0x200a,	/* AT */
    PORT_CH_UART2_TX = 0x200c,
    PORT_CH_MD_LOG_RX = 0x202a,	/* MD logging */
    PORT_CH_MD_LOG_TX = 0x202b,
    PORT_CH_LB_IT_RX = 0x203e,	/* Loop back test */
    PORT_CH_LB_IT_TX = 0x203f,
    PORT_CH_STATUS_RX = 0x2043,	/* Status events */
    PORT_CH_MIPC_RX = 0x20ce,	/* MIPC */
    PORT_CH_MIPC_TX = 0x20cf,
    PORT_CH_MBIM_RX = 0x20d0,
    PORT_CH_MBIM_TX = 0x20d1,
    PORT_CH_DSS0_RX = 0x20d2,
    PORT_CH_DSS0_TX = 0x20d3,
    PORT_CH_DSS1_RX = 0x20d4,
    PORT_CH_DSS1_TX = 0x20d5,
    PORT_CH_DSS2_RX = 0x20d6,
    PORT_CH_DSS2_TX = 0x20d7,
    PORT_CH_DSS3_RX = 0x20d8,
    PORT_CH_DSS3_TX = 0x20d9,
    PORT_CH_DSS4_RX = 0x20da,
    PORT_CH_DSS4_TX = 0x20db,
    PORT_CH_DSS5_RX = 0x20dc,
    PORT_CH_DSS5_TX = 0x20dd,
    PORT_CH_DSS6_RX = 0x20de,
    PORT_CH_DSS6_TX = 0x20df,
    PORT_CH_DSS7_RX = 0x20e0,
    PORT_CH_DSS7_TX = 0x20e1,

    PORT_CH_UNIMPORTANT = 0xffff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct port_ops {
    pub port): *mut *mut int (init)(struct t7xx_port,
    pub skb): *mut *mut *mut int (recv_skb)(struct t7xx_port port, struct sk_buff,
    pub md_state): *mut *mut *mut void (md_state_notify)(struct t7xx_port port, unsigned int,
    pub port): *mut *mut void (uninit)(struct t7xx_port,
    pub port): *mut *mut int (enable_chl)(struct t7xx_port,
    pub port): *mut *mut int (disable_chl)(struct t7xx_port,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t7xx_port_conf {
    pub tx_ch: port_ch,
    pub rx_ch: port_ch,
    pub txq_index: c_uchar,
    pub rxq_index: c_uchar,
    pub txq_exp_index: c_uchar,
    pub rxq_exp_index: c_uchar,
    pub path_id: cldma_id,
    pub ops: *mut port_ops,
    pub name: *mut c_char,
    pub port_type: wwan_port_type,
    pub debug: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t7xx_port {
// Members not initialized in definition
    pub port_conf: *const t7xx_port_conf,
    pub t7xx_dev: *mut t7xx_pci_dev,
    pub dev: *mut device,
    pub /: *mut *mut u16 seq_nums[2]; / TX/RX sequence numbers,
    pub usage_cnt: core::sync::atomic::AtomicI32,
    pub entry: list_head,
    pub queue_entry: list_head,
// TX and RX flows are asymmetric since ports are multiplexed on
// queues.
//
// TX: data blocks are sent directly to a queue. Each port
// does not maintain a TX list; instead, they only provide
// a wait_queue_head for blocking writes.
//
// RX: Each port uses a RX list to hold packets,
// allowing the modem to dispatch RX packet as quickly as possible.
//
    pub rx_skb_list: sk_buff_head,
    pub /: *mut *mut spinlock_t port_update_lock; / Protects port configuration,
    pub rx_wq: wait_queue_head_t,
    pub rx_length_th: c_int,
    pub chan_enable: bool,
    pub thread: *mut task_struct,
    pub wwan_port: *mut wwan_port,
    pub wwan: },
    pub relaych: *mut rchan,
    pub log: },
}

extern "C" {
    pub fn t7xx_get_port_mtu(port: *mut t7xx_port) -> c_int;
}
extern "C" {
    pub fn t7xx_port_enqueue_skb(port: *mut t7xx_port, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn t7xx_port_send_raw_skb(port: *mut t7xx_port, skb: *mut sk_buff) -> c_int;
}
