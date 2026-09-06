//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/vmx/vmcs12.h
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
// struct vmcs12 describes the state that our guest hypervisor (L1) keeps for a
// single nested guest (L2), hence the name vmcs12. Any VMX implementation has
// a VMCS structure, and vmcs12 is our emulated VMX's VMCS. This structure is
// stored in guest memory specified by VMPTRLD, but is opaque to the guest,
// which must access it using VMREAD/VMWRITE/VMCLEAR instructions.
// More than one of these structures may exist, if L1 runs multiple L2 guests.
// nested_vmx_run() will use the data here to build the vmcs02: a VMCS for the
// underlying hardware which will be used to run L2.
// This structure is packed to ensure that its layout is identical across
// machines (necessary for live migration).
//
// IMPORTANT: Changing the layout of existing fields in this structure
// will break save/restore compatibility with older kvm releases. When
// adding new fields, either use space in the reserved padding* arrays
// or add the new fields to the end of the structure.
//
pub type natural_width = u64;
// According to the Intel spec, a VMCS region must start with the
// following two fields. Then follow implementation-specific data.
//
// To allow migration of L1 (complete with its L2 guests) between
// machines of different natural widths (32 or 64 bit), we cannot have
// unsigned long fields with no explicit size. We use u64 (aliased
// natural_width) instead. Luckily, x86 is little-endian.
//
// VMCS12_REVISION is KVM's arbitrary ID for the layout of struct vmcs12.  KVM
// enumerates this value to L1 via MSR_IA32_VMX_BASIC, and checks the revision
// ID during nested VMPTRLD to verify that L1 is loading a VMCS that adhere's
// to KVM's virtual CPU definition.
//
// DO NOT change this value, as it will break save/restore compatibility with
// older KVM releases.
//
pub const VMCS12_REVISION: c_uint = 0x11e57ed0;
//
// VMCS12_SIZE is the number of bytes L1 should allocate for the VMXON region
// and any VMCS region. Although only sizeof(struct vmcs12) are used by the
// current implementation, 4K are reserved to avoid future complications and
// to preserve userspace ABI.
//

//
// For save/restore compatibility, the vmcs12 field offsets must not change,
// although appending fields and/or filling gaps is obviously allowed.
//

extern "C" {
    pub fn nested_vmx_setup_vmcs12_fields() -> void __init;
}
// (u16 *)p = field_value;
// (u32 *)p = field_value;
// (u64 *)p = field_value;
// (natural_width *)p = field_value;
