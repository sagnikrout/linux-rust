//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/asm-offsets_64.c
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

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {

    OFFSET(PV_IRQ_save_fl, paravirt_patch_template, irq.save_fl);

    BLANK();

    OFFSET(KVM_STEAL_TIME_preempted, kvm_steal_time, preempted);
    BLANK();

    ENTRY(bx);
    ENTRY(cx);
    ENTRY(dx);
    ENTRY(sp);
    ENTRY(bp);
    ENTRY(si);
    ENTRY(di);
    ENTRY(r8);
    ENTRY(r9);
    ENTRY(r10);
    ENTRY(r11);
    ENTRY(r12);
    ENTRY(r13);
    ENTRY(r14);
    ENTRY(r15);
    ENTRY(flags);
    BLANK();

    ENTRY(cr0);
    ENTRY(cr2);
    ENTRY(cr3);
    ENTRY(cr4);
    ENTRY(gdt_desc);
    BLANK();

    return 0;
    }
