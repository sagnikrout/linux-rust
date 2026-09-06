//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/svm.h
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


// SPDX-License-Identifier: GPL-2.0

//
// 32-bit intercept words in the VMCB Control Area, starting
// at Byte offset 000h.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intercept_words {
    INTERCEPT_CR = 0,
    INTERCEPT_DR,
    INTERCEPT_EXCEPTION,
    INTERCEPT_WORD3,
    INTERCEPT_WORD4,
    INTERCEPT_WORD5,
    MAX_INTERCEPT,
}

// Byte offset 000h (word 0)
// Byte offset 004h (word 1)
// Byte offset 008h (word 2)
// Byte offset 00Ch (word 3)
// Byte offset 010h (word 4)
// Byte offset 014h (word 5)
//
// Offset 0x3e0, 32 bytes reserved
// for use by hypervisor/software.
//
pub const TLB_CONTROL_DO_NOTHING: c_int = 0;
pub const TLB_CONTROL_FLUSH_ALL_ASID: c_int = 1;
pub const TLB_CONTROL_FLUSH_ASID: c_int = 3;
pub const TLB_CONTROL_FLUSH_ASID_LOCAL: c_int = 7;

pub const V_TPR_MASK: c_uint = 0x0f;
pub const V_IRQ_SHIFT: c_int = 8;

pub const V_GIF_SHIFT: c_int = 9;

pub const V_NMI_PENDING_SHIFT: c_int = 11;

pub const V_NMI_BLOCKING_SHIFT: c_int = 12;

pub const V_INTR_PRIO_SHIFT: c_int = 16;

pub const V_IGN_TPR_SHIFT: c_int = 20;

pub const V_INTR_MASKING_SHIFT: c_int = 24;

pub const V_GIF_ENABLE_SHIFT: c_int = 25;

pub const V_NMI_ENABLE_SHIFT: c_int = 26;

pub const AVIC_ENABLE_SHIFT: c_int = 31;

pub const X2APIC_MODE_SHIFT: c_int = 30;

pub const SVM_IOIO_STR_SHIFT: c_int = 2;
pub const SVM_IOIO_REP_SHIFT: c_int = 3;
pub const SVM_IOIO_SIZE_SHIFT: c_int = 4;
pub const SVM_IOIO_ASIZE_SHIFT: c_int = 7;
pub const SVM_IOIO_TYPE_MASK: c_int = 1;

pub const SVM_TSC_RATIO_RSVD: c_uint = 0xffffff0000000000ULL;
pub const SVM_TSC_RATIO_MIN: c_uint = 0x0000000000000001ULL;
pub const SVM_TSC_RATIO_MAX: c_uint = 0x000000ffffffffffULL;
pub const SVM_TSC_RATIO_DEFAULT: c_uint = 0x0100000000ULL;
// AVIC

pub const AVIC_LOGICAL_ID_ENTRY_VALID_BIT: c_int = 31;

//
// GA_LOG_INTR is a synthetic flag that's never propagated to hardware-visible
// tables.  GA_LOG_INTR is set if the vCPU needs device posted IRQs to generate
// GA log interrupts to wake the vCPU (because it's blocking or about to block).
//

pub const AVIC_UNACCEL_ACCESS_WRITE_MASK: c_int = 1;
pub const AVIC_UNACCEL_ACCESS_OFFSET_MASK: c_uint = 0xFF0;
pub const AVIC_UNACCEL_ACCESS_VECTOR_MASK: c_uint = 0xFFFFFFFF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avic_ipi_failure_cause {
    AVIC_IPI_FAILURE_INVALID_INT_TYPE,
    AVIC_IPI_FAILURE_TARGET_NOT_RUNNING,
    AVIC_IPI_FAILURE_INVALID_TARGET,
    AVIC_IPI_FAILURE_INVALID_BACKING_PAGE,
    AVIC_IPI_FAILURE_INVALID_IPI_VECTOR,
}

//
// For AVIC, the max index allowed for physical APIC ID table is 0xfe (254), as
// 0xff is a broadcast to all CPUs, i.e. can't be targeted individually.
//

//
// For x2AVIC, the max index allowed for physical APIC ID table is 0x1ff (511).
// With X86_FEATURE_X2AVIC_EXT, the max index is increased to 0xfff (4095).
//
pub const X2AVIC_MAX_PHYSICAL_ID: c_uint = 0x1FFUL;
pub const X2AVIC_4K_MAX_PHYSICAL_ID: c_uint = 0xFFFUL;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmcb_seg {
    pub selector: u16,
    pub attrib: u16,
    pub limit: u32,
    pub base: u64,
    pub __packed: },
// Save area definition for legacy and SEV-MEM guests
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmcb_save_area {
    pub es: vmcb_seg,
    pub cs: vmcb_seg,
    pub ss: vmcb_seg,
    pub ds: vmcb_seg,
    pub fs: vmcb_seg,
    pub gs: vmcb_seg,
    pub gdtr: vmcb_seg,
    pub ldtr: vmcb_seg,
    pub idtr: vmcb_seg,
    pub tr: vmcb_seg,
// Reserved fields are named following their struct offset
    pub reserved_0xa0: [u8; 42],
    pub vmpl: u8,
    pub cpl: u8,
    pub reserved_0xcc: [u8; 4],
    pub efer: u64,
    pub reserved_0xd8: [u8; 112],
    pub cr4: u64,
    pub cr3: u64,
    pub cr0: u64,
    pub dr7: u64,
    pub dr6: u64,
    pub rflags: u64,
    pub rip: u64,
    pub reserved_0x180: [u8; 88],
    pub rsp: u64,
    pub s_cet: u64,
    pub ssp: u64,
    pub isst_addr: u64,
    pub rax: u64,
    pub star: u64,
    pub lstar: u64,
    pub cstar: u64,
    pub sfmask: u64,
    pub kernel_gs_base: u64,
    pub sysenter_cs: u64,
    pub sysenter_esp: u64,
    pub sysenter_eip: u64,
    pub cr2: u64,
    pub reserved_0x248: [u8; 32],
    pub g_pat: u64,
    pub dbgctl: u64,
    pub br_from: u64,
    pub br_to: u64,
    pub last_excp_from: u64,
    pub last_excp_to: u64,
    pub reserved_0x298: [u8; 72],
    pub /: *mut *mut u64 spec_ctrl; / Guest version of SPEC_CTRL at 0x2E0,
    pub __packed: },
// Save area definition for SEV-ES and SEV-SNP guests
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_es_save_area {
    pub es: vmcb_seg,
    pub cs: vmcb_seg,
    pub ss: vmcb_seg,
    pub ds: vmcb_seg,
    pub fs: vmcb_seg,
    pub gs: vmcb_seg,
    pub gdtr: vmcb_seg,
    pub ldtr: vmcb_seg,
    pub idtr: vmcb_seg,
    pub tr: vmcb_seg,
    pub pl0_ssp: u64,
    pub pl1_ssp: u64,
    pub pl2_ssp: u64,
    pub pl3_ssp: u64,
    pub u_cet: u64,
    pub reserved_0xc8: [u8; 2],
    pub vmpl: u8,
    pub cpl: u8,
    pub reserved_0xcc: [u8; 4],
    pub efer: u64,
    pub reserved_0xd8: [u8; 104],
    pub xss: u64,
    pub cr4: u64,
    pub cr3: u64,
    pub cr0: u64,
    pub dr7: u64,
    pub dr6: u64,
    pub rflags: u64,
    pub rip: u64,
    pub dr0: u64,
    pub dr1: u64,
    pub dr2: u64,
    pub dr3: u64,
    pub dr0_addr_mask: u64,
    pub dr1_addr_mask: u64,
    pub dr2_addr_mask: u64,
    pub dr3_addr_mask: u64,
    pub reserved_0x1c0: [u8; 24],
    pub rsp: u64,
    pub s_cet: u64,
    pub ssp: u64,
    pub isst_addr: u64,
    pub rax: u64,
    pub star: u64,
    pub lstar: u64,
    pub cstar: u64,
    pub sfmask: u64,
    pub kernel_gs_base: u64,
    pub sysenter_cs: u64,
    pub sysenter_esp: u64,
    pub sysenter_eip: u64,
    pub cr2: u64,
    pub reserved_0x248: [u8; 32],
    pub g_pat: u64,
    pub dbgctl: u64,
    pub br_from: u64,
    pub br_to: u64,
    pub last_excp_from: u64,
    pub last_excp_to: u64,
    pub reserved_0x298: [u8; 80],
    pub pkru: u32,
    pub tsc_aux: u32,
    pub tsc_scale: u64,
    pub tsc_offset: u64,
    pub reserved_0x300: [u8; 8],
    pub rcx: u64,
    pub rdx: u64,
    pub rbx: u64,
    pub /: *mut *mut u64 reserved_0x320; / rsp already available at 0x01d8,
    pub rbp: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub reserved_0x380: [u8; 16],
    pub guest_exit_info_1: u64,
    pub guest_exit_info_2: u64,
    pub guest_exit_int_info: u64,
    pub guest_nrip: u64,
    pub sev_features: u64,
    pub vintr_ctrl: u64,
    pub guest_exit_code: u64,
    pub virtual_tom: u64,
    pub tlb_id: u64,
    pub pcpu_id: u64,
    pub event_inj: u64,
    pub xcr0: u64,
    pub reserved_0x3f0: [u8; 16],
// Floating point area
    pub x87_dp: u64,
    pub mxcsr: u32,
    pub x87_ftw: u16,
    pub x87_fsw: u16,
    pub x87_fcw: u16,
    pub x87_fop: u16,
    pub x87_ds: u16,
    pub x87_cs: u16,
    pub x87_rip: u64,
    pub fpreg_x87: [u8; 80],
    pub fpreg_xmm: [u8; 256],
    pub fpreg_ymm: [u8; 256],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ghcb_save_area {
    pub reserved_0x0: [u8; 203],
    pub cpl: u8,
    pub reserved_0xcc: [u8; 116],
    pub xss: u64,
    pub reserved_0x148: [u8; 24],
    pub dr7: u64,
    pub reserved_0x168: [u8; 16],
    pub rip: u64,
    pub reserved_0x180: [u8; 88],
    pub rsp: u64,
    pub reserved_0x1e0: [u8; 24],
    pub rax: u64,
    pub reserved_0x200: [u8; 264],
    pub rcx: u64,
    pub rdx: u64,
    pub rbx: u64,
    pub reserved_0x320: [u8; 8],
    pub rbp: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub reserved_0x380: [u8; 16],
    pub sw_exit_code: u64,
    pub sw_exit_info_1: u64,
    pub sw_exit_info_2: u64,
    pub sw_scratch: u64,
    pub reserved_0x3b0: [u8; 56],
    pub xcr0: u64,
    pub valid_bitmap: [u8; 16],
    pub x87_state_gpa: u64,
    pub __packed: },
pub const GHCB_SHARED_BUF_SIZE: c_int = 2032;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ghcb {
    pub save: ghcb_save_area,
    pub ghcb_save_area)]: u8 reserved_save[2048 - sizeof(struct,
    pub shared_buffer: [u8; GHCB_SHARED_BUF_SIZE],
    pub reserved_0xff0: [u8; 10],
    pub /: *mut *mut u16 protocol_version; / negotiated SEV-ES/GHCB protocol version,
    pub ghcb_usage: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmcb {
    pub control: vmcb_control_area,
    pub save: vmcb_save_area,
//
// For SEV-ES VMs, the save area in the VMCB is used only to
// save/load host state.  Guest state resides in a separate
// page, the aptly named VM Save Area (VMSA), that is encrypted
// with the guest's private key.
//
    pub host_sev_es_save: sev_es_save_area,
}

pub const EXPECTED_VMCB_SAVE_AREA_SIZE: c_int = 744;
pub const EXPECTED_GHCB_SAVE_AREA_SIZE: c_int = 1032;
pub const EXPECTED_SEV_ES_SAVE_AREA_SIZE: c_int = 1648;
pub const EXPECTED_VMCB_CONTROL_AREA_SIZE: c_int = 1024;

// Check offsets of reserved fields
pub const SVM_CPUID_FUNC: c_uint = 0x8000000a;
pub const SVM_SELECTOR_S_SHIFT: c_int = 4;
pub const SVM_SELECTOR_DPL_SHIFT: c_int = 5;
pub const SVM_SELECTOR_P_SHIFT: c_int = 7;
pub const SVM_SELECTOR_AVL_SHIFT: c_int = 8;
pub const SVM_SELECTOR_L_SHIFT: c_int = 9;
pub const SVM_SELECTOR_DB_SHIFT: c_int = 10;
pub const SVM_SELECTOR_G_SHIFT: c_int = 11;

pub const SVM_EVTINJ_VEC_MASK: c_uint = 0xff;
pub const SVM_EVTINJ_TYPE_SHIFT: c_int = 8;

pub const SVM_EXITINFOSHIFT_TS_REASON_IRET: c_int = 36;
pub const SVM_EXITINFOSHIFT_TS_REASON_JMP: c_int = 38;
pub const SVM_EXITINFOSHIFT_TS_HAS_ERROR_CODE: c_int = 44;
pub const SVM_EXITINFO_REG_MASK: c_uint = 0x0F;

// GHCB Accessor functions

