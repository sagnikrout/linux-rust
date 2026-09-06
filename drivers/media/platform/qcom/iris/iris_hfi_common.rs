//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/iris/iris_hfi_common.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hfi_packet_port_type {
    HFI_PORT_NONE		= 0x00000000,
    HFI_PORT_BITSTREAM	= 0x00000001,
    HFI_PORT_RAW		= 0x00000002,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hfi_packet_payload_info {
    HFI_PAYLOAD_NONE	= 0x00000000,
    HFI_PAYLOAD_U32		= 0x00000001,
    HFI_PAYLOAD_S32		= 0x00000002,
    HFI_PAYLOAD_U64		= 0x00000003,
    HFI_PAYLOAD_S64		= 0x00000004,
    HFI_PAYLOAD_STRUCTURE	= 0x00000005,
    HFI_PAYLOAD_BLOB	= 0x00000006,
    HFI_PAYLOAD_STRING	= 0x00000007,
    HFI_PAYLOAD_Q16		= 0x00000008,
    HFI_PAYLOAD_U32_ENUM	= 0x00000009,
    HFI_PAYLOAD_32_PACKED	= 0x0000000a,
    HFI_PAYLOAD_U32_ARRAY	= 0x0000000b,
    HFI_PAYLOAD_S32_ARRAY	= 0x0000000c,
    HFI_PAYLOAD_64_PACKED	= 0x0000000d,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hfi_packet_host_flags {
    HFI_HOST_FLAGS_NONE			= 0x00000000,
    HFI_HOST_FLAGS_INTR_REQUIRED		= 0x00000001,
    HFI_HOST_FLAGS_RESPONSE_REQUIRED	= 0x00000002,
    HFI_HOST_FLAGS_NON_DISCARDABLE		= 0x00000004,
    HFI_HOST_FLAGS_GET_PROPERTY		= 0x00000008,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hfi_color_primaries {
    HFI_PRIMARIES_RESERVED		= 0,
    HFI_PRIMARIES_BT709		= 1,
    HFI_PRIMARIES_UNSPECIFIED	= 2,
    HFI_PRIMARIES_BT470_SYSTEM_M	= 4,
    HFI_PRIMARIES_BT470_SYSTEM_BG	= 5,
    HFI_PRIMARIES_BT601_525		= 6,
    HFI_PRIMARIES_SMPTE_ST240M	= 7,
    HFI_PRIMARIES_GENERIC_FILM	= 8,
    HFI_PRIMARIES_BT2020		= 9,
    HFI_PRIMARIES_SMPTE_ST428_1	= 10,
    HFI_PRIMARIES_SMPTE_RP431_2	= 11,
    HFI_PRIMARIES_SMPTE_EG431_1	= 12,
    HFI_PRIMARIES_SMPTE_EBU_TECH	= 22,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hfi_transfer_characteristics {
    HFI_TRANSFER_RESERVED		= 0,
    HFI_TRANSFER_BT709		= 1,
    HFI_TRANSFER_UNSPECIFIED	= 2,
    HFI_TRANSFER_BT470_SYSTEM_M	= 4,
    HFI_TRANSFER_BT470_SYSTEM_BG	= 5,
    HFI_TRANSFER_BT601_525_OR_625	= 6,
    HFI_TRANSFER_SMPTE_ST240M	= 7,
    HFI_TRANSFER_LINEAR		= 8,
    HFI_TRANSFER_LOG_100_1		= 9,
    HFI_TRANSFER_LOG_SQRT		= 10,
    HFI_TRANSFER_XVYCC		= 11,
    HFI_TRANSFER_BT1361_0		= 12,
    HFI_TRANSFER_SRGB_SYCC		= 13,
    HFI_TRANSFER_BT2020_14		= 14,
    HFI_TRANSFER_BT2020_15		= 15,
    HFI_TRANSFER_SMPTE_ST2084_PQ	= 16,
    HFI_TRANSFER_SMPTE_ST428_1	= 17,
    HFI_TRANSFER_BT2100_2_HLG	= 18,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hfi_matrix_coefficients {
    HFI_MATRIX_COEFF_SRGB_SMPTE_ST428_1		= 0,
    HFI_MATRIX_COEFF_BT709				= 1,
    HFI_MATRIX_COEFF_UNSPECIFIED			= 2,
    HFI_MATRIX_COEFF_RESERVED			= 3,
    HFI_MATRIX_COEFF_FCC_TITLE_47			= 4,
    HFI_MATRIX_COEFF_BT470_SYS_BG_OR_BT601_625	= 5,
    HFI_MATRIX_COEFF_BT601_525_BT1358_525_OR_625	= 6,
    HFI_MATRIX_COEFF_SMPTE_ST240			= 7,
    HFI_MATRIX_COEFF_YCGCO				= 8,
    HFI_MATRIX_COEFF_BT2020_NON_CONSTANT		= 9,
    HFI_MATRIX_COEFF_BT2020_CONSTANT		= 10,
    HFI_MATRIX_COEFF_SMPTE_ST2085			= 11,
    HFI_MATRIX_COEFF_SMPTE_CHROM_DERV_NON_CONSTANT	= 12,
    HFI_MATRIX_COEFF_SMPTE_CHROM_DERV_CONSTANT	= 13,
    HFI_MATRIX_COEFF_BT2100				= 14,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iris_hfi_prop_type_handle {
    pub type: u32,
    pub plane): *mut *mut *mut int (handle)(struct iris_inst inst, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iris_hfi_sys_ops {
    pub core): *mut *mut int (sys_init)(struct iris_core,
    pub core): *mut *mut int (sys_image_version)(struct iris_core,
    pub core): *mut *mut int (sys_interframe_powercollapse)(struct iris_core,
    pub core): *mut *mut int (sys_pc_prep)(struct iris_core,
    pub core): *mut *mut void (sys_hfi_response_handler)(struct iris_core,
    pub (*sys_get_instance)(void): *mut iris_inst,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iris_hfi_session_ops {
    pub plane): *mut *mut *mut int (session_set_config_params)(struct iris_inst inst, u32,
    pub payload_size): *mut *mut void payload, u32,
    pub inst): *mut *mut int (session_open)(struct iris_inst,
    pub plane): *mut *mut *mut int (session_start)(struct iris_inst inst, u32,
    pub buffer): *mut *mut *mut int (session_queue_buf)(struct iris_inst inst, struct iris_buffer,
    pub buffer): *mut *mut *mut int (session_release_buf)(struct iris_inst inst, struct iris_buffer,
    pub plane): *mut *mut *mut int (session_pause)(struct iris_inst inst, u32,
    pub plane): *mut *mut *mut int (session_resume_drc)(struct iris_inst inst, u32,
    pub plane): *mut *mut *mut int (session_stop)(struct iris_inst inst, u32,
    pub plane): *mut *mut *mut int (session_drain)(struct iris_inst inst, u32,
    pub plane): *mut *mut *mut int (session_resume_drain)(struct iris_inst inst, u32,
    pub inst): *mut *mut int (session_close)(struct iris_inst,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_subscription_params {
    pub bitstream_resolution: u32,
    pub crop_offsets: [u32; 2],
    pub bit_depth: u32,
    pub coded_frames: u32,
    pub fw_min_count: u32,
    pub pic_order_cnt: u32,
    pub color_info: u32,
    pub profile: u32,
    pub level: u32,
    pub tier: u32,
    pub drap: u32,
    pub film_grain: u32,
    pub super_block: u32,
}

extern "C" {
    pub fn iris_hfi_get_v4l2_color_primaries(hfi_primaries: u32) -> u32;
}
extern "C" {
    pub fn iris_hfi_get_v4l2_transfer_char(hfi_characterstics: u32) -> u32;
}
extern "C" {
    pub fn iris_hfi_get_v4l2_matrix_coefficients(hfi_coefficients: u32) -> u32;
}
extern "C" {
    pub fn iris_hfi_core_init(core: *mut iris_core) -> c_int;
}
extern "C" {
    pub fn iris_hfi_pm_suspend(core: *mut iris_core) -> c_int;
}
extern "C" {
    pub fn iris_hfi_pm_resume(core: *mut iris_core) -> c_int;
}
extern "C" {
    pub fn iris_hfi_isr(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn iris_hfi_isr_handler(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
