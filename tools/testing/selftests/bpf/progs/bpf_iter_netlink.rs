//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_iter_netlink.c
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
// Copyright (c) 2020 Facebook

    char _license[] SEC("license") = "GPL";
#[no_mangle]
pub unsafe extern "C" fn __attribute__(socket: *mut *mut (noinline)) struct inode SOCK_INODE(struct socket) -> static {
    static __attribute__((noinline)) struct inode *SOCK_INODE(struct socket *socket)
    {
    return &container_of(socket, struct socket_alloc, socket).vfs_inode;
    }
    SEC("iter/netlink")
#[no_mangle]
pub unsafe extern "C" fn dump_netlink(ctx: *mut bpf_iter__netlink) -> c_int {
    int dump_netlink(struct bpf_iter__netlink *ctx)
    {
    struct seq_file *seq = ctx.meta.seq;
    struct netlink_sock *nlk = ctx.sk;
    unsigned long group, ino;
    struct inode *inode;
    struct socket *sk;
    struct sock *s;
    if (nlk == (void *)0)
    return 0;
    if (ctx.meta.seq_num == 0)
    BPF_SEQ_PRINTF(seq, "sk               Eth Pid        Groups   "
    "Rmem     Wmem     Dump  Locks    Drops    "
    "Inode\n");
    s = &nlk.sk;
    BPF_SEQ_PRINTF(seq, "%pK %-3d ", s, s.sk_protocol);
    if (!nlk.groups)  {
    group = 0;
    } else {
// FIXME: temporary use bpf_probe_read_kernel here, needs
// verifier support to do direct access.
//
    bpf_probe_read_kernel(&group, sizeof(group), &nlk.groups[0]);
    }
    BPF_SEQ_PRINTF(seq, "%-10u %08x %-8d %-8d %-5d %-8d ",
    nlk.portid, (u32)group,
    s.sk_rmem_alloc.counter,
    s.sk_wmem_alloc.refs.counter - 1,
    nlk.cb_running, s.sk_refcnt.refs.counter);
    sk = s.sk_socket;
    if (!sk) {
    ino = 0;
    } else {
// FIXME: container_of inside SOCK_INODE has a forced
// type conversion, and direct access cannot be used
// with current verifier.
//
    inode = SOCK_INODE(sk);
    bpf_probe_read_kernel(&ino, sizeof(ino), &inode.i_ino);
    }
    BPF_SEQ_PRINTF(seq, "%-8u %-8lu\n", s.sk_drops.counter, ino);
    return 0;
    }
