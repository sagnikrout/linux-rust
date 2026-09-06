//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kvm/book3s_paired_singles.c
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
// Copyright Novell Inc 2010
//
// Authors: Alexander Graf <agraf@suse.de>
//

// #define DEBUG

pub const OP_LFS: c_int = 48;
pub const OP_LFSU: c_int = 49;
pub const OP_LFD: c_int = 50;
pub const OP_LFDU: c_int = 51;
pub const OP_STFS: c_int = 52;
pub const OP_STFSU: c_int = 53;
pub const OP_STFD: c_int = 54;
pub const OP_STFDU: c_int = 55;
pub const OP_PSQ_L: c_int = 56;
pub const OP_PSQ_LU: c_int = 57;
pub const OP_PSQ_ST: c_int = 60;
pub const OP_PSQ_STU: c_int = 61;
pub const OP_31_LFSX: c_int = 535;
pub const OP_31_LFSUX: c_int = 567;
pub const OP_31_LFDX: c_int = 599;
pub const OP_31_LFDUX: c_int = 631;
pub const OP_31_STFSX: c_int = 663;
pub const OP_31_STFSUX: c_int = 695;
pub const OP_31_STFX: c_int = 727;
pub const OP_31_STFUX: c_int = 759;
pub const OP_31_LWIZX: c_int = 887;
pub const OP_31_STFIWX: c_int = 983;
pub const OP_59_FADDS: c_int = 21;
pub const OP_59_FSUBS: c_int = 20;
pub const OP_59_FSQRTS: c_int = 22;
pub const OP_59_FDIVS: c_int = 18;
pub const OP_59_FRES: c_int = 24;
pub const OP_59_FMULS: c_int = 25;
pub const OP_59_FRSQRTES: c_int = 26;
pub const OP_59_FMSUBS: c_int = 28;
pub const OP_59_FMADDS: c_int = 29;
pub const OP_59_FNMSUBS: c_int = 30;
pub const OP_59_FNMADDS: c_int = 31;
pub const OP_63_FCMPU: c_int = 0;
pub const OP_63_FCPSGN: c_int = 8;
pub const OP_63_FRSP: c_int = 12;
pub const OP_63_FCTIW: c_int = 14;
pub const OP_63_FCTIWZ: c_int = 15;
pub const OP_63_FDIV: c_int = 18;
pub const OP_63_FADD: c_int = 21;
pub const OP_63_FSQRT: c_int = 22;
pub const OP_63_FSEL: c_int = 23;
pub const OP_63_FRE: c_int = 24;
pub const OP_63_FMUL: c_int = 25;
pub const OP_63_FRSQRTE: c_int = 26;
pub const OP_63_FMSUB: c_int = 28;
pub const OP_63_FMADD: c_int = 29;
pub const OP_63_FNMSUB: c_int = 30;
pub const OP_63_FNMADD: c_int = 31;
pub const OP_63_FCMPO: c_int = 32;

pub const OP_63_FSUB: c_int = 20;
pub const OP_63_FNEG: c_int = 40;
pub const OP_63_MCRFS: c_int = 64;
pub const OP_63_MTFSB0: c_int = 70;
pub const OP_63_FMR: c_int = 72;
pub const OP_63_MTFSFI: c_int = 134;
pub const OP_63_FABS: c_int = 264;
pub const OP_63_MFFS: c_int = 583;
pub const OP_63_MTFSF: c_int = 711;
pub const OP_4X_PS_CMPU0: c_int = 0;
pub const OP_4X_PSQ_LX: c_int = 6;
pub const OP_4XW_PSQ_STX: c_int = 7;
pub const OP_4A_PS_SUM0: c_int = 10;
pub const OP_4A_PS_SUM1: c_int = 11;
pub const OP_4A_PS_MULS0: c_int = 12;
pub const OP_4A_PS_MULS1: c_int = 13;
pub const OP_4A_PS_MADDS0: c_int = 14;
pub const OP_4A_PS_MADDS1: c_int = 15;
pub const OP_4A_PS_DIV: c_int = 18;
pub const OP_4A_PS_SUB: c_int = 20;
pub const OP_4A_PS_ADD: c_int = 21;
pub const OP_4A_PS_SEL: c_int = 23;
pub const OP_4A_PS_RES: c_int = 24;
pub const OP_4A_PS_MUL: c_int = 25;
pub const OP_4A_PS_RSQRTE: c_int = 26;
pub const OP_4A_PS_MSUB: c_int = 28;
pub const OP_4A_PS_MADD: c_int = 29;
pub const OP_4A_PS_NMSUB: c_int = 30;
pub const OP_4A_PS_NMADD: c_int = 31;
pub const OP_4X_PS_CMPO0: c_int = 32;
pub const OP_4X_PSQ_LUX: c_int = 38;
pub const OP_4XW_PSQ_STUX: c_int = 39;
pub const OP_4X_PS_NEG: c_int = 40;
pub const OP_4X_PS_CMPU1: c_int = 64;
pub const OP_4X_PS_MR: c_int = 72;
pub const OP_4X_PS_CMPO1: c_int = 96;
pub const OP_4X_PS_NABS: c_int = 136;
pub const OP_4X_PS_ABS: c_int = 264;
pub const OP_4X_PS_MERGE00: c_int = 528;
pub const OP_4X_PS_MERGE01: c_int = 560;
pub const OP_4X_PS_MERGE10: c_int = 592;
pub const OP_4X_PS_MERGE11: c_int = 624;
pub const SCALAR_NONE: c_int = 0;

pub const GQR_ST_TYPE_MASK: c_uint = 0x00000007;
pub const GQR_ST_TYPE_SHIFT: c_int = 0;
pub const GQR_ST_SCALE_MASK: c_uint = 0x00003f00;
pub const GQR_ST_SCALE_SHIFT: c_int = 8;
pub const GQR_LD_TYPE_MASK: c_uint = 0x00070000;
pub const GQR_LD_TYPE_SHIFT: c_int = 16;
pub const GQR_LD_SCALE_MASK: c_uint = 0x3f000000;
pub const GQR_LD_SCALE_SHIFT: c_int = 24;
pub const GQR_QUANTIZE_FLOAT: c_int = 0;
pub const GQR_QUANTIZE_U8: c_int = 4;
pub const GQR_QUANTIZE_U16: c_int = 5;
pub const GQR_QUANTIZE_S8: c_int = 6;
pub const GQR_QUANTIZE_S16: c_int = 7;
pub const FPU_LS_SINGLE: c_int = 0;
pub const FPU_LS_DOUBLE: c_int = 1;
pub const FPU_LS_SINGLE_LOW: c_int = 2;
#[no_mangle]
pub unsafe extern "C" fn kvmppc_sync_qpr(vcpu: *mut kvm_vcpu, rt: c_int) {
    static inline void kvmppc_sync_qpr(struct kvm_vcpu *vcpu, int rt)
    {
    kvm_cvt_df(&VCPU_FPR(vcpu, rt), &vcpu.arch.qpr[rt]);
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_inject_pf(vcpu: *mut kvm_vcpu, eaddr: c_ulong, is_store: bool) {
    static void kvmppc_inject_pf(struct kvm_vcpu *vcpu, ulong eaddr, bool is_store)
    {
    u32 dsisr;
    let mut msr: u64 = kvmppc_get_msr(vcpu);
    msr = kvmppc_set_field(msr, 33, 36, 0);
    msr = kvmppc_set_field(msr, 42, 47, 0);
    kvmppc_set_msr(vcpu, msr);
    kvmppc_set_dar(vcpu, eaddr);
// Page Fault
    dsisr = kvmppc_set_field(0, 33, 33, 1);
    if (is_store)
    dsisr = kvmppc_set_field(dsisr, 38, 38, 1);
    kvmppc_set_dsisr(vcpu, dsisr);
    kvmppc_book3s_queue_irqprio(vcpu, BOOK3S_INTERRUPT_DATA_STORAGE);
    }
    static int kvmppc_emulate_fpr_load(struct kvm_vcpu *vcpu,
    int rs, ulong addr, int ls_type)
    {
    let mut emulated: c_int = EMULATE_FAIL;
    int r;
    char tmp[8];
    let mut len: c_int = sizeof(u32);
    if (ls_type == FPU_LS_DOUBLE)
    len = sizeof(u64);
// read from memory
    r = kvmppc_ld(vcpu, &addr, len, tmp, true);
    vcpu.arch.paddr_accessed = addr;
    if (r < 0) {
    kvmppc_inject_pf(vcpu, addr, false);
    goto done_load;
    } else if (r == EMULATE_DO_MMIO) {
    emulated = kvmppc_handle_load(vcpu, KVM_MMIO_REG_FPR | rs,
    len, 1);
    goto done_load;
    }
    emulated = EMULATE_DONE;
// put in registers
    switch (ls_type) {
    case FPU_LS_SINGLE:
    kvm_cvt_fd((u32*)tmp, &VCPU_FPR(vcpu, rs));
    vcpu.arch.qpr[rs] = *((u32*)tmp);
    break;
    case FPU_LS_DOUBLE:
    VCPU_FPR(vcpu, rs) = *((u64*)tmp);
    break;
    }
    dprintk(KERN_INFO "KVM: FPR_LD [0x%llx] at 0x%lx (%d)\n", *(u64*)tmp,
    addr, len);
    done_load:
    return emulated;
    }
    static int kvmppc_emulate_fpr_store(struct kvm_vcpu *vcpu,
    int rs, ulong addr, int ls_type)
    {
    let mut emulated: c_int = EMULATE_FAIL;
    int r;
    char tmp[8];
    u64 val;
    int len;
    switch (ls_type) {
    case FPU_LS_SINGLE:
    kvm_cvt_df(&VCPU_FPR(vcpu, rs), (u32*)tmp);
    val = *((u32*)tmp);
    len = sizeof(u32);
    break;
    case FPU_LS_SINGLE_LOW:
// ((u32*)tmp) = VCPU_FPR(vcpu, rs);
    val = VCPU_FPR(vcpu, rs) & 0xffffffff;
    len = sizeof(u32);
    break;
    case FPU_LS_DOUBLE:
// ((u64*)tmp) = VCPU_FPR(vcpu, rs);
    val = VCPU_FPR(vcpu, rs);
    len = sizeof(u64);
    break;
    default:
    val = 0;
    len = 0;
    }
    r = kvmppc_st(vcpu, &addr, len, tmp, true);
    vcpu.arch.paddr_accessed = addr;
    if (r < 0) {
    kvmppc_inject_pf(vcpu, addr, true);
    } else if (r == EMULATE_DO_MMIO) {
    emulated = kvmppc_handle_store(vcpu, val, len, 1);
    } else {
    emulated = EMULATE_DONE;
    }
    dprintk(KERN_INFO "KVM: FPR_ST [0x%llx] at 0x%lx (%d)\n",
    val, addr, len);
    return emulated;
    }
    static int kvmppc_emulate_psq_load(struct kvm_vcpu *vcpu,
    int rs, ulong addr, bool w, int i)
    {
    let mut emulated: c_int = EMULATE_FAIL;
    int r;
    let mut one: float = 1.0;
    u32 tmp[2];
// read from memory
    if (w) {
    r = kvmppc_ld(vcpu, &addr, sizeof(u32), tmp, true);
    memcpy(&tmp[1], &one, sizeof(u32));
    } else {
    r = kvmppc_ld(vcpu, &addr, sizeof(u32) * 2, tmp, true);
    }
    vcpu.arch.paddr_accessed = addr;
    if (r < 0) {
    kvmppc_inject_pf(vcpu, addr, false);
    goto done_load;
    } else if ((r == EMULATE_DO_MMIO) && w) {
    emulated = kvmppc_handle_load(vcpu, KVM_MMIO_REG_FPR | rs,
    4, 1);
    vcpu.arch.qpr[rs] = tmp[1];
    goto done_load;
    } else if (r == EMULATE_DO_MMIO) {
    emulated = kvmppc_handle_load(vcpu, KVM_MMIO_REG_FQPR | rs,
    8, 1);
    goto done_load;
    }
    emulated = EMULATE_DONE;
// put in registers
    kvm_cvt_fd(&tmp[0], &VCPU_FPR(vcpu, rs));
    vcpu.arch.qpr[rs] = tmp[1];
    dprintk(KERN_INFO "KVM: PSQ_LD [0x%x, 0x%x] at 0x%lx (%d)\n", tmp[0],
    tmp[1], addr, w ? 4 : 8);
    done_load:
    return emulated;
    }
    static int kvmppc_emulate_psq_store(struct kvm_vcpu *vcpu,
    int rs, ulong addr, bool w, int i)
    {
    let mut emulated: c_int = EMULATE_FAIL;
    int r;
    u32 tmp[2];
    let mut len: c_int = w ? sizeof(u32) : sizeof(u64);
    kvm_cvt_df(&VCPU_FPR(vcpu, rs), &tmp[0]);
    tmp[1] = vcpu.arch.qpr[rs];
    r = kvmppc_st(vcpu, &addr, len, tmp, true);
    vcpu.arch.paddr_accessed = addr;
    if (r < 0) {
    kvmppc_inject_pf(vcpu, addr, true);
    } else if ((r == EMULATE_DO_MMIO) && w) {
    emulated = kvmppc_handle_store(vcpu, tmp[0], 4, 1);
    } else if (r == EMULATE_DO_MMIO) {
    let mut val: u64 = ((u64)tmp[0] << 32) | tmp[1];
    emulated = kvmppc_handle_store(vcpu, val, 8, 1);
    } else {
    emulated = EMULATE_DONE;
    }
    dprintk(KERN_INFO "KVM: PSQ_ST [0x%x, 0x%x] at 0x%lx (%d)\n",
    tmp[0], tmp[1], addr, len);
    return emulated;
    }
//
// Cuts out inst bits with ordering according to spec.
// That means the leftmost bit is zero. All given bits are included.
//
#[no_mangle]
pub unsafe extern "C" fn inst_get_field(inst: u32, msb: c_int, lsb: c_int) -> u32 {
    static inline u32 inst_get_field(u32 inst, int msb, int lsb)
    {
    return kvmppc_get_field(inst, msb + 32, lsb + 32);
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_inst_is_paired_single(vcpu: *mut kvm_vcpu, inst: u32) -> bool {
    static bool kvmppc_inst_is_paired_single(struct kvm_vcpu *vcpu, u32 inst)
    {
    if (!(vcpu.arch.hflags & BOOK3S_HFLAG_PAIRED_SINGLE))
    return false;
    switch (get_op(inst)) {
    case OP_PSQ_L:
    case OP_PSQ_LU:
    case OP_PSQ_ST:
    case OP_PSQ_STU:
    case OP_LFS:
    case OP_LFSU:
    case OP_LFD:
    case OP_LFDU:
    case OP_STFS:
    case OP_STFSU:
    case OP_STFD:
    case OP_STFDU:
    return true;
    case 4:
// X form
    switch (inst_get_field(inst, 21, 30)) {
    case OP_4X_PS_CMPU0:
    case OP_4X_PSQ_LX:
    case OP_4X_PS_CMPO0:
    case OP_4X_PSQ_LUX:
    case OP_4X_PS_NEG:
    case OP_4X_PS_CMPU1:
    case OP_4X_PS_MR:
    case OP_4X_PS_CMPO1:
    case OP_4X_PS_NABS:
    case OP_4X_PS_ABS:
    case OP_4X_PS_MERGE00:
    case OP_4X_PS_MERGE01:
    case OP_4X_PS_MERGE10:
    case OP_4X_PS_MERGE11:
    return true;
    }
// XW form
    switch (inst_get_field(inst, 25, 30)) {
    case OP_4XW_PSQ_STX:
    case OP_4XW_PSQ_STUX:
    return true;
    }
// A form
    switch (inst_get_field(inst, 26, 30)) {
    case OP_4A_PS_SUM1:
    case OP_4A_PS_SUM0:
    case OP_4A_PS_MULS0:
    case OP_4A_PS_MULS1:
    case OP_4A_PS_MADDS0:
    case OP_4A_PS_MADDS1:
    case OP_4A_PS_DIV:
    case OP_4A_PS_SUB:
    case OP_4A_PS_ADD:
    case OP_4A_PS_SEL:
    case OP_4A_PS_RES:
    case OP_4A_PS_MUL:
    case OP_4A_PS_RSQRTE:
    case OP_4A_PS_MSUB:
    case OP_4A_PS_MADD:
    case OP_4A_PS_NMSUB:
    case OP_4A_PS_NMADD:
    return true;
    }
    break;
    case 59:
    switch (inst_get_field(inst, 21, 30)) {
    case OP_59_FADDS:
    case OP_59_FSUBS:
    case OP_59_FDIVS:
    case OP_59_FRES:
    case OP_59_FRSQRTES:
    return true;
    }
    switch (inst_get_field(inst, 26, 30)) {
    case OP_59_FMULS:
    case OP_59_FMSUBS:
    case OP_59_FMADDS:
    case OP_59_FNMSUBS:
    case OP_59_FNMADDS:
    return true;
    }
    break;
    case 63:
    switch (inst_get_field(inst, 21, 30)) {
    case OP_63_MTFSB0:
    case OP_63_MTFSB1:
    case OP_63_MTFSF:
    case OP_63_MTFSFI:
    case OP_63_MCRFS:
    case OP_63_MFFS:
    case OP_63_FCMPU:
    case OP_63_FCMPO:
    case OP_63_FNEG:
    case OP_63_FMR:
    case OP_63_FABS:
    case OP_63_FRSP:
    case OP_63_FDIV:
    case OP_63_FADD:
    case OP_63_FSUB:
    case OP_63_FCTIW:
    case OP_63_FCTIWZ:
    case OP_63_FRSQRTE:
    case OP_63_FCPSGN:
    return true;
    }
    switch (inst_get_field(inst, 26, 30)) {
    case OP_63_FMUL:
    case OP_63_FSEL:
    case OP_63_FMSUB:
    case OP_63_FMADD:
    case OP_63_FNMSUB:
    case OP_63_FNMADD:
    return true;
    }
    break;
    case 31:
    switch (inst_get_field(inst, 21, 30)) {
    case OP_31_LFSX:
    case OP_31_LFSUX:
    case OP_31_LFDX:
    case OP_31_LFDUX:
    case OP_31_STFSX:
    case OP_31_STFSUX:
    case OP_31_STFX:
    case OP_31_STFUX:
    case OP_31_STFIWX:
    return true;
    }
    break;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn get_d_signext(inst: u32) -> c_int {
    static int get_d_signext(u32 inst)
    {
    let mut d: c_int = inst & 0x8ff;
    if (d & 0x800)
    return -(d & 0x7ff);
    return (d & 0x7ff);
    }
    static int kvmppc_ps_three_in(struct kvm_vcpu *vcpu, bool rc,
    int reg_out, int reg_in1, int reg_in2,
    int reg_in3, int scalar,
    void (*func)(u64 *fpscr,
    u32 *dst, u32 *src1,
    u32 *src2, u32 *src3))
    {
    u32 *qpr = vcpu.arch.qpr;
    u32 ps0_out;
    u32 ps0_in1, ps0_in2, ps0_in3;
    u32 ps1_in1, ps1_in2, ps1_in3;
// RC
    WARN_ON(rc);
// PS0
    kvm_cvt_df(&VCPU_FPR(vcpu, reg_in1), &ps0_in1);
    kvm_cvt_df(&VCPU_FPR(vcpu, reg_in2), &ps0_in2);
    kvm_cvt_df(&VCPU_FPR(vcpu, reg_in3), &ps0_in3);
    if (scalar & SCALAR_LOW)
    ps0_in2 = qpr[reg_in2];
    func(&vcpu.arch.fp.fpscr, &ps0_out, &ps0_in1, &ps0_in2, &ps0_in3);
    dprintk(KERN_INFO "PS3 ps0 . f(0x%x, 0x%x, 0x%x) = 0x%x\n",
    ps0_in1, ps0_in2, ps0_in3, ps0_out);
    if (!(scalar & SCALAR_NO_PS0))
    kvm_cvt_fd(&ps0_out, &VCPU_FPR(vcpu, reg_out));
// PS1
    ps1_in1 = qpr[reg_in1];
    ps1_in2 = qpr[reg_in2];
    ps1_in3 = qpr[reg_in3];
    if (scalar & SCALAR_HIGH)
    ps1_in2 = ps0_in2;
    if (!(scalar & SCALAR_NO_PS1))
    func(&vcpu.arch.fp.fpscr, &qpr[reg_out], &ps1_in1, &ps1_in2, &ps1_in3);
    dprintk(KERN_INFO "PS3 ps1 . f(0x%x, 0x%x, 0x%x) = 0x%x\n",
    ps1_in1, ps1_in2, ps1_in3, qpr[reg_out]);
    return EMULATE_DONE;
    }
    static int kvmppc_ps_two_in(struct kvm_vcpu *vcpu, bool rc,
    int reg_out, int reg_in1, int reg_in2,
    int scalar,
    void (*func)(u64 *fpscr,
    u32 *dst, u32 *src1,
    u32 *src2))
    {
    u32 *qpr = vcpu.arch.qpr;
    u32 ps0_out;
    u32 ps0_in1, ps0_in2;
    u32 ps1_out;
    u32 ps1_in1, ps1_in2;
// RC
    WARN_ON(rc);
// PS0
    kvm_cvt_df(&VCPU_FPR(vcpu, reg_in1), &ps0_in1);
    if (scalar & SCALAR_LOW)
    ps0_in2 = qpr[reg_in2];
    else
    kvm_cvt_df(&VCPU_FPR(vcpu, reg_in2), &ps0_in2);
    func(&vcpu.arch.fp.fpscr, &ps0_out, &ps0_in1, &ps0_in2);
    if (!(scalar & SCALAR_NO_PS0)) {
    dprintk(KERN_INFO "PS2 ps0 . f(0x%x, 0x%x) = 0x%x\n",
    ps0_in1, ps0_in2, ps0_out);
    kvm_cvt_fd(&ps0_out, &VCPU_FPR(vcpu, reg_out));
    }
// PS1
    ps1_in1 = qpr[reg_in1];
    ps1_in2 = qpr[reg_in2];
    if (scalar & SCALAR_HIGH)
    ps1_in2 = ps0_in2;
    func(&vcpu.arch.fp.fpscr, &ps1_out, &ps1_in1, &ps1_in2);
    if (!(scalar & SCALAR_NO_PS1)) {
    qpr[reg_out] = ps1_out;
    dprintk(KERN_INFO "PS2 ps1 . f(0x%x, 0x%x) = 0x%x\n",
    ps1_in1, ps1_in2, qpr[reg_out]);
    }
    return EMULATE_DONE;
    }
    static int kvmppc_ps_one_in(struct kvm_vcpu *vcpu, bool rc,
    int reg_out, int reg_in,
    void (*func)(u64 *t,
    u32 *dst, u32 *src1))
    {
    u32 *qpr = vcpu.arch.qpr;
    u32 ps0_out, ps0_in;
    u32 ps1_in;
// RC
    WARN_ON(rc);
// PS0
    kvm_cvt_df(&VCPU_FPR(vcpu, reg_in), &ps0_in);
    func(&vcpu.arch.fp.fpscr, &ps0_out, &ps0_in);
    dprintk(KERN_INFO "PS1 ps0 . f(0x%x) = 0x%x\n",
    ps0_in, ps0_out);
    kvm_cvt_fd(&ps0_out, &VCPU_FPR(vcpu, reg_out));
// PS1
    ps1_in = qpr[reg_in];
    func(&vcpu.arch.fp.fpscr, &qpr[reg_out], &ps1_in);
    dprintk(KERN_INFO "PS1 ps1 . f(0x%x) = 0x%x\n",
    ps1_in, qpr[reg_out]);
    return EMULATE_DONE;
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_emulate_paired_single(vcpu: *mut kvm_vcpu) -> c_int {
    int kvmppc_emulate_paired_single(struct kvm_vcpu *vcpu)
    {
    u32 inst;
    ppc_inst_t pinst;
    let mut emulated: enum emulation_result = EMULATE_DONE;
    int ax_rd, ax_ra, ax_rb, ax_rc;
    short full_d;
    u64 *fpr_d, *fpr_a, *fpr_b, *fpr_c;
    bool rcomp;
    u32 cr;

    int i;

    emulated = kvmppc_get_last_inst(vcpu, INST_GENERIC, &pinst);
    inst = ppc_inst_val(pinst);
    if (emulated != EMULATE_DONE)
    return emulated;
    ax_rd = inst_get_field(inst, 6, 10);
    ax_ra = inst_get_field(inst, 11, 15);
    ax_rb = inst_get_field(inst, 16, 20);
    ax_rc = inst_get_field(inst, 21, 25);
    full_d = inst_get_field(inst, 16, 31);
    fpr_d = &VCPU_FPR(vcpu, ax_rd);
    fpr_a = &VCPU_FPR(vcpu, ax_ra);
    fpr_b = &VCPU_FPR(vcpu, ax_rb);
    fpr_c = &VCPU_FPR(vcpu, ax_rc);
    rcomp = (inst & 1) ? true : false;
    cr = kvmppc_get_cr(vcpu);
    if (!kvmppc_inst_is_paired_single(vcpu, inst))
    return EMULATE_FAIL;
    if (!(kvmppc_get_msr(vcpu) & MSR_FP)) {
    kvmppc_book3s_queue_irqprio(vcpu, BOOK3S_INTERRUPT_FP_UNAVAIL);
    return EMULATE_AGAIN;
    }
    kvmppc_giveup_ext(vcpu, MSR_FP);
    preempt_disable();
    enable_kernel_fp();
// Do we need to clear FE0 / FE1 here? Don't think so.

    for (i = 0; i < ARRAY_SIZE(vcpu.arch.fp.fpr); i++) {
    u32 f;
    kvm_cvt_df(&VCPU_FPR(vcpu, i), &f);
    dprintk(KERN_INFO "FPR[%d] = 0x%x / 0x%llx    QPR[%d] = 0x%x\n",
    i, f, VCPU_FPR(vcpu, i), i, vcpu.arch.qpr[i]);
    }

    switch (get_op(inst)) {
    case OP_PSQ_L:
    {
    let mut addr: c_ulong = ax_ra ? kvmppc_get_gpr(vcpu, ax_ra) : 0;
    let mut w: bool = inst_get_field(inst, 16, 16) ? true : false;
    let mut i: c_int = inst_get_field(inst, 17, 19);
    addr += get_d_signext(inst);
    emulated = kvmppc_emulate_psq_load(vcpu, ax_rd, addr, w, i);
    break;
    }
    case OP_PSQ_LU:
    {
    let mut addr: c_ulong = kvmppc_get_gpr(vcpu, ax_ra);
    let mut w: bool = inst_get_field(inst, 16, 16) ? true : false;
    let mut i: c_int = inst_get_field(inst, 17, 19);
    addr += get_d_signext(inst);
    emulated = kvmppc_emulate_psq_load(vcpu, ax_rd, addr, w, i);
    if (emulated == EMULATE_DONE)
    kvmppc_set_gpr(vcpu, ax_ra, addr);
    break;
    }
    case OP_PSQ_ST:
    {
    let mut addr: c_ulong = ax_ra ? kvmppc_get_gpr(vcpu, ax_ra) : 0;
    let mut w: bool = inst_get_field(inst, 16, 16) ? true : false;
    let mut i: c_int = inst_get_field(inst, 17, 19);
    addr += get_d_signext(inst);
    emulated = kvmppc_emulate_psq_store(vcpu, ax_rd, addr, w, i);
    break;
    }
    case OP_PSQ_STU:
    {
    let mut addr: c_ulong = kvmppc_get_gpr(vcpu, ax_ra);
    let mut w: bool = inst_get_field(inst, 16, 16) ? true : false;
    let mut i: c_int = inst_get_field(inst, 17, 19);
    addr += get_d_signext(inst);
    emulated = kvmppc_emulate_psq_store(vcpu, ax_rd, addr, w, i);
    if (emulated == EMULATE_DONE)
    kvmppc_set_gpr(vcpu, ax_ra, addr);
    break;
    }
    case 4:
// X form
    switch (inst_get_field(inst, 21, 30)) {
    case OP_4X_PS_CMPU0:
// XXX
    emulated = EMULATE_FAIL;
    break;
    case OP_4X_PSQ_LX:
    {
    let mut addr: c_ulong = ax_ra ? kvmppc_get_gpr(vcpu, ax_ra) : 0;
    let mut w: bool = inst_get_field(inst, 21, 21) ? true : false;
    let mut i: c_int = inst_get_field(inst, 22, 24);
    addr += kvmppc_get_gpr(vcpu, ax_rb);
    emulated = kvmppc_emulate_psq_load(vcpu, ax_rd, addr, w, i);
    break;
    }
    case OP_4X_PS_CMPO0:
// XXX
    emulated = EMULATE_FAIL;
    break;
    case OP_4X_PSQ_LUX:
    {
    let mut addr: c_ulong = kvmppc_get_gpr(vcpu, ax_ra);
    let mut w: bool = inst_get_field(inst, 21, 21) ? true : false;
    let mut i: c_int = inst_get_field(inst, 22, 24);
    addr += kvmppc_get_gpr(vcpu, ax_rb);
    emulated = kvmppc_emulate_psq_load(vcpu, ax_rd, addr, w, i);
    if (emulated == EMULATE_DONE)
    kvmppc_set_gpr(vcpu, ax_ra, addr);
    break;
    }
    case OP_4X_PS_NEG:
    VCPU_FPR(vcpu, ax_rd) = VCPU_FPR(vcpu, ax_rb);
    VCPU_FPR(vcpu, ax_rd) ^= 0x8000000000000000ULL;
    vcpu.arch.qpr[ax_rd] = vcpu.arch.qpr[ax_rb];
    vcpu.arch.qpr[ax_rd] ^= 0x80000000;
    break;
    case OP_4X_PS_CMPU1:
// XXX
    emulated = EMULATE_FAIL;
    break;
    case OP_4X_PS_MR:
    WARN_ON(rcomp);
    VCPU_FPR(vcpu, ax_rd) = VCPU_FPR(vcpu, ax_rb);
    vcpu.arch.qpr[ax_rd] = vcpu.arch.qpr[ax_rb];
    break;
    case OP_4X_PS_CMPO1:
// XXX
    emulated = EMULATE_FAIL;
    break;
    case OP_4X_PS_NABS:
    WARN_ON(rcomp);
    VCPU_FPR(vcpu, ax_rd) = VCPU_FPR(vcpu, ax_rb);
    VCPU_FPR(vcpu, ax_rd) |= 0x8000000000000000ULL;
    vcpu.arch.qpr[ax_rd] = vcpu.arch.qpr[ax_rb];
    vcpu.arch.qpr[ax_rd] |= 0x80000000;
    break;
    case OP_4X_PS_ABS:
    WARN_ON(rcomp);
    VCPU_FPR(vcpu, ax_rd) = VCPU_FPR(vcpu, ax_rb);
    VCPU_FPR(vcpu, ax_rd) &= ~0x8000000000000000ULL;
    vcpu.arch.qpr[ax_rd] = vcpu.arch.qpr[ax_rb];
    vcpu.arch.qpr[ax_rd] &= ~0x80000000;
    break;
    case OP_4X_PS_MERGE00:
    WARN_ON(rcomp);
    VCPU_FPR(vcpu, ax_rd) = VCPU_FPR(vcpu, ax_ra);
// vcpu->arch.qpr[ax_rd] = VCPU_FPR(vcpu, ax_rb);
    kvm_cvt_df(&VCPU_FPR(vcpu, ax_rb),
    &vcpu.arch.qpr[ax_rd]);
    break;
    case OP_4X_PS_MERGE01:
    WARN_ON(rcomp);
    VCPU_FPR(vcpu, ax_rd) = VCPU_FPR(vcpu, ax_ra);
    vcpu.arch.qpr[ax_rd] = vcpu.arch.qpr[ax_rb];
    break;
    case OP_4X_PS_MERGE10:
    WARN_ON(rcomp);
// VCPU_FPR(vcpu, ax_rd) = vcpu->arch.qpr[ax_ra];
    kvm_cvt_fd(&vcpu.arch.qpr[ax_ra],
    &VCPU_FPR(vcpu, ax_rd));
// vcpu->arch.qpr[ax_rd] = VCPU_FPR(vcpu, ax_rb);
    kvm_cvt_df(&VCPU_FPR(vcpu, ax_rb),
    &vcpu.arch.qpr[ax_rd]);
    break;
    case OP_4X_PS_MERGE11:
    WARN_ON(rcomp);
// VCPU_FPR(vcpu, ax_rd) = vcpu->arch.qpr[ax_ra];
    kvm_cvt_fd(&vcpu.arch.qpr[ax_ra],
    &VCPU_FPR(vcpu, ax_rd));
    vcpu.arch.qpr[ax_rd] = vcpu.arch.qpr[ax_rb];
    break;
    }
// XW form
    switch (inst_get_field(inst, 25, 30)) {
    case OP_4XW_PSQ_STX:
    {
    let mut addr: c_ulong = ax_ra ? kvmppc_get_gpr(vcpu, ax_ra) : 0;
    let mut w: bool = inst_get_field(inst, 21, 21) ? true : false;
    let mut i: c_int = inst_get_field(inst, 22, 24);
    addr += kvmppc_get_gpr(vcpu, ax_rb);
    emulated = kvmppc_emulate_psq_store(vcpu, ax_rd, addr, w, i);
    break;
    }
    case OP_4XW_PSQ_STUX:
    {
    let mut addr: c_ulong = kvmppc_get_gpr(vcpu, ax_ra);
    let mut w: bool = inst_get_field(inst, 21, 21) ? true : false;
    let mut i: c_int = inst_get_field(inst, 22, 24);
    addr += kvmppc_get_gpr(vcpu, ax_rb);
    emulated = kvmppc_emulate_psq_store(vcpu, ax_rd, addr, w, i);
    if (emulated == EMULATE_DONE)
    kvmppc_set_gpr(vcpu, ax_ra, addr);
    break;
    }
    }
// A form
    switch (inst_get_field(inst, 26, 30)) {
    case OP_4A_PS_SUM1:
    emulated = kvmppc_ps_two_in(vcpu, rcomp, ax_rd,
    ax_rb, ax_ra, SCALAR_NO_PS0 | SCALAR_HIGH, fps_fadds);
    VCPU_FPR(vcpu, ax_rd) = VCPU_FPR(vcpu, ax_rc);
    break;
    case OP_4A_PS_SUM0:
    emulated = kvmppc_ps_two_in(vcpu, rcomp, ax_rd,
    ax_ra, ax_rb, SCALAR_NO_PS1 | SCALAR_LOW, fps_fadds);
    vcpu.arch.qpr[ax_rd] = vcpu.arch.qpr[ax_rc];
    break;
    case OP_4A_PS_MULS0:
    emulated = kvmppc_ps_two_in(vcpu, rcomp, ax_rd,
    ax_ra, ax_rc, SCALAR_HIGH, fps_fmuls);
    break;
    case OP_4A_PS_MULS1:
    emulated = kvmppc_ps_two_in(vcpu, rcomp, ax_rd,
    ax_ra, ax_rc, SCALAR_LOW, fps_fmuls);
    break;
    case OP_4A_PS_MADDS0:
    emulated = kvmppc_ps_three_in(vcpu, rcomp, ax_rd,
    ax_ra, ax_rc, ax_rb, SCALAR_HIGH, fps_fmadds);
    break;
    case OP_4A_PS_MADDS1:
    emulated = kvmppc_ps_three_in(vcpu, rcomp, ax_rd,
    ax_ra, ax_rc, ax_rb, SCALAR_LOW, fps_fmadds);
    break;
    case OP_4A_PS_DIV:
    emulated = kvmppc_ps_two_in(vcpu, rcomp, ax_rd,
    ax_ra, ax_rb, SCALAR_NONE, fps_fdivs);
    break;
    case OP_4A_PS_SUB:
    emulated = kvmppc_ps_two_in(vcpu, rcomp, ax_rd,
    ax_ra, ax_rb, SCALAR_NONE, fps_fsubs);
    break;
    case OP_4A_PS_ADD:
    emulated = kvmppc_ps_two_in(vcpu, rcomp, ax_rd,
    ax_ra, ax_rb, SCALAR_NONE, fps_fadds);
    break;
    case OP_4A_PS_SEL:
    emulated = kvmppc_ps_three_in(vcpu, rcomp, ax_rd,
    ax_ra, ax_rc, ax_rb, SCALAR_NONE, fps_fsel);
    break;
    case OP_4A_PS_RES:
    emulated = kvmppc_ps_one_in(vcpu, rcomp, ax_rd,
    ax_rb, fps_fres);
    break;
    case OP_4A_PS_MUL:
    emulated = kvmppc_ps_two_in(vcpu, rcomp, ax_rd,
    ax_ra, ax_rc, SCALAR_NONE, fps_fmuls);
    break;
    case OP_4A_PS_RSQRTE:
    emulated = kvmppc_ps_one_in(vcpu, rcomp, ax_rd,
    ax_rb, fps_frsqrte);
    break;
    case OP_4A_PS_MSUB:
    emulated = kvmppc_ps_three_in(vcpu, rcomp, ax_rd,
    ax_ra, ax_rc, ax_rb, SCALAR_NONE, fps_fmsubs);
    break;
    case OP_4A_PS_MADD:
    emulated = kvmppc_ps_three_in(vcpu, rcomp, ax_rd,
    ax_ra, ax_rc, ax_rb, SCALAR_NONE, fps_fmadds);
    break;
    case OP_4A_PS_NMSUB:
    emulated = kvmppc_ps_three_in(vcpu, rcomp, ax_rd,
    ax_ra, ax_rc, ax_rb, SCALAR_NONE, fps_fnmsubs);
    break;
    case OP_4A_PS_NMADD:
    emulated = kvmppc_ps_three_in(vcpu, rcomp, ax_rd,
    ax_ra, ax_rc, ax_rb, SCALAR_NONE, fps_fnmadds);
    break;
    }
    break;
// Real FPU operations
    case OP_LFS:
    {
    let mut addr: c_ulong = (ax_ra ? kvmppc_get_gpr(vcpu, ax_ra) : 0) + full_d;
    emulated = kvmppc_emulate_fpr_load(vcpu, ax_rd, addr,
    FPU_LS_SINGLE);
    break;
    }
    case OP_LFSU:
    {
    let mut addr: c_ulong = kvmppc_get_gpr(vcpu, ax_ra) + full_d;
    emulated = kvmppc_emulate_fpr_load(vcpu, ax_rd, addr,
    FPU_LS_SINGLE);
    if (emulated == EMULATE_DONE)
    kvmppc_set_gpr(vcpu, ax_ra, addr);
    break;
    }
    case OP_LFD:
    {
    let mut addr: c_ulong = (ax_ra ? kvmppc_get_gpr(vcpu, ax_ra) : 0) + full_d;
    emulated = kvmppc_emulate_fpr_load(vcpu, ax_rd, addr,
    FPU_LS_DOUBLE);
    break;
    }
    case OP_LFDU:
    {
    let mut addr: c_ulong = kvmppc_get_gpr(vcpu, ax_ra) + full_d;
    emulated = kvmppc_emulate_fpr_load(vcpu, ax_rd, addr,
    FPU_LS_DOUBLE);
    if (emulated == EMULATE_DONE)
    kvmppc_set_gpr(vcpu, ax_ra, addr);
    break;
    }
    case OP_STFS:
    {
    let mut addr: c_ulong = (ax_ra ? kvmppc_get_gpr(vcpu, ax_ra) : 0) + full_d;
    emulated = kvmppc_emulate_fpr_store(vcpu, ax_rd, addr,
    FPU_LS_SINGLE);
    break;
    }
    case OP_STFSU:
    {
    let mut addr: c_ulong = kvmppc_get_gpr(vcpu, ax_ra) + full_d;
    emulated = kvmppc_emulate_fpr_store(vcpu, ax_rd, addr,
    FPU_LS_SINGLE);
    if (emulated == EMULATE_DONE)
    kvmppc_set_gpr(vcpu, ax_ra, addr);
    break;
    }
    case OP_STFD:
    {
    let mut addr: c_ulong = (ax_ra ? kvmppc_get_gpr(vcpu, ax_ra) : 0) + full_d;
    emulated = kvmppc_emulate_fpr_store(vcpu, ax_rd, addr,
    FPU_LS_DOUBLE);
    break;
    }
    case OP_STFDU:
    {
    let mut addr: c_ulong = kvmppc_get_gpr(vcpu, ax_ra) + full_d;
    emulated = kvmppc_emulate_fpr_store(vcpu, ax_rd, addr,
    FPU_LS_DOUBLE);
    if (emulated == EMULATE_DONE)
    kvmppc_set_gpr(vcpu, ax_ra, addr);
    break;
    }
    case 31:
    switch (inst_get_field(inst, 21, 30)) {
    case OP_31_LFSX:
    {
    let mut addr: c_ulong = ax_ra ? kvmppc_get_gpr(vcpu, ax_ra) : 0;
    addr += kvmppc_get_gpr(vcpu, ax_rb);
    emulated = kvmppc_emulate_fpr_load(vcpu, ax_rd,
    addr, FPU_LS_SINGLE);
    break;
    }
    case OP_31_LFSUX:
    {
    ulong addr = kvmppc_get_gpr(vcpu, ax_ra) +
    kvmppc_get_gpr(vcpu, ax_rb);
    emulated = kvmppc_emulate_fpr_load(vcpu, ax_rd,
    addr, FPU_LS_SINGLE);
    if (emulated == EMULATE_DONE)
    kvmppc_set_gpr(vcpu, ax_ra, addr);
    break;
    }
    case OP_31_LFDX:
    {
    ulong addr = (ax_ra ? kvmppc_get_gpr(vcpu, ax_ra) : 0) +
    kvmppc_get_gpr(vcpu, ax_rb);
    emulated = kvmppc_emulate_fpr_load(vcpu, ax_rd,
    addr, FPU_LS_DOUBLE);
    break;
    }
    case OP_31_LFDUX:
    {
    ulong addr = kvmppc_get_gpr(vcpu, ax_ra) +
    kvmppc_get_gpr(vcpu, ax_rb);
    emulated = kvmppc_emulate_fpr_load(vcpu, ax_rd,
    addr, FPU_LS_DOUBLE);
    if (emulated == EMULATE_DONE)
    kvmppc_set_gpr(vcpu, ax_ra, addr);
    break;
    }
    case OP_31_STFSX:
    {
    ulong addr = (ax_ra ? kvmppc_get_gpr(vcpu, ax_ra) : 0) +
    kvmppc_get_gpr(vcpu, ax_rb);
    emulated = kvmppc_emulate_fpr_store(vcpu, ax_rd,
    addr, FPU_LS_SINGLE);
    break;
    }
    case OP_31_STFSUX:
    {
    ulong addr = kvmppc_get_gpr(vcpu, ax_ra) +
    kvmppc_get_gpr(vcpu, ax_rb);
    emulated = kvmppc_emulate_fpr_store(vcpu, ax_rd,
    addr, FPU_LS_SINGLE);
    if (emulated == EMULATE_DONE)
    kvmppc_set_gpr(vcpu, ax_ra, addr);
    break;
    }
    case OP_31_STFX:
    {
    ulong addr = (ax_ra ? kvmppc_get_gpr(vcpu, ax_ra) : 0) +
    kvmppc_get_gpr(vcpu, ax_rb);
    emulated = kvmppc_emulate_fpr_store(vcpu, ax_rd,
    addr, FPU_LS_DOUBLE);
    break;
    }
    case OP_31_STFUX:
    {
    ulong addr = kvmppc_get_gpr(vcpu, ax_ra) +
    kvmppc_get_gpr(vcpu, ax_rb);
    emulated = kvmppc_emulate_fpr_store(vcpu, ax_rd,
    addr, FPU_LS_DOUBLE);
    if (emulated == EMULATE_DONE)
    kvmppc_set_gpr(vcpu, ax_ra, addr);
    break;
    }
    case OP_31_STFIWX:
    {
    ulong addr = (ax_ra ? kvmppc_get_gpr(vcpu, ax_ra) : 0) +
    kvmppc_get_gpr(vcpu, ax_rb);
    emulated = kvmppc_emulate_fpr_store(vcpu, ax_rd,
    addr,
    FPU_LS_SINGLE_LOW);
    break;
    }
    break;
    }
    break;
    case 59:
    switch (inst_get_field(inst, 21, 30)) {
    case OP_59_FADDS:
    fpd_fadds(&vcpu.arch.fp.fpscr, &cr, fpr_d, fpr_a, fpr_b);
    kvmppc_sync_qpr(vcpu, ax_rd);
    break;
    case OP_59_FSUBS:
    fpd_fsubs(&vcpu.arch.fp.fpscr, &cr, fpr_d, fpr_a, fpr_b);
    kvmppc_sync_qpr(vcpu, ax_rd);
    break;
    case OP_59_FDIVS:
    fpd_fdivs(&vcpu.arch.fp.fpscr, &cr, fpr_d, fpr_a, fpr_b);
    kvmppc_sync_qpr(vcpu, ax_rd);
    break;
    case OP_59_FRES:
    fpd_fres(&vcpu.arch.fp.fpscr, &cr, fpr_d, fpr_b);
    kvmppc_sync_qpr(vcpu, ax_rd);
    break;
    case OP_59_FRSQRTES:
    fpd_frsqrtes(&vcpu.arch.fp.fpscr, &cr, fpr_d, fpr_b);
    kvmppc_sync_qpr(vcpu, ax_rd);
    break;
    }
    switch (inst_get_field(inst, 26, 30)) {
    case OP_59_FMULS:
    fpd_fmuls(&vcpu.arch.fp.fpscr, &cr, fpr_d, fpr_a, fpr_c);
    kvmppc_sync_qpr(vcpu, ax_rd);
    break;
    case OP_59_FMSUBS:
    fpd_fmsubs(&vcpu.arch.fp.fpscr, &cr, fpr_d, fpr_a, fpr_c, fpr_b);
    kvmppc_sync_qpr(vcpu, ax_rd);
    break;
    case OP_59_FMADDS:
    fpd_fmadds(&vcpu.arch.fp.fpscr, &cr, fpr_d, fpr_a, fpr_c, fpr_b);
    kvmppc_sync_qpr(vcpu, ax_rd);
    break;
    case OP_59_FNMSUBS:
    fpd_fnmsubs(&vcpu.arch.fp.fpscr, &cr, fpr_d, fpr_a, fpr_c, fpr_b);
    kvmppc_sync_qpr(vcpu, ax_rd);
    break;
    case OP_59_FNMADDS:
    fpd_fnmadds(&vcpu.arch.fp.fpscr, &cr, fpr_d, fpr_a, fpr_c, fpr_b);
    kvmppc_sync_qpr(vcpu, ax_rd);
    break;
    }
    break;
    case 63:
    switch (inst_get_field(inst, 21, 30)) {
    case OP_63_MTFSB0:
    case OP_63_MTFSB1:
    case OP_63_MCRFS:
    case OP_63_MTFSFI:
// XXX need to implement
    break;
    case OP_63_MFFS:
// XXX missing CR
// fpr_d = vcpu->arch.fp.fpscr;
    break;
    case OP_63_MTFSF:
// XXX missing fm bits
// XXX missing CR
    vcpu.arch.fp.fpscr = *fpr_b;
    break;
    case OP_63_FCMPU:
    {
    u32 tmp_cr;
    let mut cr0_mask: u32 = 0xf0000000;
    let mut cr_shift: u32 = inst_get_field(inst, 6, 8) * 4;
    fpd_fcmpu(&vcpu.arch.fp.fpscr, &tmp_cr, fpr_a, fpr_b);
    cr &= ~(cr0_mask >> cr_shift);
    cr |= (cr & cr0_mask) >> cr_shift;
    break;
    }
    case OP_63_FCMPO:
    {
    u32 tmp_cr;
    let mut cr0_mask: u32 = 0xf0000000;
    let mut cr_shift: u32 = inst_get_field(inst, 6, 8) * 4;
    fpd_fcmpo(&vcpu.arch.fp.fpscr, &tmp_cr, fpr_a, fpr_b);
    cr &= ~(cr0_mask >> cr_shift);
    cr |= (cr & cr0_mask) >> cr_shift;
    break;
    }
    case OP_63_FNEG:
    fpd_fneg(&vcpu.arch.fp.fpscr, &cr, fpr_d, fpr_b);
    break;
    case OP_63_FMR:
// fpr_d = *fpr_b;
    break;
    case OP_63_FABS:
    fpd_fabs(&vcpu.arch.fp.fpscr, &cr, fpr_d, fpr_b);
    break;
    case OP_63_FCPSGN:
    fpd_fcpsgn(&vcpu.arch.fp.fpscr, &cr, fpr_d, fpr_a, fpr_b);
    break;
    case OP_63_FDIV:
    fpd_fdiv(&vcpu.arch.fp.fpscr, &cr, fpr_d, fpr_a, fpr_b);
    break;
    case OP_63_FADD:
    fpd_fadd(&vcpu.arch.fp.fpscr, &cr, fpr_d, fpr_a, fpr_b);
    break;
    case OP_63_FSUB:
    fpd_fsub(&vcpu.arch.fp.fpscr, &cr, fpr_d, fpr_a, fpr_b);
    break;
    case OP_63_FCTIW:
    fpd_fctiw(&vcpu.arch.fp.fpscr, &cr, fpr_d, fpr_b);
    break;
    case OP_63_FCTIWZ:
    fpd_fctiwz(&vcpu.arch.fp.fpscr, &cr, fpr_d, fpr_b);
    break;
    case OP_63_FRSP:
    fpd_frsp(&vcpu.arch.fp.fpscr, &cr, fpr_d, fpr_b);
    kvmppc_sync_qpr(vcpu, ax_rd);
    break;
    case OP_63_FRSQRTE:
    {
    let mut one: double = 1.0f;
// fD = sqrt(fB)
    fpd_fsqrt(&vcpu.arch.fp.fpscr, &cr, fpr_d, fpr_b);
// fD = 1.0f / fD
    fpd_fdiv(&vcpu.arch.fp.fpscr, &cr, fpr_d, (u64*)&one, fpr_d);
    break;
    }
    }
    switch (inst_get_field(inst, 26, 30)) {
    case OP_63_FMUL:
    fpd_fmul(&vcpu.arch.fp.fpscr, &cr, fpr_d, fpr_a, fpr_c);
    break;
    case OP_63_FSEL:
    fpd_fsel(&vcpu.arch.fp.fpscr, &cr, fpr_d, fpr_a, fpr_c, fpr_b);
    break;
    case OP_63_FMSUB:
    fpd_fmsub(&vcpu.arch.fp.fpscr, &cr, fpr_d, fpr_a, fpr_c, fpr_b);
    break;
    case OP_63_FMADD:
    fpd_fmadd(&vcpu.arch.fp.fpscr, &cr, fpr_d, fpr_a, fpr_c, fpr_b);
    break;
    case OP_63_FNMSUB:
    fpd_fnmsub(&vcpu.arch.fp.fpscr, &cr, fpr_d, fpr_a, fpr_c, fpr_b);
    break;
    case OP_63_FNMADD:
    fpd_fnmadd(&vcpu.arch.fp.fpscr, &cr, fpr_d, fpr_a, fpr_c, fpr_b);
    break;
    }
    break;
    }

    for (i = 0; i < ARRAY_SIZE(vcpu.arch.fp.fpr); i++) {
    u32 f;
    kvm_cvt_df(&VCPU_FPR(vcpu, i), &f);
    dprintk(KERN_INFO "FPR[%d] = 0x%x\n", i, f);
    }

    if (rcomp)
    kvmppc_set_cr(vcpu, cr);
    disable_kernel_fp();
    preempt_enable();
    return emulated;
    }
