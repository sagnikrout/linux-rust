//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/timer.h
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
// NB: because we have to copy the lockdep_map, setting the lockdep_map key
// (second argument) here is required, otherwise it could be initialised to
// the copy of the lockdep_map later! We use the pointer to and the string
// "<file>:<line>" as the key resp. the name of the lockdep_map.
//

// Macro flag: #define __TIMER_LOCKDEP_MAP_INITIALIZER(_kn)

//
// @TIMER_DEFERRABLE: A deferrable timer will work normally when the
// system is busy, but will not cause a CPU to come out of idle just
// to service it; instead, the timer will be serviced when the CPU
// eventually wakes up with a subsequent non-deferrable timer.
//
// @TIMER_IRQSAFE: An irqsafe timer is executed with IRQ disabled and
// it's safe to wait for the completion of the running instance from
// IRQ handlers, for example, by calling timer_delete_sync().
//
// Note: The irq disabled callback execution is a special case for
// workqueue locking issues. It's not meant for executing random crap
// with interrupts disabled. Abuse is monitored!
//
// @TIMER_PINNED: A pinned timer will always expire on the CPU on which the
// timer was enqueued. When a particular CPU is required, add_timer_on()
// has to be used. Enqueue via mod_timer() and add_timer() is always done
// on the local CPU.
//
pub const TIMER_CPUMASK: c_uint = 0x0003FFFF;
pub const TIMER_MIGRATING: c_uint = 0x00040000;

pub const TIMER_DEFERRABLE: c_uint = 0x00080000;
pub const TIMER_PINNED: c_uint = 0x00100000;
pub const TIMER_IRQSAFE: c_uint = 0x00200000;

pub const TIMER_ARRAYSHIFT: c_int = 22;
pub const TIMER_ARRAYMASK: c_uint = 0xFFC00000;

//
// LOCKDEP and DEBUG timer interfaces.
//

//
// timer_setup - prepare a timer for first use
// @timer: the timer in question
// @callback: the function to call when timer expires
// @flags: any TIMER_* flags
//
// Regular timer initialization should use either DEFINE_TIMER() above,
// or timer_setup(). For timers on the stack, timer_setup_on_stack() must
// be used and must be balanced with a call to timer_destroy_on_stack().
//

extern "C" {
    pub fn timer_destroy_on_stack(timer: *mut timer_list);
}

//
// timer_pending - is a timer pending?
// @timer: the timer in question
//
// timer_pending will tell whether a given timer is currently pending,
// or not. Callers must ensure serialization wrt. other operations done
// to this timer, eg. interrupt contexts, or other CPUs on SMP.
//
// Returns: 1 if the timer is pending, 0 if not.
//
extern "C" {
    pub fn add_timer_on(timer: *mut timer_list, cpu: c_int);
}
extern "C" {
    pub fn mod_timer(timer: *mut timer_list, expires: c_ulong) -> c_int;
}
extern "C" {
    pub fn mod_timer_pending(timer: *mut timer_list, expires: c_ulong) -> c_int;
}
extern "C" {
    pub fn timer_reduce(timer: *mut timer_list, expires: c_ulong) -> c_int;
}
//
// The jiffies value which is added to now, when there is no timer
// in the timer wheel:
//

extern "C" {
    pub fn add_timer(timer: *mut timer_list);
}
extern "C" {
    pub fn add_timer_local(timer: *mut timer_list);
}
extern "C" {
    pub fn add_timer_global(timer: *mut timer_list);
}
extern "C" {
    pub fn timer_delete_sync_try(timer: *mut timer_list) -> c_int;
}
extern "C" {
    pub fn timer_delete_sync(timer: *mut timer_list) -> c_int;
}
extern "C" {
    pub fn timer_delete(timer: *mut timer_list) -> c_int;
}
extern "C" {
    pub fn timer_shutdown_sync(timer: *mut timer_list) -> c_int;
}
extern "C" {
    pub fn timer_shutdown(timer: *mut timer_list) -> c_int;
}
extern "C" {
    pub fn timers_init();
}
extern "C" {
    pub fn it_real_fn(: *mut hrtimer) -> hrtimer_restart;
}
extern "C" {
    pub fn __round_jiffies_relative(j: c_ulong, cpu: c_int) -> c_ulong;
}
extern "C" {
    pub fn round_jiffies(j: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn round_jiffies_relative(j: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn __round_jiffies_up_relative(j: c_ulong, cpu: c_int) -> c_ulong;
}
extern "C" {
    pub fn round_jiffies_up(j: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn round_jiffies_up_relative(j: c_ulong) -> c_ulong;
}

extern "C" {
    pub fn timers_prepare_cpu(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn timers_dead_cpu(cpu: c_uint) -> c_int;
}

extern "C" {
    pub fn tmigr_isolated_exclude_cpumask(exclude_cpumask: *mut cpumask) -> c_int;
}

