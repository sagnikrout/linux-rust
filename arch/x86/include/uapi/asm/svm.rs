//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/uapi/asm/svm.h
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
pub const SVM_EXIT_READ_CR0: c_uint = 0x000;
pub const SVM_EXIT_READ_CR2: c_uint = 0x002;
pub const SVM_EXIT_READ_CR3: c_uint = 0x003;
pub const SVM_EXIT_READ_CR4: c_uint = 0x004;
pub const SVM_EXIT_READ_CR8: c_uint = 0x008;
pub const SVM_EXIT_WRITE_CR0: c_uint = 0x010;
pub const SVM_EXIT_WRITE_CR2: c_uint = 0x012;
pub const SVM_EXIT_WRITE_CR3: c_uint = 0x013;
pub const SVM_EXIT_WRITE_CR4: c_uint = 0x014;
pub const SVM_EXIT_WRITE_CR8: c_uint = 0x018;
pub const SVM_EXIT_READ_DR0: c_uint = 0x020;
pub const SVM_EXIT_READ_DR1: c_uint = 0x021;
pub const SVM_EXIT_READ_DR2: c_uint = 0x022;
pub const SVM_EXIT_READ_DR3: c_uint = 0x023;
pub const SVM_EXIT_READ_DR4: c_uint = 0x024;
pub const SVM_EXIT_READ_DR5: c_uint = 0x025;
pub const SVM_EXIT_READ_DR6: c_uint = 0x026;
pub const SVM_EXIT_READ_DR7: c_uint = 0x027;
pub const SVM_EXIT_WRITE_DR0: c_uint = 0x030;
pub const SVM_EXIT_WRITE_DR1: c_uint = 0x031;
pub const SVM_EXIT_WRITE_DR2: c_uint = 0x032;
pub const SVM_EXIT_WRITE_DR3: c_uint = 0x033;
pub const SVM_EXIT_WRITE_DR4: c_uint = 0x034;
pub const SVM_EXIT_WRITE_DR5: c_uint = 0x035;
pub const SVM_EXIT_WRITE_DR6: c_uint = 0x036;
pub const SVM_EXIT_WRITE_DR7: c_uint = 0x037;
pub const SVM_EXIT_EXCP_BASE: c_uint = 0x040;
pub const SVM_EXIT_LAST_EXCP: c_uint = 0x05f;
pub const SVM_EXIT_INTR: c_uint = 0x060;
pub const SVM_EXIT_NMI: c_uint = 0x061;
pub const SVM_EXIT_SMI: c_uint = 0x062;
pub const SVM_EXIT_INIT: c_uint = 0x063;
pub const SVM_EXIT_VINTR: c_uint = 0x064;
pub const SVM_EXIT_CR0_SEL_WRITE: c_uint = 0x065;
pub const SVM_EXIT_IDTR_READ: c_uint = 0x066;
pub const SVM_EXIT_GDTR_READ: c_uint = 0x067;
pub const SVM_EXIT_LDTR_READ: c_uint = 0x068;
pub const SVM_EXIT_TR_READ: c_uint = 0x069;
pub const SVM_EXIT_IDTR_WRITE: c_uint = 0x06a;
pub const SVM_EXIT_GDTR_WRITE: c_uint = 0x06b;
pub const SVM_EXIT_LDTR_WRITE: c_uint = 0x06c;
pub const SVM_EXIT_TR_WRITE: c_uint = 0x06d;
pub const SVM_EXIT_RDTSC: c_uint = 0x06e;
pub const SVM_EXIT_RDPMC: c_uint = 0x06f;
pub const SVM_EXIT_PUSHF: c_uint = 0x070;
pub const SVM_EXIT_POPF: c_uint = 0x071;
pub const SVM_EXIT_CPUID: c_uint = 0x072;
pub const SVM_EXIT_RSM: c_uint = 0x073;
pub const SVM_EXIT_IRET: c_uint = 0x074;
pub const SVM_EXIT_SWINT: c_uint = 0x075;
pub const SVM_EXIT_INVD: c_uint = 0x076;
pub const SVM_EXIT_PAUSE: c_uint = 0x077;
pub const SVM_EXIT_HLT: c_uint = 0x078;
pub const SVM_EXIT_INVLPG: c_uint = 0x079;
pub const SVM_EXIT_INVLPGA: c_uint = 0x07a;
pub const SVM_EXIT_IOIO: c_uint = 0x07b;
pub const SVM_EXIT_MSR: c_uint = 0x07c;
pub const SVM_EXIT_TASK_SWITCH: c_uint = 0x07d;
pub const SVM_EXIT_FERR_FREEZE: c_uint = 0x07e;
pub const SVM_EXIT_SHUTDOWN: c_uint = 0x07f;
pub const SVM_EXIT_VMRUN: c_uint = 0x080;
pub const SVM_EXIT_VMMCALL: c_uint = 0x081;
pub const SVM_EXIT_VMLOAD: c_uint = 0x082;
pub const SVM_EXIT_VMSAVE: c_uint = 0x083;
pub const SVM_EXIT_STGI: c_uint = 0x084;
pub const SVM_EXIT_CLGI: c_uint = 0x085;
pub const SVM_EXIT_SKINIT: c_uint = 0x086;
pub const SVM_EXIT_RDTSCP: c_uint = 0x087;
pub const SVM_EXIT_ICEBP: c_uint = 0x088;
pub const SVM_EXIT_WBINVD: c_uint = 0x089;
pub const SVM_EXIT_MONITOR: c_uint = 0x08a;
pub const SVM_EXIT_MWAIT: c_uint = 0x08b;
pub const SVM_EXIT_MWAIT_COND: c_uint = 0x08c;
pub const SVM_EXIT_XSETBV: c_uint = 0x08d;
pub const SVM_EXIT_RDPRU: c_uint = 0x08e;
pub const SVM_EXIT_EFER_WRITE_TRAP: c_uint = 0x08f;
pub const SVM_EXIT_CR0_WRITE_TRAP: c_uint = 0x090;
pub const SVM_EXIT_CR1_WRITE_TRAP: c_uint = 0x091;
pub const SVM_EXIT_CR2_WRITE_TRAP: c_uint = 0x092;
pub const SVM_EXIT_CR3_WRITE_TRAP: c_uint = 0x093;
pub const SVM_EXIT_CR4_WRITE_TRAP: c_uint = 0x094;
pub const SVM_EXIT_CR5_WRITE_TRAP: c_uint = 0x095;
pub const SVM_EXIT_CR6_WRITE_TRAP: c_uint = 0x096;
pub const SVM_EXIT_CR7_WRITE_TRAP: c_uint = 0x097;
pub const SVM_EXIT_CR8_WRITE_TRAP: c_uint = 0x098;
pub const SVM_EXIT_CR9_WRITE_TRAP: c_uint = 0x099;
pub const SVM_EXIT_CR10_WRITE_TRAP: c_uint = 0x09a;
pub const SVM_EXIT_CR11_WRITE_TRAP: c_uint = 0x09b;
pub const SVM_EXIT_CR12_WRITE_TRAP: c_uint = 0x09c;
pub const SVM_EXIT_CR13_WRITE_TRAP: c_uint = 0x09d;
pub const SVM_EXIT_CR14_WRITE_TRAP: c_uint = 0x09e;
pub const SVM_EXIT_CR15_WRITE_TRAP: c_uint = 0x09f;
pub const SVM_EXIT_INVPCID: c_uint = 0x0a2;
pub const SVM_EXIT_BUS_LOCK: c_uint = 0x0a5;
pub const SVM_EXIT_IDLE_HLT: c_uint = 0x0a6;
pub const SVM_EXIT_NPF: c_uint = 0x400;
pub const SVM_EXIT_AVIC_INCOMPLETE_IPI: c_uint = 0x401;
pub const SVM_EXIT_AVIC_UNACCELERATED_ACCESS: c_uint = 0x402;
pub const SVM_EXIT_VMGEXIT: c_uint = 0x403;
// SEV-ES software-defined VMGEXIT events
pub const SVM_VMGEXIT_MMIO_READ: c_uint = 0x80000001ull;
pub const SVM_VMGEXIT_MMIO_WRITE: c_uint = 0x80000002ull;
pub const SVM_VMGEXIT_NMI_COMPLETE: c_uint = 0x80000003ull;
pub const SVM_VMGEXIT_AP_HLT_LOOP: c_uint = 0x80000004ull;
pub const SVM_VMGEXIT_AP_JUMP_TABLE: c_uint = 0x80000005ull;
pub const SVM_VMGEXIT_SET_AP_JUMP_TABLE: c_int = 0;
pub const SVM_VMGEXIT_GET_AP_JUMP_TABLE: c_int = 1;
pub const SVM_VMGEXIT_PSC: c_uint = 0x80000010ull;
pub const SVM_VMGEXIT_GUEST_REQUEST: c_uint = 0x80000011ull;
pub const SVM_VMGEXIT_EXT_GUEST_REQUEST: c_uint = 0x80000012ull;
pub const SVM_VMGEXIT_AP_CREATION: c_uint = 0x80000013ull;
pub const SVM_VMGEXIT_AP_CREATE_ON_INIT: c_int = 0;
pub const SVM_VMGEXIT_AP_CREATE: c_int = 1;
pub const SVM_VMGEXIT_AP_DESTROY: c_int = 2;
pub const SVM_VMGEXIT_SNP_RUN_VMPL: c_uint = 0x80000018ull;
pub const SVM_VMGEXIT_SAVIC: c_uint = 0x8000001aull;
pub const SVM_VMGEXIT_SAVIC_REGISTER_GPA: c_int = 0;
pub const SVM_VMGEXIT_SAVIC_UNREGISTER_GPA: c_int = 1;

pub const SVM_VMGEXIT_HV_FEATURES: c_uint = 0x8000fffdull;
pub const SVM_VMGEXIT_TERM_REQUEST: c_uint = 0x8000fffeull;

// SW_EXITINFO1[3:0] */					\
// SW_EXITINFO1[11:4] */				\
pub const SVM_VMGEXIT_UNSUPPORTED_EVENT: c_uint = 0x8000ffffull;
// Exit code reserved for hypervisor/software use
pub const SVM_EXIT_SW: c_uint = 0xf0000000ull;

