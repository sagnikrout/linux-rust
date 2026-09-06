//! Automatically rewritten from C Header to Rust Module
//! Source: net/unix/af_unix.h
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

pub const UNIX_HASH_BITS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct unix_skb_parms {
    pub /: *mut *mut *mut pid pid; / skb credentials,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub /: *mut *mut *mut scm_fp_list fp; / Passed files,

    pub /: *mut *mut u32 secid; / Security ID,

    pub consumed: u32,
    pub __randomize_layout: },

// GC for SCM_RIGHTS
    pub receiver): *mut *mut void unix_add_edges(struct scm_fp_list fpl, struct unix_sock,
    pub fpl): *mut void unix_del_edges(struct scm_fp_list,
    pub receiver): *mut void unix_update_edges(struct unix_sock,
    pub fpl): *mut int unix_prepare_fpl(struct scm_fp_list,
    pub fpl): *mut void unix_destroy_fpl(struct scm_fp_list,
    pub fpl): *mut void unix_peek_fpl(struct scm_fp_list,
    pub user): *mut void unix_schedule_gc(struct user_struct,
// SOCK_DIAG
    pub sk): *mut long unix_inq_len(struct sock,
    pub sk): *mut long unix_outq_len(struct sock,
// sysctl

    pub net): *mut int unix_sysctl_register(struct net,
    pub net): *mut void unix_sysctl_unregister(struct net,

    pub 0: return,

// BPF SOCKMAP
    pub flags): *mut *mut *mut int __unix_dgram_recvmsg(struct sock sk, struct msghdr msg, size_t size, int,
    pub flags): *mut *mut *mut int __unix_stream_recvmsg(struct sock sk, struct msghdr msg, size_t size, int,

    pub unix_dgram_proto: extern struct proto,
    pub unix_stream_proto: extern struct proto,
    pub restore): *mut *mut *mut int unix_dgram_bpf_update_proto(struct sock sk, struct sk_psock psock, bool,
    pub restore): *mut *mut *mut int unix_stream_bpf_update_proto(struct sock sk, struct sk_psock psock, bool,
    pub unix_bpf_build_proto(void): void __init,

