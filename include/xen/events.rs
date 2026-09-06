//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/events.h
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

extern "C" {
    pub fn xen_evtchn_nr_channels() -> unsigned;
}
extern "C" {
    pub fn bind_evtchn_to_irq(evtchn: evtchn_port_t) -> c_int;
}
extern "C" {
    pub fn bind_evtchn_to_irq_lateeoi(evtchn: evtchn_port_t) -> c_int;
}
extern "C" {
    pub fn bind_virq_to_irq(virq: c_uint, cpu: c_uint, percpu: bool) -> c_int;
}
//
// Common unbind function for all event sources. Takes IRQ to unbind from.
// Automatically closes the underlying event channel (even for bindings
// made with bind_evtchn_to_irqhandler()).
//
extern "C" {
    pub fn unbind_from_irqhandler(irq: c_uint, dev_id: *mut c_void);
}
//
// Send late EOI for an IRQ bound to an event channel via one of the *_lateeoi
// functions above.
//
extern "C" {
    pub fn xen_irq_lateeoi(irq: c_uint, eoi_flags: c_uint);
}
// Signal an event was spurious, i.e. there was no action resulting from it.
pub const XEN_EOI_FLAG_SPURIOUS: c_uint = 0x00000001;

extern "C" {
    pub fn xen_set_irq_priority(irq: unsigned, priority: unsigned) -> c_int;
}
//
// Allow extra references to event channels exposed to userspace by evtchn
//
extern "C" {
    pub fn evtchn_make_refcounted(evtchn: evtchn_port_t, is_static: bool) -> c_int;
}
extern "C" {
    pub fn evtchn_get(evtchn: evtchn_port_t) -> c_int;
}
extern "C" {
    pub fn evtchn_put(evtchn: evtchn_port_t);
}
extern "C" {
    pub fn xen_send_IPI_one(cpu: c_uint, vector: ipi_vector);
}
extern "C" {
    pub fn rebind_evtchn_irq(evtchn: evtchn_port_t, irq: c_int);
}
extern "C" {
    pub fn notify_remote_via_irq(irq: c_int);
}
extern "C" {
    pub fn xen_irq_resume();
}
// Clear an irq's pending state, in preparation for polling on it
extern "C" {
    pub fn xen_clear_irq_pending(irq: c_int);
}
extern "C" {
    pub fn xen_test_irq_pending(irq: c_int) -> bool;
}
// Poll waiting for an irq to become pending.  In the usual case, the
extern "C" {
    pub fn xen_poll_irq(irq: c_int);
}
// Poll waiting for an irq to become pending with a timeout.  In the usual case,
// the irq will be disabled so it won't deliver an interrupt.
extern "C" {
    pub fn xen_poll_irq_timeout(irq: c_int, timeout: u64);
}
// Determine the IRQ which is bound to an event channel
extern "C" {
    pub fn irq_from_evtchn(evtchn: evtchn_port_t) -> c_uint;
}
extern "C" {
    pub fn xen_set_callback_via(via: u64) -> c_int;
}
extern "C" {
    pub fn xen_evtchn_do_upcall() -> c_int;
}
// Bind a pirq for a physical interrupt to an irq.

// Allocate a pirq for a MSI style physical interrupt.
extern "C" {
    pub fn xen_allocate_pirq_msi(dev: *mut pci_dev, msidesc: *mut msi_desc) -> c_int;
}
// Bind an PSI pirq to an irq.

// De-allocates the above mentioned physical interrupt.
extern "C" {
    pub fn xen_destroy_irq(irq: c_int) -> c_int;
}
// Return the pirq allocated to the irq.
extern "C" {
    pub fn xen_pirq_from_irq(irq: unsigned) -> c_int;
}
// Return the irq allocated to the gsi
extern "C" {
    pub fn xen_irq_from_gsi(gsi: unsigned) -> c_int;
}
// Determine whether to ignore this IRQ if it is passed to a guest.
extern "C" {
    pub fn xen_test_irq_shared(irq: c_int) -> c_int;
}
// initialize Xen IRQ subsystem
extern "C" {
    pub fn xen_init_IRQ();
}
extern "C" {
    pub fn xen_debug_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
