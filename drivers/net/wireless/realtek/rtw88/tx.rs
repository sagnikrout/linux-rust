//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw88/tx.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright(c) 2018-2019  Realtek Corporation
//
pub const RTK_TX_MAX_AGG_NUM_MASK: c_uint = 0x1f;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_tx_desc {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub w3: __le32,
    pub w4: __le32,
    pub w5: __le32,
    pub w6: __le32,
    pub w7: __le32,
    pub w8: __le32,
    pub w9: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_tx_desc_queue_select {
    TX_DESC_QSEL_TID0	= 0,
    TX_DESC_QSEL_TID1	= 1,
    TX_DESC_QSEL_TID2	= 2,
    TX_DESC_QSEL_TID3	= 3,
    TX_DESC_QSEL_TID4	= 4,
    TX_DESC_QSEL_TID5	= 5,
    TX_DESC_QSEL_TID6	= 6,
    TX_DESC_QSEL_TID7	= 7,
    TX_DESC_QSEL_TID8	= 8,
    TX_DESC_QSEL_TID9	= 9,
    TX_DESC_QSEL_TID10	= 10,
    TX_DESC_QSEL_TID11	= 11,
    TX_DESC_QSEL_TID12	= 12,
    TX_DESC_QSEL_TID13	= 13,
    TX_DESC_QSEL_TID14	= 14,
    TX_DESC_QSEL_TID15	= 15,
    TX_DESC_QSEL_BEACON	= 16,
    TX_DESC_QSEL_HIGH	= 17,
    TX_DESC_QSEL_MGMT	= 18,
    TX_DESC_QSEL_H2C	= 19,
}

    pub rtw_rsvd_packet_type: enum,
    pub skb): *mut sk_buff,
    pub txq): *mut *mut void rtw_txq_init(struct rtw_dev rtwdev, struct ieee80211_txq,
    pub txq): *mut *mut void rtw_txq_cleanup(struct rtw_dev rtwdev, struct ieee80211_txq,
    pub w): *mut void rtw_tx_work(struct work_struct,
    pub rtwdev): *mut void __rtw_tx_work(struct rtw_dev,
    pub skb): *mut sk_buff,
    pub skb): *mut *mut rtw_tx_pkt_info pkt_info, sk_buff,
    pub sn): *mut *mut *mut void rtw_tx_report_enqueue(struct rtw_dev rtwdev, struct sk_buff skb, u8,
    pub src): *mut *mut *mut void rtw_tx_report_handle(struct rtw_dev rtwdev, struct sk_buff skb, int,
    pub type): rtw_rsvd_packet_type,
    pub size): *mut *mut u8 buf, u32,
    pub size): *mut *mut u8 buf, u32,
    pub ac): rtw_tx_queue_type rtw_tx_ac_to_hwq(ieee80211_ac_numbers,
    pub skb): *mut rtw_tx_queue_type rtw_tx_queue_mapping(struct sk_buff,
    pub 0: __le16 chksum =,
    pub )(txdesc): *mut *mut __le16 data = (__le16,
    pub )txdesc: *mut *mut rtw_tx_desc tx_desc = (rtw_tx_desc,
    pub RTW_TX_DESC_W7_TXDESC_CHECKSUM): le32p_replace_bits(&tx_desc->w7, 0,,
    pub data++: *mut chksum ^=,
    pub rtwdev->chip: *const *const rtw_chip_info chip =,
    pub txdesc): chip->ops->fill_txdesc_checksum(rtwdev, pkt_info,,
