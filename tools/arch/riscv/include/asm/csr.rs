//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/riscv/include/asm/csr.h
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
// Copyright (C) 2015 Regents of the University of California
//

// Status register flags

// SATP flags

pub const SATP_MODE_SHIFT: c_int = 31;
pub const SATP_ASID_BITS: c_int = 9;
pub const SATP_ASID_SHIFT: c_int = 22;

pub const SATP_MODE_SHIFT: c_int = 60;
pub const SATP_ASID_BITS: c_int = 16;
pub const SATP_ASID_SHIFT: c_int = 44;

// Exception cause high bit - is an interrupt if set

// Interrupt causes (minus the high bit)
pub const IRQ_S_SOFT: c_int = 1;
pub const IRQ_VS_SOFT: c_int = 2;
pub const IRQ_M_SOFT: c_int = 3;
pub const IRQ_S_TIMER: c_int = 5;
pub const IRQ_VS_TIMER: c_int = 6;
pub const IRQ_M_TIMER: c_int = 7;
pub const IRQ_S_EXT: c_int = 9;
pub const IRQ_VS_EXT: c_int = 10;
pub const IRQ_M_EXT: c_int = 11;
pub const IRQ_S_GEXT: c_int = 12;
pub const IRQ_PMU_OVF: c_int = 13;

// Exception causes
pub const EXC_INST_MISALIGNED: c_int = 0;
pub const EXC_INST_ACCESS: c_int = 1;
pub const EXC_INST_ILLEGAL: c_int = 2;
pub const EXC_BREAKPOINT: c_int = 3;
pub const EXC_LOAD_MISALIGNED: c_int = 4;
pub const EXC_LOAD_ACCESS: c_int = 5;
pub const EXC_STORE_MISALIGNED: c_int = 6;
pub const EXC_STORE_ACCESS: c_int = 7;
pub const EXC_SYSCALL: c_int = 8;
pub const EXC_HYPERVISOR_SYSCALL: c_int = 9;
pub const EXC_SUPERVISOR_SYSCALL: c_int = 10;
pub const EXC_INST_PAGE_FAULT: c_int = 12;
pub const EXC_LOAD_PAGE_FAULT: c_int = 13;
pub const EXC_STORE_PAGE_FAULT: c_int = 15;
pub const EXC_INST_GUEST_PAGE_FAULT: c_int = 20;
pub const EXC_LOAD_GUEST_PAGE_FAULT: c_int = 21;
pub const EXC_VIRTUAL_INST_FAULT: c_int = 22;
pub const EXC_STORE_GUEST_PAGE_FAULT: c_int = 23;
// PMP configuration
pub const PMP_R: c_uint = 0x01;
pub const PMP_W: c_uint = 0x02;
pub const PMP_X: c_uint = 0x04;
pub const PMP_A: c_uint = 0x18;
pub const PMP_A_TOR: c_uint = 0x08;
pub const PMP_A_NA4: c_uint = 0x10;
pub const PMP_A_NAPOT: c_uint = 0x18;
pub const PMP_L: c_uint = 0x80;
// HSTATUS flags

pub const HSTATUS_VSXL_SHIFT: c_int = 32;

pub const HSTATUS_VGEIN_SHIFT: c_int = 12;

// HGATP flags

pub const HGATP32_MODE_SHIFT: c_int = 31;
pub const HGATP32_VMID_SHIFT: c_int = 22;

pub const HGATP64_MODE_SHIFT: c_int = 60;
pub const HGATP64_VMID_SHIFT: c_int = 44;

pub const HGATP_PAGE_SHIFT: c_int = 12;

// VSIP & HVIP relation

// AIA CSR bits
pub const TOPI_IID_SHIFT: c_int = 16;

pub const TOPI_IPRIO_BITS: c_int = 8;
pub const TOPEI_ID_SHIFT: c_int = 16;

pub const ISELECT_IPRIO0: c_uint = 0x30;
pub const ISELECT_IPRIO15: c_uint = 0x3f;

pub const HVICTL_IID_SHIFT: c_int = 16;

// xENVCFG flags

pub const ENVCFG_CBIE_SHIFT: c_int = 4;

// Smstateen bits
pub const SMSTATEEN0_AIA_IMSIC_SHIFT: c_int = 58;

pub const SMSTATEEN0_AIA_SHIFT: c_int = 59;

pub const SMSTATEEN0_AIA_ISEL_SHIFT: c_int = 60;

pub const SMSTATEEN0_HSENVCFG_SHIFT: c_int = 62;

pub const SMSTATEEN0_SSTATEEN0_SHIFT: c_int = 63;

// symbolic CSR names:
pub const CSR_CYCLE: c_uint = 0xc00;
pub const CSR_TIME: c_uint = 0xc01;
pub const CSR_INSTRET: c_uint = 0xc02;
pub const CSR_HPMCOUNTER3: c_uint = 0xc03;
pub const CSR_HPMCOUNTER4: c_uint = 0xc04;
pub const CSR_HPMCOUNTER5: c_uint = 0xc05;
pub const CSR_HPMCOUNTER6: c_uint = 0xc06;
pub const CSR_HPMCOUNTER7: c_uint = 0xc07;
pub const CSR_HPMCOUNTER8: c_uint = 0xc08;
pub const CSR_HPMCOUNTER9: c_uint = 0xc09;
pub const CSR_HPMCOUNTER10: c_uint = 0xc0a;
pub const CSR_HPMCOUNTER11: c_uint = 0xc0b;
pub const CSR_HPMCOUNTER12: c_uint = 0xc0c;
pub const CSR_HPMCOUNTER13: c_uint = 0xc0d;
pub const CSR_HPMCOUNTER14: c_uint = 0xc0e;
pub const CSR_HPMCOUNTER15: c_uint = 0xc0f;
pub const CSR_HPMCOUNTER16: c_uint = 0xc10;
pub const CSR_HPMCOUNTER17: c_uint = 0xc11;
pub const CSR_HPMCOUNTER18: c_uint = 0xc12;
pub const CSR_HPMCOUNTER19: c_uint = 0xc13;
pub const CSR_HPMCOUNTER20: c_uint = 0xc14;
pub const CSR_HPMCOUNTER21: c_uint = 0xc15;
pub const CSR_HPMCOUNTER22: c_uint = 0xc16;
pub const CSR_HPMCOUNTER23: c_uint = 0xc17;
pub const CSR_HPMCOUNTER24: c_uint = 0xc18;
pub const CSR_HPMCOUNTER25: c_uint = 0xc19;
pub const CSR_HPMCOUNTER26: c_uint = 0xc1a;
pub const CSR_HPMCOUNTER27: c_uint = 0xc1b;
pub const CSR_HPMCOUNTER28: c_uint = 0xc1c;
pub const CSR_HPMCOUNTER29: c_uint = 0xc1d;
pub const CSR_HPMCOUNTER30: c_uint = 0xc1e;
pub const CSR_HPMCOUNTER31: c_uint = 0xc1f;
pub const CSR_CYCLEH: c_uint = 0xc80;
pub const CSR_TIMEH: c_uint = 0xc81;
pub const CSR_INSTRETH: c_uint = 0xc82;
pub const CSR_HPMCOUNTER3H: c_uint = 0xc83;
pub const CSR_HPMCOUNTER4H: c_uint = 0xc84;
pub const CSR_HPMCOUNTER5H: c_uint = 0xc85;
pub const CSR_HPMCOUNTER6H: c_uint = 0xc86;
pub const CSR_HPMCOUNTER7H: c_uint = 0xc87;
pub const CSR_HPMCOUNTER8H: c_uint = 0xc88;
pub const CSR_HPMCOUNTER9H: c_uint = 0xc89;
pub const CSR_HPMCOUNTER10H: c_uint = 0xc8a;
pub const CSR_HPMCOUNTER11H: c_uint = 0xc8b;
pub const CSR_HPMCOUNTER12H: c_uint = 0xc8c;
pub const CSR_HPMCOUNTER13H: c_uint = 0xc8d;
pub const CSR_HPMCOUNTER14H: c_uint = 0xc8e;
pub const CSR_HPMCOUNTER15H: c_uint = 0xc8f;
pub const CSR_HPMCOUNTER16H: c_uint = 0xc90;
pub const CSR_HPMCOUNTER17H: c_uint = 0xc91;
pub const CSR_HPMCOUNTER18H: c_uint = 0xc92;
pub const CSR_HPMCOUNTER19H: c_uint = 0xc93;
pub const CSR_HPMCOUNTER20H: c_uint = 0xc94;
pub const CSR_HPMCOUNTER21H: c_uint = 0xc95;
pub const CSR_HPMCOUNTER22H: c_uint = 0xc96;
pub const CSR_HPMCOUNTER23H: c_uint = 0xc97;
pub const CSR_HPMCOUNTER24H: c_uint = 0xc98;
pub const CSR_HPMCOUNTER25H: c_uint = 0xc99;
pub const CSR_HPMCOUNTER26H: c_uint = 0xc9a;
pub const CSR_HPMCOUNTER27H: c_uint = 0xc9b;
pub const CSR_HPMCOUNTER28H: c_uint = 0xc9c;
pub const CSR_HPMCOUNTER29H: c_uint = 0xc9d;
pub const CSR_HPMCOUNTER30H: c_uint = 0xc9e;
pub const CSR_HPMCOUNTER31H: c_uint = 0xc9f;
pub const CSR_SCOUNTOVF: c_uint = 0xda0;
pub const CSR_SSTATUS: c_uint = 0x100;
pub const CSR_SIE: c_uint = 0x104;
pub const CSR_STVEC: c_uint = 0x105;
pub const CSR_SCOUNTEREN: c_uint = 0x106;
pub const CSR_SENVCFG: c_uint = 0x10a;
pub const CSR_SSTATEEN0: c_uint = 0x10c;
pub const CSR_SSCRATCH: c_uint = 0x140;
pub const CSR_SEPC: c_uint = 0x141;
pub const CSR_SCAUSE: c_uint = 0x142;
pub const CSR_STVAL: c_uint = 0x143;
pub const CSR_SIP: c_uint = 0x144;
pub const CSR_SATP: c_uint = 0x180;
pub const CSR_STIMECMP: c_uint = 0x14D;
pub const CSR_STIMECMPH: c_uint = 0x15D;
// Supervisor-Level Window to Indirectly Accessed Registers (AIA)
pub const CSR_SISELECT: c_uint = 0x150;
pub const CSR_SIREG: c_uint = 0x151;
// Supervisor-Level Interrupts (AIA)
pub const CSR_STOPEI: c_uint = 0x15c;
pub const CSR_STOPI: c_uint = 0xdb0;
// Supervisor-Level High-Half CSRs (AIA)
pub const CSR_SIEH: c_uint = 0x114;
pub const CSR_SIPH: c_uint = 0x154;
pub const CSR_VSSTATUS: c_uint = 0x200;
pub const CSR_VSIE: c_uint = 0x204;
pub const CSR_VSTVEC: c_uint = 0x205;
pub const CSR_VSSCRATCH: c_uint = 0x240;
pub const CSR_VSEPC: c_uint = 0x241;
pub const CSR_VSCAUSE: c_uint = 0x242;
pub const CSR_VSTVAL: c_uint = 0x243;
pub const CSR_VSIP: c_uint = 0x244;
pub const CSR_VSATP: c_uint = 0x280;
pub const CSR_VSTIMECMP: c_uint = 0x24D;
pub const CSR_VSTIMECMPH: c_uint = 0x25D;
pub const CSR_HSTATUS: c_uint = 0x600;
pub const CSR_HEDELEG: c_uint = 0x602;
pub const CSR_HIDELEG: c_uint = 0x603;
pub const CSR_HIE: c_uint = 0x604;
pub const CSR_HTIMEDELTA: c_uint = 0x605;
pub const CSR_HCOUNTEREN: c_uint = 0x606;
pub const CSR_HGEIE: c_uint = 0x607;
pub const CSR_HENVCFG: c_uint = 0x60a;
pub const CSR_HTIMEDELTAH: c_uint = 0x615;
pub const CSR_HENVCFGH: c_uint = 0x61a;
pub const CSR_HTVAL: c_uint = 0x643;
pub const CSR_HIP: c_uint = 0x644;
pub const CSR_HVIP: c_uint = 0x645;
pub const CSR_HTINST: c_uint = 0x64a;
pub const CSR_HGATP: c_uint = 0x680;
pub const CSR_HGEIP: c_uint = 0xe12;
// Virtual Interrupts and Interrupt Priorities (H-extension with AIA)
pub const CSR_HVIEN: c_uint = 0x608;
pub const CSR_HVICTL: c_uint = 0x609;
pub const CSR_HVIPRIO1: c_uint = 0x646;
pub const CSR_HVIPRIO2: c_uint = 0x647;
// VS-Level Window to Indirectly Accessed Registers (H-extension with AIA)
pub const CSR_VSISELECT: c_uint = 0x250;
pub const CSR_VSIREG: c_uint = 0x251;
// VS-Level Interrupts (H-extension with AIA)
pub const CSR_VSTOPEI: c_uint = 0x25c;
pub const CSR_VSTOPI: c_uint = 0xeb0;
// Hypervisor and VS-Level High-Half CSRs (H-extension with AIA)
pub const CSR_HIDELEGH: c_uint = 0x613;
pub const CSR_HVIENH: c_uint = 0x618;
pub const CSR_HVIPH: c_uint = 0x655;
pub const CSR_HVIPRIO1H: c_uint = 0x656;
pub const CSR_HVIPRIO2H: c_uint = 0x657;
pub const CSR_VSIEH: c_uint = 0x214;
pub const CSR_VSIPH: c_uint = 0x254;
// Hypervisor stateen CSRs
pub const CSR_HSTATEEN0: c_uint = 0x60c;
pub const CSR_HSTATEEN0H: c_uint = 0x61c;
pub const CSR_MSTATUS: c_uint = 0x300;
pub const CSR_MISA: c_uint = 0x301;
pub const CSR_MIDELEG: c_uint = 0x303;
pub const CSR_MIE: c_uint = 0x304;
pub const CSR_MTVEC: c_uint = 0x305;
pub const CSR_MENVCFG: c_uint = 0x30a;
pub const CSR_MENVCFGH: c_uint = 0x31a;
pub const CSR_MSCRATCH: c_uint = 0x340;
pub const CSR_MEPC: c_uint = 0x341;
pub const CSR_MCAUSE: c_uint = 0x342;
pub const CSR_MTVAL: c_uint = 0x343;
pub const CSR_MIP: c_uint = 0x344;
pub const CSR_PMPCFG0: c_uint = 0x3a0;
pub const CSR_PMPADDR0: c_uint = 0x3b0;
pub const CSR_MVENDORID: c_uint = 0xf11;
pub const CSR_MARCHID: c_uint = 0xf12;
pub const CSR_MIMPID: c_uint = 0xf13;
pub const CSR_MHARTID: c_uint = 0xf14;
// Machine-Level Window to Indirectly Accessed Registers (AIA)
pub const CSR_MISELECT: c_uint = 0x350;
pub const CSR_MIREG: c_uint = 0x351;
// Machine-Level Interrupts (AIA)
pub const CSR_MTOPEI: c_uint = 0x35c;
pub const CSR_MTOPI: c_uint = 0xfb0;
// Virtual Interrupts for Supervisor Level (AIA)
pub const CSR_MVIEN: c_uint = 0x308;
pub const CSR_MVIP: c_uint = 0x309;
// Machine-Level High-Half CSRs (AIA)
pub const CSR_MIDELEGH: c_uint = 0x313;
pub const CSR_MIEH: c_uint = 0x314;
pub const CSR_MVIENH: c_uint = 0x318;
pub const CSR_MVIPH: c_uint = 0x319;
pub const CSR_MIPH: c_uint = 0x354;
pub const CSR_VSTART: c_uint = 0x8;
pub const CSR_VCSR: c_uint = 0xf;
pub const CSR_VL: c_uint = 0xc20;
pub const CSR_VTYPE: c_uint = 0xc21;
pub const CSR_VLENB: c_uint = 0xc22;

// IE/IP (Supervisor/Machine Interrupt Enable/Pending) flags

