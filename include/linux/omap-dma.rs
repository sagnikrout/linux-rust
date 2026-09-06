//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/omap-dma.h
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
//
// Legacy OMAP DMA handling defines and functions
//
// NOTE: Do not use these any longer.
//
// Use the generic dmaengine functions as defined in
// include/linux/dmaengine.h.
//
// Copyright (C) 2003 Nokia Corporation
// Author: Juha Yrjölä <juha.yrjola@nokia.com>
//

pub const OMAP_DMA_DATA_TYPE_S8: c_uint = 0x00;
pub const OMAP_DMA_DATA_TYPE_S16: c_uint = 0x01;
pub const OMAP_DMA_DATA_TYPE_S32: c_uint = 0x02;
pub const OMAP_DMA_SYNC_ELEMENT: c_uint = 0x00;
pub const OMAP_DMA_SYNC_FRAME: c_uint = 0x01;
pub const OMAP_DMA_SYNC_BLOCK: c_uint = 0x02;
pub const OMAP_DMA_SYNC_PACKET: c_uint = 0x03;
pub const OMAP_DMA_DST_SYNC_PREFETCH: c_uint = 0x02;
pub const OMAP_DMA_SRC_SYNC: c_uint = 0x01;
pub const OMAP_DMA_DST_SYNC: c_uint = 0x00;
pub const OMAP_DMA_PORT_EMIFF: c_uint = 0x00;
pub const OMAP_DMA_PORT_EMIFS: c_uint = 0x01;
pub const OMAP_DMA_PORT_OCP_T1: c_uint = 0x02;
pub const OMAP_DMA_PORT_TIPB: c_uint = 0x03;
pub const OMAP_DMA_PORT_OCP_T2: c_uint = 0x04;
pub const OMAP_DMA_PORT_MPUI: c_uint = 0x05;
pub const OMAP_DMA_AMODE_CONSTANT: c_uint = 0x00;
pub const OMAP_DMA_AMODE_POST_INC: c_uint = 0x01;
pub const OMAP_DMA_AMODE_SINGLE_IDX: c_uint = 0x02;
pub const OMAP_DMA_AMODE_DOUBLE_IDX: c_uint = 0x03;
pub const DMA_DEFAULT_FIFO_DEPTH: c_uint = 0x10;
pub const DMA_DEFAULT_ARB_RATE: c_uint = 0x01;
// Pass THREAD_RESERVE ORed with THREAD_FIFO for tparams

// DMA4_OCP_SYSCONFIG bits

pub const DMA_IDLEMODE_SMARTIDLE: c_uint = 0x2;
pub const DMA_IDLEMODE_NO_IDLE: c_uint = 0x1;
pub const DMA_IDLEMODE_FORCE_IDLE: c_uint = 0x0;
// Chaining modes

pub const OMAP_DMA_STATIC_CHAIN: c_uint = 0x1;
pub const OMAP_DMA_DYNAMIC_CHAIN: c_uint = 0x2;
pub const OMAP_DMA_CHAIN_ACTIVE: c_uint = 0x1;
pub const OMAP_DMA_CHAIN_INACTIVE: c_uint = 0x0;

pub const DMA_CH_PRIO_HIGH: c_uint = 0x1;
pub const DMA_CH_PRIO_LOW: c_uint = 0x0 /* Def */;
// Errata handling

// Attributes for OMAP DMA Contrller

// Defines for DMA Capabilities

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_reg_offsets {

    GCR,		GSCR,		GRST1,		HW_ID,
    PCH2_ID,	PCH0_ID,	PCH1_ID,	PCHG_ID,
    PCHD_ID,	CAPS_0,		CAPS_1,		CAPS_2,
    CAPS_3,		CAPS_4,		PCH2_SR,	PCH0_SR,
    PCH1_SR,	PCHD_SR,	REVISION,	IRQSTATUS_L0,
    IRQSTATUS_L1,	IRQSTATUS_L2,	IRQSTATUS_L3,	IRQENABLE_L0,
    IRQENABLE_L1,	IRQENABLE_L2,	IRQENABLE_L3,	SYSSTATUS,
    OCP_SYSCONFIG,

// omap1+ specific
    CPC, CCR2, LCH_CTRL,

// Common registers for all omap's
    CSDP,		CCR,		CICR,		CSR,
    CEN,		CFN,		CSFI,		CSEI,
    CSAC,		CDAC,		CDEI,
    CDFI,		CLNK_CTRL,

// Channel specific registers
    CSSA,		CDSA,		COLOR,
    CCEN,		CCFN,

// omap3630 and omap4 specific
    CDP,		CNDP,		CCDN,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dma_burst_mode {
    OMAP_DMA_DATA_BURST_DIS = 0,
    OMAP_DMA_DATA_BURST_4,
    OMAP_DMA_DATA_BURST_8,
    OMAP_DMA_DATA_BURST_16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum end_type {
    OMAP_DMA_LITTLE_ENDIAN = 0,
    OMAP_DMA_BIG_ENDIAN
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dma_color_mode {
    OMAP_DMA_COLOR_DIS = 0,
    OMAP_DMA_CONSTANT_FILL,
    OMAP_DMA_TRANSPARENT_COPY
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dma_write_mode {
    OMAP_DMA_WRITE_NON_POSTED = 0,
    OMAP_DMA_WRITE_POSTED,
    OMAP_DMA_WRITE_LAST_NON_POSTED
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dma_channel_mode {
    OMAP_DMA_LCH_2D = 0,
    OMAP_DMA_LCH_G,
    OMAP_DMA_LCH_P,
    OMAP_DMA_LCH_PD
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_dma_channel_params {
    pub /: *mut *mut int data_type; / data type 8,16,32,
    pub /: *mut *mut int elem_count; / number of elements in a frame,
    pub /: *mut *mut int frame_count; / number of frames in a element,
    pub /: *mut *mut int src_port; / Only on OMAP1 REVISIT: Is this needed?,
    pub indexed,: *mut *mut int src_amode; / constant, post increment,,
    pub /: *mut *mut unsigned long src_start; / source address : physical,
    pub /: *mut *mut int src_ei; / source element index,
    pub /: *mut *mut int src_fi; / source frame index,
    pub /: *mut *mut int dst_port; / Only on OMAP1 REVISIT: Is this needed?,
    pub indexed,: *mut *mut int dst_amode; / constant, post increment,,
    pub /: *mut *mut unsigned long dst_start; / source address : physical,
    pub /: *mut *mut int dst_ei; / source element index,
    pub /: *mut *mut int dst_fi; / source frame index,
    pub is: *mut *mut int trigger; / trigger attached if the channel,
    pub /: *mut *mut int sync_mode; / sycn on element, frame , block or packet,
    pub /: *mut *mut int src_or_dst_synch; / source synch(1) or destination synch(0),
    pub /: *mut *mut int ie; / interrupt enabled,
    pub /: *mut *mut unsigned char read_prio;/ read priority,
    pub /: *mut *mut unsigned char write_prio;/ write priority,

    pub /: *mut *mut omap_dma_burst_mode burst_mode; / Burst mode 4/8/16 words,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_dma_lch {
    pub next_lch: c_int,
    pub dev_id: c_int,
    pub saved_csr: u16,
    pub enabled_irqs: u16,
    pub dev_name: *const c_char,
    pub data): *mut *mut void (callback)(int lch, u16 ch_status, void,
    pub data: *mut c_void,
    pub flags: c_long,
    pub state: c_int,
    pub chain_id: c_int,
    pub status: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_dma_dev_attr {
    pub dev_caps: u32,
    pub lch_count: u16,
    pub chan_count: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_dma_reg {
    pub offset: u16,
    pub stride: u8,
    pub type: u8,
}

// System DMA platform data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_system_dma_plat_info {
    pub reg_map: *const omap_dma_reg,
    pub channel_stride: unsigned,
    pub dma_attr: *mut omap_dma_dev_attr,
    pub errata: u32,
    pub (*show_dma_caps)(void): *mut c_void,
    pub lch): *mut *mut void (clear_lch_regs)(int,
    pub lch): *mut *mut void (clear_dma)(int,
    pub lch): *mut *mut void (dma_write)(u32 val, int reg, int,
    pub lch): *mut *mut u32 (dma_read)(int reg, int,
    pub slave_map: *const dma_slave_map,
    pub slavecnt: c_int,
}

pub const dma_omap2plus(): c_int = 1;

pub const dma_omap2plus(): c_int = 0;

extern "C" {
    pub fn omap_set_dma_priority(lch: c_int, dst_port: c_int, priority: c_int);
}

extern "C" {
    pub fn omap_free_dma(ch: c_int);
}

extern "C" {
    pub fn omap_disable_dma_irq(ch: c_int, irq_bits: u16);
}
extern "C" {
    pub fn omap_start_dma(lch: c_int);
}
extern "C" {
    pub fn omap_stop_dma(lch: c_int);
}
extern "C" {
    pub fn omap_set_dma_channel_mode(lch: c_int, mode: omap_dma_channel_mode);
}
extern "C" {
    pub fn omap_set_dma_src_data_pack(lch: c_int, enable: c_int);
}
extern "C" {
    pub fn omap_set_dma_dest_data_pack(lch: c_int, enable: c_int);
}
extern "C" {
    pub fn omap_get_dma_src_pos(lch: c_int) -> dma_addr_t;
}
extern "C" {
    pub fn omap_get_dma_dst_pos(lch: c_int) -> dma_addr_t;
}
extern "C" {
    pub fn omap_get_dma_active_status(lch: c_int) -> c_int;
}

extern "C" {
    pub fn omap_dma_running() -> c_int;
}

extern "C" {
    pub fn omap_lcd_dma_running() -> c_int;
}

