//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cn_proc.h
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


//
// cn_proc.h - process events connector
//
// Copyright (C) Matt Helsley, IBM Corp. 2005
// Based on cn_fork.h by Nguyen Anh Quynh and Guillaume Thouvenin
// Copyright (C) 2005 Nguyen Anh Quynh <aquynh@gmail.com>
// Copyright (C) 2005 Guillaume Thouvenin <guillaume.thouvenin@bull.net>
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of version 2.1 of the GNU Lesser General Public License
// as published by the Free Software Foundation.
//
// This program is distributed in the hope that it would be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
//

extern "C" {
    pub fn proc_fork_connector(task: *mut task_struct);
}
extern "C" {
    pub fn proc_exec_connector(task: *mut task_struct);
}
extern "C" {
    pub fn proc_id_connector(task: *mut task_struct, which_id: c_int);
}
extern "C" {
    pub fn proc_sid_connector(task: *mut task_struct);
}
extern "C" {
    pub fn proc_ptrace_connector(task: *mut task_struct, which_id: c_int);
}
extern "C" {
    pub fn proc_comm_connector(task: *mut task_struct);
}
extern "C" {
    pub fn proc_coredump_connector(task: *mut task_struct);
}
extern "C" {
    pub fn proc_exit_connector(task: *mut task_struct);
}

