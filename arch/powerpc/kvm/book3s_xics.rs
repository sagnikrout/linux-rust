//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/kvm/book3s_xics.h
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
// Copyright 2012 Michael Ellerman, IBM Corporation.
// Copyright 2012 Benjamin Herrenschmidt, IBM Corporation
//

//
// We use a two-level tree to store interrupt source information.
// There are up to 1024 ICS nodes, each of which can represent
// 1024 sources.
//
pub const KVMPPC_XICS_MAX_ICS_ID: c_int = 1023;
pub const KVMPPC_XICS_ICS_SHIFT: c_int = 10;

//
// Interrupt source numbers below this are reserved, for example
// 0 is "no interrupt", and 2 is used for IPIs.
//
pub const KVMPPC_XICS_FIRST_IRQ: c_int = 16;

// Priority value to use for disabling an interrupt
pub const MASKED: c_uint = 0xff;
pub const PQ_PRESENTED: c_int = 1;
pub const PQ_QUEUED: c_int = 2;
// State for one irq source
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ics_irq_state {
    pub number: u32,
    pub server: u32,
    pub pq_state: u32,
    pub priority: u8,
    pub saved_priority: u8,
    pub resend: u8,
    pub masked_pending: u8,
    pub /: *mut *mut u8 lsi; / level-sensitive interrupt,
    pub exists: u8,
    pub intr_cpu: c_int,
    pub host_irq: u32,
}

// Atomic ICP state, updated with a single compare & swap
#[repr(C)]
#[derive(Copy, Clone)]
pub union kvmppc_icp_state {
    pub raw: c_ulong,
    pub out_ee:1: u8,
    pub need_resend:1: u8,
    pub cppr: u8,
    pub mfrr: u8,
    pub pending_pri: u8,
    pub xisr: u32,
}

// One bit per ICS

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_icp {
    pub vcpu: *mut kvm_vcpu,
    pub server_num: c_ulong,
    pub state: kvmppc_icp_state,
    pub resend_map: [c_ulong; ICP_RESEND_MAP_SIZE],
// Real mode might find something too hard, here's the action
// it might request from virtual mode
//
pub const XICS_RM_KICK_VCPU: c_uint = 0x1;
pub const XICS_RM_CHECK_RESEND: c_uint = 0x2;
pub const XICS_RM_NOTIFY_EOI: c_uint = 0x8;
    pub rm_action: u32,
    pub rm_kick_target: *mut kvm_vcpu,
    pub rm_resend_icp: *mut kvmppc_icp,
    pub rm_reject: u32,
    pub rm_eoied_irq: u32,
// Counters for each reason we exited real mode
    pub n_rm_kick_vcpu: c_ulong,
    pub n_rm_check_resend: c_ulong,
    pub n_rm_notify_eoi: c_ulong,
// Counters for handling ICP processing in real mode
    pub n_check_resend: c_ulong,
    pub n_reject: c_ulong,
// Debug stuff for real mode
    pub rm_dbgstate: kvmppc_icp_state,
    pub rm_dbgtgt: *mut kvm_vcpu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_ics {
    pub lock: arch_spinlock_t,
    pub icsid: u16,
    pub irq_state: [ics_irq_state; KVMPPC_XICS_IRQ_PER_ICS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_xics {
    pub kvm: *mut kvm,
    pub dev: *mut kvm_device,
    pub dentry: *mut dentry,
    pub max_icsid: u32,
    pub real_mode: bool,
    pub real_mode_dbg: bool,
    pub err_noics: u32,
    pub err_noicp: u32,
    pub 1]: *mut *mut kvmppc_ics ics[KVMPPC_XICS_MAX_ICS_ID +,
}

// source = src;
extern "C" {
    pub fn xics_rm_h_xirr(vcpu: *mut kvm_vcpu) -> c_ulong;
}
extern "C" {
    pub fn xics_rm_h_xirr_x(vcpu: *mut kvm_vcpu) -> c_ulong;
}
extern "C" {
    pub fn xics_rm_h_cppr(vcpu: *mut kvm_vcpu, cppr: c_ulong) -> c_int;
}
extern "C" {
    pub fn xics_rm_h_eoi(vcpu: *mut kvm_vcpu, xirr: c_ulong) -> c_int;
}

