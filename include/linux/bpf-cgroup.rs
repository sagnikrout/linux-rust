//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bpf-cgroup.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_storage_buffer {
    pub rcu: rcu_head,
    pub data: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_cgroup_storage {
    pub buf: *mut bpf_storage_buffer,
    pub percpu_buf: *mut void __percpu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_cgroup_link {
    pub link: bpf_link,
    pub cgroup: *mut cgroup,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_list {
    pub node: hlist_node,
    pub prog: *mut bpf_prog,
    pub link: *mut bpf_cgroup_link,
    pub storage: [*mut bpf_cgroup_storage; MAX_BPF_CGROUP_STORAGE_TYPE],
    pub flags: u32,
}

extern "C" {
    pub fn cgroup_bpf_lifetime_notifier_init() -> void __init;
}
extern "C" {
    pub fn bpf_cgroup_storage_free(storage: *mut bpf_cgroup_storage);
}
extern "C" {
    pub fn bpf_cgroup_storage_unlink(storage: *mut bpf_cgroup_storage);
}
extern "C" {
    pub fn bpf_cgroup_storage_assign(aux: *mut bpf_prog_aux, map: *mut bpf_map) -> c_int;
}
extern "C" {
    pub fn bpf_percpu_cgroup_storage_copy(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, flags: u64) -> c_int;
}
// Opportunistic check to see whether we have any BPF program attached
// Wrappers for __cgroup_bpf_run_filter_skb() guarded by cgroup_bpf_enabled.

// BPF_CGROUP_INET4_BIND and BPF_CGROUP_INET6_BIND can return extra flags
// via upper bits of return code. The only flag that is supported
// (at bit position 0) is to indicate CAP_NET_BIND_SERVICE capability check
// should be bypassed (BPF_RET_BIND_NO_CAP_NET_BIND_SERVICE).
//

// bind_flags |= BIND_NO_CAP_NET_BIND_SERVICE;	       \

// The SOCK_OPS"_SK" macro should be used when sock_ops->sk is not a
// fullsock and its parent fullsock cannot be traced by
// sk_to_full_sk().
//
// e.g. sock_ops->sk is a request_sock and it is under syncookie mode.
// Its listener-sk is not attached to the rsk_listener.
// In this case, the caller holds the listener-sk (unlocked),
// set its sock_ops->sk to req_sk, and call this SOCK_OPS"_SK" with
// the listener-sk such that the cgroup-bpf-progs of the
// listener-sk will be run.
//
// Regardless of syncookie mode or not,
// calling bpf_setsockopt on listener-sk will not make sense anyway,
// so passing 'sock_ops->sk == req_sk' to the bpf prog is appropriate here.
//

extern "C" {
    pub fn cgroup_bpf_link_attach(attr: *const bpf_attr, prog: *mut bpf_prog) -> c_int;
}

