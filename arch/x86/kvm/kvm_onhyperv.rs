//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/kvm_onhyperv.h
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
// KVM L1 hypervisor optimizations on Hyper-V.
//

extern "C" {
    pub fn hv_flush_remote_tlbs_range(kvm: *mut kvm, gfn: gfn_t, nr_pages: gfn_t) -> c_int;
}
extern "C" {
    pub fn hv_flush_remote_tlbs(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn hv_track_root_tdp(vcpu: *mut kvm_vcpu, root_tdp: hpa_t);
}
//
// Partition assist page is something which Hyper-V running in L0
// requires from KVM running in L1 before direct TLB flush for L2
// guests can be enabled. KVM doesn't currently use the page but to
// comply with TLFS it still needs to be allocated. For now, this
// is a single page shared among all vCPUs.
//
// p_hv_pa_pg = kzalloc(PAGE_SIZE, GFP_KERNEL_ACCOUNT);
extern "C" {
    pub fn __pa(_arg: *mut p_hv_pa_pg) -> return;
}

