//! Automatically rewritten from C to Rust
//! Source: arch/x86/um/ptrace_32.c
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


//
// Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
// Licensed under the GPL
//

#[no_mangle]
pub unsafe extern "C" fn arch_switch_to(to: *mut task_struct) {
    void arch_switch_to(struct task_struct *to)
    {
    let mut err: c_int = arch_switch_tls(to);
    if (!err)
    return;
    if (err != -EINVAL)
    printk(KERN_WARNING "arch_switch_tls failed, errno %d, "
    "not EINVAL\n", -err);
    else
    printk(KERN_WARNING "arch_switch_tls failed, errno = EINVAL\n");
    }
// determines which flags the user has access to.
// 1 = access 0 = no access
pub const FLAG_MASK: c_uint = 0x00044dd5;
    static const int reg_offsets[] = {
    [EBX] = HOST_BX,
    [ECX] = HOST_CX,
    [EDX] = HOST_DX,
    [ESI] = HOST_SI,
    [EDI] = HOST_DI,
    [EBP] = HOST_BP,
    [EAX] = HOST_AX,
    [DS] = HOST_DS,
    [ES] = HOST_ES,
    [FS] = HOST_FS,
    [GS] = HOST_GS,
    [EIP] = HOST_IP,
    [CS] = HOST_CS,
    [EFL] = HOST_EFLAGS,
    [UESP] = HOST_SP,
    [SS] = HOST_SS,
    [ORIG_EAX] = HOST_ORIG_AX,
    };
#[no_mangle]
pub unsafe extern "C" fn putreg(child: *mut task_struct, regno: c_int, value: c_ulong) -> c_int {
    int putreg(struct task_struct *child, int regno, unsigned long value)
    {
    regno >>= 2;
    switch (regno) {
    case EBX:
    case ECX:
    case EDX:
    case ESI:
    case EDI:
    case EBP:
    case EAX:
    case EIP:
    case UESP:
    break;
    case ORIG_EAX:
// Update the syscall number.
    UPT_SYSCALL_NR(&child.thread.regs.regs) = value;
    break;
    case FS:
    if (value && (value & 3) != 3)
    return -EIO;
    break;
    case GS:
    if (value && (value & 3) != 3)
    return -EIO;
    break;
    case DS:
    case ES:
    if (value && (value & 3) != 3)
    return -EIO;
    value &= 0xffff;
    break;
    case SS:
    case CS:
    if ((value & 3) != 3)
    return -EIO;
    value &= 0xffff;
    break;
    case EFL:
    value &= FLAG_MASK;
    child.thread.regs.regs.gp[HOST_EFLAGS] |= value;
    return 0;
    default :
    panic("Bad register in putreg() : %d\n", regno);
    }
    child.thread.regs.regs.gp[reg_offsets[regno]] = value;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn poke_user(child: *mut task_struct, addr: c_long, data: c_long) -> c_int {
    int poke_user(struct task_struct *child, long addr, long data)
    {
    if ((addr & 3) || addr < 0)
    return -EIO;
    if (addr < MAX_REG_OFFSET)
    return putreg(child, addr, data);
    else if ((addr >= offsetof(struct user, u_debugreg[0])) &&
    (addr <= offsetof(struct user, u_debugreg[7]))) {
    addr -= offsetof(struct user, u_debugreg[0]);
    addr = addr >> 2;
    if ((addr == 4) || (addr == 5))
    return -EIO;
    child.thread.arch.debugregs[addr] = data;
    return 0;
    }
    return -EIO;
    }
#[no_mangle]
pub unsafe extern "C" fn getreg(child: *mut task_struct, regno: c_int) -> c_ulong {
    unsigned long getreg(struct task_struct *child, int regno)
    {
    let mut mask: c_ulong = ~0UL;
    regno >>= 2;
    switch (regno) {
    case FS:
    case GS:
    case DS:
    case ES:
    case SS:
    case CS:
    mask = 0xffff;
    break;
    case EIP:
    case UESP:
    case EAX:
    case EBX:
    case ECX:
    case EDX:
    case ESI:
    case EDI:
    case EBP:
    case EFL:
    case ORIG_EAX:
    break;
    default:
    panic("Bad register in getreg() : %d\n", regno);
    }
    return mask & child.thread.regs.regs.gp[reg_offsets[regno]];
    }
// read the word at location addr in the USER area.
#[no_mangle]
pub unsafe extern "C" fn peek_user(child: *mut task_struct, addr: c_long, data: c_long) -> c_int {
    int peek_user(struct task_struct *child, long addr, long data)
    {
    unsigned long tmp;
    if ((addr & 3) || addr < 0)
    return -EIO;
    tmp = 0;  /* Default return condition */
    if (addr < MAX_REG_OFFSET) {
    tmp = getreg(child, addr);
    }
    else if ((addr >= offsetof(struct user, u_debugreg[0])) &&
    (addr <= offsetof(struct user, u_debugreg[7]))) {
    addr -= offsetof(struct user, u_debugreg[0]);
    addr = addr >> 2;
    tmp = child.thread.arch.debugregs[addr];
    }
    return put_user(tmp, (unsigned long __user *) data);
    }
    long subarch_ptrace(struct task_struct *child, long request,
    unsigned long addr, unsigned long data)
    {
    let mut ret: c_int = -EIO;
    void __user *datap = (void __user *) data;
    switch (request) {
    case PTRACE_GETFPREGS: /* Get the child FPU state. */
    return copy_regset_to_user(child, task_user_regset_view(child),
    REGSET_FP_LEGACY,
    0, sizeof(struct user_i387_struct),
    datap);
    case PTRACE_SETFPREGS: /* Set the child FPU state. */
    return copy_regset_from_user(child, task_user_regset_view(child),
    REGSET_FP_LEGACY,
    0, sizeof(struct user_i387_struct),
    datap);
    case PTRACE_GETFPXREGS: /* Get the child FPU state. */
    return copy_regset_to_user(child, task_user_regset_view(child),
    REGSET_FP,
    0, sizeof(struct user_fxsr_struct),
    datap);
    case PTRACE_SETFPXREGS: /* Set the child FPU state. */
    return copy_regset_from_user(child, task_user_regset_view(child),
    REGSET_FP,
    0, sizeof(struct user_fxsr_struct),
    datap);
    default:
    ret = -EIO;
    }
    return ret;
    }
