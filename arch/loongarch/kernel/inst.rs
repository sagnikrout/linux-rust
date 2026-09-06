//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kernel/inst.c
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

    static DEFINE_RAW_SPINLOCK(patch_lock);
#[no_mangle]
pub unsafe extern "C" fn simu_pc(regs: *mut pt_regs, insn: union loongarch_instruction) {
    void simu_pc(struct pt_regs *regs, union loongarch_instruction insn)
    {
    let mut pc: c_ulong = regs.csr_era;
    let mut rd: c_uint = insn.reg1i20_format.rd;
    let mut imm: c_uint = insn.reg1i20_format.immediate;
    if (pc & 3) {
    pr_warn("%s: invalid pc 0x%lx\n", __func__, pc);
    return;
    }
    switch (insn.reg1i20_format.opcode) {
    case pcaddi_op:
    regs.regs[rd] = pc + sign_extend64(imm << 2, 21);
    break;
    case pcaddu12i_op:
    regs.regs[rd] = pc + sign_extend64(imm << 12, 31);
    break;
    case pcaddu18i_op:
    regs.regs[rd] = pc + sign_extend64(imm << 18, 37);
    break;
    case pcalau12i_op:
    regs.regs[rd] = pc + sign_extend64(imm << 12, 31);
    regs.regs[rd] &= ~((1 << 12) - 1);
    break;
    default:
    pr_info("%s: unknown opcode\n", __func__);
    return;
    }
    regs.csr_era += LOONGARCH_INSN_SIZE;
    }
#[no_mangle]
pub unsafe extern "C" fn simu_branch(regs: *mut pt_regs, insn: union loongarch_instruction) {
    void simu_branch(struct pt_regs *regs, union loongarch_instruction insn)
    {
    unsigned int imm, imm_l, imm_h, rd, rj;
    let mut pc: c_ulong = regs.csr_era;
    if (pc & 3) {
    pr_warn("%s: invalid pc 0x%lx\n", __func__, pc);
    return;
    }
    imm_l = insn.reg0i26_format.immediate_l;
    imm_h = insn.reg0i26_format.immediate_h;
    switch (insn.reg0i26_format.opcode) {
    case b_op:
    regs.csr_era = pc + sign_extend64((imm_h << 16 | imm_l) << 2, 27);
    return;
    case bl_op:
    regs.csr_era = pc + sign_extend64((imm_h << 16 | imm_l) << 2, 27);
    regs.regs[1] = pc + LOONGARCH_INSN_SIZE;
    return;
    }
    imm_l = insn.reg1i21_format.immediate_l;
    imm_h = insn.reg1i21_format.immediate_h;
    rj = insn.reg1i21_format.rj;
    switch (insn.reg1i21_format.opcode) {
    case beqz_op:
    if (regs.regs[rj] == 0)
    regs.csr_era = pc + sign_extend64((imm_h << 16 | imm_l) << 2, 22);
    else
    regs.csr_era = pc + LOONGARCH_INSN_SIZE;
    return;
    case bnez_op:
    if (regs.regs[rj] != 0)
    regs.csr_era = pc + sign_extend64((imm_h << 16 | imm_l) << 2, 22);
    else
    regs.csr_era = pc + LOONGARCH_INSN_SIZE;
    return;
    }
    imm = insn.reg2i16_format.immediate;
    rj = insn.reg2i16_format.rj;
    rd = insn.reg2i16_format.rd;
    switch (insn.reg2i16_format.opcode) {
    case beq_op:
    if (regs.regs[rj] == regs.regs[rd])
    regs.csr_era = pc + sign_extend64(imm << 2, 17);
    else
    regs.csr_era = pc + LOONGARCH_INSN_SIZE;
    break;
    case bne_op:
    if (regs.regs[rj] != regs.regs[rd])
    regs.csr_era = pc + sign_extend64(imm << 2, 17);
    else
    regs.csr_era = pc + LOONGARCH_INSN_SIZE;
    break;
    case blt_op:
    if ((long)regs.regs[rj] < (long)regs.regs[rd])
    regs.csr_era = pc + sign_extend64(imm << 2, 17);
    else
    regs.csr_era = pc + LOONGARCH_INSN_SIZE;
    break;
    case bge_op:
    if ((long)regs.regs[rj] >= (long)regs.regs[rd])
    regs.csr_era = pc + sign_extend64(imm << 2, 17);
    else
    regs.csr_era = pc + LOONGARCH_INSN_SIZE;
    break;
    case bltu_op:
    if (regs.regs[rj] < regs.regs[rd])
    regs.csr_era = pc + sign_extend64(imm << 2, 17);
    else
    regs.csr_era = pc + LOONGARCH_INSN_SIZE;
    break;
    case bgeu_op:
    if (regs.regs[rj] >= regs.regs[rd])
    regs.csr_era = pc + sign_extend64(imm << 2, 17);
    else
    regs.csr_era = pc + LOONGARCH_INSN_SIZE;
    break;
    case jirl_op:
    regs.csr_era = regs.regs[rj] + sign_extend64(imm << 2, 17);
    regs.regs[rd] = pc + LOONGARCH_INSN_SIZE;
    break;
    default:
    pr_info("%s: unknown opcode\n", __func__);
    return;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn insns_not_supported(insn: union loongarch_instruction) -> bool {
    bool insns_not_supported(union loongarch_instruction insn)
    {
    switch (insn.reg3_format.opcode) {
    case amswapw_op ... ammindbdu_op:
    pr_notice("atomic memory access instructions are not supported\n");
    return true;
    case scq_op:
    pr_notice("sc.q instruction is not supported\n");
    return true;
    }
    switch (insn.reg2i14_format.opcode) {
    case llw_op:
    case lld_op:
    case scw_op:
    case scd_op:
    pr_notice("ll and sc instructions are not supported\n");
    return true;
    }
    switch (insn.reg2_format.opcode) {
    case llacqw_op:
    case llacqd_op:
    case screlw_op:
    case screld_op:
    pr_notice("llacq and screl instructions are not supported\n");
    return true;
    }
    switch (insn.reg1i21_format.opcode) {
    case bceqz_op:
    pr_notice("bceqz and bcnez instructions are not supported\n");
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn insns_need_simulation(insn: union loongarch_instruction) -> bool {
    bool insns_need_simulation(union loongarch_instruction insn)
    {
    if (is_pc_ins(&insn))
    return true;
    if (is_branch_ins(&insn))
    return true;
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_simulate_insn(insn: union loongarch_instruction, regs: *mut pt_regs) {
    void arch_simulate_insn(union loongarch_instruction insn, struct pt_regs *regs)
    {
    if (is_pc_ins(&insn))
    simu_pc(regs, insn);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_branch_ins(&insn)) -> else {
    else if (is_branch_ins(&insn))
    simu_branch(regs, insn);
    }
#[no_mangle]
pub unsafe extern "C" fn larch_insn_read(addr: *mut c_void, insnp: *mut u32) -> c_int {
    int larch_insn_read(void *addr, u32 *insnp)
    {
    int ret;
    u32 val;
    ret = copy_from_kernel_nofault(&val, addr, LOONGARCH_INSN_SIZE);
    if (!ret)
// insnp = val;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn larch_insn_write(addr: *mut c_void, insn: u32) -> c_int {
    int larch_insn_write(void *addr, u32 insn)
    {
    int ret;
    let mut flags: c_ulong = 0;
    if ((unsigned long)addr & 3)
    return -EINVAL;
    raw_spin_lock_irqsave(&patch_lock, flags);
    ret = copy_to_kernel_nofault(addr, &insn, LOONGARCH_INSN_SIZE);
    raw_spin_unlock_irqrestore(&patch_lock, flags);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn larch_insn_patch_text(addr: *mut c_void, insn: u32) -> c_int {
    int larch_insn_patch_text(void *addr, u32 insn)
    {
    int ret;
    u32 *tp = addr;
    ret = larch_insn_write(tp, insn);
    if (!ret)
    flush_icache_range((unsigned long)tp,
    (unsigned long)tp + LOONGARCH_INSN_SIZE);
    return ret;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct insn_copy {
    pub dst: *mut c_void,
    pub src: *mut c_void,
    pub len: usize,
    pub cpu: c_uint,
}

#[no_mangle]
unsafe extern "C" fn text_copy_cb(data: *mut c_void) -> c_int {
    static int text_copy_cb(void *data)
    {
    let mut ret: c_int = 0;
    struct insn_copy *copy = data;
    if (smp_processor_id() == copy.cpu) {
    ret = copy_to_kernel_nofault(copy.dst, copy.src, copy.len);
    if (ret) {
    pr_err("%s: operation failed\n", __func__);
    return ret;
    }
    }
    flush_icache_range((unsigned long)copy.dst, (unsigned long)copy.dst + copy.len);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn larch_insn_text_copy(dst: *mut c_void, src: *mut c_void, len: usize) -> c_int {
    int larch_insn_text_copy(void *dst, void *src, size_t len)
    {
    let mut ret: c_int = 0;
    let mut err: c_int = 0;
    size_t start, end;
    struct insn_copy copy = {
    .dst = dst,
    .src = src,
    .len = len,
    .cpu = raw_smp_processor_id(),
    };
//
// Ensure copy.cpu won't be hot removed before stop_machine.
// If it is removed nobody will really update the text.
//
    lockdep_assert_cpus_held();
    start = round_down((size_t)dst, PAGE_SIZE);
    end   = round_up((size_t)dst + len, PAGE_SIZE);
    err = set_memory_rw(start, (end - start) / PAGE_SIZE);
    if (err) {
    pr_info("%s: set_memory_rw() failed\n", __func__);
    return err;
    }
    ret = stop_machine_cpuslocked(text_copy_cb, &copy, cpu_online_mask);
    err = set_memory_rox(start, (end - start) / PAGE_SIZE);
    if (err) {
    pr_info("%s: set_memory_rox() failed\n", __func__);
    return err;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn larch_insn_gen_nop() -> u32 {
    u32 larch_insn_gen_nop(void)
    {
    return INSN_NOP;
    }
#[no_mangle]
pub unsafe extern "C" fn larch_insn_gen_b(pc: c_ulong, dest: c_ulong) -> u32 {
    u32 larch_insn_gen_b(unsigned long pc, unsigned long dest)
    {
    let mut offset: c_long = dest - pc;
    union loongarch_instruction insn;
    if ((offset & 3) || offset < -SZ_128M || offset >= SZ_128M) {
    pr_warn("The generated b instruction is out of range.\n");
    return INSN_BREAK;
    }
    emit_b(&insn, offset >> 2);
    return insn.word;
    }
#[no_mangle]
pub unsafe extern "C" fn larch_insn_gen_bl(pc: c_ulong, dest: c_ulong) -> u32 {
    u32 larch_insn_gen_bl(unsigned long pc, unsigned long dest)
    {
    let mut offset: c_long = dest - pc;
    union loongarch_instruction insn;
    if ((offset & 3) || offset < -SZ_128M || offset >= SZ_128M) {
    pr_warn("The generated bl instruction is out of range.\n");
    return INSN_BREAK;
    }
    emit_bl(&insn, offset >> 2);
    return insn.word;
    }
#[no_mangle]
pub unsafe extern "C" fn larch_insn_gen_break(imm: c_int) -> u32 {
    u32 larch_insn_gen_break(int imm)
    {
    union loongarch_instruction insn;
    if (imm < 0 || imm >= SZ_32K) {
    pr_warn("The generated break instruction is out of range.\n");
    return INSN_BREAK;
    }
    emit_break(&insn, imm);
    return insn.word;
    }
#[no_mangle]
pub unsafe extern "C" fn larch_insn_gen_or(rd: enum loongarch_gpr, rj: enum loongarch_gpr, rk: enum loongarch_gpr) -> u32 {
    u32 larch_insn_gen_or(enum loongarch_gpr rd, enum loongarch_gpr rj, enum loongarch_gpr rk)
    {
    union loongarch_instruction insn;
    emit_or(&insn, rd, rj, rk);
    return insn.word;
    }
#[no_mangle]
pub unsafe extern "C" fn larch_insn_gen_move(rd: enum loongarch_gpr, rj: enum loongarch_gpr) -> u32 {
    u32 larch_insn_gen_move(enum loongarch_gpr rd, enum loongarch_gpr rj)
    {
    return larch_insn_gen_or(rd, rj, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn larch_insn_gen_lu12iw(rd: enum loongarch_gpr, imm: c_int) -> u32 {
    u32 larch_insn_gen_lu12iw(enum loongarch_gpr rd, int imm)
    {
    union loongarch_instruction insn;
    if (imm < -SZ_512K || imm >= SZ_512K) {
    pr_warn("The generated lu12i.w instruction is out of range.\n");
    return INSN_BREAK;
    }
    emit_lu12iw(&insn, rd, imm);
    return insn.word;
    }
#[no_mangle]
pub unsafe extern "C" fn larch_insn_gen_lu32id(rd: enum loongarch_gpr, imm: c_int) -> u32 {
    u32 larch_insn_gen_lu32id(enum loongarch_gpr rd, int imm)
    {
    union loongarch_instruction insn;
    if (imm < -SZ_512K || imm >= SZ_512K) {
    pr_warn("The generated lu32i.d instruction is out of range.\n");
    return INSN_BREAK;
    }
    emit_lu32id(&insn, rd, imm);
    return insn.word;
    }
#[no_mangle]
pub unsafe extern "C" fn larch_insn_gen_lu52id(rd: enum loongarch_gpr, rj: enum loongarch_gpr, imm: c_int) -> u32 {
    u32 larch_insn_gen_lu52id(enum loongarch_gpr rd, enum loongarch_gpr rj, int imm)
    {
    union loongarch_instruction insn;
    if (imm < -SZ_2K || imm >= SZ_2K) {
    pr_warn("The generated lu52i.d instruction is out of range.\n");
    return INSN_BREAK;
    }
    emit_lu52id(&insn, rd, rj, imm);
    return insn.word;
    }
#[no_mangle]
pub unsafe extern "C" fn larch_insn_gen_beq(rd: enum loongarch_gpr, rj: enum loongarch_gpr, imm: c_int) -> u32 {
    u32 larch_insn_gen_beq(enum loongarch_gpr rd, enum loongarch_gpr rj, int imm)
    {
    union loongarch_instruction insn;
    if ((imm & 3) || imm < -SZ_128K || imm >= SZ_128K) {
    pr_warn("The generated beq instruction is out of range.\n");
    return INSN_BREAK;
    }
    emit_beq(&insn, rj, rd, imm >> 2);
    return insn.word;
    }
#[no_mangle]
pub unsafe extern "C" fn larch_insn_gen_bne(rd: enum loongarch_gpr, rj: enum loongarch_gpr, imm: c_int) -> u32 {
    u32 larch_insn_gen_bne(enum loongarch_gpr rd, enum loongarch_gpr rj, int imm)
    {
    union loongarch_instruction insn;
    if ((imm & 3) || imm < -SZ_128K || imm >= SZ_128K) {
    pr_warn("The generated bne instruction is out of range.\n");
    return INSN_BREAK;
    }
    emit_bne(&insn, rj, rd, imm >> 2);
    return insn.word;
    }
#[no_mangle]
pub unsafe extern "C" fn larch_insn_gen_jirl(rd: enum loongarch_gpr, rj: enum loongarch_gpr, imm: c_int) -> u32 {
    u32 larch_insn_gen_jirl(enum loongarch_gpr rd, enum loongarch_gpr rj, int imm)
    {
    union loongarch_instruction insn;
    if ((imm & 3) || imm < -SZ_128K || imm >= SZ_128K) {
    pr_warn("The generated jirl instruction is out of range.\n");
    return INSN_BREAK;
    }
    emit_jirl(&insn, rd, rj, imm >> 2);
    return insn.word;
    }
