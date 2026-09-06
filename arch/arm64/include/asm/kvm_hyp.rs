//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/kvm_hyp.h
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

//
// Unified accessors for registers that have a different encoding
// between VHE and non-VHE. They must be specified without their "ELx"
// encoding, but with the SYS_ prefix, as defined in asm/sysreg.h.
//

//
// Without an __arch_swab32(), we fall back to ___constant_swab32(), but the
// static inline can allow the compiler to out-of-line this. KVM always wants
// the macro version as it's always inlined.
//

extern "C" {
    pub fn __vgic_v2_perform_cpuif_access(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn __gic_v3_get_lr(lr: c_uint) -> u64;
}
extern "C" {
    pub fn __gic_v3_set_lr(val: u64, lr: c_int);
}
extern "C" {
    pub fn __vgic_v3_save_state(cpu_if: *mut vgic_v3_cpu_if);
}
extern "C" {
    pub fn __vgic_v3_restore_state(cpu_if: *mut vgic_v3_cpu_if);
}
extern "C" {
    pub fn __vgic_v3_activate_traps(cpu_if: *mut vgic_v3_cpu_if);
}
extern "C" {
    pub fn __vgic_v3_deactivate_traps(cpu_if: *mut vgic_v3_cpu_if);
}
extern "C" {
    pub fn __vgic_v3_save_aprs(cpu_if: *mut vgic_v3_cpu_if);
}
extern "C" {
    pub fn __vgic_v3_restore_vmcr_aprs(cpu_if: *mut vgic_v3_cpu_if);
}
extern "C" {
    pub fn __vgic_v3_perform_cpuif_access(vcpu: *mut kvm_vcpu) -> c_int;
}
// GICv5
extern "C" {
    pub fn __vgic_v5_save_apr(cpu_if: *mut vgic_v5_cpu_if);
}
extern "C" {
    pub fn __vgic_v5_restore_vmcr_apr(cpu_if: *mut vgic_v5_cpu_if);
}
// No hypercalls for the following
extern "C" {
    pub fn __vgic_v5_save_ppi_state(cpu_if: *mut vgic_v5_cpu_if);
}
extern "C" {
    pub fn __vgic_v5_restore_ppi_state(cpu_if: *mut vgic_v5_cpu_if);
}
extern "C" {
    pub fn __vgic_v5_save_state(cpu_if: *mut vgic_v5_cpu_if);
}
extern "C" {
    pub fn __vgic_v5_restore_state(cpu_if: *mut vgic_v5_cpu_if);
}

extern "C" {
    pub fn __timer_enable_traps(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn __timer_disable_traps(vcpu: *mut kvm_vcpu);
}

extern "C" {
    pub fn __sysreg_save_state_nvhe(ctxt: *mut kvm_cpu_context);
}
extern "C" {
    pub fn __sysreg_restore_state_nvhe(ctxt: *mut kvm_cpu_context);
}

extern "C" {
    pub fn __vcpu_load_switch_sysregs(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn __vcpu_put_switch_sysregs(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn sysreg_save_host_state_vhe(ctxt: *mut kvm_cpu_context);
}
extern "C" {
    pub fn sysreg_restore_host_state_vhe(ctxt: *mut kvm_cpu_context);
}
extern "C" {
    pub fn sysreg_save_guest_state_vhe(ctxt: *mut kvm_cpu_context);
}
extern "C" {
    pub fn sysreg_restore_guest_state_vhe(ctxt: *mut kvm_cpu_context);
}

extern "C" {
    pub fn __debug_switch_to_guest(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn __debug_switch_to_host(vcpu: *mut kvm_vcpu);
}

extern "C" {
    pub fn __debug_save_host_buffers_nvhe(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn __debug_restore_host_buffers_nvhe(vcpu: *mut kvm_vcpu);
}

extern "C" {
    pub fn __guest_enter(vcpu: *mut kvm_vcpu) -> u64;
}
extern "C" {
    pub fn kvm_host_psci_handler(host_ctxt: *mut kvm_cpu_context, func_id: u32) -> bool;
}

extern "C" {
    pub fn __pkvm_init(phys: phys_addr_t, size: c_ulong, per_cpu_base: *mut c_ulong, hyp_va_bits: u32) -> c_int;
}
extern "C" {
    pub fn __host_enter(host_ctxt: *mut kvm_cpu_context) -> void __noreturn;
}

extern "C" {
    pub fn kvm_nvhe_sym(_arg: id_aa64pfr0_el1_sys_val) -> u64;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: id_aa64pfr1_el1_sys_val) -> u64;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: id_aa64pfr2_el1_sys_val) -> u64;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: id_aa64isar0_el1_sys_val) -> u64;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: id_aa64isar1_el1_sys_val) -> u64;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: id_aa64isar2_el1_sys_val) -> u64;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: id_aa64mmfr0_el1_sys_val) -> u64;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: id_aa64mmfr1_el1_sys_val) -> u64;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: id_aa64mmfr2_el1_sys_val) -> u64;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: id_aa64smfr0_el1_sys_val) -> u64;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: __icache_flags) -> c_ulong;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: kvm_arm_vmid_bits) -> c_uint;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: kvm_host_sve_max_vl) -> c_uint;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: hyp_nr_cpus) -> c_ulong;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: hyp_gicv3_nr_lr) -> c_uint;
}
