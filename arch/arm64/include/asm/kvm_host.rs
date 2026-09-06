//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/kvm_host.h
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
// Derived from arch/arm/include/asm/kvm_host.h:
// Copyright (C) 2012 - Virtual Open Systems and Columbia University
// Author: Christoffer Dall <c.dall@virtualopensystems.com>
//

pub const KVM_HALT_POLL_NS_DEFAULT: c_int = 500000;

pub const KVM_VCPU_MAX_FEATURES: c_int = 10;

// Macro flag: #define KVM_HAVE_MMU_RWLOCK
//
// Mode of operation configurable with kvm-arm.mode early param.
// See Documentation/admin-guide/kernel-parameters.txt for more information.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_mode {
    KVM_MODE_DEFAULT,
    KVM_MODE_PROTECTED,
    KVM_MODE_NV,
    KVM_MODE_NONE,
}

extern "C" {
    pub fn kvm_get_mode() -> kvm_mode;
}

extern "C" {
    pub fn kvm_arm_init_sve() -> int __init;
}
extern "C" {
    pub fn kvm_target_cpu() -> u32 __attribute_const__;
}
extern "C" {
    pub fn kvm_reset_vcpu(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_arm_vcpu_destroy(vcpu: *mut kvm_vcpu);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_hyp_memcache {
    pub head: phys_addr_t,
    pub nr_pages: c_ulong,
    pub /: *mut *mut *mut pkvm_mapping mapping; / only used from EL1,

    pub flags: c_ulong,
}

// p = mc->head;
extern "C" {
    pub fn free_hyp_memcache(mc: *mut kvm_hyp_memcache);
}
extern "C" {
    pub fn topup_hyp_memcache(mc: *mut kvm_hyp_memcache, min_pages: c_ulong) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vmid {
    pub id: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s2_mmu {
    pub vmid: kvm_vmid,
//
// stage2 entry level table
//
// Two kvm_s2_mmu structures in the same VM can point to the same
// pgd here.  This happens when running a guest using a
// translation regime that isn't affected by its own stage-2
// translation, such as a non-VHE hypervisor running at vEL2, or
// for vEL1/EL0 with vHCR_EL2.VM == 0.  In that case, we use the
// canonical stage-2 page tables.
//
    pub pgd_phys: phys_addr_t,
    pub pgt: *mut kvm_pgtable,
//
// VTCR value used on the host. For a non-NV guest (or a NV
// guest that runs in a context where its own S2 doesn't
// apply), its T0SZ value reflects that of the IPA size.
//
// For a shadow S2 MMU, T0SZ reflects the PARange exposed to
// the guest.
//
    pub vtcr: u64,
// The last vcpu id that ran on each physical CPU
    pub last_vcpu_ran: *mut int __percpu,
pub const KVM_ARM_EAGER_SPLIT_CHUNK_SIZE_DEFAULT: c_int = 0;
//
// Memory cache used to split
// KVM_CAP_ARM_EAGER_SPLIT_CHUNK_SIZE worth of huge pages. It
// is used to allocate stage2 page tables while splitting huge
// pages. The choice of KVM_CAP_ARM_EAGER_SPLIT_CHUNK_SIZE
// influences both the capacity of the split page cache, and
// how often KVM reschedules. Be wary of raising CHUNK_SIZE
// too high.
//
// Protected by kvm->slots_lock.
//
    pub split_page_cache: kvm_mmu_memory_cache,
    pub split_page_chunk_size: u64,
    pub arch: *mut kvm_arch,
//
// For a shadow stage-2 MMU, the virtual vttbr used by the
// host to parse the guest S2.
// This either contains:
// - the virtual VTTBR programmed by the guest hypervisor with
// CnP cleared
// - The value 1 (VMID=0, BADDR=0, CnP=1) if invalid
//
// We also cache the full VTCR which gets used for TLB invalidation,
// taking the ARM ARM's "Any of the bits in VTCR_EL2 are permitted
// to be cached in a TLB" to the letter.
//
    pub tlb_vttbr: u64,
    pub tlb_vtcr: u64,
//
// true when this represents a nested context where virtual
// HCR_EL2.VM == 1
//
    pub nested_stage2_enabled: bool,

    pub shadow_pt_debugfs_dentry: *mut dentry,

//
// true when this MMU needs to be unmapped before being used for a new
// purpose.
//
    pub pending_unmap: bool,
//
// 0: Nobody is currently using this, check vttbr for validity
// >0: Somebody is actively using this.
//
    pub refcnt: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_arch_memory_slot {
}

//
// struct kvm_smccc_features: Descriptor of the hypercall services exposed to the guests
//
// @std_bmap: Bitmap of standard secure service calls
// @std_hyp_bmap: Bitmap of standard hypervisor service calls
// @vendor_hyp_bmap: Bitmap of vendor specific hypervisor service calls
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_smccc_features {
    pub std_bmap: c_ulong,
    pub std_hyp_bmap: c_ulong,
    pub /: *mut *mut unsigned long vendor_hyp_bmap; / Function numbers 0-63,
    pub /: *mut *mut unsigned long vendor_hyp_bmap_2; / Function numbers 64-127,
}

pub type pkvm_handle_t = u16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_protected_vm {
    pub handle: pkvm_handle_t,
    pub teardown_mc: kvm_hyp_memcache,
    pub stage2_teardown_mc: kvm_hyp_memcache,
    pub is_protected: bool,
    pub is_created: bool,
//
// True when the guest is being torn down. When in this state, the
// guest's vCPUs can't be loaded anymore, but its pages can be
// reclaimed by the host.
//
    pub is_dying: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_mpidr_data {
    pub mpidr_mask: u64,
    pub cmpidr_to_idx): DECLARE_FLEX_ARRAY(u16,,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fgt_group_id {
    __NO_FGT_GROUP__,
    HFGRTR_GROUP,
    HFGWTR_GROUP = HFGRTR_GROUP,
    HDFGRTR_GROUP,
    HDFGWTR_GROUP = HDFGRTR_GROUP,
    HFGITR_GROUP,
    HAFGRTR_GROUP,
    HFGRTR2_GROUP,
    HFGWTR2_GROUP = HFGRTR2_GROUP,
    HDFGRTR2_GROUP,
    HDFGWTR2_GROUP = HDFGRTR2_GROUP,
    HFGITR2_GROUP,
    ICH_HFGRTR_GROUP,
    ICH_HFGWTR_GROUP = ICH_HFGRTR_GROUP,
    ICH_HFGITR_GROUP,

// Must be last
    __NR_FGT_GROUP_IDS__
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_arch {
    pub mmu: kvm_s2_mmu,
//
// Fine-Grained UNDEF, mimicking the FGT layout defined by the
// architecture. We track them globally, as we present the
// same feature-set to all vcpus.
//
// Index 0 is currently spare.
//
    pub fgu: [u64; __NR_FGT_GROUP_IDS__],
//
// Stage 2 paging state for VMs with nested S2 using a virtual
// VMID.
//
    pub nested_mmus: *mut kvm_s2_mmu,
    pub nested_mmus_size: usize,
    pub nested_mmus_next: c_int,
// Interrupt controller
    pub vgic: vgic_dist,
// Timers
    pub timer_data: arch_timer_vm_data,
// Mandated version of PSCI
    pub psci_version: u32,
// Protects VM-scoped configuration data
    pub config_lock: mutex,
//
// If we encounter a data abort without valid instruction syndrome
// information, report this to user space.  User space can (and
// should) opt in to this feature if KVM_CAP_ARM_NISV_TO_USER is
// supported.
//
pub const KVM_ARCH_FLAG_RETURN_NISV_IO_ABORT_TO_USER: c_int = 0;
// Memory Tagging Extension enabled for the guest
pub const KVM_ARCH_FLAG_MTE_ENABLED: c_int = 1;
// At least one vCPU has ran in the VM
pub const KVM_ARCH_FLAG_HAS_RAN_ONCE: c_int = 2;
// The vCPU feature set for the VM is configured
pub const KVM_ARCH_FLAG_VCPU_FEATURES_CONFIGURED: c_int = 3;
// PSCI SYSTEM_SUSPEND enabled for the guest
pub const KVM_ARCH_FLAG_SYSTEM_SUSPEND_ENABLED: c_int = 4;
// VM counter offset
pub const KVM_ARCH_FLAG_VM_COUNTER_OFFSET: c_int = 5;
// Timer PPIs made immutable
pub const KVM_ARCH_FLAG_TIMER_PPIS_IMMUTABLE: c_int = 6;
// Initial ID reg values loaded
pub const KVM_ARCH_FLAG_ID_REGS_INITIALIZED: c_int = 7;
// Fine-Grained UNDEF initialised
pub const KVM_ARCH_FLAG_FGU_INITIALIZED: c_int = 8;
// SVE exposed to guest
pub const KVM_ARCH_FLAG_GUEST_HAS_SVE: c_int = 9;
// MIDR_EL1, REVIDR_EL1, and AIDR_EL1 are writable from userspace
pub const KVM_ARCH_FLAG_WRITABLE_IMP_ID_REGS: c_int = 10;
// Unhandled SEAs are taken to userspace
pub const KVM_ARCH_FLAG_EXIT_SEA: c_int = 11;
    pub flags: c_ulong,
// VM-wide vCPU feature set
    pub KVM_VCPU_MAX_FEATURES): DECLARE_BITMAP(vcpu_features,,
// MPIDR to vcpu index mapping, optional
    pub mpidr_data: *mut kvm_mpidr_data,
//
// VM-wide PMU filter, implemented as a bitmap and big enough for
// up to 2^10 events (ARMv8.0) or 2^16 events (ARMv8.1+).
//
    pub pmu_filter: *mut c_ulong,
    pub arm_pmu: *mut arm_pmu,
    pub supported_cpus: cpumask_var_t,
// Maximum number of counters for the guest
    pub nr_pmu_counters: u8,
// PMMIR_EL1.SLOTS value exposed to the guest.
    pub pmmir_slots: u8,
// Hypercall features firmware registers' descriptor
    pub smccc_feat: kvm_smccc_features,
    pub smccc_filter: maple_tree,
//
// Emulated CPU ID registers per VM
// (Op0, Op1, CRn, CRm, Op2) of the ID registers to be saved in it
// is (3, 0, 0, crm, op2), where 1<=crm<8, 0<=op2<8.
//
// These emulated idregs are VM-wide, but accessed from the context of a vCPU.
// Atomic access to multiple idregs are guarded by kvm_arch.config_lock.
//
    pub id_regs: [u64; KVM_ARM_ID_REG_NUM],
    pub midr_el1: u64,
    pub revidr_el1: u64,
    pub aidr_el1: u64,
    pub ctr_el0: u64,
// Masks for VNCR-backed and general EL2 sysregs
    pub sysreg_masks: *mut kvm_sysreg_masks,
// Count the number of VNCR_EL2 TLBs
    pub vncr_tlb_count: core::sync::atomic::AtomicI32,
//
// For an untrusted host VM, 'pkvm.handle' is used to lookup
// the associated pKVM instance in the hypervisor.
//
    pub pkvm: kvm_protected_vm,

// Nested virtualization info
    pub debugfs_nv_dentry: *mut dentry,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_fault_info {
    pub /: *mut *mut u64 esr_el2; / Hyp Syndrom Register,
    pub /: *mut *mut u64 far_el2; / Hyp Fault Address Register,
    pub /: *mut *mut u64 hpfar_el2; / Hyp IPA Fault Address Register,
    pub /: *mut *mut u64 disr_el1; / Deferred [SError] Status Register,
}

//
// VNCR() just places the VNCR_capable registers in the enum after
// __VNCR_START__, and the value (after correction) to be an 8-byte offset
// from the VNCR base. As we don't require the enum to be otherwise ordered,
// we need the terrible hack below to ensure that we correctly size the
// sys_regs array, no matter what.
//
// The __MAX__ macro has been lifted from Sean Eron Anderson's wonderful
// treasure trove of bit hacks:
// https://graphics.stanford.edu/~seander/bithacks.html#IntegerMinOrMax
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcpu_sysreg {
    __INVALID_SYSREG__,   /* 0 is reserved as an invalid value */
    MPIDR_EL1,	/* MultiProcessor Affinity Register */
    CLIDR_EL1,	/* Cache Level ID Register */
    CSSELR_EL1,	/* Cache Size Selection Register */
    TPIDR_EL0,	/* Thread ID, User R/W */
    TPIDRRO_EL0,	/* Thread ID, User R/O */
    TPIDR_EL1,	/* Thread ID, Privileged */
    CNTKCTL_EL1,	/* Timer Control Register (EL1) */
    PAR_EL1,	/* Physical Address Register */
    MDCCINT_EL1,	/* Monitor Debug Comms Channel Interrupt Enable Reg */
    OSLSR_EL1,	/* OS Lock Status Register */
    DISR_EL1,	/* Deferred Interrupt Status Register */

// Performance Monitors Registers
    PMCR_EL0,	/* Control Register */
    PMSELR_EL0,	/* Event Counter Selection Register */
    PMEVCNTR0_EL0,	/* Event Counter Register (0-30) */
    PMEVCNTR30_EL0 = PMEVCNTR0_EL0 + 30,
    PMCCNTR_EL0,	/* Cycle Counter Register */
    PMEVTYPER0_EL0,	/* Event Type Register (0-30) */
    PMEVTYPER30_EL0 = PMEVTYPER0_EL0 + 30,
    PMCCFILTR_EL0,	/* Cycle Count Filter Register */
    PMCNTENSET_EL0,	/* Count Enable Set Register */
    PMINTENSET_EL1,	/* Interrupt Enable Set Register */
    PMOVSSET_EL0,	/* Overflow Flag Status Set Register */
    PMUSERENR_EL0,	/* User Enable Register */

// Pointer Authentication Registers in a strict increasing order.
    APIAKEYLO_EL1,
    APIAKEYHI_EL1,
    APIBKEYLO_EL1,
    APIBKEYHI_EL1,
    APDAKEYLO_EL1,
    APDAKEYHI_EL1,
    APDBKEYLO_EL1,
    APDBKEYHI_EL1,
    APGAKEYLO_EL1,
    APGAKEYHI_EL1,

// Memory Tagging Extension registers
    RGSR_EL1,	/* Random Allocation Tag Seed Register */
    GCR_EL1,	/* Tag Control Register */
    TFSRE0_EL1,	/* Tag Fault Status Register (EL0) */

    POR_EL0,	/* Permission Overlay Register 0 (EL0) */

// FP/SIMD/SVE
    SVCR,
    FPMR,

// 32bit specific registers.
    DACR32_EL2,	/* Domain Access Control Register */
    IFSR32_EL2,	/* Instruction Fault Status Register */
    FPEXC32_EL2,	/* Floating-Point Exception Control Register */
    DBGVCR32_EL2,	/* Debug Vector Catch Register */

// EL2 registers
    ACTLR_EL2,	/* Auxiliary Control Register (EL2) */
    CPTR_EL2,	/* Architectural Feature Trap Register (EL2) */
    HACR_EL2,	/* Hypervisor Auxiliary Control Register */
    TTBR0_EL2,	/* Translation Table Base Register 0 (EL2) */
    TTBR1_EL2,	/* Translation Table Base Register 1 (EL2) */
    TCR_EL2,	/* Translation Control Register (EL2) */
    PIRE0_EL2,	/* Permission Indirection Register 0 (EL2) */
    PIR_EL2,	/* Permission Indirection Register 1 (EL2) */
    POR_EL2,	/* Permission Overlay Register 2 (EL2) */
    SPSR_EL2,	/* EL2 saved program status register */
    ELR_EL2,	/* EL2 exception link register */
    AFSR0_EL2,	/* Auxiliary Fault Status Register 0 (EL2) */
    AFSR1_EL2,	/* Auxiliary Fault Status Register 1 (EL2) */
    ESR_EL2,	/* Exception Syndrome Register (EL2) */
    FAR_EL2,	/* Fault Address Register (EL2) */
    HPFAR_EL2,	/* Hypervisor IPA Fault Address Register */
    MAIR_EL2,	/* Memory Attribute Indirection Register (EL2) */
    AMAIR_EL2,	/* Auxiliary Memory Attribute Indirection Register (EL2) */
    VBAR_EL2,	/* Vector Base Address Register (EL2) */
    RVBAR_EL2,	/* Reset Vector Base Address Register */
    CONTEXTIDR_EL2,	/* Context ID Register (EL2) */
    SP_EL2,		/* EL2 Stack Pointer */
    CNTHP_CTL_EL2,
    CNTHP_CVAL_EL2,
    CNTHV_CTL_EL2,
    CNTHV_CVAL_EL2,

// Anything from this can be RES0/RES1 sanitised
    MARKER(__SANITISED_REG_START__),
    SCTLR_EL2,	/* System Control Register (EL2) */
    TCR2_EL2,	/* Extended Translation Control Register (EL2) */
    SCTLR2_EL2,	/* System Control Register 2 (EL2) */
    MDCR_EL2,	/* Monitor Debug Configuration Register (EL2) */
    CNTHCTL_EL2,	/* Counter-timer Hypervisor Control register */
    ZCR_EL2,	/* SVE Control Register (EL2) */
    HCR_EL2,	/* Hypervisor Control Register */

// Any VNCR-capable reg goes after this point
    MARKER(__VNCR_START__),

    VNCR(SCTLR_EL1),/* System Control Register */
    VNCR(ACTLR_EL1),/* Auxiliary Control Register */
    VNCR(CPACR_EL1),/* Coprocessor Access Control */
    VNCR(ZCR_EL1),	/* SVE Control */
    VNCR(TTBR0_EL1),/* Translation Table Base Register 0 */
    VNCR(TTBR1_EL1),/* Translation Table Base Register 1 */
    VNCR(TCR_EL1),	/* Translation Control Register */
    VNCR(TCR2_EL1),	/* Extended Translation Control Register */
    VNCR(SCTLR2_EL1), /* System Control Register 2 */
    VNCR(ESR_EL1),	/* Exception Syndrome Register */
    VNCR(AFSR0_EL1),/* Auxiliary Fault Status Register 0 */
    VNCR(AFSR1_EL1),/* Auxiliary Fault Status Register 1 */
    VNCR(FAR_EL1),	/* Fault Address Register */
    VNCR(MAIR_EL1),	/* Memory Attribute Indirection Register */
    VNCR(VBAR_EL1),	/* Vector Base Address Register */
    VNCR(CONTEXTIDR_EL1),	/* Context ID Register */
    VNCR(AMAIR_EL1),/* Aux Memory Attribute Indirection Register */
    VNCR(MDSCR_EL1),/* Monitor Debug System Control Register */
    VNCR(ELR_EL1),
    VNCR(SP_EL1),
    VNCR(SPSR_EL1),
    VNCR(TFSR_EL1),	/* Tag Fault Status Register (EL1) */
    VNCR(VPIDR_EL2),/* Virtualization Processor ID Register */
    VNCR(VMPIDR_EL2),/* Virtualization Multiprocessor ID Register */
    VNCR(NVHCR_EL2),/* NV Hypervisor Configuration Register */
    VNCR(HSTR_EL2),	/* Hypervisor System Trap Register */
    VNCR(VTTBR_EL2),/* Virtualization Translation Table Base Register */
    VNCR(VTCR_EL2),	/* Virtualization Translation Control Register */
    VNCR(TPIDR_EL2),/* EL2 Software Thread ID Register */
    VNCR(HCRX_EL2),	/* Extended Hypervisor Configuration Register */

// Permission Indirection Extension registers
    VNCR(PIR_EL1),	 /* Permission Indirection Register 1 (EL1) */
    VNCR(PIRE0_EL1), /*  Permission Indirection Register 0 (EL1) */

    VNCR(POR_EL1),	/* Permission Overlay Register 1 (EL1) */

// FEAT_RAS registers
    VNCR(VDISR_EL2),
    VNCR(VSESR_EL2),

    VNCR(HFGRTR_EL2),
    VNCR(HFGWTR_EL2),
    VNCR(HFGITR_EL2),
    VNCR(HDFGRTR_EL2),
    VNCR(HDFGWTR_EL2),
    VNCR(HAFGRTR_EL2),
    VNCR(HFGRTR2_EL2),
    VNCR(HFGWTR2_EL2),
    VNCR(HFGITR2_EL2),
    VNCR(HDFGRTR2_EL2),
    VNCR(HDFGWTR2_EL2),

    VNCR(VNCR_EL2),

    VNCR(CNTVOFF_EL2),
    VNCR(CNTV_CVAL_EL0),
    VNCR(CNTV_CTL_EL0),
    VNCR(CNTP_CVAL_EL0),
    VNCR(CNTP_CTL_EL0),

    VNCR(ICH_LR0_EL2),
    VNCR(ICH_LR1_EL2),
    VNCR(ICH_LR2_EL2),
    VNCR(ICH_LR3_EL2),
    VNCR(ICH_LR4_EL2),
    VNCR(ICH_LR5_EL2),
    VNCR(ICH_LR6_EL2),
    VNCR(ICH_LR7_EL2),
    VNCR(ICH_LR8_EL2),
    VNCR(ICH_LR9_EL2),
    VNCR(ICH_LR10_EL2),
    VNCR(ICH_LR11_EL2),
    VNCR(ICH_LR12_EL2),
    VNCR(ICH_LR13_EL2),
    VNCR(ICH_LR14_EL2),
    VNCR(ICH_LR15_EL2),

    VNCR(ICH_AP0R0_EL2),
    VNCR(ICH_AP0R1_EL2),
    VNCR(ICH_AP0R2_EL2),
    VNCR(ICH_AP0R3_EL2),
    VNCR(ICH_AP1R0_EL2),
    VNCR(ICH_AP1R1_EL2),
    VNCR(ICH_AP1R2_EL2),
    VNCR(ICH_AP1R3_EL2),
    VNCR(ICH_HCR_EL2),
    VNCR(ICH_VMCR_EL2),

    VNCR(ICH_HFGRTR_EL2),
    VNCR(ICH_HFGWTR_EL2),
    VNCR(ICH_HFGITR_EL2),

    NR_SYS_REGS	/* Nothing after this line! */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct resx {
    pub res0: u64,
    pub res1: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sysreg_masks {
    pub __SANITISED_REG_START__]: resx mask[NR_SYS_REGS -,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fgt_masks {
    pub str: *const c_char,
    pub mask: u64,
    pub nmask: u64,
    pub res0: u64,
    pub res1: u64,
}

extern "C" {
    pub fn kvm_nvhe_sym(_arg: hfgrtr_masks) -> fgt_masks;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: hfgwtr_masks) -> fgt_masks;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: hfgitr_masks) -> fgt_masks;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: hdfgrtr_masks) -> fgt_masks;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: hdfgwtr_masks) -> fgt_masks;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: hafgrtr_masks) -> fgt_masks;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: hfgrtr2_masks) -> fgt_masks;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: hfgwtr2_masks) -> fgt_masks;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: hfgitr2_masks) -> fgt_masks;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: hdfgrtr2_masks) -> fgt_masks;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: hdfgwtr2_masks) -> fgt_masks;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: ich_hfgrtr_masks) -> fgt_masks;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: ich_hfgwtr_masks) -> fgt_masks;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: ich_hfgitr_masks) -> fgt_masks;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_cpu_context {
    pub /: *mut *mut user_pt_regs regs; / sp = sp_el0,
    pub spsr_abt: u64,
    pub spsr_und: u64,
    pub spsr_irq: u64,
    pub spsr_fiq: u64,
    pub fp_regs: user_fpsimd_state,
    pub sys_regs: [u64; NR_SYS_REGS],
    pub __hyp_running_vcpu: *mut kvm_vcpu,
// This pointer has to be 4kB aligned.
    pub vncr_array: *mut u64,
}

//
// This structure is instantiated on a per-CPU basis, and contains
// data that is:
//
// - tied to a single physical CPU, and
// - either have a lifetime that does not extend past vcpu_put()
// - or is an invariant for the lifetime of the system
//
// Use host_data_ptr(field) as a way to access a pointer to such a
// field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_host_data {
pub const KVM_HOST_DATA_FLAG_HAS_SPE: c_int = 0;
pub const KVM_HOST_DATA_FLAG_HAS_TRBE: c_int = 1;
pub const KVM_HOST_DATA_FLAG_TRBE_ENABLED: c_int = 2;
pub const KVM_HOST_DATA_FLAG_EL1_TRACING_CONFIGURED: c_int = 3;
pub const KVM_HOST_DATA_FLAG_VCPU_IN_HYP_CONTEXT: c_int = 4;
pub const KVM_HOST_DATA_FLAG_L1_VNCR_MAPPED: c_int = 5;
pub const KVM_HOST_DATA_FLAG_HAS_BRBE: c_int = 6;
    pub flags: c_ulong,
    pub host_ctxt: kvm_cpu_context,
//
// Hyp VA.
// sve_regs is only used in pKVM and if system_supports_sve().
//
    pub sve_regs: *mut arm64_sve_state,
// Ownership of the FP regs
    pub fp_owner: },
//
// host_debug_state contains the host registers which are
// saved and restored during world switches.
//
// {Break,watch}point registers
    pub regs: kvm_guest_debug_arch,
// Statistical profiling extension
    pub pmscr_el1: u64,
    pub pmblimitr_el1: u64,
// Self-hosted trace
    pub trfcr_el1: u64,
    pub trblimitr_el1: u64,
// Values of trap registers for the host before guest entry.
    pub mdcr_el2: u64,
    pub brbcr_el1: u64,
    pub host_debug_state: },
// Guest trace filter value
    pub trfcr_while_in_guest: u64,
// Number of programmable event counters (PMCR_EL0.N) for this CPU
    pub nr_event_counters: c_uint,
// Number of debug breakpoints/watchpoints for this CPU (minus 1)
    pub debug_brps: c_uint,
    pub debug_wrps: c_uint,
// Last vgic_irq part of the AP list recorded in an LR
    pub last_lr_irq: *mut vgic_irq,
// PPI state tracking for GICv5-based guests
    pub VGIC_V5_NR_PRIVATE_IRQS): DECLARE_BITMAP(pendr,,
// The saved state of the regs when leaving the guest
    pub VGIC_V5_NR_PRIVATE_IRQS): DECLARE_BITMAP(activer_exit,,
    pub vgic_v5_ppi_state: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_host_psci_config {
// PSCI version used by host.
    pub version: u32,
    pub smccc_version: u32,
// Function IDs used by host if version is v0.1.
    pub function_ids_0_1: psci_0_1_function_ids,
    pub psci_0_1_cpu_suspend_implemented: bool,
    pub psci_0_1_cpu_on_implemented: bool,
    pub psci_0_1_cpu_off_implemented: bool,
    pub psci_0_1_migrate_implemented: bool,
}

extern "C" {
    pub fn kvm_nvhe_sym(_arg: kvm_host_psci_config) -> kvm_host_psci_config;
}

extern "C" {
    pub fn kvm_nvhe_sym(_arg: hyp_physvirt_offset) -> i64;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcpu_reset_state {
    pub pc: c_ulong,
    pub r0: c_ulong,
    pub be: bool,
    pub reset: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_arch {
    pub ctxt: kvm_cpu_context,
//
// Guest floating point state
//
// The architecture has two main floating point extensions,
// the original FPSIMD and SVE.  These have overlapping
// register views, with the FPSIMD V registers occupying the
// low 128 bits of the SVE Z registers.  When the core
// floating point code saves the register state of a task it
// records which view it saved in fp_type.
//
    pub sve_state: *mut arm64_sve_state,
    pub fp_type: fp_type,
    pub sve_max_vl: c_uint,
// Stage 2 paging state used by the hardware on next switch
    pub hw_mmu: *mut kvm_s2_mmu,
// Values of trap registers for the guest.
    pub hcr_el2: u64,
    pub hcrx_el2: u64,
    pub mdcr_el2: u64,
    pub r: u64,
    pub w: u64,
    pub fgt: [}; __NR_FGT_GROUP_IDS__],
// Exception Information
    pub fault: kvm_vcpu_fault_info,
// Configuration flags, set once and for all before the vcpu can run
    pub cflags: u8,
// Input flags to the hypervisor code, potentially cleared after use
    pub iflags: u8,
// State flags for kernel bookkeeping, unused by the hypervisor code
    pub sflags: u16,
//
// Don't run the guest (internal implementation need).
//
// Contrary to the flags above, this is set/cleared outside of
// a vcpu context, and thus cannot be mixed with the flags
// themselves (or the flag accesses need to be made atomic).
//
    pub pause: bool,
//
// We maintain more than a single set of debug registers to support
// debugging the guest from the host and to maintain separate host and
// guest state during world switches. vcpu_debug_state are the debug
// registers of the vcpu as the guest sees them.
//
// external_debug_state contains the debug values we want to debug the
// guest. This is set via the KVM_SET_GUEST_DEBUG ioctl.
//
    pub vcpu_debug_state: kvm_guest_debug_arch,
    pub external_debug_state: kvm_guest_debug_arch,
    pub external_mdscr_el1: u64,
    pub debug_owner: },
// VGIC state
    pub vgic_cpu: vgic_cpu,
    pub timer_cpu: arch_timer_cpu,
    pub pmu: kvm_pmu,
// vcpu power state
    pub mp_state: kvm_mp_state,
    pub mp_state_lock: spinlock_t,
// Cache some mmu pages needed inside spinlock regions
    pub mmu_page_cache: kvm_mmu_memory_cache,
// Pages to top-up the pKVM/EL2 guest pool
    pub pkvm_memcache: kvm_hyp_memcache,
// Virtual SError ESR to restore when HCR_EL2.VSE is set
    pub vsesr_el2: u64,
// Additional reset state
    pub reset_state: vcpu_reset_state,
// Guest PV state
    pub last_steal: u64,
    pub base: gpa_t,
    pub steal: },
// Per-vcpu CCSIDR override or NULL
    pub ccsidr: *mut u32,
// Per-vcpu TLB for VNCR_EL2 -- NULL when !NV
    pub vncr_tlb: *mut vncr_tlb,
// Hyp-readable copy of kvm_vcpu::pid
    pub pid: pid_t,
}

//
// Each 'flag' is composed of a comma-separated triplet:
//
// - the flag-set it belongs to in the vcpu->arch structure
// - the value for that flag
// - the mask for that flag
//
// __vcpu_single_flag() builds such a triplet for a single-bit flag.
// unpack_vcpu_flag() extract the flag value from the triplet for
// direct use outside of the flag accessors.
//

// Check that the flags fit in the mask */	\
// Check that the flags fit in the type */	\

//
// Note that the set/clear accessors must be preempt-safe in order to
// avoid nesting them with load/put which also manipulate flags...
//

// the nVHE hypervisor is always non-preemptible
// Macro flag: #define __vcpu_flags_preempt_disable()
// Macro flag: #define __vcpu_flags_preempt_enable()

// fset &= ~(m);				\
// fset |= (f);					\

// fset &= ~(m);					\

// KVM_ARM_VCPU_INIT completed

// SVE config completed

// pKVM VCPU setup completed

// Exception pending

//
// PC increment. Overlaps with EXCEPT_MASK on purpose so that it can't
// be set together with an exception...
//

// Target EL/MODE (not a single flag, but let's abuse the macro)

// Host-set: the hyp flushes the non-protected vCPU state in on entry

// Helpers to encode exceptions with minimum fuss

//
// When PENDING_EXCEPTION is set, EXCEPT_MASK can take the following
// values:
//
// For AArch32 EL1:
//

// For AArch64:

// For AArch64 with NV:

// Physical CPU not in supported_cpus

// WFIT instruction trapped

// vcpu system registers loaded on physical CPU

// Software step state is Active-pending for external debug

// Software step state is Active pending for guest debug

// PMUSERENR for the guest EL0 is on physical CPU

// WFI instruction trapped

// KVM is currently emulating a nested ERET

// SError pending for nested guest

// KVM is currently emulating an L2 to L1 exception

//
// Only use __vcpu_sys_reg/ctxt_sys_reg if you know you want the
// memory backed version of a register, and not the one most recently
// accessed by a running VCPU.  For example, for userspace access or
// for system registers that are never context switched, but only
// emulated.
//
// Don't bother with VNCR-based accesses in the nVHE code, it has no
// business dealing with NV.
//

extern "C" {
    pub fn kvm_vcpu_apply_reg_masks(: *const kvm_vcpu, vcpu_sysreg: enum, _arg: u64) -> u64;
}

extern "C" {
    pub fn vcpu_read_sys_reg(: *const kvm_vcpu, vcpu_sysreg: enum) -> u64;
}
extern "C" {
    pub fn vcpu_write_sys_reg(: *mut kvm_vcpu, _arg: u64, vcpu_sysreg: enum);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vm_stat {
    pub generic: kvm_vm_stat_generic,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_stat {
    pub generic: kvm_vcpu_stat_generic,
    pub hvc_exit_stat: u64,
    pub wfe_exit_stat: u64,
    pub wfi_exit_stat: u64,
    pub mmio_exit_user: u64,
    pub mmio_exit_kernel: u64,
    pub signal_exits: u64,
    pub exits: u64,
}

extern "C" {
    pub fn kvm_arm_num_regs(vcpu: *mut kvm_vcpu) -> c_ulong;
}
extern "C" {
    pub fn kvm_arm_copy_reg_indices(vcpu: *mut kvm_vcpu, indices: *mut u64 __user) -> c_int;
}
extern "C" {
    pub fn kvm_arm_get_reg(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> c_int;
}
extern "C" {
    pub fn kvm_arm_set_reg(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> c_int;
}
extern "C" {
    pub fn kvm_arm_num_sys_reg_descs(vcpu: *mut kvm_vcpu) -> c_ulong;
}
extern "C" {
    pub fn kvm_arm_copy_sys_reg_indices(vcpu: *mut kvm_vcpu, uindices: *mut u64 __user) -> c_int;
}
extern "C" {
    pub fn kvm_arm_halt_guest(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_arm_resume_guest(kvm: *mut kvm);
}

//
// The isb() below is there to guarantee the same behaviour on VHE as on !VHE,
// where the eret to EL1 acts as a context synchronization event.
//

extern "C" {
    pub fn handle_exit(vcpu: *mut kvm_vcpu, exception_index: c_int) -> c_int;
}
extern "C" {
    pub fn handle_exit_early(vcpu: *mut kvm_vcpu, exception_index: c_int);
}
extern "C" {
    pub fn kvm_handle_cp14_load_store(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_handle_cp14_32(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_handle_cp14_64(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_handle_cp15_32(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_handle_cp15_64(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_handle_sys_reg(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_handle_cp10_id(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_sys_regs_create_debugfs(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_reset_sys_regs(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_sys_reg_table_init() -> int __init;
}
extern "C" {
    pub fn populate_nv_trap_config() -> int __init;
}
extern "C" {
    pub fn kvm_calculate_traps(vcpu: *mut kvm_vcpu);
}
// MMIO helpers
extern "C" {
    pub fn kvm_mmio_write_buf(buf: *mut c_void, len: c_uint, data: c_ulong);
}
extern "C" {
    pub fn kvm_mmio_read_buf(buf: *const c_void, len: c_uint) -> c_ulong;
}
extern "C" {
    pub fn kvm_handle_mmio_return(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn io_mem_abort(vcpu: *mut kvm_vcpu, fault_ipa: phys_addr_t) -> c_int;
}
//
// Returns true if a Performance Monitoring Interrupt (PMI), a.k.a. perf event,
// arrived in guest context.  For arm64, any event that arrives while a vCPU is
// loaded is considered to be "in guest".
//
extern "C" {
    pub fn kvm_hypercall_pv_features(vcpu: *mut kvm_vcpu) -> c_long;
}
extern "C" {
    pub fn kvm_init_stolen_time(vcpu: *mut kvm_vcpu) -> gpa_t;
}
extern "C" {
    pub fn kvm_update_stolen_time(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_arm_pvtime_supported() -> bool;
}
extern "C" {
    pub fn kvm_arm_vmid_alloc_init() -> int __init;
}
extern "C" {
    pub fn kvm_arm_vmid_alloc_free() -> void __init;
}
extern "C" {
    pub fn kvm_arm_vmid_update(kvm_vmid: *mut kvm_vmid);
}
extern "C" {
    pub fn kvm_arm_vmid_clear_active();
}
//
// How we access per-CPU host data depends on the where we access it from,
// and the mode we're in:
//
// - VHE and nVHE hypervisor bits use their locally defined instance
//
// - the rest of the kernel use either the VHE or nVHE one, depending on
// the mode we're running in.
//
// Unless we're in protected mode, fully deprivileged, and the nVHE
// per-CPU stuff is exclusively accessible to the protected EL2 code.
// In this case, the EL1 code uses the *VHE* data as its private state
// (which makes sense in a way as there shouldn't be any shared state
// between the host and the hypervisor).
//
// Yes, this is all totally trivial. Shoot me now.
//

// Check whether the FP regs are owned by the guest
// Check whether the FP regs are owned by the host
// The host's MPIDR is immutable, so let's set it up at boot time
extern "C" {
    pub fn cpus_have_final_cap(_arg: ARM64_SPECTRE_V3A) -> return;
}
extern "C" {
    pub fn kvm_init_host_debug_data();
}
extern "C" {
    pub fn kvm_debug_init_vhe();
}
extern "C" {
    pub fn kvm_vcpu_load_debug(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_vcpu_put_debug(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_debug_set_guest_ownership(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_debug_handle_oslar(vcpu: *mut kvm_vcpu, val: u64);
}

// Guest/host FPSIMD coordination helpers
extern "C" {
    pub fn kvm_arch_vcpu_load_fp(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_arch_vcpu_ctxflush_fp(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_arch_vcpu_ctxsync_fp(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_arch_vcpu_put_fp(vcpu: *mut kvm_vcpu);
}

extern "C" {
    pub fn kvm_set_pmu_events(set: u64, attr: *mut perf_event_attr);
}
extern "C" {
    pub fn kvm_clr_pmu_events(clr: u64);
}
extern "C" {
    pub fn kvm_set_pmuserenr(val: u64) -> bool;
}
extern "C" {
    pub fn kvm_enable_trbe();
}
extern "C" {
    pub fn kvm_disable_trbe();
}
extern "C" {
    pub fn kvm_tracing_set_el1_configuration(trfcr_while_in_guest: u64);
}

extern "C" {
    pub fn kvm_vcpu_load_vhe(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_vcpu_put_vhe(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_set_ipa_limit() -> int __init;
}
extern "C" {
    pub fn kvm_get_pa_bits(kvm: *mut kvm) -> u32;
}

extern "C" {
    pub fn kvm_arm_vcpu_finalize(vcpu: *mut kvm_vcpu, feature: c_int) -> c_int;
}
extern "C" {
    pub fn kvm_arm_vcpu_is_finalized(vcpu: *mut kvm_vcpu) -> bool;
}

extern "C" {
    pub fn test_bit(_arg: feature, _arg: ka->vcpu_features) -> return;
}

extern "C" {
    pub fn kvm_trng_call(vcpu: *mut kvm_vcpu) -> c_int;
}

extern "C" {
    pub fn kvm_hyp_reserve() -> void __init;
}

extern "C" {
    pub fn kvm_arm_vcpu_power_off(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_arm_vcpu_stopped(vcpu: *mut kvm_vcpu) -> bool;
}

extern "C" {
    pub fn kvm_set_vm_id_reg(kvm: *mut kvm, reg: u32, val: u64);
}

// Check for a given level of PAuth support

extern "C" {
    pub fn compute_fgu(kvm: *mut kvm, fgt: fgt_group_id);
}
extern "C" {
    pub fn get_reg_fixed_bits(kvm: *mut kvm, reg: vcpu_sysreg) -> resx;
}
extern "C" {
    pub fn check_feature_map();
}
extern "C" {
    pub fn kvm_vcpu_load_fgt(vcpu: *mut kvm_vcpu);
}

extern "C" {
    pub fn kvm_get_cap_for_kvm_ioctl(ioctl: c_uint, ext: *mut c_long) -> c_long;
}
