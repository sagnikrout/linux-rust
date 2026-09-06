//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/vmx/hyperv_evmcs.h
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
// This file contains common definitions for working with Enlightened VMCS which
// are used both by Hyper-V on KVM and KVM on Hyper-V.
//

pub const KVM_EVMCS_VERSION: c_int = 1;
//
// Enlightened VMCSv1 doesn't support these:
//
// POSTED_INTR_NV                  = 0x00000002,
// GUEST_INTR_STATUS               = 0x00000810,
// APIC_ACCESS_ADDR		= 0x00002014,
// POSTED_INTR_DESC_ADDR           = 0x00002016,
// EOI_EXIT_BITMAP0                = 0x0000201c,
// EOI_EXIT_BITMAP1                = 0x0000201e,
// EOI_EXIT_BITMAP2                = 0x00002020,
// EOI_EXIT_BITMAP3                = 0x00002022,
// GUEST_PML_INDEX			= 0x00000812,
// PML_ADDRESS			= 0x0000200e,
// VM_FUNCTION_CONTROL             = 0x00002018,
// EPTP_LIST_ADDRESS               = 0x00002024,
// VMREAD_BITMAP                   = 0x00002026,
// VMWRITE_BITMAP                  = 0x00002028,
//
// TSC_MULTIPLIER                  = 0x00002032,
// PLE_GAP                         = 0x00004020,
// PLE_WINDOW                      = 0x00004022,
// VMX_PREEMPTION_TIMER_VALUE      = 0x0000482E,
//
// Currently unsupported in KVM:
// GUEST_IA32_RTIT_CTL		= 0x00002814,
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct evmcs_field {
    pub offset: u16,
    pub clean_field: u16,
}

//
// Use offset=0 to detect holes in eVMCS. This offset belongs to
// 'revision_id' but this field has no encoding and is supposed to
// be accessed directly.
//
// clean_field = evmcs_field->clean_field;
//
// vmcs12_read_any() doesn't care whether the supplied structure
// is 'struct vmcs12' or 'struct hv_enlightened_vmcs' as it takes
// the exact offset of the required field, use it for convenience
// here.
//
extern "C" {
    pub fn vmcs12_read_any()evmcs: *mut (void, _arg: field, _arg: offset) -> return;
}
