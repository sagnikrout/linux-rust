//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/uapi/asm/kvm.h
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
// Copyright (C) 2012,2013 - ARM Ltd
// Author: Marc Zyngier <marc.zyngier@arm.com>
//
// Derived from arch/arm/include/uapi/asm/kvm.h:
// Copyright (C) 2012 - Virtual Open Systems and Columbia University
// Author: Christoffer Dall <c.dall@virtualopensystems.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
pub const KVM_SPSR_EL1: c_int = 0;

pub const KVM_SPSR_ABT: c_int = 1;
pub const KVM_SPSR_UND: c_int = 2;
pub const KVM_SPSR_IRQ: c_int = 3;
pub const KVM_SPSR_FIQ: c_int = 4;
pub const KVM_NR_SPSR: c_int = 5;

pub const KVM_COALESCED_MMIO_PAGE_OFFSET: c_int = 1;
pub const KVM_DIRTY_LOG_PAGE_OFFSET: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_regs {
    pub /: *mut *mut user_pt_regs regs; / sp = sp_el0,
    pub sp_el1: __u64,
    pub elr_el1: __u64,
    pub spsr: [__u64; KVM_NR_SPSR],
    pub fp_regs: user_fpsimd_state,
}

//
// Supported CPU Targets - Adding a new target type is not recommended,
// unless there are some special registers not supported by the
// genericv8 syreg table.
//
pub const KVM_ARM_TARGET_AEM_V8: c_int = 0;
pub const KVM_ARM_TARGET_FOUNDATION_V8: c_int = 1;
pub const KVM_ARM_TARGET_CORTEX_A57: c_int = 2;
pub const KVM_ARM_TARGET_XGENE_POTENZA: c_int = 3;
pub const KVM_ARM_TARGET_CORTEX_A53: c_int = 4;
// Generic ARM v8 target
pub const KVM_ARM_TARGET_GENERIC_V8: c_int = 5;
pub const KVM_ARM_NUM_TARGETS: c_int = 6;
// KVM_ARM_SET_DEVICE_ADDR ioctl id encoding
pub const KVM_ARM_DEVICE_TYPE_SHIFT: c_int = 0;

pub const KVM_ARM_DEVICE_ID_SHIFT: c_int = 16;

// Supported device IDs
pub const KVM_ARM_DEVICE_VGIC_V2: c_int = 0;
// Supported VGIC address types
pub const KVM_VGIC_V2_ADDR_TYPE_DIST: c_int = 0;
pub const KVM_VGIC_V2_ADDR_TYPE_CPU: c_int = 1;
pub const KVM_VGIC_V2_DIST_SIZE: c_uint = 0x1000;
pub const KVM_VGIC_V2_CPU_SIZE: c_uint = 0x2000;
// Supported VGICv3 address types
pub const KVM_VGIC_V3_ADDR_TYPE_DIST: c_int = 2;
pub const KVM_VGIC_V3_ADDR_TYPE_REDIST: c_int = 3;
pub const KVM_VGIC_ITS_ADDR_TYPE: c_int = 4;
pub const KVM_VGIC_V3_ADDR_TYPE_REDIST_REGION: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_init {
    pub target: __u32,
    pub features: [__u32; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sregs {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_fpu {
}

//
// See v8 ARM ARM D7.3: Debug Registers
//
// The architectural limit is 16 debug registers of each type although
// in practice there are usually less (see ID_AA64DFR0_EL1).
//
// Although the control registers are architecturally defined as 32
// bits wide we use a 64 bit structure here to keep parity with
// KVM_GET/SET_ONE_REG behaviour which treats all system registers as
// 64 bit values. It also allows for the possibility of the
// architecture expanding the control registers without having to
// change the userspace ABI.
//
pub const KVM_ARM_MAX_DBG_REGS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_guest_debug_arch {
    pub dbg_bcr: [__u64; KVM_ARM_MAX_DBG_REGS],
    pub dbg_bvr: [__u64; KVM_ARM_MAX_DBG_REGS],
    pub dbg_wcr: [__u64; KVM_ARM_MAX_DBG_REGS],
    pub dbg_wvr: [__u64; KVM_ARM_MAX_DBG_REGS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_debug_exit_arch {
    pub hsr: __u32,
    pub /: *mut *mut __u32 hsr_high; / ESR_EL2[61:32],
    pub /: *mut *mut __u64 far; / used for watchpoints,
}

//
// Architecture specific defines for kvm_guest_debug->control
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sync_regs {
// Used with KVM_CAP_ARM_USER_IRQ
    pub device_irq_level: __u64,
}

// Bits for run->s.regs.device_irq_level

//
// PMU filter structure. Describe a range of events with a particular
// action. To be used with KVM_ARM_VCPU_PMU_V3_FILTER.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pmu_event_filter {
    pub base_event: __u16,
    pub nevents: __u16,
pub const KVM_PMU_EVENT_ALLOW: c_int = 0;
pub const KVM_PMU_EVENT_DENY: c_int = 1;
    pub action: __u8,
    pub pad: [__u8; 3],
}

// for KVM_GET/SET_VCPU_EVENTS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_events {
    pub serror_pending: __u8,
    pub serror_has_esr: __u8,
    pub ext_dabt_pending: __u8,
// Align it to 8 bytes
    pub pad: [__u8; 5],
    pub serror_esr: __u64,
    pub exception: },
    pub reserved: [__u32; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_arm_copy_mte_tags {
    pub guest_ipa: __u64,
    pub length: __u64,
    pub addr: *mut void __user,
    pub flags: __u64,
    pub reserved: [__u64; 2],
}

//
// Counter/Timer offset structure. Describe the virtual/physical offset.
// To be used with KVM_ARM_SET_COUNTER_OFFSET.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_arm_counter_offset {
    pub counter_offset: __u64,
    pub reserved: __u64,
}

pub const KVM_ARM_TAGS_TO_GUEST: c_int = 0;
pub const KVM_ARM_TAGS_FROM_GUEST: c_int = 1;
// If you need to interpret the index values, here is the key:
pub const KVM_REG_ARM_COPROC_MASK: c_uint = 0x000000000FFF0000;
pub const KVM_REG_ARM_COPROC_SHIFT: c_int = 16;
// Normal registers are mapped as coprocessor 16.

// Some registers need more space to represent values.

pub const KVM_REG_ARM_DEMUX_ID_MASK: c_uint = 0x000000000000FF00;
pub const KVM_REG_ARM_DEMUX_ID_SHIFT: c_int = 8;

pub const KVM_REG_ARM_DEMUX_VAL_MASK: c_uint = 0x00000000000000FF;
pub const KVM_REG_ARM_DEMUX_VAL_SHIFT: c_int = 0;
// AArch64 system registers

pub const KVM_REG_ARM64_SYSREG_OP0_MASK: c_uint = 0x000000000000c000;
pub const KVM_REG_ARM64_SYSREG_OP0_SHIFT: c_int = 14;
pub const KVM_REG_ARM64_SYSREG_OP1_MASK: c_uint = 0x0000000000003800;
pub const KVM_REG_ARM64_SYSREG_OP1_SHIFT: c_int = 11;
pub const KVM_REG_ARM64_SYSREG_CRN_MASK: c_uint = 0x0000000000000780;
pub const KVM_REG_ARM64_SYSREG_CRN_SHIFT: c_int = 7;
pub const KVM_REG_ARM64_SYSREG_CRM_MASK: c_uint = 0x0000000000000078;
pub const KVM_REG_ARM64_SYSREG_CRM_SHIFT: c_int = 3;
pub const KVM_REG_ARM64_SYSREG_OP2_MASK: c_uint = 0x0000000000000007;
pub const KVM_REG_ARM64_SYSREG_OP2_SHIFT: c_int = 0;

// Physical Timer EL0 Registers

//
// EL0 Virtual Timer Registers
//
// WARNING:
// KVM_REG_ARM_TIMER_CVAL and KVM_REG_ARM_TIMER_CNT are not defined
// with the appropriate register encodings.  Their values have been
// accidentally swapped.  As this is set API, the definitions here
// must be used, rather than ones derived from the encodings.
//

// KVM-as-firmware specific pseudo-registers

pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_1_NOT_AVAIL: c_int = 0;
pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_1_AVAIL: c_int = 1;
pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_1_NOT_REQUIRED: c_int = 2;
//
// Only two states can be presented by the host kernel:
// - NOT_REQUIRED: the guest doesn't need to do anything
// - NOT_AVAIL: the guest isn't mitigated (it can still use SSBS if available)
//
// All the other values are deprecated. The host still accepts all
// values (they are ABI), but will narrow them to the above two.
//

pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_NOT_AVAIL: c_int = 0;
pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_UNKNOWN: c_int = 1;
pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_AVAIL: c_int = 2;
pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_NOT_REQUIRED: c_int = 3;

pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_3_NOT_AVAIL: c_int = 0;
pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_3_AVAIL: c_int = 1;
pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_3_NOT_REQUIRED: c_int = 2;
// SVE registers

// Z- and P-regs occupy blocks at the following offsets within this range:
pub const KVM_REG_ARM64_SVE_ZREG_BASE: c_int = 0;
pub const KVM_REG_ARM64_SVE_PREG_BASE: c_uint = 0x400;
pub const KVM_REG_ARM64_SVE_FFR_BASE: c_uint = 0x600;

pub const KVM_ARM64_SVE_MAX_SLICES: c_int = 32;

//
// Register values for KVM_REG_ARM64_SVE_ZREG(), KVM_REG_ARM64_SVE_PREG() and
// KVM_REG_ARM64_SVE_FFR() are represented in memory in an endianness-
// invariant layout which differs from the layout used for the FPSIMD
// V-registers on big-endian systems: see sigcontext.h for more explanation.
//

// Vector lengths pseudo-register:

// Bitmap feature firmware registers

// Vendor hyper call function numbers 0-63

// Vendor hyper call function numbers 64-127

// Device Control API on vm fd
pub const KVM_ARM_VM_SMCCC_CTRL: c_int = 0;
pub const KVM_ARM_VM_SMCCC_FILTER: c_int = 0;
// Device Control API: ARM VGIC
pub const KVM_DEV_ARM_VGIC_GRP_ADDR: c_int = 0;
pub const KVM_DEV_ARM_VGIC_GRP_DIST_REGS: c_int = 1;
pub const KVM_DEV_ARM_VGIC_GRP_CPU_REGS: c_int = 2;
pub const KVM_DEV_ARM_VGIC_CPUID_SHIFT: c_int = 32;

pub const KVM_DEV_ARM_VGIC_V3_MPIDR_SHIFT: c_int = 32;

pub const KVM_DEV_ARM_VGIC_OFFSET_SHIFT: c_int = 0;

pub const KVM_DEV_ARM_VGIC_GRP_NR_IRQS: c_int = 3;
pub const KVM_DEV_ARM_VGIC_GRP_CTRL: c_int = 4;
pub const KVM_DEV_ARM_VGIC_GRP_REDIST_REGS: c_int = 5;
pub const KVM_DEV_ARM_VGIC_GRP_CPU_SYSREGS: c_int = 6;
pub const KVM_DEV_ARM_VGIC_GRP_LEVEL_INFO: c_int = 7;
pub const KVM_DEV_ARM_VGIC_GRP_ITS_REGS: c_int = 8;
pub const KVM_DEV_ARM_VGIC_GRP_MAINT_IRQ: c_int = 9;
pub const KVM_DEV_ARM_VGIC_LINE_LEVEL_INFO_SHIFT: c_int = 10;

pub const KVM_DEV_ARM_VGIC_LINE_LEVEL_INTID_MASK: c_uint = 0x3ff;
pub const VGIC_LEVEL_INFO_LINE_LEVEL: c_int = 0;
pub const KVM_DEV_ARM_VGIC_CTRL_INIT: c_int = 0;
pub const KVM_DEV_ARM_ITS_SAVE_TABLES: c_int = 1;
pub const KVM_DEV_ARM_ITS_RESTORE_TABLES: c_int = 2;
pub const KVM_DEV_ARM_VGIC_SAVE_PENDING_TABLES: c_int = 3;
pub const KVM_DEV_ARM_ITS_CTRL_RESET: c_int = 4;
pub const KVM_DEV_ARM_VGIC_USERSPACE_PPIS: c_int = 5;
// Device Control API on vcpu fd
pub const KVM_ARM_VCPU_PMU_V3_CTRL: c_int = 0;
pub const KVM_ARM_VCPU_PMU_V3_IRQ: c_int = 0;
pub const KVM_ARM_VCPU_PMU_V3_INIT: c_int = 1;
pub const KVM_ARM_VCPU_PMU_V3_FILTER: c_int = 2;
pub const KVM_ARM_VCPU_PMU_V3_SET_PMU: c_int = 3;
pub const KVM_ARM_VCPU_PMU_V3_SET_NR_COUNTERS: c_int = 4;
pub const KVM_ARM_VCPU_TIMER_CTRL: c_int = 1;
pub const KVM_ARM_VCPU_TIMER_IRQ_VTIMER: c_int = 0;
pub const KVM_ARM_VCPU_TIMER_IRQ_PTIMER: c_int = 1;
pub const KVM_ARM_VCPU_TIMER_IRQ_HVTIMER: c_int = 2;
pub const KVM_ARM_VCPU_TIMER_IRQ_HPTIMER: c_int = 3;
pub const KVM_ARM_VCPU_PVTIME_CTRL: c_int = 2;
pub const KVM_ARM_VCPU_PVTIME_IPA: c_int = 0;
// KVM_IRQ_LINE irq field index values
pub const KVM_ARM_IRQ_VCPU2_SHIFT: c_int = 28;
pub const KVM_ARM_IRQ_VCPU2_MASK: c_uint = 0xf;
pub const KVM_ARM_IRQ_TYPE_SHIFT: c_int = 24;
pub const KVM_ARM_IRQ_TYPE_MASK: c_uint = 0xf;
pub const KVM_ARM_IRQ_VCPU_SHIFT: c_int = 16;
pub const KVM_ARM_IRQ_VCPU_MASK: c_uint = 0xff;
pub const KVM_ARM_IRQ_NUM_SHIFT: c_int = 0;
pub const KVM_ARM_IRQ_NUM_MASK: c_uint = 0xffff;
// irq_type field
pub const KVM_ARM_IRQ_TYPE_CPU: c_int = 0;
pub const KVM_ARM_IRQ_TYPE_SPI: c_int = 1;
pub const KVM_ARM_IRQ_TYPE_PPI: c_int = 2;
// out-of-kernel GIC cpu interrupt injection irq_number field
pub const KVM_ARM_IRQ_CPU_IRQ: c_int = 0;
pub const KVM_ARM_IRQ_CPU_FIQ: c_int = 1;
//
// This used to hold the highest supported SPI, but it is now obsolete
// and only here to provide source code level compatibility with older
// userland. The highest SPI number can be set via KVM_DEV_ARM_VGIC_GRP_NR_IRQS.
//
pub const KVM_ARM_IRQ_GIC_MAX: c_int = 127;

// One single KVM irqchip, ie. the VGIC
pub const KVM_NR_IRQCHIPS: c_int = 1;
// PSCI interface
pub const KVM_PSCI_FN_BASE: c_uint = 0x95c1ba5e;

// arm64-specific kvm_run::system_event flags
//
// Reset caused by a PSCI v1.1 SYSTEM_RESET2 call.
// Valid only when the system event has a type of KVM_SYSTEM_EVENT_RESET.
//

//
// Shutdown caused by a PSCI v1.3 SYSTEM_OFF2 call.
// Valid only when the system event has a type of KVM_SYSTEM_EVENT_SHUTDOWN.
//

// run->fail_entry.hardware_entry_failure_reason codes.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_smccc_filter_action {
    KVM_SMCCC_FILTER_HANDLE = 0,
    KVM_SMCCC_FILTER_DENY,
    KVM_SMCCC_FILTER_FWD_TO_USER,

    NR_SMCCC_FILTER_ACTIONS

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_smccc_filter {
    pub base: __u32,
    pub nr_functions: __u32,
    pub action: __u8,
    pub pad: [__u8; 15],
}

// arm64-specific KVM_EXIT_HYPERCALL flags

//
// Get feature ID registers userspace writable mask.
//
// From DDI0487J.a, D19.2.66 ("ID_AA64MMFR2_EL1, AArch64 Memory Model
// Feature Register 2"):
//
// "The Feature ID space is defined as the System register space in
// AArch64 with op0==3, op1=={0, 1, 3}, CRn==0, CRm=={0-7},
// op2=={0-7}."
//
// This covers all currently known R/O registers that indicate
// anything useful feature wise, including the ID registers.
//
// If we ever need to introduce a new range, it will be described as
// such in the range field.
//

pub const KVM_ARM_FEATURE_ID_RANGE: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_mask_range {
    pub /: *mut *mut __u64 addr; / Pointer to mask array,
    pub /: *mut *mut __u32 range; / Requested range,
    pub reserved: [__u32; 13],
}

