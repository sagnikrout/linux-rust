//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/kvm_host.h
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
// Copyright (C) 2020-2023 Loongson Technology Corporation Limited
//

// Loongarch KVM register ids

pub const KVM_MAX_VCPUS: c_int = 256;
pub const KVM_MAX_CPUCFG_REGS: c_int = 21;
pub const KVM_HALT_POLL_NS_DEFAULT: c_int = 500000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vm_stat {
    pub generic: kvm_vm_stat_generic,
    pub pages: u64,
    pub hugepages: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_stat {
    pub generic: kvm_vcpu_stat_generic,
    pub int_exits: u64,
    pub idle_exits: u64,
    pub cpucfg_exits: u64,
    pub signal_exits: u64,
    pub hypercall_exits: u64,
    pub ipi_read_exits: u64,
    pub ipi_write_exits: u64,
    pub eiointc_read_exits: u64,
    pub eiointc_write_exits: u64,
    pub pch_pic_read_exits: u64,
    pub pch_pic_write_exits: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_arch_memory_slot {
    pub flags: c_ulong,
}

pub const HOST_MAX_PMNUM: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_context {
    pub vpid_cache: c_ulong,
    pub last_vcpu: *mut kvm_vcpu,
// Host PMU CSR
    pub perf_ctrl: [u64; HOST_MAX_PMNUM],
    pub perf_cntr: [u64; HOST_MAX_PMNUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_world_switch {
    pub (*exc_entry)(void): *mut c_int,
    pub vcpu): *mut *mut *mut int (enter_guest)(struct kvm_run run, struct kvm_vcpu,
}

pub const MAX_PGTABLE_LEVELS: c_int = 4;
//
// Physical CPUID is used for interrupt routing, there are different
// definitions about physical cpuid on different hardwares.
//
// For LOONGARCH_CSR_CPUID register, max CPUID size if 512
// For IPI hardware, max destination CPUID size 1024
// For eiointc interrupt controller, max destination CPUID size is 256
// For msgint interrupt controller, max supported CPUID size is 65536
//
// Currently max CPUID is defined as 256 for KVM hypervisor, in future
// it will be expanded to 4096, including 16 packages at most. And every
// package supports at most 256 vcpus
//
pub const KVM_MAX_PHYID: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_phyid_info {
    pub vcpu: *mut kvm_vcpu,
    pub enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_phyid_map {
    pub max_phyid: c_int,
    pub phys_map: [kvm_phyid_info; KVM_MAX_PHYID],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_arch {
// Guest physical mm
    pub pgd: *mut kvm_pte_t,
    pub gpa_size: c_ulong,
    pub invalid_ptes: [c_ulong; MAX_PGTABLE_LEVELS],
    pub pte_shifts: [c_uint; MAX_PGTABLE_LEVELS],
    pub root_level: c_uint,
    pub phyid_map_lock: spinlock_t,
    pub pv_setting_lock: spinlock_t,
    pub phyid_map: *mut kvm_phyid_map,
// Enabled PV features
    pub pv_features: c_ulong,
// Supported KVM features
    pub kvm_features: c_ulong,
    pub time_offset: i64,
    pub vmcs: *mut kvm_context __percpu,
    pub ipi: *mut loongarch_ipi,
    pub dmsintc: *mut loongarch_dmsintc,
    pub eiointc: *mut loongarch_eiointc,
    pub pch_pic: *mut loongarch_pch_pic,
}

pub const CSR_MAX_NUMS: c_uint = 0x800;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongarch_csrs {
    pub csrs: [c_ulong; CSR_MAX_NUMS],
}

// Resume Flags
pub const RESUME_HOST: c_int = 0;
pub const RESUME_GUEST: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum emulation_result {
    EMULATE_DONE,		/* no further processing */
    EMULATE_DO_MMIO,	/* kvm_run filled with MMIO request */
    EMULATE_DO_IOCSR,	/* handle IOCSR request */
    EMULATE_FAIL,		/* can't emulate this instruction */
    EMULATE_EXCEPT,		/* A guest exception has been generated */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_arch {
//
// Switch pointer-to-function type to unsigned long
// for loading the value into register directly.
//
    pub host_eentry: c_ulong,
    pub guest_eentry: c_ulong,
// Pointers stored here for easy accessing from assembly code
    pub vcpu): *mut *mut *mut int (handle_exit)(struct kvm_run run, struct kvm_vcpu,
// GPA (=HVA) of PGD for secondary mmu
    pub kvm_pgd: c_ulong,
// Host registers preserved across guest mode execution
    pub host_sp: c_ulong,
    pub host_tp: c_ulong,
    pub host_pgd: c_ulong,
// Host CSRs are used when handling exits from guest
    pub badi: c_ulong,
    pub badv: c_ulong,
    pub host_ecfg: c_ulong,
    pub host_estat: c_ulong,
    pub host_percpu: c_ulong,
// GPRs
    pub gprs: [c_ulong; 32],
    pub pc: c_ulong,
// Which auxiliary state is loaded (KVM_LARCH_*)
    pub aux_inuse: c_uint,
// FPU state
    pub FPU_ALIGN: loongarch_fpu fpu,
    pub lbt: loongarch_lbt,
// CSR state
    pub csr: *mut loongarch_csrs,
// Guest max PMU CSR id
    pub max_pmu_csrid: c_int,
// GPR used as IO source/target
    pub io_gpr: u32,
// KVM register to control count timer
    pub count_ctl: u32,
    pub swtimer: hrtimer,
// Bitmask of intr that are pending
    pub irq_pending: c_ulong,
// Bitmask of pending intr to be cleared
    pub irq_clear: c_ulong,
// Bitmask of exceptions that are pending
    pub exception_pending: c_ulong,
    pub esubcode: c_uint,
// Cache for pages needed inside spinlock regions
    pub mmu_page_cache: kvm_mmu_memory_cache,
// vcpu's vpid
    pub vpid: u64,
    pub flush_gpa: gpa_t,
// Frequency of stable timer in Hz
    pub timer_mhz: u64,
    pub expire: ktime_t,
// Last CPU the vCPU state was loaded on
    pub last_sched_cpu: c_int,
// mp state
    pub mp_state: kvm_mp_state,
// ipi state
    pub ipi_state: ipi_state,
    pub dmsintc_state: dmsintc_state,
// cpucfg
    pub cpucfg: [u32; KVM_MAX_CPUCFG_REGS],
// paravirt steal time
    pub guest_addr: u64,
    pub last_steal: u64,
    pub cache: gfn_to_hva_cache,
    pub preempted: u8,
    pub st: },
}

// Check whether KVM support this feature (VMM may disable it)
extern "C" {
    pub fn kvm_arch_pmi_in_guest(vcpu: *mut kvm_vcpu) -> bool;
}
// Debug: dump vcpu state
extern "C" {
    pub fn kvm_arch_vcpu_dump_regs(vcpu: *mut kvm_vcpu) -> c_int;
}
// MMU handling
extern "C" {
    pub fn kvm_flush_tlb_all();
}
extern "C" {
    pub fn kvm_flush_tlb_gpa(vcpu: *mut kvm_vcpu, gpa: c_ulong);
}
extern "C" {
    pub fn kvm_handle_mm_fault(vcpu: *mut kvm_vcpu, badv: c_ulong, write: bool, ecode: c_int) -> c_int;
}
extern "C" {
    pub fn kvm_unmap_hva_range(kvm: *mut kvm, start: c_ulong, end: c_ulong, blockable: bool) -> c_int;
}
extern "C" {
    pub fn kvm_age_hva(kvm: *mut kvm, start: c_ulong, end: c_ulong) -> c_int;
}
extern "C" {
    pub fn kvm_test_age_hva(kvm: *mut kvm, hva: c_ulong) -> c_int;
}
//
// kvm_is_ifetch_fault() - Find whether a TLBL exception is due to ifetch fault.
// @vcpu:	Virtual CPU.
//
// Returns:	Whether the TLBL exception was likely due to an instruction
// fetch fault rather than a data load fault.
//
// Misc
extern "C" {
    pub fn kvm_check_vpid(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_swtimer_wakeup(timer: *mut hrtimer) -> hrtimer_restart;
}
extern "C" {
    pub fn kvm_init_vmcs(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_exc_entry();
}
extern "C" {
    pub fn kvm_enter_guest(run: *mut kvm_run, vcpu: *mut kvm_vcpu) -> c_int;
}

extern "C" {
    pub fn get_gcsr_flag(csr: c_int) -> c_int;
}
extern "C" {
    pub fn set_hw_gcsr(csr_id: c_int, val: c_ulong);
}
