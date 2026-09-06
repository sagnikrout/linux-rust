//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bpf-cgroup-defs.h
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

// Maximum number of concurrently attachable per-cgroup LSM hooks.
pub const CGROUP_LSM_NUM: c_int = 10;

pub const CGROUP_LSM_NUM: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cgroup_bpf_attach_type {
    CGROUP_BPF_ATTACH_TYPE_INVALID = -1,
    CGROUP_INET_INGRESS = 0,
    CGROUP_INET_EGRESS,
    CGROUP_INET_SOCK_CREATE,
    CGROUP_SOCK_OPS,
    CGROUP_DEVICE,
    CGROUP_INET4_BIND,
    CGROUP_INET6_BIND,
    CGROUP_INET4_CONNECT,
    CGROUP_INET6_CONNECT,
    CGROUP_UNIX_CONNECT,
    CGROUP_INET4_POST_BIND,
    CGROUP_INET6_POST_BIND,
    CGROUP_UDP4_SENDMSG,
    CGROUP_UDP6_SENDMSG,
    CGROUP_UNIX_SENDMSG,
    CGROUP_SYSCTL,
    CGROUP_UDP4_RECVMSG,
    CGROUP_UDP6_RECVMSG,
    CGROUP_UNIX_RECVMSG,
    CGROUP_GETSOCKOPT,
    CGROUP_SETSOCKOPT,
    CGROUP_INET4_GETPEERNAME,
    CGROUP_INET6_GETPEERNAME,
    CGROUP_UNIX_GETPEERNAME,
    CGROUP_INET4_GETSOCKNAME,
    CGROUP_INET6_GETSOCKNAME,
    CGROUP_UNIX_GETSOCKNAME,
    CGROUP_INET_SOCK_RELEASE,
    CGROUP_LSM_START,
    CGROUP_LSM_END = CGROUP_LSM_START + CGROUP_LSM_NUM - 1,
    MAX_CGROUP_BPF_ATTACH_TYPE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup_bpf {
// array of effective progs in this cgroup
    pub effective: [*mut bpf_prog_array __rcu; MAX_CGROUP_BPF_ATTACH_TYPE],
// attached progs to this cgroup and attach flags
// when flags == 0 or BPF_F_ALLOW_OVERRIDE the progs list will
// have either zero or one element
// when BPF_F_ALLOW_MULTI the list can have up to BPF_CGROUP_MAX_PROGS
//
    pub progs: [hlist_head; MAX_CGROUP_BPF_ATTACH_TYPE],
    pub flags: [u8; MAX_CGROUP_BPF_ATTACH_TYPE],
    pub revisions: [u64; MAX_CGROUP_BPF_ATTACH_TYPE],
// list of cgroup shared storages
    pub storages: list_head,
// temp storage for effective prog array used by prog_attach/detach
    pub inactive: *mut bpf_prog_array,
// reference counter used to detach bpf programs after cgroup removal
    pub refcnt: percpu_ref,
// cgroup_bpf is released using a work queue
    pub release_work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup_bpf {

