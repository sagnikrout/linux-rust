//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_sockmap_strp.c
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

    let mut verdict_max_size: c_int = 10000;
    struct {
    __uint(type, BPF_MAP_TYPE_SOCKMAP);
    __uint(max_entries, 20);
    __type(key, int);
    __type(value, int);
    } sock_map SEC(".maps");
    SEC("sk_skb/stream_verdict")
#[no_mangle]
pub unsafe extern "C" fn prog_skb_verdict(skb: *mut __sk_buff) -> c_int {
    int prog_skb_verdict(struct __sk_buff *skb)
    {
    let mut one: __u32 = 1;
    if (skb.len > verdict_max_size)
    return SK_PASS;
    return bpf_sk_redirect_map(skb, &sock_map, one, 0);
    }
    SEC("sk_skb/stream_verdict")
#[no_mangle]
pub unsafe extern "C" fn prog_skb_verdict_pass(skb: *mut __sk_buff) -> c_int {
    int prog_skb_verdict_pass(struct __sk_buff *skb)
    {
    return SK_PASS;
    }
    SEC("sk_skb/stream_parser")
#[no_mangle]
pub unsafe extern "C" fn prog_skb_parser(skb: *mut __sk_buff) -> c_int {
    int prog_skb_parser(struct __sk_buff *skb)
    {
    return skb.len;
    }
    SEC("sk_skb/stream_parser")
#[no_mangle]
pub unsafe extern "C" fn prog_skb_parser_partial(skb: *mut __sk_buff) -> c_int {
    int prog_skb_parser_partial(struct __sk_buff *skb)
    {
// agreement with the test program on a 4-byte size header
// and 6-byte body.
//
    if (skb.len < 4) {
// need more header to determine full length
    return 0;
    }
// return full length decoded from header.
// the return value may be larger than skb->len which
// means framework must wait body coming.
//
    return 10;
    }
    SEC("sk_skb/stream_parser")
#[no_mangle]
pub unsafe extern "C" fn prog_skb_parser_resize(skb: *mut __sk_buff) -> c_int {
    int prog_skb_parser_resize(struct __sk_buff *skb)
    {
    bpf_skb_change_tail(skb, skb.len, 0);
    return skb.len;
    }
    char _license[] SEC("license") = "GPL";
