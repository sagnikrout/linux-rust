//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/el2_setup.h
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
// Copyright (C) 2012,2013 - ARM Ltd
// Author: Marc Zyngier <marc.zyngier@arm.com>
//

//
// Compliant CPUs advertise their VHE-onlyness with
// ID_AA64MMFR4_EL1.E2H0 < 0. On such CPUs HCR_EL2.E2H is RES1, but it
// can reset into an UNKNOWN state and might not read as 1 until it has
// been initialized explicitly.
// Initialize HCR_EL2.E2H so that later code can rely upon HCR_EL2.E2H
// indicating whether the CPU is running in E2H mode.
//
// Unfortunately, HCR_EL2.E2H can be RES1 even if not advertised
// as such via ID_AA64MMFR4_EL1.E2H0:
//
// - Fruity CPUs predate the !FEAT_E2H0 relaxation, and seem to
// have HCR_EL2.E2H implemented as RAO/WI.
//
// - On CPUs that lack FEAT_FGT, a hypervisor can't trap guest
// reads of ID_AA64MMFR4_EL1 to advertise !FEAT_E2H0. NV
// guests on these hosts can write to HCR_EL2.E2H without
// trapping to the hypervisor, but these writes have no
// functional effect.
//
// Handle both cases by checking for an essential VHE property
// (system register remapping) to decide whether we're
// effectively VHE-only or not.
//
// Enable GCS if supported
// Enable LS64, LS64_V if supported
// Check if running in host at EL2 mode, i.e., (h)VHE. Jump to fail if not.
//
// Allow Non-secure EL1 and EL0 to access physical timer and counter.
// This is not necessary for VHE, since the host kernel runs in EL2,
// and EL0 accesses are configured in the later stage of boot process.
// Note that when HCR_EL2.E2H == 1, CNTHCTL_EL2 has the same bit layout
// as CNTKCTL_EL1, and CNTKCTL_EL1 accessing instructions are redefined
// to access CNTHCTL_EL2. This allows the kernel designed to run at EL1
// to transparently mess with the EL0 bits via CNTKCTL_EL1 access in
// EL2.
//
// Branch to skip_label if SPE version is less than given version
// Statistical profiling
// use EL1&0 translation.
// Trace buffer
// to own it.
// LORegions
// Stage-2 translation
// GICv3 system register access
// GICv5 system register access
// Virtual CPU ID registers
// Coprocessor traps
//
// Configure BRBE to permit recording cycle counts and branch mispredicts.
//
// At any EL, to record cycle counts BRBE requires that both BRBCR_EL2.CC=1 and
// BRBCR_EL1.CC=1.
//
// At any EL, to record branch mispredicts BRBE requires that both
// BRBCR_EL2.MPRED=1 and BRBCR_EL1.MPRED=1.
//
// Set {CC,MPRED} in BRBCR_EL2 in case nVHE mode is used and we are
// executing in EL1.
//
// Disable any fine grained traps
// If SPEv1p2 is implemented,
// Disable PMSNEVFR_EL1 read and write traps
//
// Disable read traps for the following registers
//
// [BRBSRC|BRBTGT|RBINF]_EL1
// [BRBSRCINJ|BRBTGTINJ|BRBINFINJ|BRBTS]_EL1
//
// Disable write traps for the following registers
//
// [BRBSRCINJ|BRBTGTINJ|BRBINFINJ|BRBTS]_EL1
//
// Disable read and write traps for [BRBCR|BRBFCR]_EL1
// Disable read traps for BRBIDR_EL1
// Disable traps for BRBIALL instruction
// Disable traps for BRBINJ instruction
// Disable nVHE traps of TPIDR2 and SMPRI
// Disable trapping of PIR_EL1 / PIRE0_EL1
// Disable trapping of POR_EL0
// GCS depends on PIE so we don't check it if PIE is absent
// Disable traps of access to GCS registers at EL0 and EL1
// If SPE is implemented,
// we can read PMSIDR and
// if FEAT_SPE_FDS is implemented,
// disable traps of PMSDSFR to EL2.
//
// Initialize EL2 registers to sane values. This should be called early on all
// cores that were booted in EL2. Note that everything gets initialised as
// if VHE was not available. The kernel context will be upgraded to VHE
// if possible later on in the boot process
//
// Regs: x0, x1 and x2 are clobbered.
//
// This will clobber tmp1 and tmp2, and expect tmp1 to contain
// the id register value as read from the HW
// This will clobber tmp1 and tmp2

// This will clobber tmp

// and disable lower traps
// (h)VHE case
// Full FP in SM?
// ZT0 available?
