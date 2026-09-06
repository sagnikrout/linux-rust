//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_xdp_with_cpumap_helpers.c
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

pub const IFINDEX_LO: c_int = 1;
    struct {
    __uint(type, BPF_MAP_TYPE_CPUMAP);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(struct bpf_cpumap_val));
    __uint(max_entries, 4);
    } cpu_map SEC(".maps");
    let mut redirect_count: __u32 = 0;
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_redir_prog(ctx: *mut xdp_md) -> c_int {
    int xdp_redir_prog(struct xdp_md *ctx)
    {
    return bpf_redirect_map(&cpu_map, 0, 0);
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_dummy_prog(ctx: *mut xdp_md) -> c_int {
    int xdp_dummy_prog(struct xdp_md *ctx)
    {
    return XDP_PASS;
    }
    SEC("xdp/cpumap")
#[no_mangle]
pub unsafe extern "C" fn xdp_dummy_cm(ctx: *mut xdp_md) -> c_int {
    int xdp_dummy_cm(struct xdp_md *ctx)
    {
    if (bpf_get_smp_processor_id() == 0)
    redirect_count++;
    if (ctx.ingress_ifindex == IFINDEX_LO)
    return XDP_DROP;
    return XDP_PASS;
    }
    SEC("xdp.frags/cpumap")
#[no_mangle]
pub unsafe extern "C" fn xdp_dummy_cm_frags(ctx: *mut xdp_md) -> c_int {
    int xdp_dummy_cm_frags(struct xdp_md *ctx)
    {
    return XDP_PASS;
    }
    char _license[] SEC("license") = "GPL";
