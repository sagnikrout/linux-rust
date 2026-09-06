//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/traps.c
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
// Based on arch/arm/kernel/traps.c
//
// Copyright (C) 1995-2009 Russell King
// Copyright (C) 2012 ARM Ltd.
//

#[no_mangle]
unsafe extern "C" fn __check_eq(pstate: c_ulong) -> bool __kprobes {
    static bool __kprobes __check_eq(unsigned long pstate)
    {
    return (pstate & PSR_Z_BIT) != 0;
    }
#[no_mangle]
unsafe extern "C" fn __check_ne(pstate: c_ulong) -> bool __kprobes {
    static bool __kprobes __check_ne(unsigned long pstate)
    {
    return (pstate & PSR_Z_BIT) == 0;
    }
#[no_mangle]
unsafe extern "C" fn __check_cs(pstate: c_ulong) -> bool __kprobes {
    static bool __kprobes __check_cs(unsigned long pstate)
    {
    return (pstate & PSR_C_BIT) != 0;
    }
#[no_mangle]
unsafe extern "C" fn __check_cc(pstate: c_ulong) -> bool __kprobes {
    static bool __kprobes __check_cc(unsigned long pstate)
    {
    return (pstate & PSR_C_BIT) == 0;
    }
#[no_mangle]
unsafe extern "C" fn __check_mi(pstate: c_ulong) -> bool __kprobes {
    static bool __kprobes __check_mi(unsigned long pstate)
    {
    return (pstate & PSR_N_BIT) != 0;
    }
#[no_mangle]
unsafe extern "C" fn __check_pl(pstate: c_ulong) -> bool __kprobes {
    static bool __kprobes __check_pl(unsigned long pstate)
    {
    return (pstate & PSR_N_BIT) == 0;
    }
#[no_mangle]
unsafe extern "C" fn __check_vs(pstate: c_ulong) -> bool __kprobes {
    static bool __kprobes __check_vs(unsigned long pstate)
    {
    return (pstate & PSR_V_BIT) != 0;
    }
#[no_mangle]
unsafe extern "C" fn __check_vc(pstate: c_ulong) -> bool __kprobes {
    static bool __kprobes __check_vc(unsigned long pstate)
    {
    return (pstate & PSR_V_BIT) == 0;
    }
#[no_mangle]
unsafe extern "C" fn __check_hi(pstate: c_ulong) -> bool __kprobes {
    static bool __kprobes __check_hi(unsigned long pstate)
    {
    pstate &= ~(pstate >> 1);	/* PSR_C_BIT &= ~PSR_Z_BIT */
    return (pstate & PSR_C_BIT) != 0;
    }
#[no_mangle]
unsafe extern "C" fn __check_ls(pstate: c_ulong) -> bool __kprobes {
    static bool __kprobes __check_ls(unsigned long pstate)
    {
    pstate &= ~(pstate >> 1);	/* PSR_C_BIT &= ~PSR_Z_BIT */
    return (pstate & PSR_C_BIT) == 0;
    }
#[no_mangle]
unsafe extern "C" fn __check_ge(pstate: c_ulong) -> bool __kprobes {
    static bool __kprobes __check_ge(unsigned long pstate)
    {
    pstate ^= (pstate << 3);	/* PSR_N_BIT ^= PSR_V_BIT */
    return (pstate & PSR_N_BIT) == 0;
    }
#[no_mangle]
unsafe extern "C" fn __check_lt(pstate: c_ulong) -> bool __kprobes {
    static bool __kprobes __check_lt(unsigned long pstate)
    {
    pstate ^= (pstate << 3);	/* PSR_N_BIT ^= PSR_V_BIT */
    return (pstate & PSR_N_BIT) != 0;
    }
#[no_mangle]
unsafe extern "C" fn __check_gt(pstate: c_ulong) -> bool __kprobes {
    static bool __kprobes __check_gt(unsigned long pstate)
    {
// PSR_N_BIT ^= PSR_V_BIT
    let mut temp: c_ulong = pstate ^ (pstate << 3);
    temp |= (pstate << 1);	/*PSR_N_BIT |= PSR_Z_BIT */
    return (temp & PSR_N_BIT) == 0;
    }
#[no_mangle]
unsafe extern "C" fn __check_le(pstate: c_ulong) -> bool __kprobes {
    static bool __kprobes __check_le(unsigned long pstate)
    {
// PSR_N_BIT ^= PSR_V_BIT
    let mut temp: c_ulong = pstate ^ (pstate << 3);
    temp |= (pstate << 1);	/*PSR_N_BIT |= PSR_Z_BIT */
    return (temp & PSR_N_BIT) != 0;
    }
#[no_mangle]
unsafe extern "C" fn __check_al(pstate: c_ulong) -> bool __kprobes {
    static bool __kprobes __check_al(unsigned long pstate)
    {
    return true;
    }
//
// Note that the ARMv8 ARM calls condition code 0b1111 "nv", but states that
// it behaves identically to 0b1110 ("al").
//
    pstate_check_t * const aarch32_opcode_cond_checks[16] = {
    __check_eq, __check_ne, __check_cs, __check_cc,
    __check_mi, __check_pl, __check_vs, __check_vc,
    __check_hi, __check_ls, __check_ge, __check_lt,
    __check_gt, __check_le, __check_al, __check_al
    };
    let mut show_unhandled_signals: c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn dump_kernel_instr(kaddr: c_ulong) {
    void dump_kernel_instr(unsigned long kaddr)
    {
    char str[sizeof("00000000 ") * 5 + 2 + 1], *p = str;
    int i;
    if (!is_ttbr1_addr(kaddr))
    return;
    for (i = -4; i < 1; i++) {
    unsigned int val, bad;
    bad = aarch64_insn_read(&((u32 *)kaddr)[i], &val);
    if (!bad)
    p += sprintf(p, i == 0 ? "(%08x) " : "%08x ", val);
    else
    p += sprintf(p, i == 0 ? "(????????) " : "???????? ");
    }
    printk(KERN_EMERG "Code: %s\n", str);
    }

#[no_mangle]
unsafe extern "C" fn __die(str: *const c_char, err: c_long, regs: *mut pt_regs) -> c_int {
    static int __die(const char *str, long err, struct pt_regs *regs)
    {
    static int die_counter;
    int ret;
    let mut addr: c_ulong = instruction_pointer(regs);
    pr_emerg("Internal error: %s: %016lx [#%d] " S_SMP "\n",
    str, err, ++die_counter);
// trap and error numbers are mostly meaningless on ARM
    ret = notify_die(DIE_OOPS, str, regs, err, 0, SIGSEGV);
    if (ret == NOTIFY_STOP)
    return ret;
    print_modules();
    show_regs(regs);
    if (user_mode(regs))
    return ret;
    dump_kernel_instr(addr);
    return ret;
    }
    static DEFINE_RAW_SPINLOCK(die_lock);
//
// This function is protected against re-entrancy.
//
#[no_mangle]
pub unsafe extern "C" fn die(str: *const c_char, regs: *mut pt_regs, err: c_long) {
    void die(const char *str, struct pt_regs *regs, long err)
    {
    int ret;
    unsigned long flags;
    raw_spin_lock_irqsave(&die_lock, flags);
    oops_enter();
    console_verbose();
    bust_spinlocks(1);
    ret = __die(str, err, regs);
    if (regs && kexec_should_crash(current))
    crash_kexec(regs);
    bust_spinlocks(0);
    add_taint(TAINT_DIE, LOCKDEP_NOW_UNRELIABLE);
    oops_exit();
    if (in_interrupt())
    panic("%s: Fatal exception in interrupt", str);
    if (panic_on_oops)
    panic("%s: Fatal exception", str);
    raw_spin_unlock_irqrestore(&die_lock, flags);
    if (ret != NOTIFY_STOP)
    make_task_dead(SIGSEGV);
    }
#[no_mangle]
unsafe extern "C" fn arm64_show_signal(signo: c_int, str: *const c_char) {
    static void arm64_show_signal(int signo, const char *str)
    {
    static DEFINE_RATELIMIT_STATE(rs, DEFAULT_RATELIMIT_INTERVAL,
    DEFAULT_RATELIMIT_BURST);
    struct task_struct *tsk = current;
    let mut esr: c_ulong = tsk.thread.fault_code;
    struct pt_regs *regs = task_pt_regs(tsk);
// Leave if the signal won't be shown
    if (!show_unhandled_signals ||
    !unhandled_signal(tsk, signo) ||
    !__ratelimit(&rs))
    return;
    pr_info("%s[%d]: unhandled exception: ", tsk.comm, task_pid_nr(tsk));
    if (esr)
    pr_cont("%s, ESR 0x%016lx, ", esr_get_class_string(esr), esr);
    pr_cont("%s", str);
    print_vma_addr(KERN_CONT " in ", regs.pc);
    pr_cont("\n");
    __show_regs(regs);
    }
    void arm64_force_sig_fault(int signo, int code, unsigned long far,
    const char *str)
    {
    arm64_show_signal(signo, str);
    if (signo == SIGKILL)
    force_sig(SIGKILL);
    else
    force_sig_fault(signo, code, (void __user *)far);
    }
#[no_mangle]
pub unsafe extern "C" fn arm64_force_sig_fault_pkey(far: c_ulong, str: *const c_char, pkey: c_int) {
    void arm64_force_sig_fault_pkey(unsigned long far, const char *str, int pkey)
    {
    arm64_show_signal(SIGSEGV, str);
    force_sig_pkuerr((void __user *)far, pkey);
    }
    void arm64_force_sig_mceerr(int code, unsigned long far, short lsb,
    const char *str)
    {
    arm64_show_signal(SIGBUS, str);
    force_sig_mceerr(code, (void __user *)far, lsb);
    }
    void arm64_force_sig_ptrace_errno_trap(int errno, unsigned long far,
    const char *str)
    {
    arm64_show_signal(SIGTRAP, str);
    force_sig_ptrace_errno_trap(errno, (void __user *)far);
    }
    void arm64_notify_die(const char *str, struct pt_regs *regs,
    int signo, int sicode, unsigned long far,
    unsigned long err)
    {
    if (user_mode(regs)) {
    WARN_ON(regs != current_pt_regs());
    current.thread.fault_address = 0;
    current.thread.fault_code = err;
    arm64_force_sig_fault(signo, sicode, far, str);
    } else {
    die(str, regs, err);
    }
    }

pub const PSTATE_IT_1_0_SHIFT: c_int = 25;

pub const PSTATE_IT_7_2_SHIFT: c_int = 10;

#[no_mangle]
unsafe extern "C" fn compat_get_it_state(regs: *mut pt_regs) -> u32 {
    static u32 compat_get_it_state(struct pt_regs *regs)
    {
    u32 it, pstate = regs.pstate;
    it  = (pstate & PSTATE_IT_1_0_MASK) >> PSTATE_IT_1_0_SHIFT;
    it |= ((pstate & PSTATE_IT_7_2_MASK) >> PSTATE_IT_7_2_SHIFT) << 2;
    return it;
    }
#[no_mangle]
unsafe extern "C" fn compat_set_it_state(regs: *mut pt_regs, it: u32) {
    static void compat_set_it_state(struct pt_regs *regs, u32 it)
    {
    u32 pstate_it;
    pstate_it  = (it << PSTATE_IT_1_0_SHIFT) & PSTATE_IT_1_0_MASK;
    pstate_it |= ((it >> 2) << PSTATE_IT_7_2_SHIFT) & PSTATE_IT_7_2_MASK;
    regs.pstate &= ~PSR_AA32_IT_MASK;
    regs.pstate |= pstate_it;
    }
#[no_mangle]
unsafe extern "C" fn advance_itstate(regs: *mut pt_regs) {
    static void advance_itstate(struct pt_regs *regs)
    {
    u32 it;
// ARM mode
    if (!(regs.pstate & PSR_AA32_T_BIT) ||
    !(regs.pstate & PSR_AA32_IT_MASK))
    return;
    it  = compat_get_it_state(regs);
//
// If this is the last instruction of the block, wipe the IT
// state. Otherwise advance it.
//
    if (!(it & 7))
    it = 0;
    else
    it = (it & 0xe0) | ((it << 1) & 0x1f);
    compat_set_it_state(regs, it);
    }

#[no_mangle]
unsafe extern "C" fn advance_itstate(regs: *mut pt_regs) {
    static void advance_itstate(struct pt_regs *regs)
    {
    }

#[no_mangle]
pub unsafe extern "C" fn arm64_skip_faulting_instruction(regs: *mut pt_regs, size: c_ulong) {
    void arm64_skip_faulting_instruction(struct pt_regs *regs, unsigned long size)
    {
    regs.pc += size;
//
// If we were single stepping, we want to get the step exception after
// we return from the trap.
//
    if (user_mode(regs))
    user_fastforward_single_step(current);
    if (compat_user_mode(regs))
    advance_itstate(regs);
    else
    regs.pstate &= ~PSR_BTYPE_MASK;
    }
#[no_mangle]
unsafe extern "C" fn user_insn_read(regs: *mut pt_regs, insnp: *mut u32) -> c_int {
    static int user_insn_read(struct pt_regs *regs, u32 *insnp)
    {
    u32 instr;
    let mut pc: c_ulong = instruction_pointer(regs);
    if (compat_thumb_mode(regs)) {
// 16-bit Thumb instruction
    __le16 instr_le;
    if (get_user(instr_le, (__le16 __user *)pc))
    return -EFAULT;
    instr = le16_to_cpu(instr_le);
    if (aarch32_insn_is_wide(instr)) {
    u32 instr2;
    if (get_user(instr_le, (__le16 __user *)(pc + 2)))
    return -EFAULT;
    instr2 = le16_to_cpu(instr_le);
    instr = (instr << 16) | instr2;
    }
    } else {
// 32-bit ARM instruction
    __le32 instr_le;
    if (get_user(instr_le, (__le32 __user *)pc))
    return -EFAULT;
    instr = le32_to_cpu(instr_le);
    }
// insnp = instr;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn force_signal_inject(signal: c_int, code: c_int, address: c_ulong, err: c_ulong) {
    void force_signal_inject(int signal, int code, unsigned long address, unsigned long err)
    {
    const char *desc;
    struct pt_regs *regs = current_pt_regs();
    if (WARN_ON(!user_mode(regs)))
    return;
    switch (signal) {
    case SIGILL:
    desc = "undefined instruction";
    break;
    case SIGSEGV:
    desc = "illegal memory access";
    break;
    default:
    desc = "unknown or unrecoverable error";
    break;
    }
// Force signals we don't understand to SIGKILL
    if (WARN_ON(signal != SIGKILL &&
    siginfo_layout(signal, code) != SIL_FAULT)) {
    signal = SIGKILL;
    }
    arm64_notify_die(desc, regs, signal, code, address, err);
    }
//
// Set up process info to signal segmentation fault - called on access error.
//
#[no_mangle]
pub unsafe extern "C" fn arm64_notify_segfault(addr: c_ulong) {
    void arm64_notify_segfault(unsigned long addr)
    {
    int code;
    mmap_read_lock(current.mm);
    if (find_vma(current.mm, untagged_addr(addr)) == core::ptr::null_mut())
    code = SEGV_MAPERR;
    else
    code = SEGV_ACCERR;
    mmap_read_unlock(current.mm);
    force_signal_inject(SIGSEGV, code, addr, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn do_el0_undef(regs: *mut pt_regs, esr: c_ulong) {
    void do_el0_undef(struct pt_regs *regs, unsigned long esr)
    {
    u32 insn;
// check for AArch32 breakpoint instructions
    if (try_handle_aarch32_break(regs))
    return;
    if (user_insn_read(regs, &insn))
    goto out_err;
    if (try_emulate_mrs(regs, insn))
    return;
    if (try_emulate_armv8_deprecated(regs, insn))
    return;
    out_err:
    force_signal_inject(SIGILL, ILL_ILLOPC, regs.pc, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn do_el1_undef(regs: *mut pt_regs, esr: c_ulong) {
    void do_el1_undef(struct pt_regs *regs, unsigned long esr)
    {
    u32 insn;
    if (aarch64_insn_read((void *)regs.pc, &insn))
    goto out_err;
    if (try_emulate_el1_ssbs(regs, insn))
    return;
    out_err:
    die("Oops - Undefined instruction", regs, esr);
    }
#[no_mangle]
pub unsafe extern "C" fn do_el0_bti(regs: *mut pt_regs) {
    void do_el0_bti(struct pt_regs *regs)
    {
    force_signal_inject(SIGILL, ILL_ILLOPC, regs.pc, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn do_el1_bti(regs: *mut pt_regs, esr: c_ulong) {
    void do_el1_bti(struct pt_regs *regs, unsigned long esr)
    {
    if (efi_runtime_fixup_exception(regs, "BTI violation")) {
    regs.pstate &= ~PSR_BTYPE_MASK;
    return;
    }
    die("Oops - BTI", regs, esr);
    }
#[no_mangle]
pub unsafe extern "C" fn do_el0_gcs(regs: *mut pt_regs, esr: c_ulong) {
    void do_el0_gcs(struct pt_regs *regs, unsigned long esr)
    {
    force_signal_inject(SIGSEGV, SEGV_CPERR, regs.pc, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn do_el1_gcs(regs: *mut pt_regs, esr: c_ulong) {
    void do_el1_gcs(struct pt_regs *regs, unsigned long esr)
    {
    die("Oops - GCS", regs, esr);
    }
#[no_mangle]
pub unsafe extern "C" fn do_el0_fpac(regs: *mut pt_regs, esr: c_ulong) {
    void do_el0_fpac(struct pt_regs *regs, unsigned long esr)
    {
    force_signal_inject(SIGILL, ILL_ILLOPN, regs.pc, esr);
    }
#[no_mangle]
pub unsafe extern "C" fn do_el1_fpac(regs: *mut pt_regs, esr: c_ulong) {
    void do_el1_fpac(struct pt_regs *regs, unsigned long esr)
    {
//
// Unexpected FPAC exception in the kernel: kill the task before it
// does any more harm.
//
    die("Oops - FPAC", regs, esr);
    }
#[no_mangle]
pub unsafe extern "C" fn do_el0_mops(regs: *mut pt_regs, esr: c_ulong) {
    void do_el0_mops(struct pt_regs *regs, unsigned long esr)
    {
    arm64_mops_reset_regs(&regs.user_regs, esr);
//
// If single stepping then finish the step before executing the
// prologue instruction.
//
    user_fastforward_single_step(current);
    }
#[no_mangle]
pub unsafe extern "C" fn do_el1_mops(regs: *mut pt_regs, esr: c_ulong) {
    void do_el1_mops(struct pt_regs *regs, unsigned long esr)
    {
    arm64_mops_reset_regs(&regs.user_regs, esr);
    kernel_fastforward_single_step(regs);
    }

    if (address >= TASK_SIZE_MAX) {				\
    res = -EFAULT;					\
    } else {						\
    uaccess_ttbr0_enable();				\
    asm volatile (					\
    "1:	" insn ", %1\n"			\
    "	mov	%w0, #0\n"		\
    "2:\n"					\
    _ASM_EXTABLE_UACCESS_ERR(1b, 2b, %w0)	\
    : "=r" (res)				\
    : "r" (address));			\
    uaccess_ttbr0_disable();			\
    }
#[no_mangle]
unsafe extern "C" fn user_cache_maint_handler(esr: c_ulong, regs: *mut pt_regs) {
    static void user_cache_maint_handler(unsigned long esr, struct pt_regs *regs)
    {
    unsigned long tagged_address, address;
    let mut rt: c_int = ESR_ELx_SYS64_ISS_RT(esr);
    let mut crm: c_int = (esr & ESR_ELx_SYS64_ISS_CRM_MASK) >> ESR_ELx_SYS64_ISS_CRM_SHIFT;
    let mut ret: c_int = 0;
    tagged_address = pt_regs_read_reg(regs, rt);
    address = untagged_addr(tagged_address);
    switch (crm) {
    case ESR_ELx_SYS64_ISS_CRM_DC_CVAU:	/* DC CVAU, gets promoted */
    __user_cache_maint("dc civac", address, ret);
    break;
    case ESR_ELx_SYS64_ISS_CRM_DC_CVAC:	/* DC CVAC, gets promoted */
    __user_cache_maint("dc civac", address, ret);
    break;
    case ESR_ELx_SYS64_ISS_CRM_DC_CVADP:	/* DC CVADP */
    __user_cache_maint("sys 3, c7, c13, 1", address, ret);
    break;
    case ESR_ELx_SYS64_ISS_CRM_DC_CVAP:	/* DC CVAP */
    __user_cache_maint("sys 3, c7, c12, 1", address, ret);
    break;
    case ESR_ELx_SYS64_ISS_CRM_DC_CIVAC:	/* DC CIVAC */
    __user_cache_maint("dc civac", address, ret);
    break;
    case ESR_ELx_SYS64_ISS_CRM_IC_IVAU:	/* IC IVAU */
    __user_cache_maint("ic ivau", address, ret);
    break;
    default:
    force_signal_inject(SIGILL, ILL_ILLOPC, regs.pc, 0);
    return;
    }
    if (ret)
    arm64_notify_segfault(tagged_address);
    else
    arm64_skip_faulting_instruction(regs, AARCH64_INSN_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn ctr_read_handler(esr: c_ulong, regs: *mut pt_regs) {
    static void ctr_read_handler(unsigned long esr, struct pt_regs *regs)
    {
    let mut rt: c_int = ESR_ELx_SYS64_ISS_RT(esr);
    let mut val: c_ulong = arm64_ftr_reg_user_value(&arm64_ftr_reg_ctrel0);
    if (cpus_have_final_cap(ARM64_WORKAROUND_1542419)) {
// Hide DIC so that we can trap the unnecessary maintenance...
    val &= ~BIT(CTR_EL0_DIC_SHIFT);
// ... and fake IminLine to reduce the number of traps.
    val &= ~CTR_EL0_IminLine_MASK;
    val |= (PAGE_SHIFT - 2) & CTR_EL0_IminLine_MASK;
    }
    pt_regs_write_reg(regs, rt, val);
    arm64_skip_faulting_instruction(regs, AARCH64_INSN_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn cntvct_read_handler(esr: c_ulong, regs: *mut pt_regs) {
    static void cntvct_read_handler(unsigned long esr, struct pt_regs *regs)
    {
    if (test_thread_flag(TIF_TSC_SIGSEGV)) {
    force_sig(SIGSEGV);
    } else {
    let mut rt: c_int = ESR_ELx_SYS64_ISS_RT(esr);
    pt_regs_write_reg(regs, rt, arch_timer_read_counter());
    arm64_skip_faulting_instruction(regs, AARCH64_INSN_SIZE);
    }
    }
#[no_mangle]
unsafe extern "C" fn cntfrq_read_handler(esr: c_ulong, regs: *mut pt_regs) {
    static void cntfrq_read_handler(unsigned long esr, struct pt_regs *regs)
    {
    if (test_thread_flag(TIF_TSC_SIGSEGV)) {
    force_sig(SIGSEGV);
    } else {
    let mut rt: c_int = ESR_ELx_SYS64_ISS_RT(esr);
    pt_regs_write_reg(regs, rt, arch_timer_get_rate());
    arm64_skip_faulting_instruction(regs, AARCH64_INSN_SIZE);
    }
    }
#[no_mangle]
unsafe extern "C" fn mrs_handler(esr: c_ulong, regs: *mut pt_regs) {
    static void mrs_handler(unsigned long esr, struct pt_regs *regs)
    {
    u32 sysreg, rt;
    rt = ESR_ELx_SYS64_ISS_RT(esr);
    sysreg = esr_sys64_to_sysreg(esr);
    if (do_emulate_mrs(regs, sysreg, rt) != 0)
    force_signal_inject(SIGILL, ILL_ILLOPC, regs.pc, 0);
    }
#[no_mangle]
unsafe extern "C" fn wfi_handler(esr: c_ulong, regs: *mut pt_regs) {
    static void wfi_handler(unsigned long esr, struct pt_regs *regs)
    {
    arm64_skip_faulting_instruction(regs, AARCH64_INSN_SIZE);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sys64_hook {
    pub esr_mask: c_ulong,
    pub esr_val: c_ulong,
    pub regs): *mut *mut void (handler)(unsigned long esr, struct pt_regs,
}

    static const struct sys64_hook sys64_hooks[] = {
    {
    .esr_mask = ESR_ELx_SYS64_ISS_EL0_CACHE_OP_MASK,
    .esr_val = ESR_ELx_SYS64_ISS_EL0_CACHE_OP_VAL,
    .handler = user_cache_maint_handler,
    },
    {
// Trap read access to CTR_EL0
    .esr_mask = ESR_ELx_SYS64_ISS_SYS_OP_MASK,
    .esr_val = ESR_ELx_SYS64_ISS_SYS_CTR_READ,
    .handler = ctr_read_handler,
    },
    {
// Trap read access to CNTVCT_EL0
    .esr_mask = ESR_ELx_SYS64_ISS_SYS_OP_MASK,
    .esr_val = ESR_ELx_SYS64_ISS_SYS_CNTVCT,
    .handler = cntvct_read_handler,
    },
    {
// Trap read access to CNTVCTSS_EL0
    .esr_mask = ESR_ELx_SYS64_ISS_SYS_OP_MASK,
    .esr_val = ESR_ELx_SYS64_ISS_SYS_CNTVCTSS,
    .handler = cntvct_read_handler,
    },
    {
// Trap read access to CNTFRQ_EL0
    .esr_mask = ESR_ELx_SYS64_ISS_SYS_OP_MASK,
    .esr_val = ESR_ELx_SYS64_ISS_SYS_CNTFRQ,
    .handler = cntfrq_read_handler,
    },
    {
// Trap read access to CPUID registers
    .esr_mask = ESR_ELx_SYS64_ISS_SYS_MRS_OP_MASK,
    .esr_val = ESR_ELx_SYS64_ISS_SYS_MRS_OP_VAL,
    .handler = mrs_handler,
    },
    {
// Trap WFI instructions executed in userspace
    .esr_mask = ESR_ELx_WFx_MASK,
    .esr_val = ESR_ELx_WFx_WFI_VAL,
    .handler = wfi_handler,
    },
    {},
    };

#[no_mangle]
unsafe extern "C" fn cp15_cond_valid(esr: c_ulong, regs: *mut pt_regs) -> bool {
    static bool cp15_cond_valid(unsigned long esr, struct pt_regs *regs)
    {
    int cond;
// Only a T32 instruction can trap without CV being set
    if (!(esr & ESR_ELx_CV)) {
    u32 it;
    it = compat_get_it_state(regs);
    if (!it)
    return true;
    cond = it >> 4;
    } else {
    cond = (esr & ESR_ELx_COND_MASK) >> ESR_ELx_COND_SHIFT;
    }
    return aarch32_opcode_cond_checks[cond](regs.pstate);
    }
#[no_mangle]
unsafe extern "C" fn compat_cntfrq_read_handler(esr: c_ulong, regs: *mut pt_regs) {
    static void compat_cntfrq_read_handler(unsigned long esr, struct pt_regs *regs)
    {
    let mut reg: c_int = (esr & ESR_ELx_CP15_32_ISS_RT_MASK) >> ESR_ELx_CP15_32_ISS_RT_SHIFT;
    pt_regs_write_reg(regs, reg, arch_timer_get_rate());
    arm64_skip_faulting_instruction(regs, 4);
    }
    static const struct sys64_hook cp15_32_hooks[] = {
    {
    .esr_mask = ESR_ELx_CP15_32_ISS_SYS_MASK,
    .esr_val = ESR_ELx_CP15_32_ISS_SYS_CNTFRQ,
    .handler = compat_cntfrq_read_handler,
    },
    {},
    };
#[no_mangle]
unsafe extern "C" fn compat_cntvct_read_handler(esr: c_ulong, regs: *mut pt_regs) {
    static void compat_cntvct_read_handler(unsigned long esr, struct pt_regs *regs)
    {
    let mut rt: c_int = (esr & ESR_ELx_CP15_64_ISS_RT_MASK) >> ESR_ELx_CP15_64_ISS_RT_SHIFT;
    let mut rt2: c_int = (esr & ESR_ELx_CP15_64_ISS_RT2_MASK) >> ESR_ELx_CP15_64_ISS_RT2_SHIFT;
    let mut val: u64 = arch_timer_read_counter();
    pt_regs_write_reg(regs, rt, lower_32_bits(val));
    pt_regs_write_reg(regs, rt2, upper_32_bits(val));
    arm64_skip_faulting_instruction(regs, 4);
    }
    static const struct sys64_hook cp15_64_hooks[] = {
    {
    .esr_mask = ESR_ELx_CP15_64_ISS_SYS_MASK,
    .esr_val = ESR_ELx_CP15_64_ISS_SYS_CNTVCT,
    .handler = compat_cntvct_read_handler,
    },
    {
    .esr_mask = ESR_ELx_CP15_64_ISS_SYS_MASK,
    .esr_val = ESR_ELx_CP15_64_ISS_SYS_CNTVCTSS,
    .handler = compat_cntvct_read_handler,
    },
    {},
    };
#[no_mangle]
pub unsafe extern "C" fn do_el0_cp15(esr: c_ulong, regs: *mut pt_regs) {
    void do_el0_cp15(unsigned long esr, struct pt_regs *regs)
    {
    const struct sys64_hook *hook, *hook_base;
    if (!cp15_cond_valid(esr, regs)) {
//
// There is no T16 variant of a CP access, so we
// always advance PC by 4 bytes.
//
    arm64_skip_faulting_instruction(regs, 4);
    return;
    }
    switch (ESR_ELx_EC(esr)) {
    case ESR_ELx_EC_CP15_32:
    hook_base = cp15_32_hooks;
    break;
    case ESR_ELx_EC_CP15_64:
    hook_base = cp15_64_hooks;
    break;
    default:
    do_el0_undef(regs, esr);
    return;
    }
    for (hook = hook_base; hook.handler; hook++)
    if ((hook.esr_mask & esr) == hook.esr_val) {
    hook.handler(esr, regs);
    return;
    }
//
// New cp15 instructions may previously have been undefined at
// EL0. Fall back to our usual undefined instruction handler
// so that we handle these consistently.
//
    do_el0_undef(regs, esr);
    }

#[no_mangle]
pub unsafe extern "C" fn do_el0_sys(esr: c_ulong, regs: *mut pt_regs) {
    void do_el0_sys(unsigned long esr, struct pt_regs *regs)
    {
    const struct sys64_hook *hook;
    for (hook = sys64_hooks; hook.handler; hook++)
    if ((hook.esr_mask & esr) == hook.esr_val) {
    hook.handler(esr, regs);
    return;
    }
//
// New SYS instructions may previously have been undefined at EL0. Fall
// back to our usual undefined instruction handler so that we handle
// these consistently.
//
    do_el0_undef(regs, esr);
    }
    static const char *esr_class_str[] = {
    [0 ... ESR_ELx_EC_MAX]		= "UNRECOGNIZED EC",
    [ESR_ELx_EC_UNKNOWN]		= "Unknown/Uncategorized",
    [ESR_ELx_EC_WFx]		= "WFI/WFE",
    [ESR_ELx_EC_CP15_32]		= "CP15 MCR/MRC",
    [ESR_ELx_EC_CP15_64]		= "CP15 MCRR/MRRC",
    [ESR_ELx_EC_CP14_MR]		= "CP14 MCR/MRC",
    [ESR_ELx_EC_CP14_LS]		= "CP14 LDC/STC",
    [ESR_ELx_EC_FP_ASIMD]		= "ASIMD",
    [ESR_ELx_EC_CP10_ID]		= "CP10 MRC/VMRS",
    [ESR_ELx_EC_PAC]		= "PAC",
    [ESR_ELx_EC_CP14_64]		= "CP14 MCRR/MRRC",
    [ESR_ELx_EC_BTI]		= "BTI",
    [ESR_ELx_EC_ILL]		= "PSTATE.IL",
    [ESR_ELx_EC_SVC32]		= "SVC (AArch32)",
    [ESR_ELx_EC_HVC32]		= "HVC (AArch32)",
    [ESR_ELx_EC_SMC32]		= "SMC (AArch32)",
    [ESR_ELx_EC_SVC64]		= "SVC (AArch64)",
    [ESR_ELx_EC_HVC64]		= "HVC (AArch64)",
    [ESR_ELx_EC_SMC64]		= "SMC (AArch64)",
    [ESR_ELx_EC_SYS64]		= "MSR/MRS (AArch64)",
    [ESR_ELx_EC_SVE]		= "SVE",
    [ESR_ELx_EC_ERET]		= "ERET/ERETAA/ERETAB",
    [ESR_ELx_EC_FPAC]		= "FPAC",
    [ESR_ELx_EC_SME]		= "SME",
    [ESR_ELx_EC_IMP_DEF]		= "EL3 IMP DEF",
    [ESR_ELx_EC_IABT_LOW]		= "IABT (lower EL)",
    [ESR_ELx_EC_IABT_CUR]		= "IABT (current EL)",
    [ESR_ELx_EC_PC_ALIGN]		= "PC Alignment",
    [ESR_ELx_EC_DABT_LOW]		= "DABT (lower EL)",
    [ESR_ELx_EC_DABT_CUR]		= "DABT (current EL)",
    [ESR_ELx_EC_SP_ALIGN]		= "SP Alignment",
    [ESR_ELx_EC_MOPS]		= "MOPS",
    [ESR_ELx_EC_FP_EXC32]		= "FP (AArch32)",
    [ESR_ELx_EC_FP_EXC64]		= "FP (AArch64)",
    [ESR_ELx_EC_GCS]		= "Guarded Control Stack",
    [ESR_ELx_EC_SERROR]		= "SError",
    [ESR_ELx_EC_BREAKPT_LOW]	= "Breakpoint (lower EL)",
    [ESR_ELx_EC_BREAKPT_CUR]	= "Breakpoint (current EL)",
    [ESR_ELx_EC_SOFTSTP_LOW]	= "Software Step (lower EL)",
    [ESR_ELx_EC_SOFTSTP_CUR]	= "Software Step (current EL)",
    [ESR_ELx_EC_WATCHPT_LOW]	= "Watchpoint (lower EL)",
    [ESR_ELx_EC_WATCHPT_CUR]	= "Watchpoint (current EL)",
    [ESR_ELx_EC_BKPT32]		= "BKPT (AArch32)",
    [ESR_ELx_EC_VECTOR32]		= "Vector catch (AArch32)",
    [ESR_ELx_EC_BRK64]		= "BRK (AArch64)",
    };
    const char *esr_get_class_string(unsigned long esr)
    {
    return esr_class_str[ESR_ELx_EC(esr)];
    }
//
// bad_el0_sync handles unexpected, but potentially recoverable synchronous
// exceptions taken from EL0.
//
#[no_mangle]
pub unsafe extern "C" fn bad_el0_sync(regs: *mut pt_regs, reason: c_int, esr: c_ulong) {
    void bad_el0_sync(struct pt_regs *regs, int reason, unsigned long esr)
    {
    let mut pc: c_ulong = instruction_pointer(regs);
    current.thread.fault_address = 0;
    current.thread.fault_code = esr;
    arm64_force_sig_fault(SIGILL, ILL_ILLOPC, pc,
    "Bad EL0 synchronous exception");
    }
    DEFINE_PER_CPU(unsigned long [OVERFLOW_STACK_SIZE/sizeof(long)], overflow_stack)
    __aligned(16);
#[no_mangle]
pub unsafe extern "C" fn panic_bad_stack(regs: *mut pt_regs, esr: c_ulong, far: c_ulong) -> void __noreturn {
    void __noreturn panic_bad_stack(struct pt_regs *regs, unsigned long esr, unsigned long far)
    {
    let mut tsk_stk: c_ulong = (unsigned long)current.stack;
    let mut irq_stk: c_ulong = (unsigned long)this_cpu_read(irq_stack_ptr);
    let mut ovf_stk: c_ulong = (unsigned long)this_cpu_ptr(overflow_stack);
    console_verbose();
    pr_emerg("Insufficient stack space to handle exception!");
    pr_emerg("ESR: 0x%016lx -- %s\n", esr, esr_get_class_string(esr));
    pr_emerg("FAR: 0x%016lx\n", far);
    pr_emerg("Task stack:     [0x%016lx..0x%016lx]\n",
    tsk_stk, tsk_stk + THREAD_SIZE);
    pr_emerg("IRQ stack:      [0x%016lx..0x%016lx]\n",
    irq_stk, irq_stk + IRQ_STACK_SIZE);
    pr_emerg("Overflow stack: [0x%016lx..0x%016lx]\n",
    ovf_stk, ovf_stk + OVERFLOW_STACK_SIZE);
    __show_regs(regs);
//
// We use nmi_panic to limit the potential for recursive overflows, and
// to get a better stack trace.
//
    nmi_panic(core::ptr::null_mut(), "kernel stack overflow");
    cpu_park_loop();
    }
#[no_mangle]
pub unsafe extern "C" fn arm64_serror_panic(regs: *mut pt_regs, esr: c_ulong) -> void __noreturn {
    void __noreturn arm64_serror_panic(struct pt_regs *regs, unsigned long esr)
    {
    add_taint(TAINT_MACHINE_CHECK, LOCKDEP_STILL_OK);
    console_verbose();
    pr_crit("SError Interrupt on CPU%d, code 0x%016lx -- %s\n",
    smp_processor_id(), esr, esr_get_class_string(esr));
    if (regs)
    __show_regs(regs);
    nmi_panic(regs, "Asynchronous SError Interrupt");
    cpu_park_loop();
    }
#[no_mangle]
pub unsafe extern "C" fn arm64_is_fatal_ras_serror(regs: *mut pt_regs, esr: c_ulong) -> bool {
    bool arm64_is_fatal_ras_serror(struct pt_regs *regs, unsigned long esr)
    {
    let mut aet: c_ulong = arm64_ras_serror_get_severity(esr);
    switch (aet) {
    case ESR_ELx_AET_CE:	/* corrected error */
    case ESR_ELx_AET_UEO:	/* restartable, not yet consumed */
//
// The CPU can make progress. We may take UEO again as
// a more severe error.
//
    return false;
    case ESR_ELx_AET_UEU:	/* Uncorrected Unrecoverable */
    case ESR_ELx_AET_UER:	/* Uncorrected Recoverable */
//
// The CPU can't make progress. The exception may have
// been imprecise.
//
// Neoverse-N1 #1349291 means a non-KVM SError reported as
// Unrecoverable should be treated as Uncontainable. We
// call arm64_serror_panic() in both cases.
//
    return true;
    case ESR_ELx_AET_UC:	/* Uncontainable or Uncategorized error */
    default:
// Error has been silently propagated
    arm64_serror_panic(regs, esr);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn do_serror(regs: *mut pt_regs, esr: c_ulong) {
    void do_serror(struct pt_regs *regs, unsigned long esr)
    {
// non-RAS errors are not containable
    if (!arm64_is_ras_serror(esr) || arm64_is_fatal_ras_serror(regs, esr))
    arm64_serror_panic(regs, esr);
    }
// GENERIC_BUG traps

#[no_mangle]
pub unsafe extern "C" fn is_valid_bugaddr(addr: c_ulong) -> c_int {
    int is_valid_bugaddr(unsigned long addr)
    {
//
// bug_brk_handler() only called for BRK #BUG_BRK_IMM.
// So the answer is trivial -- any spurious instances with no
// bug table entry will be rejected by report_bug() and passed
// back to the debug-monitors code and handled as a fatal
// unexpected debug exception.
//
    return 1;
    }

#[no_mangle]
pub unsafe extern "C" fn bug_brk_handler(regs: *mut pt_regs, esr: c_ulong) -> c_int {
    int bug_brk_handler(struct pt_regs *regs, unsigned long esr)
    {
    switch (report_bug(regs.pc, regs)) {
    case BUG_TRAP_TYPE_BUG:
    die("Oops - BUG", regs, esr);
    break;
    case BUG_TRAP_TYPE_WARN:
    break;
    default:
// unknown/unrecognised bug trap type
    return DBG_HOOK_ERROR;
    }
// If thread survives, skip over the BUG instruction and continue:
    arm64_skip_faulting_instruction(regs, AARCH64_INSN_SIZE);
    return DBG_HOOK_HANDLED;
    }

#[no_mangle]
pub unsafe extern "C" fn cfi_brk_handler(regs: *mut pt_regs, esr: c_ulong) -> c_int {
    int cfi_brk_handler(struct pt_regs *regs, unsigned long esr)
    {
    unsigned long target;
    u32 type;
    target = pt_regs_read_reg(regs, FIELD_GET(CFI_BRK_IMM_TARGET, esr));
    type = (u32)pt_regs_read_reg(regs, FIELD_GET(CFI_BRK_IMM_TYPE, esr));
    switch (report_cfi_failure(regs, regs.pc, &target, type)) {
    case BUG_TRAP_TYPE_BUG:
    die("Oops - CFI", regs, esr);
    break;
    case BUG_TRAP_TYPE_WARN:
    break;
    default:
    return DBG_HOOK_ERROR;
    }
    arm64_skip_faulting_instruction(regs, AARCH64_INSN_SIZE);
    return DBG_HOOK_HANDLED;
    }

#[no_mangle]
pub unsafe extern "C" fn reserved_fault_brk_handler(regs: *mut pt_regs, esr: c_ulong) -> c_int {
    int reserved_fault_brk_handler(struct pt_regs *regs, unsigned long esr)
    {
    pr_err("%s generated an invalid instruction at %pS!\n",
    "Kernel text patching",
    (void *)instruction_pointer(regs));
// We cannot handle this
    return DBG_HOOK_ERROR;
    }

pub const KASAN_ESR_RECOVER: c_uint = 0x20;
pub const KASAN_ESR_WRITE: c_uint = 0x10;
pub const KASAN_ESR_SIZE_MASK: c_uint = 0x0f;

#[no_mangle]
pub unsafe extern "C" fn kasan_brk_handler(regs: *mut pt_regs, esr: c_ulong) -> c_int {
    int kasan_brk_handler(struct pt_regs *regs, unsigned long esr)
    {
    let mut recover: bool = esr & KASAN_ESR_RECOVER;
    let mut write: bool = esr & KASAN_ESR_WRITE;
    let mut size: usize = KASAN_ESR_SIZE(esr);
    void *addr = (void *)regs.regs[0];
    let mut pc: u64 = regs.pc;
    kasan_report(addr, size, write, pc);
//
// The instrumentation allows to control whether we can proceed after
// a crash was detected. This is done by passing the -recover flag to
// the compiler. Disabling recovery allows to generate more compact
// code.
//
// Unfortunately disabling recovery doesn't work for the kernel right
// now. KASAN reporting is disabled in some contexts (for example when
// the allocator accesses slab object metadata; this is controlled by
// current->kasan_depth). All these accesses are detected by the tool,
// even though the reports for them are not printed.
//
// This is something that might be fixed at some point in the future.
//
    if (!recover)
    die("Oops - KASAN", regs, esr);
// If thread survives, skip over the brk instruction and continue:
    arm64_skip_faulting_instruction(regs, AARCH64_INSN_SIZE);
    return DBG_HOOK_HANDLED;
    }

#[no_mangle]
pub unsafe extern "C" fn ubsan_brk_handler(regs: *mut pt_regs, esr: c_ulong) -> c_int {
    int ubsan_brk_handler(struct pt_regs *regs, unsigned long esr)
    {
    die(report_ubsan_failure(esr & UBSAN_BRK_MASK), regs, esr);
    return DBG_HOOK_HANDLED;
    }
