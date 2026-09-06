//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/char/tape.h
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
// tape device driver for 3490E tapes.
//
// S390 and zSeries version
// Copyright IBM Corp. 2001, 2009
// Author(s): Carsten Otte <cotte@de.ibm.com>
// Tuan Ngo-Anh <ngoanh@de.ibm.com>
// Martin Schwidefsky <schwidefsky@de.ibm.com>
// Stefan Bader <shbader@de.ibm.com>
//

//
// Define DBF_LIKE_HELL for lots of messages in the debug feature.
//
// Macro flag: #define DBF_LIKE_HELL

//
// macros s390 debug feature (dbf)
//

pub const TAPE_VERSION_MAJOR: c_int = 2;
pub const TAPE_VERSION_MINOR: c_int = 0;

pub const TAPEBLOCK_HSEC_SIZE: c_int = 2048;
pub const TAPEBLOCK_HSEC_S2B: c_int = 2;
pub const TAPEBLOCK_RETRIES: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tape_medium_state {
    MS_UNKNOWN,
    MS_LOADED,
    MS_UNLOADED,
    MS_SIZE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tape_state {
    TS_UNUSED=0,
    TS_IN_USE,
    TS_BLKUSE,
    TS_INIT,
    TS_NOT_OPER,
    TS_SIZE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tape_op {
    TO_BLOCK,	/* Block read */
    TO_BSB,		/* Backward space block */
    TO_BSF,		/* Backward space filemark */
    TO_DSE,		/* Data security erase */
    TO_FSB,		/* Forward space block */
    TO_FSF,		/* Forward space filemark */
    TO_LBL,		/* Locate block label */
    TO_NOP,		/* No operation */
    TO_RBA,		/* Read backward */
    TO_RBI,		/* Read block information */
    TO_RFO,		/* Read forward */
    TO_REW,		/* Rewind tape */
    TO_RUN,		/* Rewind and unload tape */
    TO_WRI,		/* Write block */
    TO_WTM,		/* Write tape mark */
    TO_MSEN,	/* Medium sense */
    TO_LOAD,	/* Load tape */
    TO_READ_CONFIG, /* Read configuration data */
    TO_READ_ATTMSG, /* Read attention message */
    TO_DIS,		/* Tape display */
    TO_ASSIGN,	/* Assign tape to channel path */
    TO_UNASSIGN,	/* Unassign tape from channel path */
    TO_RDC,		/* Read device characteristics */
    TO_SIZE,	/* #entries in tape_op_t */
}

// Forward declaration
// tape_request->status can be:
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tape_request_status {
    TAPE_REQUEST_INIT,	/* request is ready to be processed */
    TAPE_REQUEST_QUEUED,	/* request is queued to be processed */
    TAPE_REQUEST_IN_IO,	/* request is currently in IO */
    TAPE_REQUEST_DONE,	/* request is completed. */
    TAPE_REQUEST_CANCEL,	/* request should be canceled. */
    TAPE_REQUEST_LONG_BUSY, /* request has to be restarted after long busy */
}

// Tape CCW request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tape_request {
    pub /: *mut *mut list_head list; / list head for request queueing.,
    pub /: *mut *mut *mut tape_device device; / tape device of this request,
    pub /: *mut *mut *mut ccw1 cpaddr; / address of the channel program.,
    pub /: *mut *mut *mut void cpdata; / pointer to ccw data.,
    pub /: *mut *mut tape_request_status status;/ status of this request,
    pub /: *mut *mut int options; / options for execution.,
    pub /: *mut *mut int retries; / retry counter for error recovery.,
    pub /: *mut *mut int rescnt; / residual count from devstat.,
    pub /: *mut *mut timer_list timer; / timer for std_assign_timeout().,
    pub /: *mut *mut irb irb; / device status,
// Callback for delivering final status.
    pub ): *mut *mut *mut void (callback)(struct tape_request , void,
    pub callback_data: *mut c_void,
    pub op: tape_op,
    pub rc: c_int,
}

// Function type for magnetic tape commands
extern "C" {
    pub fn int(: *mut *mut tape_mtop_fn)(struct tape_device, _arg: c_int) -> typedef;
}
// Size of the array containing the mtops for a discipline

// Tape Discipline
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tape_discipline {
    pub owner: *mut module,
    pub ): *mut *mut int (setup_device)(struct tape_device,
    pub ): *mut *mut void (cleanup_device)(struct tape_device,
    pub ): *mut *mut *mut *mut int (irq)(struct tape_device , struct tape_request , struct irb,
    pub ): *mut *mut *mut tape_request (read_block)(tape_device,
    pub ): *mut *mut *mut tape_request (write_block)(tape_device,
    pub tape_device*): *mut *mut void (process_eov)(struct,
// Array of tape commands with TAPE_NR_MTOPS entries
    pub mtop_array: *mut tape_mtop_fn,
}

//
// The discipline irq function either returns an error code (<0) which
// means that the request has failed with an error or one of the following:
//

// Char Frontend Data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tape_char_data {
    pub /: *mut *mut *mut *mut idal_buffer ibs; / idal buffer array for user char data,
    pub /: *mut *mut int block_size; / of size block_size.,
}

// Tape Info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tape_device {
// entry in tape_device_list
    pub node: list_head,
    pub cdev_id: c_int,
    pub cdev: *mut *mut ccw_device,
    pub nt: *mut *mut tape_class_device,
    pub rt: *mut *mut tape_class_device,
// Device mutex to serialize tape commands.
    pub mutex: mutex,
// Device discipline information.
    pub discipline: *mut *mut tape_discipline,
// Generic status flags
    pub tape_generic_status: c_long,
// Device state information.
    pub state_change_wq: wait_queue_head_t,
    pub tape_state: tape_state,
    pub medium_state: tape_medium_state,
    pub modeset_byte: *mut *mut c_uchar,
// Reference count.
    pub ref_count: core::sync::atomic::AtomicI32,
// Request queue.
    pub req_queue: list_head,
// Request wait queue.
    pub wait_queue: wait_queue_head_t,
// Each tape device has (currently) two minor numbers.
    pub first_minor: c_int,
// Number of tapemarks required for correct termination.
    pub required_tapemarks: c_int,
// Block ID of the BOF
    pub bof: c_uint,
// Character device frontend data
    pub char_data: tape_char_data,
// Function to start or stop the next request later.
    pub tape_dnr: delayed_work,
// Timer for long busy
    pub lb_timeout: timer_list,
}

// Externals from tape_core.c
extern "C" {
    pub fn tape_free_request(: *mut tape_request);
}
extern "C" {
    pub fn tape_check_idalbuffer(device: *mut tape_device, size: usize) -> c_int;
}
extern "C" {
    pub fn tape_do_io(: *mut tape_device, : *mut tape_request) -> c_int;
}
extern "C" {
    pub fn tape_do_io_async(: *mut tape_device, : *mut tape_request) -> c_int;
}
extern "C" {
    pub fn tape_do_io_interruptible(: *mut tape_device, : *mut tape_request) -> c_int;
}
extern "C" {
    pub fn tape_cancel_io(: *mut tape_device, : *mut tape_request) -> c_int;
}
extern "C" {
    pub fn tape_open(: *mut tape_device) -> c_int;
}
extern "C" {
    pub fn tape_release(: *mut tape_device) -> c_int;
}
extern "C" {
    pub fn tape_mtop(: *mut tape_device, _arg: c_int, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn tape_state_set(: *mut tape_device, tape_state: enum);
}
extern "C" {
    pub fn tape_generic_online(: *mut tape_device, : *mut tape_discipline) -> c_int;
}
extern "C" {
    pub fn tape_generic_offline(: *mut ccw_device) -> c_int;
}
// Externals from tape_devmap.c
extern "C" {
    pub fn tape_generic_probe(: *mut ccw_device) -> c_int;
}
extern "C" {
    pub fn tape_generic_remove(: *mut ccw_device);
}
extern "C" {
    pub fn tape_put_device(: *mut tape_device);
}
// Externals from tape_char.c
extern "C" {
    pub fn tapechar_init() -> c_int;
}
extern "C" {
    pub fn tapechar_exit();
}
extern "C" {
    pub fn tapechar_setup_device(: *mut tape_device) -> c_int;
}
extern "C" {
    pub fn tapechar_cleanup_device(: *mut tape_device);
}
// Externals from tape_3490.c
extern "C" {
    pub fn tape_3490_init() -> c_int;
}
extern "C" {
    pub fn tape_3490_exit();
}
// tape initialisation functions

extern "C" {
    pub fn tape_proc_init();
}
extern "C" {
    pub fn tape_proc_cleanup();
}

// a function for dumping device sense info
// functions for handling the status of a device
extern "C" {
    pub fn tape_med_state_set(: *mut tape_device, tape_medium_state: enum);
}
// The debug area
// functions for building ccws
// Global vars
