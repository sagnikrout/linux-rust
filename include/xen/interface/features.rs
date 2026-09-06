//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/features.h
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
// features.h
//
// Feature flags, reported by XENVER_get_features.
//
// Copyright (c) 2006, Keir Fraser <keir@xensource.com>
//
// If set, the guest does not need to write-protect its pagetables, and can
// update them via direct writes.
//
pub const XENFEAT_writable_page_tables: c_int = 0;
//
// If set, the guest does not need to write-protect its segment descriptor
// tables, and can update them via direct writes.
//
pub const XENFEAT_writable_descriptor_tables: c_int = 1;
//
// If set, translation between the guest's 'pseudo-physical' address space
// and the host's machine address space are handled by the hypervisor. In this
// mode the guest does not need to perform phys-to/from-machine translations
// when performing page table operations.
//
pub const XENFEAT_auto_translated_physmap: c_int = 2;
// If set, the guest is running in supervisor mode (e.g., x86 ring 0).
pub const XENFEAT_supervisor_mode_kernel: c_int = 3;
//
// If set, the guest does not need to allocate x86 PAE page directories
// below 4GB. This flag is usually implied by auto_translated_physmap.
//
pub const XENFEAT_pae_pgdir_above_4gb: c_int = 4;
// x86: Does this Xen host support the MMU_PT_UPDATE_PRESERVE_AD hypercall?
pub const XENFEAT_mmu_pt_update_preserve_ad: c_int = 5;
// x86: Does this Xen host support the MMU_{CLEAR,COPY}_PAGE hypercall?
pub const XENFEAT_highmem_assist: c_int = 6;
//
// If set, GNTTABOP_map_grant_ref honors flags to be placed into guest kernel
// available pte bits.
//
pub const XENFEAT_gnttab_map_avail_bits: c_int = 7;
// x86: Does this Xen host support the HVM callback vector type?
pub const XENFEAT_hvm_callback_vector: c_int = 8;
// x86: pvclock algorithm is safe to use on HVM
pub const XENFEAT_hvm_safe_pvclock: c_int = 9;
// x86: pirq can be used by HVM guests
pub const XENFEAT_hvm_pirqs: c_int = 10;
// operation as Dom0 is supported
pub const XENFEAT_dom0: c_int = 11;
// Xen also maps grant references at pfn = mfn.
// This feature flag is deprecated and should not be used.
pub const XENFEAT_grant_map_identity: c_int = 12;
//
// Guest can use XENMEMF_vnode to specify virtual node for memory op.
pub const XENFEAT_memory_op_vnode_supported: c_int = 13;
// arm: Hypervisor supports ARM SMC calling convention.
pub const XENFEAT_ARM_SMCCC_supported: c_int = 14;
//
// x86/PVH: If set, ACPI RSDP can be placed at any address. Otherwise RSDP
// must be located in lower 1MB, as required by ACPI Specification for IA-PC
// systems.
// This feature flag is only consulted if XEN_ELFNOTE_GUEST_OS contains
// the "linux" string.
//
pub const XENFEAT_linux_rsdp_unrestricted: c_int = 15;
//
// A direct-mapped (or 1:1 mapped) domain is a domain for which its
// local pages have gfn == mfn. If a domain is direct-mapped,
// XENFEAT_direct_mapped is set; otherwise XENFEAT_not_direct_mapped
// is set.
//
// If neither flag is set (e.g. older Xen releases) the assumptions are:
// - not auto_translated domains (x86 only) are always direct-mapped
// - on x86, auto_translated domains are not direct-mapped
// - on ARM, Dom0 is direct-mapped, DomUs are not
//
pub const XENFEAT_not_direct_mapped: c_int = 16;
pub const XENFEAT_direct_mapped: c_int = 17;
pub const XENFEAT_NR_SUBMAPS: c_int = 1;
