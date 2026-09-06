//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/asm-offsets_32.c
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

// workaround for a warning with -Wmissing-prototypes
    void foo(void);
#[no_mangle]
pub unsafe extern "C" fn foo() {
    void foo(void)
    {
    OFFSET(PT_EBX, pt_regs, bx);
    OFFSET(PT_ECX, pt_regs, cx);
    OFFSET(PT_EDX, pt_regs, dx);
    OFFSET(PT_ESI, pt_regs, si);
    OFFSET(PT_EDI, pt_regs, di);
    OFFSET(PT_EBP, pt_regs, bp);
    OFFSET(PT_EAX, pt_regs, ax);
    OFFSET(PT_DS,  pt_regs, ds);
    OFFSET(PT_ES,  pt_regs, es);
    OFFSET(PT_FS,  pt_regs, fs);
    OFFSET(PT_GS,  pt_regs, gs);
    OFFSET(PT_ORIG_EAX, pt_regs, orig_ax);
    OFFSET(PT_EIP, pt_regs, ip);
    OFFSET(PT_CS,  pt_regs, cs);
    OFFSET(PT_EFLAGS, pt_regs, flags);
    OFFSET(PT_OLDESP, pt_regs, sp);
    OFFSET(PT_OLDSS,  pt_regs, ss);
    BLANK();
    OFFSET(saved_context_gdt_desc, saved_context, gdt_desc);
    BLANK();
//
// Offset from the entry stack to task stack stored in TSS. Kernel entry
// happens on the per-cpu entry-stack, and the asm code switches to the
// task-stack pointer stored in x86_tss.sp1, which is a copy of
// task->thread.sp0 where entry code can find it.
//
    DEFINE(TSS_entry2task_stack,
    offsetof(struct cpu_entry_area, tss.x86_tss.sp1) -
    offsetofend(struct cpu_entry_area, entry_stack_page.stack));
    BLANK();
    DEFINE(EFI_svam, offsetof(efi_runtime_services_t, set_virtual_address_map));
    }
