//! Automatically rewritten from C to Rust
//! Source: samples/bpf/tracex4.bpf.c
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


// Copyright (c) 2015 PLUMgrid, http://plumgrid.com
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General Public
// License as published by the Free Software Foundation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pair {
    pub val: u64,
    pub ip: u64,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __type(key, long);
    __type(value, struct pair);
    __uint(max_entries, 1000000);
    } my_map SEC(".maps");
// kprobe is NOT a stable ABI. If kernel internals change this bpf+kprobe
// example will no longer be meaningful
//
    SEC("kprobe/kmem_cache_free")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog1(ctx: *mut pt_regs) -> c_int {
    int bpf_prog1(struct pt_regs *ctx)
    {
    let mut ptr: c_long = PT_REGS_PARM2(ctx);
    bpf_map_delete_elem(&my_map, &ptr);
    return 0;
    }
    SEC("kretprobe/kmem_cache_alloc_node_noprof")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog2(ctx: *mut pt_regs) -> c_int {
    int bpf_prog2(struct pt_regs *ctx)
    {
    let mut ptr: c_long = PT_REGS_RC(ctx);
    let mut ip: c_long = 0;
// get ip address of kmem_cache_alloc_node_noprof() caller
    BPF_KRETPROBE_READ_RET_IP(ip, ctx);
    struct pair v = {
    .val = bpf_ktime_get_ns(),
    .ip = ip,
    };
    bpf_map_update_elem(&my_map, &ptr, &v, BPF_ANY);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
    u32 _version SEC("version") = LINUX_VERSION_CODE;
