//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/riscv/iommu.h
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
// Copyright © 2022-2024 Rivos Inc.
// Copyright © 2023 FORTH-ICS/CARV
//
// Authors
// Tomasz Jeznach <tjeznach@rivosinc.com>
// Nick Kossifidis <mick@ics.forth.gr>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct riscv_iommu_queue {
    pub /: *mut *mut atomic_t prod; / unbounded producer allocation index,
    pub /: *mut *mut atomic_t head; / unbounded shadow ring buffer consumer index,
    pub /: *mut *mut atomic_t tail; / unbounded shadow ring buffer producer index,
    pub /: *mut *mut unsigned int mask; / index mask, queue length - 1,
    pub /: *mut *mut unsigned int irq; / allocated interrupt number,
    pub /: *mut *mut *mut riscv_iommu_device iommu; / iommu device handling the queue when active,
    pub /: *mut *mut *mut void base; / ring buffer kernel pointer,
    pub /: *mut *mut dma_addr_t phys; / ring buffer physical address,
    pub /: *mut *mut u16 qbr; / base register offset, head and tail reference,
    pub /: *mut *mut u16 qcr; / control and status register offset,
    pub /: *mut *mut u8 qid; / queue identifier, same as RISCV_IOMMU_INTR_XX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct riscv_iommu_device {
// iommu core interface
    pub iommu: iommu_device,
// iommu hardware
    pub dev: *mut device,
// hardware control register space
    pub reg: *mut void __iomem,
// supported and enabled hardware capabilities
    pub caps: u64,
    pub fctl: u32,
// available interrupt numbers, MSI or WSI
    pub irqs: [c_uint; RISCV_IOMMU_INTR_COUNT],
    pub irqs_count: c_uint,
    pub icvec: c_uint,
// hardware queues
    pub cmdq: riscv_iommu_queue,
    pub fltq: riscv_iommu_queue,
// device directory
    pub ddt_mode: c_uint,
    pub ddt_phys: dma_addr_t,
    pub ddt_root: *mut u64,
}

extern "C" {
    pub fn riscv_iommu_init(iommu: *mut riscv_iommu_device) -> c_int;
}
extern "C" {
    pub fn riscv_iommu_remove(iommu: *mut riscv_iommu_device);
}
extern "C" {
    pub fn riscv_iommu_disable(iommu: *mut riscv_iommu_device);
}

