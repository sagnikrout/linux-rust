//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/irq.h
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

//
// The highest address on the IRQ stack contains a dummy frame which is
// structured as follows:
//
// top ------------
// | task sp  | <- irq_stack[cpu] + IRQ_STACK_START
// ------------
// |          | <- First frame of IRQ context
// ------------
//
// task sp holds a copy of the task stack pointer where the struct pt_regs
// from exception entry can be found.
//
extern "C" {
    pub fn spurious_interrupt();
}
pub const NR_IRQS_LEGACY: c_int = 16;
//
// 256 Vectors Mapping for AVECINTC:
//
// 0 - 15: Mapping classic IPs, e.g. IP0-12.
// 16 - 255: Mapping vectors for external IRQ.
//
pub const NR_VECTORS: c_int = 256;
pub const NR_LEGACY_VECTORS: c_int = 16;
pub const AVEC_IRQ_SHIFT: c_int = 4;
pub const AVEC_IRQ_BIT: c_int = 8;

pub const AVEC_CPU_SHIFT: c_int = 12;
pub const AVEC_CPU_BIT: c_int = 16;

extern "C" {
    pub fn arch_trigger_cpumask_backtrace(mask: *const cpumask, exclude_cpu: c_int);
}

pub const MAX_IO_PICS: c_int = 1;

pub const MAX_IO_PICS: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_vector_group {
    pub node: c_int,
    pub pci_segment: c_int,
    pub parent: *mut irq_domain,
}

pub const CORES_PER_EIO_NODE: c_int = 4;
pub const CORES_PER_VEIO_NODE: c_int = 256;

// IRQ number definitions
pub const LOONGSON_LPC_IRQ_BASE: c_int = 0;

pub const LOONGSON_CPU_IRQ_BASE: c_int = 16;

pub const LOONGSON_PCH_IRQ_BASE: c_int = 64;

extern "C" {
    pub fn complete_irq_moving();
}
extern "C" {
    pub fn irq_create_mapping(_arg: d, _arg: vector) -> return;
}

