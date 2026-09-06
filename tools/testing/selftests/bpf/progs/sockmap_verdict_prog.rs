//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/sockmap_verdict_prog.c
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
    struct {
    __uint(type, BPF_MAP_TYPE_SOCKMAP);
    __uint(max_entries, 20);
    __type(key, int);
    __type(value, int);
    } sock_map_msg SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 20);
    __type(key, int);
    __type(value, int);
    } sock_map_break SEC(".maps");
    SEC("sk_skb2")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog2(skb: *mut __sk_buff) -> c_int {
    int bpf_prog2(struct __sk_buff *skb)
    {
    void *data_end = (void *)(long) skb.data_end;
    void *data = (void *)(long) skb.data;
    let mut lport: __u32 = skb.local_port;
    let mut rport: __u32 = skb.remote_port;
    __u8 *d = data;
    __u8 sk, map;
    __sink(lport);
    __sink(rport);
    if (data + 8 > data_end) {
    if (bpf_skb_pull_data(skb, 8))
    return SK_DROP;
    data = (void *)(long)skb.data;
    data_end = (void *)(long)skb.data_end;
    if (data + 8 > data_end)
    return SK_DROP;
    d = data;
    }
    map = d[0];
    sk = d[1];
    d[0] = 0xd;
    d[1] = 0xe;
    d[2] = 0xa;
    d[3] = 0xd;
    d[4] = 0xb;
    d[5] = 0xe;
    d[6] = 0xe;
    d[7] = 0xf;
    if (!map)
    return bpf_sk_redirect_map(skb, &sock_map_rx, sk, 0);
    return bpf_sk_redirect_map(skb, &sock_map_tx, sk, 0);
    }
    char _license[] SEC("license") = "GPL";
