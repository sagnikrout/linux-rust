//! Automatically rewritten from C Header to Rust Module
//! Source: include/kvm/arm_arch_timer.h
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
// Copyright (C) 2012 ARM Ltd.
// Author: Marc Zyngier <marc.zyngier@arm.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_arch_timers {
    TIMER_PTIMER,
    TIMER_VTIMER,
    NR_KVM_EL0_TIMERS,
    TIMER_HVTIMER = NR_KVM_EL0_TIMERS,
    TIMER_HPTIMER,
    NR_KVM_TIMERS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_arch_timer_regs {
    TIMER_REG_CNT,
    TIMER_REG_CVAL,
    TIMER_REG_TVAL,
    TIMER_REG_CTL,
    TIMER_REG_VOFF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_timer_offset {
//
// If set, pointer to one of the offsets in the kvm's offset
// structure. If NULL, assume a zero offset.
//
    pub vm_offset: *mut u64,
//
// If set, pointer to one of the offsets in the vcpu's sysreg
// array. If NULL, assume a zero offset.
//
    pub vcpu_offset: *mut u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_timer_vm_data {
// Offset applied to the virtual timer/counter
    pub voffset: u64,
// Offset applied to the physical timer/counter
    pub poffset: u64,
// The PPI for each timer, global to the VM
    pub ppi: [u32; NR_KVM_TIMERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_timer_context {
// Emulated Timer (may be unused)
    pub hrtimer: hrtimer,
    pub ns_frac: u64,
// Offset for this counter/timer
    pub offset: arch_timer_offset,
//
// We have multiple paths which can save/restore the timer state onto
// the hardware, so we need some way of keeping track of where the
// latest state is.
//
    pub loaded: bool,
// Who am I?
    pub timer_id: kvm_arch_timers,
// Duplicated state from arch_timer.c for convenience
    pub host_timer_irq: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timer_map {
    pub direct_vtimer: *mut arch_timer_context,
    pub direct_ptimer: *mut arch_timer_context,
    pub emul_vtimer: *mut arch_timer_context,
    pub emul_ptimer: *mut arch_timer_context,
}

extern "C" {
    pub fn get_timer_map(vcpu: *mut kvm_vcpu, map: *mut timer_map);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_timer_cpu {
    pub timers: [arch_timer_context; NR_KVM_TIMERS],
// Background timer used when the guest is not running
    pub bg_timer: hrtimer,
// Is the timer enabled
    pub enabled: bool,
}

extern "C" {
    pub fn kvm_timer_hyp_init(has_gic: bool) -> int __init;
}
extern "C" {
    pub fn kvm_timer_enable(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_timer_vcpu_reset(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_timer_vcpu_init(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_timer_sync_nested(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_timer_sync_user(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_timer_should_notify_user(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn kvm_timer_update_run(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn kvm_timer_vcpu_terminate(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_timer_init_vm(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_arm_timer_set_attr(vcpu: *mut kvm_vcpu, attr: *mut kvm_device_attr) -> c_int;
}
extern "C" {
    pub fn kvm_arm_timer_get_attr(vcpu: *mut kvm_vcpu, attr: *mut kvm_device_attr) -> c_int;
}
extern "C" {
    pub fn kvm_arm_timer_has_attr(vcpu: *mut kvm_vcpu, attr: *mut kvm_device_attr) -> c_int;
}
extern "C" {
    pub fn kvm_phys_timer_read() -> u64;
}
extern "C" {
    pub fn kvm_timer_vcpu_load(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_timer_vcpu_put(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_timer_init_vhe();
}

// Needed for tracing
extern "C" {
    pub fn timer_get_ctl(ctxt: *mut arch_timer_context) -> u32;
}
extern "C" {
    pub fn timer_get_cval(ctxt: *mut arch_timer_context) -> u64;
}
// CPU HP callbacks
extern "C" {
    pub fn kvm_timer_cpu_up();
}
extern "C" {
    pub fn kvm_timer_cpu_down();
}
// CNTKCTL_EL1 valid bits as of DDI0487J.a

extern "C" {
    pub fn static_branch_unlikely(_arg: &broken_cntvoff_key) -> return;
}

