//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/trace/rv/monitors/deadline/deadline.h
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
// Dummy values if not available
//

// Initialised when registering the deadline container
//
// If both have dummy values, the syscalls are not supported and we don't even
// need to register the handler.
//
// is_supported_type - return true if @type is supported by the deadline monitors
//
// is_server_type - return true if @type is a supported server
//
// Use negative numbers for the server.
// Currently only one fair server per CPU, may change in the future.
//

//
// Get a unique id used for dl entities
//
// The cpu is not required for tasks as the pid is used there, if this function
// is called on a dl_se that for sure corresponds to a task, DL_TASK can be
// used in place of cpu.
// We need the cpu for servers as it is provided in the tracepoint and we
// cannot easily retrieve it from the dl_se (requires the struct rq definition).
//
extern "C" {
    pub fn fair_server_id(_arg: cpu) -> return;
}
extern "C" {
    pub fn ext_server_id(_arg: cpu) -> return;
}
// Expand id and target as arguments for da functions

// pid_out = args[0];
//
// Just copy up to sched_flags, we are not interested after that
//
// Helper functions requiring DA/HA utilities

//
// get_fair_server - get the fair server associated to a task
//
// If the task is a boosted task, the server is available in the task_struct,
// otherwise grab the dl entity saved for the CPU where the task is enqueued.
// This function assumes the task is enqueued somewhere.
//
extern "C" {
    pub fn da_get_target_by_id(_arg: fair_server_id(task_cpu(tsk))) -> return;
}
extern "C" {
    pub fn da_get_target_by_id(_arg: ext_server_id(task_cpu(tsk))) -> return;
}
//
// Initialise monitors for all tasks and pre-allocate the storage for servers.
// This is necessary since we don't have access to the servers here and
// allocation can cause deadlocks from their tracepoints. We can only fill
// pre-initialised storage from there.
//
// Might be superfluous as tasks are not started with this policy..
