//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soundwire/intel.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
// Copyright(c) 2015-17 Intel Corporation.
//
// struct sdw_intel_link_res - Soundwire Intel link resource structure,
// typically populated by the controller driver.
// @hw_ops: platform-specific ops
// @mmio_base: mmio base of SoundWire registers
// @registers: Link IO registers base
// @ip_offset: offset for MCP_IP registers
// @shim: Audio shim pointer
// @shim_vs: Audio vendor-specific shim pointer
// @alh: ALH (Audio Link Hub) pointer
// @irq: Interrupt line
// @ops: Shim callback ops
// @dev: device implementing hw_params and free callbacks
// @shim_lock: mutex to handle access to shared SHIM registers
// @shim_mask: global pointer to check SHIM register initialization
// @clock_stop_quirks: mask defining requested behavior on pm_suspend
// @mic_privacy: ACE version supports microphone privacy
// @link_mask: global mask needed for power-up/down sequences
// @cdns: Cadence master descriptor
// @list: used to walk-through all masters exposed by the same controller
// @hbus: hdac_bus pointer, needed for power management
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_intel_link_res {
    pub hw_ops: *const sdw_intel_hw_ops,
    pub /: *mut *mut *mut void __iomem mmio_base; / not strictly needed, useful for debug,
    pub registers: *mut void __iomem,
    pub ip_offset: u32,
    pub shim: *mut void __iomem,
    pub shim_vs: *mut void __iomem,
    pub alh: *mut void __iomem,
    pub irq: c_int,
    pub ops: *const sdw_intel_ops,
    pub dev: *mut device,
    pub /: *mut *mut *mut mutex shim_lock; / protect shared registers,
    pub shim_mask: *mut u32,
    pub clock_stop_quirks: u32,
    pub mic_privacy: bool,
    pub link_mask: u32,
    pub cdns: *mut sdw_cdns,
    pub list: list_head,
    pub hbus: *mut hdac_bus,
}

//
// struct sdw_intel_bpt - SoundWire Intel BPT context
// @bpt_tx_stream: BPT TX stream
// @dmab_tx_bdl: BPT TX buffer descriptor list
// @bpt_rx_stream: BPT RX stream
// @dmab_rx_bdl: BPT RX buffer descriptor list
// @pdi0_buffer_size: PDI0 buffer size
// @pdi1_buffer_size: PDI1 buffer size
// @num_frames: number of frames
// @data_per_frame: data per frame
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_intel_bpt {
    pub bpt_tx_stream: *mut hdac_ext_stream,
    pub dmab_tx_bdl: snd_dma_buffer,
    pub bpt_rx_stream: *mut hdac_ext_stream,
    pub dmab_rx_bdl: snd_dma_buffer,
    pub pdi0_buffer_size: c_uint,
    pub pdi1_buffer_size: c_uint,
    pub num_frames: c_uint,
    pub data_per_frame: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_intel {
    pub cdns: sdw_cdns,
    pub instance: c_int,
    pub link_res: *mut sdw_intel_link_res,
    pub startup_done: bool,
    pub bpt_ctx: sdw_intel_bpt,

    pub debugfs: *mut dentry,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_intel_prop {
    pub clde: u16,
    pub doaise2: u16,
    pub dodse2: u16,
    pub clds: u16,
    pub clss: u16,
    pub doaise: u16,
    pub doais: u16,
    pub dodse: u16,
    pub dods: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_pdi_type {
    INTEL_PDI_IN = 0,
    INTEL_PDI_OUT = 1,
    INTEL_PDI_BD = 2,
}

//
// Read, write helpers for HW registers
//
extern "C" {
    pub fn readl(offset: base +) -> return;
}
extern "C" {
    pub fn readw(offset: base +) -> return;
}

pub const INTEL_MASTER_RESET_ITERATIONS: c_int = 10;
pub const SDW_INTEL_DELAYED_ENUMERATION_MS: c_int = 100;

extern "C" {
    pub fn intel_ace2x_debugfs_init(sdw: *mut sdw_intel);
}
extern "C" {
    pub fn intel_ace2x_debugfs_exit(sdw: *mut sdw_intel);
}

extern "C" {
    pub fn SDW_INTEL_OPS(_arg: sdw, _arg: register_dai)(sdw) -> return;
}
extern "C" {
    pub fn SDW_INTEL_OPS(_arg: sdw, _arg: start_bus)(sdw) -> return;
}
extern "C" {
    pub fn SDW_INTEL_OPS(_arg: sdw, _arg: start_bus_after_reset)(sdw) -> return;
}
extern "C" {
    pub fn SDW_INTEL_OPS(_arg: sdw, _arg: start_bus_after_clock_stop)(sdw) -> return;
}
extern "C" {
    pub fn SDW_INTEL_OPS(_arg: sdw, _arg: stop_bus)(sdw, _arg: clock_stop) -> return;
}
extern "C" {
    pub fn SDW_INTEL_OPS(_arg: sdw, _arg: link_power_up)(sdw) -> return;
}
extern "C" {
    pub fn SDW_INTEL_OPS(_arg: sdw, _arg: link_power_down)(sdw) -> return;
}
extern "C" {
    pub fn SDW_INTEL_OPS(_arg: sdw, _arg: shim_check_wake)(sdw) -> return;
}
extern "C" {
    pub fn SDW_INTEL_OPS(_arg: sdw, _arg: sync_go_unlocked)(sdw) -> return;
}
extern "C" {
    pub fn SDW_INTEL_OPS(_arg: sdw, _arg: sync_go)(sdw) -> return;
}
extern "C" {
    pub fn SDW_INTEL_OPS(_arg: sdw, _arg: sync_check_cmdsync_unlocked)(sdw) -> return;
}
extern "C" {
    pub fn SDW_INTEL_OPS(_arg: sdw, _arg: get_link_count)(sdw) -> return;
}
// common bus management
extern "C" {
    pub fn intel_start_bus(sdw: *mut sdw_intel) -> c_int;
}
extern "C" {
    pub fn intel_start_bus_after_reset(sdw: *mut sdw_intel) -> c_int;
}
extern "C" {
    pub fn intel_check_clock_stop(sdw: *mut sdw_intel);
}
extern "C" {
    pub fn intel_start_bus_after_clock_stop(sdw: *mut sdw_intel) -> c_int;
}
extern "C" {
    pub fn intel_stop_bus(sdw: *mut sdw_intel, clock_stop: bool) -> c_int;
}
// common bank switch routines
extern "C" {
    pub fn intel_pre_bank_switch(sdw: *mut sdw_intel) -> c_int;
}
extern "C" {
    pub fn intel_post_bank_switch(sdw: *mut sdw_intel) -> c_int;
}
