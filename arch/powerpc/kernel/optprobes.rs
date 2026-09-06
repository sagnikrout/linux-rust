//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/optprobes.c
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
//
// Code for Kernel probes Jump optimization.
//
// Copyright 2017, Anju T, IBM Corp.
//

    static bool insn_page_in_use;
    void *alloc_optinsn_page(void)
    {
    if (insn_page_in_use)
    return core::ptr::null_mut();
    insn_page_in_use = true;
    return &optinsn_slot;
    }
#[no_mangle]
pub unsafe extern "C" fn free_optinsn_page(page: *mut c_void) {
    void free_optinsn_page(void *page)
    {
    insn_page_in_use = false;
    }
//
// Check if we can optimize this probe. Returns NIP post-emulation if this can
// be optimized and 0 otherwise.
//
#[no_mangle]
unsafe extern "C" fn can_optimize(p: *mut kprobe) -> c_ulong {
    static unsigned long can_optimize(struct kprobe *p)
    {
    struct pt_regs regs;
    struct instruction_op op;
    let mut nip: c_ulong = 0;
    let mut addr: c_ulong = (unsigned long)p.addr;
//
// kprobe placed for kretprobe during boot time
// has a 'nop' instruction, which can be emulated.
// So further checks can be skipped.
//
    if (p.addr == (kprobe_opcode_t *)&arch_rethook_trampoline)
    return addr + sizeof(kprobe_opcode_t);
//
// We only support optimizing kernel addresses, but not
// module addresses.
//
// FIXME: Optimize kprobes placed in module addresses.
//
    if (!is_kernel_addr(addr))
    return 0;
    memset(&regs, 0, sizeof(struct pt_regs));
    regs.nip = addr;
    regs.trap = 0x0;
    regs.msr = MSR_KERNEL;
//
// Kprobe placed in conditional branch instructions are
// not optimized, as we can't predict the nip prior with
// dummy pt_regs and can not ensure that the return branch
// from detour buffer falls in the range of address (i.e 32MB).
// A branch back from trampoline is set up in the detour buffer
// to the nip returned by the analyse_instr() here.
//
// Ensure that the instruction is not a conditional branch,
// and that can be emulated.
//
    if (!is_conditional_branch(ppc_inst_read(p.ainsn.insn)) &&
    analyse_instr(&op, &regs, ppc_inst_read(p.ainsn.insn)) == 1) {
    emulate_update_regs(&regs, &op);
    nip = regs.nip;
    }
    return nip;
    }
    static void optimized_callback(struct optimized_kprobe *op,
    struct pt_regs *regs)
    {
// This is possible if op is under delayed unoptimizing
    if (kprobe_disabled(&op.kp))
    return;
    preempt_disable();
    if (kprobe_running()) {
    kprobes_inc_nmissed_count(&op.kp);
    } else {
    __this_cpu_write(current_kprobe, &op.kp);
    regs_set_return_ip(regs, (unsigned long)op.kp.addr);
    get_kprobe_ctlblk().kprobe_status = KPROBE_HIT_ACTIVE;
    opt_pre_handler(&op.kp, regs);
    __this_cpu_write(current_kprobe, core::ptr::null_mut());
    }
    preempt_enable();
    }
    NOKPROBE_SYMBOL(optimized_callback);
#[no_mangle]
pub unsafe extern "C" fn arch_remove_optimized_kprobe(op: *mut optimized_kprobe) {
    void arch_remove_optimized_kprobe(struct optimized_kprobe *op)
    {
    if (op.optinsn.insn) {
    free_optinsn_slot(op.optinsn.insn, 1);
    op.optinsn.insn = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn patch_imm32_load_insns(val: c_ulong, reg: c_int, addr: *mut kprobe_opcode_t) {
    static void patch_imm32_load_insns(unsigned long val, int reg, kprobe_opcode_t *addr)
    {
    patch_instruction(addr++, ppc_inst(PPC_RAW_LIS(reg, PPC_HI(val))));
    patch_instruction(addr, ppc_inst(PPC_RAW_ORI(reg, reg, PPC_LO(val))));
    }
//
// Generate instructions to load provided immediate 64-bit value
// to register 'reg' and patch these instructions at 'addr'.
//
#[no_mangle]
unsafe extern "C" fn patch_imm64_load_insns(val: c_ulonglong, reg: c_int, addr: *mut kprobe_opcode_t) {
    static void patch_imm64_load_insns(unsigned long long val, int reg, kprobe_opcode_t *addr)
    {
    patch_instruction(addr++, ppc_inst(PPC_RAW_LIS(reg, PPC_HIGHEST(val))));
    patch_instruction(addr++, ppc_inst(PPC_RAW_ORI(reg, reg, PPC_HIGHER(val))));
    patch_instruction(addr++, ppc_inst(PPC_RAW_SLDI(reg, reg, 32)));
    patch_instruction(addr++, ppc_inst(PPC_RAW_ORIS(reg, reg, PPC_HI(val))));
    patch_instruction(addr, ppc_inst(PPC_RAW_ORI(reg, reg, PPC_LO(val))));
    }
#[no_mangle]
unsafe extern "C" fn patch_imm_load_insns(val: c_ulong, reg: c_int, addr: *mut kprobe_opcode_t) {
    static void patch_imm_load_insns(unsigned long val, int reg, kprobe_opcode_t *addr)
    {
    if (IS_ENABLED(CONFIG_PPC64))
    patch_imm64_load_insns(val, reg, addr);
    else
    patch_imm32_load_insns(val, reg, addr);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_prepare_optimized_kprobe(op: *mut optimized_kprobe, p: *mut kprobe) -> c_int {
    int arch_prepare_optimized_kprobe(struct optimized_kprobe *op, struct kprobe *p)
    {
    ppc_inst_t branch_op_callback, branch_emulate_step, temp;
    unsigned long op_callback_addr, emulate_step_addr;
    kprobe_opcode_t *buff;
    long b_offset;
    unsigned long nip, size;
    int rc, i;
    nip = can_optimize(p);
    if (!nip)
    return -EILSEQ;
// Allocate instruction slot for detour buffer
    buff = get_optinsn_slot();
    if (!buff)
    return -ENOMEM;
//
// OPTPROBE uses 'b' instruction to branch to optinsn.insn.
//
// The target address has to be relatively nearby, to permit use
// of branch instruction in powerpc, because the address is specified
// in an immediate field in the instruction opcode itself, ie 24 bits
// in the opcode specify the address. Therefore the address should
// be within 32MB on either side of the current instruction.
//
    b_offset = (unsigned long)buff - (unsigned long)p.addr;
    if (!is_offset_in_branch_range(b_offset))
    goto error;
// Check if the return address is also within 32MB range
    b_offset = (unsigned long)(buff + TMPL_RET_IDX) - nip;
    if (!is_offset_in_branch_range(b_offset))
    goto error;
// Setup template
// We can optimize this via patch_instruction_window later
    size = (TMPL_END_IDX * sizeof(kprobe_opcode_t)) / sizeof(int);
    pr_devel("Copying template to %p, size %lu\n", buff, size);
    for (i = 0; i < size; i++) {
    rc = patch_instruction(buff + i, ppc_inst(*(optprobe_template_entry + i)));
    if (rc < 0)
    goto error;
    }
//
// Fixup the template with instructions to:
// 1. load the address of the actual probepoint
//
    patch_imm_load_insns((unsigned long)op, 3, buff + TMPL_OP_IDX);
//
// 2. branch to optimized_callback() and emulate_step()
//
    op_callback_addr = ppc_kallsyms_lookup_name("optimized_callback");
    emulate_step_addr = ppc_kallsyms_lookup_name("emulate_step");
    if (!op_callback_addr || !emulate_step_addr) {
    WARN(1, "Unable to lookup optimized_callback()/emulate_step()\n");
    goto error;
    }
    rc = create_branch(&branch_op_callback, buff + TMPL_CALL_HDLR_IDX,
    op_callback_addr, BRANCH_SET_LINK);
    rc |= create_branch(&branch_emulate_step, buff + TMPL_EMULATE_IDX,
    emulate_step_addr, BRANCH_SET_LINK);
    if (rc)
    goto error;
    patch_instruction(buff + TMPL_CALL_HDLR_IDX, branch_op_callback);
    patch_instruction(buff + TMPL_EMULATE_IDX, branch_emulate_step);
//
// 3. load instruction to be emulated into relevant register, and
//
    temp = ppc_inst_read(p.ainsn.insn);
    patch_imm_load_insns(ppc_inst_as_ulong(temp), 4, buff + TMPL_INSN_IDX);
//
// 4. branch back from trampoline
//
    patch_branch(buff + TMPL_RET_IDX, nip, 0);
    flush_icache_range((unsigned long)buff, (unsigned long)(&buff[TMPL_END_IDX]));
    op.optinsn.insn = buff;
    return 0;
    error:
    free_optinsn_slot(buff, 0);
    return -ERANGE;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_prepared_optinsn(optinsn: *mut arch_optimized_insn) -> c_int {
    int arch_prepared_optinsn(struct arch_optimized_insn *optinsn)
    {
    return optinsn.insn != core::ptr::null_mut();
    }
//
// On powerpc, Optprobes always replaces one instruction (4 bytes
// aligned and 4 bytes long). It is impossible to encounter another
// kprobe in this address range. So always return 0.
//
#[no_mangle]
pub unsafe extern "C" fn arch_check_optimized_kprobe(op: *mut optimized_kprobe) -> c_int {
    int arch_check_optimized_kprobe(struct optimized_kprobe *op)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_optimize_kprobes(oplist: *mut list_head) {
    void arch_optimize_kprobes(struct list_head *oplist)
    {
    ppc_inst_t instr;
    struct optimized_kprobe *op;
    struct optimized_kprobe *tmp;
    list_for_each_entry_safe(op, tmp, oplist, list) {
//
// Backup instructions which will be replaced
// by jump address
//
    memcpy(op.optinsn.copied_insn, op.kp.addr, RELATIVEJUMP_SIZE);
    create_branch(&instr, op.kp.addr, (unsigned long)op.optinsn.insn, 0);
    patch_instruction(op.kp.addr, instr);
    list_del_init(&op.list);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn arch_unoptimize_kprobe(op: *mut optimized_kprobe) {
    void arch_unoptimize_kprobe(struct optimized_kprobe *op)
    {
    arch_arm_kprobe(&op.kp);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_unoptimize_kprobes(oplist: *mut list_head, done_list: *mut list_head) {
    void arch_unoptimize_kprobes(struct list_head *oplist, struct list_head *done_list)
    {
    struct optimized_kprobe *op;
    struct optimized_kprobe *tmp;
    list_for_each_entry_safe(op, tmp, oplist, list) {
    arch_unoptimize_kprobe(op);
    list_move(&op.list, done_list);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn arch_within_optimized_kprobe(op: *mut optimized_kprobe, addr: *mut kprobe_opcode_t) -> c_int {
    int arch_within_optimized_kprobe(struct optimized_kprobe *op, kprobe_opcode_t *addr)
    {
    return (op.kp.addr <= addr &&
    op.kp.addr + (RELATIVEJUMP_SIZE / sizeof(kprobe_opcode_t)) > addr);
    }
