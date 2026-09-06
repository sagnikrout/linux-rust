//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kvm/inject_fault.c
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
// Fault injection for both 32 and 64bit guests.
//
// Copyright (C) 2012,2013 - ARM Ltd
// Author: Marc Zyngier <marc.zyngier@arm.com>
//
// Based on arch/arm/kvm/emulate.c
// Copyright (C) 2012 - Virtual Open Systems and Columbia University
// Author: Christoffer Dall <c.dall@virtualopensystems.com>
//

#[no_mangle]
unsafe extern "C" fn exception_target_el(vcpu: *mut kvm_vcpu) -> c_uint {
    static unsigned int exception_target_el(struct kvm_vcpu *vcpu)
    {
// If not nesting, EL1 is the only possible exception target
    if (likely(!vcpu_has_nv(vcpu)))
    return PSR_MODE_EL1h;
//
// With NV, we need to pick between EL1 and EL2. Note that we
// never deal with a nesting exception here, hence never
// changing context, and the exception itself can be delayed
// until the next entry.
//
    switch(*vcpu_cpsr(vcpu) & PSR_MODE_MASK) {
    case PSR_MODE_EL2h:
    case PSR_MODE_EL2t:
    return PSR_MODE_EL2h;
    case PSR_MODE_EL1h:
    case PSR_MODE_EL1t:
    return PSR_MODE_EL1h;
    case PSR_MODE_EL0t:
    return vcpu_el2_tge_is_set(vcpu) ? PSR_MODE_EL2h : PSR_MODE_EL1h;
    default:
    BUG();
    }
    }
#[no_mangle]
unsafe extern "C" fn exception_esr_elx(vcpu: *mut kvm_vcpu) -> enum vcpu_sysreg {
    static enum vcpu_sysreg exception_esr_elx(struct kvm_vcpu *vcpu)
    {
    if (exception_target_el(vcpu) == PSR_MODE_EL2h)
    return ESR_EL2;
    return ESR_EL1;
    }
#[no_mangle]
unsafe extern "C" fn exception_far_elx(vcpu: *mut kvm_vcpu) -> enum vcpu_sysreg {
    static enum vcpu_sysreg exception_far_elx(struct kvm_vcpu *vcpu)
    {
    if (exception_target_el(vcpu) == PSR_MODE_EL2h)
    return FAR_EL2;
    return FAR_EL1;
    }
#[no_mangle]
unsafe extern "C" fn pend_sync_exception(vcpu: *mut kvm_vcpu) {
    static void pend_sync_exception(struct kvm_vcpu *vcpu)
    {
    if (exception_target_el(vcpu) == PSR_MODE_EL1h)
    kvm_pend_exception(vcpu, EXCEPT_AA64_EL1_SYNC);
    else
    kvm_pend_exception(vcpu, EXCEPT_AA64_EL2_SYNC);
    }
#[no_mangle]
unsafe extern "C" fn pend_serror_exception(vcpu: *mut kvm_vcpu) {
    static void pend_serror_exception(struct kvm_vcpu *vcpu)
    {
    if (exception_target_el(vcpu) == PSR_MODE_EL1h)
    kvm_pend_exception(vcpu, EXCEPT_AA64_EL1_SERR);
    else
    kvm_pend_exception(vcpu, EXCEPT_AA64_EL2_SERR);
    }
#[no_mangle]
unsafe extern "C" fn __effective_sctlr2_bit(vcpu: *mut kvm_vcpu, idx: c_uint) -> bool {
    static bool __effective_sctlr2_bit(struct kvm_vcpu *vcpu, unsigned int idx)
    {
    u64 sctlr2;
    if (!kvm_has_sctlr2(vcpu.kvm))
    return false;
    if (is_nested_ctxt(vcpu) &&
    !(__vcpu_sys_reg(vcpu, HCRX_EL2) & HCRX_EL2_SCTLR2En))
    return false;
    if (exception_target_el(vcpu) == PSR_MODE_EL1h)
    sctlr2 = vcpu_read_sys_reg(vcpu, SCTLR2_EL1);
    else
    sctlr2 = vcpu_read_sys_reg(vcpu, SCTLR2_EL2);
    return sctlr2 & BIT(idx);
    }
#[no_mangle]
unsafe extern "C" fn effective_sctlr2_ease(vcpu: *mut kvm_vcpu) -> bool {
    static bool effective_sctlr2_ease(struct kvm_vcpu *vcpu)
    {
    return __effective_sctlr2_bit(vcpu, SCTLR2_EL1_EASE_SHIFT);
    }
#[no_mangle]
unsafe extern "C" fn effective_sctlr2_nmea(vcpu: *mut kvm_vcpu) -> bool {
    static bool effective_sctlr2_nmea(struct kvm_vcpu *vcpu)
    {
    return __effective_sctlr2_bit(vcpu, SCTLR2_EL1_NMEA_SHIFT);
    }
#[no_mangle]
unsafe extern "C" fn inject_abt64(vcpu: *mut kvm_vcpu, is_iabt: bool, addr: c_ulong) {
    static void inject_abt64(struct kvm_vcpu *vcpu, bool is_iabt, unsigned long addr)
    {
    let mut cpsr: c_ulong = *vcpu_cpsr(vcpu);
    let mut is_aarch32: bool = vcpu_mode_is_32bit(vcpu);
    let mut esr: u64 = 0, fsc;
    int level;
//
// If injecting an abort from a failed S1PTW, rewalk the S1 PTs to
// find the failing level. If we can't find it, assume the error was
// transient and restart without changing the state.
//
    if (kvm_vcpu_abt_iss1tw(vcpu)) {
    let mut hpfar: u64 = kvm_vcpu_get_fault_ipa(vcpu);
    int ret;
    if (hpfar == INVALID_GPA)
    return;
    ret = __kvm_find_s1_desc_level(vcpu, addr, hpfar, &level);
    if (ret)
    return;
    WARN_ON_ONCE(level < -1 || level > 3);
    fsc = ESR_ELx_FSC_SEA_TTW(level);
    } else {
    fsc = ESR_ELx_FSC_EXTABT;
    }
// This delight is brought to you by FEAT_DoubleFault2.
    if (effective_sctlr2_ease(vcpu))
    pend_serror_exception(vcpu);
    else
    pend_sync_exception(vcpu);
//
// Build an {i,d}abort, depending on the level.
// Report an external synchronous abort.
//
    esr |= ESR_ELx_IL;
//
// Here, the guest runs in AArch64 mode when in EL1. If we get
// an AArch32 fault, it means we managed to trap an EL0 fault.
//
    if (is_aarch32 || (cpsr & PSR_MODE_MASK) == PSR_MODE_EL0t)
    esr |= (ESR_ELx_EC_IABT_LOW << ESR_ELx_EC_SHIFT);
    else
    esr |= (ESR_ELx_EC_IABT_CUR << ESR_ELx_EC_SHIFT);
    if (!is_iabt)
    esr |= ESR_ELx_EC_DABT_LOW << ESR_ELx_EC_SHIFT;
    esr |= fsc;
    vcpu_write_sys_reg(vcpu, addr, exception_far_elx(vcpu));
    vcpu_write_sys_reg(vcpu, esr, exception_esr_elx(vcpu));
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_inject_sync(vcpu: *mut kvm_vcpu, esr: u64) {
    void kvm_inject_sync(struct kvm_vcpu *vcpu, u64 esr)
    {
    pend_sync_exception(vcpu);
    vcpu_write_sys_reg(vcpu, esr, exception_esr_elx(vcpu));
    }
#[no_mangle]
unsafe extern "C" fn inject_undef64(vcpu: *mut kvm_vcpu) {
    static void inject_undef64(struct kvm_vcpu *vcpu)
    {
    let mut esr: u64 = (ESR_ELx_EC_UNKNOWN << ESR_ELx_EC_SHIFT) | ESR_ELx_IL;
    kvm_inject_sync(vcpu, esr);
    }
pub const DFSR_FSC_EXTABT_LPAE: c_uint = 0x10;
pub const DFSR_FSC_EXTABT_nLPAE: c_uint = 0x08;

#[no_mangle]
unsafe extern "C" fn inject_undef32(vcpu: *mut kvm_vcpu) {
    static void inject_undef32(struct kvm_vcpu *vcpu)
    {
    kvm_pend_exception(vcpu, EXCEPT_AA32_UND);
    }
//
// Modelled after TakeDataAbortException() and TakePrefetchAbortException
// pseudocode.
//
#[no_mangle]
unsafe extern "C" fn inject_abt32(vcpu: *mut kvm_vcpu, is_pabt: bool, addr: u32) {
    static void inject_abt32(struct kvm_vcpu *vcpu, bool is_pabt, u32 addr)
    {
    u64 far;
    u32 fsr;
// Give the guest an IMPLEMENTATION DEFINED exception
    if (vcpu_read_sys_reg(vcpu, TCR_EL1) & TTBCR_EAE) {
    fsr = DFSR_LPAE | DFSR_FSC_EXTABT_LPAE;
    } else {
// no need to shuffle FS[4] into DFSR[10] as it's 0
    fsr = DFSR_FSC_EXTABT_nLPAE;
    }
    far = vcpu_read_sys_reg(vcpu, FAR_EL1);
    if (is_pabt) {
    kvm_pend_exception(vcpu, EXCEPT_AA32_IABT);
    far &= GENMASK(31, 0);
    far |= (u64)addr << 32;
    vcpu_write_sys_reg(vcpu, fsr, IFSR32_EL2);
    } else { /* !iabt */
    kvm_pend_exception(vcpu, EXCEPT_AA32_DABT);
    far &= GENMASK(63, 32);
    far |= addr;
    vcpu_write_sys_reg(vcpu, fsr, ESR_EL1);
    }
    vcpu_write_sys_reg(vcpu, far, FAR_EL1);
    }
#[no_mangle]
unsafe extern "C" fn __kvm_inject_sea(vcpu: *mut kvm_vcpu, iabt: bool, addr: u64) {
    static void __kvm_inject_sea(struct kvm_vcpu *vcpu, bool iabt, u64 addr)
    {
    if (vcpu_el1_is_32bit(vcpu))
    inject_abt32(vcpu, iabt, addr);
    else
    inject_abt64(vcpu, iabt, addr);
    }
#[no_mangle]
unsafe extern "C" fn kvm_sea_target_is_el2(vcpu: *mut kvm_vcpu) -> bool {
    static bool kvm_sea_target_is_el2(struct kvm_vcpu *vcpu)
    {
    if (__vcpu_sys_reg(vcpu, HCR_EL2) & (HCR_TGE | HCR_TEA))
    return true;
    if (!vcpu_mode_priv(vcpu))
    return false;
    return (*vcpu_cpsr(vcpu) & PSR_A_BIT) &&
    (__vcpu_sys_reg(vcpu, HCRX_EL2) & HCRX_EL2_TMEA);
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_inject_sea(vcpu: *mut kvm_vcpu, iabt: bool, addr: u64) -> c_int {
    int kvm_inject_sea(struct kvm_vcpu *vcpu, bool iabt, u64 addr)
    {
    lockdep_assert_held(&vcpu.mutex);
    if (is_nested_ctxt(vcpu) && kvm_sea_target_is_el2(vcpu))
    return kvm_inject_nested_sea(vcpu, iabt, addr);
    __kvm_inject_sea(vcpu, iabt, addr);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn kvm_inject_nested_excl_atomic(vcpu: *mut kvm_vcpu, addr: u64) -> c_int {
    static int kvm_inject_nested_excl_atomic(struct kvm_vcpu *vcpu, u64 addr)
    {
    u64 esr = FIELD_PREP(ESR_ELx_EC_MASK, ESR_ELx_EC_DABT_LOW) |
    FIELD_PREP(ESR_ELx_FSC, ESR_ELx_FSC_EXCL_ATOMIC) |
    ESR_ELx_IL;
    vcpu_write_sys_reg(vcpu, addr, FAR_EL2);
    return kvm_inject_nested_sync(vcpu, esr);
    }
//
// kvm_inject_dabt_excl_atomic - inject a data abort for unsupported exclusive
// or atomic access
// @vcpu: The VCPU to receive the data abort
// @addr: The address to report in the DFAR
//
// It is assumed that this code is called from the VCPU thread and that the
// VCPU therefore is not currently executing guest code.
//
#[no_mangle]
pub unsafe extern "C" fn kvm_inject_dabt_excl_atomic(vcpu: *mut kvm_vcpu, addr: u64) -> c_int {
    int kvm_inject_dabt_excl_atomic(struct kvm_vcpu *vcpu, u64 addr)
    {
    u64 esr;
    if (is_nested_ctxt(vcpu) && (vcpu_read_sys_reg(vcpu, HCR_EL2) & HCR_VM))
    return kvm_inject_nested_excl_atomic(vcpu, addr);
    __kvm_inject_sea(vcpu, false, addr);
    esr = vcpu_read_sys_reg(vcpu, exception_esr_elx(vcpu));
    esr &= ~ESR_ELx_FSC;
    esr |= ESR_ELx_FSC_EXCL_ATOMIC;
    vcpu_write_sys_reg(vcpu, esr, exception_esr_elx(vcpu));
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_inject_size_fault(vcpu: *mut kvm_vcpu) {
    void kvm_inject_size_fault(struct kvm_vcpu *vcpu)
    {
    unsigned long addr, esr;
    addr  = kvm_vcpu_get_fault_ipa(vcpu);
    addr |= FAR_TO_FIPA_OFFSET(kvm_vcpu_get_hfar(vcpu));
    __kvm_inject_sea(vcpu, kvm_vcpu_trap_is_iabt(vcpu), addr);
//
// If AArch64 or LPAE, set FSC to 0 to indicate an Address
// Size Fault at level 0, as if exceeding PARange.
//
// Non-LPAE guests will only get the external abort, as there
// is no way to describe the ASF.
//
    if (vcpu_el1_is_32bit(vcpu) &&
    !(vcpu_read_sys_reg(vcpu, TCR_EL1) & TTBCR_EAE))
    return;
    esr = vcpu_read_sys_reg(vcpu, exception_esr_elx(vcpu));
    esr &= ~GENMASK_ULL(5, 0);
    vcpu_write_sys_reg(vcpu, esr, exception_esr_elx(vcpu));
    }
//
// kvm_inject_undefined - inject an undefined instruction into the guest
// @vcpu: The vCPU in which to inject the exception
//
// It is assumed that this code is called from the VCPU thread and that the
// VCPU therefore is not currently executing guest code.
//
#[no_mangle]
pub unsafe extern "C" fn kvm_inject_undefined(vcpu: *mut kvm_vcpu) {
    void kvm_inject_undefined(struct kvm_vcpu *vcpu)
    {
    if (vcpu_el1_is_32bit(vcpu))
    inject_undef32(vcpu);
    else
    inject_undef64(vcpu);
    }
#[no_mangle]
unsafe extern "C" fn serror_is_masked(vcpu: *mut kvm_vcpu) -> bool {
    static bool serror_is_masked(struct kvm_vcpu *vcpu)
    {
    return (*vcpu_cpsr(vcpu) & PSR_A_BIT) && !effective_sctlr2_nmea(vcpu);
    }
#[no_mangle]
unsafe extern "C" fn kvm_serror_target_is_el2(vcpu: *mut kvm_vcpu) -> bool {
    static bool kvm_serror_target_is_el2(struct kvm_vcpu *vcpu)
    {
    if (is_hyp_ctxt(vcpu) || vcpu_el2_amo_is_set(vcpu))
    return true;
    if (!(__vcpu_sys_reg(vcpu, HCRX_EL2) & HCRX_EL2_TMEA))
    return false;
//
// In another example where FEAT_DoubleFault2 is entirely backwards,
// "masked" as it relates to the routing effects of HCRX_EL2.TMEA
// doesn't consider SCTLR2_EL1.NMEA. That is to say, even if EL1 asked
// for non-maskable SErrors, the EL2 bit takes priority if A is set.
//
    if (vcpu_mode_priv(vcpu))
    return *vcpu_cpsr(vcpu) & PSR_A_BIT;
//
// Otherwise SErrors are considered unmasked when taken from EL0 and
// NMEA is set.
//
    return serror_is_masked(vcpu);
    }
#[no_mangle]
unsafe extern "C" fn kvm_serror_undeliverable_at_el2(vcpu: *mut kvm_vcpu) -> bool {
    static bool kvm_serror_undeliverable_at_el2(struct kvm_vcpu *vcpu)
    {
    return !(vcpu_el2_tge_is_set(vcpu) || vcpu_el2_amo_is_set(vcpu));
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_inject_serror_esr(vcpu: *mut kvm_vcpu, esr: u64) -> c_int {
    int kvm_inject_serror_esr(struct kvm_vcpu *vcpu, u64 esr)
    {
    lockdep_assert_held(&vcpu.mutex);
    if (is_nested_ctxt(vcpu) && kvm_serror_target_is_el2(vcpu))
    return kvm_inject_nested_serror(vcpu, esr);
    if (vcpu_is_el2(vcpu) && kvm_serror_undeliverable_at_el2(vcpu)) {
    vcpu_set_vsesr(vcpu, esr);
    vcpu_set_flag(vcpu, NESTED_SERROR_PENDING);
    return 1;
    }
//
// Emulate the exception entry if SErrors are unmasked. This is useful if
// the vCPU is in a nested context w/ vSErrors enabled then we've already
// delegated he hardware vSError context (i.e. HCR_EL2.VSE, VSESR_EL2,
// VDISR_EL2) to the guest hypervisor.
//
// As we're emulating the SError injection we need to explicitly populate
// ESR_ELx.EC because hardware will not do it on our behalf.
//
    if (!serror_is_masked(vcpu)) {
    pend_serror_exception(vcpu);
    esr |= FIELD_PREP(ESR_ELx_EC_MASK, ESR_ELx_EC_SERROR) | ESR_ELx_IL;
    vcpu_write_sys_reg(vcpu, esr, exception_esr_elx(vcpu));
    return 1;
    }
    vcpu_set_vsesr(vcpu, esr & ESR_ELx_ISS_MASK);
// vcpu_hcr(vcpu) |= HCR_VSE;
    return 1;
    }
