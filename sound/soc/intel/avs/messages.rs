//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/intel/avs/messages.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_msg_target {
    AVS_FW_GEN_MSG = 0,
    AVS_MOD_MSG = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_msg_direction {
    AVS_MSG_REQUEST = 0,
    AVS_MSG_REPLY = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_global_msg_type {
    AVS_GLB_ROM_CONTROL = 1,
    AVS_GLB_LOAD_MULTIPLE_MODULES = 15,
    AVS_GLB_UNLOAD_MULTIPLE_MODULES = 16,
    AVS_GLB_CREATE_PIPELINE = 17,
    AVS_GLB_DELETE_PIPELINE = 18,
    AVS_GLB_SET_PIPELINE_STATE = 19,
    AVS_GLB_GET_PIPELINE_STATE = 20,
    AVS_GLB_LOAD_LIBRARY = 24,
    AVS_GLB_NOTIFICATION = 27,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union avs_global_msg {
    pub val: u64,
    pub primary: u32,
    pub rsvd:24: u32,
    pub global_msg_type:5: u32,
    pub msg_direction:1: u32,
    pub msg_target:1: u32,
}

// set boot config
// module loading
// pipeline management
// library loading
// pipeline management
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_tlv {
    pub type: u32,
    pub length: u32,
    pub value: [u32; ],
    pub __packed: },
    pub 8): static_assert(sizeof(struct avs_tlv) ==,

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_module_msg_type {
    AVS_MOD_INIT_INSTANCE = 0,
    AVS_MOD_LARGE_CONFIG_GET = 3,
    AVS_MOD_LARGE_CONFIG_SET = 4,
    AVS_MOD_BIND = 5,
    AVS_MOD_UNBIND = 6,
    AVS_MOD_SET_DX = 7,
    AVS_MOD_SET_D0IX = 8,
    AVS_MOD_DELETE_INSTANCE = 11,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union avs_module_msg {
    pub val: u64,
    pub primary: u32,
    pub module_id:16: u32,
    pub instance_id:8: u32,
    pub module_msg_type:5: u32,
    pub msg_direction:1: u32,
    pub msg_target:1: u32,
}

// pre-IceLake
// IceLake and onwards
pub const AVS_IPC_NOT_SUPPORTED: c_int = 15;
#[repr(C)]
#[derive(Copy, Clone)]
pub union avs_reply_msg {
    pub val: u64,
    pub primary: u32,
    pub status:24: u32,
    pub global_msg_type:5: u32,
    pub msg_direction:1: u32,
    pub msg_target:1: u32,
}

// module loading
// pipeline management
// module management
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_notify_msg_type {
    AVS_NOTIFY_PHRASE_DETECTED = 4,
    AVS_NOTIFY_RESOURCE_EVENT = 5,
    AVS_NOTIFY_LOG_BUFFER_STATUS = 6,
    AVS_NOTIFY_FW_READY = 8,
    AVS_NOTIFY_EXCEPTION_CAUGHT = 10,
    AVS_NOTIFY_MODULE_EVENT = 12,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union avs_notify_msg {
    pub val: u64,
    pub primary: u32,
    pub rsvd:16: u32,
    pub notify_msg_type:8: u32,
    pub global_msg_type:5: u32,
    pub msg_direction:1: u32,
    pub msg_target:1: u32,
}

// Notification types
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_notify_voice_data {
    pub kpd_score: u16,
    pub reserved: u16,
    pub __packed: },
    pub 4): static_assert(sizeof(struct avs_notify_voice_data) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_notify_res_data {
    pub resource_type: u32,
    pub resource_id: u32,
    pub event_type: u32,
    pub reserved: u32,
    pub data: [u32; 6],
    pub __packed: },
    pub 40): static_assert(sizeof(struct avs_notify_res_data) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_notify_mod_data {
    pub module_instance_id: u32,
    pub event_id: u32,
    pub data_size: u32,
    pub data: [u32; ],
    pub __packed: },
    pub 12): static_assert(sizeof(struct avs_notify_mod_data) ==,
// ROM messages
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_rom_control_msg_type {
    AVS_ROM_SET_BOOT_CONFIG = 0,
}

    pub purge): *mut *mut int avs_ipc_set_boot_config(struct avs_dev adev, u32 dma_id, u32,
// Code loading messages
    pub num_mod_ids): *mut *mut *mut int avs_ipc_load_modules(struct avs_dev adev, u16 mod_ids, u32,
    pub num_mod_ids): *mut *mut *mut int avs_ipc_unload_modules(struct avs_dev adev, u16 mod_ids, u32,
    pub lib_id): *mut *mut int avs_ipc_load_library(struct avs_dev adev, u32 dma_id, u32,
// Pipeline management messages
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_pipeline_state {
    AVS_PPL_STATE_INVALID,
    AVS_PPL_STATE_UNINITIALIZED,
    AVS_PPL_STATE_RESET,
    AVS_PPL_STATE_PAUSED,
    AVS_PPL_STATE_RUNNING,
}

    pub attributes): u8 instance_id, bool lp, u16,
    pub instance_id): *mut *mut int avs_ipc_delete_pipeline(struct avs_dev adev, u8,
    pub state): avs_pipeline_state,
    pub state): *mut avs_pipeline_state,
// Module management messages
    pub param_size): *mut *mut void param, u32,
    pub instance_id): *mut *mut int avs_ipc_delete_instance(struct avs_dev adev, u16 module_id, u8,
    pub src_queue): u8 dst_queue, u8,
    pub src_queue): u8 dst_queue, u8,
    pub request_size): *mut *mut u8 request, size_t,
    pub reply_size): *mut *mut *mut u8 reply_data, size_t,
// DSP cores and domains power management messages
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_dxstate_info {
    pub /: *mut *mut u32 core_mask; / which cores are subject for power transition,
    pub /: *mut *mut u32 dx_mask; / bit[n]=1 core n goes to D0, bit[n]=0 it goes to D3,
    pub __packed: },
    pub 8): static_assert(sizeof(struct avs_dxstate_info) ==,
    pub powerup): *mut *mut int avs_ipc_set_dx(struct avs_dev adev, u32 core_mask, bool,
    pub streaming): *mut *mut int avs_ipc_set_d0ix(struct avs_dev adev, bool enable_pg, bool,
// Base-firmware runtime parameters
pub const AVS_BASEFW_MOD_ID: c_int = 0;
pub const AVS_BASEFW_INST_ID: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_basefw_runtime_param {
    AVS_BASEFW_ENABLE_LOGS = 6,
    AVS_BASEFW_FIRMWARE_CONFIG = 7,
    AVS_BASEFW_HARDWARE_CONFIG = 8,
    AVS_BASEFW_MODULES_INFO = 9,
    AVS_BASEFW_LIBRARIES_INFO = 16,
    AVS_BASEFW_SYSTEM_TIME = 20,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_log_enable {
    AVS_LOG_DISABLE = 0,
    AVS_LOG_ENABLE = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_skl_log_priority {
    AVS_SKL_LOG_CRITICAL = 1,
    AVS_SKL_LOG_HIGH,
    AVS_SKL_LOG_MEDIUM,
    AVS_SKL_LOG_LOW,
    AVS_SKL_LOG_VERBOSE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_skl_log_state {
    pub enable: u32,
    pub min_priority: u32,
    pub __packed: },
    pub 8): static_assert(sizeof(struct avs_skl_log_state) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_skl_log_state_info {
    pub core_mask: u32,
    pub logs_core: [avs_skl_log_state; ],
    pub __packed: },
    pub 4): static_assert(sizeof(struct avs_skl_log_state_info) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_apl_log_state_info {
    pub aging_timer_period: u32,
    pub fifo_full_timer_period: u32,
    pub core_mask: u32,
    pub logs_core: [avs_skl_log_state; ],
    pub __packed: },
    pub 12): static_assert(sizeof(struct avs_apl_log_state_info) ==,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_icl_log_priority {
    AVS_ICL_LOG_CRITICAL = 0,
    AVS_ICL_LOG_HIGH,
    AVS_ICL_LOG_MEDIUM,
    AVS_ICL_LOG_LOW,
    AVS_ICL_LOG_VERBOSE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_icl_log_source {
    AVS_ICL_LOG_INFRA = 0,
    AVS_ICL_LOG_HAL,
    AVS_ICL_LOG_MODULE,
    AVS_ICL_LOG_AUDIO,
    AVS_ICL_LOG_SENSING,
    AVS_ICL_LOG_ULP_INFRA,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_icl_log_state_info {
    pub aging_timer_period: u32,
    pub fifo_full_timer_period: u32,
    pub enable: u32,
    pub logs_priorities_mask: [u32; ],
    pub __packed: },
    pub 12): static_assert(sizeof(struct avs_icl_log_state_info) ==,
    pub size): *mut *mut *mut int avs_ipc_set_enable_logs(struct avs_dev adev, u8 log_info, size_t,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_fw_version {
    pub major: u16,
    pub minor: u16,
    pub hotfix: u16,
    pub build: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_fw_cfg_params {
    AVS_FW_CFG_FW_VERSION = 0,
    AVS_FW_CFG_MEMORY_RECLAIMED,
    AVS_FW_CFG_SLOW_CLOCK_FREQ_HZ,
    AVS_FW_CFG_FAST_CLOCK_FREQ_HZ,
    AVS_FW_CFG_DMA_BUFFER_CONFIG,
    AVS_FW_CFG_ALH_SUPPORT_LEVEL,
    AVS_FW_CFG_IPC_DL_MAILBOX_BYTES,
    AVS_FW_CFG_IPC_UL_MAILBOX_BYTES,
    AVS_FW_CFG_TRACE_LOG_BYTES,
    AVS_FW_CFG_MAX_PPL_COUNT,
    AVS_FW_CFG_MAX_ASTATE_COUNT,
    AVS_FW_CFG_MAX_MODULE_PIN_COUNT,
    AVS_FW_CFG_MODULES_COUNT,
    AVS_FW_CFG_MAX_MOD_INST_COUNT,
    AVS_FW_CFG_MAX_LL_TASKS_PER_PRI_COUNT,
    AVS_FW_CFG_LL_PRI_COUNT,
    AVS_FW_CFG_MAX_DP_TASKS_COUNT,
    AVS_FW_CFG_MAX_LIBS_COUNT,
    AVS_FW_CFG_SCHEDULER_CONFIG,
    AVS_FW_CFG_XTAL_FREQ_HZ,
    AVS_FW_CFG_CLOCKS_CONFIG,
    AVS_FW_CFG_RESERVED,
    AVS_FW_CFG_POWER_GATING_POLICY,
    AVS_FW_CFG_ASSERT_MODE,
    AVS_FW_CFG_RESERVED2,
    AVS_FW_CFG_BUS_HARDWARE_ID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_fw_cfg {
    pub fw_version: avs_fw_version,
    pub memory_reclaimed: u32,
    pub slow_clock_freq_hz: u32,
    pub fast_clock_freq_hz: u32,
    pub alh_support: u32,
    pub ipc_dl_mailbox_bytes: u32,
    pub ipc_ul_mailbox_bytes: u32,
    pub trace_log_bytes: u32,
    pub max_ppl_count: u32,
    pub max_astate_count: u32,
    pub max_module_pin_count: u32,
    pub modules_count: u32,
    pub max_mod_inst_count: u32,
    pub max_ll_tasks_per_pri_count: u32,
    pub ll_pri_count: u32,
    pub max_dp_tasks_count: u32,
    pub max_libs_count: u32,
    pub xtal_freq_hz: u32,
    pub power_gating_policy: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_bus_hwid {
    pub device: u32,
    pub subsystem: u32,
    pub revision: u8,
}

extern "C" {
    pub fn avs_ipc_get_fw_config(adev: *mut avs_dev, cfg: *mut avs_fw_cfg) -> c_int;
}
extern "C" {
    pub fn avs_ipc_set_fw_config(adev: *mut avs_dev, num_tlvs: usize, ...) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_hw_cfg_params {
    AVS_HW_CFG_AVS_VER,
    AVS_HW_CFG_DSP_CORES,
    AVS_HW_CFG_MEM_PAGE_BYTES,
    AVS_HW_CFG_TOTAL_PHYS_MEM_PAGES,
    AVS_HW_CFG_I2S_CAPS,
    AVS_HW_CFG_GPDMA_CAPS,
    AVS_HW_CFG_GATEWAY_COUNT,
    AVS_HW_CFG_HP_EBB_COUNT,
    AVS_HW_CFG_LP_EBB_COUNT,
    AVS_HW_CFG_EBB_SIZE_BYTES,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_iface_version {
    AVS_AVS_VER_1_5 = 0x10005,
    AVS_AVS_VER_1_8 = 0x10008,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_i2s_version {
    AVS_I2S_VER_15_SKYLAKE   = 0x00000,
    AVS_I2S_VER_15_BROXTON   = 0x10000,
    AVS_I2S_VER_15_BROXTON_P = 0x20000,
    AVS_I2S_VER_18_KBL_CNL   = 0x30000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_i2s_caps {
    pub i2s_version: u32,
    pub ctrl_count: u32,
    pub ctrl_base_addr: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_hw_cfg {
    pub avs_version: u32,
    pub dsp_cores: u32,
    pub mem_page_bytes: u32,
    pub total_phys_mem_pages: u32,
    pub i2s_caps: avs_i2s_caps,
    pub gateway_count: u32,
    pub hp_ebb_count: u32,
    pub lp_ebb_count: u32,
    pub ebb_size_bytes: u32,
}

extern "C" {
    pub fn avs_ipc_get_hw_config(adev: *mut avs_dev, cfg: *mut avs_hw_cfg) -> c_int;
}
pub const AVS_MODULE_LOAD_TYPE_BUILTIN: c_int = 0;
pub const AVS_MODULE_LOAD_TYPE_LOADABLE: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_module_type {
    pub load_type:4: u32,
    pub auto_start:1: u32,
    pub domain_ll:1: u32,
    pub domain_dp:1: u32,
    pub lib_code:1: u32,
    pub rsvd:24: u32,
    pub __packed: },
    pub 4): static_assert(sizeof(struct avs_module_type) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub union avs_segment_flags {
    pub ul: u32,
    pub contents:1: u32,
    pub alloc:1: u32,
    pub load:1: u32,
    pub readonly:1: u32,
    pub code:1: u32,
    pub data:1: u32,
    pub rsvd_1:2: u32,
    pub type:4: u32,
    pub rsvd_2:4: u32,
    pub length:16: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_segment_desc {
    pub flags: avs_segment_flags,
    pub v_base_addr: u32,
    pub file_offset: u32,
    pub __packed: },
    pub 12): static_assert(sizeof(struct avs_segment_desc) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_module_entry {
    pub module_id: u16,
    pub state_flags: u16,
    pub name: [u8; 8],
    pub uuid: guid_t,
    pub type: avs_module_type,
    pub hash: [u8; 32],
    pub entry_point: u32,
    pub cfg_offset: u16,
    pub cfg_count: u16,
    pub affinity_mask: u32,
    pub instance_max_count: u16,
    pub instance_bss_size: u16,
    pub segments: [avs_segment_desc; 3],
    pub __packed: },
    pub 116): static_assert(sizeof(struct avs_module_entry) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_mods_info {
    pub count: u32,
    pub entries: [avs_module_entry; ],
    pub __packed: },
    pub 4): static_assert(sizeof(struct avs_mods_info) ==,
    pub AVS_MODULE_STATE_LOADED: mentry->state_flags &,
    pub info): *mut *mut int avs_ipc_get_modules_info(struct avs_dev adev, struct avs_mods_info,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_sys_time {
    pub val_l: u32,
    pub val_u: u32,
    pub __packed: },
    pub 8): static_assert(sizeof(struct avs_sys_time) ==,
    pub adev): *mut int avs_ipc_set_system_time(struct avs_dev,
// Module configuration

// channel map
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_channel_index {
    AVS_CHANNEL_LEFT = 0,
    AVS_CHANNEL_RIGHT = 1,
    AVS_CHANNEL_CENTER = 2,
    AVS_CHANNEL_LEFT_SURROUND = 3,
    AVS_CHANNEL_CENTER_SURROUND = 3,
    AVS_CHANNEL_RIGHT_SURROUND = 4,
    AVS_CHANNEL_LFE = 7,
    AVS_CHANNEL_INVALID = 0xF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_channel_config {
    AVS_CHANNEL_CONFIG_MONO = 0,
    AVS_CHANNEL_CONFIG_STEREO = 1,
    AVS_CHANNEL_CONFIG_2_1 = 2,
    AVS_CHANNEL_CONFIG_3_0 = 3,
    AVS_CHANNEL_CONFIG_3_1 = 4,
    AVS_CHANNEL_CONFIG_QUATRO = 5,
    AVS_CHANNEL_CONFIG_4_0 = 6,
    AVS_CHANNEL_CONFIG_5_0 = 7,
    AVS_CHANNEL_CONFIG_5_1 = 8,
    AVS_CHANNEL_CONFIG_DUAL_MONO = 9,
    AVS_CHANNEL_CONFIG_I2S_DUAL_STEREO_0 = 10,
    AVS_CHANNEL_CONFIG_I2S_DUAL_STEREO_1 = 11,
    AVS_CHANNEL_CONFIG_7_1 = 12,
    AVS_CHANNEL_CONFIG_INVALID
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_interleaving {
    AVS_INTERLEAVING_PER_CHANNEL = 0,
    AVS_INTERLEAVING_PER_SAMPLE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_sample_type {
    AVS_SAMPLE_TYPE_INT_MSB = 0,
    AVS_SAMPLE_TYPE_INT_LSB = 1,
    AVS_SAMPLE_TYPE_INT_SIGNED = 2,
    AVS_SAMPLE_TYPE_INT_UNSIGNED = 3,
    AVS_SAMPLE_TYPE_FLOAT = 4,
}

pub const AVS_COEFF_CHANNELS_MAX: c_int = 8;

pub const AVS_CHANNELS_MAX: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_audio_format {
    pub sampling_freq: u32,
    pub bit_depth: u32,
    pub channel_map: u32,
    pub channel_config: u32,
    pub interleaving: u32,
    pub num_channels:8: u32,
    pub valid_bit_depth:8: u32,
    pub sample_type:8: u32,
    pub reserved:8: u32,
    pub __packed: },
    pub 24): static_assert(sizeof(struct avs_audio_format) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_modcfg_base {
    pub cpc: u32,
    pub ibs: u32,
    pub obs: u32,
    pub is_pages: u32,
    pub audio_fmt: avs_audio_format,
    pub __packed: },
    pub 40): static_assert(sizeof(struct avs_modcfg_base) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_pin_format {
    pub pin_index: u32,
    pub iobs: u32,
    pub audio_fmt: avs_audio_format,
    pub __packed: },
    pub 32): static_assert(sizeof(struct avs_pin_format) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_modcfg_ext {
    pub base: avs_modcfg_base,
    pub num_input_pins: u16,
    pub num_output_pins: u16,
    pub reserved: [u8; 12],
// input pin formats followed by output ones
    pub pin_fmts: [avs_pin_format; ],
    pub __packed: },
    pub 56): static_assert(sizeof(struct avs_modcfg_ext) ==,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_dma_type {
    AVS_DMA_HDA_HOST_OUTPUT = 0,
    AVS_DMA_HDA_HOST_INPUT = 1,
    AVS_DMA_HDA_LINK_OUTPUT = 8,
    AVS_DMA_HDA_LINK_INPUT = 9,
    AVS_DMA_DMIC_LINK_INPUT = 11,
    AVS_DMA_I2S_LINK_OUTPUT = 12,
    AVS_DMA_I2S_LINK_INPUT = 13,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union avs_virtual_index {
    pub val: u8,
    pub time_slot:4: u8,
    pub instance:4: u8,
    pub i2s: },
    pub queue_id:3: u8,
    pub time_slot:2: u8,
    pub instance:3: u8,
    pub dmic: },
    pub __packed: },
    pub 1): static_assert(sizeof(union avs_virtual_index) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub union avs_connector_node_id {
    pub val: u32,
    pub vindex:8: u32,
    pub dma_type:5: u32,
    pub rsvd:19: u32,
}

pub const INVALID_PIPELINE_ID: c_uint = 0xFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub union avs_gtw_attributes {
    pub val: u32,
    pub lp_buffer_alloc:1: u32,
    pub rsvd:31: u32,
}

pub const AVS_GTW_DMA_CONFIG_ID: c_uint = 0x1000;
pub const AVS_DMA_METHOD_HDA: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_dma_device_stream_channel_map {
    pub device_address: u32,
    pub channel_map: u32,
    pub __packed: },
    pub 8): static_assert(sizeof(struct avs_dma_device_stream_channel_map) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_dma_stream_channel_map {
    pub device_count: u32,
    pub map: [avs_dma_device_stream_channel_map; 16],
    pub __packed: },
    pub 132): static_assert(sizeof(struct avs_dma_stream_channel_map) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_dma_cfg {
    pub dma_method: u8,
    pub pre_allocated: u8,
    pub rsvd: u16,
    pub dma_channel_id: u32,
    pub stream_id: u32,
    pub map: avs_dma_stream_channel_map,
    pub config_size: u32,
    pub __counted_by(config_size): u8 config[],
    pub __packed: },
    pub 148): static_assert(sizeof(struct avs_dma_cfg) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_copier_gtw_cfg {
    pub node_id: avs_connector_node_id,
    pub dma_buffer_size: u32,
    pub config_length: u32,
    pub attrs: avs_gtw_attributes,
    pub blob): DECLARE_FLEX_ARRAY(u32,,
    pub config: },
    pub __packed: },
    pub 16): static_assert(sizeof(struct avs_copier_gtw_cfg) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_copier_cfg {
    pub base: avs_modcfg_base,
    pub out_fmt: avs_audio_format,
    pub feature_mask: u32,
    pub gtw_cfg: avs_copier_gtw_cfg,
    pub __packed: },
    pub 84): static_assert(sizeof(struct avs_copier_cfg) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_volume_cfg {
    pub channel_id: u32,
    pub target_volume: u32,
    pub curve_type: u32,
    pub /: *mut *mut u32 reserved; / alignment,
    pub curve_duration: u64,
    pub __packed: },
    pub 24): static_assert(sizeof(struct avs_volume_cfg) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_mute_cfg {
    pub channel_id: u32,
    pub mute: u32,
    pub curve_type: u32,
    pub /: *mut *mut u32 reserved; / alignment,
    pub curve_duration: u64,
    pub __packed: },
    pub 24): static_assert(sizeof(struct avs_mute_cfg) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_peakvol_cfg {
    pub base: avs_modcfg_base,
    pub vols: [avs_volume_cfg; ],
    pub __packed: },
    pub 40): static_assert(sizeof(struct avs_peakvol_cfg) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_micsel_cfg {
    pub base: avs_modcfg_base,
    pub out_fmt: avs_audio_format,
    pub __packed: },
    pub 64): static_assert(sizeof(struct avs_micsel_cfg) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_mux_cfg {
    pub base: avs_modcfg_base,
    pub ref_fmt: avs_audio_format,
    pub out_fmt: avs_audio_format,
    pub __packed: },
    pub 88): static_assert(sizeof(struct avs_mux_cfg) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_updown_mixer_cfg {
    pub base: avs_modcfg_base,
    pub out_channel_config: u32,
    pub coefficients_select: u32,
    pub coefficients: [i32; AVS_COEFF_CHANNELS_MAX],
    pub channel_map: u32,
    pub __packed: },
    pub 84): static_assert(sizeof(struct avs_updown_mixer_cfg) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_src_cfg {
    pub base: avs_modcfg_base,
    pub out_freq: u32,
    pub __packed: },
    pub 44): static_assert(sizeof(struct avs_src_cfg) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_probe_gtw_cfg {
    pub node_id: avs_connector_node_id,
    pub dma_buffer_size: u32,
    pub __packed: },
    pub 8): static_assert(sizeof(struct avs_probe_gtw_cfg) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_probe_cfg {
    pub base: avs_modcfg_base,
    pub gtw_cfg: avs_probe_gtw_cfg,
    pub __packed: },
    pub 48): static_assert(sizeof(struct avs_probe_cfg) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_aec_cfg {
    pub base: avs_modcfg_base,
    pub ref_fmt: avs_audio_format,
    pub out_fmt: avs_audio_format,
    pub cpc_lp_mode: u32,
    pub __packed: },
    pub 92): static_assert(sizeof(struct avs_aec_cfg) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_asrc_cfg {
    pub base: avs_modcfg_base,
    pub out_freq: u32,
    pub mode:2: u32,
    pub rsvd2:2: u32,
    pub disable_jitter_buffer:1: u32,
    pub rsvd3:27: u32,
    pub __packed: },
    pub 48): static_assert(sizeof(struct avs_asrc_cfg) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_wov_cfg {
    pub base: avs_modcfg_base,
    pub cpc_lp_mode: u32,
    pub __packed: },
    pub 44): static_assert(sizeof(struct avs_wov_cfg) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_whm_cfg {
    pub base: avs_modcfg_base,
// Audio format for output pin 0
    pub ref_fmt: avs_audio_format,
    pub out_fmt: avs_audio_format,
    pub wake_tick_period: u32,
    pub gtw_cfg: avs_copier_gtw_cfg,
    pub __packed: },
    pub 108): static_assert(sizeof(struct avs_whm_cfg) ==,
// Module runtime parameters
pub const AVS_VENDOR_CONFIG: c_uint = 0xFF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_copier_runtime_param {
    AVS_COPIER_SET_SINK_FORMAT = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_copier_sink_format {
    pub sink_id: u32,
    pub src_fmt: avs_audio_format,
    pub sink_fmt: avs_audio_format,
    pub __packed: },
    pub 52): static_assert(sizeof(struct avs_copier_sink_format) ==,
    pub sink_fmt): *const avs_audio_format,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_peakvol_runtime_param {
    AVS_PEAKVOL_VOLUME = 0,
    AVS_PEAKVOL_MUTE = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_audio_curve_type {
    AVS_AUDIO_CURVE_NONE = 0,
    AVS_AUDIO_CURVE_WINDOWS_FADE = 1,
}

    pub num_vols): *mut *mut *mut avs_volume_cfg vols, size_t,
    pub vol): *mut avs_volume_cfg,
    pub num_vols): *mut *mut avs_volume_cfg vols, size_t,
    pub num_mutes): *mut *mut *mut avs_mute_cfg mutes, size_t,
    pub mute): *mut avs_mute_cfg,
    pub num_mutes): *mut *mut avs_mute_cfg mutes, size_t,
pub const AVS_PROBE_INST_ID: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_probe_runtime_param {
    AVS_PROBE_INJECTION_DMA = 1,
    AVS_PROBE_INJECTION_DMA_DETACH,
    AVS_PROBE_POINTS,
    AVS_PROBE_POINTS_DISCONNECT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_probe_dma {
    pub node_id: avs_connector_node_id,
    pub dma_buffer_size: u32,
    pub __packed: },
    pub 8): static_assert(sizeof(struct avs_probe_dma) ==,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_probe_type {
    AVS_PROBE_TYPE_INPUT = 0,
    AVS_PROBE_TYPE_OUTPUT,
    AVS_PROBE_TYPE_INTERNAL
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union avs_probe_point_id {
    pub value: u32,
    pub module_id:16: u32,
    pub instance_id:8: u32,
    pub type:2: u32,
    pub index:6: u32,
    pub id: },
    pub __packed: },
    pub 4): static_assert(sizeof(union avs_probe_point_id) ==,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_connection_purpose {
    AVS_CONNECTION_PURPOSE_EXTRACT = 0,
    AVS_CONNECTION_PURPOSE_INJECT,
    AVS_CONNECTION_PURPOSE_INJECT_REEXTRACT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_probe_point_desc {
    pub id: avs_probe_point_id,
    pub purpose: u32,
    pub node_id: avs_connector_node_id,
    pub __packed: },
    pub 12): static_assert(sizeof(struct avs_probe_point_desc) ==,
    pub num_dmas): *mut *mut *mut *mut int avs_ipc_probe_get_dma(struct avs_dev adev, struct avs_probe_dma dmas, size_t,
    pub num_dmas): *mut *mut *mut int avs_ipc_probe_attach_dma(struct avs_dev adev, struct avs_probe_dma dmas, size_t,
    pub num_node_ids): usize,
    pub num_descs): *mut usize,
    pub num_descs): usize,
    pub num_ids): usize,
