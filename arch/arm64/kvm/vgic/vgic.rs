//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/kvm/vgic/vgic.h
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
// Copyright (C) 2015, 2016 ARM Ltd.
//

pub const PRODUCT_ID_KVM: c_uint = 0x4b	/* ASCII code K */;
pub const IMPLEMENTER_ARM: c_uint = 0x43b;

pub const INTERRUPT_ID_BITS_SPIS: c_int = 10;
pub const INTERRUPT_ID_BITS_ITS: c_int = 16;

pub const VGIC_PRI_BITS: c_int = 5;

pub const VGIC_AFFINITY_0_SHIFT: c_int = 0;

pub const VGIC_AFFINITY_1_SHIFT: c_int = 8;

pub const VGIC_AFFINITY_2_SHIFT: c_int = 16;

pub const VGIC_AFFINITY_3_SHIFT: c_int = 24;

//
// The Userspace encodes the affinity differently from the MPIDR,
// Below macro converts vgic userspace format to MPIDR reg format.
//

//
// As per Documentation/virt/kvm/devices/arm-vgic-v3.rst,
// below macros are defined for CPUREG encoding.
//
pub const KVM_REG_ARM_VGIC_SYSREG_OP0_MASK: c_uint = 0x000000000000c000;
pub const KVM_REG_ARM_VGIC_SYSREG_OP0_SHIFT: c_int = 14;
pub const KVM_REG_ARM_VGIC_SYSREG_OP1_MASK: c_uint = 0x0000000000003800;
pub const KVM_REG_ARM_VGIC_SYSREG_OP1_SHIFT: c_int = 11;
pub const KVM_REG_ARM_VGIC_SYSREG_CRN_MASK: c_uint = 0x0000000000000780;
pub const KVM_REG_ARM_VGIC_SYSREG_CRN_SHIFT: c_int = 7;
pub const KVM_REG_ARM_VGIC_SYSREG_CRM_MASK: c_uint = 0x0000000000000078;
pub const KVM_REG_ARM_VGIC_SYSREG_CRM_SHIFT: c_int = 3;
pub const KVM_REG_ARM_VGIC_SYSREG_OP2_MASK: c_uint = 0x0000000000000007;
pub const KVM_REG_ARM_VGIC_SYSREG_OP2_SHIFT: c_int = 0;

// All non-RES0 bits are in the bottom 32bits
//
// As per Documentation/virt/kvm/devices/arm-vgic-its.rst,
// below macros are defined for ITS table entry encoding.
//
pub const KVM_ITS_CTE_VALID_SHIFT: c_int = 63;

pub const KVM_ITS_CTE_RDBASE_SHIFT: c_int = 16;

pub const KVM_ITS_ITE_NEXT_SHIFT: c_int = 48;
pub const KVM_ITS_ITE_PINTID_SHIFT: c_int = 16;

pub const KVM_ITS_DTE_VALID_SHIFT: c_int = 63;

pub const KVM_ITS_DTE_NEXT_SHIFT: c_int = 49;

pub const KVM_ITS_DTE_ITTADDR_SHIFT: c_int = 5;

// we only support 64 kB translation table page size

pub const KVM_VGIC_V3_RDIST_FLAGS_SHIFT: c_int = 12;

pub const KVM_VGIC_V3_RDIST_COUNT_SHIFT: c_int = 52;

// Macro flag: #define DEBUG_SPINLOCK_BUG_ON(p)

// Requires the irq_lock to be held by the caller.
// Account for the active state as an interrupt
// All the traps are in the bottom 16bits
//
// This struct provides an intermediate representation of the fields contained
// in the GICH_VMCR and ICH_VMCR registers, such that code exporting the GIC
// state to userspace can generate either GICv2 or GICv3 CPU interface
// registers regardless of the hardware backed GIC used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vgic_vmcr {
    pub /: *mut *mut u32 en; / GICv5-specific,
    pub grpen0: u32,
    pub grpen1: u32,
    pub ackctl: u32,
    pub fiqen: u32,
    pub cbpr: u32,
    pub eoim: u32,
    pub abpr: u32,
    pub bpr: u32,
    pub and: *mut *mut u32 pmr; / Priority mask field in the GICC_PMR,
// ICC_PMR_EL1 priority field format
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vgic_reg_attr {
    pub vcpu: *mut kvm_vcpu,
    pub addr: gpa_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct its_device {
    pub dev_list: list_head,
// the head for the list of ITTEs
    pub itt_head: list_head,
    pub num_eventid_bits: u32,
    pub itt_addr: gpa_t,
    pub device_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct its_collection {
    pub coll_list: list_head,
    pub collection_id: u32,
    pub target_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct its_ite {
    pub ite_list: list_head,
    pub irq: *mut vgic_irq,
    pub collection: *mut its_collection,
    pub event_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_list_summary {
    pub /: *mut *mut unsigned int nr_pend; / purely pending, not active,
    pub /: *mut *mut unsigned int nr_act; / active, or active+pending,
    pub /: *mut *mut unsigned int nr_sgi; / any SGI,
}

extern "C" {
    pub fn vgic_put_irq(kvm: *mut kvm, irq: *mut vgic_irq);
}
extern "C" {
    pub fn vgic_get_phys_line_level(irq: *mut vgic_irq) -> bool;
}
extern "C" {
    pub fn vgic_irq_set_phys_pending(irq: *mut vgic_irq, pending: bool);
}
extern "C" {
    pub fn vgic_irq_set_phys_active(irq: *mut vgic_irq, active: bool);
}
extern "C" {
    pub fn vgic_kick_vcpus(kvm: *mut kvm);
}
extern "C" {
    pub fn vgic_v2_fold_lr_state(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vgic_v2_populate_lr(vcpu: *mut kvm_vcpu, irq: *mut vgic_irq, lr: c_int);
}
extern "C" {
    pub fn vgic_v2_deactivate(vcpu: *mut kvm_vcpu, val: u32);
}
extern "C" {
    pub fn vgic_v2_clear_lr(vcpu: *mut kvm_vcpu, lr: c_int);
}
extern "C" {
    pub fn vgic_v2_configure_hcr(vcpu: *mut kvm_vcpu, als: *mut ap_list_summary);
}
extern "C" {
    pub fn vgic_v2_has_attr_regs(dev: *mut kvm_device, attr: *mut kvm_device_attr) -> c_int;
}
extern "C" {
    pub fn vgic_v2_set_vmcr(vcpu: *mut kvm_vcpu, vmcr: *mut vgic_vmcr);
}
extern "C" {
    pub fn vgic_v2_get_vmcr(vcpu: *mut kvm_vcpu, vmcr: *mut vgic_vmcr);
}
extern "C" {
    pub fn vgic_v2_reset(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vgic_v2_probe(info: *const gic_kvm_info) -> c_int;
}
extern "C" {
    pub fn vgic_v2_map_resources(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn vgic_v2_init_lrs();
}
extern "C" {
    pub fn vgic_v2_load(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vgic_v2_put(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vgic_v2_save_state(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vgic_v2_restore_state(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn refcount_inc_not_zero(_arg: &irq->refcount) -> return;
}
extern "C" {
    pub fn vgic_v3_fold_lr_state(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vgic_v3_populate_lr(vcpu: *mut kvm_vcpu, irq: *mut vgic_irq, lr: c_int);
}
extern "C" {
    pub fn vgic_v3_clear_lr(vcpu: *mut kvm_vcpu, lr: c_int);
}
extern "C" {
    pub fn vgic_v3_deactivate(vcpu: *mut kvm_vcpu, val: u64);
}
extern "C" {
    pub fn vgic_v3_configure_hcr(vcpu: *mut kvm_vcpu, als: *mut ap_list_summary);
}
extern "C" {
    pub fn vgic_v3_set_vmcr(vcpu: *mut kvm_vcpu, vmcr: *mut vgic_vmcr);
}
extern "C" {
    pub fn vgic_v3_get_vmcr(vcpu: *mut kvm_vcpu, vmcr: *mut vgic_vmcr);
}
extern "C" {
    pub fn vgic_v3_reset(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vgic_v3_enable_cpuif_traps();
}
extern "C" {
    pub fn vgic_v3_probe(info: *const gic_kvm_info) -> c_int;
}
extern "C" {
    pub fn vgic_v3_map_resources(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn vgic_v3_lpi_sync_pending_status(kvm: *mut kvm, irq: *mut vgic_irq) -> c_int;
}
extern "C" {
    pub fn vgic_v3_save_pending_tables(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn vgic_v3_set_redist_base(kvm: *mut kvm, index: u32, addr: u64, count: u32) -> c_int;
}
extern "C" {
    pub fn vgic_register_redist_iodev(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn vgic_unregister_redist_iodev(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vgic_v3_check_base(kvm: *mut kvm) -> bool;
}
extern "C" {
    pub fn vgic_v3_load(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vgic_v3_put(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vgic_has_its(kvm: *mut kvm) -> bool;
}
extern "C" {
    pub fn kvm_vgic_register_its_device() -> c_int;
}
extern "C" {
    pub fn vgic_enable_lpis(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vgic_flush_pending_lpis(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vgic_its_inject_msi(kvm: *mut kvm, msi: *mut kvm_msi) -> c_int;
}
extern "C" {
    pub fn vgic_v3_has_attr_regs(dev: *mut kvm_device, attr: *mut kvm_device_attr) -> c_int;
}
extern "C" {
    pub fn vgic_v3_has_cpu_sysregs_attr(vcpu: *mut kvm_vcpu, attr: *mut kvm_device_attr) -> c_int;
}
extern "C" {
    pub fn kvm_register_vgic_device(type: c_ulong) -> c_int;
}
extern "C" {
    pub fn vgic_set_vmcr(vcpu: *mut kvm_vcpu, vmcr: *mut vgic_vmcr);
}
extern "C" {
    pub fn vgic_get_vmcr(vcpu: *mut kvm_vcpu, vmcr: *mut vgic_vmcr);
}
extern "C" {
    pub fn vgic_lazy_init(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn vgic_init(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn vgic_debug_init(kvm: *mut kvm);
}
extern "C" {
    pub fn vgic_debug_destroy(kvm: *mut kvm);
}
extern "C" {
    pub fn vgic_v5_probe(info: *const gic_kvm_info) -> c_int;
}
extern "C" {
    pub fn vgic_v5_reset(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vgic_v5_init(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn vgic_v5_map_resources(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn vgic_v5_set_ppi_ops(vcpu: *mut kvm_vcpu, vintid: u32);
}
extern "C" {
    pub fn vgic_v5_has_pending_ppi(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn vgic_v5_flush_ppi_state(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vgic_v5_fold_ppi_state(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vgic_v5_load(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vgic_v5_put(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vgic_v5_set_vmcr(vcpu: *mut kvm_vcpu, vmcr: *mut vgic_vmcr);
}
extern "C" {
    pub fn vgic_v5_get_vmcr(vcpu: *mut kvm_vcpu, vmcr: *mut vgic_vmcr);
}
extern "C" {
    pub fn vgic_v5_restore_state(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vgic_v5_save_state(vcpu: *mut kvm_vcpu);
}

//
// num_pri_bits are initialized with HW supported values.
// We can rely safely on num_pri_bits even if VM has not
// restored ICC_CTLR_EL1 before restoring APnR registers.
//
extern "C" {
    pub fn vgic_v3_free_redist_region(kvm: *mut kvm, rdreg: *mut vgic_redist_region);
}
extern "C" {
    pub fn vgic_v3_rdist_overlap(kvm: *mut kvm, base: gpa_t, size: usize) -> bool;
}
extern "C" {
    pub fn vgic_lpis_enabled(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn vgic_its_inject_cached_translation(kvm: *mut kvm, msi: *mut kvm_msi) -> c_int;
}
extern "C" {
    pub fn vgic_its_invalidate_all_caches(kvm: *mut kvm);
}
// GICv4.1 MMIO interface
extern "C" {
    pub fn vgic_its_inv_lpi(kvm: *mut kvm, irq: *mut vgic_irq) -> c_int;
}
extern "C" {
    pub fn vgic_its_invall(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn vgic_v4_init(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn vgic_v4_teardown(kvm: *mut kvm);
}
extern "C" {
    pub fn vgic_v4_configure_vsgis(kvm: *mut kvm);
}
extern "C" {
    pub fn vgic_v4_get_vlpi_state(irq: *mut vgic_irq, val: *mut bool);
}
extern "C" {
    pub fn vgic_v4_request_vpe_irq(vcpu: *mut kvm_vcpu, irq: c_int) -> c_int;
}
extern "C" {
    pub fn vcpu_set_ich_hcr(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_has_feat(_arg: kvm, _arg: ID_AA64PFR0_EL1, _arg: GIC, _arg: IMP) -> return;
}
extern "C" {
    pub fn kvm_has_feat(_arg: kvm, _arg: ID_AA64PFR2_EL1, _arg: GCIE, _arg: IMP) -> return;
}
extern "C" {
    pub fn vgic_v3_flush_nested(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vgic_v3_sync_nested(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vgic_v3_load_nested(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vgic_v3_put_nested(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vgic_v3_handle_nested_maint_irq(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vgic_v3_nested_update_mi(vcpu: *mut kvm_vcpu);
}
//
// Either the host is a native GICv3, or it is GICv5 with
// FEAT_GCIE_LEGACY.
//
extern "C" {
    pub fn system_supports_direct_sgis() -> bool;
}
extern "C" {
    pub fn vgic_supports_direct_msis(kvm: *mut kvm) -> bool;
}
extern "C" {
    pub fn vgic_supports_direct_sgis(kvm: *mut kvm) -> bool;
}
// GICv5 always supports direct IRQs
extern "C" {
    pub fn vgic_supports_direct_msis(vgic_supports_direct_sgis(kvm: kvm) ||) -> return;
}
extern "C" {
    pub fn vgic_its_debug_init(dev: *mut kvm_device) -> c_int;
}
extern "C" {
    pub fn vgic_its_debug_destroy(dev: *mut kvm_device);
}
