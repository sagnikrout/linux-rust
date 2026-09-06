//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/t7xx/t7xx_hif_dpmaif.h
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

// SKB control buffer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct t7xx_skb_cb {
    pub netif_idx: u8,
    pub txq_number: u8,
    pub rx_pkt_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpmaif_rdwr {
    DPMAIF_READ,
    DPMAIF_WRITE,
}

// Structure of DL BAT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmaif_cur_rx_skb_info {
    pub msg_pit_received: bool,
    pub cur_skb: *mut sk_buff,
    pub cur_chn_idx: c_uint,
    pub check_sum: c_uint,
    pub pit_dp: c_uint,
    pub pkt_type: c_uint,
    pub err_payload: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmaif_bat {
    pub p_buffer_addr: c_uint,
    pub buffer_addr_ext: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmaif_bat_skb {
    pub skb: *mut sk_buff,
    pub data_bus_addr: dma_addr_t,
    pub data_len: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmaif_bat_page {
    pub page: *mut page,
    pub data_bus_addr: dma_addr_t,
    pub offset: c_uint,
    pub data_len: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bat_type {
    BAT_TYPE_NORMAL,
    BAT_TYPE_FRAG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmaif_bat_request {
    pub bat_base: *mut c_void,
    pub bat_bus_addr: dma_addr_t,
    pub bat_size_cnt: c_uint,
    pub bat_wr_idx: c_uint,
    pub bat_release_rd_idx: c_uint,
    pub bat_skb: *mut c_void,
    pub pkt_buf_sz: c_uint,
    pub bat_bitmap: *mut c_ulong,
    pub refcnt: core::sync::atomic::AtomicI32,
    pub /: *mut *mut spinlock_t mask_lock; / Protects BAT mask,
    pub type: bat_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmaif_rx_queue {
    pub index: c_uint,
    pub que_started: bool,
    pub budget: c_uint,
    pub pit_base: *mut c_void,
    pub pit_bus_addr: dma_addr_t,
    pub pit_size_cnt: c_uint,
    pub pit_rd_idx: c_uint,
    pub pit_wr_idx: c_uint,
    pub pit_release_rd_idx: c_uint,
    pub bat_req: *mut dpmaif_bat_request,
    pub bat_frag: *mut dpmaif_bat_request,
    pub rx_processing: core::sync::atomic::AtomicI32,
    pub dpmaif_ctrl: *mut dpmaif_ctrl,
    pub expect_pit_seq: c_uint,
    pub pit_remain_release_cnt: c_uint,
    pub rx_data_info: dpmaif_cur_rx_skb_info,
    pub napi: napi_struct,
    pub sleep_lock_pending: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmaif_tx_queue {
    pub index: c_uint,
    pub que_started: bool,
    pub tx_budget: core::sync::atomic::AtomicI32,
    pub drb_base: *mut c_void,
    pub drb_bus_addr: dma_addr_t,
    pub drb_size_cnt: c_uint,
    pub drb_wr_idx: c_uint,
    pub drb_rd_idx: c_uint,
    pub drb_release_rd_idx: c_uint,
    pub drb_skb_base: *mut c_void,
    pub req_wq: wait_queue_head_t,
    pub worker: *mut workqueue_struct,
    pub dpmaif_tx_work: work_struct,
    pub /: *mut *mut spinlock_t tx_lock; / Protects txq DRB,
    pub tx_processing: core::sync::atomic::AtomicI32,
    pub dpmaif_ctrl: *mut dpmaif_ctrl,
    pub tx_skb_head: sk_buff_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmaif_isr_para {
    pub dpmaif_ctrl: *mut dpmaif_ctrl,
    pub pcie_int: c_uchar,
    pub dlq_id: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpmaif_state {
    DPMAIF_STATE_MIN,
    DPMAIF_STATE_PWROFF,
    DPMAIF_STATE_PWRON,
    DPMAIF_STATE_EXCEPTION,
    DPMAIF_STATE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpmaif_txq_state {
    DMPAIF_TXQ_STATE_IRQ,
    DMPAIF_TXQ_STATE_FULL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmaif_callbacks {
    pub txq_number): dpmaif_txq_state state, int,
    pub napi): *mut napi_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmaif_ctrl {
    pub dev: *mut device,
    pub t7xx_dev: *mut t7xx_pci_dev,
    pub dpmaif_pm_entity: md_pm_entity,
    pub state: dpmaif_state,
    pub dpmaif_sw_init_done: bool,
    pub hw_info: dpmaif_hw_info,
    pub txq: [dpmaif_tx_queue; DPMAIF_TXQ_NUM],
    pub rxq: [dpmaif_rx_queue; DPMAIF_RXQ_NUM],
    pub rxq_int_mapping: [c_uchar; DPMAIF_RXQ_NUM],
    pub isr_para: [dpmaif_isr_para; DPMAIF_RXQ_NUM],
    pub bat_req: dpmaif_bat_request,
    pub bat_frag: dpmaif_bat_request,
    pub bat_release_wq: *mut workqueue_struct,
    pub bat_release_work: work_struct,
    pub tx_wq: wait_queue_head_t,
    pub tx_thread: *mut task_struct,
    pub callbacks: *mut dpmaif_callbacks,
}

extern "C" {
    pub fn t7xx_dpmaif_hif_exit(dpmaif_ctrl: *mut dpmaif_ctrl);
}
extern "C" {
    pub fn t7xx_dpmaif_md_state_callback(dpmaif_ctrl: *mut dpmaif_ctrl, state: md_state) -> c_int;
}
extern "C" {
    pub fn t7xx_ring_buf_get_next_wr_idx(buf_len: c_uint, buf_idx: c_uint) -> c_uint;
}
