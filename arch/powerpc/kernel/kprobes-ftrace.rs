//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/kprobes-ftrace.c
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
// Dynamic Ftrace based Kprobes Optimization
//
// Copyright (C) Hitachi Ltd., 2012
// Copyright 2016 Naveen N. Rao <naveen.n.rao@linux.vnet.ibm.com>
// IBM Corporation
//

// Ftrace callback handler for kprobes
    void kprobe_ftrace_handler(unsigned long nip, unsigned long parent_nip,
    struct ftrace_ops *ops, struct ftrace_regs *fregs)
    {
    struct kprobe *p;
    struct kprobe_ctlblk *kcb;
    struct pt_regs *regs;
    int bit;
    if (unlikely(kprobe_ftrace_disabled))
    return;
    bit = ftrace_test_recursion_trylock(nip, parent_nip);
    if (bit < 0)
    return;
    regs = ftrace_get_regs(fregs);
    p = get_kprobe((kprobe_opcode_t *)nip);
    if (unlikely(!p) || kprobe_disabled(p))
    goto out;
    kcb = get_kprobe_ctlblk();
    if (kprobe_running()) {
    kprobes_inc_nmissed_count(p);
    } else {
//
// On powerpc, NIP is *before* this instruction for the
// pre handler
//
    regs_add_return_ip(regs, -MCOUNT_INSN_SIZE);
    __this_cpu_write(current_kprobe, p);
    kcb.kprobe_status = KPROBE_HIT_ACTIVE;
    if (!p.pre_handler || !p.pre_handler(p, regs)) {
//
// Emulate singlestep (and also recover regs->nip)
// as if there is a nop
//
    regs_add_return_ip(regs, MCOUNT_INSN_SIZE);
    if (unlikely(p.post_handler)) {
    kcb.kprobe_status = KPROBE_HIT_SSDONE;
    p.post_handler(p, regs, 0);
    }
    }
//
// If pre_handler returns !0, it changes regs->nip. We have to
// skip emulating post_handler.
//
    __this_cpu_write(current_kprobe, core::ptr::null_mut());
    }
    out:
    ftrace_test_recursion_unlock(bit);
    }
    NOKPROBE_SYMBOL(kprobe_ftrace_handler);
#[no_mangle]
pub unsafe extern "C" fn arch_prepare_kprobe_ftrace(p: *mut kprobe) -> c_int {
    int arch_prepare_kprobe_ftrace(struct kprobe *p)
    {
    p.ainsn.insn = core::ptr::null_mut();
    p.ainsn.boostable = -1;
    return 0;
    }
