//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/stop_machine.h
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
// stop_cpu[s]() is simplistic per-cpu maximum priority cpu
// monopolization mechanism.  The caller can specify a non-sleeping
// function to be executed on a single or multiple cpus preempting all
// other processes and monopolizing those cpus until it finishes.
//
// Resources for this mechanism are preallocated when a cpu is brought
// up and requests are guaranteed to be served as long as the target
// cpus are online.
//
extern "C" {
    pub fn int(arg: *mut *mut cpu_stop_fn_t)(void) -> typedef;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_stop_work {
    pub /: *mut *mut list_head list; / cpu_stopper->works,
    pub fn: cpu_stop_fn_t,
    pub caller: c_ulong,
    pub arg: *mut c_void,
    pub done: *mut cpu_stop_done,
}

extern "C" {
    pub fn stop_one_cpu(cpu: c_uint, fn: cpu_stop_fn_t, arg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn stop_two_cpus(cpu1: c_uint, cpu2: c_uint, fn: cpu_stop_fn_t, arg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn stop_machine_park(cpu: c_int);
}
extern "C" {
    pub fn stop_machine_unpark(cpu: c_int);
}
extern "C" {
    pub fn stop_machine_yield(cpumask: *const cpumask);
}
extern "C" {
    pub fn print_stop_info(log_lvl: *const c_char, task: *mut task_struct);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_stop_work {
    pub work: work_struct,
    pub fn: cpu_stop_fn_t,
    pub arg: *mut c_void,
}

//
// stop_machine "Bogolock": stop the entire machine, disable interrupts.
// This is a very heavy lock, which is equivalent to grabbing every raw
// spinlock (and more).  So the "read" side to such a lock is anything
// which disables preemption.
//

//
// stop_machine: freeze the machine on all CPUs and run this function
// @fn: the function to run
// @data: the data ptr to pass to @fn()
// @cpus: the cpus to run @fn() on (NULL = one unspecified online CPU)
//
// Description: This causes a thread to be scheduled on every CPU, which
// will run with interrupts disabled.  Each CPU specified by @cpus will
// run @fn.  While @fn is executing, there will no other CPUs holding
// a raw spinlock or running within any other type of preempt-disabled
// region of code.
//
// When @cpus specifies only a single CPU, this can be thought of as
// a reader-writer lock where readers disable preemption (for example,
// by holding a raw spinlock) and where the insanely heavy writers run
// @fn while also preventing any other CPU from doing any useful work.
// These writers can also be thought of as having implicitly grabbed every
// raw spinlock in the kernel.
//
// When @fn is a no-op, this can be thought of as an RCU implementation
// where readers again disable preemption and writers use stop_machine()
// in place of synchronize_rcu(), albeit with orders of magnitude more
// disruption than even that of synchronize_rcu_expedited().
//
// Although only one stop_machine() operation can proceed at a time,
// the possibility of blocking in cpus_read_lock() means that the caller
// cannot usefully rely on this serialization.
//
// Return: 0 if all invocations of @fn return zero.  Otherwise, the
// value returned by an arbitrarily chosen member of the set of calls to
// @fn that returned non-zero.
//
extern "C" {
    pub fn stop_machine(fn: cpu_stop_fn_t, data: *mut c_void, cpus: *const cpumask) -> c_int;
}
//
// stop_machine_cpuslocked: freeze the machine on all CPUs and run this function
// @fn: the function to run
// @data: the data ptr to pass to @fn()
// @cpus: the cpus to run @fn() on (NULL = one unspecified online CPU)
//
// Same as above.  Avoids nested calls to cpus_read_lock().
//
// Context: Must be called from within a cpus_read_lock() protected region.
//
extern "C" {
    pub fn stop_machine_cpuslocked(fn: cpu_stop_fn_t, data: *mut c_void, cpus: *const cpumask) -> c_int;
}
//
// stop_core_cpuslocked: - stop all threads on just one core
// @cpu: any cpu in the targeted core
// @fn: the function to run on each CPU in the core containing @cpu
// @data: the data ptr to pass to @fn()
//
// Same as above, but instead of every CPU, only the logical CPUs of the
// single core containing @cpu are affected.
//
// Context: Must be called from within a cpus_read_lock() protected region.
//
// Return: 0 if all invocations of @fn return zero.  Otherwise, the
// value returned by an arbitrarily chosen member of the set of calls to
// @fn that returned non-zero.
//
extern "C" {
    pub fn stop_core_cpuslocked(cpu: c_uint, fn: cpu_stop_fn_t, data: *mut c_void) -> c_int;
}

extern "C" {
    pub fn stop_machine_cpuslocked(_arg: fn, _arg: data, _arg: cpus) -> return;
}
extern "C" {
    pub fn stop_machine(_arg: fn, _arg: data, _arg: cpus) -> return;
}

