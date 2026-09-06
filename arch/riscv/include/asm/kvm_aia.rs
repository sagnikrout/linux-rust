//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/kvm_aia.h
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
// Copyright (C) 2021 Western Digital Corporation or its affiliates.
// Copyright (C) 2022 Ventana Micro Systems Inc.
//
// Authors:
// Anup Patel <apatel@ventanamicro.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_aia {
// In-kernel irqchip created
    pub in_kernel: bool,
// In-kernel irqchip initialized
    pub initialized: bool,
// Virtualization mode (Emulation, HW Accelerated, or Auto)
    pub mode: u32,
// Number of MSIs
    pub nr_ids: u32,
// Number of wired IRQs
    pub nr_sources: u32,
// Number of group bits in IMSIC address
    pub nr_group_bits: u32,
// Position of group bits in IMSIC address
    pub nr_group_shift: u32,
// Number of hart bits in IMSIC address
    pub nr_hart_bits: u32,
// Number of guest bits in IMSIC address
    pub nr_guest_bits: u32,
// Guest physical address of APLIC
    pub aplic_addr: gpa_t,
// Internal state of APLIC
    pub aplic_state: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_aia_csr {
    pub vsiselect: c_ulong,
    pub hviprio1: c_ulong,
    pub hviprio2: c_ulong,
    pub vsieh: c_ulong,
    pub hviph: c_ulong,
    pub hviprio1h: c_ulong,
    pub hviprio2h: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_aia {
// CPU AIA CSR context of Guest VCPU
    pub guest_csr: kvm_vcpu_aia_csr,
// Guest physical address of IMSIC for this VCPU
    pub imsic_addr: gpa_t,
// HART index of IMSIC extacted from guest physical address
    pub hart_index: u32,
// Internal state of IMSIC for this VCPU
    pub imsic_state: *mut c_void,
}

extern "C" {
    pub fn kvm_riscv_vcpu_aia_imsic_has_interrupt(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn kvm_riscv_vcpu_aia_imsic_load(vcpu: *mut kvm_vcpu, cpu: c_int);
}
extern "C" {
    pub fn kvm_riscv_vcpu_aia_imsic_put(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_vcpu_aia_imsic_release(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_vcpu_aia_imsic_update(vcpu: *mut kvm_vcpu) -> c_int;
}

extern "C" {
    pub fn kvm_riscv_aia_imsic_has_attr(kvm: *mut kvm, type: c_ulong) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_vcpu_aia_imsic_reset(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_vcpu_aia_imsic_init(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_vcpu_aia_imsic_cleanup(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_aia_aplic_set_attr(kvm: *mut kvm, type: c_ulong, v: u32) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_aia_aplic_get_attr(kvm: *mut kvm, type: c_ulong, v: *mut u32) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_aia_aplic_has_attr(kvm: *mut kvm, type: c_ulong) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_aia_aplic_inject(kvm: *mut kvm, source: u32, level: bool) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_aia_aplic_init(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_aia_aplic_cleanup(kvm: *mut kvm);
}

extern "C" {
    pub fn kvm_riscv_vcpu_aia_flush_interrupts(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_vcpu_aia_sync_interrupts(vcpu: *mut kvm_vcpu);
}

extern "C" {
    pub fn kvm_riscv_vcpu_aia_has_interrupts(vcpu: *mut kvm_vcpu, mask: u64) -> bool;
}
extern "C" {
    pub fn kvm_riscv_vcpu_aia_update_hvip(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_vcpu_aia_load(vcpu: *mut kvm_vcpu, cpu: c_int);
}
extern "C" {
    pub fn kvm_riscv_vcpu_aia_put(vcpu: *mut kvm_vcpu);
}

extern "C" {
    pub fn kvm_riscv_vcpu_aia_update(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_vcpu_aia_reset(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_vcpu_aia_init(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_vcpu_aia_deinit(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_aia_inject_msi(kvm: *mut kvm, msi: *mut kvm_msi) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_aia_inject_irq(kvm: *mut kvm, irq: c_uint, level: bool) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_aia_init_vm(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_riscv_aia_destroy_vm(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_riscv_aia_free_hgei(cpu: c_int, hgei: c_int);
}
extern "C" {
    pub fn kvm_riscv_aia_pm_exit();
}
extern "C" {
    pub fn kvm_riscv_aia_pm_enter();
}
extern "C" {
    pub fn kvm_riscv_aia_enable();
}
extern "C" {
    pub fn kvm_riscv_aia_disable();
}
extern "C" {
    pub fn kvm_riscv_aia_init() -> c_int;
}
extern "C" {
    pub fn kvm_riscv_aia_exit();
}
