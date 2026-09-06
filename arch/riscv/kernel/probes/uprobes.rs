//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/probes/uprobes.c
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

#[no_mangle]
pub unsafe extern "C" fn is_swbp_insn(insn: *mut uprobe_opcode_t) -> bool {
    bool is_swbp_insn(uprobe_opcode_t *insn)
    {

    return (*insn & 0xffff) == UPROBE_SWBP_INSN;

    return *insn == UPROBE_SWBP_INSN;

    }
#[no_mangle]
pub unsafe extern "C" fn is_trap_insn(insn: *mut uprobe_opcode_t) -> bool {
    bool is_trap_insn(uprobe_opcode_t *insn)
    {
    return riscv_insn_is_ebreak(*insn) || riscv_insn_is_c_ebreak(*insn);
    }
#[no_mangle]
pub unsafe extern "C" fn uprobe_get_swbp_addr(regs: *mut pt_regs) -> c_ulong {
    unsigned long uprobe_get_swbp_addr(struct pt_regs *regs)
    {
    return instruction_pointer(regs);
    }
    int arch_uprobe_analyze_insn(struct arch_uprobe *auprobe, struct mm_struct *mm,
    unsigned long addr)
    {
    probe_opcode_t opcode;
    opcode = *(probe_opcode_t *)(&auprobe.insn[0]);
    auprobe.insn_size = GET_INSN_LENGTH(opcode);
    switch (riscv_probe_decode_insn(&opcode, &auprobe.api)) {
    case INSN_REJECTED:
    return -EINVAL;
    case INSN_GOOD_NO_SLOT:
    auprobe.simulate = true;
    break;
    case INSN_GOOD:
    auprobe.simulate = false;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_uprobe_pre_xol(auprobe: *mut arch_uprobe, regs: *mut pt_regs) -> c_int {
    int arch_uprobe_pre_xol(struct arch_uprobe *auprobe, struct pt_regs *regs)
    {
    struct uprobe_task *utask = current.utask;
    utask.autask.saved_cause = current.thread.bad_cause;
    current.thread.bad_cause = UPROBE_TRAP_NR;
    instruction_pointer_set(regs, utask.xol_vaddr);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_uprobe_post_xol(auprobe: *mut arch_uprobe, regs: *mut pt_regs) -> c_int {
    int arch_uprobe_post_xol(struct arch_uprobe *auprobe, struct pt_regs *regs)
    {
    struct uprobe_task *utask = current.utask;
    WARN_ON_ONCE(current.thread.bad_cause != UPROBE_TRAP_NR);
    current.thread.bad_cause = utask.autask.saved_cause;
    instruction_pointer_set(regs, utask.vaddr + auprobe.insn_size);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_uprobe_xol_was_trapped(t: *mut task_struct) -> bool {
    bool arch_uprobe_xol_was_trapped(struct task_struct *t)
    {
    if (t.thread.bad_cause != UPROBE_TRAP_NR)
    return true;
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_uprobe_skip_sstep(auprobe: *mut arch_uprobe, regs: *mut pt_regs) -> bool {
    bool arch_uprobe_skip_sstep(struct arch_uprobe *auprobe, struct pt_regs *regs)
    {
    probe_opcode_t insn;
    unsigned long addr;
    if (!auprobe.simulate)
    return false;
    insn = *(probe_opcode_t *)(&auprobe.insn[0]);
    addr = instruction_pointer(regs);
    if (auprobe.api.handler)
    auprobe.api.handler(insn, addr, regs);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_uprobe_abort_xol(auprobe: *mut arch_uprobe, regs: *mut pt_regs) {
    void arch_uprobe_abort_xol(struct arch_uprobe *auprobe, struct pt_regs *regs)
    {
    struct uprobe_task *utask = current.utask;
    current.thread.bad_cause = utask.autask.saved_cause;
//
// Task has received a fatal signal, so reset back to probed
// address.
//
    instruction_pointer_set(regs, utask.vaddr);
    }
    bool arch_uretprobe_is_alive(struct return_instance *ret, enum rp_check ctx,
    struct pt_regs *regs)
    {
    if (ctx == RP_CHECK_CHAIN_CALL)
    return regs.sp <= ret.stack;
    else
    return regs.sp < ret.stack;
    }
    unsigned long
    arch_uretprobe_hijack_return_addr(unsigned long trampoline_vaddr,
    struct pt_regs *regs)
    {
    unsigned long ra;
    ra = regs.ra;
    regs.ra = trampoline_vaddr;
    return ra;
    }
    int arch_uprobe_exception_notify(struct notifier_block *self,
    unsigned long val, void *data)
    {
    return NOTIFY_DONE;
    }
#[no_mangle]
pub unsafe extern "C" fn uprobe_breakpoint_handler(regs: *mut pt_regs) -> bool {
    bool uprobe_breakpoint_handler(struct pt_regs *regs)
    {
    if (uprobe_pre_sstep_notifier(regs))
    return true;
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn uprobe_single_step_handler(regs: *mut pt_regs) -> bool {
    bool uprobe_single_step_handler(struct pt_regs *regs)
    {
    if (uprobe_post_sstep_notifier(regs))
    return true;
    return false;
    }
    void arch_uprobe_copy_ixol(struct page *page, unsigned long vaddr,
    void *src, unsigned long len)
    {
// Initialize the slot
    void *kaddr = kmap_local_page(page);
    void *dst = kaddr + (vaddr & ~PAGE_MASK);
    let mut start: c_ulong = (unsigned long)dst;
    memcpy(dst, src, len);
// Add ebreak behind opcode to simulate singlestep
    if (vaddr) {
    dst += GET_INSN_LENGTH(*(probe_opcode_t *)src);
// (uprobe_opcode_t *)dst = __BUG_INSN_32;
    }
    flush_icache_range(start, start + len);
    kunmap_local(kaddr);
    }
