//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bench_sockmap_prog.c
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

    let mut process_byte: c_long = 0;
    let mut verdict_dir: c_int = 0;
    let mut dropped: c_int = 0;
    let mut pkt_size: c_int = 0;
    struct {
    __uint(type, BPF_MAP_TYPE_SOCKMAP);
    __uint(max_entries, 20);
    __type(key, int);
    __type(value, int);
    } sock_map_rx SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_SOCKMAP);
    __uint(max_entries, 20);
    __type(key, int);
    __type(value, int);
    } sock_map_tx SEC(".maps");
    SEC("sk_skb/stream_parser")
#[no_mangle]
pub unsafe extern "C" fn prog_skb_parser(skb: *mut __sk_buff) -> c_int {
    int prog_skb_parser(struct __sk_buff *skb)
    {
    return pkt_size;
    }
    SEC("sk_skb/stream_verdict")
#[no_mangle]
pub unsafe extern "C" fn prog_skb_verdict(skb: *mut __sk_buff) -> c_int {
    int prog_skb_verdict(struct __sk_buff *skb)
    {
    let mut one: c_int = 1;
    let mut ret: c_int = bpf_sk_redirect_map(skb, &sock_map_rx, one, verdict_dir);
    if (ret == SK_DROP)
    dropped++;
    __sync_fetch_and_add(&process_byte, skb.len);
    return ret;
    }
    SEC("sk_skb/stream_verdict")
#[no_mangle]
pub unsafe extern "C" fn prog_skb_pass(skb: *mut __sk_buff) -> c_int {
    int prog_skb_pass(struct __sk_buff *skb)
    {
    __sync_fetch_and_add(&process_byte, skb.len);
    return SK_PASS;
    }
    SEC("sk_msg")
#[no_mangle]
pub unsafe extern "C" fn prog_skmsg_verdict(msg: *mut sk_msg_md) -> c_int {
    int prog_skmsg_verdict(struct sk_msg_md *msg)
    {
    let mut one: c_int = 1;
    __sync_fetch_and_add(&process_byte, msg.size);
    return bpf_msg_redirect_map(msg, &sock_map_tx, one, verdict_dir);
    }
    SEC("sk_msg")
#[no_mangle]
pub unsafe extern "C" fn prog_skmsg_pass(msg: *mut sk_msg_md) -> c_int {
    int prog_skmsg_pass(struct sk_msg_md *msg)
    {
    __sync_fetch_and_add(&process_byte, msg.size);
    return SK_PASS;
    }
    char _license[] SEC("license") = "GPL";
