//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rethook.h
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
// Return hooking with list-based shadow stack.
//

extern "C" {
    pub fn void(: *mut *mut rethook_handler_t) (struct rethook_node, : *mut c_void, long: unsigned, : *mut pt_regs) -> typedef;
}
//
// struct rethook - The rethook management data structure.
// @data: The user-defined data storage.
// @handler: The user-defined return hook handler.
// @pool: The pool of struct rethook_node.
// @ref: The reference counter.
// @rcu: The rcu_head for deferred freeing.
//
// Don't embed to another data structure, because this is a self-destructive
// data structure when all rethook_node are freed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rethook {
    pub data: *mut c_void,
//
// To avoid sparse warnings, this uses a raw function pointer with
// __rcu, instead of rethook_handler_t. But this must be same as
// rethook_handler_t.
//
    pub ): *mut *mut *mut *mut void (__rcu handler) (struct rethook_node , void , unsigned long, struct pt_regs,
    pub pool: objpool_head,
    pub rcu: rcu_head,
}

//
// struct rethook_node - The rethook shadow-stack entry node.
// @rcu: The rcu_head for deferred freeing.
// @llist: The llist, linked to a struct task_struct::rethooks.
// @rethook: The pointer to the struct rethook.
// @ret_addr: The storage for the real return address.
// @frame: The storage for the frame pointer.
//
// You can embed this to your extended data structure to store any data
// on each entry of the shadow stack.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rethook_node {
    pub rcu: rcu_head,
    pub llist: llist_node,
    pub rethook: *mut rethook,
    pub ret_addr: c_ulong,
    pub frame: c_ulong,
}

extern "C" {
    pub fn rethook_stop(rh: *mut rethook);
}
extern "C" {
    pub fn rethook_free(rh: *mut rethook);
}
extern "C" {
    pub fn rethook_recycle(node: *mut rethook_node);
}
extern "C" {
    pub fn rethook_hook(node: *mut rethook_node, regs: *mut pt_regs, mcount: bool);
}
// Arch dependent code must implement arch_* and trampoline code
extern "C" {
    pub fn arch_rethook_prepare(node: *mut rethook_node, regs: *mut pt_regs, mcount: bool);
}
extern "C" {
    pub fn arch_rethook_trampoline();
}
//
// is_rethook_trampoline() - Check whether the address is rethook trampoline
// @addr: The address to be checked
//
// Return true if the @addr is the rethook trampoline address.
//
// If the architecture needs to fixup the return address, implement it.
// Generic trampoline handler, arch code must prepare asm stub

extern "C" {
    pub fn rethook_flush_task(tk: *mut task_struct);
}

