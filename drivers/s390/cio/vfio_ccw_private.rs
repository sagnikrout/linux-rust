//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/cio/vfio_ccw_private.h
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
// Private stuff for vfio_ccw driver
//
// Copyright IBM Corp. 2017
// Copyright Red Hat, Inc. 2019
//
// Author(s): Dong Jia Shi <bjsdjshi@linux.vnet.ibm.com>
// Xiao Feng Ren <renxiaof@linux.vnet.ibm.com>
// Cornelia Huck <cohuck@redhat.com>
//

pub const VFIO_CCW_OFFSET_SHIFT: c_int = 10;

// capability chain handling similar to vfio-pci
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_ccw_regops {
    pub ppos): *mut size_t count, loff_t,
    pub ppos): *const *const char __user buf, size_t count, loff_t,
    pub region): *mut vfio_ccw_region,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_ccw_region {
    pub type: u32,
    pub subtype: u32,
    pub ops: *const vfio_ccw_regops,
    pub data: *mut c_void,
    pub size: usize,
    pub flags: u32,
}

extern "C" {
    pub fn vfio_ccw_unregister_dev_regions(private: *mut vfio_ccw_private);
}
extern "C" {
    pub fn vfio_ccw_register_async_dev_regions(private: *mut vfio_ccw_private) -> c_int;
}
extern "C" {
    pub fn vfio_ccw_register_schib_dev_regions(private: *mut vfio_ccw_private) -> c_int;
}
extern "C" {
    pub fn vfio_ccw_register_crw_dev_regions(private: *mut vfio_ccw_private) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_ccw_crw {
    pub next: list_head,
    pub crw: crw,
}

//
// struct vfio_ccw_parent
//
// @dev: embedded device struct
// @parent: parent data structures for mdevs created
// @mdev_type(s): identifying information for mdevs created
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_ccw_parent {
    pub dev: device,
    pub parent: mdev_parent,
    pub mdev_type: mdev_type,
    pub mdev_types: *mut mdev_type,
}

//
// struct vfio_ccw_private
// @vdev: Embedded VFIO device
// @state: internal state of the device
// @completion: synchronization helper of the I/O completion
// @io_region: MMIO region to input/output I/O arguments/results
// @io_mutex: protect against concurrent update of I/O resources
// and @cp lifecycle
// @region: additional regions for other subchannel operations
// @cmd_region: MMIO region for asynchronous I/O commands other than START
// @schib_region: MMIO region for SCHIB information
// @crw_region: MMIO region for getting channel report words
// @num_regions: number of additional regions
// @cp: channel program for the current I/O operation
// @irb: irb info received from interrupt
// @scsw: scsw info
// @crw_lock: serialization of CRW list information
// @crw: list of Channel Report Word elements
// @io_trigger: eventfd ctx for signaling userspace I/O results
// @crw_trigger: eventfd ctx for signaling userspace CRW information
// @req_trigger: eventfd ctx for signaling userspace to return device
// @io_work: work for deferral process of I/O handling
// @crw_work: work for deferral process of CRW handling
// @notoper_work: work for deferred processing in not-operational state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_ccw_private {
    pub vdev: vfio_device,
    pub state: c_int,
    pub completion: *mut completion,
    pub io_region: *mut ccw_io_region,
    pub io_mutex: mutex,
    pub region: *mut vfio_ccw_region,
    pub cmd_region: *mut ccw_cmd_region,
    pub schib_region: *mut ccw_schib_region,
    pub crw_region: *mut ccw_crw_region,
    pub num_regions: c_int,
    pub cp: channel_program,
    pub irb: irb,
    pub scsw: scsw,
    pub crw_lock: spinlock_t,
    pub crw: list_head,
    pub io_trigger: *mut eventfd_ctx,
    pub crw_trigger: *mut eventfd_ctx,
    pub req_trigger: *mut eventfd_ctx,
    pub io_work: work_struct,
    pub crw_work: work_struct,
    pub notoper_work: work_struct,
    pub __aligned(8): },
    pub work): *mut void vfio_ccw_sch_io_todo(struct work_struct,
    pub work): *mut void vfio_ccw_crw_todo(struct work_struct,
    pub work): *mut void vfio_ccw_notoper_todo(struct work_struct,
    pub vfio_ccw_mdev_driver: extern struct mdev_driver,
//
// States of the device statemachine.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vfio_ccw_state {
    VFIO_CCW_STATE_NOT_OPER,
    VFIO_CCW_STATE_STANDBY,
    VFIO_CCW_STATE_IDLE,
    VFIO_CCW_STATE_CP_PROCESSING,
    VFIO_CCW_STATE_CP_PENDING,
// last element!
    NR_VFIO_CCW_STATES
}

//
// Asynchronous events of the device statemachine.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vfio_ccw_event {
    VFIO_CCW_EVENT_NOT_OPER,
    VFIO_CCW_EVENT_IO_REQ,
    VFIO_CCW_EVENT_INTERRUPT,
    VFIO_CCW_EVENT_ASYNC_REQ,
    VFIO_CCW_EVENT_OPEN,
    VFIO_CCW_EVENT_CLOSE,
// last element!
    NR_VFIO_CCW_EVENTS
}

//
// Action called through jumptable.
//
    pub vfio_ccw_event): *mut *mut typedef void (fsm_func_t)(struct vfio_ccw_private , enum,
    pub vfio_ccw_jumptable: [*mut extern fsm_func_t; NR_VFIO_CCW_STATES][NR_VFIO_CCW_EVENTS],
    pub to_subchannel(private->vdev.dev->parent): *mut *mut subchannel sch =,
    pub event): trace_vfio_ccw_fsm_event(sch->schid, private->state,,
    pub event): vfio_ccw_jumptable[private->state][event](private,,
    pub vfio_ccw_work_q: *mut extern struct workqueue_struct,
    pub vfio_ccw_io_region: *mut extern struct kmem_cache,
    pub vfio_ccw_cmd_region: *mut extern struct kmem_cache,
    pub vfio_ccw_schib_region: *mut extern struct kmem_cache,
    pub vfio_ccw_crw_region: *mut extern struct kmem_cache,
// s390 debug feature, similar to base cio
    pub vfio_ccw_debug_msg_id: *mut extern debug_info_t,
    pub vfio_ccw_debug_trace_id: *mut extern debug_info_t,

    pub length): debug_event(vfio_ccw_debug_trace_id, level, data,,
