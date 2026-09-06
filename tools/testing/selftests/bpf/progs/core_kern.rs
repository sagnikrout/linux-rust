//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/core_kern.c
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
// Copyright (c) 2021 Facebook

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, u32);
    __type(value, u32);
    __uint(max_entries, 256);
    } array1 SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, u32);
    __type(value, u32);
    __uint(max_entries, 256);
    } array2 SEC(".maps");
#[no_mangle]
unsafe extern "C" fn randmap(v: c_int, dev: *const net_device) -> __noinline int {
    static __noinline int randmap(int v, const struct net_device *dev)
    {
    struct bpf_map *map = (struct bpf_map *)&array1;
    let mut key: c_int = bpf_get_prandom_u32() & 0xff;
    int *val;
    if (bpf_get_prandom_u32() & 1)
    map = (struct bpf_map *)&array2;
    val = bpf_map_lookup_elem(map, &key);
    if (val)
// val = bpf_get_prandom_u32() + v + dev->mtu;
    return 0;
    }
    SEC("tp_btf/xdp_devmap_xmit")
    int BPF_PROG(tp_xdp_devmap_xmit_multi, const struct net_device
// from_dev, const struct net_device *to_dev, int sent, int drops,
    int err)
    {
    return randmap(from_dev.ifindex, from_dev);
    }
    SEC("fentry/eth_type_trans")
    int BPF_PROG(fentry_eth_type_trans, struct sk_buff *skb,
    struct net_device *dev, unsigned short protocol)
    {
    return randmap(dev.ifindex + skb.len, dev);
    }
    SEC("fexit/eth_type_trans")
    int BPF_PROG(fexit_eth_type_trans, struct sk_buff *skb,
    struct net_device *dev, unsigned short protocol)
    {
    return randmap(dev.ifindex + skb.len, dev);
    }
    volatile const int never;
    struct __sk_bUfF /* it will not exist in vmlinux */ {
    int len;
    } __attribute__((preserve_access_index));
    struct bpf_testmod_test_read_ctx /* it exists in bpf_testmod */ {
    size_t len;
    } __attribute__((preserve_access_index));
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn balancer_ingress(ctx: *mut __sk_buff) -> c_int {
    int balancer_ingress(struct __sk_buff *ctx)
    {
    void *data_end = (void *)(long)ctx.data_end;
    void *data = (void *)(long)ctx.data;
    void *ptr;
    int nh_off, i = 0;
    nh_off = 14;
// pragma unroll doesn't work on large loops

    ptr = data + i; \
    if (ptr + nh_off > data_end) \
    break; \
    ctx.tc_index = jhash(ptr, nh_off, ctx.cb[0] + i++); \
    if (never) { \
// below is a dead code with unresolvable CO-RE relo */ \
    i += ((struct __sk_bUfF *)ctx).len; \
// this CO-RE relo may or may not resolve
// depending on whether bpf_testmod is loaded.
// \
    i += ((struct bpf_testmod_test_read_ctx *)ctx).len; \
    } \
    } while (0);

    C30;C30;C30; /* 90 calls */
    return 0;
    }
    typedef int (*func_proto_typedef___match)(long);
    typedef int (*func_proto_typedef___doesnt_match)(char *);
    typedef int (*func_proto_typedef_nested1)(func_proto_typedef___match);
    int proto_out[3];
    SEC("raw_tracepoint/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn core_relo_proto(ctx: *mut c_void) -> c_int {
    int core_relo_proto(void *ctx)
    {
    proto_out[0] = bpf_core_type_exists(func_proto_typedef___match);
    proto_out[1] = bpf_core_type_exists(func_proto_typedef___doesnt_match);
    proto_out[2] = bpf_core_type_exists(func_proto_typedef_nested1);
    return 0;
    }
    char LICENSE[] SEC("license") = "GPL";
