//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8192cu/trx.h
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
// Copyright(c) 2009-2012  Realtek Corporation.
pub const RTL92C_NUM_RX_URBS: c_int = 8;
pub const RTL92C_NUM_TX_URBS: c_int = 32;

pub const RX_DRV_INFO_SIZE_UNIT: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_rx_agg_mode {
    USB_RX_AGG_DISABLE,
    USB_RX_AGG_DMA,
    USB_RX_AGG_USB,
    USB_RX_AGG_DMA_USB
}

pub const RTL_USB_TX_AGG_NUM_DESC: c_int = 5;
pub const RTL_USB_RX_AGG_PAGE_NUM: c_int = 4;
pub const RTL_USB_RX_AGG_PAGE_TIMEOUT: c_int = 3;
pub const RTL_USB_RX_AGG_BLOCK_NUM: c_int = 5;
pub const RTL_USB_RX_AGG_BLOCK_TIMEOUT: c_int = 3;
// ======================== rx status =========================================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_drv_info_92c {
//
// Driver info contain PHY status and other variabel size info
// PHY Status content as below
//
// DWORD 0
    pub gain_trsw: [u8; 4],
// DWORD 1
    pub pwdb_all: u8,
    pub cfosho: [u8; 4],
// DWORD 2
    pub cfotail: [u8; 4],
// DWORD 3
    pub rxevm: [i8; 2],
    pub rxsnr: [i8; 4],
// DWORD 4
    pub pdsnr: [u8; 2],
// DWORD 5
    pub csi_current: [u8; 2],
    pub csi_target: [u8; 2],
// DWORD 6
    pub sigevm: u8,
    pub max_ex_pwr: u8,
    pub ex_intf_flag:1: u8,
    pub sgi_en:1: u8,
    pub rxsc:2: u8,
    pub reserve:4: u8,
    pub __packed: },
// macros to read various fields in RX descriptor
// DWORD 0
    pub 0)): *mut *mut return le32_get_bits(__rxdesc, GENMASK(13,,
    pub BIT(14)): *mut *mut return le32_get_bits(__rxdesc,,
    pub BIT(15)): *mut *mut return le32_get_bits(__rxdesc,,
    pub 16)): *mut *mut return le32_get_bits(__rxdesc, GENMASK(19,,
    pub 24)): *mut *mut return le32_get_bits(__rxdesc, GENMASK(25,,
    pub BIT(26)): *mut *mut return le32_get_bits(__rxdesc,,
    pub BIT(27)): *mut *mut return le32_get_bits(__rxdesc,,
// DWORD 1
    pub BIT(14)): *mut *mut return le32_get_bits((__rxdesc + 1),,
    pub BIT(15)): *mut *mut return le32_get_bits((__rxdesc + 1),,
// DWORD 3
    pub 0)): *mut *mut return le32_get_bits((__rxdesc + 3), GENMASK(5,,
    pub BIT(6)): *mut *mut return le32_get_bits((__rxdesc + 3),,
    pub BIT(8)): *mut *mut return le32_get_bits((__rxdesc + 3),,
    pub BIT(9)): *mut *mut return le32_get_bits((__rxdesc + 3),,
// DWORD 5
    pub 5))): *mut *mut return le32_to_cpu(((__rxdesc +,
// ======================= tx desc ============================================
// macros to set various fields in TX descriptor
// Dword 0
    pub 0)): le32p_replace_bits(__txdesc, __value, GENMASK(15,,
    pub 16)): le32p_replace_bits(__txdesc, __value, GENMASK(23,,
    pub BIT(24)): le32p_replace_bits(__txdesc, __value,,
    pub BIT(25)): le32p_replace_bits(__txdesc, __value,,
    pub BIT(26)): le32p_replace_bits(__txdesc, __value,,
    pub BIT(27)): le32p_replace_bits(__txdesc, __value,,
    pub BIT(28)): le32p_replace_bits(__txdesc, __value,,
    pub BIT(31)): le32p_replace_bits(__txdesc, __value,,
// Dword 1
    pub 0)): le32p_replace_bits((__txdesc + 1), __value, GENMASK(4,,
    pub BIT(5)): le32p_replace_bits((__txdesc + 1), __value,,
    pub BIT(6)): le32p_replace_bits((__txdesc + 1), __value,,
    pub BIT(7)): le32p_replace_bits((__txdesc + 1), __value,,
    pub 8)): le32p_replace_bits((__txdesc + 1), __value, GENMASK(12,,
    pub 16)): le32p_replace_bits((__txdesc + 1), __value, GENMASK(19,,
    pub BIT(20)): le32p_replace_bits((__txdesc + 1), __value,,
    pub 22)): le32p_replace_bits((__txdesc + 1), __value, GENMASK(23,,
    pub 26)): le32p_replace_bits((__txdesc + 1), __value, GENMASK(30,,
// Dword 2
    pub BIT(17)): le32p_replace_bits((__txdesc + 2), __value,,
    pub 20)): le32p_replace_bits((__txdesc + 2), __value, GENMASK(22,,
// Dword 3
    pub 16)): le32p_replace_bits((__txdesc + 3), __value, GENMASK(27,,
    pub 28)): le32p_replace_bits((__txdesc + 3), __value, GENMASK(31,,
// Dword 4
    pub 0)): le32p_replace_bits((__txdesc + 4), __value, GENMASK(4,,
    pub BIT(6)): le32p_replace_bits((__txdesc + 4), __value,,
    pub BIT(7)): le32p_replace_bits((__txdesc + 4), __value,,
    pub BIT(8)): le32p_replace_bits((__txdesc + 4), __value,,
    pub BIT(10)): le32p_replace_bits((__txdesc + 4), __value,,
    pub BIT(11)): le32p_replace_bits((__txdesc + 4), __value,,
    pub BIT(12)): le32p_replace_bits((__txdesc + 4), __value,,
    pub BIT(13)): le32p_replace_bits((__txdesc + 4), __value,,
    pub 20)): le32p_replace_bits((__txdesc + 4), __value, GENMASK(21,,
    pub BIT(25)): le32p_replace_bits((__txdesc + 4), __value,,
    pub BIT(26)): le32p_replace_bits((__txdesc + 4), __value,,
    pub BIT(27)): le32p_replace_bits((__txdesc + 4), __value,,
    pub 28)): le32p_replace_bits((__txdesc + 4), __value, GENMASK(29,,
    pub 30)): le32p_replace_bits((__txdesc + 4), __value, GENMASK(31,,
// Dword 5
    pub 0)): le32p_replace_bits((__pdesc + 5), __val, GENMASK(5,,
    pub BIT(6)): le32p_replace_bits((__pdesc + 5), __val,,
    pub 8)): le32p_replace_bits((__txdesc + 5), __value, GENMASK(12,,
    pub 13)): le32p_replace_bits((__txdesc + 5), __value, GENMASK(16,,
// Dword 6
    pub 11)): le32p_replace_bits((__txdesc + 6), __value, GENMASK(15,,
// Dword 7
    pub 0)): le32p_replace_bits((__txdesc + 7), __value, GENMASK(15,,
    pub hw): *mut int rtl8192cu_endpoint_mapping(struct ieee80211_hw,
    pub mac80211_queue_index): u16 rtl8192cu_mq_to_hwq(__le16 fc, u16,
    pub skb): *mut *mut u8 p_desc, struct sk_buff,
    pub skb): *mut *mut *mut void rtl8192cu_rx_hdl(struct ieee80211_hw hw, struct sk_buff,
    pub skb): *mut *mut void rtl8192c_tx_cleanup(struct ieee80211_hw hw, struct sk_buff,
    pub skb): *mut sk_buff,
    pub ): *mut sk_buff_head,
    pub tcb_desc): *mut rtl_tcb_desc,
    pub skb): *mut sk_buff,
