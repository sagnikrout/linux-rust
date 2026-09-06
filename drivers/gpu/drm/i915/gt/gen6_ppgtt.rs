//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/gen6_ppgtt.h
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
// Copyright © 2020 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gen6_ppgtt {
    pub base: i915_ppgtt,
    pub flush: mutex,
    pub vma: *mut i915_vma,
    pub pd_addr: *mut gen6_pte_t __iomem,
    pub pp_dir: u32,
    pub pin_count: core::sync::atomic::AtomicI32,
    pub scan_for_unused_pt: bool,
}

extern "C" {
    pub fn i915_pte_index(_arg: addr, _arg: GEN6_PDE_SHIFT) -> return;
}
extern "C" {
    pub fn i915_pte_count(_arg: addr, _arg: length, _arg: GEN6_PDE_SHIFT) -> return;
}
extern "C" {
    pub fn i915_pde_index(_arg: addr, _arg: GEN6_PDE_SHIFT) -> return;
}

extern "C" {
    pub fn __to_gen6_ppgtt(_arg: base) -> return;
}
//
// gen6_for_each_pde() iterates over every pde from start until start+length.
// If start and start+length are not perfectly divisible, the macro will round
// down and up as needed. Start=0 and length=2G effectively iterates over
// every PDE in the system. The macro modifies ALL its parameters except 'pd',
// so each of the other parameters should preferably be a simple variable, or
// at most an lvalue with no side-effects!
//

extern "C" {
    pub fn gen6_ppgtt_pin(base: *mut i915_ppgtt, ww: *mut i915_gem_ww_ctx) -> c_int;
}
extern "C" {
    pub fn gen6_ppgtt_unpin(base: *mut i915_ppgtt);
}
extern "C" {
    pub fn gen6_ppgtt_enable(gt: *mut intel_gt);
}
extern "C" {
    pub fn gen7_ppgtt_enable(gt: *mut intel_gt);
}
