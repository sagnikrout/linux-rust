//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/psi_types.h
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

// Tracked task states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psi_task_count {
    NR_IOWAIT,
    NR_MEMSTALL,
    NR_RUNNING,
//
// For IO and CPU stalls the presence of running/oncpu tasks
// in the domain means a partial rather than a full stall.
// For memory it's not so simple because of page reclaimers:
// they are running/oncpu while representing a stall. To tell
// whether a domain has productivity left or not, we need to
// distinguish between regular running (i.e. productive)
// threads and memstall ones.
//
    NR_MEMSTALL_RUNNING,
    NR_PSI_TASK_COUNTS = 4,
}

// Task state bitmasks

// Only one task can be scheduled, no corresponding task count

// Resources that workloads could be stalled on
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psi_res {
    PSI_IO,
    PSI_MEM,
    PSI_CPU,

    PSI_IRQ,

    NR_PSI_RESOURCES,
}

//
// Pressure states for each resource:
//
// SOME: Stalled tasks & working tasks
// FULL: Stalled tasks & no working tasks
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psi_states {
    PSI_IO_SOME,
    PSI_IO_FULL,
    PSI_MEM_SOME,
    PSI_MEM_FULL,
    PSI_CPU_SOME,
    PSI_CPU_FULL,

    PSI_IRQ_FULL,

// Only per-CPU, to weigh the CPU in the global average:
    PSI_NONIDLE,
    NR_PSI_STATES,
}

// Use one bit in the state mask to track TSK_ONCPU

// Flag whether to re-arm avgs_work, see details in get_recent_times()

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psi_aggregators {
    PSI_AVGS = 0,
    PSI_POLL,
    NR_PSI_AGGREGATORS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psi_group_cpu {
// 1st cacheline updated by the scheduler
// States of the tasks belonging to this group
// Aggregate pressure state derived from the tasks
    pub state_mask: u32,
// Period time sampling buckets for each state of interest (ns)
    pub times: [u32; NR_PSI_STATES],
// Time of last task change in this group (rq_clock)
    pub state_start: u64,
// 2nd cacheline updated by the aggregator
// Delta detection against the sampling buckets
}

// PSI growth tracking window
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psi_window {
// Window size in ns
    pub size: u64,
// Start time of the current window in ns
    pub start_time: u64,
// Value at the start of the window
    pub start_value: u64,
// Value growth in the previous window
    pub prev_growth: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psi_trigger {
// PSI state being monitored by the trigger
    pub state: psi_states,
// User-spacified threshold in ns
    pub threshold: u64,
// List node inside triggers list
    pub node: list_head,
// Backpointer needed during trigger destruction
    pub group: *mut psi_group,
// Wait queue for polling
    pub event_wait: wait_queue_head_t,
// Kernfs file for cgroup triggers
    pub of: *mut kernfs_open_file,
// Pending event flag
    pub event: c_int,
// Tracking window
    pub win: psi_window,
//
// Time last event was generated. Used for rate-limiting
// events to one per window
//
    pub last_event_time: u64,
// Deferred event(s) from previous ratelimit window
    pub pending_event: bool,
// Trigger type - PSI_AVGS for unprivileged, PSI_POLL for RT
    pub aggregator: psi_aggregators,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psi_group {
    pub parent: *mut psi_group,
    pub enabled: bool,
// Protects data used by the aggregator
    pub avgs_lock: mutex,
// Per-cpu task state & time tracking
    pub pcpu: *mut psi_group_cpu __percpu,
// Running pressure averages
    pub 1]: u64 avg_total[NR_PSI_STATES -,
    pub avg_last_update: u64,
    pub avg_next_update: u64,
// Aggregator work control
    pub avgs_work: delayed_work,
// Unprivileged triggers against N*PSI_FREQ windows
    pub avg_triggers: list_head,
    pub 1]: u32 avg_nr_triggers[NR_PSI_STATES -,
// Total stall times and sampled pressure averages
    pub 1]: u64 total[NR_PSI_AGGREGATORS][NR_PSI_STATES -,
    pub 1][3]: unsigned long avg[NR_PSI_STATES -,
// Monitor RT polling work control
    pub rtpoll_task: *mut task___rcu,
    pub rtpoll_timer: timer_list,
    pub rtpoll_wait: wait_queue_head_t,
    pub rtpoll_wakeup: core::sync::atomic::AtomicI32,
    pub rtpoll_scheduled: core::sync::atomic::AtomicI32,
// Protects data used by the monitor
    pub rtpoll_trigger_lock: mutex,
// Configured RT polling triggers
    pub rtpoll_triggers: list_head,
    pub 1]: u32 rtpoll_nr_triggers[NR_PSI_STATES -,
    pub rtpoll_states: u32,
    pub rtpoll_min_period: u64,
// Total stall times at the start of RT polling monitor activation
    pub 1]: u64 rtpoll_total[NR_PSI_STATES -,
    pub rtpoll_next_update: u64,
    pub rtpoll_until: u64,
}

pub const NR_PSI_RESOURCES: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psi_group {

