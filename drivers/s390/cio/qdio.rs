//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/cio/qdio.h
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
// Copyright IBM Corp. 2000, 2009
// Author(s): Utz Bacher <utz.bacher@de.ibm.com>
// Jan Glauber <jang@linux.vnet.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qdio_irq_states {
    QDIO_IRQ_STATE_INACTIVE,
    QDIO_IRQ_STATE_ESTABLISHED,
    QDIO_IRQ_STATE_ACTIVE,
    QDIO_IRQ_STATE_STOPPED,
    QDIO_IRQ_STATE_CLEANUP,
    QDIO_IRQ_STATE_ERR,
    NR_QDIO_IRQ_STATES,
}

// used as intparm in do_IO
pub const QDIO_DOING_ESTABLISH: c_int = 1;
pub const QDIO_DOING_ACTIVATE: c_int = 2;
pub const QDIO_DOING_CLEANUP: c_int = 3;
pub const SLSB_STATE_NOT_INIT: c_uint = 0x0;
pub const SLSB_STATE_EMPTY: c_uint = 0x1;
pub const SLSB_STATE_PRIMED: c_uint = 0x2;
pub const SLSB_STATE_PENDING: c_uint = 0x3;
pub const SLSB_STATE_HALTED: c_uint = 0xe;
pub const SLSB_STATE_ERROR: c_uint = 0xf;
pub const SLSB_TYPE_INPUT: c_uint = 0x0;
pub const SLSB_TYPE_OUTPUT: c_uint = 0x20;
pub const SLSB_OWNER_PROG: c_uint = 0x80;
pub const SLSB_OWNER_CU: c_uint = 0x40;

pub const SLSB_ERROR_DURING_LOOKUP: c_uint = 0xff;
// additional CIWs returned by extended Sense-ID
pub const CIW_TYPE_EQUEUE: c_uint = 0x3 /* establish QDIO queues */;
pub const CIW_TYPE_AQUEUE: c_uint = 0x4 /* activate QDIO queues */;
// flags for st qdio sch data
pub const CHSC_FLAG_QDIO_CAPABILITY: c_uint = 0x80;
pub const CHSC_FLAG_VALIDITY: c_uint = 0x40;
// SIGA flags
pub const QDIO_SIGA_WRITE: c_uint = 0x00;
pub const QDIO_SIGA_READ: c_uint = 0x01;
pub const QDIO_SIGA_SYNC: c_uint = 0x02;
pub const QDIO_SIGA_WRITEM: c_uint = 0x03;
pub const QDIO_SIGA_WRITEQ: c_uint = 0x04;
pub const QDIO_SIGA_QEBSM_FLAG: c_uint = 0x80;
// count = _ccq & 0xff;
// start = _queuestart & 0xff;
// count = _ccq & 0xff;
// start = _queuestart & 0xff;
// state = _state & 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdio_dev_perf_stat {
    pub adapter_int: c_uint,
    pub qdio_int: c_uint,
    pub siga_read: c_uint,
    pub siga_write: c_uint,
    pub siga_sync: c_uint,
    pub inbound_call: c_uint,
    pub stop_polling: c_uint,
    pub inbound_queue_full: c_uint,
    pub outbound_call: c_uint,
    pub outbound_queue_full: c_uint,
    pub fast_requeue: c_uint,
    pub target_full: c_uint,
    pub eqbs: c_uint,
    pub eqbs_partial: c_uint,
    pub sqbs: c_uint,
    pub sqbs_partial: c_uint,
    pub int_discarded: c_uint,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdio_queue_perf_stat {
// Sorted into order-2 buckets: 1, 2-3, 4-7, ... 64-127, 128.
    pub nr_sbals: [c_uint; 8],
    pub nr_sbal_error: c_uint,
    pub nr_sbal_nop: c_uint,
    pub nr_sbal_total: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qdio_irq_poll_states {
    QDIO_IRQ_DISABLED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdio_input_q {
// Batch of SBALs that we processed while polling the queue:
    pub batch_start: c_uint,
    pub batch_count: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdio_output_q {
}

//
// Note on cache alignment: grouped slsb and write mostly data at the beginning
// sbal[] is read-only and starts on a new cacheline followed by read mostly.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdio_q {
    pub slsb: slsb,
    pub in: qdio_input_q,
    pub out: qdio_output_q,
    pub u: },
//
// inbound: next buffer the program should check for
// outbound: next buffer to check if adapter processed it
//
    pub first_to_check: c_int,
// number of buffers in use by the adapter
    pub nr_buf_used: core::sync::atomic::AtomicI32,
// last scan of the queue
    pub timestamp: u64,
    pub q_stats: qdio_queue_perf_stat,
    pub ____cacheline_aligned: *mut *mut qdio_buffer sbal[QDIO_MAX_BUFFERS_PER_Q],
// queue number
    pub nr: c_int,
// bitmask of queue number
    pub mask: c_int,
// input or output queue
    pub is_input_q: c_int,
// upper-layer program handler
    pub (*handler): *mut qdio_handler_t,
    pub irq_ptr: *mut qdio_irq,
// memory page (PAGE_SIZE) used to place slib and sl on
    pub sl_page: *mut c_void,
    pub sl: *mut sl,
    pub slib: *mut slib,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdio_irq {
    pub qib: qib,
    pub /: *mut *mut *mut u32 dsci; / address of device state change indicator,
    pub cdev: *mut ccw_device,
    pub /: *mut *mut list_head entry; / list of thinint devices,
    pub debugfs_dev: *mut dentry,
    pub last_data_irq_time: u64,
    pub int_parm: c_ulong,
    pub schid: subchannel_id,
    pub /: *mut *mut unsigned long sch_token; / QEBSM facility,
    pub state: qdio_irq_states,
    pub qdioac1: u8,
    pub nr_input_qs: c_int,
    pub nr_output_qs: c_int,
    pub ccw: *mut ccw1,
    pub ssqd_desc: qdio_ssqd_desc,
    pub ): *mut *mut *mut void (orig_handler) (struct ccw_device , unsigned long, struct irb,
    pub (*error_handler): *mut qdio_handler_t,
    pub perf_stat_enabled: c_int,
    pub qdr: *mut qdr,
    pub chsc_page: c_ulong,
    pub input_qs: [*mut qdio_q; QDIO_MAX_QUEUES_PER_IRQ],
    pub output_qs: [*mut qdio_q; QDIO_MAX_QUEUES_PER_IRQ],
    pub max_input_qs: c_uint,
    pub max_output_qs: c_uint,
    pub data): *mut *mut *mut void (irq_poll)(struct ccw_device cdev, unsigned long,
    pub poll_state: c_ulong,
    pub debug_area: *mut debug_info_t,
    pub setup_mutex: mutex,
    pub perf_stat: qdio_dev_perf_stat,
}

// helper functions

// the highest iqdio queue is used for multicast

// prototypes for thin interrupt
extern "C" {
    pub fn qdio_establish_thinint(irq_ptr: *mut qdio_irq) -> c_int;
}
extern "C" {
    pub fn qdio_shutdown_thinint(irq_ptr: *mut qdio_irq);
}
extern "C" {
    pub fn qdio_thinint_init() -> c_int;
}
extern "C" {
    pub fn qdio_thinint_exit();
}
extern "C" {
    pub fn test_nonshared_ind(: *mut qdio_irq) -> c_int;
}
// prototypes for setup
extern "C" {
    pub fn qdio_setup_ssqd_info(irq_ptr: *mut qdio_irq);
}
extern "C" {
    pub fn qdio_setup_irq(irq_ptr: *mut qdio_irq, init_data: *mut qdio_initialize);
}
extern "C" {
    pub fn qdio_shutdown_irq(irq: *mut qdio_irq);
}
extern "C" {
    pub fn qdio_print_subchannel_info(irq_ptr: *mut qdio_irq);
}
extern "C" {
    pub fn qdio_free_queues(irq_ptr: *mut qdio_irq);
}
extern "C" {
    pub fn qdio_setup_init() -> c_int;
}
extern "C" {
    pub fn qdio_setup_exit();
}
