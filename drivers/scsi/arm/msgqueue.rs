//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/arm/msgqueue.h
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
// linux/drivers/acorn/scsi/msgqueue.h
//
// Copyright (C) 1997 Russell King
//
// message queue handling
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct message {
    pub msg: [c_char; 8],
    pub length: c_int,
    pub fifo: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msgqueue_entry {
    pub msg: message,
    pub next: *mut msgqueue_entry,
}

pub const NR_MESSAGES: c_int = 4;
//
// Function: void msgqueue_initialise(MsgQueue_t *msgq)
// Purpose : initialise a message queue
// Params  : msgq - queue to initialise
//
extern "C" {
    pub fn msgqueue_initialise(msgq: *mut MsgQueue_t);
}
//
// Function: void msgqueue_free(MsgQueue_t *msgq)
// Purpose : free a queue
// Params  : msgq - queue to free
//
extern "C" {
    pub fn msgqueue_free(msgq: *mut MsgQueue_t);
}
//
// Function: int msgqueue_msglength(MsgQueue_t *msgq)
// Purpose : calculate the total length of all messages on the message queue
// Params  : msgq - queue to examine
// Returns : number of bytes of messages in queue
//
extern "C" {
    pub fn msgqueue_msglength(msgq: *mut MsgQueue_t) -> c_int;
}
//
// Function: struct message *msgqueue_getmsg(MsgQueue_t *msgq, int msgno)
// Purpose : return a message & its length
// Params  : msgq   - queue to obtain message from
// : msgno  - message number
// Returns : pointer to message string, or NULL
//
// Function: int msgqueue_addmsg(MsgQueue_t *msgq, int length, ...)
// Purpose : add a message onto a message queue
// Params  : msgq   - queue to add message on
// length - length of message
// ...    - message bytes
// Returns : != 0 if successful
//
extern "C" {
    pub fn msgqueue_addmsg(msgq: *mut MsgQueue_t, length: c_int, ...) -> c_int;
}
//
// Function: void msgqueue_flush(MsgQueue_t *msgq)
// Purpose : flush all messages from message queue
// Params  : msgq - queue to flush
//
extern "C" {
    pub fn msgqueue_flush(msgq: *mut MsgQueue_t);
}
