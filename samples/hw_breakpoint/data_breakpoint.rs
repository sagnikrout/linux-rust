//! Automatically rewritten from C to Rust
//! Source: samples/hw_breakpoint/data_breakpoint.c
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
// data_breakpoint.c - Sample HW Breakpoint file to watch kernel data address
//
// usage: insmod data_breakpoint.ko ksym=<ksym_name>
//
// This file is a kernel module that places a breakpoint over ksym_name kernel
// variable using Hardware Breakpoint register. The corresponding handler which
// prints a backtrace is invoked every time a write operation is performed on
// that variable.
//
// Copyright (C) IBM Corporation, 2009
//
// Author: K.Prasad <prasad@linux.vnet.ibm.com>
//

    static struct perf_event * __percpu *sample_hbp;
    static char ksym_name[KSYM_NAME_LEN] = "jiffies";
    module_param_string(ksym, ksym_name, KSYM_NAME_LEN, S_IRUGO);
    MODULE_PARM_DESC(ksym, "Kernel symbol to monitor; this module will report any"
    " write operations on the kernel symbol");
    static void sample_hbp_handler(struct perf_event *bp,
    struct perf_sample_data *data,
    struct pt_regs *regs)
    {
    printk(KERN_INFO "%s value is changed\n", ksym_name);
    dump_stack();
    printk(KERN_INFO "Dump stack from sample_hbp_handler\n");
    }
#[no_mangle]
unsafe extern "C" fn hw_break_module_init() -> int __init {
    static int __init hw_break_module_init(void)
    {
    int ret;
    struct perf_event_attr attr;
    void *addr = __symbol_get(ksym_name);
    if (!addr)
    return -ENXIO;
    hw_breakpoint_init(&attr);
    attr.bp_addr = (unsigned long)addr;
    attr.bp_len = HW_BREAKPOINT_LEN_4;
    attr.bp_type = HW_BREAKPOINT_W;
    sample_hbp = register_wide_hw_breakpoint(&attr, sample_hbp_handler, core::ptr::null_mut());
    if (IS_ERR_PCPU(sample_hbp)) {
    ret = PTR_ERR_PCPU(sample_hbp);
    goto fail;
    }
    printk(KERN_INFO "HW Breakpoint for %s write installed\n", ksym_name);
    return 0;
    fail:
    printk(KERN_INFO "Breakpoint registration failed\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hw_break_module_exit() -> void __exit {
    static void __exit hw_break_module_exit(void)
    {
    unregister_wide_hw_breakpoint(sample_hbp);

    __symbol_put(ksym_name);

    printk(KERN_INFO "HW Breakpoint for %s write uninstalled\n", ksym_name);
    }
    module_init(hw_break_module_init);
    module_exit(hw_break_module_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("K.Prasad");
    MODULE_DESCRIPTION("ksym breakpoint");
