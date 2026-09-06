//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/traps.c
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
// S390 version
// Copyright IBM Corp. 1999, 2000
// Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com),
// Denis Joseph Barrow (djbarrow@de.ibm.com,barrow_dj@yahoo.com),
//
// Derived from "arch/i386/kernel/traps.c"
// Copyright (C) 1991, 1992 Linus Torvalds
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgm_stat {
    pub count: [c_uint; 128],
}

    static DEFINE_PER_CPU_SHARED_ALIGNED(struct pgm_stat, pgm_stat);
    static inline void __user *get_trap_ip(struct pt_regs *regs)
    {
    unsigned long address;
    if (regs.int_code & 0x200)
    address = current.thread.trap_tdb.data[3];
    else
    address = regs.psw.addr;
    return (void __user *)(address - (regs.int_code >> 16));
    }

#[no_mangle]
pub unsafe extern "C" fn is_valid_bugaddr(addr: c_ulong) -> c_int {
    int is_valid_bugaddr(unsigned long addr)
    {
    return 1;
    }

#[no_mangle]
pub unsafe extern "C" fn do_report_trap(regs: *mut pt_regs, si_signo: c_int, si_code: c_int, str: *mut c_char) {
    void do_report_trap(struct pt_regs *regs, int si_signo, int si_code, char *str)
    {
    if (user_mode(regs)) {
    force_sig_fault(si_signo, si_code, get_trap_ip(regs));
    report_user_fault(regs, si_signo, 0);
    } else {
    if (!fixup_exception(regs))
    die(regs, str);
    }
    }
#[no_mangle]
unsafe extern "C" fn do_trap(regs: *mut pt_regs, si_signo: c_int, si_code: c_int, str: *mut c_char) {
    static void do_trap(struct pt_regs *regs, int si_signo, int si_code, char *str)
    {
    if (notify_die(DIE_TRAP, str, regs, 0, regs.int_code, si_signo) == NOTIFY_STOP)
    return;
    do_report_trap(regs, si_signo, si_code, str);
    }
    NOKPROBE_SYMBOL(do_trap);
#[no_mangle]
pub unsafe extern "C" fn do_per_trap(regs: *mut pt_regs) {
    void do_per_trap(struct pt_regs *regs)
    {
    if (notify_die(DIE_SSTEP, "sstep", regs, 0, 0, SIGTRAP) == NOTIFY_STOP)
    return;
    if (!current.ptrace)
    return;
    force_sig_fault(SIGTRAP, TRAP_HWBKPT, (void  __user *)current.thread.per_event.address);
    }
    NOKPROBE_SYMBOL(do_per_trap);
#[no_mangle]
unsafe extern "C" fn default_trap_handler(regs: *mut pt_regs) {
    static void default_trap_handler(struct pt_regs *regs)
    {
    if (user_mode(regs)) {
    report_user_fault(regs, SIGSEGV, 0);
    force_exit_sig(SIGSEGV);
    } else
    die(regs, "Unknown program exception");
    }

    static void name(struct pt_regs *regs)		\
    {						\
    do_trap(regs, signr, sicode, str);	\
    }
    DO_ERROR_INFO(addressing_exception, SIGILL, ILL_ILLADR, "addressing exception")
    DO_ERROR_INFO(divide_exception, SIGFPE, FPE_INTDIV, "fixpoint divide exception")
    DO_ERROR_INFO(execute_exception, SIGILL, ILL_ILLOPN, "execute exception")
    DO_ERROR_INFO(hfp_divide_exception, SIGFPE, FPE_FLTDIV, "HFP divide exception")
    DO_ERROR_INFO(hfp_overflow_exception, SIGFPE, FPE_FLTOVF, "HFP overflow exception")
    DO_ERROR_INFO(hfp_significance_exception, SIGFPE, FPE_FLTRES, "HFP significance exception")
    DO_ERROR_INFO(hfp_sqrt_exception, SIGFPE, FPE_FLTINV, "HFP square root exception")
    DO_ERROR_INFO(hfp_underflow_exception, SIGFPE, FPE_FLTUND, "HFP underflow exception")
    DO_ERROR_INFO(operand_exception, SIGILL, ILL_ILLOPN, "operand exception")
    DO_ERROR_INFO(overflow_exception, SIGFPE, FPE_INTOVF, "fixpoint overflow exception")
    DO_ERROR_INFO(privileged_op, SIGILL, ILL_PRVOPC, "privileged operation")
    DO_ERROR_INFO(special_op_exception, SIGILL, ILL_ILLOPN, "special operation exception")
    DO_ERROR_INFO(specification_exception, SIGILL, ILL_ILLOPN, "specification exception");
    DO_ERROR_INFO(transaction_exception, SIGILL, ILL_ILLOPN, "transaction constraint exception")
#[no_mangle]
pub unsafe extern "C" fn do_fp_trap(regs: *mut pt_regs, fpc: __u32) {
    static inline void do_fp_trap(struct pt_regs *regs, __u32 fpc)
    {
    let mut si_code: c_int = 0;
// FPC[2] is Data Exception Code
    if ((fpc & 0x00000300) == 0) {
// bits 6 and 7 of DXC are 0 iff IEEE exception
    if (fpc & 0x8000) /* invalid fp operation */
    si_code = FPE_FLTINV;
    else if (fpc & 0x4000) /* div by 0 */
    si_code = FPE_FLTDIV;
    else if (fpc & 0x2000) /* overflow */
    si_code = FPE_FLTOVF;
    else if (fpc & 0x1000) /* underflow */
    si_code = FPE_FLTUND;
    else if (fpc & 0x0800) /* inexact */
    si_code = FPE_FLTRES;
    }
    do_trap(regs, SIGFPE, si_code, "floating point exception");
    }
#[no_mangle]
unsafe extern "C" fn translation_specification_exception(regs: *mut pt_regs) {
    static void translation_specification_exception(struct pt_regs *regs)
    {
// May never happen.
    panic("Translation-Specification Exception");
    }
#[no_mangle]
unsafe extern "C" fn illegal_op(regs: *mut pt_regs) {
    static void illegal_op(struct pt_regs *regs)
    {
    let mut is_uprobe_insn: c_int = 0;
    u16 __user *location;
    let mut signal: c_int = 0;
    u16 opcode;
    location = get_trap_ip(regs);
    if (user_mode(regs)) {
    if (get_user(opcode, location))
    return;
    if (opcode == S390_BREAKPOINT_U16) {
    if (current.ptrace)
    force_sig_fault(SIGTRAP, TRAP_BRKPT, location);
    else
    signal = SIGILL;

    } else if (opcode == UPROBE_SWBP_INSN) {
    is_uprobe_insn = 1;

    } else {
    signal = SIGILL;
    }
    }
//
// This is either an illegal op in kernel mode, or user space trapped
// on a uprobes illegal instruction. See if kprobes or uprobes picks
// it up. If not, SIGILL.
//
    if (is_uprobe_insn || !user_mode(regs)) {
    if (notify_die(DIE_BPT, "bpt", regs, 0, 3, SIGTRAP) != NOTIFY_STOP)
    signal = SIGILL;
    }
    if (signal)
    do_trap(regs, signal, ILL_ILLOPC, "illegal operation");
    }
    NOKPROBE_SYMBOL(illegal_op);
#[no_mangle]
unsafe extern "C" fn vector_exception(regs: *mut pt_regs) {
    static void vector_exception(struct pt_regs *regs)
    {
    int si_code, vic;
// get vector interrupt code from fpc
    save_user_fpu_regs();
    vic = (current.thread.ufpu.fpc & 0xf00) >> 8;
    switch (vic) {
    case 1: /* invalid vector operation */
    si_code = FPE_FLTINV;
    break;
    case 2: /* division by zero */
    si_code = FPE_FLTDIV;
    break;
    case 3: /* overflow */
    si_code = FPE_FLTOVF;
    break;
    case 4: /* underflow */
    si_code = FPE_FLTUND;
    break;
    case 5:	/* inexact */
    si_code = FPE_FLTRES;
    break;
    default: /* unknown cause */
    si_code = 0;
    }
    do_trap(regs, SIGFPE, si_code, "vector exception");
    }
#[no_mangle]
unsafe extern "C" fn data_exception(regs: *mut pt_regs) {
    static void data_exception(struct pt_regs *regs)
    {
    save_user_fpu_regs();
    if (current.thread.ufpu.fpc & FPC_DXC_MASK)
    do_fp_trap(regs, current.thread.ufpu.fpc);
    else
    do_trap(regs, SIGILL, ILL_ILLOPN, "data exception");
    }
#[no_mangle]
unsafe extern "C" fn space_switch_exception(regs: *mut pt_regs) {
    static void space_switch_exception(struct pt_regs *regs)
    {
// Set user psw back to home space mode.
    if (user_mode(regs))
    regs.psw.mask |= PSW_ASC_HOME;
// Send SIGILL.
    do_trap(regs, SIGILL, ILL_PRVOPC, "space switch event");
    }

    void *__warn_args(struct arch_va_list *args, struct pt_regs *regs)
    {
    struct stack_frame *stack_frame;
//
// Generate va_list from pt_regs. See ELF Application Binary Interface
// s390x Supplement documentation for details.
//
// - __overflow_arg_area needs to point to the parameter area, which
// is right above the standard stack frame (160 bytes)
//
// - __reg_save_area needs to point to a register save area where
// general registers (%r2 - %r6) can be found at offset 16. Which
// means that the gprs save area of pt_regs can be used
//
// - __gpr must be set to one, since the first parameter has been
// processed (pointer to bug_entry)
//
    stack_frame = (struct stack_frame *)regs.gprs[15];
    args.__overflow_arg_area = stack_frame + 1;
    args.__reg_save_area = regs.gprs;
    args.__gpr = 1;
    return args;
    }

#[no_mangle]
unsafe extern "C" fn monitor_event_exception(regs: *mut pt_regs) {
    static void monitor_event_exception(struct pt_regs *regs)
    {
    enum bug_trap_type btt;
    if (user_mode(regs))
    return;
    if (regs.monitor_code == MONCODE_BUG_ARG) {
    regs.psw.addr = regs.gprs[14];
    btt = report_bug_entry((struct bug_entry *)regs.gprs[2], regs);
    } else {
    btt = report_bug(regs.psw.addr - (regs.int_code >> 16), regs);
    }
    switch (btt) {
    case BUG_TRAP_TYPE_NONE:
    fixup_exception(regs);
    break;
    case BUG_TRAP_TYPE_WARN:
    break;
    case BUG_TRAP_TYPE_BUG:
    die(regs, "monitor event");
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kernel_stack_invalid(regs: *mut pt_regs) {
    void kernel_stack_invalid(struct pt_regs *regs)
    {
//
// Normally regs are unpoisoned by the generic entry code, but
// kernel_stack_overflow() is a rare case that is called bypassing it.
//
    kmsan_unpoison_entry_regs(regs);
    bust_spinlocks(1);
    pr_emerg("Kernel stack pointer invalid\n");
    show_regs(regs);
    bust_spinlocks(0);
    panic("Invalid kernel stack pointer, cannot continue");
    }
    NOKPROBE_SYMBOL(kernel_stack_invalid);
#[no_mangle]
unsafe extern "C" fn test_monitor_call() -> void __init {
    static void __init test_monitor_call(void)
    {
    let mut val: c_int = 1;
    if (!IS_ENABLED(CONFIG_BUG))
    return;
    asm_inline volatile(
    "	mc	%[monc](%%r0),0\n"
    "0:	lhi	%[val],0\n"
    "1:\n"
    EX_TABLE(0b, 1b)
    : [val] "+d" (val)
    : [monc] "i" (MONCODE_BUG));
    if (!val)
    panic("Monitor call doesn't work!\n");
    }
#[no_mangle]
pub unsafe extern "C" fn trap_init() -> void __init {
    void __init trap_init(void)
    {
    struct lowcore *lc = get_lowcore();
    unsigned long flags;
    struct ctlreg cr0;
    local_irq_save(flags);
    cr0 = local_ctl_clear_bit(0, CR0_LOW_ADDRESS_PROTECTION_BIT);
    psw_bits(lc.external_new_psw).mcheck = 1;
    psw_bits(lc.program_new_psw).mcheck = 1;
    psw_bits(lc.svc_new_psw).mcheck = 1;
    psw_bits(lc.io_new_psw).mcheck = 1;
    local_ctl_load(0, &cr0);
    local_irq_restore(flags);
    local_mcck_enable();
    test_monitor_call();
    }
    static void (*pgm_check_table[128])(struct pt_regs *regs);
#[no_mangle]
pub unsafe extern "C" fn __do_pgm_check(regs: *mut pt_regs, flags: c_ulong) -> void noinstr {
    void noinstr __do_pgm_check(struct pt_regs *regs, unsigned long flags)
    {
    struct lowcore *lc = get_lowcore();
    bool percpu_needs_fixup;
    irqentry_state_t state;
    struct pgm_stat *stat;
    unsigned int trapnr;
    union teid teid;
    teid.val = lc.trans_exc_code;
    regs.int_code = lc.pgm_int_code;
    regs.int_parm_long = teid.val;
    regs.monitor_code = lc.monitor_code;
    trapnr = regs.int_code & PGM_INT_CODE_MASK;
    stat = this_cpu_ptr(&pgm_stat);
    stat.count[trapnr]++;
//
// In case of a guest fault, short-circuit the fault handler and return.
// This way the sie64a() function will return 0; fault address and
// other relevant bits are saved in current->thread.gmap_teid, and
// the fault number in current->thread.gmap_int_code. KVM will be
// able to use this information to handle the fault.
//
    if (flags & PGM_FLAG_GUEST_FAULT) {
    current.thread.gmap_teid.val = regs.int_parm_long;
    current.thread.gmap_int_code = regs.int_code & 0xffff;
    return;
    }
    percpu_entry(regs);
    state = irqentry_enter(regs);
    if (user_mode(regs)) {
    update_timer_sys();
    if (!cpu_has_bear()) {
    if (regs.last_break < 4096)
    regs.last_break = 1;
    }
    current.thread.last_break = regs.last_break;
    }
    if (lc.pgm_code & 0x0200) {
// transaction abort
    current.thread.trap_tdb = lc.pgm_tdb;
    }
    if (lc.pgm_code & PGM_INT_CODE_PER) {
    if (user_mode(regs)) {
    struct per_event *ev = &current.thread.per_event;
    set_thread_flag(TIF_PER_TRAP);
    ev.address = lc.per_address;
    ev.cause = lc.per_code_combined;
    ev.paid = lc.per_access_id;
    } else {
// PER event in kernel is kprobes
    __arch_local_irq_ssm(regs.psw.mask & ~PSW_MASK_PER);
    do_per_trap(regs);
    goto out;
    }
    }
    if (!irqs_disabled_flags(regs.psw.mask))
    trace_hardirqs_on();
    __arch_local_irq_ssm(regs.psw.mask & ~PSW_MASK_PER);
    if (trapnr)
    pgm_check_table[trapnr](regs);
    out:
    local_irq_disable();
    percpu_needs_fixup = percpu_code_check(regs);
    irqentry_exit(regs, state);
    percpu_exit(regs, percpu_needs_fixup);
    }
#[no_mangle]
unsafe extern "C" fn pgm_check_stat_show(p: *mut seq_file, v: *mut c_void) -> c_int {
    static int pgm_check_stat_show(struct seq_file *p, void *v)
    {
    int i, cpu;
    cpus_read_lock();
    seq_puts(p, "          ");
    for_each_online_cpu(cpu)
    seq_printf(p, "CPU%-8d", cpu);
    seq_putc(p, '\n');
    for (i = 0; i < 128; i++) {
    seq_printf(p, "%02x: ", i);
    for_each_online_cpu(cpu)
    seq_printf(p, "%10u ", per_cpu(pgm_stat, cpu).count[i]);
    seq_putc(p, '\n');
    }
    cpus_read_unlock();
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(pgm_check_stat);
#[no_mangle]
unsafe extern "C" fn debugfs_pgm_check_init() -> int __init {
    static int __init debugfs_pgm_check_init(void)
    {
    debugfs_create_file("exceptions", 0400, arch_debugfs_dir, core::ptr::null_mut(), &pgm_check_stat_fops);
    return 0;
    }
    late_initcall(debugfs_pgm_check_init);
//
// The program check table contains exactly 128 (0x00-0x7f) entries. Each
// line defines the function to be called corresponding to the program check
// interruption code.
//
    static void (*pgm_check_table[128])(struct pt_regs *regs) = {
    [0x00]		= default_trap_handler,
    [0x01]		= illegal_op,
    [0x02]		= privileged_op,
    [0x03]		= execute_exception,
    [0x04]		= do_protection_exception,
    [0x05]		= addressing_exception,
    [0x06]		= specification_exception,
    [0x07]		= data_exception,
    [0x08]		= overflow_exception,
    [0x09]		= divide_exception,
    [0x0a]		= overflow_exception,
    [0x0b]		= divide_exception,
    [0x0c]		= hfp_overflow_exception,
    [0x0d]		= hfp_underflow_exception,
    [0x0e]		= hfp_significance_exception,
    [0x0f]		= hfp_divide_exception,
    [0x10]		= do_dat_exception,
    [0x11]		= do_dat_exception,
    [0x12]		= translation_specification_exception,
    [0x13]		= special_op_exception,
    [0x14]		= default_trap_handler,
    [0x15]		= operand_exception,
    [0x16]		= default_trap_handler,
    [0x17]		= default_trap_handler,
    [0x18]		= transaction_exception,
    [0x19]		= default_trap_handler,
    [0x1a]		= default_trap_handler,
    [0x1b]		= vector_exception,
    [0x1c]		= space_switch_exception,
    [0x1d]		= hfp_sqrt_exception,
    [0x1e ... 0x37] = default_trap_handler,
    [0x38]		= do_dat_exception,
    [0x39]		= do_dat_exception,
    [0x3a]		= do_dat_exception,
    [0x3b]		= do_dat_exception,
    [0x3c]		= default_trap_handler,
    [0x3d]		= do_secure_storage_access,
    [0x3e]		= default_trap_handler,
    [0x3f]		= default_trap_handler,
    [0x40]		= monitor_event_exception,
    [0x41 ... 0x7f] = default_trap_handler,
    };

    ".weak " __stringify(x) "\n\t"		\
    ".set  " __stringify(x) ","		\
    __stringify(default_trap_handler))
    COND_TRAP(do_secure_storage_access);
