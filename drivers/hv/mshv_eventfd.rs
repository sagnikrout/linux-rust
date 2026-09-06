//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hv/mshv_eventfd.h
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
// irqfd: Allows an fd to be used to inject an interrupt to the guest.
// ioeventfd: Allow an fd to be used to receive a signal from the guest.
// All credit goes to kvm developers.
//

// struct to contain list of irqfds sharing an irq. Updates are protected by
// partition.irqfds.resampler_lock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_irqfd_resampler {
    pub rsmplr_partn: *mut mshv_partition,
    pub rsmplr_irqfd_list: hlist_head,
    pub rsmplr_notifier: mshv_irq_ack_notifier,
    pub rsmplr_hnode: hlist_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_irqfd {
    pub irqfd_partn: *mut mshv_partition,
    pub irqfd_eventfd_ctx: *mut eventfd_ctx,
    pub irqfd_girq_ent: mshv_guest_irq_ent,
    pub irqfd_irqe_sc: seqcount_spinlock_t,
    pub irqfd_irqnum: u32,
    pub irqfd_lapic_irq: mshv_lapic_irq,
    pub irqfd_hnode: hlist_node,
    pub irqfd_polltbl: poll_table,
    pub irqfd_wait: wait_queue_entry_t,
    pub irqfd_shutdown: work_struct,
    pub irqfd_resampler: *mut mshv_irqfd_resampler,
    pub irqfd_resamplefd: *mut eventfd_ctx,
    pub irqfd_resampler_hnode: hlist_node,
}

extern "C" {
    pub fn mshv_eventfd_init(partition: *mut mshv_partition);
}
extern "C" {
    pub fn mshv_eventfd_release(partition: *mut mshv_partition);
}
extern "C" {
    pub fn mshv_notify_acked_gsi(partition: *mut mshv_partition, gsi: c_int) -> bool;
}
extern "C" {
    pub fn mshv_irqfd_wq_init() -> c_int;
}
extern "C" {
    pub fn mshv_irqfd_wq_cleanup();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_ioeventfd {
    pub iovntfd_hnode: hlist_node,
    pub iovntfd_addr: u64,
    pub iovntfd_length: c_int,
    pub iovntfd_eventfd: *mut eventfd_ctx,
    pub iovntfd_datamatch: u64,
    pub iovntfd_doorbell_id: c_int,
    pub iovntfd_wildcard: bool,
}
