//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/virtio_snd.h
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


// SPDX-License-Identifier: BSD-3-Clause
//
// Copyright (C) 2021 OpenSynergy GmbH
//

//
// FEATURE BITS
//
// device supports control elements
//
// CONFIGURATION SPACE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_snd_config {
// # of available physical jacks
    pub jacks: __le32,
// # of available PCM streams
    pub streams: __le32,
// # of available channel maps
    pub chmaps: __le32,
// # of available control elements (if VIRTIO_SND_F_CTLS)
    pub controls: __le32,
}

// device virtqueue indexes
// # of device virtqueues
//
// COMMON DEFINITIONS
//
// supported dataflow directions
// jack control request types
// PCM control request types
// channel map control request types
// control element request types
// jack event types
// PCM event types
// control element event types
// common status codes
// common header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_snd_hdr {
    pub code: __le32,
}

// event notification
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_snd_event {
// VIRTIO_SND_EVT_XXX
    pub hdr: virtio_snd_hdr,
// optional event data
    pub data: __le32,
}

// common control request to query an item information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_snd_query_info {
// VIRTIO_SND_R_XXX_INFO
    pub hdr: virtio_snd_hdr,
// item start identifier
    pub start_id: __le32,
// item count to query
    pub count: __le32,
// item information size in bytes
    pub size: __le32,
}

// common item information header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_snd_info {
// function group node id (High Definition Audio Specification 7.1.2)
    pub hda_fn_nid: __le32,
}

//
// JACK CONTROL MESSAGES
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_snd_jack_hdr {
// VIRTIO_SND_R_JACK_XXX
    pub hdr: virtio_snd_hdr,
// 0 ... virtio_snd_config::jacks - 1
    pub jack_id: __le32,
}

// supported jack features
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_snd_jack_info {
// common header
    pub hdr: virtio_snd_info,
// supported feature bit map (1 << VIRTIO_SND_JACK_F_XXX)
    pub features: __le32,
// pin configuration (High Definition Audio Specification 7.3.3.31)
    pub hda_reg_defconf: __le32,
// pin capabilities (High Definition Audio Specification 7.3.4.9)
    pub hda_reg_caps: __le32,
// current jack connection status (0: disconnected, 1: connected)
    pub connected: __u8,
    pub padding: [__u8; 7],
}

// jack remapping control request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_snd_jack_remap {
// .code = VIRTIO_SND_R_JACK_REMAP
    pub hdr: virtio_snd_jack_hdr,
// selected association number
    pub association: __le32,
// selected sequence number
    pub sequence: __le32,
}

//
// PCM CONTROL MESSAGES
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_snd_pcm_hdr {
// VIRTIO_SND_R_PCM_XXX
    pub hdr: virtio_snd_hdr,
// 0 ... virtio_snd_config::streams - 1
    pub stream_id: __le32,
}

// supported PCM stream features
// supported PCM sample formats
// analog formats (width / physical width)
// digital formats (width / physical width)
// supported PCM frame rates
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_snd_pcm_info {
// common header
    pub hdr: virtio_snd_info,
// supported feature bit map (1 << VIRTIO_SND_PCM_F_XXX)
    pub features: __le32,
// supported sample format bit map (1 << VIRTIO_SND_PCM_FMT_XXX)
    pub formats: __le64,
// supported frame rate bit map (1 << VIRTIO_SND_PCM_RATE_XXX)
    pub rates: __le64,
// dataflow direction (VIRTIO_SND_D_XXX)
    pub direction: __u8,
// minimum # of supported channels
    pub channels_min: __u8,
// maximum # of supported channels
    pub channels_max: __u8,
    pub padding: [__u8; 5],
}

// set PCM stream format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_snd_pcm_set_params {
// .code = VIRTIO_SND_R_PCM_SET_PARAMS
    pub hdr: virtio_snd_pcm_hdr,
// size of the hardware buffer
    pub buffer_bytes: __le32,
// size of the hardware period
    pub period_bytes: __le32,
// selected feature bit map (1 << VIRTIO_SND_PCM_F_XXX)
    pub features: __le32,
// selected # of channels
    pub channels: __u8,
// selected sample format (VIRTIO_SND_PCM_FMT_XXX)
    pub format: __u8,
// selected frame rate (VIRTIO_SND_PCM_RATE_XXX)
    pub rate: __u8,
    pub padding: __u8,
}

//
// PCM I/O MESSAGES
//
// I/O request header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_snd_pcm_xfer {
// 0 ... virtio_snd_config::streams - 1
    pub stream_id: __le32,
}

// I/O request status
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_snd_pcm_status {
// VIRTIO_SND_S_XXX
    pub status: __le32,
// current device latency
    pub latency_bytes: __le32,
}

//
// CHANNEL MAP CONTROL MESSAGES
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_snd_chmap_hdr {
// VIRTIO_SND_R_CHMAP_XXX
    pub hdr: virtio_snd_hdr,
// 0 ... virtio_snd_config::chmaps - 1
    pub chmap_id: __le32,
}

// standard channel position definition
// maximum possible number of channels
pub const VIRTIO_SND_CHMAP_MAX_SIZE: c_int = 18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_snd_chmap_info {
// common header
    pub hdr: virtio_snd_info,
// dataflow direction (VIRTIO_SND_D_XXX)
    pub direction: __u8,
// # of valid channel position values
    pub channels: __u8,
// channel position values (VIRTIO_SND_CHMAP_XXX)
    pub positions: [__u8; VIRTIO_SND_CHMAP_MAX_SIZE],
}

//
// CONTROL ELEMENTS MESSAGES
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_snd_ctl_hdr {
// VIRTIO_SND_R_CTL_XXX
    pub hdr: virtio_snd_hdr,
// 0 ... virtio_snd_config::controls - 1
    pub control_id: __le32,
}

// supported roles for control elements
// supported value types for control elements
// supported access rights for control elements
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_snd_ctl_info {
// common header
    pub hdr: virtio_snd_info,
// element role (VIRTIO_SND_CTL_ROLE_XXX)
    pub role: __le32,
// element value type (VIRTIO_SND_CTL_TYPE_XXX)
    pub type: __le32,
// element access right bit map (1 << VIRTIO_SND_CTL_ACCESS_XXX)
    pub access: __le32,
// # of members in the element value
    pub count: __le32,
// index for an element with a non-unique name
    pub index: __le32,
// name identifier string for the element
    pub name: [__u8; 44],
// additional information about the element's value
// VIRTIO_SND_CTL_TYPE_INTEGER
// minimum supported value
    pub min: __le32,
// maximum supported value
    pub max: __le32,
// fixed step size for value (0 = variable size)
    pub step: __le32,
    pub integer: },
// VIRTIO_SND_CTL_TYPE_INTEGER64
// minimum supported value
    pub min: __le64,
// maximum supported value
    pub max: __le64,
// fixed step size for value (0 = variable size)
    pub step: __le64,
    pub integer64: },
// VIRTIO_SND_CTL_TYPE_ENUMERATED
// # of options supported for value
    pub items: __le32,
    pub enumerated: },
    pub value: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_snd_ctl_enum_item {
// option name
    pub item: [__u8; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_snd_ctl_iec958 {
// AES/IEC958 channel status bits
    pub status: [__u8; 24],
// AES/IEC958 subcode bits
    pub subcode: [__u8; 147],
// nothing
    pub pad: __u8,
// AES/IEC958 subframe bits
    pub dig_subframe: [__u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_snd_ctl_value {
// VIRTIO_SND_CTL_TYPE_BOOLEAN|INTEGER value
    pub integer: [__le32; 128],
// VIRTIO_SND_CTL_TYPE_INTEGER64 value
    pub integer64: [__le64; 64],
// VIRTIO_SND_CTL_TYPE_ENUMERATED value (option indexes)
    pub enumerated: [__le32; 128],
// VIRTIO_SND_CTL_TYPE_BYTES value
    pub bytes: [__u8; 512],
// VIRTIO_SND_CTL_TYPE_IEC958 value
    pub iec958: virtio_snd_ctl_iec958,
    pub value: },
}

// supported event reason types
// element's value has changed
// element's information has changed
// element's metadata has changed
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_snd_ctl_event {
// VIRTIO_SND_EVT_CTL_NOTIFY
    pub hdr: virtio_snd_hdr,
// 0 ... virtio_snd_config::controls - 1
    pub control_id: __le16,
// event reason bit map (1 << VIRTIO_SND_CTL_EVT_MASK_XXX)
    pub mask: __le16,
}
