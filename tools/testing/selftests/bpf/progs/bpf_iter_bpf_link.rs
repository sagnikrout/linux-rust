//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_iter_bpf_link.c
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
// Copyright (c) 2022 Red Hat, Inc.

    char _license[] SEC("license") = "GPL";
    SEC("iter/bpf_link")
#[no_mangle]
pub unsafe extern "C" fn dump_bpf_link(ctx: *mut bpf_iter__bpf_link) -> c_int {
    int dump_bpf_link(struct bpf_iter__bpf_link *ctx)
    {
    struct seq_file *seq = ctx.meta.seq;
    struct bpf_link *link = ctx.link;
    int link_id;
    if (!link)
    return 0;
    link_id = link.id;
    bpf_seq_write(seq, &link_id, sizeof(link_id));
    return 0;
    }
