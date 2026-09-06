//! Automatically rewritten from C to Rust
//! Source: samples/bpf/lwt_len_hist.bpf.c
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


// Copyright (c) 2016 Thomas Graf <tgraf@tgraf.ch>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General Public
// License as published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//

    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_HASH);
    __type(key, u64);
    __type(value, u64);
    __uint(pinning, LIBBPF_PIN_BY_NAME);
    __uint(max_entries, 1024);
    } lwt_len_hist_map SEC(".maps");
#[no_mangle]
unsafe extern "C" fn log2(v: c_uint) -> c_uint {
    static unsigned int log2(unsigned int v)
    {
    unsigned int r;
    unsigned int shift;
    r = (v > 0xFFFF) << 4; v >>= r;
    shift = (v > 0xFF) << 3; v >>= shift; r |= shift;
    shift = (v > 0xF) << 2; v >>= shift; r |= shift;
    shift = (v > 0x3) << 1; v >>= shift; r |= shift;
    r |= (v >> 1);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn log2l(v: c_ulong) -> c_uint {
    static unsigned int log2l(unsigned long v)
    {
    let mut hi: c_uint = v >> 32;
    if (hi)
    return log2(hi) + 32;
    else
    return log2(v);
    }
    SEC("len_hist")
#[no_mangle]
pub unsafe extern "C" fn do_len_hist(skb: *mut __sk_buff) -> c_int {
    int do_len_hist(struct __sk_buff *skb)
    {
    __u64 *value, key, init_val = 1;
    key = log2l(skb.len);
    value = bpf_map_lookup_elem(&lwt_len_hist_map, &key);
    if (value)
    __sync_fetch_and_add(value, 1);
    else
    bpf_map_update_elem(&lwt_len_hist_map, &key, &init_val, BPF_ANY);
    return BPF_OK;
    }
    char _license[] SEC("license") = "GPL";
