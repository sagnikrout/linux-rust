//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_sockmap_redir.c
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

    SEC(".maps") struct {
    __uint(type, BPF_MAP_TYPE_SOCKMAP);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
    } nop_map, sock_map;
    SEC(".maps") struct {
    __uint(type, BPF_MAP_TYPE_SOCKHASH);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
    } nop_hash, sock_hash;
    SEC(".maps") struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 2);
    __type(key, int);
    __type(value, unsigned int);
    } verdict_map;
// Set by user space
    int redirect_type;
    int redirect_flags;

    _Generic((__data),                                                     \
    struct __sk_buff * : bpf_sk_redirect_map,                     \
    struct sk_msg_md * : bpf_msg_redirect_map                     \
    )((__data), &sock_map, (__u32){0}, redirect_flags)

    _Generic((__data),                                                     \
    struct __sk_buff * : bpf_sk_redirect_hash,                    \
    struct sk_msg_md * : bpf_msg_redirect_hash                    \
    )((__data), &sock_hash, &(__u32){0}, redirect_flags)

    SEC("sk_" XSTR(__type))                                                        \
    int prog_ ## __type ## _verdict(__param data)                                  \
    {                                                                              \
    unsigned int *count;                                                   \
    int verdict;                                                           \
    \
    if (redirect_type == BPF_MAP_TYPE_SOCKMAP)                             \
    verdict = redirect_map(data);                                  \
    else if (redirect_type == BPF_MAP_TYPE_SOCKHASH)                       \
    verdict = redirect_hash(data);                                 \
    else                                                                   \
    verdict = redirect_type - __MAX_BPF_MAP_TYPE;                  \
    \
    count = bpf_map_lookup_elem(&verdict_map, &verdict);                   \
    if (count)                                                             \
    (*count)++;                                                    \
    \
    return verdict;                                                        \
    }
    DEFINE_PROG(skb, struct __sk_buff *);
    DEFINE_PROG(msg, struct sk_msg_md *);
    char _license[] SEC("license") = "GPL";
