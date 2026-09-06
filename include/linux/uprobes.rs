//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/uprobes.h
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
// User-space Probes (UProbes)
//
// Copyright (C) IBM Corporation, 2008-2012
// Authors:
// Srikar Dronamraju
// Jim Keniston
// Copyright (C) 2011-2012 Red Hat, Inc., Peter Zijlstra
//

//
// Allowed return values from uprobe consumer's handler callback
// with following meaning:
//
// UPROBE_HANDLER_REMOVE
// - Remove the uprobe breakpoint from current->mm.
// UPROBE_HANDLER_IGNORE
// - Ignore ret_handler callback for this consumer.
//
pub const UPROBE_HANDLER_REMOVE: c_int = 1;
pub const UPROBE_HANDLER_IGNORE: c_int = 2;
pub const MAX_URETPROBE_DEPTH: c_int = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uprobe_consumer {
//
// handler() can return UPROBE_HANDLER_REMOVE to signal the need to
// unregister uprobe for current process. If UPROBE_HANDLER_REMOVE is
// returned, filter() callback has to be implemented as well and it
// should return false to "confirm" the decision to uninstall uprobe
// for the current process. If filter() is omitted or returns true,
// UPROBE_HANDLER_REMOVE is effectively ignored.
//
    pub data): *mut *mut *mut *mut int (handler)(struct uprobe_consumer self, struct pt_regs regs, __u64,
    pub data): *mut *mut pt_regs regs, __u64,
    pub mm): *mut *mut *mut bool (filter)(struct uprobe_consumer self, struct mm_struct,
    pub cons_node: list_head,
    pub /: *mut *mut __u64 id; / set when uprobe_consumer is registered,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uprobe_task_state {
    UTASK_RUNNING,
    UTASK_SSTEP,
    UTASK_SSTEP_ACK,
    UTASK_SSTEP_TRAPPED,
}

// The state of hybrid-lifetime uprobe inside struct return_instance
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hprobe_state {
    HPROBE_LEASED,		/* uretprobes_srcu-protected uprobe */
    HPROBE_STABLE,		/* refcounted uprobe */
    HPROBE_GONE,		/* NULL uprobe, SRCU expired, refcount failed */
    HPROBE_CONSUMED,	/* uprobe "consumed" by uretprobe handler */
}

//
// Hybrid lifetime uprobe. Represents a uprobe instance that could be either
// SRCU protected (with SRCU protection eventually potentially timing out),
// refcounted using uprobe->ref, or there could be no valid uprobe (NULL).
//
// hprobe's internal state is setup such that background timer thread can
// atomically "downgrade" temporarily RCU-protected uprobe into refcounted one
// (or no uprobe, if refcounting failed).
//
// *stable* pointer always point to the uprobe (or could be NULL if there is
// was no valid underlying uprobe to begin with).
//
// *leased* pointer is the key to achieving race-free atomic lifetime state
// transition and can have three possible states:
// - either the same non-NULL value as *stable*, in which case uprobe is
// SRCU-protected;
// - NULL, in which case uprobe (if there is any) is refcounted;
// - special __UPROBE_DEAD value, which represents an uprobe that was SRCU
// protected initially, but SRCU period timed out and we attempted to
// convert it to refcounted, but refcount_inc_not_zero() failed, because
// uprobe effectively went away (the last consumer unsubscribed). In this
// case it's important to know that *stable* pointer (which still has
// non-NULL uprobe pointer) shouldn't be used, because lifetime of
// underlying uprobe is not guaranteed anymore. __UPROBE_DEAD is just an
// internal marker and is handled transparently by hprobe_fetch() helper.
//
// When uprobe is SRCU-protected, we also record srcu_scp value, necessary for
// SRCU unlocking.
//
// See hprobe_expire() and hprobe_fetch() for details of race-free uprobe
// state transitioning details. It all hinges on atomic xchg() over *leaded
// pointer. *stable* pointer, once initially set, is not modified concurrently.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hprobe {
    pub state: hprobe_state,
    pub srcu_scp: *mut srcu_ctr __percpu,
    pub uprobe: *mut uprobe,
}

//
// uprobe_task: Metadata of a task while it singlesteps.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uprobe_task {
    pub state: uprobe_task_state,
    pub depth: c_uint,
    pub return_instances: *mut return_instance,
    pub ri_pool: *mut return_instance,
    pub ri_timer: timer_list,
    pub ri_seqcount: seqcount_t,
    pub autask: arch_uprobe_task,
    pub vaddr: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct return_consumer {
    pub cookie: __u64,
    pub id: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct return_instance {
    pub hprobe: hprobe,
    pub func: c_ulong,
    pub /: *mut *mut unsigned long stack; / stack pointer,
    pub /: *mut *mut unsigned long orig_ret_vaddr; / original return address,
    pub /: *mut *mut bool chained; / true, if instance is nested,
    pub /: *mut *mut int cons_cnt; / total number of session consumers,
    pub /: *mut *mut *mut return_instance next; / keep as stack,
    pub rcu: rcu_head,
// singular pre-allocated return_consumer instance for common case
    pub consumer: return_consumer,
//
// extra return_consumer instances for rare cases of multiple session consumers,
// contains (cons_cnt - 1) elements
//
    pub extra_consumers: *mut return_consumer,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rp_check {
    RP_CHECK_CALL,
    RP_CHECK_CHAIN_CALL,
    RP_CHECK_RET,
}

    pub xol_area: struct,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uprobes_state {
    pub xol_area: *mut xol_area,
}

extern "C" {
    pub fn uprobes_init() -> void __init;
}
extern "C" {
    pub fn set_swbp(aup: *mut arch_uprobe, vma: *mut vm_area_struct, vaddr: c_ulong) -> c_int;
}
extern "C" {
    pub fn set_orig_insn(aup: *mut arch_uprobe, vma: *mut vm_area_struct, vaddr: c_ulong) -> c_int;
}
extern "C" {
    pub fn is_swbp_insn(insn: *mut uprobe_opcode_t) -> bool;
}
extern "C" {
    pub fn is_trap_insn(insn: *mut uprobe_opcode_t) -> bool;
}
extern "C" {
    pub fn uprobe_get_swbp_addr(regs: *mut pt_regs) -> c_ulong;
}
extern "C" {
    pub fn uprobe_get_trap_addr(regs: *mut pt_regs) -> c_ulong;
}
extern "C" {
    pub fn uprobe_apply(uprobe: *mut uprobe, uc: *mut uprobe_consumer, _arg: bool) -> c_int;
}
extern "C" {
    pub fn uprobe_unregister_nosync(uprobe: *mut uprobe, uc: *mut uprobe_consumer);
}
extern "C" {
    pub fn uprobe_unregister_sync();
}
extern "C" {
    pub fn uprobe_mmap(vma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn uprobe_munmap(vma: *mut vm_area_struct, start: c_ulong, end: c_ulong);
}
extern "C" {
    pub fn uprobe_start_dup_mmap();
}
extern "C" {
    pub fn uprobe_end_dup_mmap();
}
extern "C" {
    pub fn uprobe_dup_mmap(oldmm: *mut mm_struct, newmm: *mut mm_struct);
}
extern "C" {
    pub fn uprobe_free_utask(t: *mut task_struct);
}
extern "C" {
    pub fn uprobe_copy_process(t: *mut task_struct, flags: u64);
}
extern "C" {
    pub fn uprobe_post_sstep_notifier(regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn uprobe_pre_sstep_notifier(regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn uprobe_notify_resume(regs: *mut pt_regs);
}
extern "C" {
    pub fn uprobe_deny_signal() -> bool;
}
extern "C" {
    pub fn arch_uprobe_skip_sstep(aup: *mut arch_uprobe, regs: *mut pt_regs) -> bool;
}
extern "C" {
    pub fn uprobe_clear_state(mm: *mut mm_struct);
}
extern "C" {
    pub fn arch_uprobe_analyze_insn(aup: *mut arch_uprobe, mm: *mut mm_struct, addr: c_ulong) -> c_int;
}
extern "C" {
    pub fn arch_uprobe_pre_xol(aup: *mut arch_uprobe, regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn arch_uprobe_post_xol(aup: *mut arch_uprobe, regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn arch_uprobe_xol_was_trapped(tsk: *mut task_struct) -> bool;
}
extern "C" {
    pub fn arch_uprobe_exception_notify(self: *mut notifier_block, val: c_ulong, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn arch_uprobe_abort_xol(aup: *mut arch_uprobe, regs: *mut pt_regs);
}
extern "C" {
    pub fn arch_uretprobe_hijack_return_addr(trampoline_vaddr: c_ulong, regs: *mut pt_regs) -> c_ulong;
}
extern "C" {
    pub fn arch_uretprobe_is_alive(ret: *mut return_instance, ctx: rp_check, regs: *mut pt_regs) -> bool;
}
extern "C" {
    pub fn arch_uprobe_ignore(aup: *mut arch_uprobe, regs: *mut pt_regs) -> bool;
}
extern "C" {
    pub fn uprobe_handle_trampoline(regs: *mut pt_regs);
}
extern "C" {
    pub fn uprobe_get_trampoline_vaddr() -> c_ulong;
}
extern "C" {
    pub fn uprobe_copy_from_page(page: *mut page, vaddr: c_ulong, dst: *mut c_void, len: c_int);
}
extern "C" {
    pub fn handle_syscall_uprobe(regs: *mut pt_regs, bp_vaddr: c_ulong);
}
extern "C" {
    pub fn arch_uprobe_optimize(auprobe: *mut arch_uprobe, vaddr: c_ulong);
}
extern "C" {
    pub fn arch_uprobe_get_xol_area() -> c_ulong;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uprobes_state {
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}

