//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/kvm_vcpu_timer.h
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
// Copyright (C) 2019 Western Digital Corporation or its affiliates.
//
// Authors:
// Atish Patra <atish.patra@wdc.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_guest_timer {
// Mult & Shift values to get nanoseconds from cycles
    pub nsec_mult: u32,
    pub nsec_shift: u32,
// Time delta value
    pub time_delta: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_timer {
// Flag for whether init is done
    pub init_done: bool,
// Flag for whether timer event is configured
    pub next_set: bool,
// Next timer event cycles
    pub next_cycles: u64,
// Underlying hrtimer instance
    pub hrt: hrtimer,
// Flag to check if sstc is enabled or not
    pub sstc_enabled: bool,
// A function pointer to switch between stimecmp or hrtimer at runtime
    pub ncycles): *mut *mut *mut int (timer_next_event)(struct kvm_vcpu vcpu, u64,
}

extern "C" {
    pub fn kvm_riscv_vcpu_timer_next_event(vcpu: *mut kvm_vcpu, ncycles: u64) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_vcpu_timer_init(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_vcpu_timer_deinit(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_vcpu_timer_reset(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_vcpu_timer_restore(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_guest_timer_init(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_riscv_vcpu_timer_sync(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_vcpu_timer_save(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_vcpu_timer_pending(vcpu: *mut kvm_vcpu) -> bool;
}
