//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_kdb.c
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
// kdb helper for dumping the ftrace buffer
//
// Copyright (C) 2010 Jason Wessel <jason.wessel@windriver.com>
//
// ftrace_dump_buf based on ftrace_dump:
// Copyright (C) 2007-2008 Steven Rostedt <srostedt@redhat.com>
// Copyright (C) 2008 Ingo Molnar <mingo@redhat.com>
//

    static struct trace_iterator iter;
    static struct ring_buffer_iter *buffer_iter[CONFIG_NR_CPUS];
#[no_mangle]
unsafe extern "C" fn ftrace_dump_buf(skip_entries: c_int, cpu_file: c_long) {
    static void ftrace_dump_buf(int skip_entries, long cpu_file)
    {
    struct trace_array *tr;
    unsigned int old_userobj;
    let mut cnt: c_int = 0, cpu;
    tr = iter.tr;
    old_userobj = tr.trace_flags;
// don't look at user memory in panic mode
    tr.trace_flags &= ~TRACE_ITER(SYM_USEROBJ);
    kdb_printf("Dumping ftrace buffer:\n");
    if (skip_entries)
    kdb_printf("(skipping %d entries)\n", skip_entries);
    trace_iterator_reset(&iter);
    iter.iter_flags |= TRACE_FILE_LAT_FMT;
    if (cpu_file == RING_BUFFER_ALL_CPUS) {
    for_each_tracing_cpu(cpu) {
    iter.buffer_iter[cpu] =
    ring_buffer_read_start(iter.array_buffer.buffer,
    cpu, GFP_ATOMIC);
    tracing_iter_reset(&iter, cpu);
    }
    } else {
    iter.cpu_file = cpu_file;
    iter.buffer_iter[cpu_file] =
    ring_buffer_read_start(iter.array_buffer.buffer,
    cpu_file, GFP_ATOMIC);
    tracing_iter_reset(&iter, cpu_file);
    }
    while (trace_find_next_entry_inc(&iter)) {
    if (!cnt)
    kdb_printf("---------------------------------\n");
    cnt++;
    if (!skip_entries) {
    print_trace_line(&iter);
    trace_printk_seq(&iter.seq);
    } else {
    skip_entries--;
    }
    if (KDB_FLAG(CMD_INTERRUPT))
    goto out;
    }
    if (!cnt)
    kdb_printf("   (ftrace buffer empty)\n");
    else
    kdb_printf("---------------------------------\n");
    out:
    tr.trace_flags = old_userobj;
    for_each_tracing_cpu(cpu) {
    if (iter.buffer_iter[cpu]) {
    ring_buffer_read_finish(iter.buffer_iter[cpu]);
    iter.buffer_iter[cpu] = core::ptr::null_mut();
    }
    }
    }
//
// kdb_ftdump - Dump the ftrace log buffer
//
#[no_mangle]
unsafe extern "C" fn kdb_ftdump(argc: c_int, argv: *const c_char) -> c_int {
    static int kdb_ftdump(int argc, const char **argv)
    {
    let mut skip_entries: c_int = 0;
    long cpu_file;
    int err;
    int cnt;
    if (argc > 2)
    return KDB_ARGCOUNT;
    if (argc && kstrtoint(argv[1], 0, &skip_entries))
    return KDB_BADINT;
    if (argc == 2) {
    err = kstrtol(argv[2], 0, &cpu_file);
    if (err || cpu_file >= NR_CPUS || cpu_file < 0 ||
    !cpu_online(cpu_file))
    return KDB_BADINT;
    } else {
    cpu_file = RING_BUFFER_ALL_CPUS;
    }
    kdb_trap_printk++;
    trace_init_global_iter(&iter);
    iter.buffer_iter = buffer_iter;
    tracer_tracing_disable(iter.tr);
// A negative skip_entries means skip all but the last entries
    if (skip_entries < 0) {
    if (cpu_file == RING_BUFFER_ALL_CPUS)
    cnt = trace_total_entries(core::ptr::null_mut());
    else
    cnt = trace_total_entries_cpu(core::ptr::null_mut(), cpu_file);
    skip_entries = max(cnt + skip_entries, 0);
    }
    ftrace_dump_buf(skip_entries, cpu_file);
    tracer_tracing_enable(iter.tr);
    kdb_trap_printk--;
    return 0;
    }
    static kdbtab_t ftdump_cmd = {
    .name = "ftdump",
    .func = kdb_ftdump,
    .usage = "[skip_#entries] [cpu]",
    .help = "Dump ftrace log; -skip dumps last #entries",
    .flags = KDB_ENABLE_ALWAYS_SAFE,
    };
#[no_mangle]
unsafe extern "C" fn kdb_ftrace_register() -> __init int {
    static __init int kdb_ftrace_register(void)
    {
    kdb_register(&ftdump_cmd);
    return 0;
    }
    late_initcall(kdb_ftrace_register);
