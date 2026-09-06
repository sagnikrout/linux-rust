//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/kvm/book3s_hv.h
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
// Privileged (non-hypervisor) host registers to save.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct p9_host_os_sprs {
    pub iamr: c_ulong,
    pub amr: c_ulong,
    pub pmc1: c_uint,
    pub pmc2: c_uint,
    pub pmc3: c_uint,
    pub pmc4: c_uint,
    pub pmc5: c_uint,
    pub pmc6: c_uint,
    pub mmcr0: c_ulong,
    pub mmcr1: c_ulong,
    pub mmcr2: c_ulong,
    pub mmcr3: c_ulong,
    pub mmcra: c_ulong,
    pub siar: c_ulong,
    pub sier1: c_ulong,
    pub sier2: c_ulong,
    pub sier3: c_ulong,
    pub sdar: c_ulong,
}

extern "C" {
    pub fn store_vcpu_state(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn save_p9_host_os_sprs(host_os_sprs: *mut p9_host_os_sprs);
}

extern "C" {
    pub fn accumulate_time(vcpu: *mut kvm_vcpu, next: *mut kvmhv_tb_accumulator);
}

