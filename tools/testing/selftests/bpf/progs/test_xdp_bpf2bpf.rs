//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_xdp_bpf2bpf.c
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
pub struct net_device {
// Structure does not need to contain all entries,
// as "preserve_access_index" will use BTF to fix this...
//
    pub ifindex: c_int,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_rxq_info {
// Structure does not need to contain all entries,
// as "preserve_access_index" will use BTF to fix this...
//
    pub dev: *mut net_device,
    pub queue_index: __u32,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_buff {
    pub data: *mut c_void,
    pub data_end: *mut c_void,
    pub data_meta: *mut c_void,
    pub data_hard_start: *mut c_void,
    pub handle: c_ulong,
    pub rxq: *mut xdp_rxq_info,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meta {
    pub ifindex: c_int,
    pub pkt_len: c_int,
}

    struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
    __type(key, int);
    __type(value, int);
    } perf_buf_map SEC(".maps");
    let mut test_result_fentry: __u64 = 0;
    SEC("fentry/FUNC")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: trace_on_entry, xdp: *mut xdp_buff) -> c_int {
    int BPF_PROG(trace_on_entry, struct xdp_buff *xdp)
    {
    struct meta meta;
    meta.ifindex = xdp.rxq.dev.ifindex;
    meta.pkt_len = bpf_xdp_get_buff_len((struct xdp_md *)xdp);
    bpf_xdp_output(xdp, &perf_buf_map,
    ((__u64) meta.pkt_len << 32) |
    BPF_F_CURRENT_CPU,
    &meta, sizeof(meta));
    test_result_fentry = xdp.rxq.dev.ifindex;
    return 0;
    }
    let mut test_result_fexit: __u64 = 0;
    SEC("fexit/FUNC")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: trace_on_exit, xdp: *mut xdp_buff, ret: c_int) -> c_int {
    int BPF_PROG(trace_on_exit, struct xdp_buff *xdp, int ret)
    {
    test_result_fexit = ret;
    return 0;
    }
