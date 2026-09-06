//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt7601u/dma.h
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
// Copyright (C) 2014 Felix Fietkau <nbd@openwrt.org>
// Copyright (C) 2015 Jakub Kicinski <kubakici@wp.pl>
//

pub const MT_DMA_HDR_LEN: c_int = 4;
pub const MT_RX_INFO_LEN: c_int = 4;
pub const MT_FCE_INFO_LEN: c_int = 4;

// Common Tx DMA descriptor fields

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76_msg_port {
    WLAN_PORT,
    CPU_RX_PORT,
    CPU_TX_PORT,
    HOST_PORT,
    VIRTUAL_CPU_RX_PORT,
    VIRTUAL_CPU_TX_PORT,
    DISCARD,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76_info_type {
    DMA_PACKET,
    DMA_COMMAND,
}

// Tx DMA packet specific flags

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76_qsel {
    MT_QSEL_MGMT,
    MT_QSEL_HCCA,
    MT_QSEL_EDCA,
    MT_QSEL_EDCA_2,
}

// Tx DMA MCU command specific flags

// Buffer layout:
// |   4B   | xfer len |      pad       |  4B  |
// | TXINFO | pkt/cmd  | zero pad to 4B | zero |
//
// length field of TXINFO should be set to 'xfer len'.
//
extern "C" {
    pub fn skb_put_padto(_arg: skb, _arg: round_up(skb->len, 4: 4) +) -> return;
}
extern "C" {
    pub fn mt7601u_dma_skb_wrap(_arg: skb, _arg: WLAN_PORT, _arg: DMA_PACKET, _arg: flags) -> return;
}
// Common Rx DMA descriptor fields

// Rx DMA packet specific flags

// Rx DMA MCU command specific flags

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76_evt_type {
    CMD_DONE,
    CMD_ERROR,
    CMD_RETRY,
    EVENT_PWR_RSP,
    EVENT_WOW_RSP,
    EVENT_CARRIER_DETECT_RSP,
    EVENT_DFS_DETECT_RSP,
}
