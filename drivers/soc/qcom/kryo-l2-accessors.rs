//! Automatically rewritten from C to Rust
//! Source: drivers/soc/qcom/kryo-l2-accessors.c
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
// Copyright (c) 2018, The Linux Foundation. All rights reserved.
//

    static DEFINE_RAW_SPINLOCK(l2_access_lock);
//
// kryo_l2_set_indirect_reg() - write value to an L2 register
// @reg: Address of L2 register.
// @val: Value to be written to register.
//
// Use architecturally required barriers for ordering between system register
// accesses, and system registers with respect to device memory
//
#[no_mangle]
pub unsafe extern "C" fn kryo_l2_set_indirect_reg(reg: u64, val: u64) {
    void kryo_l2_set_indirect_reg(u64 reg, u64 val)
    {
    unsigned long flags;
    raw_spin_lock_irqsave(&l2_access_lock, flags);
    write_sysreg_s(reg, L2CPUSRSELR_EL1);
    isb();
    write_sysreg_s(val, L2CPUSRDR_EL1);
    isb();
    raw_spin_unlock_irqrestore(&l2_access_lock, flags);
    }
    EXPORT_SYMBOL_GPL(kryo_l2_set_indirect_reg);
//
// kryo_l2_get_indirect_reg() - read an L2 register value
// @reg: Address of L2 register.
//
// Use architecturally required barriers for ordering between system register
// accesses, and system registers with respect to device memory
//
#[no_mangle]
pub unsafe extern "C" fn kryo_l2_get_indirect_reg(reg: u64) -> u64 {
    u64 kryo_l2_get_indirect_reg(u64 reg)
    {
    u64 val;
    unsigned long flags;
    raw_spin_lock_irqsave(&l2_access_lock, flags);
    write_sysreg_s(reg, L2CPUSRSELR_EL1);
    isb();
    val = read_sysreg_s(L2CPUSRDR_EL1);
    raw_spin_unlock_irqrestore(&l2_access_lock, flags);
    return val;
    }
    EXPORT_SYMBOL_GPL(kryo_l2_get_indirect_reg);
