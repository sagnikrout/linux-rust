//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/posix-timers.h
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

// Macro flag: #define _linux_POSIX_TIMERS_H

extern "C" {
    pub fn make_process_cpuclock(_arg: tid, CPUCLOCK_PERTHREAD_MASK: clock |) -> return;
}
extern "C" {
    pub fn make_process_cpuclock(fd: (unsigned int), _arg: CLOCKFD) -> return;
}

//
// struct cpu_timer - Posix CPU timer representation for k_itimer
// @node:	timerqueue node to queue in the task/sig
// @head:	timerqueue head on which this timer is queued
// @pid:	Pointer to target task PID
// @elist:	List head for the expiry list
// @firing:	Timer is currently firing
// @nanosleep:	Timer is used for nanosleep and is not a regular posix-timer
// @handling:	Pointer to the task which handles expiry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_timer {
    pub node: timerqueue_node,
    pub head: *mut timerqueue_head,
    pub pid: *mut pid,
    pub elist: list_head,
    pub firing: bool,
    pub nanosleep: bool,
    pub handling: *mut task___rcu,
}

extern "C" {
    pub fn timerqueue_add(_arg: head, _arg: &ctmr->node) -> return;
}
extern "C" {
    pub fn posix_cputimers_group_init(pct: *mut posix_cputimers, cpu_limit: u64);
}
extern "C" {
    pub fn posixtimer_rearm_itimer(p: *mut task_struct);
}
extern "C" {
    pub fn posixtimer_init_sigqueue(q: *mut sigqueue) -> bool;
}
extern "C" {
    pub fn posixtimer_send_sigqueue(tmr: *mut k_itimer);
}
extern "C" {
    pub fn posixtimer_deliver_signal(info: *mut kernel_siginfo, timer_sigq: *mut sigqueue) -> bool;
}
extern "C" {
    pub fn posixtimer_free_timer(timer: *mut k_itimer);
}
extern "C" {
    pub fn posixtimer_create_prctl(ctrl: c_ulong) -> c_long;
}
// Init task static initializer

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_timer {
// Macro flag: #define INIT_CPU_TIMERS(s)
    pub }: *mut *mut sigqueue timer_sigq) { return false;,
    pub }: static inline long posixtimer_create_prctl(unsigned long ctrl) { return -EINVAL;,

    pub p): *mut void clear_posix_cputimers_work(struct task_struct,
    pub posix_cputimers_init_work(void): c_void,

//
// struct k_itimer - POSIX.1b interval timer structure.
// @list:		List node for binding the timer to tsk::signal::posix_timers
// @ignored_list:	List node for tracking ignored timers in tsk::signal::ignored_posix_timers
// @t_hash:		Entry in the posix timer hash table
// @it_lock:		Lock protecting the timer
// @kclock:		Pointer to the k_clock struct handling this timer
// @it_clock:		The posix timer clock id
// @it_id:		The posix timer id for identifying the timer
// @it_status:		The status of the timer
// @it_sig_periodic:	The periodic status at signal delivery
// @it_overrun:		The overrun counter for pending signals
// @it_overrun_last:	The overrun at the time of the last delivered signal
// @it_signal_seq:	Sequence count to control signal delivery
// @it_sigqueue_seq:	The sequence count at the point where the signal was queued
// @it_sigev_notify:	The notify word of sigevent struct for signal delivery
// @it_interval:	The interval for periodic timers
// @it_pid_type:	The type of the PID
// @it_signal:		Pointer to the creators signal struct
// @it_pid:		The pid of the process/task targeted by the signal
// @it_process:		The task to wakeup on clock_nanosleep (CPU timers)
// @rcuref:		Reference count for life time management
// @sigq:		Embedded sigqueue
// @it:			Union representing the various posix timer type
// internals.
// @rcu:		RCU head for freeing the timer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct k_itimer {
// 1st cacheline contains read-mostly fields
    pub t_hash: hlist_node,
    pub list: hlist_node,
    pub it_id: timer_t,
    pub it_clock: clockid_t,
    pub it_sigev_notify: c_int,
    pub it_pid_type: pid_type,
    pub it_signal: *mut signal_struct,
    pub kclock: *const k_clock,
// 2nd cacheline and above contain fields which are modified regularly
    pub it_lock: spinlock_t,
    pub it_status: c_int,
    pub it_sig_periodic: bool,
    pub it_overrun: i64,
    pub it_overrun_last: i64,
    pub it_signal_seq: c_uint,
    pub it_sigqueue_seq: c_uint,
    pub it_interval: ktime_t,
    pub ignored_list: hlist_node,
    pub it_pid: *mut pid,
    pub it_process: *mut task_struct,
}

extern "C" {
    pub fn run_posix_cpu_timers();
}
extern "C" {
    pub fn posix_cpu_timers_exit(task: *mut task_struct);
}
extern "C" {
    pub fn posix_cpu_timers_exit_group(task: *mut task_struct);
}
extern "C" {
    pub fn update_rlimit_cpu(task: *mut task_struct, rlim_new: c_ulong) -> c_int;
}

