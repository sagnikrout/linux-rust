//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/vendor_extensions/thead.c
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

// All T-Head vendor extensions supported in Linux
    static const struct riscv_isa_ext_data riscv_isa_vendor_ext_thead[] = {
    __RISCV_ISA_EXT_DATA(xtheadvector, RISCV_ISA_VENDOR_EXT_XTHEADVECTOR),
    };
    struct riscv_isa_vendor_ext_data_list riscv_isa_vendor_ext_list_thead = {
    .ext_data_count = ARRAY_SIZE(riscv_isa_vendor_ext_thead),
    .ext_data = riscv_isa_vendor_ext_thead,
    };
#[no_mangle]
pub unsafe extern "C" fn disable_xtheadvector() {
    void disable_xtheadvector(void)
    {
    int cpu;
    for_each_possible_cpu(cpu)
    clear_bit(RISCV_ISA_VENDOR_EXT_XTHEADVECTOR, riscv_isa_vendor_ext_list_thead.per_hart_isa_bitmap[cpu].isa);
    clear_bit(RISCV_ISA_VENDOR_EXT_XTHEADVECTOR, riscv_isa_vendor_ext_list_thead.all_harts_isa_bitmap.isa);
    }
