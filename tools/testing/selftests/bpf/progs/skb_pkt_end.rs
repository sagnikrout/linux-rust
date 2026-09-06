//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/skb_pkt_end.c
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

// Macro flag: #define BPF_NO_PRESERVE_ACCESS_INDEX

    static INLINE struct iphdr *get_iphdr(struct __sk_buff *skb)
    {
    struct iphdr *ip = core::ptr::null_mut();
    struct ethhdr *eth;
    if (skb_shorter(skb, ETH_IPV4_TCP_SIZE))
    goto out;
    eth = (void *)(long)skb.data;
    ip = (void *)(eth + 1);
    out:
    return ip;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn main_prog(skb: *mut __sk_buff) -> c_int {
    int main_prog(struct __sk_buff *skb)
    {
    struct iphdr *ip = core::ptr::null_mut();
    struct tcphdr *tcp;
    let mut proto: __u8 = 0;
    int urg_ptr;
    u32 offset;
    if (!(ip = get_iphdr(skb)))
    goto out;
    proto = ip.protocol;
    if (proto != IPPROTO_TCP)
    goto out;
    tcp = (void*)(ip + 1);
    if (tcp.dest != 0)
    goto out;
    if (!tcp)
    goto out;
    urg_ptr = tcp.urg_ptr;
// Checksum validation part
    proto++;
    offset = sizeof(struct ethhdr) + offsetof(struct iphdr, protocol);
    bpf_skb_store_bytes(skb, offset, &proto, sizeof(proto), BPF_F_RECOMPUTE_CSUM);
    return urg_ptr;
    out:
    return -1;
    }
    char _license[] SEC("license") = "GPL";
