//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/t7xx/t7xx_hif_cldma.h
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
// Sreehari Kancharla <sreehari.kancharla@intel.com>
//
// Contributors:
// Amir Hanania <amir.hanania@intel.com>
// Chiranjeevi Rapolu <chiranjeevi.rapolu@intel.com>
// Eliot Lee <eliot.lee@intel.com>
//

pub const CLDMA_SHARED_Q_BUFF_SZ: c_int = 3584;
pub const CLDMA_DEDICATED_Q_BUFF_SZ: c_int = 2048;
//
// enum cldma_id - Identifiers for CLDMA HW units.
// @CLDMA_ID_MD: Modem control channel.
// @CLDMA_ID_AP: Application Processor control channel.
// @CLDMA_NUM:   Number of CLDMA HW units available.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cldma_id {
    CLDMA_ID_MD,
    CLDMA_ID_AP,
    CLDMA_NUM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cldma_gpd {
    pub flags: u8,
    pub not_used1: u8,
    pub rx_data_allow_len: __le16,
    pub next_gpd_ptr_h: __le32,
    pub next_gpd_ptr_l: __le32,
    pub data_buff_bd_ptr_h: __le32,
    pub data_buff_bd_ptr_l: __le32,
    pub data_buff_len: __le16,
    pub not_used2: __le16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cldma_cfg {
    CLDMA_SHARED_Q_CFG,
    CLDMA_DEDICATED_Q_CFG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cldma_request {
    pub /: *mut *mut *mut cldma_gpd gpd; / Virtual address for CPU,
    pub /: *mut *mut dma_addr_t gpd_addr; / Physical address for DMA,
    pub skb: *mut sk_buff,
    pub mapped_buff: dma_addr_t,
    pub entry: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cldma_ring {
    pub /: *mut *mut list_head gpd_ring; / Ring of cldma_request,
    pub /: *mut *mut unsigned int length; / Number of struct cldma_request,
    pub pkt_size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cldma_queue {
    pub md_ctrl: *mut cldma_ctrl,
    pub dir: mtk_txrx,
    pub index: c_uint,
    pub tr_ring: *mut cldma_ring,
    pub tr_done: *mut cldma_request,
    pub rx_refill: *mut cldma_request,
    pub tx_next: *mut cldma_request,
    pub /: *mut *mut int budget; / Same as ring buffer size by default,
    pub ring_lock: spinlock_t,
    pub /: *mut *mut wait_queue_head_t req_wq; / Only for TX,
    pub worker: *mut workqueue_struct,
    pub cldma_work: work_struct,
    pub skb): *mut *mut *mut int (recv_skb)(struct cldma_queue queue, struct sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cldma_ctrl {
    pub hif_id: cldma_id,
    pub dev: *mut device,
    pub t7xx_dev: *mut t7xx_pci_dev,
    pub txq: [cldma_queue; CLDMA_TXQ_NUM],
    pub rxq: [cldma_queue; CLDMA_RXQ_NUM],
    pub txq_active: c_ushort,
    pub rxq_active: c_ushort,
    pub txq_started: c_ushort,
    pub /: *mut *mut spinlock_t cldma_lock; / Protects CLDMA structure,
// Assumes T/R GPD/BD/SPD have the same size
    pub gpd_dmapool: *mut dma_pool,
    pub tx_ring: [cldma_ring; CLDMA_TXQ_NUM],
    pub rx_ring: [cldma_ring; CLDMA_RXQ_NUM],
    pub pm_entity: *mut md_pm_entity,
    pub hw_info: t7xx_cldma_hw,
    pub is_late_init: bool,
}

pub const CLDMA_Q_IDX_DUMP: c_int = 1;

pub const GPD_DMAPOOL_ALIGN: c_int = 16;
extern "C" {
    pub fn t7xx_cldma_alloc(hif_id: cldma_id, t7xx_dev: *mut t7xx_pci_dev) -> c_int;
}
extern "C" {
    pub fn t7xx_cldma_hif_hw_init(md_ctrl: *mut cldma_ctrl);
}
extern "C" {
    pub fn t7xx_cldma_init(md_ctrl: *mut cldma_ctrl) -> c_int;
}
extern "C" {
    pub fn t7xx_cldma_exit(md_ctrl: *mut cldma_ctrl);
}
extern "C" {
    pub fn t7xx_cldma_switch_cfg(md_ctrl: *mut cldma_ctrl, cfg_id: cldma_cfg);
}
extern "C" {
    pub fn t7xx_cldma_start(md_ctrl: *mut cldma_ctrl);
}
extern "C" {
    pub fn t7xx_cldma_stop(md_ctrl: *mut cldma_ctrl) -> c_int;
}
extern "C" {
    pub fn t7xx_cldma_reset(md_ctrl: *mut cldma_ctrl);
}
extern "C" {
    pub fn t7xx_cldma_send_skb(md_ctrl: *mut cldma_ctrl, qno: c_int, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn t7xx_cldma_stop_all_qs(md_ctrl: *mut cldma_ctrl, tx_rx: mtk_txrx);
}
extern "C" {
    pub fn t7xx_cldma_clear_all_qs(md_ctrl: *mut cldma_ctrl, tx_rx: mtk_txrx);
}
