//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/ptrace/ptrace.c
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
// PowerPC version
// Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
//
// Derived from "arch/m68k/kernel/ptrace.c"
// Copyright (C) 1994 by Hamish Macdonald
// Taken from linux/kernel/ptrace.c and modified for M680x0.
// linux/kernel/ptrace.c is by Ross Biro 1/23/92, edited by Linus Torvalds
//
// Modified by Cort Dougan (cort@hq.fsmlabs.com)
// and Paul Mackerras (paulus@samba.org).
//

//
// Called by kernel/ptrace.c when detaching..
//
// Make sure single step bits etc are not set.
//
#[no_mangle]
pub unsafe extern "C" fn ptrace_disable(child: *mut task_struct) {
    void ptrace_disable(struct task_struct *child)
    {
// make sure the single step bit is not set.
    user_disable_single_step(child);
    }
    long arch_ptrace(struct task_struct *child, long request,
    unsigned long addr, unsigned long data)
    {
    let mut ret: c_int = -EPERM;
    void __user *datavp = (void __user *) data;
    unsigned long __user *datalp = datavp;
    switch (request) {
// read the word at location addr in the USER area.
    case PTRACE_PEEKUSR: {
    unsigned long index, tmp;
    ret = -EIO;
// convert to index and check
    index = addr / sizeof(long);
    if ((addr & (sizeof(long) - 1)) || !child.thread.regs)
    break;
    if (index < PT_FPR0)
    ret = ptrace_get_reg(child, (int) index, &tmp);
    else
    ret = ptrace_get_fpr(child, index, &tmp);
    if (ret)
    break;
    ret = put_user(tmp, datalp);
    break;
    }
// write the word at location addr in the USER area
    case PTRACE_POKEUSR: {
    unsigned long index;
    ret = -EIO;
// convert to index and check
    index = addr / sizeof(long);
    if ((addr & (sizeof(long) - 1)) || !child.thread.regs)
    break;
    if (index < PT_FPR0)
    ret = ptrace_put_reg(child, index, data);
    else
    ret = ptrace_put_fpr(child, index, data);
    break;
    }
    case PPC_PTRACE_GETHWDBGINFO: {
    struct ppc_debug_info dbginfo;
    ppc_gethwdinfo(&dbginfo);
    if (copy_to_user(datavp, &dbginfo,
    sizeof(struct ppc_debug_info)))
    return -EFAULT;
    return 0;
    }
    case PPC_PTRACE_SETHWDEBUG: {
    struct ppc_hw_breakpoint bp_info;
    if (copy_from_user(&bp_info, datavp,
    sizeof(struct ppc_hw_breakpoint)))
    return -EFAULT;
    return ppc_set_hwdebug(child, &bp_info);
    }
    case PPC_PTRACE_DELHWDEBUG: {
    ret = ppc_del_hwdebug(child, data);
    break;
    }
    case PTRACE_GET_DEBUGREG:
    ret = ptrace_get_debugreg(child, addr, datalp);
    break;
    case PTRACE_SET_DEBUGREG:
    ret = ptrace_set_debugreg(child, addr, data);
    break;

    case PTRACE_GETREGS64:

    case PTRACE_GETREGS:	/* Get all pt_regs from the child. */
    return copy_regset_to_user(child, &user_ppc_native_view,
    REGSET_GPR,
    0, sizeof(struct user_pt_regs),
    datavp);

    case PTRACE_SETREGS64:

    case PTRACE_SETREGS:	/* Set all gp regs in the child. */
    return copy_regset_from_user(child, &user_ppc_native_view,
    REGSET_GPR,
    0, sizeof(struct user_pt_regs),
    datavp);
    case PTRACE_GETFPREGS: /* Get the child FPU state (FPR0...31 + FPSCR) */
    return copy_regset_to_user(child, &user_ppc_native_view,
    REGSET_FPR,
    0, sizeof(elf_fpregset_t),
    datavp);
    case PTRACE_SETFPREGS: /* Set the child FPU state (FPR0...31 + FPSCR) */
    return copy_regset_from_user(child, &user_ppc_native_view,
    REGSET_FPR,
    0, sizeof(elf_fpregset_t),
    datavp);

    case PTRACE_GETVRREGS:
    return copy_regset_to_user(child, &user_ppc_native_view,
    REGSET_VMX,
    0, (33 * sizeof(vector128) +
    sizeof(u32)),
    datavp);
    case PTRACE_SETVRREGS:
    return copy_regset_from_user(child, &user_ppc_native_view,
    REGSET_VMX,
    0, (33 * sizeof(vector128) +
    sizeof(u32)),
    datavp);

    case PTRACE_GETVSRREGS:
    return copy_regset_to_user(child, &user_ppc_native_view,
    REGSET_VSX,
    0, 32 * sizeof(double),
    datavp);
    case PTRACE_SETVSRREGS:
    return copy_regset_from_user(child, &user_ppc_native_view,
    REGSET_VSX,
    0, 32 * sizeof(double),
    datavp);

    case PTRACE_GETEVRREGS:
// Get the child spe register state.
    return copy_regset_to_user(child, &user_ppc_native_view,
    REGSET_SPE, 0, 35 * sizeof(u32),
    datavp);
    case PTRACE_SETEVRREGS:
// Set the child spe register state.
    return copy_regset_from_user(child, &user_ppc_native_view,
    REGSET_SPE, 0, 35 * sizeof(u32),
    datavp);

    default:
    ret = ptrace_request(child, request, addr, data);
    break;
    }
    return ret;
    }
    void __init pt_regs_check(void);
//
// Dummy function, its purpose is to break the build if struct pt_regs and
// struct user_pt_regs don't match.
//
#[no_mangle]
pub unsafe extern "C" fn pt_regs_check() -> void __init {
    void __init pt_regs_check(void)
    {
    BUILD_BUG_ON(offsetof(struct pt_regs, gpr) !=
    offsetof(struct user_pt_regs, gpr));
    BUILD_BUG_ON(offsetof(struct pt_regs, nip) !=
    offsetof(struct user_pt_regs, nip));
    BUILD_BUG_ON(offsetof(struct pt_regs, msr) !=
    offsetof(struct user_pt_regs, msr));
    BUILD_BUG_ON(offsetof(struct pt_regs, orig_gpr3) !=
    offsetof(struct user_pt_regs, orig_gpr3));
    BUILD_BUG_ON(offsetof(struct pt_regs, ctr) !=
    offsetof(struct user_pt_regs, ctr));
    BUILD_BUG_ON(offsetof(struct pt_regs, link) !=
    offsetof(struct user_pt_regs, link));
    BUILD_BUG_ON(offsetof(struct pt_regs, xer) !=
    offsetof(struct user_pt_regs, xer));
    BUILD_BUG_ON(offsetof(struct pt_regs, ccr) !=
    offsetof(struct user_pt_regs, ccr));

    BUILD_BUG_ON(offsetof(struct pt_regs, softe) !=
    offsetof(struct user_pt_regs, softe));

    BUILD_BUG_ON(offsetof(struct pt_regs, mq) !=
    offsetof(struct user_pt_regs, mq));

    BUILD_BUG_ON(offsetof(struct pt_regs, trap) !=
    offsetof(struct user_pt_regs, trap));
    BUILD_BUG_ON(offsetof(struct pt_regs, dar) !=
    offsetof(struct user_pt_regs, dar));
    BUILD_BUG_ON(offsetof(struct pt_regs, dear) !=
    offsetof(struct user_pt_regs, dar));
    BUILD_BUG_ON(offsetof(struct pt_regs, dsisr) !=
    offsetof(struct user_pt_regs, dsisr));
    BUILD_BUG_ON(offsetof(struct pt_regs, esr) !=
    offsetof(struct user_pt_regs, dsisr));
    BUILD_BUG_ON(offsetof(struct pt_regs, result) !=
    offsetof(struct user_pt_regs, result));
    BUILD_BUG_ON(sizeof(struct user_pt_regs) > sizeof(struct pt_regs));
// Now check that the pt_regs offsets match the uapi #defines

    BUILD_BUG_ON(_pt != (offsetof(struct user_pt_regs, _reg) / \
    sizeof(unsigned long)));
    CHECK_REG(PT_R0,  gpr[0]);
    CHECK_REG(PT_R1,  gpr[1]);
    CHECK_REG(PT_R2,  gpr[2]);
    CHECK_REG(PT_R3,  gpr[3]);
    CHECK_REG(PT_R4,  gpr[4]);
    CHECK_REG(PT_R5,  gpr[5]);
    CHECK_REG(PT_R6,  gpr[6]);
    CHECK_REG(PT_R7,  gpr[7]);
    CHECK_REG(PT_R8,  gpr[8]);
    CHECK_REG(PT_R9,  gpr[9]);
    CHECK_REG(PT_R10, gpr[10]);
    CHECK_REG(PT_R11, gpr[11]);
    CHECK_REG(PT_R12, gpr[12]);
    CHECK_REG(PT_R13, gpr[13]);
    CHECK_REG(PT_R14, gpr[14]);
    CHECK_REG(PT_R15, gpr[15]);
    CHECK_REG(PT_R16, gpr[16]);
    CHECK_REG(PT_R17, gpr[17]);
    CHECK_REG(PT_R18, gpr[18]);
    CHECK_REG(PT_R19, gpr[19]);
    CHECK_REG(PT_R20, gpr[20]);
    CHECK_REG(PT_R21, gpr[21]);
    CHECK_REG(PT_R22, gpr[22]);
    CHECK_REG(PT_R23, gpr[23]);
    CHECK_REG(PT_R24, gpr[24]);
    CHECK_REG(PT_R25, gpr[25]);
    CHECK_REG(PT_R26, gpr[26]);
    CHECK_REG(PT_R27, gpr[27]);
    CHECK_REG(PT_R28, gpr[28]);
    CHECK_REG(PT_R29, gpr[29]);
    CHECK_REG(PT_R30, gpr[30]);
    CHECK_REG(PT_R31, gpr[31]);
    CHECK_REG(PT_NIP, nip);
    CHECK_REG(PT_MSR, msr);
    CHECK_REG(PT_ORIG_R3, orig_gpr3);
    CHECK_REG(PT_CTR, ctr);
    CHECK_REG(PT_LNK, link);
    CHECK_REG(PT_XER, xer);
    CHECK_REG(PT_CCR, ccr);

    CHECK_REG(PT_SOFTE, softe);

    CHECK_REG(PT_MQ, mq);

    CHECK_REG(PT_TRAP, trap);
    CHECK_REG(PT_DAR, dar);
    CHECK_REG(PT_DSISR, dsisr);
    CHECK_REG(PT_RESULT, result);

    BUILD_BUG_ON(PT_REGS_COUNT != sizeof(struct user_pt_regs) / sizeof(unsigned long));
//
// PT_DSCR isn't a real reg, but it's important that it doesn't overlap the
// real registers.
//
    BUILD_BUG_ON(PT_DSCR < sizeof(struct user_pt_regs) / sizeof(unsigned long));
// ptrace_get/put_fpr() rely on PPC32 and VSX being incompatible
    BUILD_BUG_ON(IS_ENABLED(CONFIG_PPC32) && IS_ENABLED(CONFIG_VSX));
    }
