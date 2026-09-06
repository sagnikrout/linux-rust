//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/44x/machine_check.c
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

#[no_mangle]
pub unsafe extern "C" fn machine_check_4xx(regs: *mut pt_regs) -> c_int {
    int machine_check_4xx(struct pt_regs *regs)
    {
    let mut reason: c_ulong = regs.esr;
    if (reason & ESR_IMCP) {
    printk("Instruction");
    mtspr(SPRN_ESR, reason & ~ESR_IMCP);
    } else
    printk("Data");
    printk(" machine check in kernel mode.\n");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn machine_check_440A(regs: *mut pt_regs) -> c_int {
    int machine_check_440A(struct pt_regs *regs)
    {
    let mut reason: c_ulong = regs.esr;
    printk("Machine check in kernel mode.\n");
    if (reason & ESR_IMCP){
    printk("Instruction Synchronous Machine Check exception\n");
    mtspr(SPRN_ESR, reason & ~ESR_IMCP);
    }
    else {
    let mut mcsr: u32 = mfspr(SPRN_MCSR);
    if (mcsr & MCSR_IB)
    printk("Instruction Read PLB Error\n");
    if (mcsr & MCSR_DRB)
    printk("Data Read PLB Error\n");
    if (mcsr & MCSR_DWB)
    printk("Data Write PLB Error\n");
    if (mcsr & MCSR_TLBP)
    printk("TLB Parity Error\n");
    if (mcsr & MCSR_ICP){
    flush_instruction_cache();
    printk("I-Cache Parity Error\n");
    }
    if (mcsr & MCSR_DCSP)
    printk("D-Cache Search Parity Error\n");
    if (mcsr & MCSR_DCFP)
    printk("D-Cache Flush Parity Error\n");
    if (mcsr & MCSR_IMPE)
    printk("Machine Check exception is imprecise\n");
// Clear MCSR
    mtspr(SPRN_MCSR, mcsr);
    }
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn machine_check_47x(regs: *mut pt_regs) -> c_int {
    int machine_check_47x(struct pt_regs *regs)
    {
    let mut reason: c_ulong = regs.esr;
    u32 mcsr;
    printk(KERN_ERR "Machine check in kernel mode.\n");
    if (reason & ESR_IMCP) {
    printk(KERN_ERR "Instruction Synchronous Machine Check exception\n");
    mtspr(SPRN_ESR, reason & ~ESR_IMCP);
    return 0;
    }
    mcsr = mfspr(SPRN_MCSR);
    if (mcsr & MCSR_IB)
    printk(KERN_ERR "Instruction Read PLB Error\n");
    if (mcsr & MCSR_DRB)
    printk(KERN_ERR "Data Read PLB Error\n");
    if (mcsr & MCSR_DWB)
    printk(KERN_ERR "Data Write PLB Error\n");
    if (mcsr & MCSR_TLBP)
    printk(KERN_ERR "TLB Parity Error\n");
    if (mcsr & MCSR_ICP) {
    flush_instruction_cache();
    printk(KERN_ERR "I-Cache Parity Error\n");
    }
    if (mcsr & MCSR_DCSP)
    printk(KERN_ERR "D-Cache Search Parity Error\n");
    if (mcsr & PPC47x_MCSR_GPR)
    printk(KERN_ERR "GPR Parity Error\n");
    if (mcsr & PPC47x_MCSR_FPR)
    printk(KERN_ERR "FPR Parity Error\n");
    if (mcsr & PPC47x_MCSR_IPR)
    printk(KERN_ERR "Machine Check exception is imprecise\n");
// Clear MCSR
    mtspr(SPRN_MCSR, mcsr);
    return 0;
    }
