//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/intel/avs/avs.h
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
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
// Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

//
// struct avs_dsp_ops - Platform-specific DSP operations
//
// @power: Power on or off DSP cores
// @reset: Enter or exit reset state on DSP cores
// @stall: Stall or run DSP cores
// @irq_handler: Top half of IPC servicing
// @irq_thread: Bottom half of IPC servicing
// @int_control: Enable or disable IPC interrupts
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_dsp_ops {
    pub bool): *const *const *const int ( power)(struct avs_dev , u32,,
    pub bool): *const *const *const int ( reset)(struct avs_dev , u32,,
    pub bool): *const *const *const int ( stall)(struct avs_dev , u32,,
    pub ): *const *const irqreturn_t ( dsp_interrupt)(struct avs_dev,
    pub bool): *const *const *const void ( int_control)(struct avs_dev ,,
    pub ): *const *const *const int ( load_basefw)(struct avs_dev , struct firmware,
    pub u32): *const *const *const *const int ( load_lib)(struct avs_dev , struct firmware ,,
    pub u32): *const *const *const *const int ( transfer_mods)(struct avs_dev , bool, struct avs_module_entry ,,
    pub ): *const *const int ( config_basefw)(struct avs_dev,
    pub ): *mut u32,
    pub u32): *const *const *const int ( log_buffer_offset)(struct avs_dev ,,
    pub ): *const *const *const int ( log_buffer_status)(struct avs_dev , union avs_notify_msg,
    pub ): *const *const *const int ( coredump)(struct avs_dev , union avs_notify_msg,
    pub bool): *const *const *const *const bool ( d0ix_toggle)(struct avs_dev , struct avs_ipc_msg ,,
    pub bool): *const *const *const int ( set_d0ix)(struct avs_dev ,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_sram_spec {
    pub base_offset: u32,
    pub window_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_hipc_spec {
    pub req_offset: u32,
    pub req_ext_offset: u32,
    pub req_busy_mask: u32,
    pub ack_offset: u32,
    pub ack_done_mask: u32,
    pub rsp_offset: u32,
    pub rsp_busy_mask: u32,
    pub ctl_offset: u32,
    pub sts_offset: u32,
}

// Platform specific descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_spec {
    pub name: *const c_char,
    pub dsp_ops: *const *const avs_dsp_ops,
    pub /: *mut *mut avs_fw_version min_fw_version; / anything below is rejected,
    pub /: *const *const u32 core_init_mask; / used during DSP boot,
    pub /: *const *const *const u64 attributes; / bitmask of AVS_PLATATTR_,
    pub sram: *const avs_sram_spec,
    pub hipc: *const avs_hipc_spec,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_fw_entry {
    pub name: *const c_char,
    pub fw: *const firmware,
    pub node: list_head,
}

//
// struct avs_dev - Intel HD-Audio driver data
//
// @dev: PCI device
// @dsp_ba: DSP bar address
// @spec: platform-specific descriptor
// @fw_cfg: Firmware configuration, obtained through FW_CONFIG message
// @hw_cfg: Hardware configuration, obtained through HW_CONFIG message
// @mods_info: Available module-types, obtained through MODULES_INFO message
// @mod_idas: Module instance ID pool, one per module-type
// @modres_mutex: For synchronizing any @mods_info updates
// @ppl_ida: Pipeline instance ID pool
// @fw_list: List of libraries loaded, including base firmware
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_dev {
    pub base: hda_bus,
    pub dev: *mut device,
    pub dsp_ba: *mut void __iomem,
    pub spec: *const avs_spec,
    pub ipc: *mut avs_ipc,
    pub fw_cfg: avs_fw_cfg,
    pub hw_cfg: avs_hw_cfg,
    pub mods_info: *mut avs_mods_info,
    pub mod_idas: *mut ida,
    pub modres_mutex: mutex,
    pub /: *mut *mut *mut void modcfg_buf; / module configuration buffer,
    pub ppl_ida: ida,
    pub fw_list: list_head,
    pub /: *mut *mut *mut int core_refs; / reference count per core,
    pub lib_names: *mut c_char,
    pub num_lp_paths: c_int,
    pub /: *mut *mut atomic_t l1sen_counter; / controls whether L1SEN should be disabled,
    pub fw_ready: completion,
    pub probe_work: work_struct,
    pub comp_list: list_head,
    pub comp_list_mutex: mutex,
    pub path_list: list_head,
    pub path_list_lock: spinlock_t,
    pub path_mutex: mutex,
    pub /: *mut *mut spinlock_t trace_lock; / serialize debug window I/O between each LOG_BUFFER_STATUS,

    pub trace_fifo: kfifo,
    pub trace_waitq: wait_queue_head_t,
    pub aging_timer_period: u32,
    pub fifo_full_timer_period: u32,
    pub /: *mut *mut u32 logged_resources; / context dependent: core or library,
    pub debugfs_root: *mut dentry,
// probes
    pub extractor: *mut hdac_ext_stream,
    pub num_probe_streams: c_uint,

}

// from hda_bus to avs_dev

// from hdac_bus to avs_dev

// from device to avs_dev

extern "C" {
    pub fn avs_dsp_core_power(adev: *mut avs_dev, core_mask: u32, power: bool) -> c_int;
}
extern "C" {
    pub fn avs_dsp_core_reset(adev: *mut avs_dev, core_mask: u32, reset: bool) -> c_int;
}
extern "C" {
    pub fn avs_dsp_core_stall(adev: *mut avs_dev, core_mask: u32, stall: bool) -> c_int;
}
extern "C" {
    pub fn avs_dsp_core_enable(adev: *mut avs_dev, core_mask: u32) -> c_int;
}
extern "C" {
    pub fn avs_dsp_core_disable(adev: *mut avs_dev, core_mask: u32) -> c_int;
}
// Inter Process Communication
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_ipc_msg {
    pub header: u64,
    pub glb: avs_global_msg,
    pub rsp: avs_reply_msg,
}

//
// struct avs_ipc - DSP IPC context
//
// @dev: PCI device
// @rx: Reply message cache
// @default_timeout_ms: default message timeout in MS
// @ready: whether firmware is ready and communication is open
// @rx_completed: whether RX for previously sent TX has been received
// @rx_lock: for serializing manipulation of rx_* fields
// @msg_lock: for synchronizing request handling
// @done_completion: DONE-part of IPC i.e. ROM and ACKs from FW
// @busy_completion: BUSY-part of IPC i.e. receiving responses from FW
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_ipc {
    pub dev: *mut device,
    pub rx: avs_ipc_msg,
    pub default_timeout_ms: u32,
    pub ready: bool,
    pub recovering: core::sync::atomic::AtomicI32,
    pub rx_completed: bool,
    pub rx_lock: spinlock_t,
    pub msg_mutex: mutex,
    pub done_completion: completion,
    pub busy_completion: completion,
    pub recovery_work: work_struct,
    pub d0ix_work: delayed_work,
    pub d0ix_disable_depth: core::sync::atomic::AtomicI32,
    pub in_d0ix: bool,
}

//
// IPC handlers may return positive value (firmware error code) what denotes
// successful HOST <-> DSP communication yet failure to process specific request.
//
// Below macro converts returned value to linux kernel error code.
// All IPC callers MUST use it as soon as firmware error code is consumed.
//

extern "C" {
    pub fn avs_dsp_process_response(adev: *mut avs_dev, header: u64);
}
// Two variants below are for messages that control DSP power states.
extern "C" {
    pub fn avs_dsp_send_rom_msg(adev: *mut avs_dev, request: *mut avs_ipc_msg, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn avs_dsp_interrupt_control(adev: *mut avs_dev, enable: bool);
}
extern "C" {
    pub fn avs_ipc_init(ipc: *mut avs_ipc, dev: *mut device) -> c_int;
}
extern "C" {
    pub fn avs_ipc_block(ipc: *mut avs_ipc);
}
extern "C" {
    pub fn avs_dsp_disable_d0ix(adev: *mut avs_dev) -> c_int;
}
extern "C" {
    pub fn avs_dsp_enable_d0ix(adev: *mut avs_dev) -> c_int;
}
extern "C" {
    pub fn avs_mtl_core_power(adev: *mut avs_dev, core_mask: u32, power: bool) -> c_int;
}
extern "C" {
    pub fn avs_mtl_core_reset(adev: *mut avs_dev, core_mask: u32, power: bool) -> c_int;
}
extern "C" {
    pub fn avs_mtl_core_stall(adev: *mut avs_dev, core_mask: u32, stall: bool) -> c_int;
}
extern "C" {
    pub fn avs_lnl_core_stall(adev: *mut avs_dev, core_mask: u32, stall: bool) -> c_int;
}
extern "C" {
    pub fn avs_mtl_interrupt_control(adev: *mut avs_dev, enable: bool);
}
extern "C" {
    pub fn avs_skl_ipc_interrupt(adev: *mut avs_dev);
}
extern "C" {
    pub fn avs_cnl_dsp_interrupt(adev: *mut avs_dev) -> irqreturn_t;
}
extern "C" {
    pub fn avs_mtl_dsp_interrupt(adev: *mut avs_dev) -> irqreturn_t;
}
extern "C" {
    pub fn avs_skl_log_buffer_offset(adev: *mut avs_dev, core: u32) -> c_int;
}
extern "C" {
    pub fn avs_icl_log_buffer_offset(adev: *mut avs_dev, core: u32) -> c_int;
}
extern "C" {
    pub fn avs_apl_log_buffer_status(adev: *mut avs_dev, msg: *mut avs_notify_msg) -> c_int;
}
extern "C" {
    pub fn avs_apl_coredump(adev: *mut avs_dev, msg: *mut avs_notify_msg) -> c_int;
}
extern "C" {
    pub fn avs_apl_d0ix_toggle(adev: *mut avs_dev, tx: *mut avs_ipc_msg, wake: bool) -> bool;
}
extern "C" {
    pub fn avs_icl_d0ix_toggle(adev: *mut avs_dev, tx: *mut avs_ipc_msg, wake: bool) -> bool;
}
extern "C" {
    pub fn avs_apl_set_d0ix(adev: *mut avs_dev, enable: bool) -> c_int;
}
extern "C" {
    pub fn avs_icl_set_d0ix(adev: *mut avs_dev, enable: bool) -> c_int;
}
// Firmware resources management
extern "C" {
    pub fn avs_get_module_entry(adev: *mut avs_dev, uuid: *const guid_t, entry: *mut avs_module_entry) -> c_int;
}
extern "C" {
    pub fn avs_get_module_id_entry(adev: *mut avs_dev, module_id: u32, entry: *mut avs_module_entry) -> c_int;
}
extern "C" {
    pub fn avs_get_module_id(adev: *mut avs_dev, uuid: *const guid_t) -> c_int;
}
extern "C" {
    pub fn avs_is_module_ida_empty(adev: *mut avs_dev, module_id: u32) -> bool;
}
extern "C" {
    pub fn avs_module_info_init(adev: *mut avs_dev, purge: bool) -> c_int;
}
extern "C" {
    pub fn avs_module_info_free(adev: *mut avs_dev);
}
extern "C" {
    pub fn avs_module_id_alloc(adev: *mut avs_dev, module_id: u16) -> c_int;
}
extern "C" {
    pub fn avs_module_id_free(adev: *mut avs_dev, module_id: u16, instance_id: u8);
}
extern "C" {
    pub fn avs_request_firmware(adev: *mut avs_dev, fw_p: *const firmware, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn avs_release_last_firmware(adev: *mut avs_dev);
}
extern "C" {
    pub fn avs_release_firmwares(adev: *mut avs_dev);
}
extern "C" {
    pub fn avs_dsp_delete_pipeline(adev: *mut avs_dev, instance_id: u8) -> c_int;
}
// Firmware loading
extern "C" {
    pub fn avs_hda_clock_gating_enable(adev: *mut avs_dev, enable: bool);
}
extern "C" {
    pub fn avs_hda_power_gating_enable(adev: *mut avs_dev, enable: bool);
}
extern "C" {
    pub fn avs_hda_l1sen_enable(adev: *mut avs_dev, enable: bool);
}
extern "C" {
    pub fn avs_dsp_load_libraries(adev: *mut avs_dev, libs: *mut avs_tplg_library, num_libs: u32) -> c_int;
}
extern "C" {
    pub fn avs_dsp_boot_firmware(adev: *mut avs_dev, purge: bool) -> c_int;
}
extern "C" {
    pub fn avs_dsp_first_boot_firmware(adev: *mut avs_dev) -> c_int;
}
extern "C" {
    pub fn avs_cldma_load_basefw(adev: *mut avs_dev, fw: *mut firmware) -> c_int;
}
extern "C" {
    pub fn avs_cldma_load_library(adev: *mut avs_dev, lib: *mut firmware, id: u32) -> c_int;
}
extern "C" {
    pub fn avs_hda_load_basefw(adev: *mut avs_dev, fw: *mut firmware) -> c_int;
}
extern "C" {
    pub fn avs_hda_load_library(adev: *mut avs_dev, lib: *mut firmware, id: u32) -> c_int;
}
extern "C" {
    pub fn avs_icl_load_basefw(adev: *mut avs_dev, fw: *mut firmware) -> c_int;
}
// Soc component members
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_soc_component {
    pub base: *mut snd_soc_component,
    pub tplg: *mut avs_tplg,
    pub node: list_head,
}

extern "C" {
    pub fn avs_register_dmic_component(adev: *mut avs_dev, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn avs_register_hda_component(adev: *mut avs_dev, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn avs_register_all_boards(adev: *mut avs_dev) -> c_int;
}
extern "C" {
    pub fn avs_unregister_all_boards(adev: *mut avs_dev);
}
extern "C" {
    pub fn avs_parse_sched_cfg(adev: *mut avs_dev, buf: *const c_char, len: usize) -> c_int;
}
// Filesystems integration
