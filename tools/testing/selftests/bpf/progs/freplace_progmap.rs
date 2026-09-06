//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/freplace_progmap.c
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
    __uint(type, BPF_MAP_TYPE_CPUMAP);
    __type(key, __u32);
    __type(value, struct bpf_cpumap_val);
    __uint(max_entries, 1);
    } cpu_map SEC(".maps");
    SEC("xdp/cpumap")
#[no_mangle]
pub unsafe extern "C" fn xdp_drop_prog(ctx: *mut xdp_md) -> c_int {
    int xdp_drop_prog(struct xdp_md *ctx)
    {
    return XDP_DROP;
    }
    SEC("freplace")
#[no_mangle]
pub unsafe extern "C" fn xdp_cpumap_prog(ctx: *mut xdp_md) -> c_int {
    int xdp_cpumap_prog(struct xdp_md *ctx)
    {
    return bpf_redirect_map(&cpu_map, 0, XDP_PASS);
    }
    char _license[] SEC("license") = "GPL";
