//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kprobes.h
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
// Kernel Probes (KProbes)
//
// Copyright (C) IBM Corporation, 2002, 2004
//
// 2002-Oct	Created by Vamsi Krishna S <vamsi_krishna@in.ibm.com> Kernel
// Probes initial implementation ( includes suggestions from
// Rusty Russell).
// 2004-July	Suparna Bhattacharya <suparna@in.ibm.com> added jumper probes
// interface to access function arguments.
// 2005-May	Hien Nguyen <hien@us.ibm.com> and Jim Keniston
// <jkenisto@us.ibm.com>  and Prasanna S Panchamukhi
// <prasanna@in.ibm.com> added function-return probes.
//

// kprobe_status settings
pub const KPROBE_HIT_ACTIVE: c_uint = 0x00000001;
pub const KPROBE_HIT_SS: c_uint = 0x00000002;
pub const KPROBE_REENTER: c_uint = 0x00000004;
pub const KPROBE_HIT_SSDONE: c_uint = 0x00000008;

pub type kprobe_opcode_t = c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_specific_insn {
    pub dummy: c_int,
}

extern "C" {
    pub fn int(: *mut *mut kprobe_pre_handler_t) (struct kprobe, : *mut pt_regs) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kprobe {
    pub hlist: hlist_node,
// list of kprobes for multi-handler support
    pub list: list_head,
// count the number of times this probe was temporarily disarmed
    pub nmissed: c_ulong,
// location of the probe point
    pub addr: *mut kprobe_opcode_t,
// Allow user to indicate symbol name of the probe point
    pub symbol_name: *const c_char,
// Offset into the symbol
    pub offset: c_uint,
// Called before addr is executed.
    pub pre_handler: kprobe_pre_handler_t,
// Called after addr is executed, unless...
    pub post_handler: kprobe_post_handler_t,
// Saved opcode (which has been replaced with breakpoint)
    pub opcode: kprobe_opcode_t,
// copy of the original instruction
    pub ainsn: arch_specific_insn,
//
// Indicates various status flags.
// Protected by kprobe_mutex after this kprobe is registered.
//
    pub flags: u32,
}

// Kprobe status flags

// probe is really optimized.
// NOTE:
// this flag is only for optimized_kprobe.
//

// Has this kprobe gone ?
// Is this kprobe disabled ?
// Is this kprobe really running optimized path ?
// Is this kprobe uses ftrace ?
//
// Function-return probe -
// Note:
// User needs to provide a handler function, and initialize maxactive.
// maxactive - The maximum number of instances of the probed function that
// can be active concurrently.
// nmissed - tracks the number of times the probed function's return was
// ignored, due to maxactive being too low.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kretprobe_holder {
    pub rp: *mut kretprobe __rcu,
    pub pool: objpool_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kretprobe {
    pub kp: kprobe,
    pub handler: kretprobe_handler_t,
    pub entry_handler: kretprobe_handler_t,
    pub maxactive: c_int,
    pub nmissed: c_int,
    pub data_size: usize,

    pub rh: *mut rethook,

    pub rph: *mut kretprobe_holder,

}

pub const KRETPROBE_MAX_DATA_SIZE: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kretprobe_instance {

    pub node: rethook_node,

    pub rcu: rcu_head,
    pub llist: llist_node,
    pub rph: *mut kretprobe_holder,
    pub ret_addr: *mut kprobe_opcode_t,
    pub fp: *mut c_void,
    pub data: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kretprobe_blackpoint {
    pub name: *const c_char,
    pub addr: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kprobe_blacklist_entry {
    pub list: list_head,
    pub start_addr: c_ulong,
    pub end_addr: c_ulong,
    pub rcu: rcu_head,
}

extern "C" {
    pub fn kprobe_busy_begin();
}
extern "C" {
    pub fn kprobe_busy_end();
}

// Check whether @p is used for implementing a trampoline.
extern "C" {
    pub fn arch_trampoline_kprobe(p: *mut kprobe) -> c_int;
}

// rethook::data is non-changed field, so that you can access it freely.

extern "C" {
    pub fn __kretprobe_trampoline();
}
//
// Since some architecture uses structured function pointer,
// use dereference_function_descriptor() to get real function address.
//
extern "C" {
    pub fn dereference_kernel_function_descriptor(_arg: __kretprobe_trampoline) -> return;
}
// If the trampoline handler called from a kprobe, use this version
//
// Set a dummy kprobe for avoiding kretprobe recursion.
// Since kretprobe never runs in kprobe handler, no kprobe must
// be running at this point.
//
extern "C" {
    pub fn rcu_dereference_check(_arg: ri->rph->rp, _arg: rcu_read_lock_any_held()) -> return;
}

// Markers of '_kprobe_blacklist' section
extern "C" {
    pub fn arch_prepare_kprobe(p: *mut kprobe) -> c_int;
}
extern "C" {
    pub fn arch_arm_kprobe(p: *mut kprobe);
}
extern "C" {
    pub fn arch_disarm_kprobe(p: *mut kprobe);
}
extern "C" {
    pub fn arch_init_kprobes() -> c_int;
}
extern "C" {
    pub fn kprobes_inc_nmissed_count(p: *mut kprobe);
}
extern "C" {
    pub fn arch_within_kprobe_blacklist(addr: c_ulong) -> bool;
}
extern "C" {
    pub fn arch_populate_kprobe_blacklist() -> c_int;
}
extern "C" {
    pub fn kprobe_on_func_entry(addr: *mut kprobe_opcode_t, sym: *const c_char, offset: c_ulong) -> c_int;
}
extern "C" {
    pub fn within_kprobe_blacklist(addr: c_ulong) -> bool;
}
extern "C" {
    pub fn kprobe_add_ksym_blacklist(entry: c_ulong) -> c_int;
}
extern "C" {
    pub fn kprobe_add_area_blacklist(start: c_ulong, end: c_ulong) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kprobe_insn_cache {
    pub mutex: mutex,
    pub /: *mut *mut *mut *mut void (alloc)(void); / allocate insn page,
    pub /: *mut *mut *mut *mut void (free)(void ); / free insn page,
    pub /: *const *const *const char sym; / symbol for insn pages,
    pub /: *mut *mut list_head pages; / list of kprobe_insn_page,
    pub /: *mut *mut size_t insn_size; / size of instruction slot,
    pub nr_garbage: c_int,
}

// sleep-less address checking routine

//
// Internal structure for direct jump optimized probe
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct optimized_kprobe {
    pub kp: kprobe,
    pub /: *mut *mut list_head list; / list for optimizing queue,
    pub optinsn: arch_optimized_insn,
}

// Architecture dependent functions for direct jump optimization
extern "C" {
    pub fn arch_prepared_optinsn(optinsn: *mut arch_optimized_insn) -> c_int;
}
extern "C" {
    pub fn arch_check_optimized_kprobe(op: *mut optimized_kprobe) -> c_int;
}
extern "C" {
    pub fn arch_remove_optimized_kprobe(op: *mut optimized_kprobe);
}
extern "C" {
    pub fn arch_optimize_kprobes(oplist: *mut list_head);
}
extern "C" {
    pub fn arch_unoptimize_kprobe(op: *mut optimized_kprobe);
}
extern "C" {
    pub fn opt_pre_handler(p: *mut kprobe, regs: *mut pt_regs);
}
extern "C" {
    pub fn wait_for_kprobe_optimizer();
}
extern "C" {
    pub fn optprobe_queued_unopt(op: *mut optimized_kprobe) -> bool;
}
extern "C" {
    pub fn kprobe_disarmed(p: *mut kprobe) -> bool;
}

extern "C" {
    pub fn arch_prepare_kprobe_ftrace(p: *mut kprobe) -> c_int;
}
// Set when ftrace has been killed: kprobes on ftrace must be disabled for safety
extern "C" {
    pub fn kprobe_ftrace_kill();
}

// Get the kprobe at this addr (if any) - called with preemption disabled
// kprobe_running() will just return the current_kprobe on this CPU
extern "C" {
    pub fn __this_cpu_read(_arg: current_kprobe) -> return;
}
extern "C" {
    pub fn this_cpu_ptr(_arg: &kprobe_ctlblk) -> return;
}
extern "C" {
    pub fn register_kprobe(p: *mut kprobe) -> c_int;
}
extern "C" {
    pub fn unregister_kprobe(p: *mut kprobe);
}
extern "C" {
    pub fn register_kprobes(kps: *mut kprobe, num: c_int) -> c_int;
}
extern "C" {
    pub fn unregister_kprobes(kps: *mut kprobe, num: c_int);
}
extern "C" {
    pub fn register_kretprobe(rp: *mut kretprobe) -> c_int;
}
extern "C" {
    pub fn unregister_kretprobe(rp: *mut kretprobe);
}
extern "C" {
    pub fn register_kretprobes(rps: *mut kretprobe, num: c_int) -> c_int;
}
extern "C" {
    pub fn unregister_kretprobes(rps: *mut kretprobe, num: c_int);
}

extern "C" {
    pub fn kprobe_flush_task(tk: *mut task_struct);
}

extern "C" {
    pub fn kprobe_free_init_mem();
}
extern "C" {
    pub fn disable_kprobe(kp: *mut kprobe) -> c_int;
}
extern "C" {
    pub fn enable_kprobe(kp: *mut kprobe) -> c_int;
}
extern "C" {
    pub fn dump_kprobe(kp: *mut kprobe);
}
extern "C" {
    pub fn free_optinsn_page(page: *mut c_void);
}

extern "C" {
    pub fn disable_kprobe(_arg: &rp->kp) -> return;
}
extern "C" {
    pub fn enable_kprobe(_arg: &rp->kp) -> return;
}

extern "C" {
    pub fn is_rethook_trampoline(_arg: addr) -> return;
}
extern "C" {
    pub fn rethook_find_ret_addr(_arg: tsk, long)fp: (unsigned, _arg: cur) -> return;
}

// Returns true if kprobes handled the fault
//
// To be potentially processing a kprobe fault and to be allowed
// to call kprobe_running(), we have to be non-preemptible.
//
extern "C" {
    pub fn kprobe_fault_handler(_arg: regs, _arg: trap) -> return;
}
