//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7603/mac.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rx_pkt_type {
    PKT_TYPE_TXS		= 0,
    PKT_TYPE_TXRXV		= 1,
    PKT_TYPE_NORMAL		= 2,
    PKT_TYPE_RX_DUP_RFB	= 3,
    PKT_TYPE_RX_TMR		= 4,
    PKT_TYPE_RETRIEVE	= 5,
    PKT_TYPE_RX_EVENT	= 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7603_tx_header_format {
    MT_HDR_FORMAT_802_3,
    MT_HDR_FORMAT_CMD,
    MT_HDR_FORMAT_802_11,
    MT_HDR_FORMAT_802_11_EXT,
}

