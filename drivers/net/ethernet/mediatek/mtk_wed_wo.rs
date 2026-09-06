//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mediatek/mtk_wed_wo.h
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
// Copyright (C) 2022 Lorenzo Bianconi <lorenzo@kernel.org>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_wed_mcu_hdr {
// DW0
    pub version: u8,
    pub cmd: u8,
    pub length: __le16,
// DW1
    pub seq: __le16,
    pub flag: __le16,
// DW2
    pub status: __le32,
// DW3
    pub rsv: [u8; 20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_wed_wo_log_info {
    pub sn: __le32,
    pub total: __le32,
    pub rro: __le32,
    pub mod: __le32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_wed_wo_event {
    MTK_WED_WO_EVT_LOG_DUMP		= 0x1,
    MTK_WED_WO_EVT_PROFILING	= 0x2,
    MTK_WED_WO_EVT_RXCNT_INFO	= 0x3,
}

pub const MTK_WED_MODULE_ID_WO: c_int = 1;

pub const MTK_WED_WO_CPU_MCUSYS_RESET_ADDR: c_uint = 0x15194050;
pub const MTK_WED_WO_CPU_WO0_MCUSYS_RESET_MASK: c_uint = 0x20;
pub const MTK_WED_WO_CPU_WO1_MCUSYS_RESET_MASK: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_wed_wo_state {
    MTK_WED_WO_STATE_UNDEFINED,
    MTK_WED_WO_STATE_INIT,
    MTK_WED_WO_STATE_ENABLE,
    MTK_WED_WO_STATE_DISABLE,
    MTK_WED_WO_STATE_HALT,
    MTK_WED_WO_STATE_GATING,
    MTK_WED_WO_STATE_SER_RESET,
    MTK_WED_WO_STATE_WF_RESET,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_wed_wo_done_state {
    MTK_WED_WOIF_UNDEFINED,
    MTK_WED_WOIF_DISABLE_DONE,
    MTK_WED_WOIF_TRIGGER_ENABLE,
    MTK_WED_WOIF_ENABLE_DONE,
    MTK_WED_WOIF_TRIGGER_GATING,
    MTK_WED_WOIF_GATING_DONE,
    MTK_WED_WOIF_TRIGGER_HALT,
    MTK_WED_WOIF_HALT_DONE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_wed_dummy_cr_idx {
    MTK_WED_DUMMY_CR_FWDL,
    MTK_WED_DUMMY_CR_WO_STATUS,
}

pub const MTK_WO_MCU_CFG_LS_BASE: c_int = 0;

pub const MTK_WED_WO_RING_SIZE: c_int = 256;
pub const MTK_WED_WO_CMD_LEN: c_int = 1504;
pub const MTK_WED_WO_TXCH_NUM: c_int = 0;
pub const MTK_WED_WO_RXCH_NUM: c_int = 1;
pub const MTK_WED_WO_RXCH_WO_EXCEPTION: c_int = 7;

pub const MTK_WED_WO_CCIF_BUSY: c_uint = 0x004;
pub const MTK_WED_WO_CCIF_START: c_uint = 0x008;
pub const MTK_WED_WO_CCIF_TCHNUM: c_uint = 0x00c;
pub const MTK_WED_WO_CCIF_RCHNUM: c_uint = 0x010;

pub const MTK_WED_WO_CCIF_ACK: c_uint = 0x014;
pub const MTK_WED_WO_CCIF_IRQ0_MASK: c_uint = 0x018;
pub const MTK_WED_WO_CCIF_IRQ1_MASK: c_uint = 0x01c;
pub const MTK_WED_WO_CCIF_DUMMY1: c_uint = 0x020;
pub const MTK_WED_WO_CCIF_DUMMY2: c_uint = 0x024;
pub const MTK_WED_WO_CCIF_DUMMY3: c_uint = 0x028;
pub const MTK_WED_WO_CCIF_DUMMY4: c_uint = 0x02c;
pub const MTK_WED_WO_CCIF_SHADOW1: c_uint = 0x030;
pub const MTK_WED_WO_CCIF_SHADOW2: c_uint = 0x034;
pub const MTK_WED_WO_CCIF_SHADOW3: c_uint = 0x038;
pub const MTK_WED_WO_CCIF_SHADOW4: c_uint = 0x03c;
pub const MTK_WED_WO_CCIF_DUMMY5: c_uint = 0x050;
pub const MTK_WED_WO_CCIF_DUMMY6: c_uint = 0x054;
pub const MTK_WED_WO_CCIF_DUMMY7: c_uint = 0x058;
pub const MTK_WED_WO_CCIF_DUMMY8: c_uint = 0x05c;
pub const MTK_WED_WO_CCIF_SHADOW5: c_uint = 0x060;
pub const MTK_WED_WO_CCIF_SHADOW6: c_uint = 0x064;
pub const MTK_WED_WO_CCIF_SHADOW7: c_uint = 0x068;
pub const MTK_WED_WO_CCIF_SHADOW8: c_uint = 0x06c;

pub const MTK_WED_WO_CTL_SD_LEN0_SHIFT: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_wed_wo_memory_region {
    pub name: *const c_char,
    pub addr: *mut void __iomem,
    pub phy_addr: phys_addr_t,
    pub size: u32,
    pub shared:1: bool,
    pub consumed:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_wed_fw_region {
    pub decomp_crc: __le32,
    pub decomp_len: __le32,
    pub decomp_blk_sz: __le32,
    pub rsv0: [u8; 4],
    pub addr: __le32,
    pub len: __le32,
    pub feature_set: u8,
    pub rsv1: [u8; 15],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_wed_fw_trailer {
    pub chip_id: u8,
    pub eco_code: u8,
    pub num_region: u8,
    pub format_ver: u8,
    pub format_flag: u8,
    pub rsv: [u8; 2],
    pub fw_ver: [c_char; 10],
    pub build_date: [c_char; 15],
    pub crc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_wed_wo_queue_regs {
    pub desc_base: u32,
    pub ring_size: u32,
    pub cpu_idx: u32,
    pub dma_idx: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_wed_wo_queue_desc {
    pub buf0: __le32,
    pub ctrl: __le32,
    pub buf1: __le32,
    pub info: __le32,
    pub reserved: [__le32; 4],
    pub __aligned(32): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_wed_wo_queue_entry {
    pub addr: dma_addr_t,
    pub buf: *mut c_void,
    pub len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_wed_wo_queue {
    pub regs: mtk_wed_wo_queue_regs,
    pub cache: page_frag_cache,
    pub desc: *mut mtk_wed_wo_queue_desc,
    pub desc_dma: dma_addr_t,
    pub entry: *mut mtk_wed_wo_queue_entry,
    pub head: u16,
    pub tail: u16,
    pub n_desc: c_int,
    pub queued: c_int,
    pub buf_size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_wed_wo {
    pub hw: *mut mtk_wed_hw,
    pub q_tx: mtk_wed_wo_queue,
    pub q_rx: mtk_wed_wo_queue,
    pub mutex: mutex,
    pub timeout: c_int,
    pub seq: u16,
    pub res_q: sk_buff_head,
    pub wait: wait_queue_head_t,
    pub mcu: },
    pub regs: *mut regmap,
    pub lock: spinlock_t,
    pub irq_tasklet: tasklet_struct,
    pub irq: c_int,
    pub irq_mask: u32,
    pub mmio: },
}

extern "C" {
    pub fn mtk_wed_mcu_rx_event(wo: *mut mtk_wed_wo, skb: *mut sk_buff);
}
extern "C" {
    pub fn mtk_wed_mcu_init(wo: *mut mtk_wed_wo) -> c_int;
}
extern "C" {
    pub fn mtk_wed_wo_init(hw: *mut mtk_wed_hw) -> c_int;
}
extern "C" {
    pub fn mtk_wed_wo_deinit(hw: *mut mtk_wed_hw);
}
