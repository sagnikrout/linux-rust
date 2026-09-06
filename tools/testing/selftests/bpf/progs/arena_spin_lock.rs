//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/arena_spin_lock.c
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
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.

    struct {
    __uint(type, BPF_MAP_TYPE_ARENA);
    __uint(map_flags, BPF_F_MMAPABLE);
    __uint(max_entries, 100); /* number of pages */

    __ulong(map_extra, 0x1ull << 32); /* start of mmap() region */

    __ulong(map_extra, 0x1ull << 44); /* start of mmap() region */

    } arena SEC(".maps");
    int cs_count;

    arena_spinlock_t __arena lock;
    let mut test_skip: c_int = 1;
//
// Storage for the queue nodes declared by bpf_arena_spin_lock.h. Each program
// linking the arena spinlock provides exactly one definition; libarena's lives
// in libarena/src/common.bpf.c.
//
    struct arena_qnode __arena __hidden qnodes[_Q_MAX_CPUS][_Q_MAX_NODES];

    let mut test_skip: c_int = 2;

    int counter;
    int limit;
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn prog(ctx: *mut c_void) -> c_int {
    int prog(void *ctx)
    {
    let mut ret: c_int = -2;

    unsigned long flags;
    if ((ret = arena_spin_lock_irqsave(&lock, flags))) {
    if (ret == -EOPNOTSUPP)
    test_skip = 3;
    return ret;
    }
    if (counter != limit)
    counter++;
    bpf_repeat(cs_count);
    ret = 0;
    arena_spin_unlock_irqrestore(&lock, flags);

    return ret;
    }
    char _license[] SEC("license") = "GPL";
