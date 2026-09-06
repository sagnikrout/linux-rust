//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/sev-guest.h
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


// SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note
//
// Userspace interface for AMD SEV and SNP guest driver.
//
// Copyright (C) 2021 Advanced Micro Devices, Inc.
//
// Author: Brijesh Singh <brijesh.singh@amd.com>
//
// SEV API specification is available at: https://developer.amd.com/sev
//

pub const SNP_REPORT_USER_DATA_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snp_report_req {
// user data that should be included in the report
    pub user_data: [__u8; SNP_REPORT_USER_DATA_SIZE],
// The vmpl level to be included in the report
    pub vmpl: __u32,
// Must be zero filled
    pub rsvd: [__u8; 28],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snp_report_resp {
// response data, see SEV-SNP spec for the format
    pub data: [__u8; 4000],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snp_derived_key_req {
    pub root_key_select: __u32,
    pub rsvd: __u32,
    pub guest_field_select: __u64,
    pub vmpl: __u32,
    pub guest_svn: __u32,
    pub tcb_version: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snp_derived_key_resp {
// response data, see SEV-SNP spec for the format
    pub data: [__u8; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snp_guest_request_ioctl {
// message version number (must be non-zero)
    pub msg_version: __u8,
// Request and response structure address
    pub req_data: __u64,
    pub resp_data: __u64,
// bits[63:32]: VMM error code, bits[31:0] firmware error code (see psp-sev.h)
    pub exitinfo2: __u64,
    pub fw_error: __u32,
    pub vmm_error: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snp_ext_report_req {
    pub data: snp_report_req,
// where to copy the certificate blob
    pub certs_address: __u64,
// length of the certificate blob
    pub certs_len: __u32,
}

// Get SNP attestation report

// Get a derived key from the root

// Get SNP extended report as defined in the GHCB specification version 2.

// Guest message request EXIT_INFO_2 constants

pub const SNP_GUEST_VMM_ERR_SHIFT: c_int = 32;

pub const SNP_GUEST_VMM_ERR_INVALID_LEN: c_int = 1;
pub const SNP_GUEST_VMM_ERR_BUSY: c_int = 2;
