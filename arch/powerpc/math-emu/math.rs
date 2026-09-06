//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/math-emu/math.c
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
// Copyright (C) 1999  Eddie C. Dost  (ecd@atecom.com)
//

// The instructions list which may be not implemented by a hardware FPU
    FLOATFUNC(fre);
    FLOATFUNC(frsqrtes);
    FLOATFUNC(fsqrt);
    FLOATFUNC(fsqrts);
    FLOATFUNC(mtfsf);
    FLOATFUNC(mtfsfi);

    void *op4) { return 0; }

    FLOATFUNC(fadd);
    FLOATFUNC(fadds);
    FLOATFUNC(fdiv);
    FLOATFUNC(fdivs);
    FLOATFUNC(fmul);
    FLOATFUNC(fmuls);
    FLOATFUNC(fsub);
    FLOATFUNC(fsubs);
    FLOATFUNC(fmadd);
    FLOATFUNC(fmadds);
    FLOATFUNC(fmsub);
    FLOATFUNC(fmsubs);
    FLOATFUNC(fnmadd);
    FLOATFUNC(fnmadds);
    FLOATFUNC(fnmsub);
    FLOATFUNC(fnmsubs);
    FLOATFUNC(fctiw);
    FLOATFUNC(fctiwz);
    FLOATFUNC(frsp);
    FLOATFUNC(fcmpo);
    FLOATFUNC(fcmpu);
    FLOATFUNC(mcrfs);
    FLOATFUNC(mffs);
    FLOATFUNC(mtfsb0);
    FLOATFUNC(mtfsb1);
    FLOATFUNC(lfd);
    FLOATFUNC(lfs);
    FLOATFUNC(stfd);
    FLOATFUNC(stfs);
    FLOATFUNC(stfiwx);
    FLOATFUNC(fabs);
    FLOATFUNC(fmr);
    FLOATFUNC(fnabs);
    FLOATFUNC(fneg);
// Optional
    FLOATFUNC(fres);
    FLOATFUNC(frsqrte);
    FLOATFUNC(fsel);
pub const OP31: c_uint = 0x1f		/*   31 */;
pub const LFS: c_uint = 0x30		/*   48 */;
pub const LFSU: c_uint = 0x31		/*   49 */;
pub const LFD: c_uint = 0x32		/*   50 */;
pub const LFDU: c_uint = 0x33		/*   51 */;
pub const STFS: c_uint = 0x34		/*   52 */;
pub const STFSU: c_uint = 0x35		/*   53 */;
pub const STFD: c_uint = 0x36		/*   54 */;
pub const STFDU: c_uint = 0x37		/*   55 */;
pub const OP59: c_uint = 0x3b		/*   59 */;
pub const OP63: c_uint = 0x3f		/*   63 */;
// Opcode 31:
// X-Form:
pub const LFSX: c_uint = 0x217		/*  535 */;
pub const LFSUX: c_uint = 0x237		/*  567 */;
pub const LFDX: c_uint = 0x257		/*  599 */;
pub const LFDUX: c_uint = 0x277		/*  631 */;
pub const STFSX: c_uint = 0x297		/*  663 */;
pub const STFSUX: c_uint = 0x2b7		/*  695 */;
pub const STFDX: c_uint = 0x2d7		/*  727 */;
pub const STFDUX: c_uint = 0x2f7		/*  759 */;
pub const STFIWX: c_uint = 0x3d7		/*  983 */;
// Opcode 59:
// A-Form:
pub const FDIVS: c_uint = 0x012		/*   18 */;
pub const FSUBS: c_uint = 0x014		/*   20 */;
pub const FADDS: c_uint = 0x015		/*   21 */;
pub const FSQRTS: c_uint = 0x016		/*   22 */;
pub const FRES: c_uint = 0x018		/*   24 */;
pub const FMULS: c_uint = 0x019		/*   25 */;
pub const FRSQRTES: c_uint = 0x01a		/*   26 */;
pub const FMSUBS: c_uint = 0x01c		/*   28 */;
pub const FMADDS: c_uint = 0x01d		/*   29 */;
pub const FNMSUBS: c_uint = 0x01e		/*   30 */;
pub const FNMADDS: c_uint = 0x01f		/*   31 */;
// Opcode 63:
// A-Form:
pub const FDIV: c_uint = 0x012		/*   18 */;
pub const FSUB: c_uint = 0x014		/*   20 */;
pub const FADD: c_uint = 0x015		/*   21 */;
pub const FSQRT: c_uint = 0x016		/*   22 */;
pub const FSEL: c_uint = 0x017		/*   23 */;
pub const FRE: c_uint = 0x018		/*   24 */;
pub const FMUL: c_uint = 0x019		/*   25 */;
pub const FRSQRTE: c_uint = 0x01a		/*   26 */;
pub const FMSUB: c_uint = 0x01c		/*   28 */;
pub const FMADD: c_uint = 0x01d		/*   29 */;
pub const FNMSUB: c_uint = 0x01e		/*   30 */;
pub const FNMADD: c_uint = 0x01f		/*   31 */;
// X-Form:
pub const FCMPU: c_uint = 0x000		/*    0	*/;
pub const FRSP: c_uint = 0x00c		/*   12 */;
pub const FCTIW: c_uint = 0x00e		/*   14 */;
pub const FCTIWZ: c_uint = 0x00f		/*   15 */;
pub const FCMPO: c_uint = 0x020		/*   32 */;
pub const MTFSB1: c_uint = 0x026		/*   38 */;
pub const FNEG: c_uint = 0x028		/*   40 */;
pub const MCRFS: c_uint = 0x040		/*   64 */;
pub const MTFSB0: c_uint = 0x046		/*   70 */;
pub const FMR: c_uint = 0x048		/*   72 */;
pub const MTFSFI: c_uint = 0x086		/*  134 */;
pub const FNABS: c_uint = 0x088		/*  136 */;
pub const FABS: c_uint = 0x108		/*  264 */;
pub const MFFS: c_uint = 0x247		/*  583 */;
pub const MTFSF: c_uint = 0x2c7		/*  711 */;
pub const AB: c_int = 2;
pub const AC: c_int = 3;
pub const ABC: c_int = 4;
pub const D: c_int = 5;
pub const DU: c_int = 6;
pub const X: c_int = 7;
pub const XA: c_int = 8;
pub const XB: c_int = 9;
pub const XCR: c_int = 11;
pub const XCRB: c_int = 12;
pub const XCRI: c_int = 13;
pub const XCRL: c_int = 16;
pub const XE: c_int = 14;
pub const XEU: c_int = 15;
pub const XFLB: c_int = 10;
    static int
    record_exception(struct pt_regs *regs, int eflag)
    {
    u32 fpscr;
    fpscr = __FPU_FPSCR;
    if (eflag) {
    fpscr |= FPSCR_FX;
    if (eflag & EFLAG_OVERFLOW)
    fpscr |= FPSCR_OX;
    if (eflag & EFLAG_UNDERFLOW)
    fpscr |= FPSCR_UX;
    if (eflag & EFLAG_DIVZERO)
    fpscr |= FPSCR_ZX;
    if (eflag & EFLAG_INEXACT)
    fpscr |= FPSCR_XX;
    if (eflag & EFLAG_INVALID)
    fpscr |= FPSCR_VX;
    if (eflag & EFLAG_VXSNAN)
    fpscr |= FPSCR_VXSNAN;
    if (eflag & EFLAG_VXISI)
    fpscr |= FPSCR_VXISI;
    if (eflag & EFLAG_VXIDI)
    fpscr |= FPSCR_VXIDI;
    if (eflag & EFLAG_VXZDZ)
    fpscr |= FPSCR_VXZDZ;
    if (eflag & EFLAG_VXIMZ)
    fpscr |= FPSCR_VXIMZ;
    if (eflag & EFLAG_VXVC)
    fpscr |= FPSCR_VXVC;
    if (eflag & EFLAG_VXSOFT)
    fpscr |= FPSCR_VXSOFT;
    if (eflag & EFLAG_VXSQRT)
    fpscr |= FPSCR_VXSQRT;
    if (eflag & EFLAG_VXCVI)
    fpscr |= FPSCR_VXCVI;
    }
// fpscr &= ~(FPSCR_VX);
    if (fpscr & (FPSCR_VXSNAN | FPSCR_VXISI | FPSCR_VXIDI |
    FPSCR_VXZDZ | FPSCR_VXIMZ | FPSCR_VXVC |
    FPSCR_VXSOFT | FPSCR_VXSQRT | FPSCR_VXCVI))
    fpscr |= FPSCR_VX;
    fpscr &= ~(FPSCR_FEX);
    if (((fpscr & FPSCR_VX) && (fpscr & FPSCR_VE)) ||
    ((fpscr & FPSCR_OX) && (fpscr & FPSCR_OE)) ||
    ((fpscr & FPSCR_UX) && (fpscr & FPSCR_UE)) ||
    ((fpscr & FPSCR_ZX) && (fpscr & FPSCR_ZE)) ||
    ((fpscr & FPSCR_XX) && (fpscr & FPSCR_XE)))
    fpscr |= FPSCR_FEX;
    __FPU_FPSCR = fpscr;
    return (fpscr & FPSCR_FEX) ? 1 : 0;
    }
    int
    do_mathemu(struct pt_regs *regs)
    {
    void *op0 = core::ptr::null_mut(), *op1 = core::ptr::null_mut(), *op2 = core::ptr::null_mut(), *op3 = core::ptr::null_mut();
    let mut pc: c_ulong = regs.nip;
    signed short sdisp;
    let mut insn: u32 = 0;
    let mut idx: c_int = 0;
    int (*func)(void *, void *, void *, void *);
    let mut type: c_int = 0;
    int eflag, trap;
    if (get_user(insn, (u32 __user *)pc))
    return -EFAULT;
    switch (insn >> 26) {
    case LFS:	func = lfs;	type = D;	break;
    case LFSU:	func = lfs;	type = DU;	break;
    case LFD:	func = lfd;	type = D;	break;
    case LFDU:	func = lfd;	type = DU;	break;
    case STFS:	func = stfs;	type = D;	break;
    case STFSU:	func = stfs;	type = DU;	break;
    case STFD:	func = stfd;	type = D;	break;
    case STFDU:	func = stfd;	type = DU;	break;
    case OP31:
    switch ((insn >> 1) & 0x3ff) {
    case LFSX:	func = lfs;	type = XE;	break;
    case LFSUX:	func = lfs;	type = XEU;	break;
    case LFDX:	func = lfd;	type = XE;	break;
    case LFDUX:	func = lfd;	type = XEU;	break;
    case STFSX:	func = stfs;	type = XE;	break;
    case STFSUX:	func = stfs;	type = XEU;	break;
    case STFDX:	func = stfd;	type = XE;	break;
    case STFDUX:	func = stfd;	type = XEU;	break;
    case STFIWX:	func = stfiwx;	type = XE;	break;
    default:
    goto illegal;
    }
    break;
    case OP59:
    switch ((insn >> 1) & 0x1f) {
    case FDIVS:	func = fdivs;	type = AB;	break;
    case FSUBS:	func = fsubs;	type = AB;	break;
    case FADDS:	func = fadds;	type = AB;	break;
    case FSQRTS:	func = fsqrts;	type = XB;	break;
    case FRES:	func = fres;	type = XB;	break;
    case FMULS:	func = fmuls;	type = AC;	break;
    case FRSQRTES:	func = frsqrtes;type = XB;	break;
    case FMSUBS:	func = fmsubs;	type = ABC;	break;
    case FMADDS:	func = fmadds;	type = ABC;	break;
    case FNMSUBS:	func = fnmsubs;	type = ABC;	break;
    case FNMADDS:	func = fnmadds;	type = ABC;	break;
    default:
    goto illegal;
    }
    break;
    case OP63:
    if (insn & 0x20) {
    switch ((insn >> 1) & 0x1f) {
    case FDIV:	func = fdiv;	type = AB;	break;
    case FSUB:	func = fsub;	type = AB;	break;
    case FADD:	func = fadd;	type = AB;	break;
    case FSQRT:	func = fsqrt;	type = XB;	break;
    case FRE:	func = fre;	type = XB;	break;
    case FSEL:	func = fsel;	type = ABC;	break;
    case FMUL:	func = fmul;	type = AC;	break;
    case FRSQRTE:	func = frsqrte;	type = XB;	break;
    case FMSUB:	func = fmsub;	type = ABC;	break;
    case FMADD:	func = fmadd;	type = ABC;	break;
    case FNMSUB:	func = fnmsub;	type = ABC;	break;
    case FNMADD:	func = fnmadd;	type = ABC;	break;
    default:
    goto illegal;
    }
    break;
    }
    switch ((insn >> 1) & 0x3ff) {
    case FCMPU:	func = fcmpu;	type = XCR;	break;
    case FRSP:	func = frsp;	type = XB;	break;
    case FCTIW:	func = fctiw;	type = XB;	break;
    case FCTIWZ:	func = fctiwz;	type = XB;	break;
    case FCMPO:	func = fcmpo;	type = XCR;	break;
    case MTFSB1:	func = mtfsb1;	type = XCRB;	break;
    case FNEG:	func = fneg;	type = XB;	break;
    case MCRFS:	func = mcrfs;	type = XCRL;	break;
    case MTFSB0:	func = mtfsb0;	type = XCRB;	break;
    case FMR:	func = fmr;	type = XB;	break;
    case MTFSFI:	func = mtfsfi;	type = XCRI;	break;
    case FNABS:	func = fnabs;	type = XB;	break;
    case FABS:	func = fabs;	type = XB;	break;
    case MFFS:	func = mffs;	type = X;	break;
    case MTFSF:	func = mtfsf;	type = XFLB;	break;
    default:
    goto illegal;
    }
    break;
    default:
    goto illegal;
    }
    switch (type) {
    case AB:
    op0 = (void *)&current.thread.TS_FPR((insn >> 21) & 0x1f);
    op1 = (void *)&current.thread.TS_FPR((insn >> 16) & 0x1f);
    op2 = (void *)&current.thread.TS_FPR((insn >> 11) & 0x1f);
    break;
    case AC:
    op0 = (void *)&current.thread.TS_FPR((insn >> 21) & 0x1f);
    op1 = (void *)&current.thread.TS_FPR((insn >> 16) & 0x1f);
    op2 = (void *)&current.thread.TS_FPR((insn >>  6) & 0x1f);
    break;
    case ABC:
    op0 = (void *)&current.thread.TS_FPR((insn >> 21) & 0x1f);
    op1 = (void *)&current.thread.TS_FPR((insn >> 16) & 0x1f);
    op2 = (void *)&current.thread.TS_FPR((insn >> 11) & 0x1f);
    op3 = (void *)&current.thread.TS_FPR((insn >>  6) & 0x1f);
    break;
    case D:
    idx = (insn >> 16) & 0x1f;
    sdisp = (insn & 0xffff);
    op0 = (void *)&current.thread.TS_FPR((insn >> 21) & 0x1f);
    op1 = (void *)((idx ? regs.gpr[idx] : 0) + sdisp);
    break;
    case DU:
    idx = (insn >> 16) & 0x1f;
    if (!idx)
    goto illegal;
    sdisp = (insn & 0xffff);
    op0 = (void *)&current.thread.TS_FPR((insn >> 21) & 0x1f);
    op1 = (void *)(regs.gpr[idx] + sdisp);
    break;
    case X:
    op0 = (void *)&current.thread.TS_FPR((insn >> 21) & 0x1f);
    break;
    case XA:
    op0 = (void *)&current.thread.TS_FPR((insn >> 21) & 0x1f);
    op1 = (void *)&current.thread.TS_FPR((insn >> 16) & 0x1f);
    break;
    case XB:
    op0 = (void *)&current.thread.TS_FPR((insn >> 21) & 0x1f);
    op1 = (void *)&current.thread.TS_FPR((insn >> 11) & 0x1f);
    break;
    case XE:
    idx = (insn >> 16) & 0x1f;
    op0 = (void *)&current.thread.TS_FPR((insn >> 21) & 0x1f);
    op1 = (void *)((idx ? regs.gpr[idx] : 0)
    + regs.gpr[(insn >> 11) & 0x1f]);
    break;
    case XEU:
    idx = (insn >> 16) & 0x1f;
    if (!idx)
    goto illegal;
    op0 = (void *)&current.thread.TS_FPR((insn >> 21) & 0x1f);
    op1 = (void *)(regs.gpr[idx]
    + regs.gpr[(insn >> 11) & 0x1f]);
    break;
    case XCR:
    op0 = (void *)&regs.ccr;
    op1 = (void *)(long)((insn >> 23) & 0x7);
    op2 = (void *)&current.thread.TS_FPR((insn >> 16) & 0x1f);
    op3 = (void *)&current.thread.TS_FPR((insn >> 11) & 0x1f);
    break;
    case XCRL:
    op0 = (void *)&regs.ccr;
    op1 = (void *)(long)((insn >> 23) & 0x7);
    op2 = (void *)(long)((insn >> 18) & 0x7);
    break;
    case XCRB:
    op0 = (void *)(long)((insn >> 21) & 0x1f);
    break;
    case XCRI:
    op0 = (void *)(long)((insn >> 23) & 0x7);
    op1 = (void *)(long)((insn >> 12) & 0xf);
    break;
    case XFLB:
    op0 = (void *)(long)((insn >> 17) & 0xff);
    op1 = (void *)&current.thread.TS_FPR((insn >> 11) & 0x1f);
    break;
    default:
    goto illegal;
    }
//
// If we support a HW FPU, we need to ensure the FP state
// is flushed into the thread_struct before attempting
// emulation
//
    flush_fp_to_thread(current);
    eflag = func(op0, op1, op2, op3);
    if (insn & 1) {
    regs.ccr &= ~(0x0f000000);
    regs.ccr |= (__FPU_FPSCR >> 4) & 0x0f000000;
    }
    trap = record_exception(regs, eflag);
    if (trap)
    return 1;
    switch (type) {
    case DU:
    case XEU:
    regs.gpr[idx] = (unsigned long)op1;
    break;
    default:
    break;
    }
    regs_add_return_ip(regs, 4);
    return 0;
    illegal:
    return -ENOSYS;
    }
