//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/xenpmu.h
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

pub const XENPMU_VER_MAJ: c_int = 0;
pub const XENPMU_VER_MIN: c_int = 1;
//
// ` enum neg_errnoval
// ` HYPERVISOR_xenpmu_op(enum xenpmu_op cmd, struct xenpmu_params *args);
//
// @cmd  == XENPMU_* (PMU operation)
// @args == struct xenpmu_params
//
// ` enum xenpmu_op {

pub const XENPMU_mode_set: c_int = 1;
pub const XENPMU_feature_get: c_int = 2;
pub const XENPMU_feature_set: c_int = 3;
pub const XENPMU_init: c_int = 4;
pub const XENPMU_finish: c_int = 5;
pub const XENPMU_lvtpc_set: c_int = 6;
pub const XENPMU_flush: c_int = 7;
// ` }
// Parameters structure for HYPERVISOR_xenpmu_op call
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pmu_params {
// IN/OUT parameters
    pub maj: u32,
    pub min: u32,
    pub version: },
    pub val: u64,
// IN parameters
    pub vcpu: u32,
    pub pad: u32,
}

// PMU modes:
// - XENPMU_MODE_OFF:   No PMU virtualization
// - XENPMU_MODE_SELF:  Guests can profile themselves
// - XENPMU_MODE_HV:    Guests can profile themselves, dom0 profiles
// itself and Xen
// - XENPMU_MODE_ALL:   Only dom0 has access to VPMU and it profiles
// everyone: itself, the hypervisor and the guests.
//
pub const XENPMU_MODE_OFF: c_int = 0;

//
// PMU features:
// - XENPMU_FEATURE_INTEL_BTS: Intel BTS support (ignored on AMD)
//
pub const XENPMU_FEATURE_INTEL_BTS: c_int = 1;
//
// Shared PMU data between hypervisor and PV(H) domains.
//
// The hypervisor fills out this structure during PMU interrupt and sends an
// interrupt to appropriate VCPU.
// Architecture-independent fields of xen_pmu_data are WO for the hypervisor
// and RO for the guest but some fields in xen_pmu_arch can be writable
// by both the hypervisor and the guest (see arch-$arch/pmu.h).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pmu_data {
// Interrupted VCPU
    pub vcpu_id: u32,
//
// Physical processor on which the interrupt occurred. On non-privileged
// guests set to vcpu_id;
//
    pub pcpu_id: u32,
//
// Domain that was interrupted. On non-privileged guests set to
// DOMID_SELF.
// On privileged guests can be DOMID_SELF, DOMID_XEN, or, when in
// XENPMU_MODE_ALL mode, domain ID of another domain.
//
    pub domain_id: domid_t,
    pub pad: [u8; 6],
// Architecture-specific information
    pub pmu: xen_pmu_arch,
}
