//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/pseries/hvCall_inst.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2006 Mike Kravetz IBM Corporation
//
// Hypervisor Call Instrumentation
//

// For hcall instrumentation. One structure per-hcall, per-CPU
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hcall_stats {
    pub /: *mut *mut unsigned long num_calls; / number of calls (on this CPU),
    pub /: *mut *mut unsigned long tb_total; / total wall time (mftb) of calls.,
    pub /: *mut *mut unsigned long purr_total; / total cpu time (PURR) of calls.,
    pub tb_start: c_ulong,
    pub purr_start: c_ulong,
}

    static DEFINE_PER_CPU(struct hcall_stats[HCALL_STAT_ARRAY_SIZE], hcall_stats);
//
// Routines for displaying the statistics in debugfs
//
    static void *hc_start(struct seq_file *m, loff_t *pos)
    {
    if ((int)*pos < (HCALL_STAT_ARRAY_SIZE-1))
    return (void *)(unsigned long)(*pos + 1);
    return core::ptr::null_mut();
    }
    static void *hc_next(struct seq_file *m, void *p, loff_t * pos)
    {
    ++*pos;
    return hc_start(m, pos);
    }
#[no_mangle]
unsafe extern "C" fn hc_stop(m: *mut seq_file, p: *mut c_void) {
    static void hc_stop(struct seq_file *m, void *p)
    {
    }
#[no_mangle]
unsafe extern "C" fn hc_show(m: *mut seq_file, p: *mut c_void) -> c_int {
    static int hc_show(struct seq_file *m, void *p)
    {
    let mut h_num: c_ulong = (unsigned long)p;
    struct hcall_stats *hs = m.private;
    if (hs[h_num].num_calls) {
    if (cpu_has_feature(CPU_FTR_PURR))
    seq_printf(m, "%lu %lu %lu %lu\n", h_num<<2,
    hs[h_num].num_calls,
    hs[h_num].tb_total,
    hs[h_num].purr_total);
    else
    seq_printf(m, "%lu %lu %lu\n", h_num<<2,
    hs[h_num].num_calls,
    hs[h_num].tb_total);
    }
    return 0;
    }
    static const struct seq_operations hcall_inst_sops = {
    .start = hc_start,
    .next  = hc_next,
    .stop  = hc_stop,
    .show  = hc_show
    };
    DEFINE_SEQ_ATTRIBUTE(hcall_inst);

pub const CPU_NAME_BUF_SIZE: c_int = 32;
#[no_mangle]
unsafe extern "C" fn probe_hcall_entry(ignored: *mut c_void, opcode: c_ulong, args: *mut c_ulong) {
    static void probe_hcall_entry(void *ignored, unsigned long opcode, unsigned long *args)
    {
    struct hcall_stats *h;
    if (opcode > MAX_HCALL_OPCODE)
    return;
    h = this_cpu_ptr(&hcall_stats[opcode / 4]);
    h.tb_start = mftb();
    h.purr_start = mfspr(SPRN_PURR);
    }
    static void probe_hcall_exit(void *ignored, unsigned long opcode, long retval,
    unsigned long *retbuf)
    {
    struct hcall_stats *h;
    if (opcode > MAX_HCALL_OPCODE)
    return;
    h = this_cpu_ptr(&hcall_stats[opcode / 4]);
    h.num_calls++;
    h.tb_total += mftb() - h.tb_start;
    h.purr_total += mfspr(SPRN_PURR) - h.purr_start;
    }
#[no_mangle]
unsafe extern "C" fn hcall_inst_init() -> int __init {
    static int __init hcall_inst_init(void)
    {
    struct dentry *hcall_root;
    char cpu_name_buf[CPU_NAME_BUF_SIZE];
    int cpu;
    if (!firmware_has_feature(FW_FEATURE_LPAR))
    return 0;
    if (register_trace_hcall_entry(probe_hcall_entry, core::ptr::null_mut()))
    return -EINVAL;
    if (register_trace_hcall_exit(probe_hcall_exit, core::ptr::null_mut())) {
    unregister_trace_hcall_entry(probe_hcall_entry, core::ptr::null_mut());
    return -EINVAL;
    }
    hcall_root = debugfs_create_dir(HCALL_ROOT_DIR, core::ptr::null_mut());
    for_each_possible_cpu(cpu) {
    snprintf(cpu_name_buf, CPU_NAME_BUF_SIZE, "cpu%d", cpu);
    debugfs_create_file(cpu_name_buf, 0444, hcall_root,
    per_cpu(hcall_stats, cpu),
    &hcall_inst_fops);
    }
    return 0;
    }
    machine_device_initcall(pseries, hcall_inst_init);
