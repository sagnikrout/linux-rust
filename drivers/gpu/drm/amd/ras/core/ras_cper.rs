//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/ras/core/ras_cper.h
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


// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
pub const CPER_UUID_MAX_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cper_guid {
    pub b: [u8; CPER_UUID_MAX_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_cper_type {
    RAS_CPER_TYPE_RUNTIME,
    RAS_CPER_TYPE_FATAL,
    RAS_CPER_TYPE_BOOT,
    RAS_CPER_TYPE_RMA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_cper_severity {
    RAS_CPER_SEV_NON_FATAL_UE   = 0,
    RAS_CPER_SEV_FATAL_UE       = 1,
    RAS_CPER_SEV_NON_FATAL_CE   = 2,
    RAS_CPER_SEV_RMA            = 3,

    RAS_CPER_SEV_UNUSED = 10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_cper_aca_reg {
    RAS_CPER_ACA_REG_CTL    = 0,
    RAS_CPER_ACA_REG_STATUS = 1,
    RAS_CPER_ACA_REG_ADDR   = 2,
    RAS_CPER_ACA_REG_MISC0  = 3,
    RAS_CPER_ACA_REG_CONFIG = 4,
    RAS_CPER_ACA_REG_IPID   = 5,
    RAS_CPER_ACA_REG_SYND   = 6,
    RAS_CPER_ACA_REG_DESTAT	= 8,
    RAS_CPER_ACA_REG_DEADDR	= 9,
    RAS_CPER_ACA_REG_MASK	= 10,

    RAS_CPER_ACA_REG_COUNT     = 16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cper_timestamp {
    pub seconds: u8,
    pub minutes: u8,
    pub hours: u8,
    pub flag: u8,
    pub day: u8,
    pub month: u8,
    pub year: u8,
    pub century: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_section_hdr {
    pub /: *mut *mut char signature[4]; / "CPER",
    pub revision: u16,
    pub /: *mut *mut uint32_t signature_end; / 0xFFFFFFFF,
    pub sec_cnt: u16,
    pub error_severity: ras_cper_severity,
    pub 1: uint32_t platform_id :,
    pub 1: uint32_t timestamp :,
    pub 1: uint32_t partition_id :,
    pub 29: uint32_t reserved :,
    pub valid_bits: },
    pub valid_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_section_descriptor {
    pub /: *mut *mut uint32_t sec_offset; / Offset from the start of CPER entry,
    pub sec_length: u32,
    pub /: *mut *mut uint8_t revision_minor; / CPER_SEC_MINOR_REV_1,
    pub /: *mut *mut uint8_t revision_major; / CPER_SEC_MAJOR_REV_22,
    pub 1: uint8_t fru_id :,
    pub 1: uint8_t fru_text :,
    pub 6: uint8_t reserved :,
    pub valid_bits: },
    pub valid_mask: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct runtime_hdr {
    pub 1: uint64_t apic_id :,
    pub 1: uint64_t fw_id :,
    pub 6: uint64_t err_info_cnt :,
    pub 6: uint64_t err_context_cnt :,
    pub valid_bits: },
    pub valid_mask: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct runtime_descriptor {
    pub error_type: ras_cper_guid,
    pub 1: uint64_t ms_chk :,
    pub 1: uint64_t target_addr_id :,
    pub 1: uint64_t req_id :,
    pub 1: uint64_t resp_id :,
    pub 1: uint64_t instr_ptr :,
    pub 59: uint64_t reserved :,
    pub valid_bits: },
    pub valid_mask: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct runtime_error_reg {
    pub reg_ctx_type: u16,
    pub reg_arr_size: u16,
    pub msr_addr: u32,
    pub mm_reg_addr: u64,
    pub reg_dump: [u64; RAS_CPER_ACA_REG_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_section_runtime {
    pub hdr: runtime_hdr,
    pub descriptor: runtime_descriptor,
    pub reg: runtime_error_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crashdump_hdr {
    pub reserved1: u64,
    pub reserved2: u64,
    pub fw_id: [c_char; 48],
    pub reserved3: [u64; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fatal_reg_info {
    pub status: u64,
    pub addr: u64,
    pub ipid: u64,
    pub synd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crashdump_fatal {
    pub reg_ctx_type: u16,
    pub reg_arr_size: u16,
    pub reserved1: u32,
    pub reserved2: u64,
    pub reg: fatal_reg_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crashdump_boot {
    pub reg_ctx_type: u16,
    pub reg_arr_size: u16,
    pub reserved1: u32,
    pub reserved2: u64,
    pub msg: [u64; CPER_OAM_MAX_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_section_fatal {
    pub hdr: crashdump_hdr,
    pub data: crashdump_fatal,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_section_boot {
    pub hdr: crashdump_hdr,
    pub data: crashdump_boot,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cper_fatal_record {
    pub hdr: cper_section_hdr,
    pub descriptor: cper_section_descriptor,
    pub fatal: cper_section_fatal,
}

