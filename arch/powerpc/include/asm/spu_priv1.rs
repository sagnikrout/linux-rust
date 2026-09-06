//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/spu_priv1.h
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
// Defines an spu hypervisor abstraction layer.
//
// Copyright 2006 Sony Corp.
//

// access to priv1 registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu_priv1_ops {
    pub mask): *mut *mut *mut void (int_mask_and) (struct spu spu, int class, u64,
    pub mask): *mut *mut *mut void (int_mask_or) (struct spu spu, int class, u64,
    pub mask): *mut *mut *mut void (int_mask_set) (struct spu spu, int class, u64,
    pub class): *mut *mut *mut u64 (int_mask_get) (struct spu spu, int,
    pub stat): *mut *mut *mut void (int_stat_clear) (struct spu spu, int class, u64,
    pub class): *mut *mut *mut u64 (int_stat_get) (struct spu spu, int,
    pub cpu): *mut *mut *mut void (cpu_affinity_set) (struct spu spu, int,
    pub spu): *mut *mut u64 (mfc_dar_get) (struct spu,
    pub spu): *mut *mut u64 (mfc_dsisr_get) (struct spu,
    pub dsisr): *mut *mut *mut void (mfc_dsisr_set) (struct spu spu, u64,
    pub spu): *mut *mut void (mfc_sdr_setup) (struct spu,
    pub sr1): *mut *mut *mut void (mfc_sr1_set) (struct spu spu, u64,
    pub spu): *mut *mut u64 (mfc_sr1_get) (struct spu,
    pub tclass_id): *mut *mut *mut void (mfc_tclass_id_set) (struct spu spu, u64,
    pub spu): *mut *mut u64 (mfc_tclass_id_get) (struct spu,
    pub spu): *mut *mut void (tlb_invalidate) (struct spu,
    pub id): *mut *mut *mut void (resource_allocation_groupID_set) (struct spu spu, u64,
    pub spu): *mut *mut u64 (resource_allocation_groupID_get) (struct spu,
    pub enable): *mut *mut *mut void (resource_allocation_enable_set) (struct spu spu, u64,
    pub spu): *mut *mut u64 (resource_allocation_enable_get) (struct spu,
}

// spu management abstraction
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu_management_ops {
    pub data)): *mut *mut *mut int (enumerate_spus)(int (fn)(void,
    pub data): *mut *mut *mut int (create_spu)(struct spu spu, void,
    pub spu): *mut *mut int (destroy_spu)(struct spu,
    pub ctx): *mut *mut void (enable_spu)(struct spu_context,
    pub ctx): *mut *mut void (disable_spu)(struct spu_context,
    pub (*init_affinity)(void): *mut c_int,
}

//
// The declarations following are put here for convenience
// and only intended to be used by the platform setup code.
//

