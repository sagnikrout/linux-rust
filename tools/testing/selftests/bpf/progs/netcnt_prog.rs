//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/netcnt_prog.c
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

pub const REFRESH_TIME_NS: c_int = 100000000;
pub const NS_PER_SEC: c_int = 1000000000;
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE);
    __type(key, struct bpf_cgroup_storage_key);
    __type(value, union percpu_net_cnt);
    } percpu_netcnt SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_CGROUP_STORAGE);
    __type(key, struct bpf_cgroup_storage_key);
    __type(value, union net_cnt);
    } netcnt SEC(".maps");
    SEC("cgroup/skb")
#[no_mangle]
pub unsafe extern "C" fn bpf_nextcnt(skb: *mut __sk_buff) -> c_int {
    int bpf_nextcnt(struct __sk_buff *skb)
    {
    union percpu_net_cnt *percpu_cnt;
    union net_cnt *cnt;
    __u64 ts, dt;
    int ret;
    cnt = bpf_get_local_storage(&netcnt, 0);
    percpu_cnt = bpf_get_local_storage(&percpu_netcnt, 0);
    percpu_cnt.packets++;
    percpu_cnt.bytes += skb.len;
    if (percpu_cnt.packets > MAX_PERCPU_PACKETS) {
    __sync_fetch_and_add(&cnt.packets,
    percpu_cnt.packets);
    percpu_cnt.packets = 0;
    __sync_fetch_and_add(&cnt.bytes,
    percpu_cnt.bytes);
    percpu_cnt.bytes = 0;
    }
    ts = bpf_ktime_get_ns();
    dt = ts - percpu_cnt.prev_ts;
    dt *= MAX_BPS;
    dt /= NS_PER_SEC;
    if (cnt.bytes + percpu_cnt.bytes - percpu_cnt.prev_bytes < dt)
    ret = 1;
    else
    ret = 0;
    if (dt > REFRESH_TIME_NS) {
    percpu_cnt.prev_ts = ts;
    percpu_cnt.prev_packets = cnt.packets;
    percpu_cnt.prev_bytes = cnt.bytes;
    }
    return !!ret;
    }
    char _license[] SEC("license") = "GPL";
