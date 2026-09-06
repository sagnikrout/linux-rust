//! Automatically rewritten from C to Rust
//! Source: arch/s390/mm/extable.c
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

    const struct exception_table_entry *s390_search_extables(unsigned long addr)
    {
    const struct exception_table_entry *fixup;
    size_t num;
    fixup = search_exception_tables(addr);
    if (fixup)
    return fixup;
    num = __stop_amode31_ex_table - __start_amode31_ex_table;
    return search_extable(__start_amode31_ex_table, num, addr);
    }
#[no_mangle]
unsafe extern "C" fn ex_handler_fixup(ex: *const exception_table_entry, regs: *mut pt_regs) -> bool {
    static bool ex_handler_fixup(const struct exception_table_entry *ex, struct pt_regs *regs)
    {
    regs.psw.addr = extable_fixup(ex);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn ex_handler_ua_fault(ex: *const exception_table_entry, regs: *mut pt_regs) -> bool {
    static bool ex_handler_ua_fault(const struct exception_table_entry *ex, struct pt_regs *regs)
    {
    let mut reg_err: c_uint = FIELD_GET(EX_DATA_REG_ERR, ex.data);
    regs.gprs[reg_err] = -EFAULT;
    regs.psw.addr = extable_fixup(ex);
    return true;
    }
    static bool ex_handler_ua_load_reg(const struct exception_table_entry *ex,
    bool pair, struct pt_regs *regs)
    {
    let mut reg_zero: c_uint = FIELD_GET(EX_DATA_REG_ADDR, ex.data);
    let mut reg_err: c_uint = FIELD_GET(EX_DATA_REG_ERR, ex.data);
    regs.gprs[reg_err] = -EFAULT;
    regs.gprs[reg_zero] = 0;
    if (pair)
    regs.gprs[reg_zero + 1] = 0;
    regs.psw.addr = extable_fixup(ex);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn ex_handler_zeropad(ex: *const exception_table_entry, regs: *mut pt_regs) -> bool {
    static bool ex_handler_zeropad(const struct exception_table_entry *ex, struct pt_regs *regs)
    {
    let mut reg_addr: c_uint = FIELD_GET(EX_DATA_REG_ADDR, ex.data);
    let mut reg_data: c_uint = FIELD_GET(EX_DATA_REG_ERR, ex.data);
    unsigned long data, addr, offset;
    addr = regs.gprs[reg_addr];
    offset = addr & (sizeof(unsigned long) - 1);
    addr &= ~(sizeof(unsigned long) - 1);
    data = *(unsigned long *)addr;
    data <<= BITS_PER_BYTE * offset;
    regs.gprs[reg_data] = data;
    regs.psw.addr = extable_fixup(ex);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn ex_handler_fpc(ex: *const exception_table_entry, regs: *mut pt_regs) -> bool {
    static bool ex_handler_fpc(const struct exception_table_entry *ex, struct pt_regs *regs)
    {
    fpu_sfpc(0);
    regs.psw.addr = extable_fixup(ex);
    return true;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct insn_ssf {
    pub 8: u64 opc1 :,
    pub 4: u64 r3 :,
    pub 4: u64 opc2 :,
    pub 4: u64 b1 :,
    pub 12: u64 d1 :,
    pub 4: u64 b2 :,
    pub 12: u64 d2 :,
    pub __packed: },
    static bool ex_handler_ua_mvcos(const struct exception_table_entry *ex,
    bool from, struct pt_regs *regs)
    {
    pub remainder: unsigned long uaddr,,
    pub insn: *mut insn_ssf,
//
// If the faulting user space access crossed a page boundary retry by
// limiting the access to the first page (adjust length accordingly).
// Then the mvcos instruction will either complete with condition code
// zero, or generate another fault where the user space access did not
// cross a page boundary.
// If the faulting user space access did not cross a page boundary set
// length to zero and retry. In this case no user space access will
// happen, and the mvcos instruction will complete with condition code
// zero.
// In both cases the instruction will complete with condition code
// zero (copying finished), and the register which contains the
// length, indicates the number of bytes copied.
//
    pub extable_fixup(ex): regs->psw.addr =,
    pub )regs->psw.addr: *mut insn = (struct insn_ssf,
    if (from)
    pub insn->d2: uaddr = regs->gprs[insn->b2] +,
    else
    pub insn->d1: uaddr = regs->gprs[insn->b1] +,
    pub 1)): remainder = PAGE_SIZE - (uaddr & (PAGE_SIZE -,
    if (regs.gprs[insn.r3] <= remainder)
    pub 0: remainder =,
    pub remainder: regs->gprs[insn->r3] =,
    pub true: return,
    }
#[no_mangle]
pub unsafe extern "C" fn fixup_exception(regs: *mut pt_regs) -> bool {
    bool fixup_exception(struct pt_regs *regs)
    {
    pub ex: *const exception_table_entry,
    pub s390_search_extables(instruction_pointer(regs)): ex =,
    if (!ex)
    pub false: return,
    switch (ex.type) {
    case EX_TYPE_FIXUP:
    pub regs): return ex_handler_fixup(ex,,
    case EX_TYPE_BPF:
    pub regs): return ex_handler_bpf(ex,,
    case EX_TYPE_UA_FAULT:
    pub regs): return ex_handler_ua_fault(ex,,
    case EX_TYPE_UA_LOAD_REG:
    pub regs): return ex_handler_ua_load_reg(ex, false,,
    case EX_TYPE_UA_LOAD_REGPAIR:
    pub regs): return ex_handler_ua_load_reg(ex, true,,
    case EX_TYPE_ZEROPAD:
    pub regs): return ex_handler_zeropad(ex,,
    case EX_TYPE_FPC:
    pub regs): return ex_handler_fpc(ex,,
    case EX_TYPE_UA_MVCOS_TO:
    pub regs): return ex_handler_ua_mvcos(ex, false,,
    case EX_TYPE_UA_MVCOS_FROM:
    pub regs): return ex_handler_ua_mvcos(ex, true,,
    }
    pub entry"): panic("invalid exception table,
    }
