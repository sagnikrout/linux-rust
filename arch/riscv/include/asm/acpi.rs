//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/acpi.h
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
// Copyright (C) 2013-2014, Linaro Ltd.
// Author: Al Stone <al.stone@linaro.org>
// Author: Graeme Gregory <graeme.gregory@linaro.org>
// Author: Hanjun Guo <hanjun.guo@linaro.org>
//
// Copyright (C) 2021-2023, Ventana Micro Systems Inc.
// Author: Sunil V L <sunilvl@ventanamicro.com>
//

// Basic configuration for ACPI

pub type phys_cpuid_t = u64;

// ACPI table mapping after acpi_permanent_mmap is set

//
// The ACPI processor driver for ACPI core code needs this macro
// to find out whether this cpu was already mapped (mapping from CPU hardware
// ID to CPU logical ID) or not.
//

//
// Since MADT must provide at least one RINTC structure, the
// CPU will be always available in MADT on RISC-V.
//
extern "C" {
    pub fn acpi_init_rintc_map();
}
extern "C" {
    pub fn acpi_get_riscv_gsi_handle(gsi: u32) -> acpi_handle;
}
//
// RISC-V Functional Fixed Hardware Specification Version v1.0.1,
// Chapter 3.1.2, Table 4: Arch. Context Lost Flags
//

extern "C" {
    pub fn acpi_map_cpus_to_nodes();
}

