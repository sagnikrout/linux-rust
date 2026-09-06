//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/ti/knav_dma.h
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
// Copyright (C) 2014 Texas Instruments Incorporated
// Authors:	Sandeep Nair <sandeep_n@ti.com
// Cyril Chemparathy <cyril@ti.com
//

//
// PKTDMA descriptor manipulation macros for host packet descriptor
//

pub const KNAV_DMA_DESC_PKT_LEN_SHIFT: c_int = 0;

pub const KNAV_DMA_DESC_PS_INFO_IN_DESC: c_int = 0;

pub const KNAV_DMA_DESC_SAG_HI_SHIFT: c_int = 24;
pub const KNAV_DMA_DESC_STAG_LO_SHIFT: c_int = 16;
pub const KNAV_DMA_DESC_DTAG_HI_SHIFT: c_int = 8;
pub const KNAV_DMA_DESC_DTAG_LO_SHIFT: c_int = 0;

pub const KNAV_DMA_DESC_NO_EPIB: c_int = 0;
pub const KNAV_DMA_DESC_PSLEN_SHIFT: c_int = 24;

pub const KNAV_DMA_DESC_ERR_FLAG_SHIFT: c_int = 20;

pub const KNAV_DMA_DESC_PSFLAG_SHIFT: c_int = 16;

pub const KNAV_DMA_DESC_RETQ_SHIFT: c_int = 0;

pub const KNAV_DMA_DESC_EFLAGS_SHIFT: c_int = 20;
pub const KNAV_DMA_NUM_EPIB_WORDS: c_int = 4;
pub const KNAV_DMA_NUM_PS_WORDS: c_int = 16;
pub const KNAV_DMA_NUM_SW_DATA_WORDS: c_int = 4;
pub const KNAV_DMA_FDQ_PER_CHAN: c_int = 4;
// Tx channel scheduling priority
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum knav_dma_tx_priority {
    DMA_PRIO_HIGH	= 0,
    DMA_PRIO_MED_H,
    DMA_PRIO_MED_L,
    DMA_PRIO_LOW
}

// Rx channel error handling mode during buffer starvation
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum knav_dma_rx_err_mode {
    DMA_DROP = 0,
    DMA_RETRY
}

// Rx flow size threshold configuration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum knav_dma_rx_thresholds {
    DMA_THRESH_NONE		= 0,
    DMA_THRESH_0		= 1,
    DMA_THRESH_0_1		= 3,
    DMA_THRESH_0_1_2	= 7
}

// Descriptor type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum knav_dma_desc_type {
    DMA_DESC_HOST = 0,
    DMA_DESC_MONOLITHIC = 2
}

//
// struct knav_dma_tx_cfg:	Tx channel configuration
// @filt_einfo:			Filter extended packet info
// @filt_pswords:		Filter PS words present
// @priority:			Tx channel scheduling priority
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_dma_tx_cfg {
    pub filt_einfo: bool,
    pub filt_pswords: bool,
    pub priority: knav_dma_tx_priority,
}

//
// struct knav_dma_rx_cfg:	Rx flow configuration
// @einfo_present:		Extended packet info present
// @psinfo_present:		PS words present
// @err_mode:			Error during buffer starvation
// @desc_type:			Host or Monolithic desc
// @psinfo_at_sop:		PS word located at start of packet
// @sop_offset:			Start of packet offset
// @dst_q:			Destination queue for a given flow
// @thresh:			Rx flow size threshold
// @fdq:			Free desc Queue array
// @sz_thresh0:			RX packet size threshold 0
// @sz_thresh1:			RX packet size threshold 1
// @sz_thresh2:			RX packet size threshold 2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_dma_rx_cfg {
    pub einfo_present: bool,
    pub psinfo_present: bool,
    pub err_mode: knav_dma_rx_err_mode,
    pub desc_type: knav_dma_desc_type,
    pub psinfo_at_sop: bool,
    pub sop_offset: c_uint,
    pub dst_q: c_uint,
    pub thresh: knav_dma_rx_thresholds,
    pub fdq: [c_uint; KNAV_DMA_FDQ_PER_CHAN],
    pub sz_thresh0: c_uint,
    pub sz_thresh1: c_uint,
    pub sz_thresh2: c_uint,
}

//
// struct knav_dma_cfg:	Pktdma channel configuration
// @direction:			DMA transfer mode and direction
// @u:				union containing @tx or @rx
// @tx:				Tx channel configuration
// @rx:				Rx flow configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_dma_cfg {
    pub direction: dma_transfer_direction,
    pub tx: knav_dma_tx_cfg,
    pub rx: knav_dma_rx_cfg,
    pub u: },
}

//
// struct knav_dma_desc:	Host packet descriptor layout
// @desc_info:			Descriptor information like id, type, length
// @tag_info:			Flow tag info written in during RX
// @packet_info:		Queue Manager, policy, flags etc
// @buff_len:			Buffer length in bytes
// @buff:			Buffer pointer
// @next_desc:			For chaining the descriptors
// @orig_len:			length since 'buff_len' can be overwritten
// @orig_buff:			buff pointer since 'buff' can be overwritten
// @epib:			Extended packet info block
// @psdata:			Protocol specific
// @sw_data:			Software private data not touched by h/w
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_dma_desc {
    pub desc_info: __le32,
    pub tag_info: __le32,
    pub packet_info: __le32,
    pub buff_len: __le32,
    pub buff: __le32,
    pub next_desc: __le32,
    pub orig_len: __le32,
    pub orig_buff: __le32,
    pub epib: [__le32; KNAV_DMA_NUM_EPIB_WORDS],
    pub psdata: [__le32; KNAV_DMA_NUM_PS_WORDS],
    pub sw_data: [u32; KNAV_DMA_NUM_SW_DATA_WORDS],
    pub ____cacheline_aligned: },

    pub config): *mut knav_dma_cfg,
    pub channel): *mut void knav_dma_close_channel(void,
    pub channel): *mut int knav_dma_get_flow(void,
    pub knav_dma_device_ready(void): bool,

    pub NULL: *mut *mut return (void ),
    pub -EINVAL: return,
    pub false: return,

