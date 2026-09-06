//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/cn_proc.h
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


// SPDX-License-Identifier: LGPL-2.1 WITH Linux-syscall-note
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

//
// Userspace sends this enum to register with the kernel that it is listening
// for events on the connector.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum proc_cn_mcast_op {
    PROC_CN_MCAST_LISTEN = 1,
    PROC_CN_MCAST_IGNORE = 2
}

//
// If you add an entry in proc_cn_event, make sure you add it in
// PROC_EVENT_ALL above as well.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum proc_cn_event {
// Use successive bits so the enums can be used to record
// sets of events as well
//
    PROC_EVENT_NONE = 0x00000000,
    PROC_EVENT_FORK = 0x00000001,
    PROC_EVENT_EXEC = 0x00000002,
    PROC_EVENT_UID  = 0x00000004,
    PROC_EVENT_GID  = 0x00000040,
    PROC_EVENT_SID  = 0x00000080,
    PROC_EVENT_PTRACE = 0x00000100,
    PROC_EVENT_COMM = 0x00000200,
// "next" should be 0x00000400
// "last" is the last process event: exit,
// while "next to last" is coredumping event
// before that is report only if process dies
// with non-zero exit status
//
    PROC_EVENT_NONZERO_EXIT = 0x20000000,
    PROC_EVENT_COREDUMP = 0x40000000,
    PROC_EVENT_EXIT = 0x80000000
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_input {
    pub mcast_op: proc_cn_mcast_op,
    pub event_type: proc_cn_event,
}

//
// From the user's point of view, the process
// ID is the thread group ID and thread ID is the internal
// kernel "pid". So, fields are assigned as follow:
//
// In user space     -  In  kernel space
//
// parent process ID  =  parent->tgid
// parent thread  ID  =  parent->pid
// child  process ID  =  child->tgid
// child  thread  ID  =  child->pid
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_event {
    pub what: proc_cn_event,
    pub cpu: __u32,
// C attribute field omitted
// Number of nano seconds since system boot
    pub err: __u32,
    pub ack: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fork_proc_event {
    pub parent_pid: __kernel_pid_t,
    pub parent_tgid: __kernel_pid_t,
    pub child_pid: __kernel_pid_t,
    pub child_tgid: __kernel_pid_t,
    pub fork: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exec_proc_event {
    pub process_pid: __kernel_pid_t,
    pub process_tgid: __kernel_pid_t,
    pub exec: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct id_proc_event {
    pub process_pid: __kernel_pid_t,
    pub process_tgid: __kernel_pid_t,
    pub /: *mut *mut __u32 ruid; / task uid,
    pub /: *mut *mut __u32 rgid; / task gid,
    pub r: },
    pub euid: __u32,
    pub egid: __u32,
    pub e: },
    pub id: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sid_proc_event {
    pub process_pid: __kernel_pid_t,
    pub process_tgid: __kernel_pid_t,
    pub sid: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptrace_proc_event {
    pub process_pid: __kernel_pid_t,
    pub process_tgid: __kernel_pid_t,
    pub tracer_pid: __kernel_pid_t,
    pub tracer_tgid: __kernel_pid_t,
    pub ptrace: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct comm_proc_event {
    pub process_pid: __kernel_pid_t,
    pub process_tgid: __kernel_pid_t,
    pub comm: [c_char; 16],
    pub comm: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coredump_proc_event {
    pub process_pid: __kernel_pid_t,
    pub process_tgid: __kernel_pid_t,
    pub parent_pid: __kernel_pid_t,
    pub parent_tgid: __kernel_pid_t,
    pub coredump: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exit_proc_event {
    pub process_pid: __kernel_pid_t,
    pub process_tgid: __kernel_pid_t,
    pub exit_signal: __u32 exit_code,,
    pub parent_pid: __kernel_pid_t,
    pub parent_tgid: __kernel_pid_t,
    pub exit: },
    pub event_data: },
}
