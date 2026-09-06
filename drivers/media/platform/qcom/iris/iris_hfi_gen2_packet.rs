//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/iris/iris_hfi_gen2_packet.h
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
// Copyright (c) 2022-2024 Qualcomm Innovation Center, Inc. All rights reserved.
//

//
// struct iris_hfi_header
//
// @size: size of the total packet in bytes including hfi_header
// @session_id: For session level hfi_header session_id is non-zero.
// For  system level hfi_header session_id is zero.
// @header_id: unique header id for each hfi_header
// @reserved: reserved for future use
// @num_packets: number of hfi_packet that are included with the hfi_header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iris_hfi_header {
    pub size: u32,
    pub session_id: u32,
    pub header_id: u32,
    pub reserved: [u32; 4],
    pub num_packets: u32,
}

//
// struct iris_hfi_packet
//
// @size: size of the hfi_packet in bytes including payload
// @type: one of the below hfi_packet types:
// HFI_CMD_*,
// HFI_PROP_*,
// HFI_ERROR_*,
// HFI_INFO_*,
// HFI_SYS_ERROR_
// @flags: hfi_packet flags. It is represented as bit masks.
// host packet flags are "enum hfi_packet_host_flags"
// firmware packet flags are "enum hfi_packet_firmware_flags"
// @payload_info: payload information indicated by "enum hfi_packet_payload_info"
// @port: hfi_packet port type indicated by "enum hfi_packet_port_type"
// This is bitmask and may be applicable to multiple ports.
// @packet_id: host hfi_packet contains unique packet id.
// firmware returns host packet id in response packet
// wherever applicable. If not applicable firmware sets it to zero.
// @reserved: reserved for future use.
// @payload: flexible array of payload having additional packet information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iris_hfi_packet {
    pub size: u32,
    pub type: u32,
    pub flags: u32,
    pub payload_info: u32,
    pub port: u32,
    pub packet_id: u32,
    pub reserved: [u32; 2],
    pub payload: [u32; ],
}

//
// struct iris_hfi_buffer
//
// @type: buffer type indicated by "enum hfi_buffer_type"
// FW needs to return proper type for any buffer command.
// @index: index of the buffer
// @base_address: base address of the buffer.
// This buffer address is always 4KBytes aligned.
// @addr_offset: accessible buffer offset from base address
// Decoder bitstream buffer: 256 Bytes aligned
// Firmware can uniquely identify a buffer based on
// base_address & addr_offset.
// HW can read memory only from base_address+addr_offset.
// @buffer_size: accessible buffer size in bytes starting from addr_offset
// @data_offset: data starts from "base_address + addr_offset + data_offset"
// RAW buffer: data_offset is 0. Restriction: 4KBytes aligned
// decoder bitstream buffer: no restriction (can be any value)
// @data_size: data size in bytes
// @flags: buffer flags. It is represented as bit masks.
// host buffer flags are "enum hfi_buffer_host_flags"
// firmware buffer flags are "enum hfi_buffer_firmware_flags"
// @timestamp: timestamp of the buffer in nano seconds (ns)
// It is Presentation timestamp (PTS) for encoder & decoder.
// Decoder: it is pass through from bitstream to raw buffer.
// firmware does not need to return as part of input buffer done.
// For any internal buffers: there is no timestamp. Host sets as 0.
// @reserved: reserved for future use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iris_hfi_buffer {
    pub type: u32,
    pub index: u32,
    pub base_address: u64,
    pub addr_offset: u32,
    pub buffer_size: u32,
    pub data_offset: u32,
    pub data_size: u32,
    pub timestamp: u64,
    pub flags: u32,
    pub reserved: [u32; 5],
}

extern "C" {
    pub fn iris_hfi_gen2_get_color_primaries(primaries: u32) -> u32;
}
extern "C" {
    pub fn iris_hfi_gen2_get_transfer_char(characterstics: u32) -> u32;
}
extern "C" {
    pub fn iris_hfi_gen2_get_matrix_coefficients(coefficients: u32) -> u32;
}
extern "C" {
    pub fn iris_hfi_gen2_packet_sys_init(core: *mut iris_core, hdr: *mut iris_hfi_header);
}
extern "C" {
    pub fn iris_hfi_gen2_packet_image_version(core: *mut iris_core, hdr: *mut iris_hfi_header);
}
extern "C" {
    pub fn iris_hfi_gen2_packet_sys_pc_prep(core: *mut iris_core, hdr: *mut iris_hfi_header);
}
