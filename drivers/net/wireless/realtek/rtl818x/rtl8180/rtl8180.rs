//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtl818x/rtl8180/rtl8180.h
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

pub const BB_ANTATTEN_CHAN14: c_uint = 0x0C;
pub const BB_ANTENNA_B: c_uint = 0x40;

pub const BB_HOST_BANG_DATA: c_int = 1;
pub const ANAPARAM_TXDACOFF_SHIFT: c_int = 27;
pub const ANAPARAM_PWR0_SHIFT: c_int = 28;

pub const ANAPARAM_PWR1_SHIFT: c_int = 20;

// rtl8180/rtl8185 have 3 queue + beacon queue.
// mac80211 can use just one, + beacon = 2 tot.
//
pub const RTL8180_NR_TX_QUEUES: c_int = 2;
// rtl8187SE have 6 queues + beacon queues
// mac80211 can use 4 QoS data queue, + beacon = 5 tot
//
pub const RTL8187SE_NR_TX_QUEUES: c_int = 5;
// for array static allocation, it is the max of above
pub const RTL818X_NR_TX_QUEUES: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8180_tx_desc {
    pub flags: __le32,
    pub rts_duration: __le16,
    pub plcp_len: __le16,
    pub tx_buf: __le32,
    pub frame_len: __le32,
    pub frame_len_se: __le16,
    pub frame_duration: __le16,
    pub __packed: },
    pub __packed: },
    pub next_tx_desc: __le32,
    pub cw: u8,
    pub retry_limit: u8,
    pub agc: u8,
    pub flags2: u8,
// rsvd for 8180/8185.
// valid for 8187se but we dont use it
//
    pub reserved: u32,
// all rsvd for 8180/8185
    pub flags3: __le16,
    pub frag_qsize: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl818x_rx_cmd_desc {
    pub flags: __le32,
    pub reserved: u32,
    pub rx_buf: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8180_rx_desc {
    pub flags: __le32,
    pub flags2: __le32,
    pub tsft: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8187se_rx_desc {
    pub flags: __le32,
    pub tsft: __le64,
    pub flags2: __le32,
    pub flags3: __le32,
    pub reserved: [u32; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8180_tx_ring {
    pub desc: *mut rtl8180_tx_desc,
    pub dma: dma_addr_t,
    pub idx: c_uint,
    pub entries: c_uint,
    pub queue: sk_buff_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8180_vif {
    pub dev: *mut ieee80211_hw,
// beaconing
    pub beacon_work: delayed_work,
    pub enable_beacon: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8180_priv {
// common between rtl818x drivers
    pub map: *mut rtl818x_csr __iomem,
    pub rf: *const rtl818x_rf_ops,
    pub vif: *mut ieee80211_vif,
// rtl8180 driver specific
    pub map_pio: bool,
    pub lock: spinlock_t,
    pub rx_ring: *mut c_void,
    pub rx_ring_sz: u8,
    pub rx_ring_dma: dma_addr_t,
    pub rx_idx: c_uint,
    pub rx_buf: [*mut sk_buff; 32],
    pub tx_ring: [rtl8180_tx_ring; RTL818X_NR_TX_QUEUES],
    pub channels: [ieee80211_channel; 14],
    pub rates: [ieee80211_rate; 12],
    pub band: ieee80211_supported_band,
    pub queue_param: [ieee80211_tx_queue_params; 4],
    pub pdev: *mut pci_dev,
    pub rx_conf: u32,
    pub slot_time: u8,
    pub ack_time: u16,
    pub chip_family: },
    pub anaparam: u32,
    pub rfparam: u16,
    pub csthreshold: u8,
    pub mac_addr: [u8; ETH_ALEN],
    pub rf_type: u8,
    pub xtal_out: u8,
    pub xtal_in: u8,
    pub xtal_cal: u8,
    pub thermal_meter_val: u8,
    pub thermal_meter_en: u8,
    pub antenna_diversity_en: u8,
    pub antenna_diversity_default: u8,
// sequence #
    pub seqno: u16,
}

extern "C" {
    pub fn rtl8180_write_phy(dev: *mut ieee80211_hw, addr: u8, data: u32);
}
extern "C" {
    pub fn rtl8180_set_anaparam(priv: *mut rtl8180_priv, anaparam: u32);
}
extern "C" {
    pub fn rtl8180_set_anaparam2(priv: *mut rtl8180_priv, anaparam2: u32);
}
extern "C" {
    pub fn ioread8(_arg: addr) -> return;
}
extern "C" {
    pub fn ioread16(_arg: addr) -> return;
}
extern "C" {
    pub fn ioread32(_arg: addr) -> return;
}
