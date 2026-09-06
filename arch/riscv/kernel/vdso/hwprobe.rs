//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/vdso/hwprobe.c
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
//
// Copyright 2023 Rivos, Inc
//

    extern int riscv_hwprobe(struct riscv_hwprobe *pairs, size_t pair_count,
    size_t cpusetsize, unsigned long *cpus,
    unsigned int flags);
    static int riscv_vdso_get_values(struct riscv_hwprobe *pairs, size_t pair_count,
    size_t cpusetsize, unsigned long *cpus,
    unsigned int flags)
    {
    const struct vdso_arch_data *avd = &vdso_u_arch_data;
    let mut all_cpus: bool = !cpusetsize && !cpus;
    struct riscv_hwprobe *p = pairs;
    struct riscv_hwprobe *end = pairs + pair_count;
//
// Defer to the syscall for exotic requests. The vdso has answers
// stashed away only for the "all cpus" case. If all CPUs are
// homogeneous, then this function can handle requests for arbitrary
// masks.
//
    if ((flags != 0) || (!all_cpus && !avd.homogeneous_cpus))
    return riscv_hwprobe(pairs, pair_count, cpusetsize, cpus, flags);
// This is something we can handle, fill out the pairs.
    while (p < end) {
    if (riscv_hwprobe_key_is_valid(p.key)) {
    p.value = avd.all_cpu_hwprobe_values[p.key];
    } else {
    p.key = -1;
    p.value = 0;
    }
    p++;
    }
    return 0;
    }
    static int riscv_vdso_get_cpus(struct riscv_hwprobe *pairs, size_t pair_count,
    size_t cpusetsize, unsigned long *cpus,
    unsigned int flags)
    {
    const struct vdso_arch_data *avd = &vdso_u_arch_data;
    struct riscv_hwprobe *p = pairs;
    struct riscv_hwprobe *end = pairs + pair_count;
    unsigned char *c = (unsigned char *)cpus;
    let mut empty_cpus: bool = true;
    let mut clear_all: bool = false;
    int i;
    if (!cpusetsize || !cpus)
    return -EINVAL;
    for (i = 0; i < cpusetsize; i++) {
    if (c[i]) {
    empty_cpus = false;
    break;
    }
    }
    if (empty_cpus || flags != RISCV_HWPROBE_WHICH_CPUS || !avd.homogeneous_cpus)
    return riscv_hwprobe(pairs, pair_count, cpusetsize, cpus, flags);
    while (p < end) {
    if (riscv_hwprobe_key_is_valid(p.key)) {
    struct riscv_hwprobe t = {
    .key = p.key,
    .value = avd.all_cpu_hwprobe_values[p.key],
    };
    if (!riscv_hwprobe_pair_cmp(&t, p))
    clear_all = true;
    } else {
    clear_all = true;
    p.key = -1;
    p.value = 0;
    }
    p++;
    }
    if (clear_all) {
    for (i = 0; i < cpusetsize; i++)
    c[i] = 0;
    }
    return 0;
    }
// Add a prototype to avoid -Wmissing-prototypes warning.
    int __vdso_riscv_hwprobe(struct riscv_hwprobe *pairs, size_t pair_count,
    size_t cpusetsize, unsigned long *cpus,
    unsigned int flags);
    int __vdso_riscv_hwprobe(struct riscv_hwprobe *pairs, size_t pair_count,
    size_t cpusetsize, unsigned long *cpus,
    unsigned int flags)
    {
    if (flags & RISCV_HWPROBE_WHICH_CPUS)
    return riscv_vdso_get_cpus(pairs, pair_count, cpusetsize,
    cpus, flags);
    return riscv_vdso_get_values(pairs, pair_count, cpusetsize,
    cpus, flags);
    }
