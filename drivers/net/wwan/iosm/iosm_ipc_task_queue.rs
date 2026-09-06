//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/iosm/iosm_ipc_task_queue.h
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
// Copyright (C) 2020-21 Intel Corporation.
//
// Number of available element for the input message queue of the IPC
// ipc_task
//
pub const IPC_THREAD_QUEUE_SIZE: c_int = 256;
//
// struct ipc_task_queue_args - Struct for Task queue elements
// @ipc_imem:   Pointer to struct iosm_imem
// @msg:        Message argument for tasklet function. (optional, can be NULL)
// @completion: OS object used to wait for the tasklet function to finish for
// synchronous calls
// @func:       Function to be called in tasklet (tl) context
// @arg:        Generic integer argument for tasklet function (optional)
// @size:       Message size argument for tasklet function (optional)
// @response:   Return code of tasklet function for synchronous calls
// @is_copy:    Is true if msg contains a pointer to a copy of the original msg
// for async. calls that needs to be freed once the tasklet returns
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_task_queue_args {
    pub ipc_imem: *mut iosm_imem,
    pub msg: *mut c_void,
    pub completion: *mut completion,
    pub size): usize,
    pub arg: c_int,
    pub size: usize,
    pub response: c_int,
    pub is_copy:1: u8,
}

//
// struct ipc_task_queue - Struct for Task queue
// @q_lock:     Protect the message queue of the ipc ipc_task
// @args:       Message queue of the IPC ipc_task
// @q_rpos:     First queue element to process.
// @q_wpos:     First free element of the input queue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_task_queue {
    pub /: *mut *mut spinlock_t q_lock; / for atomic operation on queue,
    pub args: [ipc_task_queue_args; IPC_THREAD_QUEUE_SIZE],
    pub q_rpos: c_uint,
    pub q_wpos: c_uint,
}

//
// struct ipc_task - Struct for Task
// @dev:	 Pointer to device structure
// @ipc_tasklet: Tasklet for serialized work offload
// from interrupts and OS callbacks
// @ipc_queue:	 Task for entry into ipc task queue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_task {
    pub dev: *mut device,
    pub ipc_tasklet: *mut tasklet_struct,
    pub ipc_queue: ipc_task_queue,
}

//
// ipc_task_init - Allocate a tasklet
// @ipc_task:	Pointer to ipc_task structure
// Returns: 0 on success and failure value on error.
//
extern "C" {
    pub fn ipc_task_init(ipc_task: *mut ipc_task) -> c_int;
}
//
// ipc_task_deinit - Free a tasklet, invalidating its pointer.
// @ipc_task:	Pointer to ipc_task structure
//
extern "C" {
    pub fn ipc_task_deinit(ipc_task: *mut ipc_task);
}
//
// ipc_task_queue_send_task - Synchronously/Asynchronously call a function in
// tasklet context.
// @imem:		Pointer to iosm_imem struct
// @func:		Function to be called in tasklet context
// @arg:		Integer argument for func
// @msg:		Message pointer argument for func
// @size:		Size argument for func
// @wait:		if true wait for result
//
// Returns: Result value returned by func or failure value if func could not
// be called.
//
