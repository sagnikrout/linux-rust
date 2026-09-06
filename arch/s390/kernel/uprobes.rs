//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/uprobes.c
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
// User-space Probes (UProbes) for s390
//
// Copyright IBM Corp. 2014
// Author(s): Jan Willeke,
//

    int arch_uprobe_analyze_insn(struct arch_uprobe *auprobe, struct mm_struct *mm,
    unsigned long addr)
    {
    return probe_is_prohibited_opcode(auprobe.insn);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_uprobe_pre_xol(auprobe: *mut arch_uprobe, regs: *mut pt_regs) -> c_int {
    int arch_uprobe_pre_xol(struct arch_uprobe *auprobe, struct pt_regs *regs)
    {
    if (psw_bits(regs.psw).eaba == PSW_BITS_AMODE_24BIT)
    return -EINVAL;
    if (psw_bits(regs.psw).eaba == PSW_BITS_AMODE_31BIT)
    return -EINVAL;
    clear_thread_flag(TIF_PER_TRAP);
    auprobe.saved_per = psw_bits(regs.psw).per;
    auprobe.saved_int_code = regs.int_code;
    regs.int_code = UPROBE_TRAP_NR;
    regs.psw.addr = current.utask.xol_vaddr;
    set_tsk_thread_flag(current, TIF_UPROBE_SINGLESTEP);
    update_cr_regs(current);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_uprobe_xol_was_trapped(tsk: *mut task_struct) -> bool {
    bool arch_uprobe_xol_was_trapped(struct task_struct *tsk)
    {
    struct pt_regs *regs = task_pt_regs(tsk);
    if (regs.int_code != UPROBE_TRAP_NR)
    return true;
    return false;
    }
    static int check_per_event(unsigned short cause, unsigned long control,
    struct pt_regs *regs)
    {
    if (!(regs.psw.mask & PSW_MASK_PER))
    return 0;
// user space single step
    if (control == 0)
    return 1;
// over indication for storage alteration
    if ((control & 0x20200000) && (cause & 0x2000))
    return 1;
    if (cause & 0x8000) {
// all branches
    if ((control & 0x80800000) == 0x80000000)
    return 1;
// branch into selected range
    if (((control & 0x80800000) == 0x80800000) &&
    regs.psw.addr >= current.thread.per_user.start &&
    regs.psw.addr <= current.thread.per_user.end)
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_uprobe_post_xol(auprobe: *mut arch_uprobe, regs: *mut pt_regs) -> c_int {
    int arch_uprobe_post_xol(struct arch_uprobe *auprobe, struct pt_regs *regs)
    {
    let mut fixup: c_int = probe_get_fixup_type(auprobe.insn);
    struct uprobe_task *utask = current.utask;
    clear_tsk_thread_flag(current, TIF_UPROBE_SINGLESTEP);
    update_cr_regs(current);
    psw_bits(regs.psw).per = auprobe.saved_per;
    regs.int_code = auprobe.saved_int_code;
    if (fixup & FIXUP_PSW_NORMAL)
    regs.psw.addr += utask.vaddr - utask.xol_vaddr;
    if (fixup & FIXUP_RETURN_REGISTER) {
    let mut reg: c_int = (auprobe.insn[0] & 0xf0) >> 4;
    regs.gprs[reg] += utask.vaddr - utask.xol_vaddr;
    }
    if (fixup & FIXUP_BRANCH_NOT_TAKEN) {
    let mut ilen: c_int = insn_length(auprobe.insn[0] >> 8);
    if (regs.psw.addr - utask.xol_vaddr == ilen)
    regs.psw.addr = utask.vaddr + ilen;
    }
    if (check_per_event(current.thread.per_event.cause,
    current.thread.per_user.control, regs)) {
// fix per address
    current.thread.per_event.address = utask.vaddr;
// trigger per event
    set_thread_flag(TIF_PER_TRAP);
    }
    return 0;
    }
    int arch_uprobe_exception_notify(struct notifier_block *self, unsigned long val,
    void *data)
    {
    struct die_args *args = data;
    struct pt_regs *regs = args.regs;
    if (!user_mode(regs))
    return NOTIFY_DONE;
    if (regs.int_code & 0x200) /* Trap during transaction */
    return NOTIFY_DONE;
    switch (val) {
    case DIE_BPT:
    if (uprobe_pre_sstep_notifier(regs))
    return NOTIFY_STOP;
    break;
    case DIE_SSTEP:
    if (uprobe_post_sstep_notifier(regs))
    return NOTIFY_STOP;
    break;
    default:
    break;
    }
    return NOTIFY_DONE;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_uprobe_abort_xol(auprobe: *mut arch_uprobe, regs: *mut pt_regs) {
    void arch_uprobe_abort_xol(struct arch_uprobe *auprobe, struct pt_regs *regs)
    {
    clear_thread_flag(TIF_UPROBE_SINGLESTEP);
    regs.int_code = auprobe.saved_int_code;
    regs.psw.addr = current.utask.vaddr;
    current.thread.per_event.address = current.utask.vaddr;
    }
    unsigned long arch_uretprobe_hijack_return_addr(unsigned long trampoline,
    struct pt_regs *regs)
    {
    unsigned long orig;
    orig = regs.gprs[14];
    regs.gprs[14] = trampoline;
    return orig;
    }
    bool arch_uretprobe_is_alive(struct return_instance *ret, enum rp_check ctx,
    struct pt_regs *regs)
    {
    if (ctx == RP_CHECK_CHAIN_CALL)
    return user_stack_pointer(regs) <= ret.stack;
    else
    return user_stack_pointer(regs) < ret.stack;
    }
// Instruction Emulation
pub const EMU_ILLEGAL_OP: c_int = 1;
pub const EMU_SPECIFICATION: c_int = 2;
pub const EMU_ADDRESSING: c_int = 3;

    ({							\
    unsigned int mask = sizeof(*(ptr)) - 1;		\
    __typeof__(*(ptr)) input;			\
    int __rc = 0;					\
    \
    if ((u64 )ptr & mask)			\
    __rc = EMU_SPECIFICATION;		\
    else if (get_user(input, ptr))			\
    __rc = EMU_ADDRESSING;			\
    else						\
// (output) = input;			\
    __rc;						\
    })

    ({							\
    unsigned int mask = sizeof(*(ptr)) - 1;		\
    __typeof__(ptr) __ptr = (ptr);			\
    int __rc = 0;					\
    \
    if ((u64 )__ptr & mask)			\
    __rc = EMU_SPECIFICATION;		\
    else if (put_user(*(input), __ptr))		\
    __rc = EMU_ADDRESSING;			\
    if (__rc == 0)					\
    sim_stor_event(regs,			\
    (void  *)__ptr,	\
    mask + 1);		\
    __rc;						\
    })

    ({							\
    unsigned int mask = sizeof(*(ptr)) - 1;		\
    __typeof__(*(ptr)) input;			\
    int __rc = 0;					\
    \
    if ((u64 )ptr & mask)			\
    __rc = EMU_SPECIFICATION;		\
    else if (get_user(input, ptr))			\
    __rc = EMU_ADDRESSING;			\
    else if (input > *(cmp))			\
    psw_bits((regs).psw).cc = 1;		\
    else if (input < *(cmp))			\
    psw_bits((regs).psw).cc = 2;		\
    else						\
    psw_bits((regs).psw).cc = 0;		\
    __rc;						\
    })
#[repr(C)]
#[derive(Copy, Clone)]
pub struct insn_ril {
    pub opc0: u8,
    pub 4: u8 reg :,
    pub 4: u8 opc1 :,
    pub disp: i32,
    pub __packed: },
    union split_register {
    pub u64: u64,
    pub u32: [u32; 2],
    pub u16: [u16; 4],
    pub s64: i64,
    pub s32: [i32; 2],
    pub s16: [i16; 4],
}

//
// If user per registers are setup to trace storage alterations and an
// emulated store took place on a fitting address a user trap is generated.
//
#[no_mangle]
unsafe extern "C" fn sim_stor_event(regs: *mut pt_regs, addr: *mut c_void, len: c_int) {
    static void sim_stor_event(struct pt_regs *regs, void *addr, int len)
    {
    if (!(regs.psw.mask & PSW_MASK_PER))
    return;
    if (!(current.thread.per_user.control & PER_EVENT_STORE))
    return;
    if ((void *)current.thread.per_user.start > (addr + len))
    return;
    if ((void *)current.thread.per_user.end < addr)
    return;
    current.thread.per_event.address = regs.psw.addr;
    current.thread.per_event.cause = PER_EVENT_STORE >> 16;
    set_thread_flag(TIF_PER_TRAP);
    }
//
// pc relative instructions are emulated, since parameters may not be
// accessible from the xol area due to range limitations.
//
#[no_mangle]
unsafe extern "C" fn handle_insn_ril(auprobe: *mut arch_uprobe, regs: *mut pt_regs) {
    static void handle_insn_ril(struct arch_uprobe *auprobe, struct pt_regs *regs)
    {
    union split_register *rx;
    struct insn_ril *insn;
    unsigned int ilen;
    void *uptr;
    let mut rc: c_int = 0;
    insn = (struct insn_ril *) &auprobe.insn;
    rx = (union split_register *) &regs.gprs[insn.reg];
    uptr = (void *)(regs.psw.addr + (insn.disp * 2));
    ilen = insn_length(insn.opc0);
    switch (insn.opc0) {
    case 0xc0:
    switch (insn.opc1) {
    case 0x00: /* larl */
    rx.u64 = (unsigned long)uptr;
    break;
    }
    break;
    case 0xc4:
    switch (insn.opc1) {
    case 0x02: /* llhrl */
    rc = emu_load_ril((u16 __user *)uptr, &rx.u32[1]);
    break;
    case 0x04: /* lghrl */
    rc = emu_load_ril((s16 __user *)uptr, &rx.u64);
    break;
    case 0x05: /* lhrl */
    rc = emu_load_ril((s16 __user *)uptr, &rx.u32[1]);
    break;
    case 0x06: /* llghrl */
    rc = emu_load_ril((u16 __user *)uptr, &rx.u64);
    break;
    case 0x08: /* lgrl */
    rc = emu_load_ril((u64 __user *)uptr, &rx.u64);
    break;
    case 0x0c: /* lgfrl */
    rc = emu_load_ril((s32 __user *)uptr, &rx.u64);
    break;
    case 0x0d: /* lrl */
    rc = emu_load_ril((u32 __user *)uptr, &rx.u32[1]);
    break;
    case 0x0e: /* llgfrl */
    rc = emu_load_ril((u32 __user *)uptr, &rx.u64);
    break;
    case 0x07: /* sthrl */
    rc = emu_store_ril(regs, (u16 __user *)uptr, &rx.u16[3]);
    break;
    case 0x0b: /* stgrl */
    rc = emu_store_ril(regs, (u64 __user *)uptr, &rx.u64);
    break;
    case 0x0f: /* strl */
    rc = emu_store_ril(regs, (u32 __user *)uptr, &rx.u32[1]);
    break;
    }
    break;
    case 0xc6:
    switch (insn.opc1) {
    case 0x04: /* cghrl */
    rc = emu_cmp_ril(regs, (s16 __user *)uptr, &rx.s64);
    break;
    case 0x05: /* chrl */
    rc = emu_cmp_ril(regs, (s16 __user *)uptr, &rx.s32[1]);
    break;
    case 0x06: /* clghrl */
    rc = emu_cmp_ril(regs, (u16 __user *)uptr, &rx.u64);
    break;
    case 0x07: /* clhrl */
    rc = emu_cmp_ril(regs, (u16 __user *)uptr, &rx.u32[1]);
    break;
    case 0x08: /* cgrl */
    rc = emu_cmp_ril(regs, (s64 __user *)uptr, &rx.s64);
    break;
    case 0x0a: /* clgrl */
    rc = emu_cmp_ril(regs, (u64 __user *)uptr, &rx.u64);
    break;
    case 0x0c: /* cgfrl */
    rc = emu_cmp_ril(regs, (s32 __user *)uptr, &rx.s64);
    break;
    case 0x0d: /* crl */
    rc = emu_cmp_ril(regs, (s32 __user *)uptr, &rx.s32[1]);
    break;
    case 0x0e: /* clgfrl */
    rc = emu_cmp_ril(regs, (u32 __user *)uptr, &rx.u64);
    break;
    case 0x0f: /* clrl */
    rc = emu_cmp_ril(regs, (u32 __user *)uptr, &rx.u32[1]);
    break;
    }
    break;
    }
    regs.psw.addr = __forward_psw(regs.psw, ilen);
    switch (rc) {
    case EMU_ILLEGAL_OP:
    regs.int_code = ilen << 16 | 0x0001;
    do_report_trap(regs, SIGILL, ILL_ILLOPC, core::ptr::null_mut());
    break;
    case EMU_SPECIFICATION:
    regs.int_code = ilen << 16 | 0x0006;
    do_report_trap(regs, SIGILL, ILL_ILLOPC , core::ptr::null_mut());
    break;
    case EMU_ADDRESSING:
    regs.int_code = ilen << 16 | 0x0005;
    do_report_trap(regs, SIGSEGV, SEGV_MAPERR, core::ptr::null_mut());
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn arch_uprobe_skip_sstep(auprobe: *mut arch_uprobe, regs: *mut pt_regs) -> bool {
    bool arch_uprobe_skip_sstep(struct arch_uprobe *auprobe, struct pt_regs *regs)
    {
    if ((psw_bits(regs.psw).eaba == PSW_BITS_AMODE_24BIT) ||
    (psw_bits(regs.psw).eaba == PSW_BITS_AMODE_31BIT)) {
    regs.psw.addr = __rewind_psw(regs.psw, UPROBE_SWBP_INSN_SIZE);
    do_report_trap(regs, SIGILL, ILL_ILLADR, core::ptr::null_mut());
    return true;
    }
    if (probe_is_insn_relative_long(auprobe.insn)) {
    handle_insn_ril(auprobe, regs);
    return true;
    }
    return false;
    }
