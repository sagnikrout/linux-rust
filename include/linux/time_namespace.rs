//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/time_namespace.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timens_offsets {
    pub monotonic: timespec64,
    pub boottime: timespec64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct time_namespace {
    pub user_ns: *mut user_namespace,
    pub ucounts: *mut ucounts,
    pub ns: ns_common,
    pub offsets: timens_offsets,

    pub vvar_page: *mut page,

// If set prevents changing offsets after any task joined namespace.
    pub frozen_offsets: bool,
    pub __randomize_layout: },
    pub init_time_ns: extern struct time_namespace,

    pub ns): return container_of(ns, struct time_namespace,,
    pub time_ns_init(void): void __init,
    pub ns: return,
    pub old_ns): *mut time_namespace,
    pub ns): *mut void free_time_ns(struct time_namespace,
    pub tsk): *mut *mut void timens_on_fork(struct nsproxy nsproxy, struct task_struct,
    pub m): *mut *mut void proc_timens_show_offsets(struct task_struct p, struct seq_file,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_timens_offset {
    pub clockid: c_int,
    pub val: timespec64,
}

// ts = timespec64_add(*ts, ns_offsets->monotonic);
// ts = timespec64_add(*ts, ns_offsets->boottime);
// ts = timespec64_sub(*ts, ns_offsets->boottime);
extern "C" {
    pub fn do_timens_ktime_to_host(_arg: clockid, _arg: tim, _arg: &ns->offsets) -> return;
}

extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}

extern "C" {
    pub fn timens_commit(tsk: *mut task_struct, ns: *mut time_namespace);
}

