//! Automatically rewritten from C to Rust
//! Source: tools/objtool/arch/loongarch/orc.c
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


// SPDX-License-Identifier: GPL-2.0-or-later

#[no_mangle]
pub unsafe extern "C" fn init_orc_entry(orc: *mut orc_entry, cfi: *mut cfi_state, insn: *mut instruction) -> c_int {
    int init_orc_entry(struct orc_entry *orc, struct cfi_state *cfi, struct instruction *insn)
    {
    struct cfi_reg *fp = &cfi.regs[CFI_FP];
    struct cfi_reg *ra = &cfi.regs[CFI_RA];
    memset(orc, 0, sizeof(*orc));
    if (!cfi) {
//
// This is usually either unreachable nops/traps (which don't
// trigger unreachable instruction warnings), or
// STACK_FRAME_NON_STANDARD functions.
//
    orc.type = ORC_TYPE_UNDEFINED;
    return 0;
    }
    switch (cfi.type) {
    case UNWIND_HINT_TYPE_UNDEFINED:
    orc.type = ORC_TYPE_UNDEFINED;
    return 0;
    case UNWIND_HINT_TYPE_END_OF_STACK:
    orc.type = ORC_TYPE_END_OF_STACK;
    return 0;
    case UNWIND_HINT_TYPE_CALL:
    orc.type = ORC_TYPE_CALL;
    break;
    case UNWIND_HINT_TYPE_REGS:
    orc.type = ORC_TYPE_REGS;
    break;
    case UNWIND_HINT_TYPE_REGS_PARTIAL:
    orc.type = ORC_TYPE_REGS_PARTIAL;
    break;
    default:
    ERROR_INSN(insn, "unknown unwind hint type %d", cfi.type);
    return -1;
    }
    orc.signal = cfi.signal;
    switch (cfi.cfa.base) {
    case CFI_SP:
    orc.sp_reg = ORC_REG_SP;
    break;
    case CFI_FP:
    orc.sp_reg = ORC_REG_FP;
    break;
    default:
    ERROR_INSN(insn, "unknown CFA base reg %d", cfi.cfa.base);
    return -1;
    }
    switch (fp.base) {
    case CFI_UNDEFINED:
    orc.fp_reg = ORC_REG_UNDEFINED;
    orc.fp_offset = 0;
    break;
    case CFI_CFA:
    orc.fp_reg = ORC_REG_PREV_SP;
    orc.fp_offset = fp.offset;
    break;
    case CFI_FP:
    orc.fp_reg = ORC_REG_FP;
    break;
    default:
    ERROR_INSN(insn, "unknown FP base reg %d", fp.base);
    return -1;
    }
    switch (ra.base) {
    case CFI_UNDEFINED:
    orc.ra_reg = ORC_REG_UNDEFINED;
    orc.ra_offset = 0;
    break;
    case CFI_CFA:
    orc.ra_reg = ORC_REG_PREV_SP;
    orc.ra_offset = ra.offset;
    break;
    case CFI_FP:
    orc.ra_reg = ORC_REG_FP;
    break;
    default:
    ERROR_INSN(insn, "unknown RA base reg %d", ra.base);
    return -1;
    }
    orc.sp_offset = cfi.cfa.offset;
    return 0;
    }
    int write_orc_entry(struct elf *elf, struct section *orc_sec,
    struct section *ip_sec, unsigned int idx,
    struct section *insn_sec, unsigned long insn_off,
    struct orc_entry *o)
    {
    struct orc_entry *orc;
// populate ORC data
    orc = (struct orc_entry *)orc_sec.data.d_buf + idx;
    memcpy(orc, o, sizeof(*orc));
// populate reloc for ip
    if (!elf_init_reloc_text_sym(elf, ip_sec, idx * sizeof(int), idx,
    insn_sec, insn_off))
    return -1;
    return 0;
    }
    static const char *reg_name(unsigned int reg)
    {
    switch (reg) {
    case ORC_REG_SP:
    return "sp";
    case ORC_REG_FP:
    return "fp";
    case ORC_REG_PREV_SP:
    return "prevsp";
    default:
    return "?";
    }
    }
    static const char *orc_type_name(unsigned int type)
    {
    switch (type) {
    case UNWIND_HINT_TYPE_CALL:
    return "call";
    case UNWIND_HINT_TYPE_REGS:
    return "regs";
    case UNWIND_HINT_TYPE_REGS_PARTIAL:
    return "regs (partial)";
    default:
    return "?";
    }
    }
#[no_mangle]
unsafe extern "C" fn print_reg(reg: c_uint, offset: c_int) {
    static void print_reg(unsigned int reg, int offset)
    {
    if (reg == ORC_REG_UNDEFINED)
    printf(" (und) ");
    else
    printf("%s + %3d", reg_name(reg), offset);
    }
#[no_mangle]
pub unsafe extern "C" fn orc_print_dump(dummy_elf: *mut elf, orc: *mut orc_entry, i: c_int) {
    void orc_print_dump(struct elf *dummy_elf, struct orc_entry *orc, int i)
    {
    printf("type:%s", orc_type_name(orc[i].type));
    printf(" sp:");
    print_reg(orc[i].sp_reg, orc[i].sp_offset);
    printf(" fp:");
    print_reg(orc[i].fp_reg, orc[i].fp_offset);
    printf(" ra:");
    print_reg(orc[i].ra_reg, orc[i].ra_offset);
    printf(" signal:%d\n", orc[i].signal);
    }
