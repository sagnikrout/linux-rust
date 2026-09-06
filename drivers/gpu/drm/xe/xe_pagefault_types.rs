//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_pagefault_types.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2025 Intel Corporation
//

// enum xe_pagefault_access_type - Xe page fault access type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_pagefault_access_type {
// @XE_PAGEFAULT_ACCESS_TYPE_READ: Read access type
    XE_PAGEFAULT_ACCESS_TYPE_READ	= 0,
// @XE_PAGEFAULT_ACCESS_TYPE_WRITE: Write access type
    XE_PAGEFAULT_ACCESS_TYPE_WRITE	= 1,
// @XE_PAGEFAULT_ACCESS_TYPE_ATOMIC: Atomic access type
    XE_PAGEFAULT_ACCESS_TYPE_ATOMIC	= 2,
}

// enum xe_pagefault_type - Xe page fault type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_pagefault_type {
// @XE_PAGEFAULT_TYPE_NOT_PRESENT: Not present
    XE_PAGEFAULT_TYPE_NOT_PRESENT			= 0,
// @XE_PAGEFAULT_TYPE_WRITE_ACCESS_VIOLATION: Write access violation
    XE_PAGEFAULT_TYPE_WRITE_ACCESS_VIOLATION	= 1,
// @XE_PAGEFAULT_TYPE_ATOMIC_ACCESS_VIOLATION: Atomic access violation
    XE_PAGEFAULT_TYPE_ATOMIC_ACCESS_VIOLATION	= 2,
}

// struct xe_pagefault_ops - Xe pagefault ops (producer)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_pagefault_ops {
//
// @ack_fault: Ack fault
// @pf: Page fault
// @err: Error state of fault
//
// Page fault producer receives acknowledgment from the consumer and
// sends the result to the HW/FW interface.
//
    pub err): *mut *mut *mut void (ack_fault)(struct xe_pagefault pf, int,
}

//
// struct xe_pagefault - Xe page fault
//
// Generic page fault structure for communication between producer and consumer.
// Carefully sized to be 64 bytes. Upon a device page fault, the producer
// populates this structure, and the consumer copies it into the page-fault
// queue for deferred handling.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_pagefault {
//
// @gt: GT of fault
//
    pub gt: *mut xe_gt,
//
// @consumer: State for the software handling the fault. Populated by
// the producer and may be modified by the consumer to communicate
// information back to the producer upon fault acknowledgment.
//
// @consumer.page_addr: address of page fault
    pub page_addr: u64,
// @consumer.asid: address space ID
    pub asid: u32,
//
// @consumer.access_type: access type and prefetch flag packed
// into a u8.
//
    pub access_type: u8,

//
// @consumer.fault_type_level: fault type and level, u8 rather
// than enum to keep size compact
//
    pub fault_type_level: u8,
pub const XE_PAGEFAULT_TYPE_LEVEL_NACK: c_uint = 0xff	/* Producer indicates nack fault */;

// @consumer.engine_class: engine class
    pub engine_class: u8,
// @consumer.engine_instance: engine instance
    pub engine_instance: u8,
// @consumer.reserved: reserved bits for future expansion
    pub reserved: u64,
    pub consumer: },
//
// @producer: State for the producer (i.e., HW/FW interface). Populated
// by the producer and should not be modified—or even inspected—by the
// consumer, except for calling operations.
//
// @producer.private: private pointer
    pub private: *mut c_void,
// @producer.ops: operations
    pub ops: *const xe_pagefault_ops,
pub const XE_PAGEFAULT_PRODUCER_MSG_LEN_DW: c_int = 4;
//
// @producer.msg: page fault message, used by producer in fault
// acknowledgment to formulate response to HW/FW interface.
// Included in the page-fault message because the producer
// typically receives the fault in a context where memory cannot
// be allocated (e.g., atomic context or the reclaim path).
//
    pub msg: [u32; XE_PAGEFAULT_PRODUCER_MSG_LEN_DW],
    pub producer: },
}

//
// struct xe_pagefault_queue - Xe pagefault queue (consumer)
//
// Used to capture all device page faults for deferred processing. Size this
// queue to absorb the device’s worst-case number of outstanding faults.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_pagefault_queue {
//
// @data: Data in queue containing struct xe_pagefault, protected by
// @lock
//
    pub data: *mut c_void,
// @size: Size of queue in bytes
    pub size: u32,
// @head: Head pointer in bytes, moved by producer, protected by @lock
    pub head: u32,
// @tail: Tail pointer in bytes, moved by consumer, protected by @lock
    pub tail: u32,
// @lock: protects page fault queue
    pub lock: spinlock_t,
// @worker: to process page faults
    pub worker: work_struct,
}
