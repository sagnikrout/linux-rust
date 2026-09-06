//! Automatically rewritten from C to Rust
//! Source: arch/x86/um/sysrq_64.c
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
// Copyright 2003 PathScale, Inc.
//
// Licensed under the GPL
//

#[no_mangle]
pub unsafe extern "C" fn show_regs(regs: *mut pt_regs) {
    void show_regs(struct pt_regs *regs)
    {
    printk("\n");
    print_modules();
    printk(KERN_INFO "Pid: %d, comm: %.20s %s %s\n", task_pid_nr(current),
    current.comm, print_tainted(), init_utsname().release);
    printk(KERN_INFO "RIP: %04lx:%pS\n", PT_REGS_CS(regs) & 0xffff,
    (void *)PT_REGS_IP(regs));
    printk(KERN_INFO "RSP: %016lx  EFLAGS: %08lx\n", PT_REGS_SP(regs),
    PT_REGS_EFLAGS(regs));
    printk(KERN_INFO "RAX: %016lx RBX: %016lx RCX: %016lx\n",
    PT_REGS_AX(regs), PT_REGS_BX(regs), PT_REGS_CX(regs));
    printk(KERN_INFO "RDX: %016lx RSI: %016lx RDI: %016lx\n",
    PT_REGS_DX(regs), PT_REGS_SI(regs), PT_REGS_DI(regs));
    printk(KERN_INFO "RBP: %016lx R08: %016lx R09: %016lx\n",
    PT_REGS_BP(regs), PT_REGS_R8(regs), PT_REGS_R9(regs));
    printk(KERN_INFO "R10: %016lx R11: %016lx R12: %016lx\n",
    PT_REGS_R10(regs), PT_REGS_R11(regs), PT_REGS_R12(regs));
    printk(KERN_INFO "R13: %016lx R14: %016lx R15: %016lx\n",
    PT_REGS_R13(regs), PT_REGS_R14(regs), PT_REGS_R15(regs));
    }
