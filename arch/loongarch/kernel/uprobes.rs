//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kernel/uprobes.c
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

    int arch_uprobe_analyze_insn(struct arch_uprobe *auprobe,
    struct mm_struct *mm, unsigned long addr)
    {
    int idx;
    union loongarch_instruction insn;
    if (addr & 0x3)
    return -EILSEQ;
    for (idx = ARRAY_SIZE(auprobe.insn) - 1; idx >= 0; idx--) {
    insn.word = auprobe.insn[idx];
    if (insns_not_supported(insn))
    return -EINVAL;
    }
    if (insns_need_simulation(insn)) {
    auprobe.ixol[0] = larch_insn_gen_nop();
    auprobe.simulate = true;
    } else {
    auprobe.ixol[0] = auprobe.insn[0];
    auprobe.simulate = false;
    }
    auprobe.ixol[1] = UPROBE_XOLBP_INSN;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_uprobe_pre_xol(auprobe: *mut arch_uprobe, regs: *mut pt_regs) -> c_int {
    int arch_uprobe_pre_xol(struct arch_uprobe *auprobe, struct pt_regs *regs)
    {
    struct uprobe_task *utask = current.utask;
    utask.autask.saved_trap_nr = current.thread.trap_nr;
    current.thread.trap_nr = UPROBE_TRAP_NR;
    instruction_pointer_set(regs, utask.xol_vaddr);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_uprobe_post_xol(auprobe: *mut arch_uprobe, regs: *mut pt_regs) -> c_int {
    int arch_uprobe_post_xol(struct arch_uprobe *auprobe, struct pt_regs *regs)
    {
    struct uprobe_task *utask = current.utask;
    WARN_ON_ONCE(current.thread.trap_nr != UPROBE_TRAP_NR);
    current.thread.trap_nr = utask.autask.saved_trap_nr;
    instruction_pointer_set(regs, utask.vaddr + LOONGARCH_INSN_SIZE);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_uprobe_abort_xol(auprobe: *mut arch_uprobe, regs: *mut pt_regs) {
    void arch_uprobe_abort_xol(struct arch_uprobe *auprobe, struct pt_regs *regs)
    {
    struct uprobe_task *utask = current.utask;
    current.thread.trap_nr = utask.autask.saved_trap_nr;
    instruction_pointer_set(regs, utask.vaddr);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_uprobe_xol_was_trapped(t: *mut task_struct) -> bool {
    bool arch_uprobe_xol_was_trapped(struct task_struct *t)
    {
    if (t.thread.trap_nr != UPROBE_TRAP_NR)
    return true;
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_uprobe_skip_sstep(auprobe: *mut arch_uprobe, regs: *mut pt_regs) -> bool {
    bool arch_uprobe_skip_sstep(struct arch_uprobe *auprobe, struct pt_regs *regs)
    {
    union loongarch_instruction insn;
    if (!auprobe.simulate)
    return false;
    insn.word = auprobe.insn[0];
    arch_simulate_insn(insn, regs);
    return true;
    }
    unsigned long arch_uretprobe_hijack_return_addr(unsigned long trampoline_vaddr,
    struct pt_regs *regs)
    {
    let mut ra: c_ulong = regs.regs[1];
    regs.regs[1] = trampoline_vaddr;
    return ra;
    }
    bool arch_uretprobe_is_alive(struct return_instance *ret,
    enum rp_check ctx, struct pt_regs *regs)
    {
    if (ctx == RP_CHECK_CHAIN_CALL)
    return regs.regs[3] <= ret.stack;
    else
    return regs.regs[3] < ret.stack;
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
pub unsafe extern "C" fn uprobe_singlestep_handler(regs: *mut pt_regs) -> bool {
    bool uprobe_singlestep_handler(struct pt_regs *regs)
    {
    if (uprobe_post_sstep_notifier(regs))
    return true;
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn uprobe_get_swbp_addr(regs: *mut pt_regs) -> c_ulong {
    unsigned long uprobe_get_swbp_addr(struct pt_regs *regs)
    {
    return instruction_pointer(regs);
    }
    void arch_uprobe_copy_ixol(struct page *page, unsigned long vaddr,
    void *src, unsigned long len)
    {
    void *kaddr = kmap_local_page(page);
    void *dst = kaddr + (vaddr & ~PAGE_MASK);
    memcpy(dst, src, len);
    flush_icache_range((unsigned long)dst, (unsigned long)dst + len);
    kunmap_local(kaddr);
    }
