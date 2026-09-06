//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soundwire/sdw_amd.h
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
//
// Copyright (C) 2023-24 Advanced Micro Devices, Inc. All rights reserved.
//

// AMD pm_runtime quirk definitions
//
// Force the clock to stop(ClockStopMode0) when suspend callback
// is invoked.
//
pub const AMD_SDW_CLK_STOP_MODE: c_int = 1;
//
// Stop the bus when runtime suspend/system level suspend callback
// is invoked. If set, a complete bus reset and re-enumeration will
// be performed when the bus restarts. In-band wake interrupts are
// not supported in this mode.
//
pub const AMD_SDW_POWER_OFF_MODE: c_int = 2;
pub const ACP_SDW0: c_int = 0;
pub const ACP_SDW1: c_int = 1;
pub const AMD_SDW_MAX_MANAGER_COUNT: c_int = 2;
pub const ACP63_PCI_REV_ID: c_uint = 0x63;
pub const ACP70_PCI_REV_ID: c_uint = 0x70;
pub const ACP71_PCI_REV_ID: c_uint = 0x71;
pub const ACP72_PCI_REV_ID: c_uint = 0x72;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acp_sdw_pdata {
    pub instance: u16,
    pub acp_rev: u32,
// mutex to protect acp common register access
    pub acp_sdw_lock: *mut mutex,
}

//
// struct sdw_amd_dai_runtime: AMD sdw dai runtime  data
//
// @name: SoundWire stream name
// @stream: stream runtime
// @bus: Bus handle
// @stream_type: Stream type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_amd_dai_runtime {
    pub name: *mut c_char,
    pub stream: *mut sdw_stream_runtime,
    pub bus: *mut sdw_bus,
    pub stream_type: sdw_stream_type,
}

//
// struct amd_sdw_manager - amd manager driver context
// @bus: bus handle
// @dev: linux device
// @mmio: SoundWire registers mmio base
// @acp_mmio: acp registers mmio base
// @amd_sdw_irq_thread: SoundWire manager irq workqueue
// @amd_sdw_work: peripheral status work queue
// @acp_sdw_lock: mutex to protect acp share register access
// @status: peripheral devices status array
// @num_din_ports: number of input ports
// @num_dout_ports: number of output ports
// @max_ports: total number of input ports and output ports
// @cols_index: Column index in frame shape
// @rows_index: Rows index in frame shape
// @port_offset_map: dynamic array to map port block offset
// @instance: SoundWire manager instance
// @quirks: SoundWire manager quirks
// @wake_en_mask: wake enable mask per SoundWire manager
// @acp_rev: acp pci device revision id
// @clk_stopped: flag set to true when clock is stopped
// @power_mode_mask: flag interprets amd SoundWire manager power mode
// @dai_runtime_array: dai runtime array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_sdw_manager {
    pub bus: sdw_bus,
    pub dev: *mut device,
    pub mmio: *mut void __iomem,
    pub acp_mmio: *mut void __iomem,
    pub amd_sdw_irq_thread: work_struct,
    pub amd_sdw_work: work_struct,
// mutex to protect acp common register access
    pub acp_sdw_lock: *mut mutex,
    pub 1]: sdw_slave_status status[SDW_MAX_DEVICES +,
    pub num_din_ports: c_int,
    pub num_dout_ports: c_int,
    pub max_ports: c_int,
    pub cols_index: c_int,
    pub rows_index: c_int,
    pub port_offset_map: *mut c_int,
    pub instance: u32,
    pub quirks: u32,
    pub wake_en_mask: u32,
    pub power_mode_mask: u32,
    pub acp_rev: u32,
    pub clk_stopped: bool,
    pub dai_runtime_array: *mut sdw_amd_dai_runtime,
}

//
// struct sdw_amd_acpi_info - Soundwire AMD information found in ACPI tables
// @handle: ACPI controller handle
// @count: maximum no of soundwire manager links supported on AMD platform.
// @link_mask: bit-wise mask listing links enabled by BIOS menu
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_amd_acpi_info {
    pub handle: acpi_handle,
    pub count: c_int,
    pub link_mask: u32,
}

//
// struct sdw_amd_ctx - context allocated by the controller driver probe
//
// @count: link count
// @link_mask: bit-wise mask listing SoundWire links reported by the
// Controller
// @pdev: platform device structure
// @peripherals: array representing Peripherals exposed across all enabled links
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_amd_ctx {
    pub count: c_int,
    pub link_mask: u32,
    pub pdev: [*mut platform_device; AMD_SDW_MAX_MANAGER_COUNT],
    pub peripherals: *mut sdw_peripherals,
}

//
// struct sdw_amd_res - Soundwire AMD global resource structure,
// typically populated by the DSP driver/Legacy driver
//
// @acp_rev: acp pci device revision id
// @addr: acp pci device resource start address
// @reg_range: ACP register range
// @link_mask: bit-wise mask listing links selected by the DSP driver
// legacy driver
// @count: link count
// @mmio_base: mmio base of SoundWire registers
// @handle: ACPI parent handle
// @parent: parent device
// @dev: device implementing hwparams and free callbacks
// @acp_lock: mutex protecting acp common registers access
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_amd_res {
    pub acp_rev: u32,
    pub addr: u32,
    pub reg_range: u32,
    pub link_mask: u32,
    pub count: c_int,
    pub mmio_base: *mut void __iomem,
    pub handle: acpi_handle,
    pub parent: *mut device,
    pub dev: *mut device,
// use to protect acp common registers access
    pub acp_lock: *mut mutex,
}

extern "C" {
    pub fn sdw_amd_probe(res: *mut sdw_amd_res, ctx: *mut sdw_amd_ctx) -> c_int;
}
extern "C" {
    pub fn sdw_amd_exit(ctx: *mut sdw_amd_ctx);
}
extern "C" {
    pub fn sdw_amd_get_slave_info(ctx: *mut sdw_amd_ctx) -> c_int;
}
extern "C" {
    pub fn amd_sdw_scan_controller(info: *mut sdw_amd_acpi_info) -> c_int;
}
