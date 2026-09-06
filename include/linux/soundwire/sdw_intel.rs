//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soundwire/sdw_intel.h
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
// cAVS and ACE1.x definitions
//
pub const SDW_SHIM_BASE: c_uint = 0x2C000;
pub const SDW_ALH_BASE: c_uint = 0x2C800;
pub const SDW_SHIM_BASE_ACE: c_uint = 0x38000;
pub const SDW_ALH_BASE_ACE: c_uint = 0x24000;
pub const SDW_LINK_BASE: c_uint = 0x30000;
pub const SDW_LINK_SIZE: c_uint = 0x10000;
// Intel SHIM Registers Definition
// LCAP
pub const SDW_SHIM_LCAP: c_uint = 0x0;

// LCTL
pub const SDW_SHIM_LCTL: c_uint = 0x4;

pub const SDW_SHIM_MLCS_XTAL_CLK: c_uint = 0x0;
pub const SDW_SHIM_MLCS_CARDINAL_CLK: c_uint = 0x1;
pub const SDW_SHIM_MLCS_AUDIO_PLL_CLK: c_uint = 0x2;
// SYNC
pub const SDW_SHIM_SYNC: c_uint = 0xC;

// Control stream capabililities and channel mask

// PCM Stream capabilities

// PCM Stream Channel Map

// PCM Stream Channel Count

// IO control

// Wake Enable
pub const SDW_SHIM_WAKEEN: c_uint = 0x190;

// Wake Status
pub const SDW_SHIM_WAKESTS: c_uint = 0x192;

// AC Timing control

// Intel ALH Register definitions

pub const SDW_ALH_NUM_STREAMS: c_int = 64;
pub const SDW_ALH_STRMZCFG_DMAT_VAL: c_uint = 0x3;

//
// ACE2.x definitions for SHIM registers - only accessible when the
// HDAudio extended link LCTL.SPA/CPA = 1.
//
// x variable is link index

// SHIM2 Generic Registers
// Read-only capabilities
pub const SDW_SHIM2_LECAP: c_uint = 0x00;

// PCM Stream capabilities
pub const SDW_SHIM2_PCMSCAP: c_uint = 0x10;

// Read-only PCM Stream Channel Count, y variable is stream

// PCM Stream Channel Map

// SHIM2 vendor-specific registers
pub const SDW_SHIM2_INTEL_VS_LVSCTL: c_uint = 0x04;

pub const SDW_SHIM2_MLCS_XTAL_CLK: c_uint = 0x0;
pub const SDW_SHIM2_MLCS_CARDINAL_CLK: c_uint = 0x1;
pub const SDW_SHIM2_MLCS_AUDIO_PLL_CLK: c_uint = 0x2;
pub const SDW_SHIM2_MLCS_MCLK_INPUT_CLK: c_uint = 0x3;
pub const SDW_SHIM2_MLCS_WOV_RING_OSC_CLK: c_uint = 0x4;
pub const SDW_SHIM2_INTEL_VS_WAKEEN: c_uint = 0x08;

pub const SDW_SHIM2_INTEL_VS_WAKESTS: c_uint = 0x0A;

pub const SDW_SHIM2_INTEL_VS_IOCTL: c_uint = 0x0C;

pub const SDW_SHIM2_INTEL_VS_ACTMCTL: c_uint = 0x0E;

// ACE3+ Mic privacy control and status register
pub const SDW_SHIM2_INTEL_VS_PVCCS: c_uint = 0x10;
//
// struct sdw_intel_stream_params_data: configuration passed during
// the @params_stream callback, e.g. for interaction with DSP
// firmware.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_intel_stream_params_data {
    pub substream: *mut snd_pcm_substream,
    pub dai: *mut snd_soc_dai,
    pub hw_params: *mut snd_pcm_hw_params,
    pub link_id: c_int,
    pub alh_stream_id: c_int,
}

//
// struct sdw_intel_stream_free_data: configuration passed during
// the @free_stream callback, e.g. for interaction with DSP
// firmware.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_intel_stream_free_data {
    pub substream: *mut snd_pcm_substream,
    pub dai: *mut snd_soc_dai,
    pub link_id: c_int,
}

//
// struct sdw_intel_ops: Intel audio driver callback ops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_intel_ops {
    pub params_data): *mut sdw_intel_stream_params_data,
    pub free_data): *mut sdw_intel_stream_free_data,
    pub dai): *mut *mut *mut int (trigger)(struct snd_pcm_substream substream, int cmd, struct snd_soc_dai,
}

//
// struct sdw_intel_acpi_info - Soundwire Intel information found in ACPI tables
// @handle: ACPI controller handle
// @count: link count found with "sdw-master-count" or "sdw-manager-list" property
// @link_mask: bit-wise mask listing links enabled by BIOS menu
//
// this structure could be expanded to e.g. provide all the _ADR
// information in case the link_mask is not sufficient to identify
// platform capabilities.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_intel_acpi_info {
    pub handle: acpi_handle,
    pub count: c_int,
    pub link_mask: u32,
}

// Intel clock-stop/pm_runtime quirk definitions
//
// Force the clock to remain on during pm_runtime suspend. This might
// be needed if Slave devices do not have an alternate clock source or
// if the latency requirements are very strict.
//

//
// Stop the bus during pm_runtime suspend. If set, a complete bus
// reset and re-enumeration will be performed when the bus
// restarts. This mode shall not be used if Slave devices can generate
// in-band wakes.
//

//
// Stop the bus during pm_suspend if Slaves are not wake capable
// (e.g. speaker amplifiers). The clock-stop mode is typically
// slightly higher power than when the IP is completely powered-off.
//

//
// Require a bus reset (and complete re-enumeration) when exiting
// clock stop modes. This may be needed if the controller power was
// turned off and all context lost. This quirk shall not be used if a
// Slave device needs to remain enumerated and keep its context,
// e.g. to provide the reasons for the wake, report acoustic events or
// pass a history buffer.
//

//
// struct sdw_intel_ctx - context allocated by the controller
// driver probe
// @count: link count
// @mmio_base: mmio base of SoundWire registers, only used to check
// hardware capabilities after all power dependencies are settled.
// @link_mask: bit-wise mask listing SoundWire links reported by the
// Controller
// @handle: ACPI parent handle
// @ldev: information for each link (controller-specific and kept
// opaque here)
// @link_list: list to handle interrupts across all links
// @shim_lock: mutex to handle concurrent rmw access to shared SHIM registers.
// @shim_mask: flags to track initialization of SHIM shared registers
// @shim_base: sdw shim base.
// @alh_base: sdw alh base.
// @peripherals: array representing Peripherals exposed across all enabled links
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_intel_ctx {
    pub count: c_int,
    pub mmio_base: *mut void __iomem,
    pub link_mask: u32,
    pub handle: acpi_handle,
    pub ldev: *mut sdw_intel_link_dev,
    pub link_list: list_head,
    pub /: *mut *mut mutex shim_lock; / lock for access to shared SHIM registers,
    pub shim_mask: u32,
    pub shim_base: u32,
    pub alh_base: u32,
    pub peripherals: *mut sdw_peripherals,
}

//
// struct sdw_intel_res - Soundwire Intel global resource structure,
// typically populated by the DSP driver
//
// @hw_ops: abstraction for platform ops
// @count: link count
// @mmio_base: mmio base of SoundWire registers
// @irq: interrupt number
// @handle: ACPI parent handle
// @parent: parent device
// @ops: callback ops
// @dev: device implementing hwparams and free callbacks
// @link_mask: bit-wise mask listing links selected by the DSP driver
// This mask may be a subset of the one reported by the controller since
// machine-specific quirks are handled in the DSP driver.
// @clock_stop_quirks: mask array of possible behaviors requested by the
// DSP driver. The quirks are common for all links for now.
// @shim_base: sdw shim base.
// @alh_base: sdw alh base.
// @ext: extended HDaudio link support
// @mic_privacy: ACE version supports microphone privacy
// @hbus: hdac_bus pointer, needed for power management
// @eml_lock: mutex protecting shared registers in the HDaudio multi-link
// space
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_intel_res {
    pub hw_ops: *const sdw_intel_hw_ops,
    pub count: c_int,
    pub mmio_base: *mut void __iomem,
    pub irq: c_int,
    pub handle: acpi_handle,
    pub parent: *mut device,
    pub ops: *const sdw_intel_ops,
    pub dev: *mut device,
    pub link_mask: u32,
    pub clock_stop_quirks: u32,
    pub shim_base: u32,
    pub alh_base: u32,
    pub ext: bool,
    pub mic_privacy: bool,
    pub hbus: *mut hdac_bus,
    pub eml_lock: *mut mutex,
}

//
// On Intel platforms, the SoundWire IP has dependencies on power
// rails shared with the DSP, and the initialization steps are split
// in three. First an ACPI scan to check what the firmware describes
// in DSDT tables, then an allocation step (with no hardware
// configuration but with all the relevant devices created) and last
// the actual hardware configuration. The final stage is a global
// interrupt enable which is controlled by the DSP driver. Splitting
// these phases helps simplify the boot flow and make early decisions
// on e.g. which machine driver to select (I2S mode, HDaudio or
// SoundWire).
//
extern "C" {
    pub fn sdw_intel_process_wakeen_event(ctx: *mut sdw_intel_ctx);
}
extern "C" {
    pub fn sdw_intel_startup(ctx: *mut sdw_intel_ctx) -> c_int;
}
extern "C" {
    pub fn sdw_intel_exit(ctx: *mut sdw_intel_ctx);
}
extern "C" {
    pub fn sdw_intel_thread(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}

// struct intel_sdw_hw_ops - SoundWire ops for Intel platforms.
// @debugfs_init: initialize all debugfs capabilities
// @debugfs_exit: close and cleanup debugfs capabilities
// @get_link_count: fetch link count from hardware registers
// @register_dai: read all PDI information and register DAIs
// @check_clock_stop: throw error message if clock is not stopped.
// @start_bus: normal start
// @start_bus_after_reset: start after reset
// @start_bus_after_clock_stop: start after mode0 clock stop
// @stop_bus: stop all bus
// @link_power_up: power-up using chip-specific helpers
// @link_power_down: power-down with chip-specific helpers
// @shim_check_wake: check if a wake was received
// @shim_wake: enable/disable in-band wake management
// @pre_bank_switch: helper for bus management
// @post_bank_switch: helper for bus management
// @sync_arm: helper for multi-link synchronization
// @sync_go_unlocked: helper for multi-link synchronization -
// shim_lock is assumed to be locked at higher level
// @sync_go: helper for multi-link synchronization
// @sync_check_cmdsync_unlocked: helper for multi-link synchronization
// and bank switch - shim_lock is assumed to be locked at higher level
// @program_sdi: helper for codec command/control based on dev_num
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_intel_hw_ops {
    pub sdw): *mut *mut void (debugfs_init)(struct sdw_intel,
    pub sdw): *mut *mut void (debugfs_exit)(struct sdw_intel,
    pub sdw): *mut *mut int (get_link_count)(struct sdw_intel,
    pub sdw): *mut *mut int (register_dai)(struct sdw_intel,
    pub sdw): *mut *mut void (check_clock_stop)(struct sdw_intel,
    pub sdw): *mut *mut int (start_bus)(struct sdw_intel,
    pub sdw): *mut *mut int (start_bus_after_reset)(struct sdw_intel,
    pub sdw): *mut *mut int (start_bus_after_clock_stop)(struct sdw_intel,
    pub clock_stop): *mut *mut *mut int (stop_bus)(struct sdw_intel sdw, bool,
    pub sdw): *mut *mut int (link_power_up)(struct sdw_intel,
    pub sdw): *mut *mut int (link_power_down)(struct sdw_intel,
    pub sdw): *mut *mut int (shim_check_wake)(struct sdw_intel,
    pub wake_enable): *mut *mut *mut void (shim_wake)(struct sdw_intel sdw, bool,
    pub sdw): *mut *mut int (pre_bank_switch)(struct sdw_intel,
    pub sdw): *mut *mut int (post_bank_switch)(struct sdw_intel,
    pub sdw): *mut *mut void (sync_arm)(struct sdw_intel,
    pub sdw): *mut *mut int (sync_go_unlocked)(struct sdw_intel,
    pub sdw): *mut *mut int (sync_go)(struct sdw_intel,
    pub sdw): *mut *mut bool (sync_check_cmdsync_unlocked)(struct sdw_intel,
    pub dev_num): *mut *mut *mut void (program_sdi)(struct sdw_intel sdw, int,
    pub msg): *mut sdw_bpt_msg,
    pub msg): *mut *mut *mut *mut int (bpt_wait)(struct sdw_intel sdw, struct sdw_slave slave, struct sdw_bpt_msg,
}

//
// IDA min selected to allow for 5 unconstrained devices per link,
// and 6 system-unique Device Numbers for wake-capable devices.
//
pub const SDW_INTEL_DEV_NUM_IDA_MIN: c_int = 6;
//
// Max number of links supported in hardware
//
pub const SDW_INTEL_MAX_LINKS: c_int = 5;
