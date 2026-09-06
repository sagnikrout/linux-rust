//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/arm/queue.h
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
// linux/drivers/acorn/scsi/queue.h: queue handling
//
// Copyright (C) 1997 Russell King
//
// Function: void queue_initialise (Queue_t *queue)
// Purpose : initialise a queue
// Params  : queue - queue to initialise
//
extern "C" {
    pub fn queue_initialise(queue: *mut Queue_t) -> c_int;
}
//
// Function: void queue_free (Queue_t *queue)
// Purpose : free a queue
// Params  : queue - queue to free
//
extern "C" {
    pub fn queue_free(queue: *mut Queue_t);
}
//
// Function: struct scsi_cmnd *queue_remove (queue)
// Purpose : removes first SCSI command from a queue
// Params  : queue   - queue to remove command from
// Returns : struct scsi_cmnd if successful (and a reference), or NULL if no command available
//
// Function: struct scsi_cmnd *queue_remove_exclude_ref (queue, exclude)
// Purpose : remove a SCSI command from a queue
// Params  : queue   - queue to remove command from
// exclude - array of busy LUNs
// Returns : struct scsi_cmnd if successful (and a reference), or NULL if no command available
//

//
// Function: int __queue_add(Queue_t *queue, struct scsi_cmnd *SCpnt, int head)
// Purpose : Add a new command onto a queue
// Params  : queue - destination queue
// SCpnt - command to add
// head  - add command to head of queue
// Returns : 0 on error, !0 on success
//
extern "C" {
    pub fn __queue_add(queue: *mut Queue_t, SCpnt: *mut scsi_cmnd, head: c_int) -> c_int;
}
//
// Function: struct scsi_cmnd *queue_remove_tgtluntag (queue, target, lun, tag)
// Purpose : remove a SCSI command from the queue for a specified target/lun/tag
// Params  : queue  - queue to remove command from
// target - target that we want
// lun    - lun on device
// tag    - tag on device
// Returns : struct scsi_cmnd if successful, or NULL if no command satisfies requirements
//
// Function: queue_remove_all_target(queue, target)
// Purpose : remove all SCSI commands from the queue for a specified target
// Params  : queue  - queue to remove command from
// target - target device id
// Returns : nothing
//
extern "C" {
    pub fn queue_remove_all_target(queue: *mut Queue_t, target: c_int);
}
//
// Function: int queue_probetgtlun (queue, target, lun)
// Purpose : check to see if we have a command in the queue for the specified
// target/lun.
// Params  : queue  - queue to look in
// target - target we want to probe
// lun    - lun on target
// Returns : 0 if not found, != 0 if found
//
extern "C" {
    pub fn queue_probetgtlun(queue: *mut Queue_t, target: c_int, lun: c_int) -> c_int;
}
//
// Function: int queue_remove_cmd (Queue_t *queue, struct scsi_cmnd *SCpnt)
// Purpose : remove a specific command from the queues
// Params  : queue - queue to look in
// SCpnt - command to find
// Returns : 0 if not found
//
extern "C" {
    pub fn queue_remove_cmd(queue: *mut Queue_t, SCpnt: *mut scsi_cmnd) -> c_int;
}
