//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/kvm_host.h
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
// Copyright (C) 2019 Western Digital Corporation or its affiliates.
//
// Authors:
// Anup Patel <anup.patel@wdc.com>
//

pub const KVM_MAX_VCPUS: c_int = 1024;
pub const KVM_HALT_POLL_NS_DEFAULT: c_int = 500000;
pub const KVM_VCPU_MAX_FEATURES: c_int = 0;
pub const KVM_IRQCHIP_NUM_PINS: c_int = 1024;

// Macro flag: #define KVM_HAVE_MMU_RWLOCK

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vm_stat {
    pub generic: kvm_vm_stat_generic,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_stat {
    pub generic: kvm_vcpu_stat_generic,
    pub ecall_exit_stat: u64,
    pub wfi_exit_stat: u64,
    pub wrs_exit_stat: u64,
    pub mmio_exit_user: u64,
    pub mmio_exit_kernel: u64,
    pub csr_exit_user: u64,
    pub csr_exit_kernel: u64,
    pub signal_exits: u64,
    pub exits: u64,
    pub instr_illegal_exits: u64,
    pub load_misaligned_exits: u64,
    pub store_misaligned_exits: u64,
    pub load_access_exits: u64,
    pub store_access_exits: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_arch_memory_slot {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_arch {
// G-stage vmid
    pub vmid: kvm_vmid,
// G-stage page table
    pub pgd: *mut pgd_t,
    pub pgd_phys: phys_addr_t,
    pub pgd_levels: c_ulong,
    pub pgd_split_page_cache: kvm_mmu_memory_cache,
// Guest Timer
    pub timer: kvm_guest_timer,
// AIA Guest/VM context
    pub aia: kvm_aia,
// KVM_CAP_RISCV_MP_STATE_RESET
    pub mp_state_reset: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_cpu_trap {
    pub sepc: c_ulong,
    pub scause: c_ulong,
    pub stval: c_ulong,
    pub htval: c_ulong,
    pub htinst: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_cpu_context {
    pub zero: c_ulong,
    pub ra: c_ulong,
    pub sp: c_ulong,
    pub gp: c_ulong,
    pub tp: c_ulong,
    pub t0: c_ulong,
    pub t1: c_ulong,
    pub t2: c_ulong,
    pub s0: c_ulong,
    pub s1: c_ulong,
    pub a0: c_ulong,
    pub a1: c_ulong,
    pub a2: c_ulong,
    pub a3: c_ulong,
    pub a4: c_ulong,
    pub a5: c_ulong,
    pub a6: c_ulong,
    pub a7: c_ulong,
    pub s2: c_ulong,
    pub s3: c_ulong,
    pub s4: c_ulong,
    pub s5: c_ulong,
    pub s6: c_ulong,
    pub s7: c_ulong,
    pub s8: c_ulong,
    pub s9: c_ulong,
    pub s10: c_ulong,
    pub s11: c_ulong,
    pub t3: c_ulong,
    pub t4: c_ulong,
    pub t5: c_ulong,
    pub t6: c_ulong,
    pub sepc: c_ulong,
    pub sstatus: c_ulong,
    pub hstatus: c_ulong,
    pub fp: __riscv_fp_state,
    pub vector: __riscv_v_ext_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_csr {
    pub vsstatus: c_ulong,
    pub vsie: c_ulong,
    pub vstvec: c_ulong,
    pub vsscratch: c_ulong,
    pub vsepc: c_ulong,
    pub vscause: c_ulong,
    pub vstval: c_ulong,
    pub hvip: c_ulong,
    pub vsatp: c_ulong,
    pub scounteren: c_ulong,
    pub senvcfg: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_smstateen_csr {
    pub sstateen0: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_zicfiss_csr {
    pub ssp: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_reset_state {
    pub lock: spinlock_t,
    pub pc: c_ulong,
    pub a1: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_arch {
// VCPU ran at least once
    pub ran_atleast_once: bool,
// Last Host CPU on which Guest VCPU exited
    pub last_exit_cpu: c_int,
// ISA feature bits (similar to MISA)
    pub RISCV_ISA_EXT_MAX): DECLARE_BITMAP(isa,,
// Vendor, Arch, and Implementation details
    pub mvendorid: c_ulong,
    pub marchid: c_ulong,
    pub mimpid: c_ulong,
// SSCRATCH, STVEC, and SCOUNTEREN of Host
    pub host_sscratch: c_ulong,
    pub host_stvec: c_ulong,
    pub host_scounteren: c_ulong,
    pub host_senvcfg: c_ulong,
    pub host_sstateen0: c_ulong,
// CPU context of Host
    pub host_context: kvm_cpu_context,
// CPU context of Guest VCPU
    pub guest_context: kvm_cpu_context,
// CPU CSR context of Guest VCPU
    pub guest_csr: kvm_vcpu_csr,
// CPU Smstateen CSR context of Guest VCPU
    pub smstateen_csr: kvm_vcpu_smstateen_csr,
// CPU Zicfiss CSR context of Guest VCPU
    pub zicfiss_csr: kvm_vcpu_zicfiss_csr,
// CPU reset state of Guest VCPU
    pub reset_state: kvm_vcpu_reset_state,
//
// VCPU interrupts
//
// The irqs_pending bitmap represents pending interrupts whereas
// irqs_pending_mask represents bits changed in irqs_pending. Updates
// to these bitmaps are serialized so vcpu interrupt sync/flush cannot
// drop a newly injected interrupt while syncing guest-visible HVIP.
//
pub const KVM_RISCV_VCPU_NR_IRQS: c_int = 64;
    pub irqs_pending_lock: raw_spinlock_t,
    pub KVM_RISCV_VCPU_NR_IRQS): DECLARE_BITMAP(irqs_pending,,
    pub KVM_RISCV_VCPU_NR_IRQS): DECLARE_BITMAP(irqs_pending_mask,,
// VCPU Timer
    pub timer: kvm_vcpu_timer,
// HFENCE request queue
    pub hfence_lock: spinlock_t,
    pub hfence_head: c_ulong,
    pub hfence_tail: c_ulong,
    pub hfence_queue: [kvm_riscv_hfence; KVM_RISCV_VCPU_MAX_HFENCE],
// MMIO instruction details
    pub mmio_decode: kvm_mmio_decode,
// CSR instruction details
    pub csr_decode: kvm_csr_decode,
// SBI context
    pub sbi_context: kvm_vcpu_sbi_context,
// AIA VCPU context
    pub aia_context: kvm_vcpu_aia,
// Cache pages needed to program page tables with spinlock held
    pub mmu_page_cache: kvm_mmu_memory_cache,
// VCPU power state
    pub mp_state: kvm_mp_state,
    pub mp_state_lock: spinlock_t,
// Don't run the VCPU (blocked)
    pub pause: bool,
// Performance monitoring context
    pub pmu_context: kvm_pmu,
// Firmware feature SBI extension context
    pub fwft_context: kvm_sbi_fwft,
// 'static' configurations which are set only once
    pub cfg: kvm_vcpu_config,
// Indicates modified guest CSRs
    pub csr_dirty: bool,
// SBI steal-time accounting
    pub shmem: gpa_t,
    pub last_steal: u64,
    pub sta: },
}

//
// Returns true if a Performance Monitoring Interrupt (PMI), a.k.a. perf event,
// arrived in guest context.  For riscv, any event that arrives while a vCPU is
// loaded is considered to be "in guest".
//
extern "C" {
    pub fn kvm_riscv_clear_former_vcpu();
}
extern "C" {
    pub fn kvm_riscv_setup_default_irq_routing(kvm: *mut kvm, lines: u32) -> c_int;
}
extern "C" {
    pub fn __kvm_riscv_unpriv_trap();
}
extern "C" {
    pub fn __kvm_riscv_switch_to(vcpu_arch: *mut kvm_vcpu_arch);
}
extern "C" {
    pub fn kvm_riscv_vcpu_setup_isa(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_vcpu_num_regs(vcpu: *mut kvm_vcpu) -> c_ulong;
}
extern "C" {
    pub fn kvm_riscv_vcpu_set_interrupt(vcpu: *mut kvm_vcpu, irq: c_uint) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_vcpu_unset_interrupt(vcpu: *mut kvm_vcpu, irq: c_uint) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_vcpu_flush_interrupts(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_vcpu_sync_interrupts(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_vcpu_has_interrupts(vcpu: *mut kvm_vcpu, mask: u64) -> bool;
}
extern "C" {
    pub fn __kvm_riscv_vcpu_power_off(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_vcpu_power_off(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn __kvm_riscv_vcpu_power_on(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_vcpu_power_on(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_vcpu_stopped(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn kvm_riscv_vcpu_record_steal_time(vcpu: *mut kvm_vcpu);
}
// Flags representing implementation specific details
