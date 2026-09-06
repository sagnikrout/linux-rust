//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/xive.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2016,2017 IBM Corporation.
//

pub const XIVE_INVALID_VP: c_uint = 0xffffffff;

//
// Thread Interrupt Management Area (TIMA)
//
// This is a global MMIO region divided in 4 pages of varying access
// permissions, providing access to per-cpu interrupt management
// functions. It always identifies the CPU doing the access based
// on the PowerBus initiator ID, thus we always access via the
// same offset regardless of where the code is executing
//
// Offset in the TM area of our current execution level (provided by
// the backend)
//
// Per-irq data (irq_get_handler_data for normal IRQs), IPIs
// have it stored in the xive_cpu structure. We also cache
// for normal interrupts the current target CPU.
//
// This structure is setup by the backend for each interrupt.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xive_irq_data {
    pub flags: u64,
    pub eoi_page: u64,
    pub eoi_mmio: *mut void __iomem,
    pub trig_page: u64,
    pub trig_mmio: *mut void __iomem,
    pub esb_shift: u32,
    pub src_chip: c_int,
    pub hw_irq: u32,
// Setup/used by frontend
    pub target: c_int,
//
// saved_p means that there is a queue entry for this interrupt
// in some CPU's queue (not including guest vcpu queues), even
// if P is not set in the source ESB.
// stale_p means that there is no queue entry for this interrupt
// in some CPU's queue, even if P is set in the source ESB.
//
    pub saved_p: bool,
    pub stale_p: bool,
}

pub const XIVE_IRQ_FLAG_STORE_EOI: c_uint = 0x01;
pub const XIVE_IRQ_FLAG_LSI: c_uint = 0x02;
// #define XIVE_IRQ_FLAG_SHIFT_BUG	0x04 */ /* P9 DD1.0 workaround
// #define XIVE_IRQ_FLAG_MASK_FW	0x08 */ /* P9 DD1.0 workaround
// #define XIVE_IRQ_FLAG_EOI_FW	0x10 */ /* P9 DD1.0 workaround
pub const XIVE_IRQ_FLAG_H_INT_ESB: c_uint = 0x20;
// Special flag set by KVM for excalation interrupts
pub const XIVE_IRQ_FLAG_NO_EOI: c_uint = 0x80;

// A queue tracking structure in a CPU
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xive_q {
    pub qpage: *mut __be32,
    pub msk: u32,
    pub idx: u32,
    pub toggle: u32,
    pub eoi_phys: u64,
    pub esc_irq: u32,
    pub count: core::sync::atomic::AtomicI32,
    pub pending_count: core::sync::atomic::AtomicI32,
    pub guest_qaddr: u64,
    pub guest_qshift: u32,
}

// Global enable flags for the XIVE support
extern "C" {
    pub fn xive_spapr_init() -> bool;
}
extern "C" {
    pub fn xive_native_init() -> bool;
}
extern "C" {
    pub fn xive_smp_probe() -> c_int;
}
extern "C" {
    pub fn xive_smp_prepare_cpu(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn xive_smp_setup_cpu();
}
extern "C" {
    pub fn xive_smp_disable_cpu();
}
extern "C" {
    pub fn xive_teardown_cpu();
}
extern "C" {
    pub fn xive_shutdown();
}
extern "C" {
    pub fn xive_flush_interrupt();
}
// xmon hook
extern "C" {
    pub fn xmon_xive_do_dump(cpu: c_int);
}
extern "C" {
    pub fn xmon_xive_get_irq_config(hw_irq: u32, d: *mut irq_data) -> c_int;
}
extern "C" {
    pub fn xmon_xive_get_irq_all();
}
// APIs used by KVM
extern "C" {
    pub fn xive_native_default_eq_shift() -> u32;
}
extern "C" {
    pub fn xive_native_alloc_vp_block(max_vcpus: u32) -> u32;
}
extern "C" {
    pub fn xive_native_free_vp_block(vp_base: u32);
}
extern "C" {
    pub fn xive_cleanup_irq_data(xd: *mut xive_irq_data);
}
extern "C" {
    pub fn xive_native_free_irq(irq: u32);
}
extern "C" {
    pub fn xive_native_configure_irq(hw_irq: u32, target: u32, prio: u8, sw_irq: u32) -> c_int;
}
extern "C" {
    pub fn xive_native_disable_queue(vp_id: u32, q: *mut xive_q, prio: u8);
}
extern "C" {
    pub fn xive_native_sync_source(hw_irq: u32);
}
extern "C" {
    pub fn xive_native_sync_queue(hw_irq: u32);
}
extern "C" {
    pub fn is_xive_irq(chip: *mut irq_chip) -> bool;
}
extern "C" {
    pub fn xive_native_enable_vp(vp_id: u32, single_escalation: bool) -> c_int;
}
extern "C" {
    pub fn xive_native_disable_vp(vp_id: u32) -> c_int;
}
extern "C" {
    pub fn xive_native_get_vp_info(vp_id: u32, out_cam_id: *mut u32, out_chip_id: *mut u32) -> c_int;
}
extern "C" {
    pub fn xive_native_has_single_escalation() -> bool;
}
extern "C" {
    pub fn xive_native_has_save_restore() -> bool;
}
extern "C" {
    pub fn xive_native_get_vp_state(vp_id: u32, out_state: *mut u64) -> c_int;
}
extern "C" {
    pub fn xive_native_has_queue_state_support() -> bool;
}
extern "C" {
    pub fn xive_native_alloc_irq_on_chip(chip_id: u32) -> u32;
}
extern "C" {
    pub fn xive_native_alloc_irq_on_chip(_arg: OPAL_XIVE_ANY_CHIP) -> return;
}

