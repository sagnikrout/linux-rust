//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/kvm/hyp/include/hyp/switch.h
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
// Copyright (C) 2015 - ARM Ltd
// Author: Marc Zyngier <marc.zyngier@arm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_exception_table_entry {
    pub fixup: int insn,,
}

// Save the 32-bit only FPSIMD system register state
//
// We are about to set CPTR_EL2.TFP to trap all floating point
// register accesses to EL2, however, the ARM ARM clearly states that
// traps are only taken to EL2 if the operation would not otherwise
// trap to EL1.  Therefore, always make sure that for 32-bit guests,
// we set FPEXC.EN to prevent traps to EL1, when setting the TFP bit.
// If FP/ASIMD is not implemented, FPEXC is UNDEFINED and any access to
// it will cause an exception.
//
// Always trap SME since it's not supported in KVM.
// TSM is RES1 if SME isn't implemented.
//
// With VHE (HCR.E2H == 1), accesses to CPACR_EL1 are routed to
// CPTR_EL2. In general, CPACR_EL1 has the same layout as CPTR_EL2,
// except for some missing controls, such as TAM.
// In this case, CPTR_EL2.TAM has the same position with or without
// VHE (HCR.E2H == 1) which allows us to use here the CPTR_EL2.TAM
// shift value for trapping the AMU accesses.
//
// The architecture is a bit crap (what a surprise): an EL2 guest
// writing to CPTR_EL2 via CPACR_EL1 can't set any of TCPAC or TTA,
// as they are RES0 in the guest's view. To work around it, trap the
// sucker using the very same bit it can't set. FEAT_NV2p1 fixes it.
//
// Layer the guest hypervisor's trap configuration on top of our own if
// we're in a nested context.
//
// Pay attention, there's some interesting detail here.
//
// The CPTR_EL2.xEN fields are 2 bits wide, although there are only two
// meaningful trap states when HCR_EL2.TGE = 0 (running a nested guest):
//
// - CPTR_EL2.xEN = x0, traps are enabled
// - CPTR_EL2.xEN = x1, traps are disabled
//
// In other words, bit[0] determines if guest accesses trap or not. In
// the interest of simplicity, clear the entire field if the guest
// hypervisor has traps enabled to dispel any illusion of something more
// complicated taking place.
//

// trap guest access to MPAMIDR_EL1
// From v1.1 TIDR can trap MPAMIDR, set it unconditionally
//
// Just like for HCR_EL2, we can't let the guest mess with some of the
// basics we rely on in HCRX_EL2. However, the major difference is that
// HCRX_EL2 only affects EL1, and never EL2 (sudden outburst of sanity, I
// guess). So it is always the guest inflicting it on its own guestx.
//
// Things we don't want to let the guest control are:
//
// - TMEA: That's for us to decide how an SEA is routed, not the guest.
//
// - PTTWI: Similarly, it is for us to decide whether Reduced Coherency for
// the PTW is a thing. It really isn't.
//
// - EnIDCP128: We don't allow IMPDEF sysregs -- full stop.
//

// Trap on AArch32 cp15 c15 (impdef sysregs) accesses (EL1 or EL0)
//
// Make sure we trap PMU access from EL0 to EL2. Also sanitize
// PMSELR_EL0 to make sure it never contains the cycle
// counter, which could make a PMXEVCNTR_EL0 access UNDEF at
// EL1 instead of being trapped to EL2.
//
// When HCR_EL2.AMO is set, physical SErrors are taken to EL2
// and vSError injection is enabled for EL1. Conveniently, for
// NV this means that it is never the case where a 'physical'
// SError (injected by KVM or userspace) and vSError are
// deliverable to the same context.
//
// As such, we can trivially select between the host or guest's
// VSESR_EL2. Except for the case that FEAT_RAS hasn't been
// exposed to the guest, where ESR propagation in hardware
// occurs unconditionally.
//
// Paper over the architectural wart and use an IMPLEMENTATION
// DEFINED ESR value in case FEAT_RAS is hidden from the guest.
//
// If we pended a virtual abort, preserve it until it gets
// cleared. See D1.14.3 (Virtual Interrupts) for details, but
// the crucial bit is "On taking a vSError interrupt,
// HCR_EL2.VSE is cleared to 0."
//
// Additionally, when in a nested context we need to propagate the
// updated state to the guest hypervisor's HCR_EL2.
//
// hcr &= ~HCR_VSE;
// hcr |= read_sysreg(hcr_el2) & HCR_VSE;
extern "C" {
    pub fn __get_fault_info(_arg: vcpu->arch.fault.esr_el2, _arg: &vcpu->arch.fault) -> return;
}
// vcpu_pc(vcpu) = read_sysreg_el2(SYS_ELR);
//
// Finish potential single step before executing the prologue
// instruction. Modify the hardware SPSR_EL2 directly, as vcpu_cpsr()
// may hold a synthetic (vEL2) value for a guest hypervisor.
//
// The vCPU's saved SVE state layout always matches the max VL of the
// vCPU. Start off with the max VL so we can load the SVE state.
//
// The effective VL for a VM could differ from the max VL when running a
// nested guest, as the guest hypervisor could select a smaller VL. Slap
// that into hardware before wrapping up.
//
// A guest hypervisor may restrict the effective max VL.
//
// When the guest owns the FP regs, we know that guest+hyp traps for
// any FPSIMD/SVE/SME features exposed to the guest have been disabled
// by either __activate_cptr_traps() or kvm_hyp_handle_fpsimd()
// prior to __guest_entry(). As __guest_entry() guarantees a context
// synchronization event, we don't need an ISB here to avoid taking
// traps for anything that was exposed to the guest.
//
// The guest's state is always saved using the guest's max VL.
// Ensure that the host has the guest's max VL active such that
// the host can save the guest's state lazily, but don't
// artificially restrict the host to the guest's max VL.
//
// Non-protected kvm relies on the host restoring its sve state.
// Protected kvm restores the host's sve state as not to reveal that
// fpsimd was used by a guest nor leak upper sve bits.
//
// We trap the first access to the FP/SIMD to save the host context and
// restore the guest context lazily.
// If FP/SIMD is not implemented, handle the trap and inject an undefined
// instruction exception to the guest. Similarly for trapped SVE accesses.
//
// Only handle traps the vCPU can support here:
// Forward traps to the guest hypervisor as required
// Valid trap.  Switch the context:
// First disable enough traps to allow us to update the registers
// Write out the host state if it's in the registers
// Restore the guest state
// Skip restoring fpexc32 for AArch64 guests
// host_data_ptr(fp_owner) = FP_STATE_GUEST_OWNED;
//
// Re-enable traps necessary for the current state of the guest, e.g.
// those enabled by a guest hypervisor. The ERET to the guest will
// provide the necessary context synchronization.
//
// The normal sysreg handling code expects to see the traps,
// let's not do anything here.
//
extern "C" {
    pub fn arch_timer_read_cntpct_el0(timer_get_offset(ctxt: ) -) -> return;
}
//
// We only get here for 64bit guests, 32bit guests will hit
// the long and winding road all the way to the standard
// handling. Yes, it sucks to be irrelevant.
//
// Also, we only deal with non-hypervisor context here (either
// an EL1 guest, or a non-HYP context of an EL2 guest).
//
// Check for guest hypervisor trapping
//
// Affected parts do not advertise support for hardware Access Flag
// Dirty state management in ID_AA64MMFR1_EL1.HAFDBS, but the underlying
// control bits are still functional. The architecture requires these be
// RES0 on systems that do not implement FEAT_HAFDBS.
//
// Uphold the requirements of the architecture by masking guest writes
// to TCR_EL1.{HA,HD} here.
//

// Promote an illegal access to an SError.
// exit_code = ARM_EXCEPTION_EL1_SERROR;
extern "C" {
    pub fn bool(: *mut *mut exit_handler_fn)(struct kvm_vcpu, : *mut u64) -> typedef;
}
//
// Allow the hypervisor to handle the exit with an exit handler if it has one.
//
// Returns true if the hypervisor handled the exit, and control should go back
// to the guest, or false if it hasn't.
//
extern "C" {
    pub fn fn(_arg: vcpu, _arg: exit_code) -> return;
}
//
// Check for the conditions of Cortex-A510's #2077057. When these occur
// SPSR_EL2 can't be trusted, but isn't needed either as it is
// unchanged from the value in vcpu_gp_regs(vcpu)->pstate.
// Are we single-stepping the guest, and took a PAC exception from the
// active-not-pending state?
//
// vcpu_cpsr(vcpu) & DBG_SPSR_SS				&&
//
// Return true when we were able to fixup the guest exit and should return to
// the guest, false when we should restore the host state and return to the
// main run loop.
//
// HVC already have an adjusted PC, which we need to
// correct in order to return to after having injected
// the SError.
//
// SMC, on the other hand, is *trapped*, meaning its
// preferred return address is the SMC itself.
//
// We're using the raw exception code in order to only process
// the trap if no SError is pending. We will come back to the
// same PC once the SError has been injected, and replay the
// trapping instruction.
//
// Check if there's an exit handler and allow it to handle the exit.
// Return to the host kernel and handle the exit
// Re-enter the guest
// Trigger a panic after restoring the hyp context.
