//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/hw_irq.h
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
// Copyright (C) 1999 Cort Dougan <cort@cs.nmt.edu>
//

//
// PACA flags in paca->irq_happened.
//
// This bits are set when interrupts occur while soft-disabled
// and allow a proper replay.
//
// The PACA_IRQ_HARD_DIS is set whenever we hard disable. It is almost
// always in synch with the MSR[EE] state, except:
// - A window in interrupt entry, where hardware disables MSR[EE] and that
// must be "reconciled" with the soft mask state.
// - NMI interrupts that hit in awkward places, until they fix the state.
// - When local irqs are being enabled and state is being fixed up.
// - When returning from an interrupt there are some windows where this
// can become out of synch, but gets fixed before the RFI or before
// executing the next user instruction (see arch/powerpc/kernel/interrupt.c).
//
pub const PACA_IRQ_HARD_DIS: c_uint = 0x01;
pub const PACA_IRQ_DBELL: c_uint = 0x02;
pub const PACA_IRQ_EE: c_uint = 0x04;
pub const PACA_IRQ_DEC: c_uint = 0x08 /* Or FIT */;
pub const PACA_IRQ_HMI: c_uint = 0x10;
pub const PACA_IRQ_PMI: c_uint = 0x20;
pub const PACA_IRQ_REPLAYING: c_uint = 0x40;
//
// Some soft-masked interrupts must be hard masked until they are replayed
// (e.g., because the soft-masked handler does not clear the exception).
// Interrupt replay itself must remain hard masked too.
//

//
// flags for paca->irq_soft_mask
//
pub const IRQS_ENABLED: c_int = 0;

pub const IRQS_PMI_DISABLED: c_int = 2;

//
// The "memory" clobber acts as both a compiler barrier
// for the critical section and as a clobber because
// we changed paca->irq_soft_mask
//
// The irq mask must always include the STD bit if any are set.
//
// and interrupts don't get replayed until the standard
// interrupt (local_irq_disable()) is unmasked.
//
// Other masks must only provide additional masking beyond
// the standard, and they are also not replayed until the
// standard interrupt becomes unmasked.
//
// This could be changed, but it will require partial
// unmasks to be replayed, among other things. For now, take
// the simple approach.
//
extern "C" {
    pub fn irq_soft_mask_return() -> return;
}
extern "C" {
    pub fn arch_local_irq_restore(long: unsigned);
}
extern "C" {
    pub fn irq_soft_mask_or_return(_arg: IRQS_DISABLED) -> return;
}
extern "C" {
    pub fn arch_irqs_disabled_flags(_arg: arch_local_save_flags()) -> return;
}
//
// Invoked from PMU callback functions to set PMI bit in the paca.
// This has to be called with irq's disabled (via hard_irq_disable()).
//
// Invoked from PMU callback functions to clear the pending PMI bit
// in the paca.
//
// Invoked from PMU callback functions to check if there is a pending
// PMI bit in the paca.
//

//
// To support disabling and enabling of irq with PMI, set of
// new powerpc_local_irq_pmu_save() and powerpc_local_irq_restore()
// functions are added. These macros are implemented using generic
// linux local_irq_* code from include/linux/irqflags.h.
//

//
// Check if a lazy IRQ is pending. Should be called with IRQs hard disabled.
//
extern "C" {
    pub fn __lazy_irq_pending(_arg: get_paca()->irq_happened) -> return;
}
//
// Check if a lazy IRQ is pending, with no debugging checks.
// Should be called with IRQs hard disabled.
// For use in RI disabled code or other constrained situations.
//
extern "C" {
    pub fn __lazy_irq_pending(_arg: local_paca->irq_happened) -> return;
}
extern "C" {
    pub fn power_pmu_wants_prompt_pmi() -> bool;
}
//
// This is called by asynchronous interrupts to check whether to
// conditionally re-enable hard interrupts after having cleared
// the source of the interrupt. They are kept disabled if there
// is a different soft-masked interrupt pending that requires hard
// masking.
//
// If the PMU is not running, there is not much reason to enable
// MSR[EE] in irq handlers because any interrupts would just be
// soft-masked.
//
// TODO: Add test for 64e
//
// If PMIs are disabled then IRQs should be disabled as well,
// so we shouldn't see this condition, check for it just in
// case because we are about to enable PMIs.
//
// Do the hard enabling, only call this if should_hard_irq_enable is true.
// This allows PMI interrupts to profile irq handlers.
//
// Asynch interrupts come in with IRQS_ALL_DISABLED,
// PACA_IRQ_HARD_DIS, and MSR[EE]=0.
//
extern "C" {
    pub fn prep_irq_for_idle() -> bool;
}
extern "C" {
    pub fn prep_irq_for_idle_irqsoff() -> bool;
}
extern "C" {
    pub fn irq_set_pending_from_srr1(srr1: c_ulong);
}

extern "C" {
    pub fn force_external_irq_replay();
}

extern "C" {
    pub fn mfmsr() -> return;
}
extern "C" {
    pub fn arch_irqs_disabled_flags(_arg: arch_local_save_flags()) -> return;
}

//
// With soft-masking, MSR[EE] can change from 1 to 0
// asynchronously when irqs are disabled, and we don't want to
// set MSR[EE] back to 1 here if that has happened. A race-free
// way to do this is ensure EE is already 0. Another way it
// could be done is with a RESTART_TABLE handler, but that's
// probably overkill here.
//

