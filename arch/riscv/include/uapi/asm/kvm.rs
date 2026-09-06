//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/uapi/asm/kvm.h
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
// Copyright (C) 2019 Western Digital Corporation or its affiliates.
//
// Authors:
// Anup Patel <anup.patel@wdc.com>
//

pub const KVM_COALESCED_MMIO_PAGE_OFFSET: c_int = 1;
pub const KVM_DIRTY_LOG_PAGE_OFFSET: c_int = 64;

// for KVM_GET_REGS and KVM_SET_REGS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_regs {
}

// for KVM_GET_FPU and KVM_SET_FPU
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_fpu {
}

// KVM Debug exit structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_debug_exit_arch {
}

// for KVM_SET_GUEST_DEBUG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_guest_debug_arch {
}

// definition of registers in kvm_run
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sync_regs {
}

// for KVM_GET_SREGS and KVM_SET_SREGS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sregs {
}

// CONFIG registers for KVM_GET_ONE_REG and KVM_SET_ONE_REG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_riscv_config {
    pub isa: c_ulong,
    pub zicbom_block_size: c_ulong,
    pub mvendorid: c_ulong,
    pub marchid: c_ulong,
    pub mimpid: c_ulong,
    pub zicboz_block_size: c_ulong,
    pub satp_mode: c_ulong,
    pub zicbop_block_size: c_ulong,
}

// CORE registers for KVM_GET_ONE_REG and KVM_SET_ONE_REG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_riscv_core {
    pub regs: user_regs_struct,
    pub mode: c_ulong,
}

// Possible privilege modes for kvm_riscv_core
pub const KVM_RISCV_MODE_S: c_int = 1;
pub const KVM_RISCV_MODE_U: c_int = 0;
// General CSR registers for KVM_GET_ONE_REG and KVM_SET_ONE_REG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_riscv_csr {
    pub sstatus: c_ulong,
    pub sie: c_ulong,
    pub stvec: c_ulong,
    pub sscratch: c_ulong,
    pub sepc: c_ulong,
    pub scause: c_ulong,
    pub stval: c_ulong,
    pub sip: c_ulong,
    pub satp: c_ulong,
    pub scounteren: c_ulong,
    pub senvcfg: c_ulong,
}

// AIA CSR registers for KVM_GET_ONE_REG and KVM_SET_ONE_REG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_riscv_aia_csr {
    pub siselect: c_ulong,
    pub iprio1: c_ulong,
    pub iprio2: c_ulong,
    pub sieh: c_ulong,
    pub siph: c_ulong,
    pub iprio1h: c_ulong,
    pub iprio2h: c_ulong,
}

// Smstateen CSR for KVM_GET_ONE_REG and KVM_SET_ONE_REG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_riscv_smstateen_csr {
    pub sstateen0: c_ulong,
}

// Zicfiss CSR for KVM_GET_ONE_REG and KVM_SET_ONE_REG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_riscv_zicfiss_csr {
    pub ssp: c_ulong,
}

// TIMER registers for KVM_GET_ONE_REG and KVM_SET_ONE_REG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_riscv_timer {
    pub frequency: __u64,
    pub time: __u64,
    pub compare: __u64,
    pub state: __u64,
}

// Possible states for kvm_riscv_timer
pub const KVM_RISCV_TIMER_STATE_OFF: c_int = 0;
pub const KVM_RISCV_TIMER_STATE_ON: c_int = 1;
//
// ISA extension IDs specific to KVM. This is not the same as the host ISA
// extension IDs as that is internal to the host and should not be exposed
// to the guest. This should always be contiguous to keep the mapping simple
// in KVM implementation.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum KVM_RISCV_ISA_EXT_ID {
    KVM_RISCV_ISA_EXT_A = 0,
    KVM_RISCV_ISA_EXT_C,
    KVM_RISCV_ISA_EXT_D,
    KVM_RISCV_ISA_EXT_F,
    KVM_RISCV_ISA_EXT_H,
    KVM_RISCV_ISA_EXT_I,
    KVM_RISCV_ISA_EXT_M,
    KVM_RISCV_ISA_EXT_SVPBMT,
    KVM_RISCV_ISA_EXT_SSTC,
    KVM_RISCV_ISA_EXT_SVINVAL,
    KVM_RISCV_ISA_EXT_ZIHINTPAUSE,
    KVM_RISCV_ISA_EXT_ZICBOM,
    KVM_RISCV_ISA_EXT_ZICBOZ,
    KVM_RISCV_ISA_EXT_ZBB,
    KVM_RISCV_ISA_EXT_SSAIA,
    KVM_RISCV_ISA_EXT_V,
    KVM_RISCV_ISA_EXT_SVNAPOT,
    KVM_RISCV_ISA_EXT_ZBA,
    KVM_RISCV_ISA_EXT_ZBS,
    KVM_RISCV_ISA_EXT_ZICNTR,
    KVM_RISCV_ISA_EXT_ZICSR,
    KVM_RISCV_ISA_EXT_ZIFENCEI,
    KVM_RISCV_ISA_EXT_ZIHPM,
    KVM_RISCV_ISA_EXT_SMSTATEEN,
    KVM_RISCV_ISA_EXT_ZICOND,
    KVM_RISCV_ISA_EXT_ZBC,
    KVM_RISCV_ISA_EXT_ZBKB,
    KVM_RISCV_ISA_EXT_ZBKC,
    KVM_RISCV_ISA_EXT_ZBKX,
    KVM_RISCV_ISA_EXT_ZKND,
    KVM_RISCV_ISA_EXT_ZKNE,
    KVM_RISCV_ISA_EXT_ZKNH,
    KVM_RISCV_ISA_EXT_ZKR,
    KVM_RISCV_ISA_EXT_ZKSED,
    KVM_RISCV_ISA_EXT_ZKSH,
    KVM_RISCV_ISA_EXT_ZKT,
    KVM_RISCV_ISA_EXT_ZVBB,
    KVM_RISCV_ISA_EXT_ZVBC,
    KVM_RISCV_ISA_EXT_ZVKB,
    KVM_RISCV_ISA_EXT_ZVKG,
    KVM_RISCV_ISA_EXT_ZVKNED,
    KVM_RISCV_ISA_EXT_ZVKNHA,
    KVM_RISCV_ISA_EXT_ZVKNHB,
    KVM_RISCV_ISA_EXT_ZVKSED,
    KVM_RISCV_ISA_EXT_ZVKSH,
    KVM_RISCV_ISA_EXT_ZVKT,
    KVM_RISCV_ISA_EXT_ZFH,
    KVM_RISCV_ISA_EXT_ZFHMIN,
    KVM_RISCV_ISA_EXT_ZIHINTNTL,
    KVM_RISCV_ISA_EXT_ZVFH,
    KVM_RISCV_ISA_EXT_ZVFHMIN,
    KVM_RISCV_ISA_EXT_ZFA,
    KVM_RISCV_ISA_EXT_ZTSO,
    KVM_RISCV_ISA_EXT_ZACAS,
    KVM_RISCV_ISA_EXT_SSCOFPMF,
    KVM_RISCV_ISA_EXT_ZIMOP,
    KVM_RISCV_ISA_EXT_ZCA,
    KVM_RISCV_ISA_EXT_ZCB,
    KVM_RISCV_ISA_EXT_ZCD,
    KVM_RISCV_ISA_EXT_ZCF,
    KVM_RISCV_ISA_EXT_ZCMOP,
    KVM_RISCV_ISA_EXT_ZAWRS,
    KVM_RISCV_ISA_EXT_SMNPM,
    KVM_RISCV_ISA_EXT_SSNPM,
    KVM_RISCV_ISA_EXT_SVADE,
    KVM_RISCV_ISA_EXT_SVADU,
    KVM_RISCV_ISA_EXT_SVVPTC,
    KVM_RISCV_ISA_EXT_ZABHA,
    KVM_RISCV_ISA_EXT_ZICCRSE,
    KVM_RISCV_ISA_EXT_ZAAMO,
    KVM_RISCV_ISA_EXT_ZALRSC,
    KVM_RISCV_ISA_EXT_ZICBOP,
    KVM_RISCV_ISA_EXT_ZFBFMIN,
    KVM_RISCV_ISA_EXT_ZVFBFMIN,
    KVM_RISCV_ISA_EXT_ZVFBFWMA,
    KVM_RISCV_ISA_EXT_ZCLSD,
    KVM_RISCV_ISA_EXT_ZILSD,
    KVM_RISCV_ISA_EXT_ZALASR,
    KVM_RISCV_ISA_EXT_ZICFILP,
    KVM_RISCV_ISA_EXT_ZICFISS,
    KVM_RISCV_ISA_EXT_MAX,
}

//
// SBI extension IDs specific to KVM. This is not the same as the SBI
// extension IDs defined by the RISC-V SBI specification.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum KVM_RISCV_SBI_EXT_ID {
    KVM_RISCV_SBI_EXT_V01 = 0,
    KVM_RISCV_SBI_EXT_TIME,
    KVM_RISCV_SBI_EXT_IPI,
    KVM_RISCV_SBI_EXT_RFENCE,
    KVM_RISCV_SBI_EXT_SRST,
    KVM_RISCV_SBI_EXT_HSM,
    KVM_RISCV_SBI_EXT_PMU,
    KVM_RISCV_SBI_EXT_EXPERIMENTAL,
    KVM_RISCV_SBI_EXT_VENDOR,
    KVM_RISCV_SBI_EXT_DBCN,
    KVM_RISCV_SBI_EXT_STA,
    KVM_RISCV_SBI_EXT_SUSP,
    KVM_RISCV_SBI_EXT_FWFT,
    KVM_RISCV_SBI_EXT_MPXY,
    KVM_RISCV_SBI_EXT_MAX,
}

// SBI STA extension registers for KVM_GET_ONE_REG and KVM_SET_ONE_REG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_riscv_sbi_sta {
    pub shmem_lo: c_ulong,
    pub shmem_hi: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_riscv_sbi_fwft_feature {
    pub enable: c_ulong,
    pub flags: c_ulong,
    pub value: c_ulong,
}

// SBI FWFT extension registers for KVM_GET_ONE_REG and KVM_SET_ONE_REG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_riscv_sbi_fwft {
    pub misaligned_deleg: kvm_riscv_sbi_fwft_feature,
    pub pointer_masking: kvm_riscv_sbi_fwft_feature,
    pub pte_ad_hw_updating: kvm_riscv_sbi_fwft_feature,
    pub landing_pad: kvm_riscv_sbi_fwft_feature,
    pub shadow_stack: kvm_riscv_sbi_fwft_feature,
}

// If you need to interpret the index values, here is the key:
pub const KVM_REG_RISCV_TYPE_MASK: c_uint = 0x00000000FF000000;
pub const KVM_REG_RISCV_TYPE_SHIFT: c_int = 24;
pub const KVM_REG_RISCV_SUBTYPE_MASK: c_uint = 0x0000000000FF0000;
pub const KVM_REG_RISCV_SUBTYPE_SHIFT: c_int = 16;
// Config registers are mapped as type 1

// Core registers are mapped as type 2

// Control and status registers are mapped as type 3

// Timer registers are mapped as type 4

// F extension registers are mapped as type 5

// D extension registers are mapped as type 6

// ISA Extension registers are mapped as type 7

// SBI extension registers are mapped as type 8

// V extension registers are mapped as type 9

// Registers for specific SBI extensions are mapped as type 10

// Device Control API: RISC-V AIA
pub const KVM_DEV_RISCV_APLIC_ALIGN: c_uint = 0x1000;
pub const KVM_DEV_RISCV_APLIC_SIZE: c_uint = 0x4000;
pub const KVM_DEV_RISCV_APLIC_MAX_HARTS: c_uint = 0x4000;
pub const KVM_DEV_RISCV_IMSIC_ALIGN: c_uint = 0x1000;
pub const KVM_DEV_RISCV_IMSIC_SIZE: c_uint = 0x1000;
pub const KVM_DEV_RISCV_AIA_GRP_CONFIG: c_int = 0;
pub const KVM_DEV_RISCV_AIA_CONFIG_MODE: c_int = 0;
pub const KVM_DEV_RISCV_AIA_CONFIG_IDS: c_int = 1;
pub const KVM_DEV_RISCV_AIA_CONFIG_SRCS: c_int = 2;
pub const KVM_DEV_RISCV_AIA_CONFIG_GROUP_BITS: c_int = 3;
pub const KVM_DEV_RISCV_AIA_CONFIG_GROUP_SHIFT: c_int = 4;
pub const KVM_DEV_RISCV_AIA_CONFIG_HART_BITS: c_int = 5;
pub const KVM_DEV_RISCV_AIA_CONFIG_GUEST_BITS: c_int = 6;
//
// Modes of RISC-V AIA device:
// 1) EMUL (aka Emulation): Trap-n-emulate IMSIC
// 2) HWACCEL (aka HW Acceleration): Virtualize IMSIC using IMSIC guest files
// 3) AUTO (aka Automatic): Virtualize IMSIC using IMSIC guest files whenever
// available otherwise fallback to trap-n-emulation
//
pub const KVM_DEV_RISCV_AIA_MODE_EMUL: c_int = 0;
pub const KVM_DEV_RISCV_AIA_MODE_HWACCEL: c_int = 1;
pub const KVM_DEV_RISCV_AIA_MODE_AUTO: c_int = 2;
pub const KVM_DEV_RISCV_AIA_IDS_MIN: c_int = 63;
pub const KVM_DEV_RISCV_AIA_IDS_MAX: c_int = 2048;
pub const KVM_DEV_RISCV_AIA_SRCS_MAX: c_int = 1024;
pub const KVM_DEV_RISCV_AIA_GROUP_BITS_MAX: c_int = 8;
pub const KVM_DEV_RISCV_AIA_GROUP_SHIFT_MIN: c_int = 24;
pub const KVM_DEV_RISCV_AIA_GROUP_SHIFT_MAX: c_int = 56;
pub const KVM_DEV_RISCV_AIA_HART_BITS_MAX: c_int = 16;
pub const KVM_DEV_RISCV_AIA_GUEST_BITS_MAX: c_int = 8;
pub const KVM_DEV_RISCV_AIA_GRP_ADDR: c_int = 1;
pub const KVM_DEV_RISCV_AIA_ADDR_APLIC: c_int = 0;

pub const KVM_DEV_RISCV_AIA_GRP_CTRL: c_int = 2;
pub const KVM_DEV_RISCV_AIA_CTRL_INIT: c_int = 0;
//
// The device attribute type contains the memory mapped offset of the
// APLIC register (range 0x0000-0x3FFF) and it must be 4-byte aligned.
//
pub const KVM_DEV_RISCV_AIA_GRP_APLIC: c_int = 3;
//
// The lower 12-bits of the device attribute type contains the iselect
// value of the IMSIC register (range 0x70-0xFF) whereas the higher order
// bits contains the VCPU id.
//
pub const KVM_DEV_RISCV_AIA_GRP_IMSIC: c_int = 4;
pub const KVM_DEV_RISCV_AIA_IMSIC_ISEL_BITS: c_int = 12;

// One single KVM irqchip, ie. the AIA
pub const KVM_NR_IRQCHIPS: c_int = 1;

