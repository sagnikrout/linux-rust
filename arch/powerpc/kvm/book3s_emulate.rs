//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kvm/book3s_emulate.c
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
// Copyright SUSE Linux Products GmbH 2009
//
// Authors: Alexander Graf <agraf@suse.de>
//

pub const OP_19_XOP_RFID: c_int = 18;
pub const OP_19_XOP_RFI: c_int = 50;
pub const OP_31_XOP_MFMSR: c_int = 83;
pub const OP_31_XOP_MTMSR: c_int = 146;
pub const OP_31_XOP_MTMSRD: c_int = 178;
pub const OP_31_XOP_MTSR: c_int = 210;
pub const OP_31_XOP_MTSRIN: c_int = 242;
pub const OP_31_XOP_TLBIEL: c_int = 274;
// Opcode is officially reserved, reuse it as sc 1 when sc 1 doesn't trap
pub const OP_31_XOP_FAKE_SC1: c_int = 308;
pub const OP_31_XOP_SLBMTE: c_int = 402;
pub const OP_31_XOP_SLBIE: c_int = 434;
pub const OP_31_XOP_SLBIA: c_int = 498;
pub const OP_31_XOP_MFSR: c_int = 595;
pub const OP_31_XOP_MFSRIN: c_int = 659;
pub const OP_31_XOP_DCBA: c_int = 758;
pub const OP_31_XOP_SLBMFEV: c_int = 851;
pub const OP_31_XOP_EIOIO: c_int = 854;
pub const OP_31_XOP_SLBMFEE: c_int = 915;
pub const OP_31_XOP_SLBFEE: c_int = 979;
pub const OP_31_XOP_TBEGIN: c_int = 654;
pub const OP_31_XOP_TABORT: c_int = 910;
pub const OP_31_XOP_TRECLAIM: c_int = 942;
pub const OP_31_XOP_TRCHKPT: c_int = 1006;
// DCBZ is actually 1014, but we patch it to 1010 so we get a trap
pub const OP_31_XOP_DCBZ: c_int = 1010;
pub const OP_LFS: c_int = 48;
pub const OP_LFD: c_int = 50;
pub const OP_STFS: c_int = 52;
pub const OP_STFD: c_int = 54;
pub const SPRN_GQR0: c_int = 912;
pub const SPRN_GQR1: c_int = 913;
pub const SPRN_GQR2: c_int = 914;
pub const SPRN_GQR3: c_int = 915;
pub const SPRN_GQR4: c_int = 916;
pub const SPRN_GQR5: c_int = 917;
pub const SPRN_GQR6: c_int = 918;
pub const SPRN_GQR7: c_int = 919;
    enum priv_level {
    PRIV_PROBLEM = 0,
    PRIV_SUPER = 1,
    PRIV_HYPER = 2,
    };
#[no_mangle]
unsafe extern "C" fn spr_allowed(vcpu: *mut kvm_vcpu, level: enum priv_level) -> bool {
    static bool spr_allowed(struct kvm_vcpu *vcpu, enum priv_level level)
    {
// PAPR VMs only access supervisor SPRs
    if (vcpu.arch.papr_enabled && (level > PRIV_SUPER))
    return false;
// Limit user space to its own small SPR set
    if ((kvmppc_get_msr(vcpu) & MSR_PR) && level > PRIV_PROBLEM)
    return false;
    return true;
    }

#[no_mangle]
pub unsafe extern "C" fn kvmppc_copyto_vcpu_tm(vcpu: *mut kvm_vcpu) {
    static inline void kvmppc_copyto_vcpu_tm(struct kvm_vcpu *vcpu)
    {
    memcpy(&vcpu.arch.gpr_tm[0], &vcpu.arch.regs.gpr[0],
    sizeof(vcpu.arch.gpr_tm));
    memcpy(&vcpu.arch.fp_tm, &vcpu.arch.fp,
    sizeof(struct thread_fp_state));
    memcpy(&vcpu.arch.vr_tm, &vcpu.arch.vr,
    sizeof(struct thread_vr_state));
    vcpu.arch.ppr_tm = vcpu.arch.ppr;
    vcpu.arch.dscr_tm = vcpu.arch.dscr;
    vcpu.arch.amr_tm = vcpu.arch.amr;
    vcpu.arch.ctr_tm = vcpu.arch.regs.ctr;
    vcpu.arch.tar_tm = vcpu.arch.tar;
    vcpu.arch.lr_tm = vcpu.arch.regs.link;
    vcpu.arch.cr_tm = vcpu.arch.regs.ccr;
    vcpu.arch.xer_tm = vcpu.arch.regs.xer;
    vcpu.arch.vrsave_tm = vcpu.arch.vrsave;
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_copyfrom_vcpu_tm(vcpu: *mut kvm_vcpu) {
    static inline void kvmppc_copyfrom_vcpu_tm(struct kvm_vcpu *vcpu)
    {
    memcpy(&vcpu.arch.regs.gpr[0], &vcpu.arch.gpr_tm[0],
    sizeof(vcpu.arch.regs.gpr));
    memcpy(&vcpu.arch.fp, &vcpu.arch.fp_tm,
    sizeof(struct thread_fp_state));
    memcpy(&vcpu.arch.vr, &vcpu.arch.vr_tm,
    sizeof(struct thread_vr_state));
    vcpu.arch.ppr = vcpu.arch.ppr_tm;
    vcpu.arch.dscr = vcpu.arch.dscr_tm;
    vcpu.arch.amr = vcpu.arch.amr_tm;
    vcpu.arch.regs.ctr = vcpu.arch.ctr_tm;
    vcpu.arch.tar = vcpu.arch.tar_tm;
    vcpu.arch.regs.link = vcpu.arch.lr_tm;
    vcpu.arch.regs.ccr = vcpu.arch.cr_tm;
    vcpu.arch.regs.xer = vcpu.arch.xer_tm;
    vcpu.arch.vrsave = vcpu.arch.vrsave_tm;
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_emulate_treclaim(vcpu: *mut kvm_vcpu, ra_val: c_int) {
    static void kvmppc_emulate_treclaim(struct kvm_vcpu *vcpu, int ra_val)
    {
    let mut guest_msr: c_ulong = kvmppc_get_msr(vcpu);
    let mut fc_val: c_int = ra_val ? ra_val : 1;
    uint64_t texasr;
// CR0 = 0 | MSR[TS] | 0
    vcpu.arch.regs.ccr = (vcpu.arch.regs.ccr & ~(CR0_MASK << CR0_SHIFT)) |
    (((guest_msr & MSR_TS_MASK) >> (MSR_TS_S_LG - 1))
    << CR0_SHIFT);
    preempt_disable();
    tm_enable();
    texasr = mfspr(SPRN_TEXASR);
    kvmppc_save_tm_pr(vcpu);
    kvmppc_copyfrom_vcpu_tm(vcpu);
// failure recording depends on Failure Summary bit
    if (!(texasr & TEXASR_FS)) {
    texasr &= ~TEXASR_FC;
    texasr |= ((u64)fc_val << TEXASR_FC_LG) | TEXASR_FS;
    texasr &= ~(TEXASR_PR | TEXASR_HV);
    if (kvmppc_get_msr(vcpu) & MSR_PR)
    texasr |= TEXASR_PR;
    if (kvmppc_get_msr(vcpu) & MSR_HV)
    texasr |= TEXASR_HV;
    vcpu.arch.texasr = texasr;
    vcpu.arch.tfiar = kvmppc_get_pc(vcpu);
    mtspr(SPRN_TEXASR, texasr);
    mtspr(SPRN_TFIAR, vcpu.arch.tfiar);
    }
    tm_disable();
//
// treclaim need quit to non-transactional state.
//
    guest_msr &= ~(MSR_TS_MASK);
    kvmppc_set_msr(vcpu, guest_msr);
    preempt_enable();
    if (vcpu.arch.shadow_fscr & FSCR_TAR)
    mtspr(SPRN_TAR, vcpu.arch.tar);
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_emulate_trchkpt(vcpu: *mut kvm_vcpu) {
    static void kvmppc_emulate_trchkpt(struct kvm_vcpu *vcpu)
    {
    let mut guest_msr: c_ulong = kvmppc_get_msr(vcpu);
    preempt_disable();
//
// need flush FP/VEC/VSX to vcpu save area before
// copy.
//
    kvmppc_giveup_ext(vcpu, MSR_VSX);
    kvmppc_giveup_fac(vcpu, FSCR_TAR_LG);
    kvmppc_copyto_vcpu_tm(vcpu);
    kvmppc_save_tm_sprs(vcpu);
//
// as a result of trecheckpoint. set TS to suspended.
//
    guest_msr &= ~(MSR_TS_MASK);
    guest_msr |= MSR_TS_S;
    kvmppc_set_msr(vcpu, guest_msr);
    kvmppc_restore_tm_pr(vcpu);
    preempt_enable();
    }
// emulate tabort. at guest privilege state
#[no_mangle]
pub unsafe extern "C" fn kvmppc_emulate_tabort(vcpu: *mut kvm_vcpu, ra_val: c_int) {
    void kvmppc_emulate_tabort(struct kvm_vcpu *vcpu, int ra_val)
    {
// currently we only emulate tabort. but no emulation of other
// tabort variants since there is no kernel usage of them at
// present.
//
    let mut guest_msr: c_ulong = kvmppc_get_msr(vcpu);
    uint64_t org_texasr;
    preempt_disable();
    tm_enable();
    org_texasr = mfspr(SPRN_TEXASR);
    tm_abort(ra_val);
// CR0 = 0 | MSR[TS] | 0
    vcpu.arch.regs.ccr = (vcpu.arch.regs.ccr & ~(CR0_MASK << CR0_SHIFT)) |
    (((guest_msr & MSR_TS_MASK) >> (MSR_TS_S_LG - 1))
    << CR0_SHIFT);
    vcpu.arch.texasr = mfspr(SPRN_TEXASR);
// failure recording depends on Failure Summary bit,
// and tabort will be treated as nops in non-transactional
// state.
//
    if (!(org_texasr & TEXASR_FS) &&
    MSR_TM_ACTIVE(guest_msr)) {
    vcpu.arch.texasr &= ~(TEXASR_PR | TEXASR_HV);
    if (guest_msr & MSR_PR)
    vcpu.arch.texasr |= TEXASR_PR;
    if (guest_msr & MSR_HV)
    vcpu.arch.texasr |= TEXASR_HV;
    vcpu.arch.tfiar = kvmppc_get_pc(vcpu);
    }
    tm_disable();
    preempt_enable();
    }

    int kvmppc_core_emulate_op_pr(struct kvm_vcpu *vcpu,
    unsigned int inst, int *advance)
    {
    let mut emulated: c_int = EMULATE_DONE;
    let mut rt: c_int = get_rt(inst);
    let mut rs: c_int = get_rs(inst);
    let mut ra: c_int = get_ra(inst);
    let mut rb: c_int = get_rb(inst);
    let mut inst_sc: u32 = 0x44000002;
    switch (get_op(inst)) {
    case 0:
    emulated = EMULATE_FAIL;
    if ((kvmppc_get_msr(vcpu) & MSR_LE) &&
    (inst == swab32(inst_sc))) {
//
// This is the byte reversed syscall instruction of our
// hypercall handler. Early versions of LE Linux didn't
// swap the instructions correctly and ended up in
// illegal instructions.
// Just always fail hypercalls on these broken systems.
//
    kvmppc_set_gpr(vcpu, 3, EV_UNIMPLEMENTED);
    kvmppc_set_pc(vcpu, kvmppc_get_pc(vcpu) + 4);
    emulated = EMULATE_DONE;
    }
    break;
    case 19:
    switch (get_xop(inst)) {
    case OP_19_XOP_RFID:
    case OP_19_XOP_RFI: {
    let mut srr1: c_ulong = kvmppc_get_srr1(vcpu);

    let mut cur_msr: c_ulong = kvmppc_get_msr(vcpu);
//
// add rules to fit in ISA specification regarding TM
// state transition in TM disable/Suspended state,
// and target TM state is TM inactive(00) state. (the
// change should be suppressed).
//
    if (((cur_msr & MSR_TM) == 0) &&
    ((srr1 & MSR_TM) == 0) &&
    MSR_TM_SUSPENDED(cur_msr) &&
    !MSR_TM_ACTIVE(srr1))
    srr1 |= MSR_TS_S;

    kvmppc_set_pc(vcpu, kvmppc_get_srr0(vcpu));
    kvmppc_set_msr(vcpu, srr1);
// advance = 0;
    break;
    }
    default:
    emulated = EMULATE_FAIL;
    break;
    }
    break;
    case 31:
    switch (get_xop(inst)) {
    case OP_31_XOP_MFMSR:
    kvmppc_set_gpr(vcpu, rt, kvmppc_get_msr(vcpu));
    break;
    case OP_31_XOP_MTMSRD:
    {
    let mut rs_val: c_ulong = kvmppc_get_gpr(vcpu, rs);
    if (inst & 0x10000) {
    let mut new_msr: c_ulong = kvmppc_get_msr(vcpu);
    new_msr &= ~(MSR_RI | MSR_EE);
    new_msr |= rs_val & (MSR_RI | MSR_EE);
    kvmppc_set_msr_fast(vcpu, new_msr);
    } else
    kvmppc_set_msr(vcpu, rs_val);
    break;
    }
    case OP_31_XOP_MTMSR:
    kvmppc_set_msr(vcpu, kvmppc_get_gpr(vcpu, rs));
    break;
    case OP_31_XOP_MFSR:
    {
    int srnum;
    srnum = kvmppc_get_field(inst, 12 + 32, 15 + 32);
    if (vcpu.arch.mmu.mfsrin) {
    u32 sr;
    sr = vcpu.arch.mmu.mfsrin(vcpu, srnum);
    kvmppc_set_gpr(vcpu, rt, sr);
    }
    break;
    }
    case OP_31_XOP_MFSRIN:
    {
    int srnum;
    srnum = (kvmppc_get_gpr(vcpu, rb) >> 28) & 0xf;
    if (vcpu.arch.mmu.mfsrin) {
    u32 sr;
    sr = vcpu.arch.mmu.mfsrin(vcpu, srnum);
    kvmppc_set_gpr(vcpu, rt, sr);
    }
    break;
    }
    case OP_31_XOP_MTSR:
    vcpu.arch.mmu.mtsrin(vcpu,
    (inst >> 16) & 0xf,
    kvmppc_get_gpr(vcpu, rs));
    break;
    case OP_31_XOP_MTSRIN:
    vcpu.arch.mmu.mtsrin(vcpu,
    (kvmppc_get_gpr(vcpu, rb) >> 28) & 0xf,
    kvmppc_get_gpr(vcpu, rs));
    break;
    case OP_31_XOP_TLBIE:
    case OP_31_XOP_TLBIEL:
    {
    let mut large: bool = (inst & 0x00200000) ? true : false;
    let mut addr: c_ulong = kvmppc_get_gpr(vcpu, rb);
    vcpu.arch.mmu.tlbie(vcpu, addr, large);
    break;
    }

    case OP_31_XOP_FAKE_SC1:
    {
// SC 1 papr hypercalls
    let mut cmd: c_ulong = kvmppc_get_gpr(vcpu, 3);
    int i;
    if ((kvmppc_get_msr(vcpu) & MSR_PR) ||
    !vcpu.arch.papr_enabled) {
    emulated = EMULATE_FAIL;
    break;
    }
    if (kvmppc_h_pr(vcpu, cmd) == EMULATE_DONE)
    break;
    vcpu.run.papr_hcall.nr = cmd;
    for (i = 0; i < 9; ++i) {
    let mut gpr: c_ulong = kvmppc_get_gpr(vcpu, 4 + i);
    vcpu.run.papr_hcall.args[i] = gpr;
    }
    vcpu.run.exit_reason = KVM_EXIT_PAPR_HCALL;
    vcpu.arch.hcall_needed = 1;
    emulated = EMULATE_EXIT_USER;
    break;
    }

    case OP_31_XOP_EIOIO:
    break;
    case OP_31_XOP_SLBMTE:
    if (!vcpu.arch.mmu.slbmte)
    return EMULATE_FAIL;
    vcpu.arch.mmu.slbmte(vcpu,
    kvmppc_get_gpr(vcpu, rs),
    kvmppc_get_gpr(vcpu, rb));
    break;
    case OP_31_XOP_SLBIE:
    if (!vcpu.arch.mmu.slbie)
    return EMULATE_FAIL;
    vcpu.arch.mmu.slbie(vcpu,
    kvmppc_get_gpr(vcpu, rb));
    break;
    case OP_31_XOP_SLBIA:
    if (!vcpu.arch.mmu.slbia)
    return EMULATE_FAIL;
    vcpu.arch.mmu.slbia(vcpu);
    break;
    case OP_31_XOP_SLBFEE:
    if (!(inst & 1) || !vcpu.arch.mmu.slbfee) {
    return EMULATE_FAIL;
    } else {
    ulong b, t;
    let mut cr: c_ulong = kvmppc_get_cr(vcpu) & ~CR0_MASK;
    b = kvmppc_get_gpr(vcpu, rb);
    if (!vcpu.arch.mmu.slbfee(vcpu, b, &t))
    cr |= 2 << CR0_SHIFT;
    kvmppc_set_gpr(vcpu, rt, t);
// copy XER[SO] bit to CR0[SO]
    cr |= (vcpu.arch.regs.xer & 0x80000000) >>
    (31 - CR0_SHIFT);
    kvmppc_set_cr(vcpu, cr);
    }
    break;
    case OP_31_XOP_SLBMFEE:
    if (!vcpu.arch.mmu.slbmfee) {
    emulated = EMULATE_FAIL;
    } else {
    ulong t, rb_val;
    rb_val = kvmppc_get_gpr(vcpu, rb);
    t = vcpu.arch.mmu.slbmfee(vcpu, rb_val);
    kvmppc_set_gpr(vcpu, rt, t);
    }
    break;
    case OP_31_XOP_SLBMFEV:
    if (!vcpu.arch.mmu.slbmfev) {
    emulated = EMULATE_FAIL;
    } else {
    ulong t, rb_val;
    rb_val = kvmppc_get_gpr(vcpu, rb);
    t = vcpu.arch.mmu.slbmfev(vcpu, rb_val);
    kvmppc_set_gpr(vcpu, rt, t);
    }
    break;
    case OP_31_XOP_DCBA:
// Gets treated as NOP
    break;
    case OP_31_XOP_DCBZ:
    {
    let mut rb_val: c_ulong = kvmppc_get_gpr(vcpu, rb);
    let mut ra_val: c_ulong = 0;
    ulong addr, vaddr;
    u32 zeros[8] = { 0, 0, 0, 0, 0, 0, 0, 0 };
    u32 dsisr;
    int r;
    if (ra)
    ra_val = kvmppc_get_gpr(vcpu, ra);
    addr = (ra_val + rb_val) & ~31ULL;
    if (!(kvmppc_get_msr(vcpu) & MSR_SF))
    addr &= 0xffffffff;
    vaddr = addr;
    r = kvmppc_st(vcpu, &addr, 32, zeros, true);
    if ((r == -ENOENT) || (r == -EPERM)) {
// advance = 0;
    kvmppc_set_dar(vcpu, vaddr);
    vcpu.arch.fault_dar = vaddr;
    dsisr = DSISR_ISSTORE;
    if (r == -ENOENT)
    dsisr |= DSISR_NOHPTE;
#[no_mangle]
pub unsafe extern "C" fn if(-EPERM: r ==) -> else {
    else if (r == -EPERM)
    dsisr |= DSISR_PROTFAULT;
    kvmppc_set_dsisr(vcpu, dsisr);
    vcpu.arch.fault_dsisr = dsisr;
    kvmppc_book3s_queue_irqprio(vcpu,
    BOOK3S_INTERRUPT_DATA_STORAGE);
    }
    break;
    }

    case OP_31_XOP_TBEGIN:
    {
    if (!cpu_has_feature(CPU_FTR_TM))
    break;
    if (!(kvmppc_get_msr(vcpu) & MSR_TM)) {
    kvmppc_trigger_fac_interrupt(vcpu, FSCR_TM_LG);
    emulated = EMULATE_AGAIN;
    break;
    }
    if (!(kvmppc_get_msr(vcpu) & MSR_PR)) {
    preempt_disable();
    vcpu.arch.regs.ccr = (CR0_TBEGIN_FAILURE |
    (vcpu.arch.regs.ccr & ~(CR0_MASK << CR0_SHIFT)));
    vcpu.arch.texasr = (TEXASR_FS | TEXASR_EXACT |
    (((u64)(TM_CAUSE_EMULATE | TM_CAUSE_PERSISTENT))
    << TEXASR_FC_LG));
    if ((inst >> 21) & 0x1)
    vcpu.arch.texasr |= TEXASR_ROT;
    if (kvmppc_get_msr(vcpu) & MSR_HV)
    vcpu.arch.texasr |= TEXASR_HV;
    vcpu.arch.tfhar = kvmppc_get_pc(vcpu) + 4;
    vcpu.arch.tfiar = kvmppc_get_pc(vcpu);
    kvmppc_restore_tm_sprs(vcpu);
    preempt_enable();
    } else
    emulated = EMULATE_FAIL;
    break;
    }
    case OP_31_XOP_TABORT:
    {
    let mut guest_msr: c_ulong = kvmppc_get_msr(vcpu);
    let mut ra_val: c_ulong = 0;
    if (!cpu_has_feature(CPU_FTR_TM))
    break;
    if (!(kvmppc_get_msr(vcpu) & MSR_TM)) {
    kvmppc_trigger_fac_interrupt(vcpu, FSCR_TM_LG);
    emulated = EMULATE_AGAIN;
    break;
    }
// only emulate for privilege guest, since problem state
// guest can run with TM enabled and we don't expect to
// trap at here for that case.
//
    WARN_ON(guest_msr & MSR_PR);
    if (ra)
    ra_val = kvmppc_get_gpr(vcpu, ra);
    kvmppc_emulate_tabort(vcpu, ra_val);
    break;
    }
    case OP_31_XOP_TRECLAIM:
    {
    let mut guest_msr: c_ulong = kvmppc_get_msr(vcpu);
    let mut ra_val: c_ulong = 0;
    if (!cpu_has_feature(CPU_FTR_TM))
    break;
    if (!(kvmppc_get_msr(vcpu) & MSR_TM)) {
    kvmppc_trigger_fac_interrupt(vcpu, FSCR_TM_LG);
    emulated = EMULATE_AGAIN;
    break;
    }
// generate interrupts based on priorities
    if (guest_msr & MSR_PR) {
// Privileged Instruction type Program Interrupt
    kvmppc_core_queue_program(vcpu, SRR1_PROGPRIV);
    emulated = EMULATE_AGAIN;
    break;
    }
    if (!MSR_TM_ACTIVE(guest_msr)) {
// TM bad thing interrupt
    kvmppc_core_queue_program(vcpu, SRR1_PROGTM);
    emulated = EMULATE_AGAIN;
    break;
    }
    if (ra)
    ra_val = kvmppc_get_gpr(vcpu, ra);
    kvmppc_emulate_treclaim(vcpu, ra_val);
    break;
    }
    case OP_31_XOP_TRCHKPT:
    {
    let mut guest_msr: c_ulong = kvmppc_get_msr(vcpu);
    unsigned long texasr;
    if (!cpu_has_feature(CPU_FTR_TM))
    break;
    if (!(kvmppc_get_msr(vcpu) & MSR_TM)) {
    kvmppc_trigger_fac_interrupt(vcpu, FSCR_TM_LG);
    emulated = EMULATE_AGAIN;
    break;
    }
// generate interrupt based on priorities
    if (guest_msr & MSR_PR) {
// Privileged Instruction type Program Intr
    kvmppc_core_queue_program(vcpu, SRR1_PROGPRIV);
    emulated = EMULATE_AGAIN;
    break;
    }
    tm_enable();
    texasr = mfspr(SPRN_TEXASR);
    tm_disable();
    if (MSR_TM_ACTIVE(guest_msr) ||
    !(texasr & (TEXASR_FS))) {
// TM bad thing interrupt
    kvmppc_core_queue_program(vcpu, SRR1_PROGTM);
    emulated = EMULATE_AGAIN;
    break;
    }
    kvmppc_emulate_trchkpt(vcpu);
    break;
    }

    default:
    emulated = EMULATE_FAIL;
    }
    break;
    default:
    emulated = EMULATE_FAIL;
    }
    if (emulated == EMULATE_FAIL)
    emulated = kvmppc_emulate_paired_single(vcpu);
    return emulated;
    }
    void kvmppc_set_bat(struct kvm_vcpu *vcpu, struct kvmppc_bat *bat, bool upper,
    u32 val)
    {
    if (upper) {
// Upper BAT
    let mut bl: u32 = (val >> 2) & 0x7ff;
    bat.bepi_mask = (~bl << 17);
    bat.bepi = val & 0xfffe0000;
    bat.vs = (val & 2) ? 1 : 0;
    bat.vp = (val & 1) ? 1 : 0;
    bat.raw = (bat.raw & 0xffffffff00000000ULL) | val;
    } else {
// Lower BAT
    bat.brpn = val & 0xfffe0000;
    bat.wimg = (val >> 3) & 0xf;
    bat.pp = val & 3;
    bat.raw = (bat.raw & 0x00000000ffffffffULL) | ((u64)val << 32);
    }
    }
    static struct kvmppc_bat *kvmppc_find_bat(struct kvm_vcpu *vcpu, int sprn)
    {
    struct kvmppc_vcpu_book3s *vcpu_book3s = to_book3s(vcpu);
    struct kvmppc_bat *bat;
    switch (sprn) {
    case SPRN_IBAT0U ... SPRN_IBAT3L:
    bat = &vcpu_book3s.ibat[(sprn - SPRN_IBAT0U) / 2];
    break;
    case SPRN_IBAT4U ... SPRN_IBAT7L:
    bat = &vcpu_book3s.ibat[4 + ((sprn - SPRN_IBAT4U) / 2)];
    break;
    case SPRN_DBAT0U ... SPRN_DBAT3L:
    bat = &vcpu_book3s.dbat[(sprn - SPRN_DBAT0U) / 2];
    break;
    case SPRN_DBAT4U ... SPRN_DBAT7L:
    bat = &vcpu_book3s.dbat[4 + ((sprn - SPRN_DBAT4U) / 2)];
    break;
    default:
    BUG();
    }
    return bat;
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_core_emulate_mtspr_pr(vcpu: *mut kvm_vcpu, sprn: c_int, spr_val: c_ulong) -> c_int {
    int kvmppc_core_emulate_mtspr_pr(struct kvm_vcpu *vcpu, int sprn, ulong spr_val)
    {
    let mut emulated: c_int = EMULATE_DONE;
    switch (sprn) {
    case SPRN_SDR1:
    if (!spr_allowed(vcpu, PRIV_HYPER))
    goto unprivileged;
    to_book3s(vcpu).sdr1 = spr_val;
    break;
    case SPRN_DSISR:
    kvmppc_set_dsisr(vcpu, spr_val);
    break;
    case SPRN_DAR:
    kvmppc_set_dar(vcpu, spr_val);
    break;
    case SPRN_HIOR:
    to_book3s(vcpu).hior = spr_val;
    break;
    case SPRN_IBAT0U ... SPRN_IBAT3L:
    case SPRN_IBAT4U ... SPRN_IBAT7L:
    case SPRN_DBAT0U ... SPRN_DBAT3L:
    case SPRN_DBAT4U ... SPRN_DBAT7L:
    {
    struct kvmppc_bat *bat = kvmppc_find_bat(vcpu, sprn);
    kvmppc_set_bat(vcpu, bat, !(sprn % 2), (u32)spr_val);
// BAT writes happen so rarely that we're ok to flush
// everything here
    kvmppc_mmu_pte_flush(vcpu, 0, 0);
    kvmppc_mmu_flush_segments(vcpu);
    break;
    }
    case SPRN_HID0:
    to_book3s(vcpu).hid[0] = spr_val;
    break;
    case SPRN_HID1:
    to_book3s(vcpu).hid[1] = spr_val;
    break;
    case SPRN_HID2_750FX:
    to_book3s(vcpu).hid[2] = spr_val;
    break;
    case SPRN_HID2_GEKKO:
    to_book3s(vcpu).hid[2] = spr_val;
// HID2.PSE controls paired single on gekko
    switch (vcpu.arch.pvr) {
    case 0x00080200:	/* lonestar 2.0 */
    case 0x00088202:	/* lonestar 2.2 */
    case 0x70000100:	/* gekko 1.0 */
    case 0x00080100:	/* gekko 2.0 */
    case 0x00083203:	/* gekko 2.3a */
    case 0x00083213:	/* gekko 2.3b */
    case 0x00083204:	/* gekko 2.4 */
    case 0x00083214:	/* gekko 2.4e (8SE) - retail HW2 */
    case 0x00087200:	/* broadway */
    if (vcpu.arch.hflags & BOOK3S_HFLAG_NATIVE_PS) {
// Native paired singles
    } else if (spr_val & (1 << 29)) { /* HID2.PSE */
    vcpu.arch.hflags |= BOOK3S_HFLAG_PAIRED_SINGLE;
    kvmppc_giveup_ext(vcpu, MSR_FP);
    } else {
    vcpu.arch.hflags &= ~BOOK3S_HFLAG_PAIRED_SINGLE;
    }
    break;
    }
    break;
    case SPRN_HID4:
    case SPRN_HID4_GEKKO:
    to_book3s(vcpu).hid[4] = spr_val;
    break;
    case SPRN_HID5:
    to_book3s(vcpu).hid[5] = spr_val;
// guest HID5 set can change is_dcbz32
    if (vcpu.arch.mmu.is_dcbz32(vcpu) &&
    (mfmsr() & MSR_HV))
    vcpu.arch.hflags |= BOOK3S_HFLAG_DCBZ32;
    break;
    case SPRN_GQR0:
    case SPRN_GQR1:
    case SPRN_GQR2:
    case SPRN_GQR3:
    case SPRN_GQR4:
    case SPRN_GQR5:
    case SPRN_GQR6:
    case SPRN_GQR7:
    to_book3s(vcpu).gqr[sprn - SPRN_GQR0] = spr_val;
    break;

    case SPRN_FSCR:
    kvmppc_set_fscr(vcpu, spr_val);
    break;
    case SPRN_BESCR:
    vcpu.arch.bescr = spr_val;
    break;
    case SPRN_EBBHR:
    vcpu.arch.ebbhr = spr_val;
    break;
    case SPRN_EBBRR:
    vcpu.arch.ebbrr = spr_val;
    break;

    case SPRN_TFHAR:
    case SPRN_TEXASR:
    case SPRN_TFIAR:
    if (!cpu_has_feature(CPU_FTR_TM))
    break;
    if (!(kvmppc_get_msr(vcpu) & MSR_TM)) {
    kvmppc_trigger_fac_interrupt(vcpu, FSCR_TM_LG);
    emulated = EMULATE_AGAIN;
    break;
    }
    if (MSR_TM_ACTIVE(kvmppc_get_msr(vcpu)) &&
    !((MSR_TM_SUSPENDED(kvmppc_get_msr(vcpu))) &&
    (sprn == SPRN_TFHAR))) {
// it is illegal to mtspr() TM regs in
// other than non-transactional state, with
// the exception of TFHAR in suspend state.
//
    kvmppc_core_queue_program(vcpu, SRR1_PROGTM);
    emulated = EMULATE_AGAIN;
    break;
    }
    tm_enable();
    if (sprn == SPRN_TFHAR)
    mtspr(SPRN_TFHAR, spr_val);
#[no_mangle]
pub unsafe extern "C" fn if(SPRN_TEXASR: sprn ==) -> else {
    else if (sprn == SPRN_TEXASR)
    mtspr(SPRN_TEXASR, spr_val);
    else
    mtspr(SPRN_TFIAR, spr_val);
    tm_disable();
    break;

    case SPRN_ICTC:
    case SPRN_THRM1:
    case SPRN_THRM2:
    case SPRN_THRM3:
    case SPRN_CTRLF:
    case SPRN_CTRLT:
    case SPRN_L2CR:
    case SPRN_DSCR:
    case SPRN_MMCR0_GEKKO:
    case SPRN_MMCR1_GEKKO:
    case SPRN_PMC1_GEKKO:
    case SPRN_PMC2_GEKKO:
    case SPRN_PMC3_GEKKO:
    case SPRN_PMC4_GEKKO:
    case SPRN_WPAR_GEKKO:
    case SPRN_MSSSR0:
    case SPRN_DABR:

    case SPRN_MMCRS:
    case SPRN_MMCRA:
    case SPRN_MMCR0:
    case SPRN_MMCR1:
    case SPRN_MMCR2:
    case SPRN_UMMCR2:
    case SPRN_UAMOR:
    case SPRN_IAMR:
    case SPRN_AMR:

    break;
    unprivileged:
    default:
    pr_info_ratelimited("KVM: invalid SPR write: %d\n", sprn);
    if (sprn & 0x10) {
    if (kvmppc_get_msr(vcpu) & MSR_PR) {
    kvmppc_core_queue_program(vcpu, SRR1_PROGPRIV);
    emulated = EMULATE_AGAIN;
    }
    } else {
    if ((kvmppc_get_msr(vcpu) & MSR_PR) || sprn == 0) {
    kvmppc_core_queue_program(vcpu, SRR1_PROGILL);
    emulated = EMULATE_AGAIN;
    }
    }
    break;
    }
    return emulated;
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_core_emulate_mfspr_pr(vcpu: *mut kvm_vcpu, sprn: c_int, spr_val: *mut c_ulong) -> c_int {
    int kvmppc_core_emulate_mfspr_pr(struct kvm_vcpu *vcpu, int sprn, ulong *spr_val)
    {
    let mut emulated: c_int = EMULATE_DONE;
    switch (sprn) {
    case SPRN_IBAT0U ... SPRN_IBAT3L:
    case SPRN_IBAT4U ... SPRN_IBAT7L:
    case SPRN_DBAT0U ... SPRN_DBAT3L:
    case SPRN_DBAT4U ... SPRN_DBAT7L:
    {
    struct kvmppc_bat *bat = kvmppc_find_bat(vcpu, sprn);
    if (sprn % 2)
// spr_val = bat->raw >> 32;
    else
// spr_val = bat->raw;
    break;
    }
    case SPRN_SDR1:
    if (!spr_allowed(vcpu, PRIV_HYPER))
    goto unprivileged;
// spr_val = to_book3s(vcpu)->sdr1;
    break;
    case SPRN_DSISR:
// spr_val = kvmppc_get_dsisr(vcpu);
    break;
    case SPRN_DAR:
// spr_val = kvmppc_get_dar(vcpu);
    break;
    case SPRN_HIOR:
// spr_val = to_book3s(vcpu)->hior;
    break;
    case SPRN_HID0:
// spr_val = to_book3s(vcpu)->hid[0];
    break;
    case SPRN_HID1:
// spr_val = to_book3s(vcpu)->hid[1];
    break;
    case SPRN_HID2_750FX:
    case SPRN_HID2_GEKKO:
// spr_val = to_book3s(vcpu)->hid[2];
    break;
    case SPRN_HID4:
    case SPRN_HID4_GEKKO:
// spr_val = to_book3s(vcpu)->hid[4];
    break;
    case SPRN_HID5:
// spr_val = to_book3s(vcpu)->hid[5];
    break;
    case SPRN_CFAR:
    case SPRN_DSCR:
// spr_val = 0;
    break;
    case SPRN_PURR:
//
// On exit we would have updated purr
//
// spr_val = vcpu->arch.purr;
    break;
    case SPRN_SPURR:
//
// On exit we would have updated spurr
//
// spr_val = vcpu->arch.spurr;
    break;
    case SPRN_VTB:
// spr_val = to_book3s(vcpu)->vtb;
    break;
    case SPRN_IC:
// spr_val = vcpu->arch.ic;
    break;
    case SPRN_GQR0:
    case SPRN_GQR1:
    case SPRN_GQR2:
    case SPRN_GQR3:
    case SPRN_GQR4:
    case SPRN_GQR5:
    case SPRN_GQR6:
    case SPRN_GQR7:
// spr_val = to_book3s(vcpu)->gqr[sprn - SPRN_GQR0];
    break;

    case SPRN_FSCR:
// spr_val = vcpu->arch.fscr;
    break;
    case SPRN_BESCR:
// spr_val = vcpu->arch.bescr;
    break;
    case SPRN_EBBHR:
// spr_val = vcpu->arch.ebbhr;
    break;
    case SPRN_EBBRR:
// spr_val = vcpu->arch.ebbrr;
    break;

    case SPRN_TFHAR:
    case SPRN_TEXASR:
    case SPRN_TFIAR:
    if (!cpu_has_feature(CPU_FTR_TM))
    break;
    if (!(kvmppc_get_msr(vcpu) & MSR_TM)) {
    kvmppc_trigger_fac_interrupt(vcpu, FSCR_TM_LG);
    emulated = EMULATE_AGAIN;
    break;
    }
    tm_enable();
    if (sprn == SPRN_TFHAR)
// spr_val = mfspr(SPRN_TFHAR);
#[no_mangle]
pub unsafe extern "C" fn if(SPRN_TEXASR: sprn ==) -> else {
    else if (sprn == SPRN_TEXASR)
// spr_val = mfspr(SPRN_TEXASR);
#[no_mangle]
pub unsafe extern "C" fn if(SPRN_TFIAR: sprn ==) -> else {
    else if (sprn == SPRN_TFIAR)
// spr_val = mfspr(SPRN_TFIAR);
    tm_disable();
    break;

    case SPRN_THRM1:
    case SPRN_THRM2:
    case SPRN_THRM3:
    case SPRN_CTRLF:
    case SPRN_CTRLT:
    case SPRN_L2CR:
    case SPRN_MMCR0_GEKKO:
    case SPRN_MMCR1_GEKKO:
    case SPRN_PMC1_GEKKO:
    case SPRN_PMC2_GEKKO:
    case SPRN_PMC3_GEKKO:
    case SPRN_PMC4_GEKKO:
    case SPRN_WPAR_GEKKO:
    case SPRN_MSSSR0:
    case SPRN_DABR:

    case SPRN_MMCRS:
    case SPRN_MMCRA:
    case SPRN_MMCR0:
    case SPRN_MMCR1:
    case SPRN_MMCR2:
    case SPRN_UMMCR2:
    case SPRN_TIR:
    case SPRN_UAMOR:
    case SPRN_IAMR:
    case SPRN_AMR:

// spr_val = 0;
    break;
    default:
    unprivileged:
    pr_info_ratelimited("KVM: invalid SPR read: %d\n", sprn);
    if (sprn & 0x10) {
    if (kvmppc_get_msr(vcpu) & MSR_PR) {
    kvmppc_core_queue_program(vcpu, SRR1_PROGPRIV);
    emulated = EMULATE_AGAIN;
    }
    } else {
    if ((kvmppc_get_msr(vcpu) & MSR_PR) || sprn == 0 ||
    sprn == 4 || sprn == 5 || sprn == 6) {
    kvmppc_core_queue_program(vcpu, SRR1_PROGILL);
    emulated = EMULATE_AGAIN;
    }
    }
    break;
    }
    return emulated;
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_alignment_dsisr(vcpu: *mut kvm_vcpu, inst: c_uint) -> u32 {
    u32 kvmppc_alignment_dsisr(struct kvm_vcpu *vcpu, unsigned int inst)
    {
    return make_dsisr(inst);
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_alignment_dar(vcpu: *mut kvm_vcpu, inst: c_uint) -> c_ulong {
    ulong kvmppc_alignment_dar(struct kvm_vcpu *vcpu, unsigned int inst)
    {

//
// Linux's fix_alignment() assumes that DAR is valid, so can we
//
    return vcpu.arch.fault_dar;

    let mut dar: c_ulong = 0;
    let mut ra: c_ulong = get_ra(inst);
    let mut rb: c_ulong = get_rb(inst);
    switch (get_op(inst)) {
    case OP_LFS:
    case OP_LFD:
    case OP_STFD:
    case OP_STFS:
    if (ra)
    dar = kvmppc_get_gpr(vcpu, ra);
    dar += (s32)((s16)inst);
    break;
    case 31:
    if (ra)
    dar = kvmppc_get_gpr(vcpu, ra);
    dar += kvmppc_get_gpr(vcpu, rb);
    break;
    default:
    printk(KERN_INFO "KVM: Unaligned instruction 0x%x\n", inst);
    break;
    }
    return dar;

    }
