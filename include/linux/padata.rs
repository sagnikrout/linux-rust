//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/padata.h
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
// padata.h - header for the padata parallelization interface
//
// Copyright (C) 2008, 2009 secunet Security Networks AG
// Copyright (C) 2008, 2009 Steffen Klassert <steffen.klassert@secunet.com>
//
// Copyright (c) 2020 Oracle and/or its affiliates.
// Author: Daniel Jordan <daniel.m.jordan@oracle.com>
//

pub const PADATA_CPU_SERIAL: c_uint = 0x01;
pub const PADATA_CPU_PARALLEL: c_uint = 0x02;
//
// struct padata_priv - Represents one job
//
// @list: List entry, to attach to the padata lists.
// @pd: Pointer to the internal control structure.
// @cb_cpu: Callback cpu for serializatioon.
// @seq_nr: Sequence number of the parallelized data object.
// @info: Used to pass information from the parallel to the serial function.
// @parallel: Parallel execution function.
// @serial: Serial complete function.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct padata_priv {
    pub list: list_head,
    pub pd: *mut parallel_data,
    pub cb_cpu: c_int,
    pub seq_nr: c_uint,
    pub info: c_int,
    pub padata): *mut *mut void (parallel)(struct padata_priv,
    pub padata): *mut *mut void (serial)(struct padata_priv,
}

//
// struct padata_list - one per work type per CPU
//
// @list: List head.
// @lock: List lock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct padata_list {
    pub list: list_head,
    pub lock: spinlock_t,
}

//
// struct padata_serial_queue - The percpu padata serial queue
//
// @serial: List to wait for serialization after reordering.
// @work: work struct for serialization.
// @pd: Backpointer to the internal control structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct padata_serial_queue {
    pub serial: padata_list,
    pub work: work_struct,
    pub pd: *mut parallel_data,
}

//
// struct padata_cpumask - The cpumasks for the parallel/serial workers
//
// @pcpu: cpumask for the parallel workers.
// @cbcpu: cpumask for the serial (callback) workers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct padata_cpumask {
    pub pcpu: cpumask_var_t,
    pub cbcpu: cpumask_var_t,
}

//
// struct parallel_data - Internal control structure, covers everything
// that depends on the cpumask in use.
//
// @ps: padata_shell object.
// @reorder_list: percpu reorder lists
// @squeue: percpu padata queues used for serialuzation.
// @refcnt: Number of objects holding a reference on this parallel_data.
// @seq_nr: Sequence number of the parallelized data object.
// @processed: Number of already processed objects.
// @cpu: Next CPU to be processed.
// @cpumask: The cpumasks in use for parallel and serial workers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct parallel_data {
    pub ps: *mut padata_shell,
    pub reorder_list: *mut padata_list __percpu,
    pub squeue: *mut padata_serial_queue __percpu,
    pub refcnt: refcount_t,
    pub seq_nr: c_uint,
    pub processed: c_uint,
    pub cpu: c_int,
    pub cpumask: padata_cpumask,
}

//
// struct padata_shell - Wrapper around struct parallel_data, its
// purpose is to allow the underlying control structure to be replaced
// on the fly using RCU.
//
// @pinst: padat instance.
// @pd: Actual parallel_data structure which may be substituted on the fly.
// @opd: Pointer to old pd to be freed by padata_replace.
// @list: List entry in padata_instance list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct padata_shell {
    pub pinst: *mut padata_instance,
    pub pd: *mut parallel_data __rcu,
    pub opd: *mut parallel_data,
    pub list: list_head,
}

//
// struct padata_mt_job - represents one multithreaded job
//
// @thread_fn: Called for each chunk of work that a padata thread does.
// @fn_arg: The thread function argument.
// @start: The start of the job (units are job-specific).
// @size: size of this node's work (units are job-specific).
// @align: Ranges passed to the thread function fall on this boundary, with the
// possible exceptions of the beginning and end of the job.
// @min_chunk: The minimum chunk size in job-specific units.  This allows
// the client to communicate the minimum amount of work that's
// appropriate for one worker thread to do at once.
// @max_threads: Max threads to use for the job, actual number may be less
// depending on task size and minimum chunk size.
// @numa_aware: Distribute jobs to different nodes with CPU in a round robin fashion.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct padata_mt_job {
    pub arg): *mut *mut void (thread_fn)(unsigned long start, unsigned long end, void,
    pub fn_arg: *mut c_void,
    pub start: c_ulong,
    pub size: c_ulong,
    pub align: c_ulong,
    pub min_chunk: c_ulong,
    pub max_threads: c_int,
    pub numa_aware: bool,
}

//
// struct padata_instance - The overall control structure.
//
// @cpuhp_node: Linkage for CPU hotplug callbacks.
// @parallel_wq: The workqueue used for parallel work.
// @serial_wq: The workqueue used for serial work.
// @pslist: List of padata_shell objects attached to this instance.
// @cpumask: User supplied cpumasks for parallel and serial works.
// @validate_cpumask: Internal cpumask used to validate @cpumask during hotplug.
// @kobj: padata instance kernel object.
// @lock: padata instance lock.
// @flags: padata flags.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct padata_instance {
    pub cpuhp_node: hlist_node,
    pub parallel_wq: *mut workqueue_struct,
    pub serial_wq: *mut workqueue_struct,
    pub pslist: list_head,
    pub cpumask: padata_cpumask,
    pub validate_cpumask: cpumask_var_t,
    pub kobj: kobject,
    pub lock: mutex,
    pub flags: u8,
pub const PADATA_INIT: c_int = 1;
pub const PADATA_RESET: c_int = 2;
pub const PADATA_INVALID: c_int = 4;
}

extern "C" {
    pub fn padata_init() -> void __init;
}
extern "C" {
    pub fn padata_free(pinst: *mut padata_instance);
}
extern "C" {
    pub fn padata_free_shell(ps: *mut padata_shell);
}
extern "C" {
    pub fn padata_do_serial(padata: *mut padata_priv);
}
extern "C" {
    pub fn padata_do_multithreaded(job: *mut padata_mt_job) -> void __init;
}

