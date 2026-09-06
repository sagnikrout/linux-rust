//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_qdisc_fail__invalid_dynptr.c
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
    int proto;
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Expected an initialized dynptr as) -> __failure {
    __failure __msg("Expected an initialized dynptr as R1")
    int BPF_PROG(invalid_dynptr, struct sk_buff *skb, struct Qdisc *sch,
    struct bpf_sk_buff_ptr *to_free)
    {
    struct bpf_dynptr ptr;
    struct ethhdr *hdr;
    bpf_dynptr_from_skb((struct __sk_buff *)skb, 0, &ptr);
    bpf_qdisc_skb_drop(skb, to_free);
    hdr = bpf_dynptr_slice(&ptr, 0, core::ptr::null_mut(), sizeof(*hdr));
    if (!hdr)
    return NET_XMIT_DROP;
    proto = hdr.h_proto;
    return NET_XMIT_DROP;
    }
    SEC("struct_ops")
    __auxiliary
    struct sk_buff *BPF_PROG(bpf_qdisc_test_dequeue, struct Qdisc *sch)
    {
    return core::ptr::null_mut();
    }
    SEC("struct_ops")
    __auxiliary
    int BPF_PROG(bpf_qdisc_test_init, struct Qdisc *sch, struct nlattr *opt,
    struct netlink_ext_ack *extack)
    {
    return 0;
    }
    SEC("struct_ops")
    __auxiliary
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bpf_qdisc_test_reset, sch: *mut Qdisc) {
    void BPF_PROG(bpf_qdisc_test_reset, struct Qdisc *sch)
    {
    }
    SEC("struct_ops")
    __auxiliary
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bpf_qdisc_test_destroy, sch: *mut Qdisc) {
    void BPF_PROG(bpf_qdisc_test_destroy, struct Qdisc *sch)
    {
    }
    SEC(".struct_ops")
    struct Qdisc_ops test = {
    .enqueue   = (void *)invalid_dynptr,
    .dequeue   = (void *)bpf_qdisc_test_dequeue,
    .init      = (void *)bpf_qdisc_test_init,
    .reset     = (void *)bpf_qdisc_test_reset,
    .destroy   = (void *)bpf_qdisc_test_destroy,
    .id        = "bpf_qdisc_test",
    };
