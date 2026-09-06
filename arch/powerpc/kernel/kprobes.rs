//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/kprobes.c
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
// Kernel Probes (KProbes)
//
// Copyright (C) IBM Corporation, 2002, 2004
//
// 2002-Oct	Created by Vamsi Krishna S <vamsi_krishna@in.ibm.com> Kernel
// Probes initial implementation ( includes contributions from
// Rusty Russell).
// 2004-July	Suparna Bhattacharya <suparna@in.ibm.com> added jumper probes
// interface to access function arguments.
// 2004-Nov	Ananth N Mavinakayanahalli <ananth@in.ibm.com> kprobes port
// for PPC64
//

    DEFINE_PER_CPU(struct kprobe *, current_kprobe) = core::ptr::null_mut();
    DEFINE_PER_CPU(struct kprobe_ctlblk, kprobe_ctlblk);
    struct kretprobe_blackpoint kretprobe_blacklist[] = {{core::ptr::null_mut(), core::ptr::null_mut()}};
#[no_mangle]
pub unsafe extern "C" fn arch_within_kprobe_blacklist(addr: c_ulong) -> bool {
    bool arch_within_kprobe_blacklist(unsigned long addr)
    {
    return  (addr >= (unsigned long)__kprobes_text_start &&
    addr < (unsigned long)__kprobes_text_end) ||
    (addr >= (unsigned long)_stext &&
    addr < (unsigned long)__head_end);
    }
    kprobe_opcode_t *kprobe_lookup_name(const char *name, unsigned int offset)
    {
    kprobe_opcode_t *addr = core::ptr::null_mut();

// PPC64 ABIv2 needs local entry point
    addr = (kprobe_opcode_t *)kallsyms_lookup_name(name);
    if (addr && !offset) {

    unsigned long faddr;
//
// Per livepatch.h, ftrace location is always within the first
// 16 bytes of a function on powerpc with -mprofile-kernel.
//
    faddr = ftrace_location_range((unsigned long)addr,
    (unsigned long)addr + 16);
    if (faddr)
    addr = (kprobe_opcode_t *)faddr;
    else

    addr = (kprobe_opcode_t *)ppc_function_entry(addr);
    }

//
// 64bit powerpc ABIv1 uses function descriptors:
// - Check for the dot variant of the symbol first.
// - If that fails, try looking up the symbol provided.
//
// This ensures we always get to the actual symbol and not
// the descriptor.
//
// Also handle <module:symbol> format.
//
    char dot_name[MODULE_NAME_LEN + 1 + KSYM_NAME_LEN];
    let mut dot_appended: bool = false;
    const char *c;
    let mut ret: isize = 0;
    let mut len: c_int = 0;
    if ((c = strnchr(name, MODULE_NAME_LEN, ':')) != core::ptr::null_mut()) {
    c++;
    len = c - name;
    memcpy(dot_name, name, len);
    } else
    c = name;
    if (*c != '\0' && *c != '.') {
    dot_name[len++] = '.';
    dot_appended = true;
    }
    ret = strscpy(dot_name + len, c, KSYM_NAME_LEN);
    if (ret > 0)
    addr = (kprobe_opcode_t *)kallsyms_lookup_name(dot_name);
// Fallback to the original non-dot symbol lookup
    if (!addr && dot_appended)
    addr = (kprobe_opcode_t *)kallsyms_lookup_name(name);

    addr = (kprobe_opcode_t *)kallsyms_lookup_name(name);

    return addr;
    }
#[no_mangle]
unsafe extern "C" fn arch_kprobe_on_func_entry(addr: c_ulong, offset: c_ulong) -> bool {
    static bool arch_kprobe_on_func_entry(unsigned long addr, unsigned long offset)
    {
    let mut ip: c_ulong = ftrace_location(addr);
    if (ip)
    return offset <= (ip - addr);
    if (IS_ENABLED(CONFIG_PPC64_ELF_ABI_V2) && !IS_ENABLED(CONFIG_PPC_KERNEL_PCREL))
    return offset <= 8;
    return !offset;
    }
// XXX try and fold the magic of kprobe_lookup_name() in this
    kprobe_opcode_t *arch_adjust_kprobe_addr(unsigned long addr, unsigned long offset,
    bool *on_func_entry)
    {
// on_func_entry = arch_kprobe_on_func_entry(addr, offset);
    return (kprobe_opcode_t *)(addr + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_prepare_kprobe(p: *mut kprobe) -> c_int {
    int arch_prepare_kprobe(struct kprobe *p)
    {
    let mut ret: c_int = 0;
    struct kprobe *prev;
    let mut insn: ppc_inst_t = ppc_inst_read(p.addr);
    if ((unsigned long)p.addr & 0x03) {
    printk("Attempt to register kprobe at an unaligned address\n");
    ret = -EINVAL;
    } else if (!can_single_step(ppc_inst_val(insn))) {
    printk("Cannot register a kprobe on instructions that can't be single stepped\n");
    ret = -EINVAL;
    } else if ((unsigned long)p.addr & ~PAGE_MASK &&
    ppc_inst_prefixed(ppc_inst_read(p.addr - 1))) {
    printk("Cannot register a kprobe on the second word of prefixed instruction\n");
    ret = -EINVAL;
    }
    prev = get_kprobe(p.addr - 1);
//
// When prev is a ftrace-based kprobe, we don't have an insn, and it
// doesn't probe for prefixed instruction.
//
    if (prev && !kprobe_ftrace(prev) &&
    ppc_inst_prefixed(ppc_inst_read(prev.ainsn.insn))) {
    printk("Cannot register a kprobe on the second word of prefixed instruction\n");
    ret = -EINVAL;
    }
// insn must be on a special executable page on ppc64.  This is
// not explicitly required on ppc32 (right now), but it doesn't hurt
    if (!ret) {
    p.ainsn.insn = get_insn_slot();
    if (!p.ainsn.insn)
    ret = -ENOMEM;
    }
    if (!ret) {
    patch_instruction(p.ainsn.insn, insn);
    p.opcode = ppc_inst_val(insn);
    }
    p.ainsn.boostable = 0;
    return ret;
    }
    NOKPROBE_SYMBOL(arch_prepare_kprobe);
#[no_mangle]
pub unsafe extern "C" fn arch_arm_kprobe(p: *mut kprobe) {
    void arch_arm_kprobe(struct kprobe *p)
    {
    WARN_ON_ONCE(patch_instruction(p.addr, ppc_inst(BREAKPOINT_INSTRUCTION)));
    }
    NOKPROBE_SYMBOL(arch_arm_kprobe);
#[no_mangle]
pub unsafe extern "C" fn arch_disarm_kprobe(p: *mut kprobe) {
    void arch_disarm_kprobe(struct kprobe *p)
    {
    WARN_ON_ONCE(patch_instruction(p.addr, ppc_inst(p.opcode)));
    }
    NOKPROBE_SYMBOL(arch_disarm_kprobe);
#[no_mangle]
pub unsafe extern "C" fn arch_remove_kprobe(p: *mut kprobe) {
    void arch_remove_kprobe(struct kprobe *p)
    {
    if (p.ainsn.insn) {
    free_insn_slot(p.ainsn.insn, 0);
    p.ainsn.insn = core::ptr::null_mut();
    }
    }
    NOKPROBE_SYMBOL(arch_remove_kprobe);
#[no_mangle]
unsafe extern "C" fn prepare_singlestep(p: *mut kprobe, regs: *mut pt_regs) -> nokprobe_inline void {
    static nokprobe_inline void prepare_singlestep(struct kprobe *p, struct pt_regs *regs)
    {
    enable_single_step(regs);
//
// On powerpc we should single step on the original
// instruction even if the probed insn is a trap
// variant as values in regs could play a part in
// if the trap is taken or not
//
    regs_set_return_ip(regs, (unsigned long)p.ainsn.insn);
    }
#[no_mangle]
unsafe extern "C" fn save_previous_kprobe(kcb: *mut kprobe_ctlblk) -> nokprobe_inline void {
    static nokprobe_inline void save_previous_kprobe(struct kprobe_ctlblk *kcb)
    {
    kcb.prev_kprobe.kp = kprobe_running();
    kcb.prev_kprobe.status = kcb.kprobe_status;
    kcb.prev_kprobe.saved_msr = kcb.kprobe_saved_msr;
    }
#[no_mangle]
unsafe extern "C" fn restore_previous_kprobe(kcb: *mut kprobe_ctlblk) -> nokprobe_inline void {
    static nokprobe_inline void restore_previous_kprobe(struct kprobe_ctlblk *kcb)
    {
    __this_cpu_write(current_kprobe, kcb.prev_kprobe.kp);
    kcb.kprobe_status = kcb.prev_kprobe.status;
    kcb.kprobe_saved_msr = kcb.prev_kprobe.saved_msr;
    }
    static nokprobe_inline void set_current_kprobe(struct kprobe *p, struct pt_regs *regs,
    struct kprobe_ctlblk *kcb)
    {
    __this_cpu_write(current_kprobe, p);
    kcb.kprobe_saved_msr = regs.msr;
    }
#[no_mangle]
unsafe extern "C" fn try_to_emulate(p: *mut kprobe, regs: *mut pt_regs) -> c_int {
    static int try_to_emulate(struct kprobe *p, struct pt_regs *regs)
    {
    int ret;
    let mut insn: ppc_inst_t = ppc_inst_read(p.ainsn.insn);
// regs->nip is also adjusted if emulate_step returns 1
    ret = emulate_step(regs, insn);
    if (ret > 0) {
//
// Once this instruction has been boosted
// successfully, set the boostable flag
//
    if (unlikely(p.ainsn.boostable == 0))
    p.ainsn.boostable = 1;
    } else if (ret < 0) {
//
// We don't allow kprobes on mtmsr(d)/rfi(d), etc.
// So, we should never get here... but, its still
// good to catch them, just in case...
//
    printk("Can't step on instruction %08lx\n", ppc_inst_as_ulong(insn));
    BUG();
    } else {
//
// If we haven't previously emulated this instruction, then it
// can't be boosted. Note it down so we don't try to do so again.
//
// If, however, we had emulated this instruction in the past,
// then this is just an error with the current run (for
// instance, exceptions due to a load/store). We return 0 so
// that this is now single-stepped, but continue to try
// emulating it in subsequent probe hits.
//
    if (unlikely(p.ainsn.boostable != 1))
    p.ainsn.boostable = -1;
    }
    return ret;
    }
    NOKPROBE_SYMBOL(try_to_emulate);
#[no_mangle]
pub unsafe extern "C" fn kprobe_handler(regs: *mut pt_regs) -> c_int {
    int kprobe_handler(struct pt_regs *regs)
    {
    struct kprobe *p;
    let mut ret: c_int = 0;
    unsigned int *addr = (unsigned int *)regs.nip;
    struct kprobe_ctlblk *kcb;
    if (user_mode(regs))
    return 0;
    if (!IS_ENABLED(CONFIG_BOOKE) &&
    (!(regs.msr & MSR_IR) || !(regs.msr & MSR_DR)))
    return 0;
//
// We don't want to be preempted for the entire
// duration of kprobe processing
//
    preempt_disable();
    kcb = get_kprobe_ctlblk();
    p = get_kprobe(addr);
    if (!p) {
    unsigned int instr;
    if (get_kernel_nofault(instr, addr))
    goto no_kprobe;
    if (instr != BREAKPOINT_INSTRUCTION) {
//
// PowerPC has multiple variants of the "trap"
// instruction. If the current instruction is a
// trap variant, it could belong to someone else
//
    if (is_trap(instr))
    goto no_kprobe;
//
// The breakpoint instruction was removed right
// after we hit it.  Another cpu has removed
// either a probepoint or a debugger breakpoint
// at this address.  In either case, no further
// handling of this interrupt is appropriate.
//
    ret = 1;
    }
// Not one of ours: let kernel handle it
    goto no_kprobe;
    }
// Check we're not actually recursing
    if (kprobe_running()) {
    let mut insn: kprobe_opcode_t = *p.ainsn.insn;
    if (kcb.kprobe_status == KPROBE_HIT_SS && is_trap(insn)) {
// Turn off 'trace' bits
    regs_set_return_msr(regs,
    (regs.msr & ~MSR_SINGLESTEP) |
    kcb.kprobe_saved_msr);
    goto no_kprobe;
    }
//
// We have reentered the kprobe_handler(), since another probe
// was hit while within the handler. We here save the original
// kprobes variables and just single step on the instruction of
// the new probe without calling any user handlers.
//
    save_previous_kprobe(kcb);
    set_current_kprobe(p, regs, kcb);
    kprobes_inc_nmissed_count(p);
    kcb.kprobe_status = KPROBE_REENTER;
    if (p.ainsn.boostable >= 0) {
    ret = try_to_emulate(p, regs);
    if (ret > 0) {
    restore_previous_kprobe(kcb);
    preempt_enable();
    return 1;
    }
    }
    prepare_singlestep(p, regs);
    return 1;
    }
    kcb.kprobe_status = KPROBE_HIT_ACTIVE;
    set_current_kprobe(p, regs, kcb);
    if (p.pre_handler && p.pre_handler(p, regs)) {
// handler changed execution path, so skip ss setup
    reset_current_kprobe();
    preempt_enable();
    return 1;
    }
    if (p.ainsn.boostable >= 0) {
    ret = try_to_emulate(p, regs);
    if (ret > 0) {
    if (p.post_handler)
    p.post_handler(p, regs, 0);
    kcb.kprobe_status = KPROBE_HIT_SSDONE;
    reset_current_kprobe();
    preempt_enable();
    return 1;
    }
    }
    prepare_singlestep(p, regs);
    kcb.kprobe_status = KPROBE_HIT_SS;
    return 1;
    no_kprobe:
    preempt_enable();
    return ret;
    }
    NOKPROBE_SYMBOL(kprobe_handler);
//
// Called after single-stepping.  p->addr is the address of the
// instruction whose first byte has been replaced by the "breakpoint"
// instruction.  To avoid the SMP problems that can occur when we
// temporarily put back the original opcode to single-step, we
// single-stepped a copy of the instruction.  The address of this
// copy is p->ainsn.insn.
//
#[no_mangle]
pub unsafe extern "C" fn kprobe_post_handler(regs: *mut pt_regs) -> c_int {
    int kprobe_post_handler(struct pt_regs *regs)
    {
    int len;
    struct kprobe *cur = kprobe_running();
    struct kprobe_ctlblk *kcb = get_kprobe_ctlblk();
    if (!cur || user_mode(regs))
    return 0;
    len = ppc_inst_len(ppc_inst_read(cur.ainsn.insn));
// make sure we got here for instruction we have a kprobe on
    if (((unsigned long)cur.ainsn.insn + len) != regs.nip)
    return 0;
    if ((kcb.kprobe_status != KPROBE_REENTER) && cur.post_handler) {
    kcb.kprobe_status = KPROBE_HIT_SSDONE;
    cur.post_handler(cur, regs, 0);
    }
// Adjust nip to after the single-stepped instruction
    regs_set_return_ip(regs, (unsigned long)cur.addr + len);
    regs_set_return_msr(regs, regs.msr | kcb.kprobe_saved_msr);
// Restore back the original saved kprobes variables and continue.
    if (kcb.kprobe_status == KPROBE_REENTER) {
    restore_previous_kprobe(kcb);
    goto out;
    }
    reset_current_kprobe();
    out:
    preempt_enable();
//
// if somebody else is singlestepping across a probe point, msr
// will have DE/SE set, in which case, continue the remaining processing
// of do_debug, as if this is not a probe hit.
//
    if (regs.msr & MSR_SINGLESTEP)
    return 0;
    return 1;
    }
    NOKPROBE_SYMBOL(kprobe_post_handler);
#[no_mangle]
pub unsafe extern "C" fn kprobe_fault_handler(regs: *mut pt_regs, trapnr: c_int) -> c_int {
    int kprobe_fault_handler(struct pt_regs *regs, int trapnr)
    {
    struct kprobe *cur = kprobe_running();
    struct kprobe_ctlblk *kcb = get_kprobe_ctlblk();
    const struct exception_table_entry *entry;
    switch(kcb.kprobe_status) {
    case KPROBE_HIT_SS:
    case KPROBE_REENTER:
//
// We are here because the instruction being single
// stepped caused a page fault. We reset the current
// kprobe and the nip points back to the probe address
// and allow the page fault handler to continue as a
// normal page fault.
//
    regs_set_return_ip(regs, (unsigned long)cur.addr);
// Turn off 'trace' bits
    regs_set_return_msr(regs,
    (regs.msr & ~MSR_SINGLESTEP) |
    kcb.kprobe_saved_msr);
    if (kcb.kprobe_status == KPROBE_REENTER)
    restore_previous_kprobe(kcb);
    else
    reset_current_kprobe();
    preempt_enable();
    break;
    case KPROBE_HIT_ACTIVE:
    case KPROBE_HIT_SSDONE:
//
// In case the user-specified fault handler returned
// zero, try to fix up.
//
    if ((entry = search_exception_tables(regs.nip)) != core::ptr::null_mut()) {
    regs_set_return_ip(regs, extable_fixup(entry));
    return 1;
    }
//
// fixup_exception() could not handle it,
// Let do_page_fault() fix it.
//
    break;
    default:
    break;
    }
    return 0;
    }
    NOKPROBE_SYMBOL(kprobe_fault_handler);
#[no_mangle]
pub unsafe extern "C" fn arch_trampoline_kprobe(p: *mut kprobe) -> c_int {
    int arch_trampoline_kprobe(struct kprobe *p)
    {
    if (p.addr == (kprobe_opcode_t *)&arch_rethook_trampoline)
    return 1;
    return 0;
    }
    NOKPROBE_SYMBOL(arch_trampoline_kprobe);
