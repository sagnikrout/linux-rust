//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/arm/display/komeda/komeda_dev.h
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
// (C) COPYRIGHT 2018 ARM Limited. All rights reserved.
// Author: James.Qian.Wang <james.qian.wang@arm.com>
//

// pipeline DT ports
#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_chip_info {
    pub arch_id: u32,
    pub core_id: u32,
    pub core_info: u32,
    pub bus_width: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_events {
    pub global: u64,
    pub pipes: [u64; KOMEDA_MAX_PIPELINES],
}

//
// struct komeda_dev_funcs
//
// Supplied by chip level and returned by the chip entry function xxx_identify,
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_dev_funcs {
//
// @init_format_table:
//
// initialize &komeda_dev->format_table, this function should be called
// before the &enum_resource
//
    pub mdev): *mut *mut void (init_format_table)(struct komeda_dev,
//
// @enum_resources:
//
// for CHIP to report or add pipeline and component resources to CORE
//
    pub mdev): *mut *mut int (enum_resources)(struct komeda_dev,
// @cleanup: call to chip to cleanup komeda_dev->chip data
    pub mdev): *mut *mut void (cleanup)(struct komeda_dev,
// @connect_iommu: Optional, connect to external iommu
    pub mdev): *mut *mut int (connect_iommu)(struct komeda_dev,
// @disconnect_iommu: Optional, disconnect to external iommu
    pub mdev): *mut *mut int (disconnect_iommu)(struct komeda_dev,
//
// @irq_handler:
//
// for CORE to get the HW event from the CHIP when interrupt happened.
//
    pub events): *mut komeda_events,
// @enable_irq: enable irq
    pub mdev): *mut *mut int (enable_irq)(struct komeda_dev,
// @disable_irq: disable irq
    pub mdev): *mut *mut int (disable_irq)(struct komeda_dev,
// @on_off_vblank: notify HW to on/off vblank
    pub on): int master_pipe, bool,
// @dump_register: Optional, dump registers to seq_file
    pub seq): *mut *mut *mut void (dump_register)(struct komeda_dev mdev, struct seq_file,
//
// @change_opmode:
//
// Notify HW to switch to a new display operation mode.
//
    pub new_mode): *mut *mut *mut int (change_opmode)(struct komeda_dev mdev, int,
// @flush: Notify the HW to flush or kickoff the update
    pub active_pipes): int master_pipe, u32,
}

//
// DISPLAY_MODE describes how many display been enabled, and which will be
// passed to CHIP by &komeda_dev_funcs->change_opmode(), then CHIP can do the
// pipeline resources assignment according to this usage hint.
// -   KOMEDA_MODE_DISP0: Only one display enabled, pipeline-0 work as master.
// -   KOMEDA_MODE_DISP1: Only one display enabled, pipeline-0 work as master.
// -   KOMEDA_MODE_DUAL_DISP: Dual display mode, both display has been enabled.
// And D71 supports assign two pipelines to one single display on mode
// KOMEDA_MODE_DISP0/DISP1
//
// struct komeda_dev
//
// Pipeline and component are used to describe how to handle the pixel data.
// komeda_device is for describing the whole view of the device, and the
// control-abilites of device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_dev {
// @dev: the base device structure
    pub dev: *mut device,
// @reg_base: the base address of komeda io space
    pub reg_base: *mut u32 __iomem,
// @chip: the basic chip information
    pub chip: komeda_chip_info,
// @fmt_tbl: initialized by &komeda_dev_funcs->init_format_table
    pub fmt_tbl: komeda_format_caps_table,
// @aclk: HW main engine clk
    pub aclk: *mut clk,
// @irq: irq number
    pub irq: c_int,
// @lock: used to protect dpmode
    pub lock: mutex,
// @dpmode: current display mode
    pub dpmode: u32,
// @n_pipelines: the number of pipe in @pipelines
    pub n_pipelines: c_int,
// @pipelines: the komeda pipelines
    pub pipelines: [*mut komeda_pipeline; KOMEDA_MAX_PIPELINES],
// @funcs: chip funcs to access to HW
    pub funcs: *const komeda_dev_funcs,
//
// @chip_data:
//
// chip data will be added by &komeda_dev_funcs.enum_resources() and
// destroyed by &komeda_dev_funcs.cleanup()
//
    pub chip_data: *mut c_void,
// @iommu: iommu domain
    pub iommu: *mut iommu_domain,
// @debugfs_root: root directory of komeda debugfs
    pub debugfs_root: *mut dentry,
//
// @err_verbosity: bitmask for how much extra info to print on error
//
// See KOMEDA_DEV_* macros for details. Low byte contains the debug
// level categories, the high byte contains extra debug options.
//
    pub err_verbosity: u16,
// Print a single line per error per frame with error events.

// Print a single line per warning per frame with error events.

// Print a single line per info event per frame with error events.

// Dump DRM state on an error or warning event.

// Disable rate limiting of event prints (normally one per commit)

}

extern "C" {
    pub fn komeda_dev_destroy(mdev: *mut komeda_dev);
}
extern "C" {
    pub fn komeda_print_events(evts: *mut komeda_events, dev: *mut drm_device);
}
extern "C" {
    pub fn komeda_dev_resume(mdev: *mut komeda_dev) -> c_int;
}
extern "C" {
    pub fn komeda_dev_suspend(mdev: *mut komeda_dev) -> c_int;
}
