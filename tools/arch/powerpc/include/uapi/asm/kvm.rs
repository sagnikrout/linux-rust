//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/powerpc/include/uapi/asm/kvm.h
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
// Copyright IBM Corp. 2007
//
// Authors: Hollis Blanchard <hollisb@us.ibm.com>
//

// Select powerpc specific features in <linux/kvm.h>
// Not always available, but if it is, this is the correct offset.
pub const KVM_COALESCED_MMIO_PAGE_OFFSET: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_regs {
    pub pc: __u64,
    pub cr: __u64,
    pub ctr: __u64,
    pub lr: __u64,
    pub xer: __u64,
    pub msr: __u64,
    pub srr0: __u64,
    pub srr1: __u64,
    pub pid: __u64,
    pub sprg0: __u64,
    pub sprg1: __u64,
    pub sprg2: __u64,
    pub sprg3: __u64,
    pub sprg4: __u64,
    pub sprg5: __u64,
    pub sprg6: __u64,
    pub sprg7: __u64,
    pub gpr: [__u64; 32],
}

pub const KVM_SREGS_E_IMPL_NONE: c_int = 0;
pub const KVM_SREGS_E_IMPL_FSL: c_int = 1;

// flags for kvm_run.flags

//
// Feature bits indicate which sections of the sregs struct are valid,
// both in KVM_GET_SREGS and KVM_SET_SREGS.  On KVM_SET_SREGS, registers
// corresponding to unset feature bits will not be modified.  This allows
// restoring a checkpoint made without that feature, while keeping the
// default values of the new registers.
//
// KVM_SREGS_E_BASE contains:
// CSRR0/1 (refers to SRR2/3 on 40x)
// ESR
// DEAR
// MCSR
// TSR
// TCR
// DEC
// TB
// VRSAVE (USPRG0)
//

//
// KVM_SREGS_E_ARCH206 contains:
//
// PIR
// MCSRR0/1
// DECAR
// IVPR
//

//
// Contains EPCR, plus the upper half of 64-bit registers
// that are 32-bit on 32-bit implementations.
//

//
// IVORs are used -- contains IVOR0-15, plus additional IVORs
// in combination with an appropriate feature bit.
//

//
// Contains MAS0-4, MAS6-7, TLBnCFG, MMUCFG.
// Also TLBnPS if MMUCFG[MAVN] = 1.
//

// DBSR, DBCR, IAC, DAC, DVC

// Enhanced debug -- DSRR0/1, SPRG9

// Embedded Floating Point (SPE) -- IVOR32-34 if KVM_SREGS_E_IVOR

//
// DEPRECATED! USE ONE_REG FOR THIS ONE!
// External Proxy (EXP) -- EPR
//

// External PID (E.PD) -- EPSC/EPLC

// Processor Control (E.PC) -- IVOR36-37 if KVM_SREGS_E_IVOR

// Page table (E.PT) -- EPTCFG

// Embedded Performance Monitor (E.PM) -- IVOR35 if KVM_SREGS_E_IVOR

//
// Special updates:
//
// Some registers may change even while a vcpu is not running.
// To avoid losing these changes, by default these registers are
// not updated by KVM_SET_SREGS.  To force an update, set the bit
// in u.e.update_special corresponding to the register to be updated.
//
// The update_special field is zero on return from KVM_GET_SREGS.
//
// When restoring a checkpoint, the caller can set update_special
// to 0xffffffff to ensure that everything is restored, even new features
// that the caller doesn't know about.
//

//
// In KVM_SET_SREGS, reserved/pad fields must be left untouched from a
// previous KVM_GET_REGS.
//
// Unless otherwise indicated, setting any register with KVM_SET_SREGS
// directly sets its value.  It does not trigger any special semantics such
// as write-one-to-clear.  Calling KVM_SET_SREGS on an unmodified struct
// just received from KVM_GET_SREGS is always a no-op.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sregs {
    pub pvr: __u32,
    pub sdr1: __u64,
    pub slbe: __u64,
    pub slbv: __u64,
    pub slb: [}; 64],
    pub ppc64: },
    pub sr: [__u32; 16],
    pub ibat: [__u64; 8],
    pub dbat: [__u64; 8],
    pub ppc32: },
    pub s: },
    pub /: *mut *mut __u32 features; / KVM_SREGS_E_FSL_,
    pub svr: __u32,
    pub mcar: __u64,
    pub hid0: __u32,
// KVM_SREGS_E_FSL_PIDn
    pub pid2: __u32 pid1,,
    pub fsl: },
    pub pad: [__u8; 256],
    pub impl: },
    pub /: *mut *mut __u32 features; / KVM_SREGS_E_,
    pub /: *mut *mut __u32 impl_id; / KVM_SREGS_E_IMPL_,
    pub /: *mut *mut __u32 update_special; / KVM_SREGS_E_UPDATE_,
    pub /: *mut *mut __u32 pir; / read-only,
    pub sprg8: __u64,
    pub /: *mut *mut __u64 sprg9; / E.ED,
    pub csrr0: __u64,
    pub /: *mut *mut __u64 dsrr0; / E.ED,
    pub mcsrr0: __u64,
    pub csrr1: __u32,
    pub /: *mut *mut __u32 dsrr1; / E.ED,
    pub mcsrr1: __u32,
    pub esr: __u32,
    pub dear: __u64,
    pub ivpr: __u64,
    pub mcivpr: __u64,
    pub /: *mut *mut __u64 mcsr; / KVM_SREGS_E_UPDATE_MCSR,
    pub /: *mut *mut __u32 tsr; / KVM_SREGS_E_UPDATE_TSR,
    pub tcr: __u32,
    pub decar: __u32,
    pub /: *mut *mut __u32 dec; / KVM_SREGS_E_UPDATE_DEC,
//
// Userspace can read TB directly, but the
// value reported here is consistent with "dec".
//
// Read-only.
//
    pub tb: __u64,
    pub /: *mut *mut __u32 dbsr; / KVM_SREGS_E_UPDATE_DBSR,
    pub dbcr: [__u32; 3],
//
// iac/dac registers are 64bit wide, while this API
// interface provides only lower 32 bits on 64 bit
// processors. ONE_REG interface is added for 64bit
// iac/dac registers.
//
    pub iac: [__u32; 4],
    pub dac: [__u32; 2],
    pub dvc: [__u32; 2],
    pub /: *mut *mut __u8 num_iac; / read-only,
    pub /: *mut *mut __u8 num_dac; / read-only,
    pub /: *mut *mut __u8 num_dvc; / read-only,
    pub pad: __u8,
    pub /: *mut *mut __u32 epr; / EXP,
    pub /: *mut *mut __u32 vrsave; / a.k.a. USPRG0,
    pub /: *mut *mut __u32 epcr; / KVM_SREGS_E_64,
    pub mas0: __u32,
    pub mas1: __u32,
    pub mas2: __u64,
    pub mas7_3: __u64,
    pub mas4: __u32,
    pub mas6: __u32,
    pub /: *mut *mut __u32 ivor_low[16]; / IVOR0-15,
    pub /: *mut *mut __u32 ivor_high[18]; / IVOR32+, plus room to expand,
    pub /: *mut *mut __u32 mmucfg; / read-only,
    pub /: *mut *mut __u32 eptcfg; / E.PT, read-only,
    pub /: *mut *mut __u32 tlbcfg[4];/ read-only,
    pub /: *mut *mut __u32 tlbps[4]; / read-only,
    pub /: *mut *mut __u32 eplc, epsc; / E.PD,
    pub e: },
    pub pad: [__u8; 1020],
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_fpu {
    pub fpr: [__u64; 32],
}

//
// Defines for h/w breakpoint, watchpoint (read, write or both) and
// software breakpoint.
// These are used as "type" in KVM_SET_GUEST_DEBUG ioctl and "status"
// for KVM_DEBUG_EXIT.
//
pub const KVMPPC_DEBUG_NONE: c_uint = 0x0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_debug_exit_arch {
    pub address: __u64,
//
// exiting to userspace because of h/w breakpoint, watchpoint
// (read, write or both) and software breakpoint.
//
    pub status: __u32,
    pub reserved: __u32,
}

// for KVM_SET_GUEST_DEBUG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_guest_debug_arch {
// H/W breakpoint/watchpoint address
    pub addr: __u64,
//
// Type denotes h/w breakpoint, read watchpoint, write
// watchpoint or watchpoint (both read and write).
//
    pub type: __u32,
    pub reserved: __u32,
    pub bp: [}; 16],
}

// Debug related defines
//
// kvm_guest_debug->control is a 32 bit field. The lower 16 bits are generic
// and upper 16 bits are architecture specific. Architecture specific defines
// that ioctl is for setting hardware breakpoint or software breakpoint.
//
pub const KVM_GUESTDBG_USE_SW_BP: c_uint = 0x00010000;
pub const KVM_GUESTDBG_USE_HW_BP: c_uint = 0x00020000;
// definition of registers in kvm_run
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sync_regs {
}

pub const KVM_CPU_440: c_int = 1;
pub const KVM_CPU_E500V2: c_int = 2;
pub const KVM_CPU_3S_32: c_int = 3;
pub const KVM_CPU_3S_64: c_int = 4;
pub const KVM_CPU_E500MC: c_int = 5;
// for KVM_CAP_SPAPR_TCE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_create_spapr_tce {
    pub liobn: __u64,
    pub window_size: __u32,
}

// for KVM_CAP_SPAPR_TCE_64
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_create_spapr_tce_64 {
    pub liobn: __u64,
    pub page_shift: __u32,
    pub flags: __u32,
    pub /: *mut *mut __u64 offset; / in pages,
    pub /: *mut *mut __u64 size; / in pages,
}

// for KVM_ALLOCATE_RMA
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_allocate_rma {
    pub rma_size: __u64,
}

// for KVM_CAP_PPC_RTAS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_rtas_token_args {
    pub name: [c_char; 120],
    pub /: *mut *mut __u64 token; / Use a token of 0 to undefine a mapping,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_book3e_206_tlb_entry {
    pub mas8: __u32,
    pub mas1: __u32,
    pub mas2: __u64,
    pub mas7_3: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_book3e_206_tlb_params {
//
// For mmu types KVM_MMU_FSL_BOOKE_NOHV and KVM_MMU_FSL_BOOKE_HV:
//
// - The number of ways of TLB0 must be a power of two between 2 and
// 16.
// - TLB1 must be fully associative.
// - The size of TLB0 must be a multiple of the number of ways, and
// the number of sets must be a power of two.
// - The size of TLB1 may not exceed 64 entries.
// - TLB0 supports 4 KiB pages.
// - The page sizes supported by TLB1 are as indicated by
// TLB1CFG (if MMUCFG[MAVN] = 0) or TLB1PS (if MMUCFG[MAVN] = 1)
// as returned by KVM_GET_SREGS.
// - TLB2 and TLB3 are reserved, and their entries in tlb_sizes[]
// and tlb_ways[] must be zero.
//
// tlb_ways[n] = tlb_sizes[n] means the array is fully associative.
//
// KVM will adjust TLBnCFG based on the sizes configured here,
// though arrays greater than 2048 entries will have TLBnCFG[NENTRY]
// set to zero.
//
    pub tlb_sizes: [__u32; 4],
    pub tlb_ways: [__u32; 4],
    pub reserved: [__u32; 8],
}

// For KVM_PPC_GET_HTAB_FD
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_get_htab_fd {
    pub flags: __u64,
    pub start_index: __u64,
    pub reserved: [__u64; 2],
}

// Values for kvm_get_htab_fd.flags

//
// Data read on the file descriptor is formatted as a series of
// records, each consisting of a header followed by a series of
// `n_valid' HPTEs (16 bytes each), which are all valid.  Following
// those valid HPTEs there are `n_invalid' invalid HPTEs, which
// are not represented explicitly in the stream.  The same format
// is used for writing.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_get_htab_header {
    pub index: __u32,
    pub n_valid: __u16,
    pub n_invalid: __u16,
}

// For KVM_PPC_CONFIGURE_V3_MMU
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ppc_mmuv3_cfg {
    pub flags: __u64,
    pub /: *mut *mut __u64 process_table; / second doubleword of partition table entry,
}

// Flag values for KVM_PPC_CONFIGURE_V3_MMU

// For KVM_PPC_GET_RMMU_INFO
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ppc_rmmu_info {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ppc_radix_geom {
    pub page_shift: __u8,
    pub level_bits: [__u8; 4],
    pub pad: [__u8; 3],
    pub geometries: [}; 8],
    pub ap_encodings: [__u32; 8],
}

// For KVM_PPC_GET_CPU_CHAR
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ppc_cpu_char {
    pub /: *mut *mut __u64 character; / characteristics of the CPU,
    pub /: *mut *mut __u64 behaviour; / recommended software behaviour,
    pub /: *mut *mut __u64 character_mask; / valid bits in character,
    pub /: *mut *mut __u64 behaviour_mask; / valid bits in behaviour,
}

//
// Values for character and character_mask.
// These are identical to the values used by H_GET_CPU_CHARACTERISTICS.
//

// Per-vcpu XICS interrupt controller state

pub const KVM_REG_PPC_ICP_CPPR_MASK: c_uint = 0xff;

pub const KVM_REG_PPC_ICP_XISR_MASK: c_uint = 0xffffff;

pub const KVM_REG_PPC_ICP_MFRR_MASK: c_uint = 0xff;

pub const KVM_REG_PPC_ICP_PPRI_MASK: c_uint = 0xff;

// Device control API: PPC-specific devices
pub const KVM_DEV_MPIC_GRP_MISC: c_int = 1;

// One-Reg API: PPC-specific registers

// 32 floating-point registers

// 32 VMX/Altivec vector registers

// 32 double-width FP registers for VSX
// High-order halves overlap with FP regs

// FP and vector status/control registers

//
// VSCR register is documented as a 32-bit register in the ISA, but it can
// only be accesses via a vector register. Expose VSCR as a 32-bit register
// even though the kernel represents it as a 128-bit vector.
//

// Virtual processor areas
// For SLB & DTL, address in high (first) half, length in low half

// Timer Status Register OR/CLEAR interface

// Debugging: Special instruction for software breakpoint

// MMU registers

//
// TLBnCFG fields TLBnCFG_N_ENTRY and TLBnCFG_ASSOC can be changed only using
// KVM_CAP_SW_TLB ioctl
//

// Timebase offset

// POWER8 registers

// Architecture compatibility level

// POWER9 registers

// POWER10 registers

// Transactional Memory checkpointed state:
// This is all GPRs, all VSX regs and a subset of SPRs
//

// TM GPRs

// TM VSX

// TM SPRS

// PPC64 eXternal Interrupt Controller Specification

pub const KVM_DEV_XICS_GRP_CTRL: c_int = 2;
pub const KVM_DEV_XICS_NR_SERVERS: c_int = 1;
// Layout of 64-bit source attribute values
pub const KVM_XICS_DESTINATION_SHIFT: c_int = 0;
pub const KVM_XICS_DESTINATION_MASK: c_uint = 0xffffffffULL;
pub const KVM_XICS_PRIORITY_SHIFT: c_int = 32;
pub const KVM_XICS_PRIORITY_MASK: c_uint = 0xff;

// POWER9 XIVE Native Interrupt Controller
pub const KVM_DEV_XIVE_GRP_CTRL: c_int = 1;
pub const KVM_DEV_XIVE_RESET: c_int = 1;
pub const KVM_DEV_XIVE_EQ_SYNC: c_int = 2;
pub const KVM_DEV_XIVE_NR_SERVERS: c_int = 3;

// Layout of 64-bit XIVE source attribute values

// Layout of 64-bit XIVE source configuration attribute values
pub const KVM_XIVE_SOURCE_PRIORITY_SHIFT: c_int = 0;
pub const KVM_XIVE_SOURCE_PRIORITY_MASK: c_uint = 0x7;
pub const KVM_XIVE_SOURCE_SERVER_SHIFT: c_int = 3;
pub const KVM_XIVE_SOURCE_SERVER_MASK: c_uint = 0xfffffff8ULL;
pub const KVM_XIVE_SOURCE_MASKED_SHIFT: c_int = 32;
pub const KVM_XIVE_SOURCE_MASKED_MASK: c_uint = 0x100000000ULL;
pub const KVM_XIVE_SOURCE_EISN_SHIFT: c_int = 33;
pub const KVM_XIVE_SOURCE_EISN_MASK: c_uint = 0xfffffffe00000000ULL;
// Layout of 64-bit EQ identifier
pub const KVM_XIVE_EQ_PRIORITY_SHIFT: c_int = 0;
pub const KVM_XIVE_EQ_PRIORITY_MASK: c_uint = 0x7;
pub const KVM_XIVE_EQ_SERVER_SHIFT: c_int = 3;
pub const KVM_XIVE_EQ_SERVER_MASK: c_uint = 0xfffffff8ULL;
// Layout of EQ configuration values (64 bytes)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ppc_xive_eq {
    pub flags: __u32,
    pub qshift: __u32,
    pub qaddr: __u64,
    pub qtoggle: __u32,
    pub qindex: __u32,
    pub pad: [__u8; 40],
}

pub const KVM_XIVE_EQ_ALWAYS_NOTIFY: c_uint = 0x00000001;
pub const KVM_XIVE_TIMA_PAGE_OFFSET: c_int = 0;
pub const KVM_XIVE_ESB_PAGE_OFFSET: c_int = 4;
// for KVM_PPC_GET_PVINFO

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ppc_pvinfo {
// out
    pub flags: __u32,
    pub hcall: [__u32; 4],
    pub pad: [__u8; 108],
}

// for KVM_PPC_GET_SMMU_INFO
pub const KVM_PPC_PAGE_SIZES_MAX_SZ: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ppc_one_page_size {
    pub /: *mut *mut __u32 page_shift; / Page shift (or 0),
    pub /: *mut *mut __u32 pte_enc; / Encoding in the HPTE (>>12),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ppc_one_seg_page_size {
    pub /: *mut *mut __u32 page_shift; / Base page shift of segment (or 0),
    pub /: *mut *mut __u32 slb_enc; / SLB encoding for BookS,
    pub enc: [kvm_ppc_one_page_size; KVM_PPC_PAGE_SIZES_MAX_SZ],
}

pub const KVM_PPC_PAGE_SIZES_REAL: c_uint = 0x00000001;
pub const KVM_PPC_1T_SEGMENTS: c_uint = 0x00000002;
pub const KVM_PPC_NO_HASH: c_uint = 0x00000004;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ppc_smmu_info {
    pub flags: __u64,
    pub slb_size: __u32,
    pub /: *mut *mut __u16 data_keys; / # storage keys supported for data,
    pub /: *mut *mut __u16 instr_keys; / # storage keys supported for instructions,
    pub sps: [kvm_ppc_one_seg_page_size; KVM_PPC_PAGE_SIZES_MAX_SZ],
}

// for KVM_PPC_RESIZE_HPT_{PREPARE,COMMIT}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ppc_resize_hpt {
    pub flags: __u64,
    pub shift: __u32,
    pub pad: __u32,
}
