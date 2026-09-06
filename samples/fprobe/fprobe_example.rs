//! Automatically rewritten from C to Rust
//! Source: samples/fprobe/fprobe_example.c
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
//
// Here's a sample kernel module showing the use of fprobe to dump a
// stack trace and selected registers when kernel_clone() is called.
//
// For more information on theory of operation of kprobes, see
// Documentation/trace/kprobes.rst
//
// You will see the trace data in /var/log/messages and on the console
// whenever kernel_clone() is invoked to create a new process.
//

pub const BACKTRACE_DEPTH: c_int = 16;
pub const MAX_SYMBOL_LEN: c_int = 4096;
    static struct fprobe sample_probe;
    static unsigned long nhit;
    static char symbol[MAX_SYMBOL_LEN] = "kernel_clone";
    module_param_string(symbol, symbol, sizeof(symbol), 0644);
    MODULE_PARM_DESC(symbol, "Probed symbol(s), given by comma separated symbols or a wildcard pattern.");
    static char nosymbol[MAX_SYMBOL_LEN] = "";
    module_param_string(nosymbol, nosymbol, sizeof(nosymbol), 0644);
    MODULE_PARM_DESC(nosymbol, "Not-probed symbols, given by a wildcard pattern.");
    let mut stackdump: static bool = true;
    module_param(stackdump, bool, 0644);
    MODULE_PARM_DESC(stackdump, "Enable stackdump.");
    let mut use_trace: static bool = false;
    module_param(use_trace, bool, 0644);
    MODULE_PARM_DESC(use_trace, "Use trace_printk instead of printk. This is only for debugging.");
#[no_mangle]
unsafe extern "C" fn show_backtrace() {
    static void show_backtrace(void)
    {
    unsigned long stacks[BACKTRACE_DEPTH];
    unsigned int len;
    len = stack_trace_save(stacks, BACKTRACE_DEPTH, 2);
    stack_trace_print(stacks, len, 24);
    }
    static int sample_entry_handler(struct fprobe *fp, unsigned long ip,
    unsigned long ret_ip,
    struct ftrace_regs *fregs, void *data)
    {
    if (use_trace)
//
// This is just an example, no kernel code should call
// trace_printk() except when actively debugging.
//
    trace_printk("Enter <%pS> ip = 0x%p\n", (void *)ip, (void *)ip);
    else
    pr_info("Enter <%pS> ip = 0x%p\n", (void *)ip, (void *)ip);
    nhit++;
    if (stackdump)
    show_backtrace();
    return 0;
    }
    static void sample_exit_handler(struct fprobe *fp, unsigned long ip,
    unsigned long ret_ip, struct ftrace_regs *regs,
    void *data)
    {
    let mut rip: c_ulong = ret_ip;
    if (use_trace)
//
// This is just an example, no kernel code should call
// trace_printk() except when actively debugging.
//
    trace_printk("Return from <%pS> ip = 0x%p to rip = 0x%p (%pS)\n",
    (void *)ip, (void *)ip, (void *)rip, (void *)rip);
    else
    pr_info("Return from <%pS> ip = 0x%p to rip = 0x%p (%pS)\n",
    (void *)ip, (void *)ip, (void *)rip, (void *)rip);
    nhit++;
    if (stackdump)
    show_backtrace();
    }
#[no_mangle]
unsafe extern "C" fn fprobe_init() -> int __init {
    static int __init fprobe_init(void)
    {
    char *p, *symbuf = core::ptr::null_mut();
    const char **syms;
    int ret, count, i;
    sample_probe.entry_handler = sample_entry_handler;
    sample_probe.exit_handler = sample_exit_handler;
    if (strchr(symbol, '*')) {
// filter based fprobe
    ret = register_fprobe(&sample_probe, symbol,
    nosymbol[0] == '\0' ? core::ptr::null_mut() : nosymbol);
    goto out;
    } else if (!strchr(symbol, ',')) {
    symbuf = symbol;
    ret = register_fprobe_syms(&sample_probe, (const char **)&symbuf, 1);
    goto out;
    }
// Comma separated symbols
    symbuf = kstrdup(symbol, GFP_KERNEL);
    if (!symbuf)
    return -ENOMEM;
    p = symbuf;
    count = 1;
    while ((p = strchr(++p, ',')) != core::ptr::null_mut())
    count++;
    pr_info("%d symbols found\n", count);
    syms = kcalloc(count, sizeof(char *), GFP_KERNEL);
    if (!syms) {
    kfree(symbuf);
    return -ENOMEM;
    }
    p = symbuf;
    for (i = 0; i < count; i++)
    syms[i] = strsep(&p, ",");
    ret = register_fprobe_syms(&sample_probe, syms, count);
    kfree(syms);
    kfree(symbuf);
    out:
    if (ret < 0)
    pr_err("register_fprobe failed, returned %d\n", ret);
    else
    pr_info("Planted fprobe at %s\n", symbol);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn fprobe_exit() -> void __exit {
    static void __exit fprobe_exit(void)
    {
    unregister_fprobe(&sample_probe);
    pr_info("fprobe at %s unregistered. %ld times hit, %ld times missed\n",
    symbol, nhit, sample_probe.nmissed);
    }
    module_init(fprobe_init)
    module_exit(fprobe_exit)
    MODULE_DESCRIPTION("sample kernel module showing the use of fprobe");
    MODULE_LICENSE("GPL");
