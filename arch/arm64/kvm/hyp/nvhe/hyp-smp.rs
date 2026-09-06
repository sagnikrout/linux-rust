//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kvm/hyp/nvhe/hyp-smp.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2020 - Google LLC
// Author: David Brazdil <dbrazdil@google.com>
//

//
// nVHE copy of data structures tracking available CPU cores.
// Only entries for CPUs that were online at KVM init are populated.
// Other CPUs should not be allowed to boot because their features were
// not checked against the finalized system capabilities.
//
    u64 __ro_after_init hyp_cpu_logical_map[NR_CPUS] = { [0 ... NR_CPUS-1] = INVALID_HWID };
#[no_mangle]
pub unsafe extern "C" fn cpu_logical_map(cpu: c_uint) -> u64 {
    u64 cpu_logical_map(unsigned int cpu)
    {
    BUG_ON(cpu >= ARRAY_SIZE(hyp_cpu_logical_map));
    return hyp_cpu_logical_map[cpu];
    }
    unsigned long __ro_after_init kvm_arm_hyp_percpu_base[NR_CPUS];
#[no_mangle]
pub unsafe extern "C" fn __hyp_per_cpu_offset(cpu: c_uint) -> c_ulong {
    unsigned long __hyp_per_cpu_offset(unsigned int cpu)
    {
    unsigned long *cpu_base_array;
    unsigned long this_cpu_base;
    unsigned long elf_base;
    BUG_ON(cpu >= ARRAY_SIZE(kvm_arm_hyp_percpu_base));
    cpu_base_array = (unsigned long *)&kvm_arm_hyp_percpu_base;
    this_cpu_base = kern_hyp_va(cpu_base_array[cpu]);
    elf_base = (unsigned long)&__per_cpu_start;
    return this_cpu_base - elf_base;
    }
