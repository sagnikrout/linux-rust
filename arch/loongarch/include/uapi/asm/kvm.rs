//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/uapi/asm/kvm.h
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
// Copyright (C) 2020-2023 Loongson Technology Corporation Limited
//

//
// KVM LoongArch specific structures and definitions.
//
// Some parts derived from the x86 version of this file.
//
pub const KVM_COALESCED_MMIO_PAGE_OFFSET: c_int = 1;
pub const KVM_DIRTY_LOG_PAGE_OFFSET: c_int = 64;
pub const KVM_GUESTDBG_USE_SW_BP: c_uint = 0x00010000;
//
// for KVM_GET_REGS and KVM_SET_REGS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_regs {
// out (KVM_GET_REGS) / in (KVM_SET_REGS)
    pub gpr: [__u64; 32],
    pub pc: __u64,
}

//
// for KVM_GET_FPU and KVM_SET_FPU
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_fpu {
    pub fcsr: __u32,
    pub /: *mut *mut __u64 fcc; / 8x8,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_fpureg {
    pub val64: [__u64; 4],
    pub fpr: [}; 32],
}

//
// For LoongArch, we use KVM_SET_ONE_REG and KVM_GET_ONE_REG to access various
// registers.  The id field is broken down as follows:
//
// bits[63..52] - As per linux/kvm.h
// bits[51..32] - Must be zero.
// bits[31..16] - Register set.
//
// Register set = 0: GP registers from kvm_regs (see definitions below).
//
// Register set = 1: CSR registers.
//
// Register set = 2: KVM specific registers (see definitions below).
//
// Register set = 3: FPU / SIMD registers (see definitions below).
//
// Other sets registers may be added in the future.  Each set would
// have its own identifier in bits[31..16].
//

pub const KVM_CSR_IDX_MASK: c_uint = 0x7fff;
pub const KVM_CPUCFG_IDX_MASK: c_uint = 0x7fff;
//
// KVM_REG_LOONGARCH_KVM - KVM specific control registers.
//

// Debugging: Special instruction for software breakpoint

// LBT registers

pub const LOONGARCH_REG_SHIFT: c_int = 3;

// Device Control API on vm fd
pub const KVM_LOONGARCH_VM_FEAT_CTRL: c_int = 0;
pub const KVM_LOONGARCH_VM_FEAT_LSX: c_int = 0;
pub const KVM_LOONGARCH_VM_FEAT_LASX: c_int = 1;
pub const KVM_LOONGARCH_VM_FEAT_X86BT: c_int = 2;
pub const KVM_LOONGARCH_VM_FEAT_ARMBT: c_int = 3;
pub const KVM_LOONGARCH_VM_FEAT_MIPSBT: c_int = 4;
pub const KVM_LOONGARCH_VM_FEAT_PMU: c_int = 5;
pub const KVM_LOONGARCH_VM_FEAT_PV_IPI: c_int = 6;
pub const KVM_LOONGARCH_VM_FEAT_PV_STEALTIME: c_int = 7;
pub const KVM_LOONGARCH_VM_FEAT_PTW: c_int = 8;
pub const KVM_LOONGARCH_VM_FEAT_MSGINT: c_int = 9;
pub const KVM_LOONGARCH_VM_FEAT_PV_PREEMPT: c_int = 10;
// Device Control API on vcpu fd
pub const KVM_LOONGARCH_VCPU_CPUCFG: c_int = 0;
pub const KVM_LOONGARCH_VCPU_PVTIME_CTRL: c_int = 1;
pub const KVM_LOONGARCH_VCPU_PVTIME_GPA: c_int = 0;
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

// dummy definition
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sregs {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_iocsr_entry {
    pub addr: __u32,
    pub pad: __u32,
    pub data: __u64,
}

pub const KVM_NR_IRQCHIPS: c_int = 1;
pub const KVM_IRQCHIP_NUM_PINS: c_int = 64;
pub const KVM_MAX_CORES: c_int = 256;
pub const KVM_DEV_LOONGARCH_IPI_GRP_REGS: c_uint = 0x40000001;
pub const KVM_DEV_LOONGARCH_EXTIOI_GRP_REGS: c_uint = 0x40000002;
pub const KVM_DEV_LOONGARCH_EXTIOI_GRP_SW_STATUS: c_uint = 0x40000003;
pub const KVM_DEV_LOONGARCH_EXTIOI_SW_STATUS_NUM_CPU: c_uint = 0x0;
pub const KVM_DEV_LOONGARCH_EXTIOI_SW_STATUS_FEATURE: c_uint = 0x1;
pub const KVM_DEV_LOONGARCH_EXTIOI_SW_STATUS_STATE: c_uint = 0x2;
pub const KVM_DEV_LOONGARCH_EXTIOI_GRP_CTRL: c_uint = 0x40000004;
pub const KVM_DEV_LOONGARCH_EXTIOI_CTRL_INIT_NUM_CPU: c_uint = 0x0;
pub const KVM_DEV_LOONGARCH_EXTIOI_CTRL_INIT_FEATURE: c_uint = 0x1;
pub const KVM_DEV_LOONGARCH_EXTIOI_CTRL_LOAD_FINISHED: c_uint = 0x3;
pub const KVM_DEV_LOONGARCH_PCH_PIC_GRP_REGS: c_uint = 0x40000005;
pub const KVM_DEV_LOONGARCH_PCH_PIC_GRP_CTRL: c_uint = 0x40000006;
pub const KVM_DEV_LOONGARCH_PCH_PIC_CTRL_INIT: c_int = 0;
pub const KVM_DEV_LOONGARCH_DMSINTC_GRP_CTRL: c_uint = 0x40000007;
pub const KVM_DEV_LOONGARCH_DMSINTC_MSG_ADDR_BASE: c_uint = 0x0;
pub const KVM_DEV_LOONGARCH_DMSINTC_MSG_ADDR_SIZE: c_uint = 0x1;
