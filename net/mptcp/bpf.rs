//! Automatically rewritten from C to Rust
//! Source: net/mptcp/bpf.c
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
// Multipath TCP
//
// Copyright (c) 2020, Tessares SA.
// Copyright (c) 2022, SUSE.
//
// Author: Nicolas Rybowski <nicolas.rybowski@tessares.net>
//

    struct mptcp_sock *bpf_mptcp_sock_from_subflow(struct sock *sk)
    {
    if (sk && sk_fullsock(sk) && sk_is_tcp(sk) && sk_is_mptcp(sk))
    return mptcp_sk(mptcp_subflow_ctx(sk).conn);
    return core::ptr::null_mut();
    }
    BTF_SET8_START(bpf_mptcp_fmodret_ids)
    BTF_ID_FLAGS(func, update_socket_protocol)
    BTF_SET8_END(bpf_mptcp_fmodret_ids)
    static const struct btf_kfunc_id_set bpf_mptcp_fmodret_set = {
    .owner = THIS_MODULE,
    .set   = &bpf_mptcp_fmodret_ids,
    };
#[no_mangle]
unsafe extern "C" fn bpf_mptcp_kfunc_init() -> int __init {
    static int __init bpf_mptcp_kfunc_init(void)
    {
    return register_btf_fmodret_id_set(&bpf_mptcp_fmodret_set);
    }
    late_initcall(bpf_mptcp_kfunc_init);
