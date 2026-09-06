//! Automatically rewritten from C to Rust
//! Source: arch/s390/appldata/appldata_os.c
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
// Data gathering module for Linux-VM Monitor Stream, Stage 1.
// Collects misc. OS related data (CPU utilization, running processes).
//
// Copyright IBM Corp. 2003, 2006
//
// Author: Gerald Schaefer <gerald.schaefer@de.ibm.com>
//

//
// OS data
//
// This is accessed as binary data by z/VM. If changes to it can't be avoided,
// the structure version (product ID, see appldata_base.c) needs to be changed
// as well and all documentation and z/VM applications using it must be
// updated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct appldata_os_per_cpu {
    pub /: *mut *mut u32 per_cpu_user; / timer ticks spent in user mode,
    pub /: *mut *mut u32 per_cpu_nice; / ... spent with modified priority,
    pub /: *mut *mut u32 per_cpu_system; / ... spent in kernel mode,
    pub /: *mut *mut u32 per_cpu_idle; / ... spent in idle mode,
// New in 2.6
    pub /: *mut *mut u32 per_cpu_irq; / ... spent in interrupts,
    pub /: *mut *mut u32 per_cpu_softirq; / ... spent in softirqs,
    pub /: *mut *mut u32 per_cpu_iowait; / ... spent while waiting for I/O,
// New in modification level 01
    pub /: *mut *mut u32 per_cpu_steal; / ... stolen by hypervisor,
    pub /: *mut *mut u32 cpu_id; / number of this CPU,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct appldata_os_data {
    pub timestamp: u64,
    pub /: *mut *mut u32 sync_count_1; / after VM collected the record data,,
    pub the: *mut *mut u32 sync_count_2; / sync_count_1 and sync_count_2 should be,
    same. If not, the record has been updated on
    the Linux side while VM was collecting the
    (possibly corrupt) data */
    pub /: *mut *mut u32 nr_cpus; / number of (virtual) CPUs,
    pub /: *mut *mut u32 per_cpu_size; / size of the per-cpu data struct,
    pub /: *mut *mut u32 cpu_offset; / offset of the first per-cpu data struct,
    pub /: *mut *mut u32 nr_running; / number of runnable threads,
    pub /: *mut *mut u32 nr_threads; / number of threads,
    pub /: *mut *mut u32 avenrun[3]; / average nr. of running processes during,
// the last 1, 5 and 15 minutes
// New in 2.6
    pub threads: *mut *mut u32 nr_iowait; / number of blocked,
    (waiting for I/O)               */
// per cpu data
    pub os_cpu: [appldata_os_per_cpu; ],
    pub __attribute__((packed)): },
    pub appldata_os_data: *mut static struct appldata_os_data,
    static struct appldata_ops ops = {
    .name	   = "os",
    .record_nr = APPLDATA_RECORD_OS_ID,
    .owner	   = THIS_MODULE,
    .mod_lvl   = {0xF0, 0xF1},		/* EBCDIC "01" */
}

//
// appldata_get_os_data()
//
// gather OS data
//
#[no_mangle]
unsafe extern "C" fn appldata_get_os_data(data: *mut c_void) {
    static void appldata_get_os_data(void *data)
    {
    int i, j, rc;
    struct appldata_os_data *os_data;
    unsigned int new_size;
    os_data = data;
    os_data.sync_count_1++;
    os_data.nr_threads = nr_threads;
    os_data.nr_running = nr_running();
    os_data.nr_iowait  = nr_iowait();
    os_data.avenrun[0] = avenrun[0] + (FIXED_1/200);
    os_data.avenrun[1] = avenrun[1] + (FIXED_1/200);
    os_data.avenrun[2] = avenrun[2] + (FIXED_1/200);
    j = 0;
    for_each_online_cpu(i) {
    os_data.os_cpu[j].per_cpu_user =
    nsecs_to_jiffies(kcpustat_cpu(i).cpustat[CPUTIME_USER]);
    os_data.os_cpu[j].per_cpu_nice =
    nsecs_to_jiffies(kcpustat_cpu(i).cpustat[CPUTIME_NICE]);
    os_data.os_cpu[j].per_cpu_system =
    nsecs_to_jiffies(kcpustat_cpu(i).cpustat[CPUTIME_SYSTEM]);
    os_data.os_cpu[j].per_cpu_idle =
    nsecs_to_jiffies(kcpustat_cpu(i).cpustat[CPUTIME_IDLE]);
    os_data.os_cpu[j].per_cpu_irq =
    nsecs_to_jiffies(kcpustat_cpu(i).cpustat[CPUTIME_IRQ]);
    os_data.os_cpu[j].per_cpu_softirq =
    nsecs_to_jiffies(kcpustat_cpu(i).cpustat[CPUTIME_SOFTIRQ]);
    os_data.os_cpu[j].per_cpu_iowait =
    nsecs_to_jiffies(kcpustat_cpu(i).cpustat[CPUTIME_IOWAIT]);
    os_data.os_cpu[j].per_cpu_steal =
    nsecs_to_jiffies(kcpustat_cpu(i).cpustat[CPUTIME_STEAL]);
    os_data.os_cpu[j].cpu_id = i;
    j++;
    }
    os_data.nr_cpus = j;
    new_size = struct_size(os_data, os_cpu, os_data.nr_cpus);
    if (ops.size != new_size) {
    if (ops.active) {
    rc = appldata_diag(APPLDATA_RECORD_OS_ID,
    APPLDATA_START_INTERVAL_REC,
    (unsigned long) ops.data, new_size,
    ops.mod_lvl);
    if (rc != 0)
    pr_err("Starting a new OS data collection "
    "failed with rc=%d\n", rc);
    rc = appldata_diag(APPLDATA_RECORD_OS_ID,
    APPLDATA_STOP_REC,
    (unsigned long) ops.data, ops.size,
    ops.mod_lvl);
    if (rc != 0)
    pr_err("Stopping a faulty OS data "
    "collection failed with rc=%d\n", rc);
    }
    ops.size = new_size;
    }
    os_data.timestamp = get_tod_clock();
    os_data.sync_count_2++;
    }
//
// appldata_os_init()
//
// init data, register ops
//
#[no_mangle]
unsafe extern "C" fn appldata_os_init() -> int __init {
    static int __init appldata_os_init(void)
    {
    int rc, max_size;
    max_size = struct_size(appldata_os_data, os_cpu, num_possible_cpus());
    if (max_size > APPLDATA_MAX_REC_SIZE) {
    pr_err("Maximum OS record size %i exceeds the maximum "
    "record size %i\n", max_size, APPLDATA_MAX_REC_SIZE);
    rc = -ENOMEM;
    goto out;
    }
    appldata_os_data = kzalloc(max_size, GFP_KERNEL | GFP_DMA);
    if (appldata_os_data == core::ptr::null_mut()) {
    rc = -ENOMEM;
    goto out;
    }
    appldata_os_data.per_cpu_size = sizeof(struct appldata_os_per_cpu);
    appldata_os_data.cpu_offset   = offsetof(struct appldata_os_data,
    os_cpu);
    ops.data = appldata_os_data;
    ops.callback  = &appldata_get_os_data;
    rc = appldata_register_ops(&ops);
    if (rc != 0)
    kfree(appldata_os_data);
    out:
    return rc;
    }
//
// appldata_os_exit()
//
// unregister ops
//
#[no_mangle]
unsafe extern "C" fn appldata_os_exit() -> void __exit {
    static void __exit appldata_os_exit(void)
    {
    appldata_unregister_ops(&ops);
    kfree(appldata_os_data);
    }
    module_init(appldata_os_init);
    module_exit(appldata_os_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Gerald Schaefer");
    MODULE_DESCRIPTION("Linux-VM Monitor Stream, OS statistics");
