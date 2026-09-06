//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/uapi/asm/kvm_para.h
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

// This CPUID returns the signature 'KVMKVMKVM' in ebx, ecx, and edx.  It
// should be used to determine that a VM is running under KVM.
//
pub const KVM_CPUID_SIGNATURE: c_uint = 0x40000000;

// This CPUID returns two feature bitmaps in eax, edx. Before enabling
// a particular paravirtualization, the appropriate feature bit should
// be checked in eax. The performance hint feature bit should be checked
// in edx.
//
pub const KVM_CPUID_FEATURES: c_uint = 0x40000001;
pub const KVM_FEATURE_CLOCKSOURCE: c_int = 0;
pub const KVM_FEATURE_NOP_IO_DELAY: c_int = 1;
pub const KVM_FEATURE_MMU_OP: c_int = 2;
// This indicates that the new set of kvmclock msrs
// are available. The use of 0x11 and 0x12 is deprecated
//
pub const KVM_FEATURE_CLOCKSOURCE2: c_int = 3;
pub const KVM_FEATURE_ASYNC_PF: c_int = 4;
pub const KVM_FEATURE_STEAL_TIME: c_int = 5;
pub const KVM_FEATURE_PV_EOI: c_int = 6;
pub const KVM_FEATURE_PV_UNHALT: c_int = 7;
pub const KVM_FEATURE_PV_TLB_FLUSH: c_int = 9;
pub const KVM_FEATURE_ASYNC_PF_VMEXIT: c_int = 10;
pub const KVM_FEATURE_PV_SEND_IPI: c_int = 11;
pub const KVM_FEATURE_POLL_CONTROL: c_int = 12;
pub const KVM_FEATURE_PV_SCHED_YIELD: c_int = 13;
pub const KVM_FEATURE_ASYNC_PF_INT: c_int = 14;
pub const KVM_FEATURE_MSI_EXT_DEST_ID: c_int = 15;
pub const KVM_FEATURE_HC_MAP_GPA_RANGE: c_int = 16;
pub const KVM_FEATURE_MIGRATION_CONTROL: c_int = 17;
pub const KVM_HINTS_REALTIME: c_int = 0;
// The last 8 bits are used to indicate how to interpret the flags field
// in pvclock structure. If no bits are set, all flags are ignored.
//
pub const KVM_FEATURE_CLOCKSOURCE_STABLE_BIT: c_int = 24;
pub const MSR_KVM_WALL_CLOCK: c_uint = 0x11;
pub const MSR_KVM_SYSTEM_TIME: c_uint = 0x12;
pub const KVM_MSR_ENABLED: c_int = 1;
// Custom MSRs falls in the range 0x4b564d00-0x4b564dff
pub const MSR_KVM_WALL_CLOCK_NEW: c_uint = 0x4b564d00;
pub const MSR_KVM_SYSTEM_TIME_NEW: c_uint = 0x4b564d01;
pub const MSR_KVM_ASYNC_PF_EN: c_uint = 0x4b564d02;
pub const MSR_KVM_STEAL_TIME: c_uint = 0x4b564d03;
pub const MSR_KVM_PV_EOI_EN: c_uint = 0x4b564d04;
pub const MSR_KVM_POLL_CONTROL: c_uint = 0x4b564d05;
pub const MSR_KVM_ASYNC_PF_INT: c_uint = 0x4b564d06;
pub const MSR_KVM_ASYNC_PF_ACK: c_uint = 0x4b564d07;
pub const MSR_KVM_MIGRATION_CONTROL: c_uint = 0x4b564d08;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_steal_time {
    pub steal: __u64,
    pub version: __u32,
    pub flags: __u32,
    pub preempted: __u8,
    pub u8_pad: [__u8; 3],
    pub pad: [__u32; 11],
}

pub const KVM_CLOCK_PAIRING_WALLCLOCK: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_clock_pairing {
    pub sec: __s64,
    pub nsec: __s64,
    pub tsc: __u64,
    pub flags: __u32,
    pub pad: [__u32; 9],
}

pub const KVM_STEAL_ALIGNMENT_BITS: c_int = 5;

pub const KVM_MAX_MMU_OP_BATCH: c_int = 32;

// MSR_KVM_ASYNC_PF_INT

// MSR_KVM_MIGRATION_CONTROL

// KVM_HC_MAP_GPA_RANGE
pub const KVM_MAP_GPA_RANGE_PAGE_SZ_4K: c_int = 0;

// Operations for KVM_HC_MMU_OP
pub const KVM_MMU_OP_WRITE_PTE: c_int = 1;
pub const KVM_MMU_OP_FLUSH_TLB: c_int = 2;
pub const KVM_MMU_OP_RELEASE_PT: c_int = 3;
// Payload for KVM_HC_MMU_OP
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_mmu_op_header {
    pub op: __u32,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_mmu_op_write_pte {
    pub header: kvm_mmu_op_header,
    pub pte_phys: __u64,
    pub pte_val: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_mmu_op_flush_tlb {
    pub header: kvm_mmu_op_header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_mmu_op_release_pt {
    pub header: kvm_mmu_op_header,
    pub pt_phys: __u64,
}

pub const KVM_PV_REASON_PAGE_NOT_PRESENT: c_int = 1;
pub const KVM_PV_REASON_PAGE_READY: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_pv_apf_data {
// Used for 'page not present' events delivered via #PF
    pub flags: __u32,
// Used for 'page ready' events delivered via interrupt notification
    pub token: __u32,
    pub pad: [__u8; 56],
}

pub const KVM_PV_EOI_BIT: c_int = 0;

pub const KVM_PV_EOI_DISABLED: c_uint = 0x0;
