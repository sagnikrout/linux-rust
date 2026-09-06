//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/type_cast.c
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
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.

    struct {
    __uint(type, BPF_MAP_TYPE_TASK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, long);
    } enter_id SEC(".maps");
pub const IFNAMSIZ: c_int = 16;
    int ifindex, ingress_ifindex;
    char name[IFNAMSIZ];
    unsigned int inum;
    unsigned int meta_len, frag0_len, kskb_len, kskb2_len;
    SEC("?xdp")
#[no_mangle]
pub unsafe extern "C" fn md_xdp(ctx: *mut xdp_md) -> c_int {
    int md_xdp(struct xdp_md *ctx)
    {
    struct xdp_buff *kctx = bpf_cast_to_kern_ctx(ctx);
    struct net_device *dev;
    dev = kctx.rxq.dev;
    ifindex = dev.ifindex;
    inum = dev.nd_net.net.ns.inum;
    __builtin_memcpy(name, dev.name, IFNAMSIZ);
    ingress_ifindex = ctx.ingress_ifindex;
    return XDP_PASS;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn md_skb(skb: *mut __sk_buff) -> c_int {
    int md_skb(struct __sk_buff *skb)
    {
    struct sk_buff *kskb = bpf_cast_to_kern_ctx(skb);
    struct skb_shared_info *shared_info;
    struct sk_buff *kskb2;
    kskb_len = kskb.len;
// Simulate the following kernel macro:
// #define skb_shinfo(SKB) ((struct skb_shared_info *)(skb_end_pointer(SKB)))
//
    shared_info = bpf_core_cast(kskb.head + kskb.end, struct skb_shared_info);
    meta_len = shared_info.meta_len;
    frag0_len = shared_info.frag_list.len;
// kskb2 should be equal to kskb
    kskb2 = bpf_core_cast(kskb, typeof(*kskb2));
    kskb2_len = kskb2.len;
    return 0;
    }
    SEC("?tp_btf/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: untrusted_ptr, regs: *mut pt_regs, id: c_long) -> c_int {
    int BPF_PROG(untrusted_ptr, struct pt_regs *regs, long id)
    {
    struct task_struct *task, *task_dup;
    task = bpf_get_current_task_btf();
    task_dup = bpf_core_cast(task, struct task_struct);
    (void)bpf_task_storage_get(&enter_id, task_dup, 0, 0);
    return 0;
    }
    SEC("?tracepoint/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn kctx_u64(ctx: *mut c_void) -> c_int {
    int kctx_u64(void *ctx)
    {
    u64 *kctx = bpf_core_cast(ctx, u64);
    (void)kctx;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
