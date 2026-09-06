//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/sof/ipc4/header.h
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

// maximum message size for mailbox Tx/Rx
pub const SOF_IPC4_MSG_MAX_SIZE: c_int = 4096;
// \addtogroup sof_uapi uAPI
// SOF uAPI specification.
// @{
//
// struct sof_ipc4_msg - Placeholder of an IPC4 message
// @header_u64:		IPC4 header as single u64 number
// @primary:		Primary, mandatory part of the header
// @extension:		Extended part of the header, if not used it should be
// set to 0
// @data_size:		Size of data in bytes pointed by @data_ptr
// @data_ptr:		Pointer to the optional payload of a message
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_msg {
    pub header_u64: u64,
    pub primary: u32,
    pub extension: u32,
}

//
// struct sof_ipc4_tuple - Generic type/ID and parameter tuple
// @type:		type/ID
// @size:		size of the @value array in bytes
// @value:		value for the given type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_tuple {
    pub type: u32,
    pub size: u32,
    pub value: [u32; ],
    pub __packed: },
//
// IPC4 messages have two 32 bit identifier made up as follows :-
//
// header - msg type, msg id, msg direction ...
// extension - extra params such as msg data size in mailbox
//
// These are sent at the start of the IPC message in the mailbox. Messages
// should not be sent in the doorbell (special exceptions for firmware).
//
// IPC4 primary header bit allocation for messages
// bit 0-23:	message type specific
// bit 24-28:	type:	enum sof_ipc4_global_msg if target is SOF_IPC4_FW_GEN_MSG
// enum sof_ipc4_module_type if target is SOF_IPC4_MODULE_MSG
// bit 29:	response - sof_ipc4_msg_dir
// bit 30:	target - enum sof_ipc4_msg_target
// bit 31:	reserved, unused
//
// Value of target field - must fit into 1 bit
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc4_msg_target {
// Global FW message
    SOF_IPC4_FW_GEN_MSG,

// Module message
    SOF_IPC4_MODULE_MSG
}

// Value of type field - must fit into 5 bits
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc4_global_msg {
    SOF_IPC4_GLB_BOOT_CONFIG,
    SOF_IPC4_GLB_ROM_CONTROL,
    SOF_IPC4_GLB_IPCGATEWAY_CMD,

// 3 .. 12: RESERVED - do not use

    SOF_IPC4_GLB_PERF_MEASUREMENTS_CMD = 13,
    SOF_IPC4_GLB_CHAIN_DMA,

    SOF_IPC4_GLB_LOAD_MULTIPLE_MODULES,
    SOF_IPC4_GLB_UNLOAD_MULTIPLE_MODULES,

// pipeline settings
    SOF_IPC4_GLB_CREATE_PIPELINE,
    SOF_IPC4_GLB_DELETE_PIPELINE,
    SOF_IPC4_GLB_SET_PIPELINE_STATE,
    SOF_IPC4_GLB_GET_PIPELINE_STATE,
    SOF_IPC4_GLB_GET_PIPELINE_CONTEXT_SIZE,
    SOF_IPC4_GLB_SAVE_PIPELINE,
    SOF_IPC4_GLB_RESTORE_PIPELINE,

//
// library loading
//
// Loads library (using Code Load or HD/A Host Output DMA)
//
    SOF_IPC4_GLB_LOAD_LIBRARY,
//
// Prepare the host DMA channel for library loading, must be followed by
// a SOF_IPC4_GLB_LOAD_LIBRARY message as the library loading step
//
    SOF_IPC4_GLB_LOAD_LIBRARY_PREPARE,

    SOF_IPC4_GLB_INTERNAL_MESSAGE,

// Notification (FW to SW driver)
    SOF_IPC4_GLB_NOTIFICATION,

// 28 .. 31: RESERVED - do not use

    SOF_IPC4_GLB_TYPE_LAST,
}

// Value of response field - must fit into 1 bit
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc4_msg_dir {
    SOF_IPC4_MSG_REQUEST,
    SOF_IPC4_MSG_REPLY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc4_pipeline_state {
    SOF_IPC4_PIPE_INVALID_STATE,
    SOF_IPC4_PIPE_UNINITIALIZED,
    SOF_IPC4_PIPE_RESET,
    SOF_IPC4_PIPE_PAUSED,
    SOF_IPC4_PIPE_RUNNING,
    SOF_IPC4_PIPE_EOS
}

// Generic message fields (bit 24-30)
// encoded to header's msg_tgt field
pub const SOF_IPC4_MSG_TARGET_SHIFT: c_int = 30;

// encoded to header's rsp field
pub const SOF_IPC4_MSG_DIR_SHIFT: c_int = 29;

// encoded to header's type field
pub const SOF_IPC4_MSG_TYPE_SHIFT: c_int = 24;

// Global message type specific field definitions
// pipeline creation ipc msg
pub const SOF_IPC4_GLB_PIPE_INSTANCE_SHIFT: c_int = 16;

pub const SOF_IPC4_GLB_PIPE_PRIORITY_SHIFT: c_int = 11;

pub const SOF_IPC4_GLB_PIPE_MEM_SIZE_SHIFT: c_int = 0;

pub const SOF_IPC4_GLB_PIPE_EXT_LP_SHIFT: c_int = 0;

pub const SOF_IPC4_GLB_PIPE_EXT_CORE_ID_SHIFT: c_int = 20;

pub const SOF_IPC4_GLB_PIPE_PAYLOAD_SHIFT: c_int = 29;

// pipeline set state ipc msg
pub const SOF_IPC4_GLB_PIPE_STATE_ID_SHIFT: c_int = 16;

pub const SOF_IPC4_GLB_PIPE_STATE_SHIFT: c_int = 0;

// pipeline set state IPC msg extension

// load library ipc msg
pub const SOF_IPC4_GLB_LOAD_LIBRARY_LIB_ID_SHIFT: c_int = 16;

// chain dma ipc message
pub const SOF_IPC4_GLB_CHAIN_DMA_HOST_ID_SHIFT: c_int = 0;

pub const SOF_IPC4_GLB_CHAIN_DMA_LINK_ID_SHIFT: c_int = 8;

pub const SOF_IPC4_GLB_CHAIN_DMA_ALLOCATE_SHIFT: c_int = 16;

pub const SOF_IPC4_GLB_CHAIN_DMA_ENABLE_SHIFT: c_int = 17;

pub const SOF_IPC4_GLB_CHAIN_DMA_SCS_SHIFT: c_int = 18;

pub const SOF_IPC4_GLB_EXT_CHAIN_DMA_FIFO_SIZE_SHIFT: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc4_channel_config {
// one channel only.
    SOF_IPC4_CHANNEL_CONFIG_MONO,
// L & R.
    SOF_IPC4_CHANNEL_CONFIG_STEREO,
// L, R & LFE; PCM only.
    SOF_IPC4_CHANNEL_CONFIG_2_POINT_1,
// L, C & R; MP3 & AAC only.
    SOF_IPC4_CHANNEL_CONFIG_3_POINT_0,
// L, C, R & LFE; PCM only.
    SOF_IPC4_CHANNEL_CONFIG_3_POINT_1,
// L, R, Ls & Rs; PCM only.
    SOF_IPC4_CHANNEL_CONFIG_QUATRO,
// L, C, R & Cs; MP3 & AAC only.
    SOF_IPC4_CHANNEL_CONFIG_4_POINT_0,
// L, C, R, Ls & Rs.
    SOF_IPC4_CHANNEL_CONFIG_5_POINT_0,
// L, C, R, Ls, Rs & LFE.
    SOF_IPC4_CHANNEL_CONFIG_5_POINT_1,
// one channel replicated in two.
    SOF_IPC4_CHANNEL_CONFIG_DUAL_MONO,
// Stereo (L,R) in 4 slots, 1st stream: [ L, R, -, - ]
    SOF_IPC4_CHANNEL_CONFIG_I2S_DUAL_STEREO_0,
// Stereo (L,R) in 4 slots, 2nd stream: [ -, -, L, R ]
    SOF_IPC4_CHANNEL_CONFIG_I2S_DUAL_STEREO_1,
// L, C, R, Ls, Rs & LFE., LS, RS
    SOF_IPC4_CHANNEL_CONFIG_7_POINT_1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc4_interleaved_style {
    SOF_IPC4_CHANNELS_INTERLEAVED,
    SOF_IPC4_CHANNELS_NONINTERLEAVED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc4_sample_type {
    SOF_IPC4_MSB_INTEGER, /* integer with Most Significant Byte first */
    SOF_IPC4_LSB_INTEGER, /* integer with Least Significant Byte first */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_audio_format {
    pub sampling_frequency: u32,
    pub bit_depth: u32,
    pub ch_map: u32,
    pub /: *mut *mut uint32_t ch_cfg; / sof_ipc4_channel_config,
    pub interleaving_style: u32,
    pub /: *mut *mut uint32_t fmt_cfg; / channels_count valid_bit_depth s_type,
    pub __aligned(4): } __packed,
pub const SOF_IPC4_AUDIO_FORMAT_CFG_CHANNELS_COUNT_SHIFT: c_int = 0;

pub const SOF_IPC4_AUDIO_FORMAT_CFG_V_BIT_DEPTH_SHIFT: c_int = 8;

pub const SOF_IPC4_AUDIO_FORMAT_CFG_SAMPLE_TYPE_SHIFT: c_int = 16;

// Module message type specific field definitions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc4_module_type {
    SOF_IPC4_MOD_INIT_INSTANCE,
    SOF_IPC4_MOD_CONFIG_GET,
    SOF_IPC4_MOD_CONFIG_SET,
    SOF_IPC4_MOD_LARGE_CONFIG_GET,
    SOF_IPC4_MOD_LARGE_CONFIG_SET,
    SOF_IPC4_MOD_BIND,
    SOF_IPC4_MOD_UNBIND,
    SOF_IPC4_MOD_SET_DX,
    SOF_IPC4_MOD_SET_D0IX,
    SOF_IPC4_MOD_ENTER_MODULE_RESTORE,
    SOF_IPC4_MOD_EXIT_MODULE_RESTORE,
    SOF_IPC4_MOD_DELETE_INSTANCE,

    SOF_IPC4_MOD_TYPE_LAST,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_base_module_cfg {
    pub /: *mut *mut uint32_t cpc; / the max count of Cycles Per Chunk processing,
    pub /: *mut *mut uint32_t ibs; / input Buffer Size (in bytes),
    pub /: *mut *mut uint32_t obs; / output Buffer Size (in bytes),
    pub /: *mut *mut uint32_t is_pages; / number of physical pages used,
    pub audio_fmt: sof_ipc4_audio_format,
    pub __aligned(4): } __packed,
// common module ipc msg
pub const SOF_IPC4_MOD_INSTANCE_SHIFT: c_int = 16;

pub const SOF_IPC4_MOD_ID_SHIFT: c_int = 0;

// init module ipc msg
pub const SOF_IPC4_MOD_EXT_PARAM_SIZE_SHIFT: c_int = 0;

pub const SOF_IPC4_MOD_EXT_PPL_ID_SHIFT: c_int = 16;

pub const SOF_IPC4_MOD_EXT_CORE_ID_SHIFT: c_int = 24;

pub const SOF_IPC4_MOD_EXT_DOMAIN_SHIFT: c_int = 28;

pub const SOF_IPC4_MOD_EXT_EXTENDED_INIT_SHIFT: c_int = 29;

// bind/unbind module ipc msg
pub const SOF_IPC4_MOD_EXT_DST_MOD_ID_SHIFT: c_int = 0;

pub const SOF_IPC4_MOD_EXT_DST_MOD_INSTANCE_SHIFT: c_int = 16;

pub const SOF_IPC4_MOD_EXT_DST_MOD_QUEUE_ID_SHIFT: c_int = 24;

pub const SOF_IPC4_MOD_EXT_SRC_MOD_QUEUE_ID_SHIFT: c_int = 27;

pub const MOD_ENABLE_LOG: c_int = 6;
pub const MOD_SYSTEM_TIME: c_int = 20;
// set module large config
pub const SOF_IPC4_MOD_EXT_MSG_SIZE_SHIFT: c_int = 0;

pub const SOF_IPC4_MOD_EXT_MSG_PARAM_ID_SHIFT: c_int = 20;

pub const SOF_IPC4_MOD_EXT_MSG_LAST_BLOCK_SHIFT: c_int = 28;

pub const SOF_IPC4_MOD_EXT_MSG_FIRST_BLOCK_SHIFT: c_int = 29;

// Init instance messagees
pub const SOF_IPC4_MOD_INIT_BASEFW_MOD_ID: c_int = 0;
pub const SOF_IPC4_MOD_INIT_BASEFW_INSTANCE_ID: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc4_base_fw_params {
    SOF_IPC4_FW_PARAM_ENABLE_LOGS = 6,
    SOF_IPC4_FW_PARAM_FW_CONFIG,
    SOF_IPC4_FW_PARAM_HW_CONFIG_GET,
    SOF_IPC4_FW_PARAM_MODULES_INFO_GET,
    SOF_IPC4_FW_PARAM_LIBRARIES_INFO_GET = 16,
    SOF_IPC4_FW_PARAM_SYSTEM_TIME = 20,
    SOF_IPC4_FW_PARAM_MIC_PRIVACY_STATE_CHANGE = 35,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc4_fw_config_params {
    SOF_IPC4_FW_CFG_FW_VERSION,
    SOF_IPC4_FW_CFG_MEMORY_RECLAIMED,
    SOF_IPC4_FW_CFG_SLOW_CLOCK_FREQ_HZ,
    SOF_IPC4_FW_CFG_FAST_CLOCK_FREQ_HZ,
    SOF_IPC4_FW_CFG_DMA_BUFFER_CONFIG,
    SOF_IPC4_FW_CFG_ALH_SUPPORT_LEVEL,
    SOF_IPC4_FW_CFG_DL_MAILBOX_BYTES,
    SOF_IPC4_FW_CFG_UL_MAILBOX_BYTES,
    SOF_IPC4_FW_CFG_TRACE_LOG_BYTES,
    SOF_IPC4_FW_CFG_MAX_PPL_COUNT,
    SOF_IPC4_FW_CFG_MAX_ASTATE_COUNT,
    SOF_IPC4_FW_CFG_MAX_MODULE_PIN_COUNT,
    SOF_IPC4_FW_CFG_MODULES_COUNT,
    SOF_IPC4_FW_CFG_MAX_MOD_INST_COUNT,
    SOF_IPC4_FW_CFG_MAX_LL_TASKS_PER_PRI_COUNT,
    SOF_IPC4_FW_CFG_LL_PRI_COUNT,
    SOF_IPC4_FW_CFG_MAX_DP_TASKS_COUNT,
    SOF_IPC4_FW_CFG_MAX_LIBS_COUNT,
    SOF_IPC4_FW_CFG_SCHEDULER_CONFIG,
    SOF_IPC4_FW_CFG_XTAL_FREQ_HZ,
    SOF_IPC4_FW_CFG_CLOCKS_CONFIG,
    SOF_IPC4_FW_CFG_RESERVED,
    SOF_IPC4_FW_CFG_POWER_GATING_POLICY,
    SOF_IPC4_FW_CFG_ASSERT_MODE,
    SOF_IPC4_FW_RESERVED1,
    SOF_IPC4_FW_RESERVED2,
    SOF_IPC4_FW_RESERVED3,
    SOF_IPC4_FW_RESERVED4,
    SOF_IPC4_FW_RESERVED5,
    SOF_IPC4_FW_CONTEXT_SAVE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_fw_version {
    pub major: u16,
    pub minor: u16,
    pub hotfix: u16,
    pub build: u16,
    pub __packed: },
// Payload data for SOF_IPC4_MOD_SET_DX
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_dx_state_info {
// core(s) to apply the change
    pub core_mask: u32,
// core state: 0: put core_id to D3; 1: put core_id to D0
    pub dx_mask: u32,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc4_hw_config_params {
    SOF_IPC4_HW_CFG_INTEL_MIC_PRIVACY_CAPS = 11,
}

pub const SOF_IPC_INTEL_MIC_PRIVACY_VERSION_PTL: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_intel_mic_privacy_cap {
    pub version: u32,
    pub capabilities_length: u32,
    pub capabilities: [u32; ],
    pub __packed: },
// Reply messages
//
// IPC4 primary header bit allocation for replies
// bit 0-23:	status
// bit 24-28:	type:	enum sof_ipc4_global_msg if target is SOF_IPC4_FW_GEN_MSG
// enum sof_ipc4_module_type if target is SOF_IPC4_MODULE_MSG
// bit 29:	response - sof_ipc4_msg_dir
// bit 30:	target - enum sof_ipc4_msg_target
// bit 31:	reserved, unused
//

// Notification messages
//
// IPC4 primary header bit allocation for notifications
// bit 0-15:	notification type specific
// bit 16-23:	enum sof_ipc4_notification_type
// bit 24-28:	SOF_IPC4_GLB_NOTIFICATION
// bit 29:	response - sof_ipc4_msg_dir
// bit 30:	target - enum sof_ipc4_msg_target
// bit 31:	reserved, unused
//

pub const SOF_IPC4_NOTIFICATION_TYPE_SHIFT: c_int = 16;

pub const SOF_IPC4_LOG_CORE_SHIFT: c_int = 12;

// Value of notification type field - must fit into 8 bits
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc4_notification_type {
// Phrase detected (notification from WoV module)
    SOF_IPC4_NOTIFY_PHRASE_DETECTED = 4,
// Event from a resource (pipeline or module instance)
    SOF_IPC4_NOTIFY_RESOURCE_EVENT,
// Debug log buffer status changed
    SOF_IPC4_NOTIFY_LOG_BUFFER_STATUS,
// Timestamp captured at the link
    SOF_IPC4_NOTIFY_TIMESTAMP_CAPTURED,
// FW complete initialization
    SOF_IPC4_NOTIFY_FW_READY,
// Audio classifier result (ACA)
    SOF_IPC4_NOTIFY_FW_AUD_CLASS_RESULT,
// Exception caught by DSP FW
    SOF_IPC4_NOTIFY_EXCEPTION_CAUGHT,
// 11 is skipped by the existing cavs firmware
// Custom module notification
    SOF_IPC4_NOTIFY_MODULE_NOTIFICATION = 12,
// 13 is reserved - do not use
// Probe notify data available
    SOF_IPC4_NOTIFY_PROBE_DATA_AVAILABLE = 14,
// AM module notifications
    SOF_IPC4_NOTIFY_ASYNC_MSG_SRVC_MESSAGE,

    SOF_IPC4_NOTIFY_TYPE_LAST,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc4_resource_type {
    SOF_IPC4_MODULE_INSTANCE,
    SOF_IPC4_PIPELINE,
    SOF_IPC4_GATEWAY,
    SOF_IPC4_EDF_TASK,
    SOF_IPC4_INVALID_RESOURCE_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc4_event_type {
// Underrun detected by the Mixer
    SOF_IPC4_MIXER_UNDERRUN_DETECTED = 1,
// Error caught during data processing
    SOF_IPC4_PROCESS_DATA_ERROR = 3,
// Underrun detected by gateway.
    SOF_IPC4_GATEWAY_UNDERRUN_DETECTED = 6,
// Overrun detected by gateway
    SOF_IPC4_GATEWAY_OVERRUN_DETECTED,
}

//
// struct sof_ipc4_process_data_error_event_data - process data error event payload
// @error_code: Error code returned by data processing function
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_process_data_error_event_data {
    pub error_code: u32,
}

//
// struct sof_ipc4_mixer_underrun_event_data - mixer underrun event payload
// @eos_flag: Indicates EndOfStream
// @data_mixed: Data processed by module (in bytes)
// @expected_data_mixed: Expected data to be processed (in bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_mixer_underrun_event_data {
    pub eos_flag: u32,
    pub data_mixed: u32,
    pub expected_data_mixed: u32,
}

//
// union sof_ipc4_resource_event_data - resource event specific payload
// @dws: Raw event data payload as six dwords
// @process_data_error: SOF_IPC4_PROCESS_DATA_ERROR payload
// @mixer_underrun: SOF_IPC4_MIXER_UNDERRUN_DETECTED payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union sof_ipc4_resource_event_data {
    pub dws: [u32; 6],
    pub process_data_error: sof_ipc4_process_data_error_event_data,
    pub mixer_underrun: sof_ipc4_mixer_underrun_event_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_notify_resource_data {
    pub resource_type: u32,
    pub resource_id: u32,
    pub event_type: u32,
    pub reserved: u32,
    pub data: sof_ipc4_resource_event_data,
    pub __aligned(4): } __packed,

//
// The debug memory window is divided into 16 slots, and the
// first slot is used as a recorder for the other 15 slots.
//
pub const SOF_IPC4_MAX_DEBUG_SLOTS: c_int = 15;
pub const SOF_IPC4_DEBUG_SLOT_SIZE: c_uint = 0x1000;
// debug log slot types
pub const SOF_IPC4_DEBUG_SLOT_UNUSED: c_uint = 0x00000000;
pub const SOF_IPC4_DEBUG_SLOT_CRITICAL_LOG: c_uint = 0x54524300 /* byte 0: core ID */;
pub const SOF_IPC4_DEBUG_SLOT_DEBUG_LOG: c_uint = 0x474f4c00 /* byte 0: core ID */;
pub const SOF_IPC4_DEBUG_SLOT_GDB_STUB: c_uint = 0x42444700;
pub const SOF_IPC4_DEBUG_SLOT_TELEMETRY: c_uint = 0x4c455400;
pub const SOF_IPC4_DEBUG_SLOT_BROKEN: c_uint = 0x44414544;
//
// struct sof_ipc4_notify_module_data - payload for module notification
// @instance_id: instance ID of the originator module of the notification
// @module_id: module ID of the originator of the notification
// @event_id: module specific event id
// @event_data_size: Size of the @event_data (if any) in bytes
// @event_data: Optional notification data, module and notification dependent
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_notify_module_data {
    pub instance_id: u16,
    pub module_id: u16,
    pub event_id: u32,
    pub event_data_size: u32,
    pub event_data: [u8; ],
    pub __aligned(4): } __packed,
//
// ALSA kcontrol change notification
//
// The event_id of struct sof_ipc4_notify_module_data is divided into two u16:
// upper u16: magic number for ALSA kcontrol types: 0xA15A
// lower u16: param_id of the control, which is the type of the control
// The event_data contains the struct sof_ipc4_control_msg_payload of the control
// which sent the notification.
//

pub const SOF_IPC4_NOTIFY_MODULE_EVENTID_ALSA_MAGIC_VAL: c_uint = 0xA15A0000;

//
// Macros for creating struct sof_ipc4_module_init_ext_init payload
// with its associated data. ext_init payload should be the first
// piece of payload following SOF_IPC4_MOD_INIT_INSTANCE msg, and its
// existence is indicated with SOF_IPC4_MOD_EXT_EXTENDED-bit.
//
// The macros below apply to sof_ipc4_module_init_ext_init.word0
//
pub const SOF_IPC4_MOD_INIT_EXT_RTOS_DOMAIN_SHIFT: c_int = 0;

pub const SOF_IPC4_MOD_INIT_EXT_GNA_USED_SHIFT: c_int = 1;

pub const SOF_IPC4_MOD_INIT_EXT_OBJ_ARRAY_SHIFT: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_module_init_ext_init {
    pub word0: u32,
    pub rsvd1: u32,
    pub rsvd2: u32,
    pub __aligned(4): } __packed,
//
// SOF_IPC4_MOD_EXT_EXTENDED payload may be followed by arbitrary
// number of object array objects. SOF_IPC4_MOD_INIT_EXT_DATA_ARRAY
// -bit indicates that an array object follows struct
// sof_ipc4_module_init_ext_init.
//
// The object header's SOF_IPC4_MOD_INIT_EXT_OBJ_LAST-bit in struct
// sof_ipc4_module_init_ext_object indicates if the array is continued
// with another object. The header has also fields to identify the
// object, SOF_IPC4_MOD_INIT_EXT_OBJ_ID, and to indicate the object's
// size in 32-bit words, SOF_IPC4_MOD_INIT_EXT_OBJ_WORDS, not
// including the header itself.
//
// The macros below apply to sof_ipc4_module_init_ext_object.header
//
pub const SOF_IPC4_MOD_INIT_EXT_OBJ_LAST_SHIFT: c_int = 0;

pub const SOF_IPC4_MOD_INIT_EXT_OBJ_ID_SHIFT: c_int = 1;

pub const SOF_IPC4_MOD_INIT_EXT_OBJ_WORDS_SHIFT: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_module_init_ext_object {
    pub header: u32,
    pub data: [u32; ],
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc4_mod_init_ext_obj_id {
    SOF_IPC4_MOD_INIT_DATA_ID_INVALID = 0,
    SOF_IPC4_MOD_INIT_DATA_ID_DP_DATA,
    SOF_IPC4_MOD_INIT_DATA_ID_MAX = SOF_IPC4_MOD_INIT_DATA_ID_DP_DATA,
}

// DP module memory configuration data object for object array
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_mod_init_ext_dp_memory_data {
    pub /: *mut *mut u32 domain_id; / userspace domain ID,
    pub /: *mut *mut u32 stack_bytes; / required stack size in bytes,
    pub /: *mut *mut u32 heap_bytes; / required heap size in bytes,
    pub __aligned(4): } __packed,
//
// This set of macros are very similar to the set above, but these are
// for building payload to SOF_IPC4_GLB_CREATE_PIPELINE message.
//
// Macros for creating struct sof_ipc4_glb_pipe_payload payload with
// its associated data. struct sof_ipc4_glb_pipe_payload should be the
// first piece of payload following SOF_IPC4_GLB_CREATE_PIPELINE msg,
// and its existence is indicated with SOF_IPC4_GLB_PIPE_PAYLOAD bit.
//
// The macros below apply to sof_ipc4_glb_pipe_payload.word0
//
pub const SOF_IPC4_GLB_PIPE_PAYLOAD_WORDS_SHIFT: c_int = 0;

pub const SOF_IPC4_GLB_PIPE_EXT_OBJ_ARRAY_SHIFT: c_int = 24;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_glb_pipe_payload {
    pub word0: u32,
    pub rsvd1: u32,
    pub rsvd2: u32,
    pub __aligned(4): } __packed,
//
// SOF_IPC4_GLB_CREATE_PIPELINE payload may be followed by arbitrary
// number of object array objects. SOF_IPC4_GLB_PIPE_EXT_OBJ_ARRAY-bit
// indicates that an array object follows struct
// sof_ipc4_glb_pipe_payload.
//
// The object header's SOF_IPC4_GLB_PIPE_EXT_OBJ_LAST-bit in struct
// sof_ipc4_glb_pipe_ext_object indicates if the array is continued
// with another object. The header has also fields to identify the
// object, SOF_IPC4_GLB_PIPE_EXT_OBJ_ID, and to indicate the object's
// size in 32-bit words, SOF_IPC4_GLB_PIPE_EXT_OBJ_WORDS, not
// including the header itself.
//
// The macros below apply to sof_ipc4_glb_pipe_ext_object.header
//
pub const SOF_IPC4_GLB_PIPE_EXT_OBJ_LAST_SHIFT: c_int = 0;

pub const SOF_IPC4_GLB_PIPE_EXT_OBJ_ID_SHIFT: c_int = 1;

pub const SOF_IPC4_GLB_PIPE_EXT_OBJ_WORDS_SHIFT: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_glb_pipe_ext_object {
    pub header: u32,
    pub data: [u32; ],
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc4_glb_pipe_ext_obj_id {
    SOF_IPC4_GLB_PIPE_DATA_ID_INVALID = 0,
    SOF_IPC4_GLB_PIPE_DATA_ID_MEM_DATA,
    SOF_IPC4_GLB_PIPE_DATA_ID_MAX = SOF_IPC4_GLB_PIPE_DATA_ID_MEM_DATA,
}

// Pipeline memory configuration data object for ext_init object array
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_glb_pipe_ext_obj_memory_data {
    pub /: *mut *mut u32 domain_id; / userspace domain ID,
    pub /: *mut *mut u32 stack_bytes; / stack size in bytes,
    pub /: *mut *mut u32 heap_bytes; / heap size in bytes,
    pub __aligned(4): } __packed,
// @}
