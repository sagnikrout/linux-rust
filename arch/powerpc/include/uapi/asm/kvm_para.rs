//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/uapi/asm/kvm_para.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Copyright IBM Corp. 2008
//
// Authors: Hollis Blanchard <hollisb@us.ibm.com>
//

//
// Additions to this struct must only occur at the end, and should be
// accompanied by a KVM_MAGIC_FEAT flag to advertise that they are present
// (albeit not necessarily relevant to the current target hardware platform).
//
// Struct fields are always 32 or 64 bit aligned, depending on them being 32
// or 64 bit wide respectively.
//
// See Documentation/virt/kvm/ppc-pv.rst
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_arch_shared {
    pub scratch1: __u64,
    pub scratch2: __u64,
    pub scratch3: __u64,
    pub /: *mut *mut __u64 critical; / Guest may not get interrupts if == r1,
    pub sprg0: __u64,
    pub sprg1: __u64,
    pub sprg2: __u64,
    pub sprg3: __u64,
    pub srr0: __u64,
    pub srr1: __u64,
    pub /: *mut *mut __u64 dar; / dear on BookE,
    pub msr: __u64,
    pub dsisr: __u32,
    pub /: *mut *mut __u32 int_pending; / Tells the guest if we have an interrupt,
    pub sr: [__u32; 16],
    pub mas0: __u32,
    pub mas1: __u32,
    pub mas7_3: __u64,
    pub mas2: __u64,
    pub mas4: __u32,
    pub mas6: __u32,
    pub esr: __u32,
    pub pir: __u32,
//
// SPRG4-7 are user-readable, so we can only keep these consistent
// between the shared area and the real registers when there's an
// intervening exit to KVM.  This also applies to SPRG3 on some
// chips.
//
// This suffices for access by guest userspace, since in PR-mode
// KVM, an exit must occur when changing the guest's MSR[PR].
// If the guest kernel writes to SPRG3-7 via the shared area, it
// must also use the shared area for reading while in kernel space.
//
    pub sprg4: __u64,
    pub sprg5: __u64,
    pub sprg6: __u64,
    pub sprg7: __u64,
}

pub const KVM_SC_MAGIC_R0: c_uint = 0x4b564d21 /* "KVM!" */;

pub const KVM_FEATURE_MAGIC_PAGE: c_int = 1;
// Magic page flags from host to guest

// MASn, ESR, PIR, and high SPRGs

// Magic page flags from guest to host

