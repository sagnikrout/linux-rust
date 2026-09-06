//! Automatically rewritten from C to Rust
//! Source: lib/sys_info.c
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

    static const char * const si_names[] = {
    [ilog2(SYS_INFO_TASKS)]			= "tasks",
    [ilog2(SYS_INFO_MEM)]			= "mem",
    [ilog2(SYS_INFO_TIMERS)]		= "timers",
    [ilog2(SYS_INFO_LOCKS)]			= "locks",
    [ilog2(SYS_INFO_FTRACE)]		= "ftrace",
    [ilog2(SYS_INFO_PANIC_CONSOLE_REPLAY)]	= "",
    [ilog2(SYS_INFO_ALL_BT)]		= "all_bt",
    [ilog2(SYS_INFO_BLOCKED_TASKS)]		= "blocked_tasks",
    };
//
// Default kernel sys_info mask.
// If a kernel module calls sys_info() with "parameter == 0", then
// this mask will be used.
//
    static unsigned long kernel_si_mask;
// Expecting string like "xxx_sys_info=tasks,mem,timers,locks,ftrace,..."
#[no_mangle]
pub unsafe extern "C" fn sys_info_parse_param(str: *mut c_char) -> c_ulong {
    unsigned long sys_info_parse_param(char *str)
    {
    let mut si_bits: c_ulong = 0;
    char *s, *name;
    int i;
    s = str;
    while ((name = strsep(&s, ",")) && *name) {
    i = match_string(si_names, ARRAY_SIZE(si_names), name);
    if (i >= 0)
    __set_bit(i, &si_bits);
    }
    return si_bits;
    }

    static int sys_info_write_handler(const struct ctl_table *table,
    void *buffer, size_t *lenp, loff_t *ppos,
    unsigned long *si_bits_global)
    {
    unsigned long si_bits;
    int ret;
    ret = proc_dostring(table, 1, buffer, lenp, ppos);
    if (ret)
    return ret;
    si_bits = sys_info_parse_param(table.data);
// The access to the global value is not synchronized.
    WRITE_ONCE(*si_bits_global, si_bits);
    return 0;
    }
    static int sys_info_read_handler(const struct ctl_table *table,
    void *buffer, size_t *lenp, loff_t *ppos,
    unsigned long *si_bits_global)
    {
    unsigned long si_bits;
    let mut len: c_uint = 0;
    char *delim = "";
    unsigned int i;
// The access to the global value is not synchronized.
    si_bits = READ_ONCE(*si_bits_global);
    for_each_set_bit(i, &si_bits, ARRAY_SIZE(si_names)) {
    if (*si_names[i]) {
    len += scnprintf(table.data + len, table.maxlen - len,
    "%s%s", delim, si_names[i]);
    delim = ",";
    }
    }
    return proc_dostring(table, 0, buffer, lenp, ppos);
    }
    int sysctl_sys_info_handler(const struct ctl_table *ro_table, int write,
    void *buffer, size_t *lenp,
    loff_t *ppos)
    {
    struct ctl_table table;
    unsigned int i;
    size_t maxlen;
    maxlen = 0;
    for (i = 0; i < ARRAY_SIZE(si_names); i++)
    maxlen += strlen(si_names[i]) + 1;
    char *names __free(kfree) = kzalloc(maxlen, GFP_KERNEL);
    if (!names)
    return -ENOMEM;
    table = *ro_table;
    table.data = names;
    table.maxlen = maxlen;
    if (write)
    return sys_info_write_handler(&table, buffer, lenp, ppos, ro_table.data);
    else
    return sys_info_read_handler(&table, buffer, lenp, ppos, ro_table.data);
    }
    static const struct ctl_table sys_info_sysctls[] = {
    {
    .procname	= "kernel_sys_info",
    .data		= &kernel_si_mask,
    .maxlen         = sizeof(kernel_si_mask),
    .mode		= 0644,
    .proc_handler	= sysctl_sys_info_handler,
    },
    };
#[no_mangle]
unsafe extern "C" fn sys_info_sysctl_init() -> int __init {
    static int __init sys_info_sysctl_init(void)
    {
    register_sysctl_init("kernel", sys_info_sysctls);
    return 0;
    }
    subsys_initcall(sys_info_sysctl_init);

#[no_mangle]
unsafe extern "C" fn __sys_info(si_mask: c_ulong) {
    static void __sys_info(unsigned long si_mask)
    {
    if (si_mask & SYS_INFO_TASKS)
    show_state();
    if (si_mask & SYS_INFO_MEM)
    show_mem();
    if (si_mask & SYS_INFO_TIMERS)
    sysrq_timer_list_show();
    if (si_mask & SYS_INFO_LOCKS)
    debug_show_all_locks();
    if (si_mask & SYS_INFO_FTRACE)
    ftrace_dump(DUMP_ALL);
    if (si_mask & SYS_INFO_ALL_BT)
    trigger_all_cpu_backtrace();
    if (si_mask & SYS_INFO_BLOCKED_TASKS)
    show_state_filter(TASK_UNINTERRUPTIBLE);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_info(si_mask: c_ulong) {
    void sys_info(unsigned long si_mask)
    {
    __sys_info(si_mask ? : kernel_si_mask);
    }
