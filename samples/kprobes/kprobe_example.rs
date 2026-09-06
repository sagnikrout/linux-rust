//! Automatically rewritten from C to Rust
//! Source: samples/kprobes/kprobe_example.c
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
// Here's a sample kernel module showing the use of kprobes to dump a
// stack trace and selected registers when kernel_clone() is called.
//
// For more information on theory of operation of kprobes, see
// Documentation/trace/kprobes.rst
//
// You will see the trace data in /var/log/messages and on the console
// whenever kernel_clone() is invoked to create a new process.
//

    static char symbol[KSYM_NAME_LEN] = "kernel_clone";
    module_param_string(symbol, symbol, KSYM_NAME_LEN, 0644);
// For each probe you need to allocate a kprobe structure
    static struct kprobe kp = {
    .symbol_name	= symbol,
    };
// kprobe pre_handler: called just before the probed instruction is executed
#[no_mangle]
unsafe extern "C" fn handler_pre(p: *mut kprobe, regs: *mut pt_regs) -> int __kprobes {
    static int __kprobes handler_pre(struct kprobe *p, struct pt_regs *regs)
    {

    pr_info("<%s> p.addr = 0x%p, ip = %lx, flags = 0x%lx\n",
    p.symbol_name, p.addr, regs.ip, regs.flags);

    pr_info("<%s> p.addr = 0x%p, nip = 0x%lx, msr = 0x%lx\n",
    p.symbol_name, p.addr, regs.nip, regs.msr);

    pr_info("<%s> p.addr = 0x%p, epc = 0x%lx, status = 0x%lx\n",
    p.symbol_name, p.addr, regs.cp0_epc, regs.cp0_status);

    pr_info("<%s> p.addr = 0x%p, pc = 0x%lx, pstate = 0x%lx\n",
    p.symbol_name, p.addr, (long)regs.pc, (long)regs.pstate);

    pr_info("<%s> p.addr = 0x%p, pc = 0x%lx, cpsr = 0x%lx\n",
    p.symbol_name, p.addr, (long)regs.ARM_pc, (long)regs.ARM_cpsr);

    pr_info("<%s> p.addr = 0x%p, pc = 0x%lx, status = 0x%lx\n",
    p.symbol_name, p.addr, regs.epc, regs.status);

    pr_info("<%s> p.addr, 0x%p, ip = 0x%lx, flags = 0x%lx\n",
    p.symbol_name, p.addr, regs.psw.addr, regs.flags);

    pr_info("<%s> p.addr = 0x%p, era = 0x%lx, estat = 0x%lx\n",
    p.symbol_name, p.addr, regs.csr_era, regs.csr_estat);

// A dump_stack() here will give a stack backtrace
    return 0;
    }
// kprobe post_handler: called after the probed instruction is executed
    static void __kprobes handler_post(struct kprobe *p, struct pt_regs *regs,
    unsigned long flags)
    {

    pr_info("<%s> p.addr = 0x%p, flags = 0x%lx\n",
    p.symbol_name, p.addr, regs.flags);

    pr_info("<%s> p.addr = 0x%p, msr = 0x%lx\n",
    p.symbol_name, p.addr, regs.msr);

    pr_info("<%s> p.addr = 0x%p, status = 0x%lx\n",
    p.symbol_name, p.addr, regs.cp0_status);

    pr_info("<%s> p.addr = 0x%p, pstate = 0x%lx\n",
    p.symbol_name, p.addr, (long)regs.pstate);

    pr_info("<%s> p.addr = 0x%p, cpsr = 0x%lx\n",
    p.symbol_name, p.addr, (long)regs.ARM_cpsr);

    pr_info("<%s> p.addr = 0x%p, status = 0x%lx\n",
    p.symbol_name, p.addr, regs.status);

    pr_info("<%s> p.addr, 0x%p, flags = 0x%lx\n",
    p.symbol_name, p.addr, regs.flags);

    pr_info("<%s> p.addr = 0x%p, estat = 0x%lx\n",
    p.symbol_name, p.addr, regs.csr_estat);

    }
#[no_mangle]
unsafe extern "C" fn kprobe_init() -> int __init {
    static int __init kprobe_init(void)
    {
    int ret;
    kp.pre_handler = handler_pre;
    kp.post_handler = handler_post;
    ret = register_kprobe(&kp);
    if (ret < 0) {
    pr_err("register_kprobe failed, returned %d\n", ret);
    return ret;
    }
    pr_info("Planted kprobe at %p\n", kp.addr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kprobe_exit() -> void __exit {
    static void __exit kprobe_exit(void)
    {
    unregister_kprobe(&kp);
    pr_info("kprobe at %p unregistered\n", kp.addr);
    }
    module_init(kprobe_init)
    module_exit(kprobe_exit)
    MODULE_DESCRIPTION("sample kernel module showing the use of kprobes");
    MODULE_LICENSE("GPL");
