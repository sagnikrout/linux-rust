//! Automatically rewritten from C to Rust
//! Source: arch/x86/xen/irq.c
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
// Force a proper event-channel callback from Xen after clearing the
// callback mask. We do this in a very simple manner, by making a call
// down into Xen. The pending flag will be checked by Xen on return.
//
#[no_mangle]
pub unsafe extern "C" fn xen_force_evtchn_callback() -> noinstr void {
    noinstr void xen_force_evtchn_callback(void)
    {
    (void)HYPERVISOR_xen_version(0, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn xen_safe_halt() -> noinstr void {
    static noinstr void xen_safe_halt(void)
    {
// Blocking includes an implicit local_irq_enable().
    if (HYPERVISOR_sched_op(SCHEDOP_block, core::ptr::null_mut()) != 0)
    BUG();
    }
#[no_mangle]
unsafe extern "C" fn xen_halt() {
    static void xen_halt(void)
    {
    if (irqs_disabled())
    HYPERVISOR_vcpu_op(VCPUOP_down,
    xen_vcpu_nr(smp_processor_id()), core::ptr::null_mut());
    else
    xen_safe_halt();
    }
#[no_mangle]
pub unsafe extern "C" fn xen_init_irq_ops() -> void __init {
    void __init xen_init_irq_ops(void)
    {
// Initial interrupt flag handling only called while interrupts off.
    pv_ops.irq.save_fl = __PV_IS_CALLEE_SAVE(paravirt_ret0);
    pv_ops.irq.irq_disable = __PV_IS_CALLEE_SAVE(paravirt_nop);
    pv_ops.irq.irq_enable = __PV_IS_CALLEE_SAVE(BUG_func);
    pv_ops.irq.safe_halt = xen_safe_halt;
    pv_ops.irq.halt = xen_halt;
    x86_init.irqs.intr_init = xen_init_IRQ;
    }
