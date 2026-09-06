//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/amd_cper.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cper_error_severity {
    CPER_SEV_NON_FATAL_UNCORRECTED = 0,
    CPER_SEV_FATAL_UNCORRECTED     = 1,
    CPER_SEV_NON_FATAL_CORRECTED   = 2,
    CPER_SEV_NUM                   = 3,

    CPER_SEV_UNUSED = 10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cper_aca_reg {
    CPER_ACA_REG_CTL_LO    = 0,
    CPER_ACA_REG_CTL_HI    = 1,
    CPER_ACA_REG_STATUS_LO = 2,
    CPER_ACA_REG_STATUS_HI = 3,
    CPER_ACA_REG_ADDR_LO   = 4,
    CPER_ACA_REG_ADDR_HI   = 5,
    CPER_ACA_REG_MISC0_LO  = 6,
    CPER_ACA_REG_MISC0_HI  = 7,
    CPER_ACA_REG_CONFIG_LO = 8,
    CPER_ACA_REG_CONFIG_HI = 9,
    CPER_ACA_REG_IPID_LO   = 10,
    CPER_ACA_REG_IPID_HI   = 11,
    CPER_ACA_REG_SYND_LO   = 12,
    CPER_ACA_REG_SYND_HI   = 13,

    CPER_ACA_REG_COUNT     = 32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_timestamp {
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
pub struct cper_hdr {
    pub /: *mut *mut char signature[4]; / "CPER",
    pub revision: u16,
    pub /: *mut *mut uint32_t signature_end; / 0xFFFFFFFF,
    pub sec_cnt: u16,
    pub error_severity: cper_error_severity,
    pub 1: uint32_t platform_id :,
    pub 1: uint32_t timestamp :,
    pub 1: uint32_t partition_id :,
    pub 29: uint32_t reserved :,
    pub valid_bits: },
    pub valid_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_sec_desc {
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
pub struct cper_sec_nonstd_err_hdr {
    pub 1: uint64_t apic_id :,
    pub 1: uint64_t fw_id :,
    pub 6: uint64_t err_info_cnt :,
    pub 6: uint64_t err_context_cnt :,
    pub valid_bits: },
    pub valid_mask: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_sec_nonstd_err_info {
    pub error_type: guid_t,
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
pub struct cper_sec_nonstd_err_ctx {
    pub reg_ctx_type: u16,
    pub reg_arr_size: u16,
    pub msr_addr: u32,
    pub mm_reg_addr: u64,
    pub reg_dump: [u32; CPER_ACA_REG_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_sec_nonstd_err {
    pub hdr: cper_sec_nonstd_err_hdr,
    pub info: cper_sec_nonstd_err_info,
    pub ctx: cper_sec_nonstd_err_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_sec_crashdump_hdr {
    pub reserved1: u64,
    pub reserved2: u64,
    pub fw_id: [c_char; 48],
    pub reserved3: [u64; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_sec_crashdump_reg_data {
    pub status_lo: u32,
    pub status_hi: u32,
    pub addr_lo: u32,
    pub addr_hi: u32,
    pub ipid_lo: u32,
    pub ipid_hi: u32,
    pub synd_lo: u32,
    pub synd_hi: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_sec_crashdump_body_fatal {
    pub reg_ctx_type: u16,
    pub reg_arr_size: u16,
    pub reserved1: u32,
    pub reserved2: u64,
    pub data: cper_sec_crashdump_reg_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_sec_crashdump_body_boot {
    pub reg_ctx_type: u16,
    pub reg_arr_size: u16,
    pub reserved1: u32,
    pub reserved2: u64,
    pub msg: [u64; CPER_MAX_OAM_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_sec_crashdump_fatal {
    pub hdr: cper_sec_crashdump_hdr,
    pub body: cper_sec_crashdump_body_fatal,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_sec_crashdump_boot {
    pub hdr: cper_sec_crashdump_hdr,
    pub body: cper_sec_crashdump_body_boot,
}

