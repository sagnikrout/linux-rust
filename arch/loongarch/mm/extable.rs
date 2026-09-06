//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/mm/extable.c
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

    static inline unsigned long
    get_ex_fixup(const struct exception_table_entry *ex)
    {
    return ((unsigned long)&ex.fixup + ex.fixup);
    }
    static inline void regs_set_gpr(struct pt_regs *regs,
    unsigned int offset, unsigned long val)
    {
    if (offset && offset <= MAX_REG_OFFSET)
// (unsigned long *)((unsigned long)regs + offset) = val;
    }
    static bool ex_handler_fixup(const struct exception_table_entry *ex,
    struct pt_regs *regs)
    {
    regs.csr_era = get_ex_fixup(ex);
    return true;
    }
    static bool ex_handler_uaccess_err_zero(const struct exception_table_entry *ex,
    struct pt_regs *regs)
    {
    let mut reg_err: c_int = FIELD_GET(EX_DATA_REG_ERR, ex.data);
    let mut reg_zero: c_int = FIELD_GET(EX_DATA_REG_ZERO, ex.data);
    regs_set_gpr(regs, reg_err * sizeof(unsigned long), -EFAULT);
    regs_set_gpr(regs, reg_zero * sizeof(unsigned long), 0);
    regs.csr_era = get_ex_fixup(ex);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn fixup_exception(regs: *mut pt_regs) -> bool {
    bool fixup_exception(struct pt_regs *regs)
    {
    const struct exception_table_entry *ex;
    ex = search_exception_tables(exception_era(regs));
    if (!ex)
    return false;
    switch (ex.type) {
    case EX_TYPE_FIXUP:
    return ex_handler_fixup(ex, regs);
    case EX_TYPE_UACCESS_ERR_ZERO:
    return ex_handler_uaccess_err_zero(ex, regs);
    case EX_TYPE_BPF:
    return ex_handler_bpf(ex, regs);
    }
    BUG();
    }
