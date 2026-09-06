//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/mei/hw-me.h
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
// Copyright (c) 2012-2022, Intel Corporation. All rights reserved.
// Intel Management Engine Interface (Intel MEI) Linux driver
//

//
// mei_cfg - mei device configuration
//
// @fw_status: FW status
// @quirk_probe: device exclusion quirk
// @get_kind: MEI head kind helper
// @dma_size: device DMA buffers size
// @fw_ver_supported: is fw version retrievable from FW
// @hw_trc_supported: does the hw support trc register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_cfg {
    pub fw_status: mei_fw_status,
    pub pdev): *const *const bool (quirk_probe)(struct pci_dev,
    pub parent): *const *const mei_dev_kind (get_kind)(struct device,
    pub dma_size: [usize; DMA_DSCR_NUM],
    pub fw_ver_supported:1: u32,
    pub hw_trc_supported:1: u32,
}

//
// struct mei_me_hw - me hw specific data
//
// @cfg: per device generation config and ops
// @mem_addr: io memory address
// @irq: irq number
// @pg_state: power gating state
// @d0i3_supported: di03 support
// @hbuf_depth: depth of hardware host/write buffer in slots
// @read_fws: read FW status register handler
// @polling_thread: interrupt polling thread
// @wait_active: the polling thread activity wait queue
// @is_active: the device is active
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_me_hw {
    pub cfg: *const mei_cfg,
    pub mem_addr: *mut void __iomem,
    pub irq: c_int,
    pub pg_state: mei_pg_state,
    pub d0i3_supported: bool,
    pub hbuf_depth: u8,
    pub val): *const *const *const *const int (read_fws)(struct mei_device dev, int where, char name, u32,
// polling
    pub polling_thread: *mut task_struct,
    pub wait_active: wait_queue_head_t,
    pub is_active: bool,
}

//
// enum mei_cfg_idx - indices to platform specific configurations.
//
// Note: has to be synchronized with mei_cfg_list[]
//
// @MEI_ME_UNDEF_CFG:      Lower sentinel.
// @MEI_ME_ICH_CFG:        I/O Controller Hub legacy devices.
// @MEI_ME_ICH10_CFG:      I/O Controller Hub platforms Gen10
// @MEI_ME_PCH6_CFG:       Platform Controller Hub platforms (Gen6).
// @MEI_ME_PCH7_CFG:       Platform Controller Hub platforms (Gen7).
// @MEI_ME_PCH_CPT_PBG_CFG:Platform Controller Hub workstations
// with quirk for Node Manager exclusion.
// @MEI_ME_PCH8_CFG:       Platform Controller Hub Gen8 and newer
// client platforms.
// @MEI_ME_PCH8_ITOUCH_CFG:Platform Controller Hub Gen8 and newer
// client platforms (iTouch).
// @MEI_ME_PCH8_SPS_4_CFG: Platform Controller Hub Gen8 and newer
// servers platforms with quirk for
// SPS firmware exclusion.
// @MEI_ME_PCH12_CFG:      Platform Controller Hub Gen12 and newer
// @MEI_ME_PCH12_SPS_4_CFG:Platform Controller Hub Gen12 up to 4.0
// servers platforms with quirk for
// SPS firmware exclusion.
// @MEI_ME_PCH12_SPS_CFG:  Platform Controller Hub Gen12 5.0 and newer
// servers platforms with quirk for
// SPS firmware exclusion.
// @MEI_ME_PCH12_SPS_ITOUCH_CFG: Platform Controller Hub Gen12
// client platforms (iTouch)
// @MEI_ME_PCH15_CFG:      Platform Controller Hub Gen15 and newer
// @MEI_ME_PCH15_SPS_CFG:  Platform Controller Hub Gen15 and newer
// servers platforms with quirk for
// SPS firmware exclusion.
// @MEI_ME_GSC_CFG:        Graphics System Controller
// @MEI_ME_GSCFI_CFG:      Graphics System Controller Firmware Interface
// @MEI_ME_CSC_CFG:        Chassis System Controller Firmware Interface
// @MEI_ME_PCH22_IOE_CFG:  Platform Controller Hub Gen22 and newer with IOE detection
// @MEI_ME_NUM_CFG:        Upper Sentinel.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mei_cfg_idx {
    MEI_ME_UNDEF_CFG,
    MEI_ME_ICH_CFG,
    MEI_ME_ICH10_CFG,
    MEI_ME_PCH6_CFG,
    MEI_ME_PCH7_CFG,
    MEI_ME_PCH_CPT_PBG_CFG,
    MEI_ME_PCH8_CFG,
    MEI_ME_PCH8_ITOUCH_CFG,
    MEI_ME_PCH8_SPS_4_CFG,
    MEI_ME_PCH12_CFG,
    MEI_ME_PCH12_SPS_4_CFG,
    MEI_ME_PCH12_SPS_CFG,
    MEI_ME_PCH12_SPS_ITOUCH_CFG,
    MEI_ME_PCH15_CFG,
    MEI_ME_PCH15_SPS_CFG,
    MEI_ME_GSC_CFG,
    MEI_ME_GSCFI_CFG,
    MEI_ME_CSC_CFG,
    MEI_ME_PCH22_IOE_CFG,
    MEI_ME_NUM_CFG
}

extern "C" {
    pub fn mei_me_pg_enter_sync(dev: *mut mei_device) -> c_int;
}
extern "C" {
    pub fn mei_me_pg_exit_sync(dev: *mut mei_device) -> c_int;
}
extern "C" {
    pub fn mei_me_irq_quick_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn mei_me_irq_thread_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn mei_me_polling_thread(_dev: *mut c_void) -> c_int;
}
