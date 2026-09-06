//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/venus/hfi_cmds.h
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

// commands
pub const HFI_CMD_SYS_INIT: c_uint = 0x10001;
pub const HFI_CMD_SYS_PC_PREP: c_uint = 0x10002;
pub const HFI_CMD_SYS_SET_RESOURCE: c_uint = 0x10003;
pub const HFI_CMD_SYS_RELEASE_RESOURCE: c_uint = 0x10004;
pub const HFI_CMD_SYS_SET_PROPERTY: c_uint = 0x10005;
pub const HFI_CMD_SYS_GET_PROPERTY: c_uint = 0x10006;
pub const HFI_CMD_SYS_SESSION_INIT: c_uint = 0x10007;
pub const HFI_CMD_SYS_SESSION_END: c_uint = 0x10008;
pub const HFI_CMD_SYS_SET_BUFFERS: c_uint = 0x10009;
pub const HFI_CMD_SYS_TEST_SSR: c_uint = 0x10101;
pub const HFI_CMD_SESSION_SET_PROPERTY: c_uint = 0x11001;
pub const HFI_CMD_SESSION_SET_BUFFERS: c_uint = 0x11002;
pub const HFI_CMD_SESSION_GET_SEQUENCE_HEADER: c_uint = 0x11003;
pub const HFI_CMD_SYS_SESSION_ABORT: c_uint = 0x210001;
pub const HFI_CMD_SYS_PING: c_uint = 0x210002;
pub const HFI_CMD_SESSION_LOAD_RESOURCES: c_uint = 0x211001;
pub const HFI_CMD_SESSION_START: c_uint = 0x211002;
pub const HFI_CMD_SESSION_STOP: c_uint = 0x211003;
pub const HFI_CMD_SESSION_EMPTY_BUFFER: c_uint = 0x211004;
pub const HFI_CMD_SESSION_FILL_BUFFER: c_uint = 0x211005;
pub const HFI_CMD_SESSION_SUSPEND: c_uint = 0x211006;
pub const HFI_CMD_SESSION_RESUME: c_uint = 0x211007;
pub const HFI_CMD_SESSION_FLUSH: c_uint = 0x211008;
pub const HFI_CMD_SESSION_GET_PROPERTY: c_uint = 0x211009;
pub const HFI_CMD_SESSION_PARSE_SEQUENCE_HEADER: c_uint = 0x21100a;
pub const HFI_CMD_SESSION_RELEASE_BUFFERS: c_uint = 0x21100b;
pub const HFI_CMD_SESSION_RELEASE_RESOURCES: c_uint = 0x21100c;
pub const HFI_CMD_SESSION_CONTINUE: c_uint = 0x21100d;
pub const HFI_CMD_SESSION_SYNC: c_uint = 0x21100e;
// command packets
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_sys_init_pkt {
    pub hdr: hfi_pkt_hdr,
    pub arch_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_sys_pc_prep_pkt {
    pub hdr: hfi_pkt_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_sys_set_resource_pkt {
    pub hdr: hfi_pkt_hdr,
    pub resource_handle: u32,
    pub resource_type: u32,
    pub resource_data: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_sys_release_resource_pkt {
    pub hdr: hfi_pkt_hdr,
    pub resource_type: u32,
    pub resource_handle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_sys_set_property_pkt {
    pub hdr: hfi_pkt_hdr,
    pub num_properties: u32,
    pub data: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_sys_get_property_pkt {
    pub hdr: hfi_pkt_hdr,
    pub num_properties: u32,
    pub data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_sys_set_buffers_pkt {
    pub hdr: hfi_pkt_hdr,
    pub buffer_type: u32,
    pub buffer_size: u32,
    pub num_buffers: u32,
    pub buffer_addr: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_sys_ping_pkt {
    pub hdr: hfi_pkt_hdr,
    pub client_data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_init_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub session_domain: u32,
    pub session_codec: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_end_pkt {
    pub shdr: hfi_session_hdr_pkt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_abort_pkt {
    pub shdr: hfi_session_hdr_pkt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_set_property_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub num_properties: u32,
    pub data: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_set_buffers_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub buffer_type: u32,
    pub buffer_size: u32,
    pub extradata_size: u32,
    pub min_buffer_size: u32,
    pub num_buffers: u32,
    pub buffer_info: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_get_sequence_header_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub buffer_len: u32,
    pub packet_buffer: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_load_resources_pkt {
    pub shdr: hfi_session_hdr_pkt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_start_pkt {
    pub shdr: hfi_session_hdr_pkt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_stop_pkt {
    pub shdr: hfi_session_hdr_pkt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_empty_buffer_compressed_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub time_stamp_hi: u32,
    pub time_stamp_lo: u32,
    pub flags: u32,
    pub mark_target: u32,
    pub mark_data: u32,
    pub offset: u32,
    pub alloc_len: u32,
    pub filled_len: u32,
    pub input_tag: u32,
    pub packet_buffer: u32,
    pub extradata_buffer: u32,
    pub data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_empty_buffer_uncompressed_plane0_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub view_id: u32,
    pub time_stamp_hi: u32,
    pub time_stamp_lo: u32,
    pub flags: u32,
    pub mark_target: u32,
    pub mark_data: u32,
    pub alloc_len: u32,
    pub filled_len: u32,
    pub offset: u32,
    pub input_tag: u32,
    pub packet_buffer: u32,
    pub extradata_buffer: u32,
    pub data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_empty_buffer_uncompressed_plane1_pkt {
    pub flags: u32,
    pub alloc_len: u32,
    pub filled_len: u32,
    pub offset: u32,
    pub packet_buffer2: u32,
    pub data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_empty_buffer_uncompressed_plane2_pkt {
    pub flags: u32,
    pub alloc_len: u32,
    pub filled_len: u32,
    pub offset: u32,
    pub packet_buffer3: u32,
    pub data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_fill_buffer_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub stream_id: u32,
    pub offset: u32,
    pub alloc_len: u32,
    pub filled_len: u32,
    pub output_tag: u32,
    pub packet_buffer: u32,
    pub extradata_buffer: u32,
    pub data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_flush_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub flush_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_suspend_pkt {
    pub shdr: hfi_session_hdr_pkt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_resume_pkt {
    pub shdr: hfi_session_hdr_pkt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_get_property_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub num_properties: u32,
    pub data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_release_buffer_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub buffer_type: u32,
    pub buffer_size: u32,
    pub extradata_size: u32,
    pub response_req: u32,
    pub num_buffers: u32,
    pub __counted_by(num_buffers): u32 buffer_info[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_release_resources_pkt {
    pub shdr: hfi_session_hdr_pkt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_parse_sequence_header_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub header_len: u32,
    pub packet_buffer: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_sfr {
    pub buf_size: u32,
    pub __counted_by(buf_size): u8 data[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_sys_test_ssr_pkt {
    pub hdr: hfi_pkt_hdr,
    pub trigger_type: u32,
}

extern "C" {
    pub fn pkt_set_version(version: hfi_version);
}
extern "C" {
    pub fn pkt_sys_init(pkt: *mut hfi_sys_init_pkt, arch_type: u32);
}
extern "C" {
    pub fn pkt_sys_pc_prep(pkt: *mut hfi_sys_pc_prep_pkt);
}
extern "C" {
    pub fn pkt_sys_idle_indicator(pkt: *mut hfi_sys_set_property_pkt, enable: u32);
}
extern "C" {
    pub fn pkt_sys_power_control(pkt: *mut hfi_sys_set_property_pkt, enable: u32);
}
extern "C" {
    pub fn pkt_sys_ubwc_config(pkt: *mut hfi_sys_set_property_pkt, hfi: *const hfi_ubwc_config);
}
extern "C" {
    pub fn pkt_sys_coverage_config(pkt: *mut hfi_sys_set_property_pkt, mode: u32);
}
extern "C" {
    pub fn pkt_sys_ping(pkt: *mut hfi_sys_ping_pkt, cookie: u32);
}
extern "C" {
    pub fn pkt_sys_image_version(pkt: *mut hfi_sys_get_property_pkt);
}
extern "C" {
    pub fn pkt_sys_ssr_cmd(pkt: *mut hfi_sys_test_ssr_pkt, trigger_type: u32) -> c_int;
}
extern "C" {
    pub fn pkt_session_cmd(pkt: *mut hfi_session_pkt, pkt_type: u32, cookie: *mut c_void);
}
