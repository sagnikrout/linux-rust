//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_iter_unix.c
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
// Copyright Amazon.com Inc. or its affiliates.

    char _license[] SEC("license") = "GPL";
    SEC(".maps") struct {
    __uint(type, BPF_MAP_TYPE_SOCKMAP);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
    } sockmap;
#[no_mangle]
unsafe extern "C" fn sock_i_ino(sk: *const sock) -> c_long {
    static long sock_i_ino(const struct sock *sk)
    {
    const struct socket *sk_socket = sk.sk_socket;
    const struct inode *inode;
    unsigned long ino;
    if (!sk_socket)
    return 0;
    inode = &container_of(sk_socket, struct socket_alloc, socket).vfs_inode;
    bpf_probe_read_kernel(&ino, sizeof(ino), &inode.i_ino);
    return ino;
    }
    SEC("iter/unix")
#[no_mangle]
pub unsafe extern "C" fn dump_unix(ctx: *mut bpf_iter__unix) -> c_int {
    int dump_unix(struct bpf_iter__unix *ctx)
    {
    struct unix_sock *unix_sk = ctx.unix_sk;
    struct sock *sk = (struct sock *)unix_sk;
    struct seq_file *seq;
    __u32 seq_num;
    if (!unix_sk)
    return 0;
    seq = ctx.meta.seq;
    seq_num = ctx.meta.seq_num;
    if (seq_num == 0)
    BPF_SEQ_PRINTF(seq, "Num               RefCount Protocol Flags    Type St    Inode Path\n");
    BPF_SEQ_PRINTF(seq, "%pK: %08X %08X %08X %04X %02X %8lu",
    unix_sk,
    sk.sk_refcnt.refs.counter,
    0,
    sk.sk_state == TCP_LISTEN ? __SO_ACCEPTCON : 0,
    sk.sk_type,
    sk.sk_socket ?
    (sk.sk_state == TCP_ESTABLISHED ? SS_CONNECTED : SS_UNCONNECTED) :
    (sk.sk_state == TCP_ESTABLISHED ? SS_CONNECTING : SS_DISCONNECTING),
    sock_i_ino(sk));
    if (unix_sk.addr) {
    if (unix_sk.addr.name.sun_path[0]) {
    BPF_SEQ_PRINTF(seq, " %s", unix_sk.addr.name.sun_path);
    } else {
// The name of the abstract UNIX domain socket starts
// with '\0' and can contain '\0'.  The null bytes
// should be escaped as done in unix_seq_show().
//
    __u64 i, len;
    len = unix_sk.addr.len - sizeof(short);
    BPF_SEQ_PRINTF(seq, " @");
    for (i = 1; i < len; i++) {
// unix_validate_addr() tests this upper bound.
    if (i >= sizeof(struct sockaddr_un))
    break;
    BPF_SEQ_PRINTF(seq, "%c",
    unix_sk.addr.name.sun_path[i] ?:
    '@');
    }
    }
    }
    BPF_SEQ_PRINTF(seq, "\n");
// Test for deadlock.
    bpf_map_update_elem(&sockmap, &(int){0}, sk, 0);
    return 0;
    }
