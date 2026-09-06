//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/irq/internals.h
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
// IRQ subsystem internal functions and variables:
//
// Do not ever include this file from anything else than
// kernel/irq/. Do not even think about using any information outside
// of this file for your non core code.
//

//
// Bits used by threaded handlers:
// IRQTF_RUNTHREAD - signals that the interrupt handler thread should run
// IRQTF_WARNED    - warning "IRQ_WAKE_THREAD w/o thread_fn" has been printed
// IRQTF_AFFINITY  - irq thread is requested to adjust affinity
// IRQTF_FORCED_THREAD  - irq action is force threaded
// IRQTF_READY     - signals that irq thread is ready
//
// Bit masks for desc->core_internal_state__do_not_mess_with_it
//
// IRQS_AUTODETECT		- autodetection in progress
// IRQS_SPURIOUS_DISABLED	- was disabled due to spurious interrupt
// detection
// IRQS_POLL_INPROGRESS		- polling in progress
// IRQS_ONESHOT			- irq is not unmasked in primary handler
// IRQS_REPLAY			- irq has been resent and will not be resent
// again until the handler has run and cleared
// this flag.
// IRQS_WAITING			- irq is waiting
// IRQS_PENDING			- irq needs to be resent and should be resent
// at the next available opportunity.
// IRQS_SUSPENDED		- irq is suspended
// IRQS_NMI			- irq line is used to deliver NMIs
// IRQS_SYSFS			- descriptor has been added to sysfs
//

extern "C" {
    pub fn __irq_set_trigger(desc: *mut irq_desc, flags: c_ulong) -> c_int;
}
extern "C" {
    pub fn __disable_irq(desc: *mut irq_desc);
}
extern "C" {
    pub fn __enable_irq(desc: *mut irq_desc);
}

extern "C" {
    pub fn irq_activate(desc: *mut irq_desc) -> c_int;
}
extern "C" {
    pub fn irq_activate_and_startup(desc: *mut irq_desc, resend: bool) -> c_int;
}
extern "C" {
    pub fn irq_startup(desc: *mut irq_desc, resend: bool, force: bool) -> c_int;
}
extern "C" {
    pub fn irq_startup_managed(desc: *mut irq_desc);
}
extern "C" {
    pub fn irq_shutdown(desc: *mut irq_desc);
}
extern "C" {
    pub fn irq_shutdown_and_deactivate(desc: *mut irq_desc);
}
extern "C" {
    pub fn irq_disable(desc: *mut irq_desc);
}
extern "C" {
    pub fn irq_percpu_enable(desc: *mut irq_desc, cpu: c_uint);
}
extern "C" {
    pub fn irq_percpu_disable(desc: *mut irq_desc, cpu: c_uint);
}
extern "C" {
    pub fn mask_irq(desc: *mut irq_desc);
}
extern "C" {
    pub fn unmask_irq(desc: *mut irq_desc);
}
extern "C" {
    pub fn unmask_threaded_irq(desc: *mut irq_desc);
}

extern "C" {
    pub fn irq_desc_free_rcu(desc: *mut irq_desc);
}
extern "C" {
    pub fn rcuref_get(_arg: &desc->refcnt) -> return;
}

extern "C" {
    pub fn irq_mark_irq(irq: c_uint);
}

extern "C" {
    pub fn __handle_irq_event_percpu(desc: *mut irq_desc) -> irqreturn_t;
}
extern "C" {
    pub fn handle_irq_event_percpu(desc: *mut irq_desc) -> irqreturn_t;
}
extern "C" {
    pub fn handle_irq_event(desc: *mut irq_desc) -> irqreturn_t;
}
// Resending of interrupts :
extern "C" {
    pub fn check_irq_resend(desc: *mut irq_desc, inject: bool) -> c_int;
}
extern "C" {
    pub fn clear_irq_resend(desc: *mut irq_desc);
}
extern "C" {
    pub fn irq_resend_init(desc: *mut irq_desc);
}
extern "C" {
    pub fn __irq_wake_thread(desc: *mut irq_desc, action: *mut irqaction);
}
extern "C" {
    pub fn wake_threads_waitq(desc: *mut irq_desc);
}

extern "C" {
    pub fn register_irq_proc(irq: c_uint, desc: *mut irq_desc);
}
extern "C" {
    pub fn unregister_irq_proc(irq: c_uint, desc: *mut irq_desc);
}
extern "C" {
    pub fn register_handler_proc(irq: c_uint, action: *mut irqaction);
}
extern "C" {
    pub fn unregister_handler_proc(irq: c_uint, action: *mut irqaction);
}
extern "C" {
    pub fn irq_proc_update_valid(desc: *mut irq_desc);
}

extern "C" {
    pub fn irq_can_set_affinity_usr(irq: c_uint) -> bool;
}
extern "C" {
    pub fn irq_affinity_schedule_notify_work(desc: *mut irq_desc);
}

extern "C" {
    pub fn irq_setup_affinity(desc: *mut irq_desc) -> c_int;
}

// Inline functions for support of irq chips on slow busses

extern "C" {
    pub fn __irq_put_desc_unlock(desc: *mut irq_desc, flags: c_ulong, bus: bool);
}

extern "C" {
    pub fn __irqd_to_state(_arg: d) -> return;
}
//
// Manipulation functions for irq_data.state
//

extern "C" {
    pub fn irq_common_data_get_node(_arg: &desc->irq_common_data) -> return;
}

extern "C" {
    pub fn irq_pm_handle_wakeup(desc: *mut irq_desc);
}
extern "C" {
    pub fn irq_pm_install_action(desc: *mut irq_desc, action: *mut irqaction);
}
extern "C" {
    pub fn irq_pm_remove_action(desc: *mut irq_desc, action: *mut irqaction);
}

extern "C" {
    pub fn irqd_is_setaffinity_pending(_arg: data) -> return;
}
extern "C" {
    pub fn irq_fixup_move_pending(desc: *mut irq_desc, force_clear: bool) -> bool;
}
extern "C" {
    pub fn irq_force_complete_move(desc: *mut irq_desc);
}

