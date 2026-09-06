//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/annotate-arch/annotate-powerpc.c
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

pub const MINUS_EXT_XO_FORM: c_int = 234;
pub const SUB_EXT_XO_FORM: c_int = 232;
pub const ADD_ZERO_EXT_XO_FORM: c_int = 202;
pub const SUB_ZERO_EXT_XO_FORM: c_int = 200;
    static int arithmetic__scnprintf(const struct ins *ins, char *bf, size_t size,
    struct ins_operands *ops, int max_ins_name)
    {
    return scnprintf(bf, size, "%-*s %s", max_ins_name, ins.name,
    ops.raw);
    }
//
// Sets the fields: multi_regs and "mem_ref".
// "mem_ref" is set for ops->source which is later used to
// fill the objdump->memory_ref-char field. This ops is currently
// used by powerpc and since binary instruction code is used to
// extract opcode, regs and offset, no other parsing is needed here.
//
// Dont set multi regs for 4 cases since it has only one operand
// for source:
// - Add to Minus One Extended XO-form ( Ex: addme, addmeo )
// - Subtract From Minus One Extended XO-form ( Ex: subfme )
// - Add to Zero Extended XO-form ( Ex: addze, addzeo )
// - Subtract From Zero Extended XO-form ( Ex: subfze )
//
    static int arithmetic__parse(const struct arch *arch __maybe_unused, struct ins_operands *ops,
    struct map_symbol *ms __maybe_unused, struct disasm_line *dl)
    {
    let mut opcode: c_int = PPC_OP(dl.raw.raw_insn);
    ops.source.mem_ref = false;
    if (opcode == 31) {
    if ((opcode != MINUS_EXT_XO_FORM) && (opcode != SUB_EXT_XO_FORM) &&
    (opcode != ADD_ZERO_EXT_XO_FORM) && (opcode != SUB_ZERO_EXT_XO_FORM))
    ops.source.multi_regs = true;
    }
    ops.target.mem_ref = false;
    ops.target.multi_regs = false;
    return 0;
    }
    static const struct ins_ops arithmetic_ops = {
    .parse     = arithmetic__parse,
    .scnprintf = arithmetic__scnprintf,
    };
    static int load_store__scnprintf(const struct ins *ins, char *bf, size_t size,
    struct ins_operands *ops, int max_ins_name)
    {
    return scnprintf(bf, size, "%-*s %s", max_ins_name, ins.name,
    ops.raw);
    }
//
// Sets the fields: multi_regs and "mem_ref".
// "mem_ref" is set for ops->source which is later used to
// fill the objdump->memory_ref-char field. This ops is currently
// used by powerpc and since binary instruction code is used to
// extract opcode, regs and offset, no other parsing is needed here
//
    static int load_store__parse(const struct arch *arch __maybe_unused, struct ins_operands *ops,
    struct map_symbol *ms __maybe_unused, struct disasm_line *dl __maybe_unused)
    {
    ops.source.mem_ref = true;
    ops.source.multi_regs = false;
// opcode 31 is of X form
    if (PPC_OP(dl.raw.raw_insn) == 31)
    ops.source.multi_regs = true;
    ops.target.mem_ref = false;
    ops.target.multi_regs = false;
    return 0;
    }
    static const struct ins_ops load_store_ops = {
    .parse     = load_store__parse,
    .scnprintf = load_store__scnprintf,
    };
    static const struct ins_ops *powerpc__associate_instruction_ops(struct arch *arch, const char *name)
    {
    int i;
    const struct ins_ops *ops;
//
// - Interested only if instruction starts with 'b'.
// - Few start with 'b', but aren't branch instructions.
//
    if (name[0] != 'b'             ||
    !strncmp(name, "bcd", 3)   ||
    !strncmp(name, "brinc", 5) ||
    !strncmp(name, "bper", 4))
    return core::ptr::null_mut();
    ops = &jump_ops;
    i = strlen(name) - 1;
    if (i < 0)
    return core::ptr::null_mut();
// ignore optional hints at the end of the instructions
    if (name[i] == '+' || name[i] == '-')
    i--;
    if (name[i] == 'l' || (name[i] == 'a' && name[i-1] == 'l')) {
//
// if the instruction ends up with 'l' or 'la', then
// those are considered 'calls' since they update LR.
// ... except for 'bnl' which is branch if not less than
// and the absolute form of the same.
//
    if (strcmp(name, "bnl") && strcmp(name, "bnl+") &&
    strcmp(name, "bnl-") && strcmp(name, "bnla") &&
    strcmp(name, "bnla+") && strcmp(name, "bnla-"))
    ops = &call_ops;
    }
    if (name[i] == 'r' && name[i-1] == 'l')
//
// instructions ending with 'lr' are considered to be
// return instructions
//
    ops = &ret_ops;
    arch__associate_ins_ops(arch, name, ops);
    return ops;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct insn_offset {
    pub name: *const c_char,
    pub value: c_int,
}

//
// There are memory instructions with opcode 31 which are
// of X Form, Example:
// ldx RT,RA,RB
// ______________________________________
// | 31 |  RT  |  RA |  RB |   21     |/|
// --------------------------------------
// 0    6     11    16    21         30 31
//
// But all instructions with opcode 31 are not memory.
// Example: add RT,RA,RB
//
// Use bits 21 to 30 to check memory insns with 31 as opcode.
// In ins_array below, for ldx instruction:
// name => OP_31_XOP_LDX
// value => 21
//
    static struct insn_offset ins_array[] = {
    { .name = "OP_31_XOP_LXSIWZX",  .value = 12, },
    { .name = "OP_31_XOP_LWARX",	.value = 20, },
    { .name = "OP_31_XOP_LDX",	.value = 21, },
    { .name = "OP_31_XOP_LWZX",	.value = 23, },
    { .name = "OP_31_XOP_LDUX",	.value = 53, },
    { .name = "OP_31_XOP_LWZUX",	.value = 55, },
    { .name = "OP_31_XOP_LXSIWAX",  .value = 76, },
    { .name = "OP_31_XOP_LDARX",    .value = 84, },
    { .name = "OP_31_XOP_LBZX",	.value = 87, },
    { .name = "OP_31_XOP_LVX",      .value = 103, },
    { .name = "OP_31_XOP_LBZUX",    .value = 119, },
    { .name = "OP_31_XOP_STXSIWX",  .value = 140, },
    { .name = "OP_31_XOP_STDX",	.value = 149, },
    { .name = "OP_31_XOP_STWX",	.value = 151, },
    { .name = "OP_31_XOP_STDUX",	.value = 181, },
    { .name = "OP_31_XOP_STWUX",	.value = 183, },
    { .name = "OP_31_XOP_STBX",	.value = 215, },
    { .name = "OP_31_XOP_STVX",     .value = 231, },
    { .name = "OP_31_XOP_STBUX",	.value = 247, },
    { .name = "OP_31_XOP_LHZX",	.value = 279, },
    { .name = "OP_31_XOP_LHZUX",	.value = 311, },
    { .name = "OP_31_XOP_LXVDSX",   .value = 332, },
    { .name = "OP_31_XOP_LWAX",	.value = 341, },
    { .name = "OP_31_XOP_LHAX",	.value = 343, },
    { .name = "OP_31_XOP_LWAUX",	.value = 373, },
    { .name = "OP_31_XOP_LHAUX",	.value = 375, },
    { .name = "OP_31_XOP_STHX",	.value = 407, },
    { .name = "OP_31_XOP_STHUX",	.value = 439, },
    { .name = "OP_31_XOP_LXSSPX",   .value = 524, },
    { .name = "OP_31_XOP_LDBRX",	.value = 532, },
    { .name = "OP_31_XOP_LSWX",	.value = 533, },
    { .name = "OP_31_XOP_LWBRX",	.value = 534, },
    { .name = "OP_31_XOP_LFSUX",    .value = 567, },
    { .name = "OP_31_XOP_LXSDX",    .value = 588, },
    { .name = "OP_31_XOP_LSWI",	.value = 597, },
    { .name = "OP_31_XOP_LFDX",     .value = 599, },
    { .name = "OP_31_XOP_LFDUX",    .value = 631, },
    { .name = "OP_31_XOP_STXSSPX",  .value = 652, },
    { .name = "OP_31_XOP_STDBRX",	.value = 660, },
    { .name = "OP_31_XOP_STXWX",	.value = 661, },
    { .name = "OP_31_XOP_STWBRX",	.value = 662, },
    { .name = "OP_31_XOP_STFSX",	.value = 663, },
    { .name = "OP_31_XOP_STFSUX",	.value = 695, },
    { .name = "OP_31_XOP_STXSDX",   .value = 716, },
    { .name = "OP_31_XOP_STSWI",	.value = 725, },
    { .name = "OP_31_XOP_STFDX",	.value = 727, },
    { .name = "OP_31_XOP_STFDUX",	.value = 759, },
    { .name = "OP_31_XOP_LXVW4X",   .value = 780, },
    { .name = "OP_31_XOP_LHBRX",	.value = 790, },
    { .name = "OP_31_XOP_LXVD2X",   .value = 844, },
    { .name = "OP_31_XOP_LFIWAX",	.value = 855, },
    { .name = "OP_31_XOP_LFIWZX",	.value = 887, },
    { .name = "OP_31_XOP_STXVW4X",  .value = 908, },
    { .name = "OP_31_XOP_STHBRX",	.value = 918, },
    { .name = "OP_31_XOP_STXVD2X",  .value = 972, },
    { .name = "OP_31_XOP_STFIWX",	.value = 983, },
    };
//
// Arithmetic instructions which are having opcode as 31.
// These instructions are tracked to save the register state
// changes. Example:
//
// lwz	r10,264(r3)
// add	r31, r3, r3
// lwz	r9, 0(r31)
//
// Here instruction tracking needs to identify the "add"
// instruction and save data type of r3 to r31. If a sample
// is hit at next "lwz r9, 0(r31)", by this instruction tracking,
// data type of r31 can be resolved.
//
    static struct insn_offset arithmetic_ins_op_31[] = {
    { .name = "SUB_CARRY_XO_FORM",  .value = 8, },
    { .name = "MUL_HDW_XO_FORM1",   .value = 9, },
    { .name = "ADD_CARRY_XO_FORM",  .value = 10, },
    { .name = "MUL_HW_XO_FORM1",    .value = 11, },
    { .name = "SUB_XO_FORM",        .value = 40, },
    { .name = "MUL_HDW_XO_FORM",    .value = 73, },
    { .name = "MUL_HW_XO_FORM",     .value = 75, },
    { .name = "SUB_EXT_XO_FORM",    .value = 136, },
    { .name = "ADD_EXT_XO_FORM",    .value = 138, },
    { .name = "SUB_ZERO_EXT_XO_FORM",       .value = 200, },
    { .name = "ADD_ZERO_EXT_XO_FORM",       .value = 202, },
    { .name = "SUB_EXT_XO_FORM2",   .value = 232, },
    { .name = "MUL_DW_XO_FORM",     .value = 233, },
    { .name = "ADD_EXT_XO_FORM2",   .value = 234, },
    { .name = "MUL_W_XO_FORM",      .value = 235, },
    { .name = "ADD_XO_FORM",	.value = 266, },
    { .name = "DIV_DW_XO_FORM1",    .value = 457, },
    { .name = "DIV_W_XO_FORM1",     .value = 459, },
    { .name = "DIV_DW_XO_FORM",	.value = 489, },
    { .name = "DIV_W_XO_FORM",	.value = 491, },
    };
    static struct insn_offset arithmetic_two_ops[] = {
    { .name = "mulli",      .value = 7, },
    { .name = "subfic",     .value = 8, },
    { .name = "addic",      .value = 12, },
    { .name = "addic.",     .value = 13, },
    { .name = "addi",       .value = 14, },
    { .name = "addis",      .value = 15, },
    };
#[no_mangle]
unsafe extern "C" fn cmp_offset(a: *const c_void, b: *const c_void) -> c_int {
    static int cmp_offset(const void *a, const void *b)
    {
    const struct insn_offset *val1 = a;
    const struct insn_offset *val2 = b;
    return (val1.value - val2.value);
    }
    const struct ins_ops *check_ppc_insn(struct disasm_line *dl)
    {
    let mut raw_insn: c_int = dl.raw.raw_insn;
    let mut opcode: c_int = PPC_OP(raw_insn);
    let mut mem_insn_31: c_int = PPC_21_30(raw_insn);
    struct insn_offset *ret;
    struct insn_offset mem_insns_31_opcode = {
    "OP_31_INSN",
    mem_insn_31
    };
    char name_insn[32];
//
// Instructions with opcode 32 to 63 are memory
// instructions in powerpc
//
    if ((opcode & 0x20)) {
//
// Set name in case of raw instruction to
// opcode to be used in insn-stat
//
    if (!strlen(dl.ins.name)) {
    sprintf(name_insn, "%d", opcode);
    dl.ins.name = strdup(name_insn);
    }
    return &load_store_ops;
    } else if (opcode == 31) {
// Check for memory instructions with opcode 31
    ret = bsearch(&mem_insns_31_opcode, ins_array, ARRAY_SIZE(ins_array), sizeof(ins_array[0]), cmp_offset);
    if (ret) {
    if (!strlen(dl.ins.name))
    dl.ins.name = strdup(ret.name);
    return &load_store_ops;
    } else {
    mem_insns_31_opcode.value = PPC_22_30(raw_insn);
    ret = bsearch(&mem_insns_31_opcode, arithmetic_ins_op_31, ARRAY_SIZE(arithmetic_ins_op_31),
    sizeof(arithmetic_ins_op_31[0]), cmp_offset);
    if (ret != core::ptr::null_mut())
    return &arithmetic_ops;
// Bits 21 to 30 has value 444 for "mr" insn ie, OR X form
    if (PPC_21_30(raw_insn) == 444)
    return &arithmetic_ops;
    }
    } else {
    mem_insns_31_opcode.value = opcode;
    ret = bsearch(&mem_insns_31_opcode, arithmetic_two_ops, ARRAY_SIZE(arithmetic_two_ops),
    sizeof(arithmetic_two_ops[0]), cmp_offset);
    if (ret != core::ptr::null_mut())
    return &arithmetic_ops;
    }
    return core::ptr::null_mut();
    }
//
// Instruction tracking function to track register state moves.
// Example sequence:
// ld      r10,264(r3)
// mr      r31,r3
// <<after some sequence>
// ld      r9,312(r31)
//
// Previous instruction sequence shows that register state of r3
// is moved to r31. update_insn_state_powerpc tracks these state
// changes
//

    static void update_insn_state_powerpc(struct type_state *state,
    struct data_loc_info *dloc, Dwarf_Die * cu_die __maybe_unused,
    struct disasm_line *dl)
    {
    struct annotated_insn_loc loc;
    struct annotated_op_loc *src = &loc.ops[INSN_OP_SOURCE];
    struct annotated_op_loc *dst = &loc.ops[INSN_OP_TARGET];
    struct type_state_reg *tsr;
    let mut insn_offset: u32 = dl.al.offset;
    if (annotate_get_insn_location(dloc.arch, dl, &loc) < 0)
    return;
//
// Value 444 for bits 21:30 is for "mr"
// instruction. "mr" is extended OR. So set the
// source and destination reg correctly
//
    if (PPC_21_30(dl.raw.raw_insn) == 444) {
    let mut src_reg: c_int = src.reg1;
    src.reg1 = dst.reg1;
    dst.reg1 = src_reg;
    }
    if (!has_reg_type(state, dst.reg1))
    return;
    tsr = &state.regs[dst.reg1];
    if (!has_reg_type(state, src.reg1) ||
    !state.regs[src.reg1].ok) {
    tsr.ok = false;
    return;
    }
    tsr.type = state.regs[src.reg1].type;
    tsr.kind = state.regs[src.reg1].kind;
    tsr.ok = true;
    pr_debug_dtp("mov [%x] reg%d . reg%d",
    insn_offset, src.reg1, dst.reg1);
    pr_debug_type_name(&tsr.type, tsr.kind);
    }

    const struct arch *arch__new_powerpc(const struct e_machine_and_e_flags *id,
    const char *cpuid __maybe_unused)
    {
    struct arch *arch = zalloc(sizeof(*arch));
    if (!arch)
    return core::ptr::null_mut();
    arch.name = "powerpc";
    arch.id = *id;
    arch.objdump.comment_char = '#';
    annotate_opts.show_asm_raw = true;
    arch.associate_instruction_ops = powerpc__associate_instruction_ops;

    arch.update_insn_state = update_insn_state_powerpc;

    return arch;
    }
