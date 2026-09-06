//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/venus/hfi_msgs.h
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
// Copyright (c) 2012-2016, The Linux Foundation. All rights reserved.
// Copyright (C) 2017 Linaro Ltd.
//
// message calls
pub const HFI_MSG_SYS_INIT: c_uint = 0x20001;
pub const HFI_MSG_SYS_PC_PREP: c_uint = 0x20002;
pub const HFI_MSG_SYS_RELEASE_RESOURCE: c_uint = 0x20003;
pub const HFI_MSG_SYS_DEBUG: c_uint = 0x20004;
pub const HFI_MSG_SYS_SESSION_INIT: c_uint = 0x20006;
pub const HFI_MSG_SYS_SESSION_END: c_uint = 0x20007;
pub const HFI_MSG_SYS_IDLE: c_uint = 0x20008;
pub const HFI_MSG_SYS_COV: c_uint = 0x20009;
pub const HFI_MSG_SYS_PROPERTY_INFO: c_uint = 0x2000a;
pub const HFI_MSG_EVENT_NOTIFY: c_uint = 0x21001;
pub const HFI_MSG_SESSION_GET_SEQUENCE_HEADER: c_uint = 0x21002;
pub const HFI_MSG_SYS_PING_ACK: c_uint = 0x220002;
pub const HFI_MSG_SYS_SESSION_ABORT: c_uint = 0x220004;
pub const HFI_MSG_SESSION_LOAD_RESOURCES: c_uint = 0x221001;
pub const HFI_MSG_SESSION_START: c_uint = 0x221002;
pub const HFI_MSG_SESSION_STOP: c_uint = 0x221003;
pub const HFI_MSG_SESSION_SUSPEND: c_uint = 0x221004;
pub const HFI_MSG_SESSION_RESUME: c_uint = 0x221005;
pub const HFI_MSG_SESSION_FLUSH: c_uint = 0x221006;
pub const HFI_MSG_SESSION_EMPTY_BUFFER: c_uint = 0x221007;
pub const HFI_MSG_SESSION_FILL_BUFFER: c_uint = 0x221008;
pub const HFI_MSG_SESSION_PROPERTY_INFO: c_uint = 0x221009;
pub const HFI_MSG_SESSION_RELEASE_RESOURCES: c_uint = 0x22100a;
pub const HFI_MSG_SESSION_PARSE_SEQUENCE_HEADER: c_uint = 0x22100b;
pub const HFI_MSG_SESSION_RELEASE_BUFFERS: c_uint = 0x22100c;
pub const HFI_PICTURE_I: c_uint = 0x00000001;
pub const HFI_PICTURE_P: c_uint = 0x00000002;
pub const HFI_PICTURE_B: c_uint = 0x00000004;
pub const HFI_PICTURE_IDR: c_uint = 0x00000008;
pub const HFI_FRAME_NOTCODED: c_uint = 0x7f002000;
pub const HFI_FRAME_YUV: c_uint = 0x7f004000;
pub const HFI_UNUSED_PICT: c_uint = 0x10000000;
// message packets
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_event_notify_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub event_id: u32,
    pub event_data1: u32,
    pub event_data2: u32,
    pub ext_event_data: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_event_release_buffer_ref_pkt {
    pub packet_buffer: u32,
    pub extradata_buffer: u32,
    pub output_tag: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_sys_init_done_pkt {
    pub hdr: hfi_pkt_hdr,
    pub error_type: u32,
    pub num_properties: u32,
    pub data: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_sys_pc_prep_done_pkt {
    pub hdr: hfi_pkt_hdr,
    pub error_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_sys_release_resource_done_pkt {
    pub hdr: hfi_pkt_hdr,
    pub resource_handle: u32,
    pub error_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_session_init_done_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub error_type: u32,
    pub num_properties: u32,
    pub data: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_session_end_done_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub error_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_session_get_sequence_hdr_done_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub error_type: u32,
    pub header_len: u32,
    pub sequence_header: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_sys_session_abort_done_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub error_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_sys_idle_pkt {
    pub hdr: hfi_pkt_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_sys_ping_ack_pkt {
    pub hdr: hfi_pkt_hdr,
    pub client_data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_sys_property_info_pkt {
    pub hdr: hfi_pkt_hdr,
    pub num_properties: u32,
    pub property: u32,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_session_load_resources_done_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub error_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_session_start_done_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub error_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_session_stop_done_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub error_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_session_suspend_done_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub error_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_session_resume_done_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub error_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_session_flush_done_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub error_type: u32,
    pub flush_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_session_empty_buffer_done_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub error_type: u32,
    pub offset: u32,
    pub filled_len: u32,
    pub input_tag: u32,
    pub packet_buffer: u32,
    pub extradata_buffer: u32,
    pub data: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_session_fbd_compressed_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub time_stamp_hi: u32,
    pub time_stamp_lo: u32,
    pub error_type: u32,
    pub flags: u32,
    pub mark_target: u32,
    pub mark_data: u32,
    pub stats: u32,
    pub offset: u32,
    pub alloc_len: u32,
    pub filled_len: u32,
    pub input_tag: u32,
    pub output_tag: u32,
    pub picture_type: u32,
    pub packet_buffer: u32,
    pub extradata_buffer: u32,
    pub data: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_session_fbd_uncompressed_plane0_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub stream_id: u32,
    pub view_id: u32,
    pub error_type: u32,
    pub time_stamp_hi: u32,
    pub time_stamp_lo: u32,
    pub flags: u32,
    pub mark_target: u32,
    pub mark_data: u32,
    pub stats: u32,
    pub alloc_len: u32,
    pub filled_len: u32,
    pub offset: u32,
    pub frame_width: u32,
    pub frame_height: u32,
    pub start_x_coord: u32,
    pub start_y_coord: u32,
    pub input_tag: u32,
    pub input_tag2: u32,
    pub output_tag: u32,
    pub picture_type: u32,
    pub packet_buffer: u32,
    pub extradata_buffer: u32,
    pub data: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_session_fbd_uncompressed_plane1_pkt {
    pub flags: u32,
    pub alloc_len: u32,
    pub filled_len: u32,
    pub offset: u32,
    pub packet_buffer2: u32,
    pub data: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_session_fbd_uncompressed_plane2_pkt {
    pub flags: u32,
    pub alloc_len: u32,
    pub filled_len: u32,
    pub offset: u32,
    pub packet_buffer3: u32,
    pub data: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_session_parse_sequence_header_done_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub error_type: u32,
    pub num_properties: u32,
    pub data: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_session_property_info_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub num_properties: u32,
    pub property: u32,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_session_release_resources_done_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub error_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_session_release_buffers_done_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub error_type: u32,
    pub num_buffers: u32,
    pub buffer_info: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_sys_debug_pkt {
    pub hdr: hfi_pkt_hdr,
    pub msg_type: u32,
    pub msg_size: u32,
    pub time_stamp_hi: u32,
    pub time_stamp_lo: u32,
    pub msg_data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_sys_coverage_pkt {
    pub hdr: hfi_pkt_hdr,
    pub msg_size: u32,
    pub time_stamp_hi: u32,
    pub time_stamp_lo: u32,
    pub msg_data: [u8; ],
}

extern "C" {
    pub fn hfi_process_watchdog_timeout(core: *mut venus_core);
}
extern "C" {
    pub fn hfi_process_msg_packet(core: *mut venus_core, hdr: *mut hfi_pkt_hdr) -> u32;
}
