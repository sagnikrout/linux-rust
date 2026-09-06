//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kvm_irqfd.h
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
// irqfd: Allows an fd to be used to inject an interrupt to the guest
// Credit goes to Avi Kivity for the original idea.
//

//
// Resampling irqfds are a special variety of irqfds used to emulate
// level triggered interrupts.  The interrupt is asserted on eventfd
// trigger.  On acknowledgment through the irq ack notifier, the
// interrupt is de-asserted and userspace is notified through the
// resamplefd.  All resamplers on the same gsi are de-asserted
// together, so we don't need to track the state of each individual
// user.  We can also therefore share the same irq source ID.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_kernel_irqfd_resampler {
    pub kvm: *mut kvm,
//
// List of resampling struct _irqfd objects sharing this gsi.
// RCU list modified under kvm->irqfds.resampler_lock
//
    pub list: list_head,
    pub notifier: kvm_irq_ack_notifier,
//
// Entry in list of kvm->irqfd.resampler_list.  Use for sharing
// resamplers among irqfds on the same gsi.
// RCU list modified under kvm->irqfds.resampler_lock
//
    pub link: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_kernel_irqfd {
// Used for MSI fast-path
    pub kvm: *mut kvm,
    pub wait: wait_queue_entry_t,
// Update side is protected by irqfds.lock
    pub irq_entry: kvm_kernel_irq_routing_entry,
    pub irq_entry_sc: seqcount_spinlock_t,
// Used for level IRQ fast-path
    pub gsi: c_int,
    pub inject: work_struct,
// The resampler used by this irqfd (resampler-only)
    pub resampler: *mut kvm_kernel_irqfd_resampler,
// Eventfd notified on resample (resampler-only)
    pub resamplefd: *mut eventfd_ctx,
// Entry in list of irqfds for a resampler (resampler-only)
    pub resampler_link: list_head,
// Used for setup/shutdown
    pub eventfd: *mut eventfd_ctx,
    pub list: list_head,
    pub shutdown: work_struct,
    pub consumer: irq_bypass_consumer,
    pub producer: *mut irq_bypass_producer,
    pub irq_bypass_vcpu: *mut kvm_vcpu,
    pub vcpu_list: list_head,
    pub irq_bypass_data: *mut c_void,
}
