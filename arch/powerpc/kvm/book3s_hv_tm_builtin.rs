//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kvm/book3s_hv_tm_builtin.c
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
// Copyright 2017 Paul Mackerras, IBM Corp. <paulus@au1.ibm.com>
//

//
// This handles the cases where the guest is in real suspend mode
// and we want to get back to the guest without dooming the transaction.
// The caller has checked that the guest is in real-suspend mode
// (MSR[TS] = S and the fake-suspend flag is not set).
//
#[no_mangle]
pub unsafe extern "C" fn kvmhv_p9_tm_emulation_early(vcpu: *mut kvm_vcpu) -> c_int {
    int kvmhv_p9_tm_emulation_early(struct kvm_vcpu *vcpu)
    {
    let mut instr: u32 = vcpu.arch.emul_inst;
    u64 newmsr, msr, bescr;
    int rs;
//
// rfid, rfebb, and mtmsrd encode bit 31 = 0 since it's a reserved bit
// in these instructions, so masking bit 31 out doesn't change these
// instructions. For the tsr. instruction if bit 31 = 0 then it is per
// ISA an invalid form, however P9 UM, in section 4.6.10 Book II Invalid
// Forms, informs specifically that ignoring bit 31 is an acceptable way
// to handle TM-related invalid forms that have bit 31 = 0. Moreover,
// for emulation purposes both forms (w/ and wo/ bit 31 set) can
// generate a softpatch interrupt. Hence both forms are handled below
// for tsr. to make them behave the same way.
//
    switch (instr & PO_XOP_OPCODE_MASK) {
    case PPC_INST_RFID:
// XXX do we need to check for PR=0 here?
    newmsr = vcpu.arch.shregs.srr1;
// should only get here for Sx -> T1 transition
    if (!(MSR_TM_TRANSACTIONAL(newmsr) && (newmsr & MSR_TM)))
    return 0;
    newmsr = sanitize_msr(newmsr);
    vcpu.arch.shregs.msr = newmsr;
    vcpu.arch.cfar = vcpu.arch.regs.nip - 4;
    vcpu.arch.regs.nip = vcpu.arch.shregs.srr0;
    return 1;
    case PPC_INST_RFEBB:
// check for PR=1 and arch 2.06 bit set in PCR
    msr = vcpu.arch.shregs.msr;
    if ((msr & MSR_PR) && (vcpu.arch.vcore.pcr & PCR_ARCH_206))
    return 0;
// check EBB facility is available
    if (!(vcpu.arch.hfscr & HFSCR_EBB) ||
    ((msr & MSR_PR) && !(mfspr(SPRN_FSCR) & FSCR_EBB)))
    return 0;
    bescr = mfspr(SPRN_BESCR);
// expect to see a S->T transition requested
    if (((bescr >> 30) & 3) != 2)
    return 0;
    bescr &= ~BESCR_GE;
    if (instr & (1 << 11))
    bescr |= BESCR_GE;
    mtspr(SPRN_BESCR, bescr);
    msr = (msr & ~MSR_TS_MASK) | MSR_TS_T;
    vcpu.arch.shregs.msr = msr;
    vcpu.arch.cfar = vcpu.arch.regs.nip - 4;
    vcpu.arch.regs.nip = mfspr(SPRN_EBBRR);
    return 1;
    case PPC_INST_MTMSRD:
// XXX do we need to check for PR=0 here?
    rs = (instr >> 21) & 0x1f;
    newmsr = kvmppc_get_gpr(vcpu, rs);
    msr = vcpu.arch.shregs.msr;
// check this is a Sx -> T1 transition
    if (!(MSR_TM_TRANSACTIONAL(newmsr) && (newmsr & MSR_TM)))
    return 0;
// mtmsrd doesn't change LE
    newmsr = (newmsr & ~MSR_LE) | (msr & MSR_LE);
    newmsr = sanitize_msr(newmsr);
    vcpu.arch.shregs.msr = newmsr;
    return 1;
// ignore bit 31, see comment above
    case (PPC_INST_TSR & PO_XOP_OPCODE_MASK):
// we know the MSR has the TS field = S (0b01) here
    msr = vcpu.arch.shregs.msr;
// check for PR=1 and arch 2.06 bit set in PCR
    if ((msr & MSR_PR) && (vcpu.arch.vcore.pcr & PCR_ARCH_206))
    return 0;
// check for TM disabled in the HFSCR or MSR
    if (!(vcpu.arch.hfscr & HFSCR_TM) || !(msr & MSR_TM))
    return 0;
// L=1 => tresume => set TS to T (0b10)
    if (instr & (1 << 21))
    vcpu.arch.shregs.msr = (msr & ~MSR_TS_MASK) | MSR_TS_T;
// Set CR0 to 0b0010
    vcpu.arch.regs.ccr = (vcpu.arch.regs.ccr & 0x0fffffff) |
    0x20000000;
    return 1;
    }
    return 0;
    }
//
// This is called when we are returning to a guest in TM transactional
// state.  We roll the guest state back to the checkpointed state.
//
#[no_mangle]
pub unsafe extern "C" fn kvmhv_emulate_tm_rollback(vcpu: *mut kvm_vcpu) {
    void kvmhv_emulate_tm_rollback(struct kvm_vcpu *vcpu)
    {
    vcpu.arch.shregs.msr &= ~MSR_TS_MASK;	/* go to N state */
    vcpu.arch.regs.nip = vcpu.arch.tfhar;
    copy_from_checkpoint(vcpu);
    vcpu.arch.regs.ccr = (vcpu.arch.regs.ccr & 0x0fffffff) | 0xa0000000;
    }
