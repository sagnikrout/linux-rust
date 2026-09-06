//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_sockmap_mutate.c
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

    char _license[] SEC("license") = "GPL";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock {
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter__sockmap {
    union {
    pub sk: *mut sock,
}

    } __attribute__((preserve_access_index));
    struct {
    __uint(type, BPF_MAP_TYPE_SOCKHASH);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, int);
    } sockhash SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_SOCKMAP);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, int);
    } sockmap SEC(".maps");
    enum { CG_OK = 1 };
    let mut zero: c_int = 0;
#[no_mangle]
unsafe extern "C" fn test_sockmap_delete() -> __always_inline void {
    static __always_inline void test_sockmap_delete(void)
    {
    bpf_map_delete_elem(&sockmap, &zero);
    bpf_map_delete_elem(&sockhash, &zero);
    }
#[no_mangle]
unsafe extern "C" fn test_sockmap_update(sk: *mut c_void) -> __always_inline void {
    static __always_inline void test_sockmap_update(void *sk)
    {
    if (sk) {
    bpf_map_update_elem(&sockmap, &zero, sk, BPF_ANY);
    bpf_map_update_elem(&sockhash, &zero, sk, BPF_ANY);
    }
    }
#[no_mangle]
unsafe extern "C" fn test_sockmap_lookup_and_update() -> __always_inline void {
    static __always_inline void test_sockmap_lookup_and_update(void)
    {
    struct bpf_sock *sk = bpf_map_lookup_elem(&sockmap, &zero);
    if (sk) {
    test_sockmap_update(sk);
    bpf_sk_release(sk);
    }
    }
#[no_mangle]
unsafe extern "C" fn test_sockmap_mutate(sk: *mut c_void) -> __always_inline void {
    static __always_inline void test_sockmap_mutate(void *sk)
    {
    test_sockmap_delete();
    test_sockmap_update(sk);
    }
#[no_mangle]
unsafe extern "C" fn test_sockmap_lookup_and_mutate() -> __always_inline void {
    static __always_inline void test_sockmap_lookup_and_mutate(void)
    {
    test_sockmap_delete();
    test_sockmap_lookup_and_update();
    }
    SEC("action")
#[no_mangle]
pub unsafe extern "C" fn __msg(context": "cannot update sockmap in this) -> __failure {
    __failure __msg("cannot update sockmap in this context")
#[no_mangle]
pub unsafe extern "C" fn test_sched_act(skb: *mut __sk_buff) -> c_int {
    int test_sched_act(struct __sk_buff *skb)
    {
    test_sockmap_mutate(skb.sk);
    return 0;
    }
    SEC("classifier")
#[no_mangle]
pub unsafe extern "C" fn __msg(context": "cannot update sockmap in this) -> __failure {
    __failure __msg("cannot update sockmap in this context")
#[no_mangle]
pub unsafe extern "C" fn test_sched_cls(skb: *mut __sk_buff) -> c_int {
    int test_sched_cls(struct __sk_buff *skb)
    {
    test_sockmap_mutate(skb.sk);
    return 0;
    }
    SEC("flow_dissector")
#[no_mangle]
pub unsafe extern "C" fn __msg(context": "cannot update sockmap in this) -> __failure {
    __failure __msg("cannot update sockmap in this context")
#[no_mangle]
pub unsafe extern "C" fn test_flow_dissector_delete(__always_unused: *mut *mut __sk_buff skb) -> c_int {
    int test_flow_dissector_delete(struct __sk_buff *skb __always_unused)
    {
    test_sockmap_delete();
    return 0;
    }
    SEC("flow_dissector")
#[no_mangle]
pub unsafe extern "C" fn __msg(context": "cannot update sockmap in this) -> __failure {
    __failure __msg("cannot update sockmap in this context")
#[no_mangle]
pub unsafe extern "C" fn test_flow_dissector_update(__always_unused: *mut *mut __sk_buff skb) -> c_int {
    int test_flow_dissector_update(struct __sk_buff *skb __always_unused)
    {
    test_sockmap_lookup_and_update(); /* no access to skb.sk */
    return 0;
    }
    SEC("iter/sockmap")
    __success
#[no_mangle]
pub unsafe extern "C" fn test_trace_iter(ctx: *mut bpf_iter__sockmap) -> c_int {
    int test_trace_iter(struct bpf_iter__sockmap *ctx)
    {
    test_sockmap_mutate(ctx.sk);
    return 0;
    }
    SEC("raw_tp/kfree")
#[no_mangle]
pub unsafe extern "C" fn __msg(context": "cannot update sockmap in this) -> __failure {
    __failure __msg("cannot update sockmap in this context")
#[no_mangle]
pub unsafe extern "C" fn test_raw_tp_delete(__always_unused: *const *const void ctx) -> c_int {
    int test_raw_tp_delete(const void *ctx __always_unused)
    {
    test_sockmap_delete();
    return 0;
    }
    SEC("raw_tp/kfree")
#[no_mangle]
pub unsafe extern "C" fn __msg(context": "cannot update sockmap in this) -> __failure {
    __failure __msg("cannot update sockmap in this context")
#[no_mangle]
pub unsafe extern "C" fn test_raw_tp_update(__always_unused: *const *const void ctx) -> c_int {
    int test_raw_tp_update(const void *ctx __always_unused)
    {
    test_sockmap_lookup_and_update();
    return 0;
    }
    SEC("sk_lookup")
    __success
#[no_mangle]
pub unsafe extern "C" fn test_sk_lookup(ctx: *mut bpf_sk_lookup) -> c_int {
    int test_sk_lookup(struct bpf_sk_lookup *ctx)
    {
    test_sockmap_mutate(ctx.sk);
    return 0;
    }
    SEC("sk_reuseport")
    __success
#[no_mangle]
pub unsafe extern "C" fn test_sk_reuseport(ctx: *mut sk_reuseport_md) -> c_int {
    int test_sk_reuseport(struct sk_reuseport_md *ctx)
    {
    test_sockmap_mutate(ctx.sk);
    return 0;
    }
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn __msg(context": "cannot update sockmap in this) -> __failure {
    __failure __msg("cannot update sockmap in this context")
#[no_mangle]
pub unsafe extern "C" fn test_socket_filter(skb: *mut __sk_buff) -> c_int {
    int test_socket_filter(struct __sk_buff *skb)
    {
    test_sockmap_mutate(skb.sk);
    return 0;
    }
    SEC("sockops")
    __success
#[no_mangle]
pub unsafe extern "C" fn test_sockops_delete(__always_unused: *mut *mut bpf_sock_ops ctx) -> c_int {
    int test_sockops_delete(struct bpf_sock_ops *ctx __always_unused)
    {
    test_sockmap_delete();
    return CG_OK;
    }
    SEC("sockops")
#[no_mangle]
pub unsafe extern "C" fn __msg(context": "cannot update sockmap in this) -> __failure {
    __failure __msg("cannot update sockmap in this context")
#[no_mangle]
pub unsafe extern "C" fn test_sockops_update(ctx: *mut bpf_sock_ops) -> c_int {
    int test_sockops_update(struct bpf_sock_ops *ctx)
    {
    test_sockmap_update(ctx.sk);
    return CG_OK;
    }
    SEC("sockops")
    __success
#[no_mangle]
pub unsafe extern "C" fn test_sockops_update_dedicated(ctx: *mut bpf_sock_ops) -> c_int {
    int test_sockops_update_dedicated(struct bpf_sock_ops *ctx)
    {
    bpf_sock_map_update(ctx, &sockmap, &zero, BPF_ANY);
    bpf_sock_hash_update(ctx, &sockhash, &zero, BPF_ANY);
    return CG_OK;
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn __msg(context": "cannot update sockmap in this) -> __failure {
    __failure __msg("cannot update sockmap in this context")
#[no_mangle]
pub unsafe extern "C" fn test_xdp(__always_unused: *mut *mut xdp_md ctx) -> c_int {
    int test_xdp(struct xdp_md *ctx __always_unused)
    {
    test_sockmap_lookup_and_mutate();
    return XDP_PASS;
    }
