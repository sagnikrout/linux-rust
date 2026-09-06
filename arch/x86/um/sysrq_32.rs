//! Automatically rewritten from C to Rust
//! Source: arch/x86/um/sysrq_32.c
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


//
// Copyright (C) 2001 - 2003 Jeff Dike (jdike@addtoit.com)
// Licensed under the GPL
//

// This is declared by <linux/sched.h>
#[no_mangle]
pub unsafe extern "C" fn show_regs(regs: *mut pt_regs) {
    void show_regs(struct pt_regs *regs)
    {
    printk("\n");
    printk("EIP: %04lx:[<%08lx>] CPU: %d %s",
    0xffff & PT_REGS_CS(regs), PT_REGS_IP(regs),
    smp_processor_id(), print_tainted());
    if (PT_REGS_CS(regs) & 3)
    printk(" ESP: %04lx:%08lx", 0xffff & PT_REGS_SS(regs),
    PT_REGS_SP(regs));
    printk(" EFLAGS: %08lx\n    %s\n", PT_REGS_EFLAGS(regs),
    print_tainted());
    printk("EAX: %08lx EBX: %08lx ECX: %08lx EDX: %08lx\n",
    PT_REGS_AX(regs), PT_REGS_BX(regs),
    PT_REGS_CX(regs), PT_REGS_DX(regs));
    printk("ESI: %08lx EDI: %08lx EBP: %08lx",
    PT_REGS_SI(regs), PT_REGS_DI(regs), PT_REGS_BP(regs));
    printk(" DS: %04lx ES: %04lx\n",
    0xffff & PT_REGS_DS(regs),
    0xffff & PT_REGS_ES(regs));
    }
