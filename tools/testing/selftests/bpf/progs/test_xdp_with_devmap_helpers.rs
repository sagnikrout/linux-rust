//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_xdp_with_devmap_helpers.c
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

    struct {
    __uint(type, BPF_MAP_TYPE_DEVMAP);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(struct bpf_devmap_val));
    __uint(max_entries, 4);
    } dm_ports SEC(".maps");
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_redir_prog(ctx: *mut xdp_md) -> c_int {
    int xdp_redir_prog(struct xdp_md *ctx)
    {
    return bpf_redirect_map(&dm_ports, 0, 0);
    }
// invalid program on DEVMAP entry;
// SEC name means expected attach type not set
//
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_dummy_prog(ctx: *mut xdp_md) -> c_int {
    int xdp_dummy_prog(struct xdp_md *ctx)
    {
    return XDP_PASS;
    }
// valid program on DEVMAP entry via SEC name;
// has access to egress and ingress ifindex
//
    SEC("xdp/devmap")
#[no_mangle]
pub unsafe extern "C" fn xdp_dummy_dm(ctx: *mut xdp_md) -> c_int {
    int xdp_dummy_dm(struct xdp_md *ctx)
    {
    char fmt[] = "devmap redirect: dev %u . dev %u len %u\n";
    void *data_end = (void *)(long)ctx.data_end;
    void *data = (void *)(long)ctx.data;
    let mut len: c_uint = data_end - data;
    bpf_trace_printk(fmt, sizeof(fmt),
    ctx.ingress_ifindex, ctx.egress_ifindex, len);
    return XDP_PASS;
    }
    SEC("xdp.frags/devmap")
#[no_mangle]
pub unsafe extern "C" fn xdp_dummy_dm_frags(ctx: *mut xdp_md) -> c_int {
    int xdp_dummy_dm_frags(struct xdp_md *ctx)
    {
    return XDP_PASS;
    }
    char _license[] SEC("license") = "GPL";
