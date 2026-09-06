//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kernel/sysrq.c
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
// LoongArch specific sysrq operations.
//
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

//
// Dump TLB entries on all CPUs.
//
    static DEFINE_SPINLOCK(show_lock);
#[no_mangle]
unsafe extern "C" fn sysrq_tlbdump_single(dummy: *mut c_void) {
    static void sysrq_tlbdump_single(void *dummy)
    {
    unsigned long flags;
    spin_lock_irqsave(&show_lock, flags);
    pr_info("CPU%d:\n", smp_processor_id());
    dump_tlb_regs();
    pr_info("\n");
    dump_tlb_all();
    pr_info("\n");
    spin_unlock_irqrestore(&show_lock, flags);
    }

#[no_mangle]
unsafe extern "C" fn sysrq_tlbdump_othercpus(dummy: *mut work_struct) {
    static void sysrq_tlbdump_othercpus(struct work_struct *dummy)
    {
    smp_call_function(sysrq_tlbdump_single, core::ptr::null_mut(), 0);
    }
    static DECLARE_WORK(sysrq_tlbdump, sysrq_tlbdump_othercpus);

#[no_mangle]
unsafe extern "C" fn sysrq_handle_tlbdump(key: u8) {
    static void sysrq_handle_tlbdump(u8 key)
    {
    sysrq_tlbdump_single(core::ptr::null_mut());

    schedule_work(&sysrq_tlbdump);

    }
    static struct sysrq_key_op sysrq_tlbdump_op = {
    .handler        = sysrq_handle_tlbdump,
    .help_msg       = "show-tlbs(x)",
    .action_msg     = "Show TLB entries",
    .enable_mask	= SYSRQ_ENABLE_DUMP,
    };
#[no_mangle]
unsafe extern "C" fn loongarch_sysrq_init() -> int __init {
    static int __init loongarch_sysrq_init(void)
    {
    return register_sysrq_key('x', &sysrq_tlbdump_op);
    }
    arch_initcall(loongarch_sysrq_init);
