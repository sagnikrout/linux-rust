//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sof/ipc4-priv.h
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
// Copyright(c) 2022 Intel Corporation
//

// The DSP window indices are fixed
pub const SOF_IPC4_INBOX_WINDOW_IDX: c_int = 0;
pub const SOF_IPC4_OUTBOX_WINDOW_IDX: c_int = 1;
pub const SOF_IPC4_DEBUG_WINDOW_IDX: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc4_mtrace_type {
    SOF_IPC4_MTRACE_NOT_AVAILABLE = 0,
    SOF_IPC4_MTRACE_INTEL_CAVS_1_5,
    SOF_IPC4_MTRACE_INTEL_CAVS_1_8,
    SOF_IPC4_MTRACE_INTEL_CAVS_2,
}

//
// struct sof_ipc4_fw_module - IPC4 module info
// @sof_man4_module: Module info
// @fw_mod_cfg: Pointer to the module config start of the module
// @m_ida: Module instance identifier
// @private: Module private data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_fw_module {
    pub man4_module_entry: sof_man4_module,
    pub fw_mod_cfg: *const sof_man4_module_config,
    pub m_ida: ida,
    pub private: *mut c_void,
}

//
// struct sof_ipc4_fw_library - IPC4 library information
// @sof_fw: SOF Firmware of the library
// @id: Library ID. 0 is reserved for basefw, external libraries must have unique
// ID number between 1 and (sof_ipc4_fw_data.max_libs_count - 1)
// Note: sof_ipc4_fw_data.max_libs_count == 1 implies that external libraries
// are not supported
// @num_modules : Number of FW modules in the library
// @modules: Array of FW modules
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_fw_library {
    pub sof_fw: sof_firmware,
    pub name: *const c_char,
    pub id: u32,
    pub num_modules: c_int,
    pub modules: *mut sof_ipc4_fw_module,
}

//
// struct sof_ipc4_fw_data - IPC4-specific data
// @manifest_fw_hdr_offset: FW header offset in the manifest
// @fw_lib_xa: XArray for firmware libraries, including basefw (ID = 0)
// Used to store the FW libraries and to manage the unique IDs of the
// libraries.
// @nhlt: NHLT table either from the BIOS or the topology manifest
// @mtrace_type: mtrace type supported on the booted platform
// @mtrace_log_bytes: log bytes as reported by the firmware via fw_config reply
// @num_playback_streams: max number of playback DMAs, needed for CHAIN_DMA offset
// @num_capture_streams: max number of capture DMAs
// @max_num_pipelines: max number of pipelines
// @max_libs_count: Maximum number of libraries support by the FW including the
// base firmware
// @fw_context_save: Firmware supports full context save and restore
// @libraries_restored: The libraries have been retained during firmware boot
//
// @load_library: Callback function for platform dependent library loading
// @pipeline_state_mutex: Mutex to protect pipeline triggers, ref counts, states and deletion
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_fw_data {
    pub manifest_fw_hdr_offset: u32,
    pub fw_lib_xa: xarray,
    pub nhlt: *mut c_void,
    pub mtrace_type: sof_ipc4_mtrace_type,
    pub mtrace_log_bytes: u32,
    pub num_playback_streams: c_int,
    pub num_capture_streams: c_int,
    pub max_num_pipelines: c_int,
    pub max_libs_count: u32,
    pub fw_context_save: bool,
    pub libraries_restored: bool,
    pub reload): *mut *mut sof_ipc4_fw_library fw_lib, bool,
    pub caps): *mut sof_ipc4_intel_mic_privacy_cap,
    pub /: *mut *mut mutex pipeline_state_mutex; / protect pipeline triggers, ref counts and states,
}

extern "C" {
    pub fn sof_ipc4_set_pipeline_state(sdev: *mut snd_sof_dev, instance_id: u32, state: u32) -> c_int;
}
extern "C" {
    pub fn sof_ipc4_mtrace_update_pos(sdev: *mut snd_sof_dev, core: c_int) -> c_int;
}
extern "C" {
    pub fn sof_ipc4_complete_split_release(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn sof_ipc4_query_fw_configuration(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn sof_ipc4_reload_fw_libraries(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn sof_ipc4_mic_privacy_state_change(sdev: *mut snd_sof_dev, state: bool);
}
