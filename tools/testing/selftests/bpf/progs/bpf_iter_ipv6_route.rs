//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_iter_ipv6_route.c
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
    extern bool CONFIG_IPV6_SUBTREES __kconfig __weak;
    SEC("iter/ipv6_route")
#[no_mangle]
pub unsafe extern "C" fn dump_ipv6_route(ctx: *mut bpf_iter__ipv6_route) -> c_int {
    int dump_ipv6_route(struct bpf_iter__ipv6_route *ctx)
    {
    struct seq_file *seq = ctx.meta.seq;
    struct fib6_info *rt = ctx.rt;
    const struct net_device *dev;
    struct fib6_nh *fib6_nh;
    unsigned int flags;
    struct nexthop *nh;
    if (rt == (void *)0)
    return 0;
    fib6_nh = &rt.fib6_nh[0];
    flags = rt.fib6_flags;
// FIXME: nexthop_is_multipath is not handled here.
    nh = rt.nh;
    if (rt.nh)
    fib6_nh = &nh.nh_info.fib6_nh;
    BPF_SEQ_PRINTF(seq, "%pi6 %02x ", &rt.fib6_dst.addr, rt.fib6_dst.plen);
    if (CONFIG_IPV6_SUBTREES)
    BPF_SEQ_PRINTF(seq, "%pi6 %02x ", &rt.fib6_src.addr,
    rt.fib6_src.plen);
    else
    BPF_SEQ_PRINTF(seq, "00000000000000000000000000000000 00 ");
    if (fib6_nh.fib_nh_gw_family) {
    flags |= RTF_GATEWAY;
    BPF_SEQ_PRINTF(seq, "%pi6 ", &fib6_nh.fib_nh_gw6);
    } else {
    BPF_SEQ_PRINTF(seq, "00000000000000000000000000000000 ");
    }
    dev = fib6_nh.fib_nh_dev;
    if (dev)
    BPF_SEQ_PRINTF(seq, "%08x %08x %08x %08x %8s\n", rt.fib6_metric,
    rt.fib6_ref.refs.counter, 0, flags, dev.name);
    else
    BPF_SEQ_PRINTF(seq, "%08x %08x %08x %08x\n", rt.fib6_metric,
    rt.fib6_ref.refs.counter, 0, flags);
    return 0;
    }
