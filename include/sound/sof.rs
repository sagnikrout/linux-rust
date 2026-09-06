//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/sof.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//

//
// enum sof_fw_state - DSP firmware state definitions
// @SOF_FW_BOOT_NOT_STARTED:	firmware boot is not yet started
// @SOF_DSPLESS_MODE:		DSP is not used
// @SOF_FW_BOOT_PREPARE:	preparing for boot (firmware loading for exaqmple)
// @SOF_FW_BOOT_IN_PROGRESS:	firmware boot is in progress
// @SOF_FW_BOOT_FAILED:		firmware boot failed
// @SOF_FW_BOOT_READY_FAILED:	firmware booted but fw_ready op failed
// @SOF_FW_BOOT_READY_OK:	firmware booted and fw_ready op passed
// @SOF_FW_BOOT_COMPLETE:	firmware is booted up and functional
// @SOF_FW_CRASHED:		firmware crashed after successful boot
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_fw_state {
    SOF_FW_BOOT_NOT_STARTED = 0,
    SOF_DSPLESS_MODE,
    SOF_FW_BOOT_PREPARE,
    SOF_FW_BOOT_IN_PROGRESS,
    SOF_FW_BOOT_FAILED,
    SOF_FW_BOOT_READY_FAILED,
    SOF_FW_BOOT_READY_OK,
    SOF_FW_BOOT_COMPLETE,
    SOF_FW_CRASHED,
}

// DSP power states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_dsp_power_states {
    SOF_DSP_PM_D0,
    SOF_DSP_PM_D1,
    SOF_DSP_PM_D2,
    SOF_DSP_PM_D3,
}

// Definitions for multiple IPCs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc_type {
    SOF_IPC_TYPE_3,
    SOF_IPC_TYPE_4,
    SOF_IPC_TYPE_COUNT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_loadable_file_profile {
    pub ipc_type: sof_ipc_type,
    pub fw_path: *const c_char,
    pub fw_path_postfix: *const c_char,
    pub fw_name: *const c_char,
    pub fw_lib_path: *const c_char,
    pub fw_lib_path_postfix: *const c_char,
    pub tplg_path: *const c_char,
    pub tplg_name: *const c_char,
}

//
// SOF Platform data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_pdata {
    pub name: *const c_char,
    pub platform: *const c_char,
//
// PCI SSID. As PCI does not define 0 as invalid, the subsystem_id_set
// flag indicates that a value has been written to these members.
//
    pub subsystem_vendor: c_ushort,
    pub subsystem_device: c_ushort,
    pub subsystem_id_set: bool,
    pub dev: *mut device,
//
// notification callback used if the hardware initialization
// can take time or is handled in a workqueue. This callback
// can be used by the caller to e.g. enable runtime_pm
// or limit functionality until all low-level inits are
// complete.
//
    pub dev): *mut *mut void (sof_probe_complete)(struct device,
// descriptor
    pub desc: *const sof_dev_desc,
// platform's preferred IPC type and path overrides
    pub ipc_file_profile_base: sof_loadable_file_profile,
// firmware and topology filenames
    pub fw_filename_prefix: *const c_char,
    pub fw_filename: *const c_char,
    pub tplg_filename_prefix: *const c_char,
    pub tplg_filename: *const c_char,
    pub disable_function_topology: bool,
// loadable external libraries available under this directory
    pub fw_lib_prefix: *const c_char,
// machine
    pub pdev_mach: *mut platform_device,
    pub machine: *const snd_soc_acpi_mach,
    pub of_machine: *const snd_sof_of_mach,
    pub hw_pdata: *mut c_void,
    pub ipc_type: sof_ipc_type,
}

//
// Descriptor used for setting up SOF platform data. This is used when
// ACPI/PCI data is missing or mapped differently.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_dev_desc {
// list of machines using this configuration
    pub machines: *mut snd_soc_acpi_mach,
    pub of_machines: *mut snd_sof_of_mach,
// alternate list of machines using this configuration
    pub alt_machines: *mut snd_soc_acpi_mach,
    pub use_acpi_target_states: bool,
// Platform resource indexes in BAR / ACPI resources.
// Must set to -1 if not used - add new items to end
    pub resindex_lpe_base: c_int,
    pub resindex_pcicfg_base: c_int,
    pub resindex_imr_base: c_int,
    pub irqindex_host_ipc: c_int,
// IPC timeouts in ms
    pub ipc_timeout: c_int,
    pub boot_timeout: c_int,
// chip information for dsp
    pub chip_info: *const c_void,
// defaults for no codec mode
    pub nocodec_tplg_filename: *const c_char,
// information on supported IPCs
    pub ipc_supported_mask: c_uint,
    pub ipc_default: sof_ipc_type,
// The platform supports DSPless mode
    pub dspless_mode_supported: bool,
// On demand DSP booting is possible on the platform
    pub on_demand_dsp_boot: bool,
// defaults paths for firmware, library and topology files
    pub default_fw_path: [*const c_char; SOF_IPC_TYPE_COUNT],
    pub default_lib_path: [*const c_char; SOF_IPC_TYPE_COUNT],
    pub default_tplg_path: [*const c_char; SOF_IPC_TYPE_COUNT],
// default firmware name
    pub default_fw_filename: [*const c_char; SOF_IPC_TYPE_COUNT],
    pub ops: *const snd_sof_dsp_ops,
    pub sdev): *mut *mut int (ops_init)(struct snd_sof_dev,
    pub sdev): *mut *mut void (ops_free)(struct snd_sof_dev,
}

extern "C" {
    pub fn sof_dai_get_mclk(rtd: *mut snd_soc_pcm_runtime) -> c_int;
}
extern "C" {
    pub fn sof_dai_get_bclk(rtd: *mut snd_soc_pcm_runtime) -> c_int;
}
extern "C" {
    pub fn sof_dai_get_tdm_slots(rtd: *mut snd_soc_pcm_runtime) -> c_int;
}
