//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/dma.h
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
//
// Copyright (C) 2016 Felix Fietkau <nbd@nbd.name>
//

pub const MT_RING_SIZE: c_uint = 0x10;

pub const MT_DMA_MAGIC_CNT: c_int = 16;
pub const MT_DMA_WED_IND_CMD_CNT: c_int = 8;

pub const MT_DMA_HDR_LEN: c_int = 4;
pub const MT_RX_INFO_LEN: c_int = 4;
pub const MT_FCE_INFO_LEN: c_int = 4;
pub const MT_RX_RXWI_LEN: c_int = 32;

// val = mtk_wed_device_reg_read(q->wed, q->wed_regs + offset);

// val = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_desc {
    pub buf0: __le32,
    pub ctrl: __le32,
    pub buf1: __le32,
    pub info: __le32,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_wed_rro_desc {
    pub buf0: __le32,
    pub buf1: __le32,
    pub __aligned(4): } __packed,
// data1

// data2

// data3

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_rro_rxdmad_c {
    pub data0: __le32,
    pub data1: __le32,
    pub data2: __le32,
    pub data3: __le32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76_qsel {
    MT_QSEL_MGMT,
    MT_QSEL_HCCA,
    MT_QSEL_EDCA,
    MT_QSEL_EDCA_2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76_mcu_evt_type {
    EVT_CMD_DONE,
    EVT_CMD_ERROR,
    EVT_CMD_RETRY,
    EVT_EVENT_PWR_RSP,
    EVT_EVENT_WOW_RSP,
    EVT_EVENT_CARRIER_DETECT_RSP,
    EVT_EVENT_DFS_DETECT_RSP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76_dma_wed_ind_reason {
    MT_DMA_WED_IND_REASON_NORMAL,
    MT_DMA_WED_IND_REASON_REPEAT,
    MT_DMA_WED_IND_REASON_OLDPKT,
}

extern "C" {
    pub fn mt76_dma_rx_poll(napi: *mut napi_struct, budget: c_int) -> c_int;
}
extern "C" {
    pub fn mt76_dma_attach(dev: *mut mt76_dev);
}
extern "C" {
    pub fn mt76_dma_cleanup(dev: *mut mt76_dev);
}
// drop = !!(ctrl & (MT_DMA_CTL_TO_HOST_A | MT_DMA_CTL_DROP));
// drop = true;
// drop = !(info & MT_DMA_INFO_DMA_FRAG);
// drop = !!(ctrl & MT_DMA_CTL_PN_CHK_FAIL);
