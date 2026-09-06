//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/mptcp.h
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
// Multipath TCP
//
// Copyright (c) 2017 - 2019, Intel Corporation.
//

// MPTCP sk_buff extension data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_ext {
    pub data_seq: u64,
    pub subflow_seq: u32,
    pub data_len: u16,
    pub csum: __sum16,
    pub /: *mut *mut ); / end of flags group,
}

pub const MPTCPOPT_HMAC_LEN: c_int = 20;
pub const MPTCP_RM_IDS_MAX: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_rm_list {
    pub ids: [u8; MPTCP_RM_IDS_MAX],
    pub nr: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_addr_info {
    pub id: u8,
    pub family: sa_family_t,
    pub port: __be16,
    pub addr: in_addr,

    pub addr6: in6_addr,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_out_options {

    pub suboptions: u16,
    pub rm_list: mptcp_rm_list,
    pub join_id: u8,
    pub backup: u8,
    pub sndr_key: u64,
    pub rcvr_key: u64,
    pub data_seq: u64,
    pub subflow_seq: u32,
    pub data_len: u16,
    pub csum: __sum16,
}

pub const MPTCP_SCHED_NAME_MAX: c_int = 16;
pub const MPTCP_SCHED_MAX: c_int = 128;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_sched_ops {
    pub msk): *mut *mut int (get_send)(struct mptcp_sock,
    pub msk): *mut *mut int (get_retrans)(struct mptcp_sock,
    pub name: [c_char; MPTCP_SCHED_NAME_MAX],
    pub owner: *mut module,
    pub list: list_head,
    pub msk): *mut *mut void (init)(struct mptcp_sock,
    pub msk): *mut *mut void (release)(struct mptcp_sock,
    pub ____cacheline_aligned_in_smp: },
pub const MPTCP_PM_NAME_MAX: c_int = 16;
pub const MPTCP_PM_MAX: c_int = 128;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_pm_ops {
    pub name: [c_char; MPTCP_PM_NAME_MAX],
    pub owner: *mut module,
    pub list: list_head,
    pub msk): *mut *mut void (init)(struct mptcp_sock,
    pub msk): *mut *mut void (release)(struct mptcp_sock,
    pub ____cacheline_aligned_in_smp: },

    pub mptcp_init(void): c_void,
    pub tcp_sk(sk)->is_mptcp: return,
    pub tcp_rsk(req)->is_mptcp: return,
    pub tcp_rsk(req)->drop_req: return tcp_rsk(req)->is_mptcp &&,
    pub full_space): *const *const *const void mptcp_space(struct sock ssk, int space, int,
    pub opts): *mut *mut unsigned int size, struct mptcp_out_options,
    pub opts): *mut mptcp_out_options,
    pub opts): *mut mptcp_out_options,
    pub skb): *mut *mut bool mptcp_incoming_options(struct sock sk, struct sk_buff,
    pub opts): *mut mptcp_out_options,
    pub info): *mut *mut void mptcp_diag_fill_info(struct mptcp_sock msk, struct mptcp_info,
// move the skb extension owership, with the assumption that 'to' is
// newly allocated
//
    pub from->active_extensions: to->active_extensions =,
    pub from->extensions: to->extensions =,
    pub 0: from->active_extensions =,
    pub from_ext: *mut mptcp_ext,
    pub SKB_EXT_MPTCP): from_ext = skb_ext_find(from,,
    pub 1: from_ext->frozen =,
    pub from): skb_ext_copy(to,,
// MPTCP always clears the ext when adding it to the skb, so
// holes do not bother us here
//
    pub mptcp_ext))): !memcmp(from_ext, to_ext, sizeof(struct,
// check if skbs can be collapsed.
// MPTCP collapse is allowed if neither @to or @from carry an mptcp data
// mapping, or if the extension of @to is the same as @from.
// Collapsing is not possible if @to lacks an extension, but @from carries one.
//
    pub SKB_EXT_MPTCP)): skb_ext_find(from,,
    pub seq): *mut void mptcp_seq_show(struct seq_file,
    pub skb): *mut sk_buff,
    pub attach_listener): bool,
    pub skb): *const __be32 mptcp_get_reset_option(struct sk_buff,
    pub mptcp_get_reset_option(skb): return,
    pub htonl(0u): return,
    pub expired): *mut *mut void mptcp_active_detect_blackhole(struct sock sk, bool,

    pub false: return,
    pub false: return,
    pub false: return,
    pub false: return,
    pub false: return,
    pub true: return,
    pub true: return,
    pub /: *mut *mut return 0; / TCP fallback,
    pub NULL: return,
    pub }: *const *const static inline __be32 mptcp_reset_option(struct sk_buff skb) { return htonl(0u);,

    pub mptcpv6_init(void): c_int,
    pub mapped): *mut *mut void mptcpv6_handle_mapped(struct sock sk, bool,

    pub }: static inline int mptcpv6_init(void) { return 0;,

    pub sk): *mut *mut mptcp_sock bpf_mptcp_sock_from_subflow(sock,

    pub }: *mut *mut *mut static inline struct mptcp_sock bpf_mptcp_sock_from_subflow(struct sock sk) { return NULL;,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_sock {

