//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/xmon/ppc-dis.c
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
// ppc-dis.c -- Disassemble PowerPC instructions
    Copyright (C) 1994-2016 Free Software Foundation, Inc.
    Written by Ian Lance Taylor, Cygnus Support
    This file is part of GDB, GAS, and the GNU binutils.
//

// This file provides several disassembler functions, all of which use
    the disassembler interface defined in dis-asm.h.  Several functions
    are provided because this file handles disassembly for the PowerPC
#[no_mangle]
pub unsafe extern "C" fn POWER(_arg: RS/6000) -> in both big and little endian mode and also for the {
    in both big and little endian mode and also for the POWER (RS/6000)
    chip.  */
// Extract the operand value from the PowerPC or POWER instruction.
    static long
    operand_value_powerpc (const struct powerpc_operand *operand,
    unsigned long insn, ppc_cpu_t dialect)
    {
    long value;
    int invalid;
// Extract the value from the instruction.
    if (operand.extract)
    value = (*operand.extract) (insn, dialect, &invalid);
    else
    {
    if (operand.shift >= 0)
    value = (insn >> operand.shift) & operand.bitm;
    else
    value = (insn << -operand.shift) & operand.bitm;
    if ((operand.flags & PPC_OPERAND_SIGNED) != 0)
    {
// BITM is always some number of zeros followed by some
    number of ones, followed by some number of zeros.  */
    let mut top: c_ulong = operand.bitm;
// top & -top gives the rightmost 1 bit, so this
    fills in any trailing zeros.  */
    top |= (top & -top) - 1;
    top &= ~(top >> 1);
    value = (value ^ top) - top;
    }
    }
    return value;
    }
// Determine whether the optional operand(s) should be printed.
    static int
    skip_optional_operands (const unsigned char *opindex,
    unsigned long insn, ppc_cpu_t dialect)
    {
    const struct powerpc_operand *operand;
    for (; *opindex != 0; opindex++)
    {
    operand = &powerpc_operands[*opindex];
    if ((operand.flags & PPC_OPERAND_NEXT) != 0
    || ((operand.flags & PPC_OPERAND_OPTIONAL) != 0
    && operand_value_powerpc (operand, insn, dialect) !=
    ppc_optional_operand_value (operand)))
    return 0;
    }
    return 1;
    }
// Find a match for INSN in the opcode table, given machine DIALECT.
    A DIALECT of -1 is special, matching all machine opcode variations.  */
    static const struct powerpc_opcode *
    lookup_powerpc (unsigned long insn, ppc_cpu_t dialect)
    {
    const struct powerpc_opcode *opcode;
    const struct powerpc_opcode *opcode_end;
    opcode_end = powerpc_opcodes + powerpc_num_opcodes;
// Find the first match in the opcode table for this major opcode.
    for (opcode = powerpc_opcodes; opcode < opcode_end; ++opcode)
    {
    const unsigned char *opindex;
    const struct powerpc_operand *operand;
    int invalid;
    if ((insn & opcode.mask) != opcode.opcode
    || (dialect != (ppc_cpu_t) -1
    && ((opcode.flags & dialect) == 0
    || (opcode.deprecated & dialect) != 0)))
    continue;
// Check validity of operands.
    invalid = 0;
    for (opindex = opcode.operands; *opindex != 0; opindex++)
    {
    operand = powerpc_operands + *opindex;
    if (operand.extract)
    (*operand.extract) (insn, dialect, &invalid);
    }
    if (invalid)
    continue;
    return opcode;
    }
    return core::ptr::null_mut();
    }
// Print a PowerPC or POWER instruction.
#[no_mangle]
pub unsafe extern "C" fn print_insn_powerpc(insn: c_ulong, memaddr: c_ulong) -> c_int {
    int print_insn_powerpc (unsigned long insn, unsigned long memaddr)
    {
    const struct powerpc_opcode *opcode;
    bool insn_is_short;
    ppc_cpu_t dialect;
    dialect = PPC_OPCODE_PPC | PPC_OPCODE_COMMON;
    if (IS_ENABLED(CONFIG_PPC64))
    dialect |= PPC_OPCODE_64 | PPC_OPCODE_POWER4 | PPC_OPCODE_CELL |
    PPC_OPCODE_POWER5 | PPC_OPCODE_POWER6 | PPC_OPCODE_POWER7 | PPC_OPCODE_POWER8 |
    PPC_OPCODE_POWER9;
    if (cpu_has_feature(CPU_FTR_TM))
    dialect |= PPC_OPCODE_HTM;
    if (cpu_has_feature(CPU_FTR_ALTIVEC))
    dialect |= PPC_OPCODE_ALTIVEC | PPC_OPCODE_ALTIVEC2;
    if (cpu_has_feature(CPU_FTR_VSX))
    dialect |= PPC_OPCODE_VSX | PPC_OPCODE_VSX3;
// Get the major opcode of the insn.
    opcode = core::ptr::null_mut();
    insn_is_short = false;
    if (opcode == core::ptr::null_mut())
    opcode = lookup_powerpc (insn, dialect);
    if (opcode == core::ptr::null_mut() && (dialect & PPC_OPCODE_ANY) != 0)
    opcode = lookup_powerpc (insn, (ppc_cpu_t) -1);
    if (opcode != core::ptr::null_mut())
    {
    const unsigned char *opindex;
    const struct powerpc_operand *operand;
    int need_comma;
    int need_paren;
    int skip_optional;
    if (opcode.operands[0] != 0)
    printf("%-7s ", opcode.name);
    else
    printf("%s", opcode.name);
    if (insn_is_short)
// The operands will be fetched out of the 16-bit instruction.
    insn >>= 16;
// Now extract and print the operands.
    need_comma = 0;
    need_paren = 0;
    skip_optional = -1;
    for (opindex = opcode.operands; *opindex != 0; opindex++)
    {
    long value;
    operand = powerpc_operands + *opindex;
// Operands that are marked FAKE are simply ignored.  We
    already made sure that the extract function considered
    the instruction to be valid.  */
    if ((operand.flags & PPC_OPERAND_FAKE) != 0)
    continue;
// If all of the optional operands have the value zero,
    then don't print any of them.  */
    if ((operand.flags & PPC_OPERAND_OPTIONAL) != 0)
    {
    if (skip_optional < 0)
    skip_optional = skip_optional_operands (opindex, insn,
    dialect);
    if (skip_optional)
    continue;
    }
    value = operand_value_powerpc (operand, insn, dialect);
    if (need_comma)
    {
    printf(",");
    need_comma = 0;
    }
// Print the operand as directed by the flags.
    if ((operand.flags & PPC_OPERAND_GPR) != 0
    || ((operand.flags & PPC_OPERAND_GPR_0) != 0 && value != 0))
    printf("r%ld", value);
#[no_mangle]
pub unsafe extern "C" fn if(0: (operand->flags & PPC_OPERAND_FPR) !=) -> else {
    else if ((operand.flags & PPC_OPERAND_FPR) != 0)
    printf("f%ld", value);
#[no_mangle]
pub unsafe extern "C" fn if(0: (operand->flags & PPC_OPERAND_VR) !=) -> else {
    else if ((operand.flags & PPC_OPERAND_VR) != 0)
    printf("v%ld", value);
#[no_mangle]
pub unsafe extern "C" fn if(0: (operand->flags & PPC_OPERAND_VSR) !=) -> else {
    else if ((operand.flags & PPC_OPERAND_VSR) != 0)
    printf("vs%ld", value);
#[no_mangle]
pub unsafe extern "C" fn if(0: (operand->flags & PPC_OPERAND_RELATIVE) !=) -> else {
    else if ((operand.flags & PPC_OPERAND_RELATIVE) != 0)
    print_address(memaddr + value);
#[no_mangle]
pub unsafe extern "C" fn if(0: (operand->flags & PPC_OPERAND_ABSOLUTE) !=) -> else {
    else if ((operand.flags & PPC_OPERAND_ABSOLUTE) != 0)
    print_address(value & 0xffffffff);
#[no_mangle]
pub unsafe extern "C" fn if(0: (operand->flags & PPC_OPERAND_FSL) !=) -> else {
    else if ((operand.flags & PPC_OPERAND_FSL) != 0)
    printf("fsl%ld", value);
#[no_mangle]
pub unsafe extern "C" fn if(0: (operand->flags & PPC_OPERAND_FCR) !=) -> else {
    else if ((operand.flags & PPC_OPERAND_FCR) != 0)
    printf("fcr%ld", value);
#[no_mangle]
pub unsafe extern "C" fn if(0: (operand->flags & PPC_OPERAND_UDI) !=) -> else {
    else if ((operand.flags & PPC_OPERAND_UDI) != 0)
    printf("%ld", value);
    else if ((operand.flags & PPC_OPERAND_CR_REG) != 0
    && (((dialect & PPC_OPCODE_PPC) != 0)
    || ((dialect & PPC_OPCODE_VLE) != 0)))
    printf("cr%ld", value);
#[no_mangle]
pub unsafe extern "C" fn if(0: ((operand->flags & PPC_OPERAND_CR_BIT) !=) -> else {
    else if (((operand.flags & PPC_OPERAND_CR_BIT) != 0)
    && (((dialect & PPC_OPCODE_PPC) != 0)
    || ((dialect & PPC_OPCODE_VLE) != 0)))
    {
    static const char *cbnames[4] = { "lt", "gt", "eq", "so" };
    int cr;
    int cc;
    cr = value >> 2;
    if (cr != 0)
    printf("4*cr%d+", cr);
    cc = value & 3;
    printf("%s", cbnames[cc]);
    }
    else
    printf("%d", (int) value);
    if (need_paren)
    {
    printf(")");
    need_paren = 0;
    }
    if ((operand.flags & PPC_OPERAND_PARENS) == 0)
    need_comma = 1;
    else
    {
    printf("(");
    need_paren = 1;
    }
    }
// We have found and printed an instruction.
    If it was a short VLE instruction we have more to do.  */
    if (insn_is_short)
    {
    memaddr += 2;
    return 2;
    }
    else
// Otherwise, return.
    return 4;
    }
// We could not find a match.
    printf(".long 0x%lx", insn);
    return 4;
    }
