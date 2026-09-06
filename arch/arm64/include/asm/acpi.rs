//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/acpi.h
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

// Macros for consistency checks of the GICC subtable of MADT
//
// MADT GICC minimum length refers to the MADT GICC structure table length as
// defined in the earliest ACPI version supported on arm64, ie ACPI 5.1.
//
// The efficiency_class member was added to the
// struct acpi_madt_generic_interrupt to represent the MADT GICC structure
// "Processor Power Efficiency Class" field, added in ACPI 6.0 whose offset
// is therefore used to delimit the MADT GICC structure minimum length
// appropriately.
//

//
// Arm® Functional Fixed Hardware Specification Version 1.2.
// Table 2: Arm Architecture context loss flags
//

// Basic configuration for ACPI

extern "C" {
    pub fn __acpi_get_mem_attribute(addr: phys_addr_t) -> pgprot_t;
}
// ACPI table mapping after acpi_permanent_mmap is set

pub type phys_cpuid_t = u64;

//
// The ACPI processor driver for ACPI core code needs this macro
// to find out this cpu was already mapped (mapping from CPU hardware
// ID to CPU logical ID) or not.
//

//
// It's used from ACPI core in kdump to boot UP system with SMP kernel,
// with this check the ACPI core will not override the CPU index
// obtained from GICC with 0 and not print some error message as well.
// Since MADT must provide at least one GICC structure for GIC
// initialization, CPU will be always available in MADT on ARM64.
//
extern "C" {
    pub fn get_cpu_for_acpi_id(uid: u32) -> c_int;
}
extern "C" {
    pub fn acpi_init_cpus() -> void __init;
}
extern "C" {
    pub fn apei_claim_sea(regs: *mut pt_regs) -> c_int;
}

extern "C" {
    pub fn acpi_parking_protocol_valid(cpu: c_int) -> bool;
}

//
// acpi_disable_cmcff is used in drivers/acpi/apei/hest.c for disabling
// IA-32 Architecture Corrected Machine Check (CMC) Firmware-First mode
// with a kernel command line parameter "acpi=nocmcoff". But we don't
// have this IA-32 specific feature on ARM64, this definition is only
// for compatibility.
//
pub const acpi_disable_cmcff: c_int = 1;
extern "C" {
    pub fn __acpi_get_mem_attribute(_arg: addr) -> return;
}

extern "C" {
    pub fn arm64_acpi_numa_init() -> c_int;
}
extern "C" {
    pub fn acpi_numa_get_nid(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn acpi_map_cpus_to_nodes();
}

