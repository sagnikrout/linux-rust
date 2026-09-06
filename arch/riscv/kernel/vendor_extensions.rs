//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/vendor_extensions.c
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
// Copyright 2024 Rivos, Inc
//

    struct riscv_isa_vendor_ext_data_list *riscv_isa_vendor_ext_list[] = {

    &riscv_isa_vendor_ext_list_andes,

    &riscv_isa_vendor_ext_list_mips,

    &riscv_isa_vendor_ext_list_sifive,

    &riscv_isa_vendor_ext_list_thead,

    };
    let mut riscv_isa_vendor_ext_list_size: usize = ARRAY_SIZE(riscv_isa_vendor_ext_list);
//
// __riscv_isa_vendor_extension_available() - Check whether given vendor
// extension is available or not.
//
// @cpu: check if extension is available on this cpu
// @vendor: vendor that the extension is a member of
// @bit: bit position of the desired extension
// Return: true or false
//
// NOTE: When cpu is -1, will check if extension is available on all cpus
//
#[no_mangle]
pub unsafe extern "C" fn __riscv_isa_vendor_extension_available(cpu: c_int, vendor: c_ulong, bit: c_uint) -> bool {
    bool __riscv_isa_vendor_extension_available(int cpu, unsigned long vendor, unsigned int bit)
    {
    struct riscv_isavendorinfo *bmap;
    struct riscv_isavendorinfo *cpu_bmap;
    switch (vendor) {

    case ANDES_VENDOR_ID:
    bmap = &riscv_isa_vendor_ext_list_andes.all_harts_isa_bitmap;
    cpu_bmap = riscv_isa_vendor_ext_list_andes.per_hart_isa_bitmap;
    break;

    case MIPS_VENDOR_ID:
    bmap = &riscv_isa_vendor_ext_list_mips.all_harts_isa_bitmap;
    cpu_bmap = riscv_isa_vendor_ext_list_mips.per_hart_isa_bitmap;
    break;

    case SIFIVE_VENDOR_ID:
    bmap = &riscv_isa_vendor_ext_list_sifive.all_harts_isa_bitmap;
    cpu_bmap = riscv_isa_vendor_ext_list_sifive.per_hart_isa_bitmap;
    break;

    case THEAD_VENDOR_ID:
    bmap = &riscv_isa_vendor_ext_list_thead.all_harts_isa_bitmap;
    cpu_bmap = riscv_isa_vendor_ext_list_thead.per_hart_isa_bitmap;
    break;

    default:
    return false;
    }
    if (cpu != -1)
    bmap = &cpu_bmap[cpu];
    if (bit >= RISCV_ISA_VENDOR_EXT_MAX)
    return false;
    return test_bit(bit, bmap.isa);
    }
    EXPORT_SYMBOL_GPL(__riscv_isa_vendor_extension_available);
