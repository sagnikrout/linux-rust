//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/adreno-smmu-priv.h
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
// Copyright (C) 2020 Google, Inc
//

//
// struct adreno_smmu_fault_info - container for key fault information
//
// @far: The faulting IOVA from ARM_SMMU_CB_FAR
// @ttbr0: The current TTBR0 pagetable from ARM_SMMU_CB_TTBR0
// @contextidr: The value of ARM_SMMU_CB_CONTEXTIDR
// @fsr: The fault status from ARM_SMMU_CB_FSR
// @fsynr0: The value of FSYNR0 from ARM_SMMU_CB_FSYNR0
// @fsynr1: The value of FSYNR1 from ARM_SMMU_CB_FSYNR0
// @cbfrsynra: The value of CBFRSYNRA from ARM_SMMU_GR1_CBFRSYNRA(idx)
//
// This struct passes back key page fault information to the GPU driver
// through the get_fault_info function pointer.
// The GPU driver can use this information to print informative
// log messages and provide deeper GPU specific insight into the fault.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adreno_smmu_fault_info {
    pub far: u64,
    pub ttbr0: u64,
    pub contextidr: u32,
    pub fsr: u32,
    pub fsynr0: u32,
    pub fsynr1: u32,
    pub cbfrsynra: u32,
}

//
// struct adreno_smmu_priv - private interface between adreno-smmu and GPU
//
// @cookie:        An opque token provided by adreno-smmu and passed
// back into the callbacks
// @get_ttbr1_cfg: Get the TTBR1 config for the GPUs context-bank
// @set_ttbr0_cfg: Set the TTBR0 config for the GPUs context bank.  A
// NULL config disables TTBR0 translation, otherwise
// TTBR0 translation is enabled with the specified cfg
// @get_fault_info: Called by the GPU fault handler to get information about
// the fault
// @set_stall:     Configure whether stall on fault (CFCFG) is enabled. If
// stalling on fault is enabled, the GPU driver must call
// resume_translation()
// @resume_translation: Resume translation after a fault
//
// @set_prr_bit:   [optional] Configure the GPU's Partially Resident
// Region (PRR) bit in the ACTLR register.
// @set_prr_addr:  [optional] Configure the PRR_CFG_*ADDR register with
// the physical address of PRR page passed from GPU
// driver.
//
// The GPU driver (drm/msm) and adreno-smmu work together for controlling
// the GPU's SMMU instance.  This is by necessity, as the GPU is directly
// updating the SMMU for context switches, while on the other hand we do
// not want to duplicate all of the initial setup logic from arm-smmu.
//
// This private interface is used for the two drivers to coordinate.  The
// cookie and callback functions are populated when the GPU driver attaches
// it's domain.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adreno_smmu_priv {
    pub cookie: *const c_void,
    pub cookie): *const *const *const io_pgtable_cfg (get_ttbr1_cfg)(void,
    pub cfg): *const *const *const int (set_ttbr0_cfg)(void cookie, struct io_pgtable_cfg,
    pub info): *const *const *const void (get_fault_info)(void cookie, struct adreno_smmu_fault_info,
    pub enabled): *const *const *const void (set_stall)(void cookie, bool,
    pub terminate): *const *const *const void (resume_translation)(void cookie, bool,
    pub set): *const *const *const void (set_prr_bit)(void cookie, bool,
    pub page_addr): *const *const *const void (set_prr_addr)(void cookie, phys_addr_t,
}
