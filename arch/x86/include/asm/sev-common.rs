//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/sev-common.h
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


// SPDX-License-Identifier: GPL-2.0
//
// AMD SEV header common between the guest and the hypervisor.
//
// Author: Brijesh Singh <brijesh.singh@amd.com>
//
pub const GHCB_MSR_INFO_POS: c_int = 0;
pub const GHCB_DATA_LOW: c_int = 12;

// SEV Information Request/Response
pub const GHCB_MSR_SEV_INFO_RESP: c_uint = 0x001;
pub const GHCB_MSR_SEV_INFO_REQ: c_uint = 0x002;

// GHCBData[63:48] */			\
// GHCBData[47:32] */			\
// GHCBData[31:24] */			\

// CPUID Request/Response
pub const GHCB_MSR_CPUID_REQ: c_uint = 0x004;
pub const GHCB_MSR_CPUID_RESP: c_uint = 0x005;
pub const GHCB_MSR_CPUID_FUNC_POS: c_int = 32;
pub const GHCB_MSR_CPUID_FUNC_MASK: c_uint = 0xffffffff;
pub const GHCB_MSR_CPUID_VALUE_POS: c_int = 32;
pub const GHCB_MSR_CPUID_VALUE_MASK: c_uint = 0xffffffff;
pub const GHCB_MSR_CPUID_REG_POS: c_int = 30;
pub const GHCB_MSR_CPUID_REG_MASK: c_uint = 0x3;
pub const GHCB_CPUID_REQ_EAX: c_int = 0;
pub const GHCB_CPUID_REQ_EBX: c_int = 1;
pub const GHCB_CPUID_REQ_ECX: c_int = 2;
pub const GHCB_CPUID_REQ_EDX: c_int = 3;

// GHCBData[11:0] */				\
// GHCBData[31:12] */				\
// GHCBData[63:32] */				\
// AP Reset Hold
pub const GHCB_MSR_AP_RESET_HOLD_REQ: c_uint = 0x006;
pub const GHCB_MSR_AP_RESET_HOLD_RESP: c_uint = 0x007;
pub const GHCB_MSR_AP_RESET_HOLD_RESULT_POS: c_int = 12;

// Preferred GHCB GPA Request
pub const GHCB_MSR_PREF_GPA_REQ: c_uint = 0x010;
pub const GHCB_MSR_GPA_VALUE_POS: c_int = 12;

pub const GHCB_MSR_PREF_GPA_RESP: c_uint = 0x011;
pub const GHCB_MSR_PREF_GPA_NONE: c_uint = 0xfffffffffffff;
// GHCB GPA Register
pub const GHCB_MSR_REG_GPA_REQ: c_uint = 0x012;

// GHCBData[63:12] */				\
// GHCBData[11:0] */				\
pub const GHCB_MSR_REG_GPA_RESP: c_uint = 0x013;

// GHCBData[63:12] */				\
//
// SNP Page State Change Operation
//
// GHCBData[55:52] - Page operation:
// 0x0001	Page assignment, Private
// 0x0002	Page assignment, Shared
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psc_op {
    SNP_PAGE_STATE_PRIVATE = 1,
    SNP_PAGE_STATE_SHARED,
}

pub const GHCB_MSR_PSC_REQ: c_uint = 0x014;

// GHCBData[55:52] */				\
// GHCBData[51:12] */				\
// GHCBData[11:0] */				\

pub const GHCB_MSR_PSC_RESP: c_uint = 0x015;

// GHCBData[63:32] */				\
// Set highest bit as a generic error response

// GHCB Run at VMPL Request/Response
pub const GHCB_MSR_VMPL_REQ: c_uint = 0x016;

// GHCBData[39:32] */				\
// GHCBDdata[11:0] */				\
pub const GHCB_MSR_VMPL_RESP: c_uint = 0x017;

// GHCBData[63:32] */				\
// GHCB Hypervisor Feature Request/Response
pub const GHCB_MSR_HV_FT_REQ: c_uint = 0x080;
pub const GHCB_MSR_HV_FT_RESP: c_uint = 0x081;
pub const GHCB_MSR_HV_FT_POS: c_int = 12;

// GHCBData[63:12] */				\

//
// SNP Page State Change NAE event
// The VMGEXIT_PSC_MAX_ENTRY determines the size of the PSC structure, which
// is a local stack variable in set_pages_state(). Do not increase this value
// without evaluating the impact to stack usage.
//
// Use VMGEXIT_PSC_MAX_COUNT in cases where the actual GHCB-defined max value
// is needed, such as when processing GHCB requests on the hypervisor side.
//
pub const VMGEXIT_PSC_MAX_ENTRY: c_int = 64;
pub const VMGEXIT_PSC_MAX_COUNT: c_int = 253;

pub const VMGEXIT_PSC_OP_PRIVATE: c_int = 1;
pub const VMGEXIT_PSC_OP_SHARED: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psc_hdr {
    pub cur_entry: u16,
    pub end_entry: u16,
    pub reserved: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psc_entry {
    pub 7: reserved :,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snp_psc_desc {
    pub hdr: psc_hdr,
    pub entries: [psc_entry; VMGEXIT_PSC_MAX_ENTRY],
    pub __packed: },
pub const GHCB_MSR_TERM_REQ: c_uint = 0x100;
pub const GHCB_MSR_TERM_REASON_SET_POS: c_int = 12;
pub const GHCB_MSR_TERM_REASON_SET_MASK: c_uint = 0xf;
pub const GHCB_MSR_TERM_REASON_POS: c_int = 16;
pub const GHCB_MSR_TERM_REASON_MASK: c_uint = 0xff;

// GHCBData[15:12] */				\
// GHCBData[23:16] */				\
// Error codes from reason set 0
pub const SEV_TERM_SET_GEN: c_int = 0;
pub const GHCB_SEV_ES_GEN_REQ: c_int = 0;
pub const GHCB_SEV_ES_PROT_UNSUPPORTED: c_int = 1;
pub const GHCB_SNP_UNSUPPORTED: c_int = 2;
// Linux-specific reason codes (used with reason set 1)
pub const SEV_TERM_SET_LINUX: c_int = 1;

//
// GHCB-defined return codes that are communicated back to the guest via
// SW_EXITINFO1.
//
pub const GHCB_HV_RESP_NO_ACTION: c_int = 0;
pub const GHCB_HV_RESP_ISSUE_EXCEPTION: c_int = 1;
pub const GHCB_HV_RESP_MALFORMED_INPUT: c_int = 2;
//
// GHCB-defined sub-error codes for malformed input (see above) that are
// communicated back to the guest via SW_EXITINFO2[31:0].
//
pub const GHCB_ERR_NOT_REGISTERED: c_int = 1;
pub const GHCB_ERR_INVALID_USAGE: c_int = 2;
pub const GHCB_ERR_INVALID_SCRATCH_AREA: c_int = 3;
pub const GHCB_ERR_MISSING_INPUT: c_int = 4;
pub const GHCB_ERR_INVALID_INPUT: c_int = 5;
pub const GHCB_ERR_INVALID_EVENT: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_config {
//
// Indicates when the per-CPU GHCB has been created and registered
// and thus can be used by the BSP instead of the early boot GHCB.
//
// For APs, the per-CPU GHCB is created before they are started
// and registered upon startup, so this flag can be used globally
// for the BSP and APs.
//
// Indicates when the per-CPU SVSM CA is to be used instead of the
// boot SVSM CA.
//
// For APs, the per-CPU SVSM CA is created as part of the AP
// bringup, so this flag can be used globally for the BSP and APs.
//
    pub 61: __reserved :,
}
