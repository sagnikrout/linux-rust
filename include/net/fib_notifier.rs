//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/fib_notifier.h
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


#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_notifier_info {
    pub family: c_int,
    pub extack: *mut netlink_ext_ack,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fib_event_type {
    FIB_EVENT_ENTRY_REPLACE,
    FIB_EVENT_ENTRY_APPEND,
    FIB_EVENT_ENTRY_ADD,
    FIB_EVENT_ENTRY_DEL,
    FIB_EVENT_RULE_ADD,
    FIB_EVENT_RULE_DEL,
    FIB_EVENT_NH_ADD,
    FIB_EVENT_NH_DEL,
    FIB_EVENT_VIF_ADD,
    FIB_EVENT_VIF_DEL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_notifier_ops {
    pub family: c_int,
    pub list: list_head,
    pub net): *const *const unsigned int (fib_seq_read)(struct net,
    pub extack): *mut netlink_ext_ack,
    pub owner: *mut module,
    pub rcu: rcu_head,
}

extern "C" {
    pub fn unregister_fib_notifier(net: *mut net, nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn fib_notifier_ops_unregister(ops: *mut fib_notifier_ops);
}
