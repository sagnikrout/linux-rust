//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_xdp_attach_fail.c
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
// Copyright Leon Hwang

pub const ERRMSG_LEN: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_errmsg {
    pub msg: [c_char; ERRMSG_LEN],
}

    struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
    __type(key, int);
    __type(value, int);
    } xdp_errmsg_pb SEC(".maps");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_attach_error_ctx {
    pub unused: c_ulong,
//
// bpf does not support tracepoint __data_loc directly.
//
// Actually, this field is a 32 bit integer whose value encodes
// information on where to find the actual data. The first 2 bytes is
// the size of the data. The last 2 bytes is the offset from the start
// of the tracepoint struct where the data begins.
// -- https://github.com/iovisor/bpftrace/pull/1542
//
    pub msg: __u32 msg; // __data_loc char[],
}

//
// Catch the error message at the tracepoint.
//
    SEC("tp/xdp/bpf_xdp_link_attach_failed")
#[no_mangle]
pub unsafe extern "C" fn tp__xdp__bpf_xdp_link_attach_failed(ctx: *mut xdp_attach_error_ctx) -> c_int {
    int tp__xdp__bpf_xdp_link_attach_failed(struct xdp_attach_error_ctx *ctx)
    {
    char *msg = (void *)(__u64) ((void *) ctx + (__u16) ctx.msg);
    let mut errmsg: xdp_errmsg = {};
    bpf_probe_read_kernel_str(&errmsg.msg, ERRMSG_LEN, msg);
    bpf_perf_event_output(ctx, &xdp_errmsg_pb, BPF_F_CURRENT_CPU, &errmsg,
    ERRMSG_LEN);
    return 0;
    }
//
// Reuse the XDP program in xdp_dummy.c.
//
    char LICENSE[] SEC("license") = "GPL";
