//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nf_flow_table_procfs.c
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

    static void *nf_flow_table_cpu_seq_start(struct seq_file *seq, loff_t *pos)
    {
    struct net *net = seq_file_net(seq);
    int cpu;
    if (*pos == 0)
    return SEQ_START_TOKEN;
    for (cpu = *pos - 1; cpu < nr_cpu_ids; ++cpu) {
    if (!cpu_possible(cpu))
    continue;
// pos = cpu + 1;
    return per_cpu_ptr(net.ft.stat, cpu);
    }
    return core::ptr::null_mut();
    }
    static void *nf_flow_table_cpu_seq_next(struct seq_file *seq, void *v, loff_t *pos)
    {
    struct net *net = seq_file_net(seq);
    int cpu;
    for (cpu = *pos; cpu < nr_cpu_ids; ++cpu) {
    if (!cpu_possible(cpu))
    continue;
// pos = cpu + 1;
    return per_cpu_ptr(net.ft.stat, cpu);
    }
    (*pos)++;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn nf_flow_table_cpu_seq_stop(seq: *mut seq_file, v: *mut c_void) {
    static void nf_flow_table_cpu_seq_stop(struct seq_file *seq, void *v)
    {
    }
#[no_mangle]
unsafe extern "C" fn nf_flow_table_cpu_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int nf_flow_table_cpu_seq_show(struct seq_file *seq, void *v)
    {
    const struct nf_flow_table_stat *st = v;
    if (v == SEQ_START_TOKEN) {
    seq_puts(seq, "wq_add   wq_del   wq_stats\n");
    return 0;
    }
    seq_printf(seq, "%8d %8d %8d\n",
    st.count_wq_add,
    st.count_wq_del,
    st.count_wq_stats
    );
    return 0;
    }
    static const struct seq_operations nf_flow_table_cpu_seq_ops = {
    .start	= nf_flow_table_cpu_seq_start,
    .next	= nf_flow_table_cpu_seq_next,
    .stop	= nf_flow_table_cpu_seq_stop,
    .show	= nf_flow_table_cpu_seq_show,
    };
#[no_mangle]
pub unsafe extern "C" fn nf_flow_table_init_proc(net: *mut net) -> c_int {
    int nf_flow_table_init_proc(struct net *net)
    {
    struct proc_dir_entry *pde;
    pde = proc_create_net("nf_flowtable", 0444, net.proc_net_stat,
    &nf_flow_table_cpu_seq_ops,
    sizeof(struct seq_net_private));
    return pde ? 0 : -ENOMEM;
    }
#[no_mangle]
pub unsafe extern "C" fn nf_flow_table_fini_proc(net: *mut net) {
    void nf_flow_table_fini_proc(struct net *net)
    {
    remove_proc_entry("nf_flowtable", net.proc_net_stat);
    }
