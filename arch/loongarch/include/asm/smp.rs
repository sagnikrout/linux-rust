//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/smp.h
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
// Author: Huacai Chen <chenhuacai@loongson.cn>
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_ops {
    pub (*init_ipi)(void): *mut c_void,
    pub action): *mut *mut void (send_ipi_single)(int cpu, unsigned int,
    pub action): *const *const *const void (send_ipi_mask)(struct cpumask mask, unsigned int,
}

extern "C" {
    pub fn loongson_smp_setup();
}
extern "C" {
    pub fn loongson_prepare_cpus(max_cpus: c_uint);
}
extern "C" {
    pub fn loongson_boot_secondary(cpu: c_int, idle: *mut task_struct);
}
extern "C" {
    pub fn loongson_init_secondary();
}
extern "C" {
    pub fn loongson_smp_finish();
}

extern "C" {
    pub fn loongson_cpu_disable() -> c_int;
}
extern "C" {
    pub fn loongson_cpu_die(cpu: c_uint);
}

extern "C" {
    pub fn vdso_smp_processor_id() -> return;
}

// Map from cpu id to sequential logical cpu number.  This will only
// not be idempotent when cpus failed to come on-line.

// The reverse map from sequential logical cpu number to cpu id.

pub const ACTION_BOOT_CPU: c_int = 0;
pub const ACTION_RESCHEDULE: c_int = 1;
pub const ACTION_CALL_FUNCTION: c_int = 2;
pub const ACTION_IRQ_WORK: c_int = 3;
pub const ACTION_CLEAR_VECTOR: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct secondary_data {
    pub task: c_ulong,
    pub stack: c_ulong,
    pub offset: c_ulong,
}

extern "C" {
    pub fn smpboot_entry() -> asmlinkage void;
}
extern "C" {
    pub fn start_secondary() -> asmlinkage void;
}
extern "C" {
    pub fn calculate_cpu_foreign_map();
}
//
// Generate IPI list text
//
extern "C" {
    pub fn show_ipi_list(p: *mut seq_file, prec: c_int);
}

extern "C" {
    pub fn loongson_cpu_disable() -> return;
}

pub const cpu_logical_map(cpu): c_int = 0;

