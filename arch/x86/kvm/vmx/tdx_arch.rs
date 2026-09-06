//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/vmx/tdx_arch.h
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
// architectural constants/data definitions for TDX SEAMCALLs

// TDX control structure (TDR/TDCS/TDVPS) field access codes

pub const TDX_CLASS_SHIFT: c_int = 56;

// Class code for TD

// Class code for TDVPS

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tdx_tdcs_execution_control {
    TD_TDCS_EXEC_TSC_OFFSET = 10,
    TD_TDCS_EXEC_TSC_MULTIPLIER = 11,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tdx_vcpu_guest_other_state {
    TD_VCPU_STATE_DETAILS_NON_ARCH = 0x100,
}

// @field is any of enum tdx_tdcs_execution_control

// @field is the VMCS field encoding

// @field is any of enum tdx_guest_other_state

// Management class fields
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tdx_vcpu_guest_management {
    TD_VCPU_PEND_NMI = 11,
}

// @field is any of enum tdx_vcpu_guest_management

pub const TDX_EXTENDMR_CHUNKSIZE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdx_cpuid_value {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub __packed: },

pub const TDX_EXT_EXIT_QUAL_TYPE_PENDING_EPT_VIOLATION: c_int = 6;
//
// TD_PARAMS is provided as an input to TDH_MNG_INIT, the size of which is 1024B.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct td_params {
    pub attributes: u64,
    pub xfam: u64,
    pub max_vcpus: u16,
    pub reserved0: [u8; 6],
    pub eptp_controls: u64,
    pub config_flags: u64,
    pub tsc_frequency: u16,
    pub reserved1: [u8; 38],
    pub mrconfigid: [u64; 6],
    pub mrowner: [u64; 6],
    pub mrownerconfig: [u64; 6],
    pub reserved2: [u64; 4],
    pub cpuid_values): DECLARE_FLEX_ARRAY(struct tdx_cpuid_value,,
    pub reserved3: [u8; 768],
}

//
// Guest uses MAX_PA for GPAW when set.
// 0: GPA.SHARED bit is GPA[47]
// 1: GPA.SHARED bit is GPA[51]
//

//
// TDH.VP.ENTER, TDG.VP.VMCALL preserves RBP
// 0: RBP can be used for TDG.VP.VMCALL input. RBP is clobbered.
// 1: RBP can't be used for TDG.VP.VMCALL input. RBP is preserved.
//

//
// TDX requires the frequency to be defined in units of 25MHz, which is the
// frequency of the core crystal clock on TDX-capable platforms, i.e. the TDX
// module can only program frequencies that are multiples of 25MHz.  The
// frequency must be between 100mhz and 10ghz (inclusive).
//

// Additional Secure EPT entry information

pub const TDX_SEPT_STATE_SHIFT: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tdx_sept_entry_state {
    TDX_SEPT_FREE = 0,
    TDX_SEPT_BLOCKED = 1,
    TDX_SEPT_PENDING = 2,
    TDX_SEPT_PENDING_BLOCKED = 3,
    TDX_SEPT_PRESENT = 4,
}

//
// TD scope metadata field ID.
//
pub const TD_MD_FIELD_ID_CPUID_VALUES: c_uint = 0x9410000300000000ULL;
