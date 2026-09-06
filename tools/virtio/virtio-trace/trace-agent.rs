//! Automatically rewritten from C Header to Rust Module
//! Source: tools/virtio/virtio-trace/trace-agent.h
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

pub const MAX_CPUS: c_int = 256;

//
// agent_info - structure managing total information of guest agent
// @pipe_size:	size of pipe (default 1MB)
// @use_stdout:	set to true when o option is added (default false)
// @cpus:	total number of CPUs
// @ctl_fd:	fd of control path, /dev/virtio-ports/agent-ctl-path
// @rw_ti:	structure managing information of read/write threads
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct agent_info {
    pub pipe_size: c_ulong,
    pub use_stdout: bool,
    pub cpus: c_int,
    pub ctl_fd: c_int,
    pub rw_ti: [*mut rw_thread_info; MAX_CPUS],
}

//
// rw_thread_info - structure managing a read/write thread a cpu
// @cpu_num:	cpu number operating this read/write thread
// @in_fd:	fd of reading trace data path in cpu_num
// @out_fd:	fd of writing trace data path in cpu_num
// @read_pipe:	fd of read pipe
// @write_pipe:	fd of write pipe
// @pipe_size:	size of pipe (default 1MB)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rw_thread_info {
    pub cpu_num: c_int,
    pub in_fd: c_int,
    pub out_fd: c_int,
    pub read_pipe: c_int,
    pub write_pipe: c_int,
    pub pipe_size: c_ulong,
}

// use for stopping rw threads
// use for notification
// for controller of read/write threads
extern "C" {
    pub fn rw_ctl_init(ctl_path: *const c_char) -> c_int;
}
// for trace read/write thread
extern "C" {
    pub fn rw_thread_run(rw_ti: *mut rw_thread_info) -> pthread_t;
}
extern "C" {
    pub fn calloc(_arg: 1, _arg: size) -> return;
}

