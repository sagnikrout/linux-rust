//! Automatically rewritten from C to Rust
//! Source: arch/x86/um/os-Linux/mcontext.c
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
// Macro flag: #define __FRAME_OFFSETS

//
// musl defines struct sigcontext in <bits/signal.h>.  Rename the kernel's
// copy to avoid redefinition while keeping the FP-state types available.
//

#[no_mangle]
pub unsafe extern "C" fn get_regs_from_mc(regs: *mut uml_pt_regs, mc: *mut mcontext_t) {
    void get_regs_from_mc(struct uml_pt_regs *regs, mcontext_t *mc)
    {

    COPY_SEG(GS); COPY_SEG(FS); COPY_SEG(ES); COPY_SEG(DS);
    COPY(EDI); COPY(ESI); COPY(EBP);
    COPY2(UESP, ESP); /* sic */
    COPY(EBX); COPY(EDX); COPY(ECX); COPY(EAX);
    COPY(EIP); COPY_SEG_CPL3(CS); COPY(EFL); COPY_SEG_CPL3(SS);

    COPY(R8); COPY(R9); COPY(R10); COPY(R11);
    COPY(R12); COPY(R13); COPY(R14); COPY(R15);
    COPY(RDI); COPY(RSI); COPY(RBP); COPY(RBX);
    COPY(RDX); COPY(RAX); COPY(RCX); COPY(RSP);
    COPY(RIP);
    COPY2(EFLAGS, EFL);
    COPY2(CS, CSGSFS);
    regs.gp[SS / sizeof(unsigned long)] = mc.gregs[REG_CSGSFS] >> 48;

    }
#[no_mangle]
pub unsafe extern "C" fn mc_set_rip(_mc: *mut c_void, target: *mut c_void) {
    void mc_set_rip(void *_mc, void *target)
    {
    mcontext_t *mc = _mc;

    mc.gregs[REG_EIP] = (unsigned long)target;

    mc.gregs[REG_RIP] = (unsigned long)target;

    }
// Same thing, but the copy macros are turned around.
#[no_mangle]
pub unsafe extern "C" fn get_mc_from_regs(regs: *mut uml_pt_regs, mc: *mut mcontext_t, single_stepping: c_int) {
    void get_mc_from_regs(struct uml_pt_regs *regs, mcontext_t *mc, int single_stepping)
    {

    COPY_SEG(GS); COPY_SEG(FS); COPY_SEG(ES); COPY_SEG(DS);
    COPY(EDI); COPY(ESI); COPY(EBP);
    COPY2(UESP, ESP); /* sic */
    COPY(EBX); COPY(EDX); COPY(ECX); COPY(EAX);
    COPY(EIP); COPY_SEG_CPL3(CS); COPY(EFL); COPY_SEG_CPL3(SS);

    COPY(R8); COPY(R9); COPY(R10); COPY(R11);
    COPY(R12); COPY(R13); COPY(R14); COPY(R15);
    COPY(RDI); COPY(RSI); COPY(RBP); COPY(RBX);
    COPY(RDX); COPY(RAX); COPY(RCX); COPY(RSP);
    COPY(RIP);
    COPY2(EFLAGS, EFL);
    mc.gregs[REG_CSGSFS] = mc.gregs[REG_CSGSFS] & 0xffffffffffffl;
    mc.gregs[REG_CSGSFS] |= (regs.gp[SS / sizeof(unsigned long)] & 0xffff) << 48;

    if (single_stepping)
    mc.gregs[REG_EFL] |= X86_EFLAGS_TF;
    else
    mc.gregs[REG_EFL] &= ~X86_EFLAGS_TF;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _xstate_64 {
    pub fpstate: _fpstate_64,
    pub xstate_hdr: _header,
    pub ymmh: _ymmh_state,
// New processor state extensions go here:
}

// Not quite the right structures as these contain more information
    int um_i387_from_fxsr(struct _fpstate_32 *i387,
    const struct _fpstate_64 *fxsave);
    int um_fxsr_from_i387(struct _fpstate_64 *fxsave,
    const struct _fpstate_32 *from);

    static struct _fpstate *get_fpstate(struct stub_data *data,
    mcontext_t *mcontext,
    int *fp_size)
    {
    struct _fpstate *res;
// Assume floating point registers are on the same page
    res = (void *)(((unsigned long)mcontext.fpregs &
    (UM_KERN_PAGE_SIZE - 1)) +
    (unsigned long)&data.sigstack[0]);
    if ((void *)res + sizeof(struct _fpstate) >
    (void *)data.sigstack + sizeof(data.sigstack))
    return core::ptr::null_mut();
    if (res.sw_reserved.magic1 != FP_XSTATE_MAGIC1) {
// fp_size = sizeof(struct _fpstate);
    } else {
    char *magic2_addr;
    magic2_addr = (void *)res;
    magic2_addr += res.sw_reserved.extended_size;
    magic2_addr -= FP_XSTATE_MAGIC2_SIZE;
// We still need to be within our stack
    if ((void *)magic2_addr >
    (void *)data.sigstack + sizeof(data.sigstack))
    return core::ptr::null_mut();
// If we do not read MAGIC2, then we did something wrong
    if (*(__u32 *)magic2_addr != FP_XSTATE_MAGIC2)
    return core::ptr::null_mut();
// Remove MAGIC2 from the size, we do not save/restore it
// fp_size = res->sw_reserved.extended_size -
    FP_XSTATE_MAGIC2_SIZE;
    }
    return res;
    }
    int get_stub_state(struct uml_pt_regs *regs, struct stub_data *data,
    unsigned long *fp_size_out)
    {
    mcontext_t *mcontext;
    struct _fpstate *fpstate_stub;
    struct _xstate_64 *xstate_stub;
    int fp_size, xstate_size;
// mctx_offset is verified by wait_stub_done_seccomp
    mcontext = (void *)&data.sigstack[data.mctx_offset];
    get_regs_from_mc(regs, mcontext);
    fpstate_stub = get_fpstate(data, mcontext, &fp_size);
    if (!fpstate_stub)
    return -EINVAL;

    xstate_stub = (void *)&fpstate_stub._fxsr_env;
    xstate_size = fp_size - offsetof(struct _fpstate_32, _fxsr_env);

    xstate_stub = (void *)fpstate_stub;
    xstate_size = fp_size;

    if (fp_size_out)
// fp_size_out = xstate_size;
    if (xstate_size > host_fp_size)
    return -ENOSPC;
    memcpy(&regs.fp, xstate_stub, xstate_size);
// We do not need to read the x86_64 FS_BASE/GS_BASE registers as
// we do not permit userspace to set them directly.
//

// Read the i387 legacy FP registers
    if (um_fxsr_from_i387((void *)&regs.fp, fpstate_stub))
    return -EINVAL;

    return 0;
    }
// Copied because we cannot include regset.h here.
    struct task_struct;
    struct user_regset;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct membuf {
    pub p: *mut c_void,
    pub left: usize,
}

    int fpregs_legacy_get(struct task_struct *target,
    const struct user_regset *regset,
    struct membuf to);
    int set_stub_state(struct uml_pt_regs *regs, struct stub_data *data,
    int single_stepping)
    {
    mcontext_t *mcontext;
    struct _fpstate *fpstate_stub;
    struct _xstate_64 *xstate_stub;
    int fp_size, xstate_size;
// mctx_offset is verified by wait_stub_done_seccomp
    mcontext = (void *)&data.sigstack[data.mctx_offset];
    if ((unsigned long)mcontext < (unsigned long)data.sigstack ||
    (unsigned long)mcontext >
    (unsigned long) data.sigstack +
    sizeof(data.sigstack) - sizeof(*mcontext))
    return -EINVAL;
    get_mc_from_regs(regs, mcontext, single_stepping);
    fpstate_stub = get_fpstate(data, mcontext, &fp_size);
    if (!fpstate_stub)
    return -EINVAL;

    xstate_stub = (void *)&fpstate_stub._fxsr_env;
    xstate_size = fp_size - offsetof(struct _fpstate_32, _fxsr_env);

    xstate_stub = (void *)fpstate_stub;
    xstate_size = fp_size;

    memcpy(xstate_stub, &regs.fp, xstate_size);

//
// On x86, the GDT entries are updated by arch_set_tls.
//
// Store the i387 legacy FP registers which the host will use
    if (um_i387_from_fxsr(fpstate_stub, (void *)&regs.fp))
    return -EINVAL;

//
// On x86_64, we need to sync the FS_BASE/GS_BASE registers using the
// arch specific data.
//
    if (data.arch_data.fs_base != regs.gp[FS_BASE / sizeof(unsigned long)]) {
    data.arch_data.fs_base = regs.gp[FS_BASE / sizeof(unsigned long)];
    data.arch_data.sync |= STUB_SYNC_FS_BASE;
    }
    if (data.arch_data.gs_base != regs.gp[GS_BASE / sizeof(unsigned long)]) {
    data.arch_data.gs_base = regs.gp[GS_BASE / sizeof(unsigned long)];
    data.arch_data.sync |= STUB_SYNC_GS_BASE;
    }

    return 0;
    }
