//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/uapi/linux/taskstats.h
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
// taskstats.h - exporting per-task statistics
//
// Copyright (C) Shailabh Nagar, IBM Corp. 2006
// (C) Balbir Singh,   IBM Corp. 2006
// (C) Jay Lan,        SGI, 2006
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of version 2.1 of the GNU Lesser General Public License
// as published by the Free Software Foundation.
//
// This program is distributed in the hope that it would be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
//

// Format for per-task data returned to userland when
// - a task exits
// - listener requests stats for a task
//
// The struct is versioned. Newer versions should only add fields to
// the bottom of the struct to maintain backward compatibility.
//
// To add new fields
// a) bump up TASKSTATS_VERSION
// b) add comment indicating new version number at end of struct
// c) add new fields after version comment; maintain 64-bit alignment
//
pub const TASKSTATS_VERSION: c_int = 17;

// in linux/sched.h
#[repr(C)]
#[derive(Copy, Clone)]
pub struct taskstats {
// The version number of this struct. This field is always set to
// TAKSTATS_VERSION, which is defined in <linux/taskstats.h>.
// Each time the struct is changed, the value should be incremented.
//
    pub version: __u16,
    pub /: *mut *mut __u32 ac_exitcode; / Exit status,
// The accounting flags of a task as defined in <linux/acct.h>
// Defined values are AFORK, ASU, ACOMPAT, ACORE, AXSIG, and AGROUP.
// (AGROUP since version 12).
//
    pub /: *mut *mut __u8 ac_flag; / Record flags,
    pub /: *mut *mut __u8 ac_nice; / task_nice,
// Delay accounting fields start
//
// All values, until comment "Delay accounting fields end" are
// available only if delay accounting is enabled, even though the last
// few fields are not delays
//
// xxx_count is the number of delay values recorded
// xxx_delay_total is the corresponding cumulative delay in nanoseconds
//
// xxx_delay_total wraps around to zero on overflow
// xxx_count incremented regardless of overflow
//
// Delay waiting for cpu, while runnable
// count, delay_total NOT updated atomically
//
    pub __attribute__((aligned(8))): __u64 cpu_count,
    pub cpu_delay_total: __u64,
// Following four fields atomically updated using task->delays->lock
// Delay waiting for synchronous block I/O to complete
// does not account for delays in I/O submission
//
    pub blkio_count: __u64,
    pub blkio_delay_total: __u64,
// Delay waiting for page fault I/O (swap in only)
    pub swapin_count: __u64,
    pub swapin_delay_total: __u64,
// cpu "wall-clock" running time
// On some architectures, value will adjust for cpu time stolen
// from the kernel in involuntary waits due to virtualization.
// Value is cumulative, in nanoseconds, without a corresponding count
// and wraps around to zero silently on overflow
//
    pub cpu_run_real_total: __u64,
// cpu "virtual" running time
// Uses time intervals seen by the kernel i.e. no adjustment
// for kernel's involuntary waits due to virtualization.
// Value is cumulative, in nanoseconds, without a corresponding count
// and wraps around to zero silently on overflow
//
    pub cpu_run_virtual_total: __u64,
// Delay accounting fields end
// version 1 ends here
// Basic Accounting Fields start
    pub /: *mut *mut char ac_comm[TS_COMM_LEN]; / Command name,
    pub __attribute__((aligned(8))): __u8 ac_sched,
// Scheduling discipline
    pub ac_pad: [__u8; 3],
    pub __attribute__((aligned(8))): __u32 ac_uid,
// User ID
    pub /: *mut *mut __u32 ac_gid; / Group ID,
    pub /: *mut *mut __u32 ac_pid; / Process ID,
    pub /: *mut *mut __u32 ac_ppid; / Parent process ID,
// __u32 range means times from 1970 to 2106
    pub /: *mut *mut __u32 ac_btime; / Begin time [sec since 1970],
    pub __attribute__((aligned(8))): __u64 ac_etime,
// Elapsed time [usec]
    pub /: *mut *mut __u64 ac_utime; / User CPU time [usec],
    pub /: *mut *mut __u64 ac_stime; / SYstem CPU time [usec],
    pub /: *mut *mut __u64 ac_minflt; / Minor Page Fault Count,
    pub /: *mut *mut __u64 ac_majflt; / Major Page Fault Count,
// Basic Accounting Fields end
// Extended accounting fields start
// Accumulated RSS usage in duration of a task, in MBytes-usecs.
// The current rss usage is added to this counter every time
// a tick is charged to a task's system time. So, at the end we
// will have memory usage multiplied by system time. Thus an
// average usage per system time unit can be calculated.
//
    pub /: *mut *mut __u64 coremem; / accumulated RSS usage in MB-usec,
// Accumulated virtual memory usage in duration of a task.
// Same as acct_rss_mem1 above except that we keep track of VM usage.
//
    pub /: *mut *mut __u64 virtmem; / accumulated VM usage in MB-usec,
// High watermark of RSS and virtual memory usage in duration of
// a task, in KBytes.
//
    pub /: *mut *mut __u64 hiwater_rss; / High-watermark of RSS usage, in KB,
    pub /: *mut *mut __u64 hiwater_vm; / High-water VM usage, in KB,
// The following four fields are I/O statistics of a task.
    pub /: *mut *mut __u64 read_char; / bytes read,
    pub /: *mut *mut __u64 write_char; / bytes written,
    pub /: *mut *mut __u64 read_syscalls; / read syscalls,
    pub /: *mut *mut __u64 write_syscalls; / write syscalls,
// Extended accounting fields end
// Macro flag: #define TASKSTATS_HAS_IO_ACCOUNTING
// Per-task storage I/O accounting starts
    pub /: *mut *mut __u64 read_bytes; / bytes of read I/O,
    pub /: *mut *mut __u64 write_bytes; / bytes of write I/O,
    pub /: *mut *mut __u64 cancelled_write_bytes; / bytes of cancelled write I/O,
    pub /: *mut *mut __u64 nvcsw; / voluntary_ctxt_switches,
    pub /: *mut *mut __u64 nivcsw; / nonvoluntary_ctxt_switches,
// time accounting for SMT machines
    pub /: *mut *mut __u64 ac_utimescaled; / utime scaled on frequency etc,
    pub /: *mut *mut __u64 ac_stimescaled; / stime scaled on frequency etc,
    pub /: *mut *mut __u64 cpu_scaled_run_real_total; / scaled cpu_run_real_total,
// Delay waiting for memory reclaim
    pub freepages_count: __u64,
    pub freepages_delay_total: __u64,
// Delay waiting for thrashing page
    pub thrashing_count: __u64,
    pub thrashing_delay_total: __u64,
// v10: 64-bit btime to avoid overflow
    pub /: *mut *mut __u64 ac_btime64; / 64-bit begin time,
// v11: Delay waiting for memory compact
    pub compact_count: __u64,
    pub compact_delay_total: __u64,
// v12 begin
    pub /: *mut *mut __u32 ac_tgid; / thread group ID,
// Thread group walltime up to now. This is total process walltime if
// AGROUP flag is set.
//
    pub __attribute__((aligned(8))): __u64 ac_tgetime,
// Lightweight information to identify process binary files.
// This leaves userspace to match this to a file system path, using
// MAJOR() and MINOR() macros to identify a device and mount point,
// the inode to identify the executable file. This is /proc/self/exe
// at the end, so matching the most recent exec(). Values are zero
// for kernel threads.
//
    pub /: *mut *mut __u64 ac_exe_dev; / program binary device ID,
    pub /: *mut *mut __u64 ac_exe_inode; / program binary inode number,
// v12 end
// v13: Delay waiting for write-protect copy
    pub wpcopy_count: __u64,
    pub wpcopy_delay_total: __u64,
// v14: Delay waiting for IRQ/SOFTIRQ
    pub irq_count: __u64,
    pub irq_delay_total: __u64,
// v15: add Delay max and Delay min
// v16: move Delay max and Delay min to the end of taskstat
    pub cpu_delay_max: __u64,
    pub cpu_delay_min: __u64,
    pub blkio_delay_max: __u64,
    pub blkio_delay_min: __u64,
    pub swapin_delay_max: __u64,
    pub swapin_delay_min: __u64,
    pub freepages_delay_max: __u64,
    pub freepages_delay_min: __u64,
    pub thrashing_delay_max: __u64,
    pub thrashing_delay_min: __u64,
    pub compact_delay_max: __u64,
    pub compact_delay_min: __u64,
    pub wpcopy_delay_max: __u64,
    pub wpcopy_delay_min: __u64,
    pub irq_delay_max: __u64,
    pub irq_delay_min: __u64,
// v17: delay max timestamp record
    pub cpu_delay_max_ts: __kernel_timespec,
    pub blkio_delay_max_ts: __kernel_timespec,
    pub swapin_delay_max_ts: __kernel_timespec,
    pub freepages_delay_max_ts: __kernel_timespec,
    pub thrashing_delay_max_ts: __kernel_timespec,
    pub compact_delay_max_ts: __kernel_timespec,
    pub wpcopy_delay_max_ts: __kernel_timespec,
    pub irq_delay_max_ts: __kernel_timespec,
}

//
// Commands sent from userspace
// Not versioned. New commands should only be inserted at the enum's end
// prior to __TASKSTATS_CMD_MAX
//

// NETLINK_GENERIC related info

pub const TASKSTATS_GENL_VERSION: c_uint = 0x1;
