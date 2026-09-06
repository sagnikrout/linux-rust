//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/kvm_book3s_asm.h
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
// XICS ICP register offsets
pub const XICS_XIRR: c_int = 4;
pub const XICS_MFRR: c_uint = 0xc;

// Maximum number of threads per physical core
pub const MAX_SMT_THREADS: c_int = 8;
// Maximum number of subcores per physical core
pub const MAX_SUBCORES: c_int = 4;

// Struct used for coordinating micro-threading (split-core) mode changes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_split_mode {
    pub rpr: c_ulong,
    pub pmmar: c_ulong,
    pub ldbar: c_ulong,
    pub subcore_size: u8,
    pub do_nap: u8,
    pub napped: [u8; MAX_SMT_THREADS],
    pub vc: [*mut kvmppc_vcore; MAX_SUBCORES],
}

//
// This struct goes in the PACA on 64-bit processors.  It is used
// to store host state that needs to be saved when we enter a guest
// and restored when we exit, but isn't specific to any particular
// guest or vcpu.  It also has some scratch fields used by the guest
// exit code.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_host_state {
    pub host_r1: c_ulong,
    pub host_r2: c_ulong,
    pub host_msr: c_ulong,
    pub vmhandler: c_ulong,
    pub scratch0: c_ulong,
    pub scratch1: c_ulong,
    pub scratch2: c_ulong,
    pub in_guest: u8,
    pub restore_hid5: u8,
    pub napping: u8,

    pub hwthread_req: u8,
    pub hwthread_state: u8,
    pub host_ipi: u8,
    pub /: *mut *mut u8 ptid; / thread number within subcore when split,
    pub fake_suspend: u8,
    pub kvm_vcpu: *mut kvm_vcpu,
    pub kvm_vcore: *mut kvmppc_vcore,
    pub xics_phys: *mut void __iomem,
    pub xive_tima_phys: *mut void __iomem,
    pub xive_tima_virt: *mut void __iomem,
    pub saved_xirr: u32,
    pub dabr: u64,
    pub /: *mut *mut u64 host_mmcr[7]; / MMCR 0,1,A, SIAR, SDAR, MMCR2, SIER,
    pub host_pmc: [u32; 8],
    pub host_purr: u64,
    pub host_spurr: u64,
    pub host_dscr: u64,
    pub dec_expires: u64,
    pub kvm_split_mode: *mut kvm_split_mode,

    pub cfar: u64,
    pub ppr: u64,
    pub host_fscr: u64,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_book3s_shadow_vcpu {
    pub in_use: bool,
    pub gpr: [c_ulong; 14],
    pub cr: u32,
    pub xer: c_ulong,
    pub ctr: c_ulong,
    pub lr: c_ulong,
    pub pc: c_ulong,
    pub shadow_srr1: c_ulong,
    pub fault_dar: c_ulong,
    pub fault_dsisr: u32,
    pub last_inst: u32,

    pub /: *mut *mut u32 sr[16]; / Guest SRs,
    pub hstate: kvmppc_host_state,

    pub /: *mut *mut u8 slb_max; / highest used guest slb entry,
    pub esid: u64,
    pub vsid: u64,
    pub /: *mut *mut } slb[64]; / guest SLB,
    pub shadow_fscr: u64,

}

// Values for kvm_state
pub const KVM_HWTHREAD_IN_KERNEL: c_int = 0;
pub const KVM_HWTHREAD_IN_IDLE: c_int = 1;
pub const KVM_HWTHREAD_IN_KVM: c_int = 2;
