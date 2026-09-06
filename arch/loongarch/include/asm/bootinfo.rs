//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/bootinfo.h
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
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

extern "C" {
    pub fn init_environ();
}
extern "C" {
    pub fn memblock_init();
}
extern "C" {
    pub fn platform_init();
}
extern "C" {
    pub fn init_numa_memory() -> int __init;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongson_board_info {
    pub bios_size: c_int,
    pub bios_vendor: *const c_char,
    pub bios_version: *const c_char,
    pub bios_release_date: *const c_char,
    pub board_name: *const c_char,
    pub board_vendor: *const c_char,
}

//
// The "core" of cores_per_node and cores_per_package stands for a
// logical core, which means in a SMT system it stands for a thread.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongson_system_configuration {
    pub nr_cpus: c_int,
    pub nr_nodes: c_int,
    pub boot_cpu_id: c_int,
    pub cores_per_node: c_int,
    pub cores_per_package: c_int,
    pub cores_io_master: [c_ulong; NR_WORDS],
    pub suspend_addr: c_ulong,
    pub cpuname: *const c_char,
}

extern "C" {
    pub fn test_bit(_arg: cpu, _arg: loongson_sysconf.cores_io_master) -> return;
}
