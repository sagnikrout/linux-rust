//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/probes/uprobes.c
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
//
// Copyright (C) 2014-2016 Pratyush Anand <panand@redhat.com>
//

    void arch_uprobe_copy_ixol(struct page *page, unsigned long vaddr,
    void *src, unsigned long len)
    {
    void *xol_page_kaddr = kmap_local_page(page);
    void *dst = xol_page_kaddr + (vaddr & ~PAGE_MASK);
//
// Initial cache maintenance of the xol page done via set_pte_at().
// Subsequent CMOs only needed if the xol slot changes.
//
    if (!memcmp(dst, src, len))
    goto done;
// Initialize the slot
    memcpy(dst, src, len);
// flush caches (dcache/icache)
    sync_icache_aliases((unsigned long)dst, (unsigned long)dst + len);
    done:
    kunmap_local(xol_page_kaddr);
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
    u32 insn;
// TODO: Currently we do not support AARCH32 instruction probing
    if (mm.context.flags & MMCF_AARCH32)
    return -EOPNOTSUPP;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !IS_ALIGNED(addr, _arg: AARCH64_INSN_SIZE)) -> else {
    else if (!IS_ALIGNED(addr, AARCH64_INSN_SIZE))
    return -EINVAL;
    insn = le32_to_cpu(auprobe.insn);
    switch (arm_probe_decode_insn(insn, &auprobe.api)) {
    case INSN_REJECTED:
    return -EINVAL;
    case INSN_GOOD_NO_SLOT:
    auprobe.simulate = true;
    break;
    default:
    break;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_uprobe_pre_xol(auprobe: *mut arch_uprobe, regs: *mut pt_regs) -> c_int {
    int arch_uprobe_pre_xol(struct arch_uprobe *auprobe, struct pt_regs *regs)
    {
    struct uprobe_task *utask = current.utask;
// Initialize with an invalid fault code to detect if ol insn trapped
    current.thread.fault_code = UPROBE_INV_FAULT_CODE;
// Instruction points to execute ol
    instruction_pointer_set(regs, utask.xol_vaddr);
    user_enable_single_step(current);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_uprobe_post_xol(auprobe: *mut arch_uprobe, regs: *mut pt_regs) -> c_int {
    int arch_uprobe_post_xol(struct arch_uprobe *auprobe, struct pt_regs *regs)
    {
    struct uprobe_task *utask = current.utask;
    WARN_ON_ONCE(current.thread.fault_code != UPROBE_INV_FAULT_CODE);
// Instruction points to execute next to breakpoint address
    instruction_pointer_set(regs, utask.vaddr + 4);
    user_disable_single_step(current);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_uprobe_xol_was_trapped(t: *mut task_struct) -> bool {
    bool arch_uprobe_xol_was_trapped(struct task_struct *t)
    {
//
// Between arch_uprobe_pre_xol and arch_uprobe_post_xol, if an xol
// insn itself is trapped, then detect the case with the help of
// invalid fault code which is being set in arch_uprobe_pre_xol
//
    return t.thread.fault_code != UPROBE_INV_FAULT_CODE;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_uprobe_skip_sstep(auprobe: *mut arch_uprobe, regs: *mut pt_regs) -> bool {
    bool arch_uprobe_skip_sstep(struct arch_uprobe *auprobe, struct pt_regs *regs)
    {
    u32 insn;
    unsigned long addr;
    if (!auprobe.simulate)
    return false;
    insn = le32_to_cpu(auprobe.insn);
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
//
// Task has received a fatal signal, so reset back to probed
// address.
//
    instruction_pointer_set(regs, utask.vaddr);
    user_disable_single_step(current);
    }
    bool arch_uretprobe_is_alive(struct return_instance *ret, enum rp_check ctx,
    struct pt_regs *regs)
    {
//
// If a simple branch instruction (B) was called for retprobed
// assembly label then return true even when regs->sp and ret->stack
// are same. It will ensure that cleanup and reporting of return
// instances corresponding to callee label is done when
// handle_trampoline for called function is executed.
//
    if (ctx == RP_CHECK_CHAIN_CALL)
    return regs.sp <= ret.stack;
    else
    return regs.sp < ret.stack;
    }
    unsigned long
    arch_uretprobe_hijack_return_addr(unsigned long trampoline_vaddr,
    struct pt_regs *regs)
    {
    unsigned long orig_ret_vaddr;
    unsigned long gcs_ret_vaddr;
    let mut err: c_int = 0;
    u64 gcspr;
    orig_ret_vaddr = procedure_link_pointer(regs);
    if (task_gcs_el0_enabled(current)) {
    gcspr = read_sysreg_s(SYS_GCSPR_EL0);
    gcs_ret_vaddr = get_user_gcs(( unsigned long __user *)gcspr, &err);
    if (err) {
    force_sig(SIGSEGV);
    goto out;
    }
//
// If the LR and GCS return addr don't match, then some kind of PAC
// signing or control flow occurred since entering the probed function.
// Likely because the user is attempting to retprobe on an instruction
// that isn't a function boundary or inside a leaf function. Explicitly
// abort this retprobe because it will generate a GCS exception.
//
    if (gcs_ret_vaddr != orig_ret_vaddr) {
    orig_ret_vaddr = -1;
    goto out;
    }
    put_user_gcs(trampoline_vaddr, ( unsigned long __user *)gcspr, &err);
    if (err) {
    force_sig(SIGSEGV);
    goto out;
    }
    }
// Replace the return addr with trampoline addr
    procedure_link_pointer_set(regs, trampoline_vaddr);
    out:
    return orig_ret_vaddr;
    }
    int arch_uprobe_exception_notify(struct notifier_block *self,
    unsigned long val, void *data)
    {
    return NOTIFY_DONE;
    }
    int uprobe_brk_handler(struct pt_regs *regs,
    unsigned long esr)
    {
    if (uprobe_pre_sstep_notifier(regs))
    return DBG_HOOK_HANDLED;
    return DBG_HOOK_ERROR;
    }
    int uprobe_single_step_handler(struct pt_regs *regs,
    unsigned long esr)
    {
    struct uprobe_task *utask = current.utask;
    WARN_ON(utask && (instruction_pointer(regs) != utask.xol_vaddr + 4));
    if (uprobe_post_sstep_notifier(regs))
    return DBG_HOOK_HANDLED;
    return DBG_HOOK_ERROR;
    }
