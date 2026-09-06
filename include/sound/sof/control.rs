//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/sof/control.h
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

//
// Component Mixers and Controls
//
// channel positions - uses same values as ALSA
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc_chmap {
    SOF_CHMAP_UNKNOWN = 0,
    SOF_CHMAP_NA,		/**< N/A, silent */
    SOF_CHMAP_MONO,		/**< mono stream */
    SOF_CHMAP_FL,		/**< front left */
    SOF_CHMAP_FR,		/**< front right */
    SOF_CHMAP_RL,		/**< rear left */
    SOF_CHMAP_RR,		/**< rear right */
    SOF_CHMAP_FC,		/**< front centre */
    SOF_CHMAP_LFE,		/**< LFE */
    SOF_CHMAP_SL,		/**< side left */
    SOF_CHMAP_SR,		/**< side right */
    SOF_CHMAP_RC,		/**< rear centre */
    SOF_CHMAP_FLC,		/**< front left centre */
    SOF_CHMAP_FRC,		/**< front right centre */
    SOF_CHMAP_RLC,		/**< rear left centre */
    SOF_CHMAP_RRC,		/**< rear right centre */
    SOF_CHMAP_FLW,		/**< front left wide */
    SOF_CHMAP_FRW,		/**< front right wide */
    SOF_CHMAP_FLH,		/**< front left high */
    SOF_CHMAP_FCH,		/**< front centre high */
    SOF_CHMAP_FRH,		/**< front right high */
    SOF_CHMAP_TC,		/**< top centre */
    SOF_CHMAP_TFL,		/**< top front left */
    SOF_CHMAP_TFR,		/**< top front right */
    SOF_CHMAP_TFC,		/**< top front centre */
    SOF_CHMAP_TRL,		/**< top rear left */
    SOF_CHMAP_TRR,		/**< top rear right */
    SOF_CHMAP_TRC,		/**< top rear centre */
    SOF_CHMAP_TFLC,		/**< top front left centre */
    SOF_CHMAP_TFRC,		/**< top front right centre */
    SOF_CHMAP_TSL,		/**< top side left */
    SOF_CHMAP_TSR,		/**< top side right */
    SOF_CHMAP_LLFE,		/**< left LFE */
    SOF_CHMAP_RLFE,		/**< right LFE */
    SOF_CHMAP_BC,		/**< bottom centre */
    SOF_CHMAP_BLC,		/**< bottom left centre */
    SOF_CHMAP_BRC,		/**< bottom right centre */
    SOF_CHMAP_LAST = SOF_CHMAP_BRC,
}

// control data type and direction
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc_ctrl_type {
// per channel data - uses struct sof_ipc_ctrl_value_chan
    SOF_CTRL_TYPE_VALUE_CHAN_GET = 0,
    SOF_CTRL_TYPE_VALUE_CHAN_SET,
// component data - uses struct sof_ipc_ctrl_value_comp
    SOF_CTRL_TYPE_VALUE_COMP_GET,
    SOF_CTRL_TYPE_VALUE_COMP_SET,
// bespoke data - uses struct sof_abi_hdr
    SOF_CTRL_TYPE_DATA_GET,
    SOF_CTRL_TYPE_DATA_SET,
}

// control command type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc_ctrl_cmd {
    SOF_CTRL_CMD_VOLUME = 0, /**< maps to ALSA volume style controls */
    SOF_CTRL_CMD_ENUM,	/**< maps to ALSA enum style controls */
    SOF_CTRL_CMD_SWITCH,	/**< maps to ALSA switch style controls */
    SOF_CTRL_CMD_BINARY,	/**< maps to ALSA binary style controls */
}

// generic channel mapped value data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_ctrl_value_chan {
    pub /: *mut *mut *mut uint32_t channel; /< channel map - enum sof_ipc_chmap,
    pub value: u32,
    pub __packed: },
// generic component mapped value data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_ctrl_value_comp {
    pub /: *mut *mut *mut uint32_t index; /< component source/sink/control index in control,
    pub uvalue: u32,
    pub svalue: i32,
}

// generic control data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_ctrl_data {
    pub rhdr: sof_ipc_reply,
    pub comp_id: u32,
// control access and data type
    pub /: *mut *mut *mut uint32_t type; /< enum sof_ipc_ctrl_type,
    pub /: *mut *mut *mut uint32_t cmd; /< enum sof_ipc_ctrl_cmd,
    pub /: *mut *mut *mut uint32_t index; /< control index for comps > 1 control,
// control data - can either be appended or DMAed from host
    pub buffer: sof_ipc_host_buffer,
    pub /: *mut *mut *mut uint32_t num_elems; /< in array elems or bytes for data type,
    pub /: *mut *mut *mut uint32_t elems_remaining; /< elems remaining if sent in parts,
    pub /: *mut *mut *mut uint32_t msg_index; /< for large messages sent in parts,
// reserved for future use
    pub reserved: [u32; 6],
// control data - add new types if needed
// channel values can be used by volume type controls
    pub chanv): DECLARE_FLEX_ARRAY(struct sof_ipc_ctrl_value_chan,,
// component values used by routing controls like mux, mixer
    pub compv): DECLARE_FLEX_ARRAY(struct sof_ipc_ctrl_value_comp,,
// data can be used by binary controls
    pub data): DECLARE_FLEX_ARRAY(struct sof_abi_hdr,,
}

// Event type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc_ctrl_event_type {
    SOF_CTRL_EVENT_GENERIC = 0,	/**< generic event */
    SOF_CTRL_EVENT_GENERIC_METADATA,	/**< generic event with metadata */
    SOF_CTRL_EVENT_KD,	/**< keyword detection event */
    SOF_CTRL_EVENT_VAD,	/**< voice activity detection event */
}

//
// Generic notification data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_comp_event {
    pub rhdr: sof_ipc_reply,
    pub /: *mut *mut *mut uint16_t src_comp_type; /< COMP_TYPE_,
    pub /: *mut *mut *mut uint32_t src_comp_id; /< source component id,
    pub /: *mut *mut *mut *mut uint32_t event_type; /< event type - SOF_CTRL_EVENT_,
    pub /: *mut *mut *mut uint32_t num_elems; /< in array elems or bytes for data type,
// reserved for future use
    pub reserved: [u32; 8],
// control data - add new types if needed
// data can be used by binary controls
    pub data: [sof_abi_hdr; 0],
// event specific values
    pub event_value: u32,
}
