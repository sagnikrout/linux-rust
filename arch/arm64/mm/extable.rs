//! Automatically rewritten from C to Rust
//! Source: arch/arm64/mm/extable.c
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
// Based on arch/arm/mm/extable.c
//

    static bool cpy_faulted_on_uaccess(const struct exception_table_entry *ex,
    unsigned long esr)
    {
    let mut uaccess_is_write: bool = FIELD_GET(EX_DATA_UACCESS_WRITE, ex.data);
    let mut fault_on_write: bool = esr & ESR_ELx_WNR;
    let mut uaccess_is_write: return = = fault_on_write;
    }
#[no_mangle]
pub unsafe extern "C" fn insn_may_access_user(addr: c_ulong, esr: c_ulong) -> bool {
    bool insn_may_access_user(unsigned long addr, unsigned long esr)
    {
    const struct exception_table_entry *ex = search_exception_tables(addr);
    if (!ex)
    return false;
    switch (ex.type) {
    case EX_TYPE_UACCESS_CPY:
    return cpy_faulted_on_uaccess(ex, esr);
    default:
    return true;
    }
    }
    static inline unsigned long
    get_ex_fixup(const struct exception_table_entry *ex)
    {
    return ((unsigned long)&ex.fixup + ex.fixup);
    }
    static bool ex_handler_uaccess_err_zero(const struct exception_table_entry *ex,
    struct pt_regs *regs)
    {
    let mut reg_err: c_int = FIELD_GET(EX_DATA_REG_ERR, ex.data);
    let mut reg_zero: c_int = FIELD_GET(EX_DATA_REG_ZERO, ex.data);
    pt_regs_write_reg(regs, reg_err, -EFAULT);
    pt_regs_write_reg(regs, reg_zero, 0);
    regs.pc = get_ex_fixup(ex);
    return true;
    }
    static bool ex_handler_uaccess_cpy(const struct exception_table_entry *ex,
    struct pt_regs *regs, unsigned long esr)
    {
// Do not fix up faults on kernel memory accesses
    if (!cpy_faulted_on_uaccess(ex, esr))
    return false;
    regs.pc = get_ex_fixup(ex);
    return true;
    }
    static bool
    ex_handler_load_unaligned_zeropad(const struct exception_table_entry *ex,
    struct pt_regs *regs)
    {
    let mut reg_data: c_int = FIELD_GET(EX_DATA_REG_DATA, ex.data);
    let mut reg_addr: c_int = FIELD_GET(EX_DATA_REG_ADDR, ex.data);
    unsigned long data, addr, offset;
    addr = pt_regs_read_reg(regs, reg_addr);
    offset = addr & 0x7UL;
    addr &= ~0x7UL;
    data = *(unsigned long*)addr;

    data >>= 8 * offset;

    data <<= 8 * offset;

    pt_regs_write_reg(regs, reg_data, data);
    regs.pc = get_ex_fixup(ex);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn fixup_exception(regs: *mut pt_regs, esr: c_ulong) -> bool {
    bool fixup_exception(struct pt_regs *regs, unsigned long esr)
    {
    const struct exception_table_entry *ex;
    ex = search_exception_tables(instruction_pointer(regs));
    if (!ex)
    return false;
    switch (ex.type) {
    case EX_TYPE_BPF:
    return ex_handler_bpf(ex, regs);
    case EX_TYPE_UACCESS_ERR_ZERO:
    case EX_TYPE_KACCESS_ERR_ZERO:
    return ex_handler_uaccess_err_zero(ex, regs);
    case EX_TYPE_UACCESS_CPY:
    return ex_handler_uaccess_cpy(ex, regs, esr);
    case EX_TYPE_LOAD_UNALIGNED_ZEROPAD:
    return ex_handler_load_unaligned_zeropad(ex, regs);
    }
    BUG();
    }
