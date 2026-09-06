//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/vmx.h
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
// vmx.h: VMX Architecture related definitions
// Copyright (c) 2004, Intel Corporation.
//
// A few random additions are:
// Copyright (C) 2006 Qumranet
// Avi Kivity <avi@qumranet.com>
// Yaniv Kamay <yaniv@qumranet.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmcs_hdr {
    pub revision_id:31: u32,
    pub shadow_vmcs:1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmcs {
    pub hdr: vmcs_hdr,
    pub abort: u32,
    pub data: [c_char; ],
}

//
// Definitions of Primary Processor-Based VM-Execution Controls.
//

pub const CPU_BASED_ALWAYSON_WITHOUT_TRUE_MSR: c_uint = 0x0401e172;
//
// Definitions of Secondary Processor-Based VM-Execution Controls.
//

//
// Definitions of Tertiary Processor-Based VM-Execution Controls.
//

pub const PIN_BASED_ALWAYSON_WITHOUT_TRUE_MSR: c_uint = 0x00000016;
pub const VM_EXIT_SAVE_DEBUG_CONTROLS: c_uint = 0x00000004;
pub const VM_EXIT_HOST_ADDR_SPACE_SIZE: c_uint = 0x00000200;
pub const VM_EXIT_LOAD_IA32_PERF_GLOBAL_CTRL: c_uint = 0x00001000;
pub const VM_EXIT_ACK_INTR_ON_EXIT: c_uint = 0x00008000;
pub const VM_EXIT_SAVE_IA32_PAT: c_uint = 0x00040000;
pub const VM_EXIT_LOAD_IA32_PAT: c_uint = 0x00080000;
pub const VM_EXIT_SAVE_IA32_EFER: c_uint = 0x00100000;
pub const VM_EXIT_LOAD_IA32_EFER: c_uint = 0x00200000;
pub const VM_EXIT_SAVE_VMX_PREEMPTION_TIMER: c_uint = 0x00400000;
pub const VM_EXIT_CLEAR_BNDCFGS: c_uint = 0x00800000;
pub const VM_EXIT_PT_CONCEAL_PIP: c_uint = 0x01000000;
pub const VM_EXIT_CLEAR_IA32_RTIT_CTL: c_uint = 0x02000000;
pub const VM_EXIT_LOAD_CET_STATE: c_uint = 0x10000000;
pub const VM_EXIT_SAVE_IA32_PERF_GLOBAL_CTRL: c_uint = 0x40000000;
pub const VM_EXIT_ALWAYSON_WITHOUT_TRUE_MSR: c_uint = 0x00036dff;
pub const VM_ENTRY_LOAD_DEBUG_CONTROLS: c_uint = 0x00000004;
pub const VM_ENTRY_IA32E_MODE: c_uint = 0x00000200;
pub const VM_ENTRY_SMM: c_uint = 0x00000400;
pub const VM_ENTRY_DEACT_DUAL_MONITOR: c_uint = 0x00000800;
pub const VM_ENTRY_LOAD_IA32_PERF_GLOBAL_CTRL: c_uint = 0x00002000;
pub const VM_ENTRY_LOAD_IA32_PAT: c_uint = 0x00004000;
pub const VM_ENTRY_LOAD_IA32_EFER: c_uint = 0x00008000;
pub const VM_ENTRY_LOAD_BNDCFGS: c_uint = 0x00010000;
pub const VM_ENTRY_PT_CONCEAL_PIP: c_uint = 0x00020000;
pub const VM_ENTRY_LOAD_IA32_RTIT_CTL: c_uint = 0x00040000;
pub const VM_ENTRY_LOAD_CET_STATE: c_uint = 0x00100000;
pub const VM_ENTRY_ALWAYSON_WITHOUT_TRUE_MSR: c_uint = 0x000011ff;
// VMFUNC functions

pub const VMFUNC_EPTP_ENTRIES: c_int = 512;

pub const VMX_MISC_MSR_LIST_MULTIPLIER: c_int = 512;
// VMCS Encodings
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmcs_field {
    VIRTUAL_PROCESSOR_ID            = 0x00000000,
    POSTED_INTR_NV                  = 0x00000002,
    LAST_PID_POINTER_INDEX		= 0x00000008,
    GUEST_ES_SELECTOR               = 0x00000800,
    GUEST_CS_SELECTOR               = 0x00000802,
    GUEST_SS_SELECTOR               = 0x00000804,
    GUEST_DS_SELECTOR               = 0x00000806,
    GUEST_FS_SELECTOR               = 0x00000808,
    GUEST_GS_SELECTOR               = 0x0000080a,
    GUEST_LDTR_SELECTOR             = 0x0000080c,
    GUEST_TR_SELECTOR               = 0x0000080e,
    GUEST_INTR_STATUS               = 0x00000810,
    GUEST_PML_INDEX			= 0x00000812,
    HOST_ES_SELECTOR                = 0x00000c00,
    HOST_CS_SELECTOR                = 0x00000c02,
    HOST_SS_SELECTOR                = 0x00000c04,
    HOST_DS_SELECTOR                = 0x00000c06,
    HOST_FS_SELECTOR                = 0x00000c08,
    HOST_GS_SELECTOR                = 0x00000c0a,
    HOST_TR_SELECTOR                = 0x00000c0c,
    IO_BITMAP_A                     = 0x00002000,
    IO_BITMAP_A_HIGH                = 0x00002001,
    IO_BITMAP_B                     = 0x00002002,
    IO_BITMAP_B_HIGH                = 0x00002003,
    MSR_BITMAP                      = 0x00002004,
    MSR_BITMAP_HIGH                 = 0x00002005,
    VM_EXIT_MSR_STORE_ADDR          = 0x00002006,
    VM_EXIT_MSR_STORE_ADDR_HIGH     = 0x00002007,
    VM_EXIT_MSR_LOAD_ADDR           = 0x00002008,
    VM_EXIT_MSR_LOAD_ADDR_HIGH      = 0x00002009,
    VM_ENTRY_MSR_LOAD_ADDR          = 0x0000200a,
    VM_ENTRY_MSR_LOAD_ADDR_HIGH     = 0x0000200b,
    PML_ADDRESS			= 0x0000200e,
    PML_ADDRESS_HIGH		= 0x0000200f,
    TSC_OFFSET                      = 0x00002010,
    TSC_OFFSET_HIGH                 = 0x00002011,
    VIRTUAL_APIC_PAGE_ADDR          = 0x00002012,
    VIRTUAL_APIC_PAGE_ADDR_HIGH     = 0x00002013,
    APIC_ACCESS_ADDR		= 0x00002014,
    APIC_ACCESS_ADDR_HIGH		= 0x00002015,
    POSTED_INTR_DESC_ADDR           = 0x00002016,
    POSTED_INTR_DESC_ADDR_HIGH      = 0x00002017,
    VM_FUNCTION_CONTROL             = 0x00002018,
    VM_FUNCTION_CONTROL_HIGH        = 0x00002019,
    EPT_POINTER                     = 0x0000201a,
    EPT_POINTER_HIGH                = 0x0000201b,
    EOI_EXIT_BITMAP0                = 0x0000201c,
    EOI_EXIT_BITMAP0_HIGH           = 0x0000201d,
    EOI_EXIT_BITMAP1                = 0x0000201e,
    EOI_EXIT_BITMAP1_HIGH           = 0x0000201f,
    EOI_EXIT_BITMAP2                = 0x00002020,
    EOI_EXIT_BITMAP2_HIGH           = 0x00002021,
    EOI_EXIT_BITMAP3                = 0x00002022,
    EOI_EXIT_BITMAP3_HIGH           = 0x00002023,
    EPTP_LIST_ADDRESS               = 0x00002024,
    EPTP_LIST_ADDRESS_HIGH          = 0x00002025,
    VMREAD_BITMAP                   = 0x00002026,
    VMREAD_BITMAP_HIGH              = 0x00002027,
    VMWRITE_BITMAP                  = 0x00002028,
    VMWRITE_BITMAP_HIGH             = 0x00002029,
    VE_INFORMATION_ADDRESS		= 0x0000202A,
    VE_INFORMATION_ADDRESS_HIGH	= 0x0000202B,
    XSS_EXIT_BITMAP                 = 0x0000202C,
    XSS_EXIT_BITMAP_HIGH            = 0x0000202D,
    ENCLS_EXITING_BITMAP		= 0x0000202E,
    ENCLS_EXITING_BITMAP_HIGH	= 0x0000202F,
    TSC_MULTIPLIER                  = 0x00002032,
    TSC_MULTIPLIER_HIGH             = 0x00002033,
    TERTIARY_VM_EXEC_CONTROL	= 0x00002034,
    TERTIARY_VM_EXEC_CONTROL_HIGH	= 0x00002035,
    SHARED_EPT_POINTER		= 0x0000203C,
    PID_POINTER_TABLE		= 0x00002042,
    PID_POINTER_TABLE_HIGH		= 0x00002043,
    GUEST_PHYSICAL_ADDRESS          = 0x00002400,
    GUEST_PHYSICAL_ADDRESS_HIGH     = 0x00002401,
    VMCS_LINK_POINTER               = 0x00002800,
    VMCS_LINK_POINTER_HIGH          = 0x00002801,
    GUEST_IA32_DEBUGCTL             = 0x00002802,
    GUEST_IA32_DEBUGCTL_HIGH        = 0x00002803,
    GUEST_IA32_PAT			= 0x00002804,
    GUEST_IA32_PAT_HIGH		= 0x00002805,
    GUEST_IA32_EFER			= 0x00002806,
    GUEST_IA32_EFER_HIGH		= 0x00002807,
    GUEST_IA32_PERF_GLOBAL_CTRL	= 0x00002808,
    GUEST_IA32_PERF_GLOBAL_CTRL_HIGH= 0x00002809,
    GUEST_PDPTR0                    = 0x0000280a,
    GUEST_PDPTR0_HIGH               = 0x0000280b,
    GUEST_PDPTR1                    = 0x0000280c,
    GUEST_PDPTR1_HIGH               = 0x0000280d,
    GUEST_PDPTR2                    = 0x0000280e,
    GUEST_PDPTR2_HIGH               = 0x0000280f,
    GUEST_PDPTR3                    = 0x00002810,
    GUEST_PDPTR3_HIGH               = 0x00002811,
    GUEST_BNDCFGS                   = 0x00002812,
    GUEST_BNDCFGS_HIGH              = 0x00002813,
    GUEST_IA32_RTIT_CTL		= 0x00002814,
    GUEST_IA32_RTIT_CTL_HIGH	= 0x00002815,
    HOST_IA32_PAT			= 0x00002c00,
    HOST_IA32_PAT_HIGH		= 0x00002c01,
    HOST_IA32_EFER			= 0x00002c02,
    HOST_IA32_EFER_HIGH		= 0x00002c03,
    HOST_IA32_PERF_GLOBAL_CTRL	= 0x00002c04,
    HOST_IA32_PERF_GLOBAL_CTRL_HIGH	= 0x00002c05,
    PIN_BASED_VM_EXEC_CONTROL       = 0x00004000,
    CPU_BASED_VM_EXEC_CONTROL       = 0x00004002,
    EXCEPTION_BITMAP                = 0x00004004,
    PAGE_FAULT_ERROR_CODE_MASK      = 0x00004006,
    PAGE_FAULT_ERROR_CODE_MATCH     = 0x00004008,
    CR3_TARGET_COUNT                = 0x0000400a,
    VM_EXIT_CONTROLS                = 0x0000400c,
    VM_EXIT_MSR_STORE_COUNT         = 0x0000400e,
    VM_EXIT_MSR_LOAD_COUNT          = 0x00004010,
    VM_ENTRY_CONTROLS               = 0x00004012,
    VM_ENTRY_MSR_LOAD_COUNT         = 0x00004014,
    VM_ENTRY_INTR_INFO_FIELD        = 0x00004016,
    VM_ENTRY_EXCEPTION_ERROR_CODE   = 0x00004018,
    VM_ENTRY_INSTRUCTION_LEN        = 0x0000401a,
    TPR_THRESHOLD                   = 0x0000401c,
    SECONDARY_VM_EXEC_CONTROL       = 0x0000401e,
    PLE_GAP                         = 0x00004020,
    PLE_WINDOW                      = 0x00004022,
    NOTIFY_WINDOW                   = 0x00004024,
    VM_INSTRUCTION_ERROR            = 0x00004400,
    VM_EXIT_REASON                  = 0x00004402,
    VM_EXIT_INTR_INFO               = 0x00004404,
    VM_EXIT_INTR_ERROR_CODE         = 0x00004406,
    IDT_VECTORING_INFO_FIELD        = 0x00004408,
    IDT_VECTORING_ERROR_CODE        = 0x0000440a,
    VM_EXIT_INSTRUCTION_LEN         = 0x0000440c,
    VMX_INSTRUCTION_INFO            = 0x0000440e,
    GUEST_ES_LIMIT                  = 0x00004800,
    GUEST_CS_LIMIT                  = 0x00004802,
    GUEST_SS_LIMIT                  = 0x00004804,
    GUEST_DS_LIMIT                  = 0x00004806,
    GUEST_FS_LIMIT                  = 0x00004808,
    GUEST_GS_LIMIT                  = 0x0000480a,
    GUEST_LDTR_LIMIT                = 0x0000480c,
    GUEST_TR_LIMIT                  = 0x0000480e,
    GUEST_GDTR_LIMIT                = 0x00004810,
    GUEST_IDTR_LIMIT                = 0x00004812,
    GUEST_ES_AR_BYTES               = 0x00004814,
    GUEST_CS_AR_BYTES               = 0x00004816,
    GUEST_SS_AR_BYTES               = 0x00004818,
    GUEST_DS_AR_BYTES               = 0x0000481a,
    GUEST_FS_AR_BYTES               = 0x0000481c,
    GUEST_GS_AR_BYTES               = 0x0000481e,
    GUEST_LDTR_AR_BYTES             = 0x00004820,
    GUEST_TR_AR_BYTES               = 0x00004822,
    GUEST_INTERRUPTIBILITY_INFO     = 0x00004824,
    GUEST_ACTIVITY_STATE            = 0x00004826,
    GUEST_SYSENTER_CS               = 0x0000482A,
    VMX_PREEMPTION_TIMER_VALUE      = 0x0000482E,
    HOST_IA32_SYSENTER_CS           = 0x00004c00,
    CR0_GUEST_HOST_MASK             = 0x00006000,
    CR4_GUEST_HOST_MASK             = 0x00006002,
    CR0_READ_SHADOW                 = 0x00006004,
    CR4_READ_SHADOW                 = 0x00006006,
    CR3_TARGET_VALUE0               = 0x00006008,
    CR3_TARGET_VALUE1               = 0x0000600a,
    CR3_TARGET_VALUE2               = 0x0000600c,
    CR3_TARGET_VALUE3               = 0x0000600e,
    EXIT_QUALIFICATION              = 0x00006400,
    GUEST_LINEAR_ADDRESS            = 0x0000640a,
    GUEST_CR0                       = 0x00006800,
    GUEST_CR3                       = 0x00006802,
    GUEST_CR4                       = 0x00006804,
    GUEST_ES_BASE                   = 0x00006806,
    GUEST_CS_BASE                   = 0x00006808,
    GUEST_SS_BASE                   = 0x0000680a,
    GUEST_DS_BASE                   = 0x0000680c,
    GUEST_FS_BASE                   = 0x0000680e,
    GUEST_GS_BASE                   = 0x00006810,
    GUEST_LDTR_BASE                 = 0x00006812,
    GUEST_TR_BASE                   = 0x00006814,
    GUEST_GDTR_BASE                 = 0x00006816,
    GUEST_IDTR_BASE                 = 0x00006818,
    GUEST_DR7                       = 0x0000681a,
    GUEST_RSP                       = 0x0000681c,
    GUEST_RIP                       = 0x0000681e,
    GUEST_RFLAGS                    = 0x00006820,
    GUEST_PENDING_DBG_EXCEPTIONS    = 0x00006822,
    GUEST_SYSENTER_ESP              = 0x00006824,
    GUEST_SYSENTER_EIP              = 0x00006826,
    GUEST_S_CET                     = 0x00006828,
    GUEST_SSP                       = 0x0000682a,
    GUEST_INTR_SSP_TABLE            = 0x0000682c,
    HOST_CR0                        = 0x00006c00,
    HOST_CR3                        = 0x00006c02,
    HOST_CR4                        = 0x00006c04,
    HOST_FS_BASE                    = 0x00006c06,
    HOST_GS_BASE                    = 0x00006c08,
    HOST_TR_BASE                    = 0x00006c0a,
    HOST_GDTR_BASE                  = 0x00006c0c,
    HOST_IDTR_BASE                  = 0x00006c0e,
    HOST_IA32_SYSENTER_ESP          = 0x00006c10,
    HOST_IA32_SYSENTER_EIP          = 0x00006c12,
    HOST_RSP                        = 0x00006c14,
    HOST_RIP                        = 0x00006c16,
    HOST_S_CET                      = 0x00006c18,
    HOST_SSP                        = 0x00006c1a,
    HOST_INTR_SSP_TABLE             = 0x00006c1c
}

//
// Interruption-information format
//
pub const INTR_INFO_VECTOR_MASK: c_uint = 0xff            /* 7:0 */;
pub const INTR_INFO_INTR_TYPE_MASK: c_uint = 0x700           /* 10:8 */;
pub const INTR_INFO_DELIVER_CODE_MASK: c_uint = 0x800           /* 11 */;
pub const INTR_INFO_UNBLOCK_NMI: c_uint = 0x1000		/* 12 */;
pub const INTR_INFO_VALID_MASK: c_uint = 0x80000000      /* 31 */;
pub const INTR_INFO_RESVD_BITS_MASK: c_uint = 0x7ffff000;

// GUEST_INTERRUPTIBILITY_INFO flags.
pub const GUEST_INTR_STATE_STI: c_uint = 0x00000001;
pub const GUEST_INTR_STATE_MOV_SS: c_uint = 0x00000002;
pub const GUEST_INTR_STATE_SMI: c_uint = 0x00000004;
pub const GUEST_INTR_STATE_NMI: c_uint = 0x00000008;
pub const GUEST_INTR_STATE_ENCLAVE_INTR: c_uint = 0x00000010;
// GUEST_ACTIVITY_STATE flags
pub const GUEST_ACTIVITY_ACTIVE: c_int = 0;
pub const GUEST_ACTIVITY_HLT: c_int = 1;
pub const GUEST_ACTIVITY_SHUTDOWN: c_int = 2;
pub const GUEST_ACTIVITY_WAIT_SIPI: c_int = 3;
//
// Exit Qualifications for MOV for Control Register Access
//
pub const CONTROL_REG_ACCESS_NUM: c_uint = 0x7     /* 2:0, number of control reg.*/;
pub const CONTROL_REG_ACCESS_TYPE: c_uint = 0x30    /* 5:4, access type */;
pub const CONTROL_REG_ACCESS_REG: c_uint = 0xf00   /* 10:8, general purpose reg. */;
pub const LMSW_SOURCE_DATA_SHIFT: c_int = 16;

//
// Exit Qualifications for MOV for Debug Register Access
//
pub const DEBUG_REG_ACCESS_NUM: c_uint = 0x7     /* 2:0, number of debug reg. */;
pub const DEBUG_REG_ACCESS_TYPE: c_uint = 0x10    /* 4, direction of access */;

//
// Exit Qualifications for APIC-Access
//
pub const APIC_ACCESS_OFFSET: c_uint = 0xfff   /* 11:0, offset within the APIC page */;
pub const APIC_ACCESS_TYPE: c_uint = 0xf000  /* 15:12, access type */;

// segment AR in VMCS -- these are different from what LAR reports

pub const VMX_AR_TYPE_ACCESSES_MASK: c_int = 1;

pub const VMX_AR_TYPE_MASK: c_uint = 0x0f;
pub const VMX_AR_TYPE_BUSY_64_TSS: c_int = 11;
pub const VMX_AR_TYPE_BUSY_32_TSS: c_int = 11;
pub const VMX_AR_TYPE_BUSY_16_TSS: c_int = 3;
pub const VMX_AR_TYPE_LDT: c_int = 2;

pub const VMX_AR_DPL_SHIFT: c_int = 5;

pub const VMX_AR_RESERVD_MASK: c_uint = 0xfffe0f00;

pub const VMX_VPID_EXTENT_INDIVIDUAL_ADDR: c_int = 0;
pub const VMX_VPID_EXTENT_SINGLE_CONTEXT: c_int = 1;
pub const VMX_VPID_EXTENT_ALL_CONTEXT: c_int = 2;
pub const VMX_VPID_EXTENT_SINGLE_NON_GLOBAL: c_int = 3;
pub const VMX_EPT_EXTENT_CONTEXT: c_int = 1;
pub const VMX_EPT_EXTENT_GLOBAL: c_int = 2;
pub const VMX_EPT_EXTENT_SHIFT: c_int = 24;

pub const VMX_EPT_MT_EPTE_SHIFT: c_int = 3;
pub const VMX_EPTP_PWL_MASK: c_uint = 0x38ull;
pub const VMX_EPTP_PWL_4: c_uint = 0x18ull;
pub const VMX_EPTP_PWL_5: c_uint = 0x20ull;

// The EPTP memtype is encoded in bits 2:0, i.e. doesn't need to be shifted.
pub const VMX_EPTP_MT_MASK: c_uint = 0x7ull;

pub const VMX_EPT_READABLE_MASK: c_uint = 0x1ull;
pub const VMX_EPT_WRITABLE_MASK: c_uint = 0x2ull;
pub const VMX_EPT_EXECUTABLE_MASK: c_uint = 0x4ull;

// @eptp must be pre-validated by the caller.
// The mask to use to trigger an EPT Misconfiguration in order to track MMIO

pub const VMX_EPT_IDENTITY_PAGETABLE_ADDR: c_uint = 0xfffbc000ul;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmx_msr_entry {
    pub index: u32,
    pub reserved: u32,
    pub value: u64,
    pub __aligned(16): },
//
// Exit Qualifications for entry failure during or after loading guest state
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vm_entry_failure_code {
    ENTRY_FAIL_DEFAULT		= 0,
    ENTRY_FAIL_PDPTE		= 2,
    ENTRY_FAIL_NMI			= 3,
    ENTRY_FAIL_VMCS_LINK_PTR	= 4,
}

//
// Exit Qualifications for EPT Violations
//

    pub EPT_VIOLATION_PROT_EXEC)): (EPT_VIOLATION_PROT_READ | EPT_VIOLATION_PROT_WRITE |,
//
// Exit Qualifications for NOTIFY VM EXIT
//

//
// VM-instruction error numbers
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vm_instruction_error_number {
    VMXERR_VMCALL_IN_VMX_ROOT_OPERATION = 1,
    VMXERR_VMCLEAR_INVALID_ADDRESS = 2,
    VMXERR_VMCLEAR_VMXON_POINTER = 3,
    VMXERR_VMLAUNCH_NONCLEAR_VMCS = 4,
    VMXERR_VMRESUME_NONLAUNCHED_VMCS = 5,
    VMXERR_VMRESUME_AFTER_VMXOFF = 6,
    VMXERR_ENTRY_INVALID_CONTROL_FIELD = 7,
    VMXERR_ENTRY_INVALID_HOST_STATE_FIELD = 8,
    VMXERR_VMPTRLD_INVALID_ADDRESS = 9,
    VMXERR_VMPTRLD_VMXON_POINTER = 10,
    VMXERR_VMPTRLD_INCORRECT_VMCS_REVISION_ID = 11,
    VMXERR_UNSUPPORTED_VMCS_COMPONENT = 12,
    VMXERR_VMWRITE_READ_ONLY_VMCS_COMPONENT = 13,
    VMXERR_VMXON_IN_VMX_ROOT_OPERATION = 15,
    VMXERR_ENTRY_INVALID_EXECUTIVE_VMCS_POINTER = 16,
    VMXERR_ENTRY_NONLAUNCHED_EXECUTIVE_VMCS = 17,
    VMXERR_ENTRY_EXECUTIVE_VMCS_POINTER_NOT_VMXON_POINTER = 18,
    VMXERR_VMCALL_NONCLEAR_VMCS = 19,
    VMXERR_VMCALL_INVALID_VM_EXIT_CONTROL_FIELDS = 20,
    VMXERR_VMCALL_INCORRECT_MSEG_REVISION_ID = 22,
    VMXERR_VMXOFF_UNDER_DUAL_MONITOR_TREATMENT_OF_SMIS_AND_SMM = 23,
    VMXERR_VMCALL_INVALID_SMM_MONITOR_FEATURES = 24,
    VMXERR_ENTRY_INVALID_VM_EXECUTION_CONTROL_FIELDS_IN_EXECUTIVE_VMCS = 25,
    VMXERR_ENTRY_EVENTS_BLOCKED_BY_MOV_SS = 26,
    VMXERR_INVALID_OPERAND_TO_INVEPT_INVVPID = 28,
}

//
// VM-instruction errors that can be encountered on VM-Enter, used to trace
// nested VM-Enter failures reported by hardware.  Errors unique to VM-Enter
// from a SMI Transfer Monitor are not included as things have gone seriously
// sideways if we get one of those...
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmx_l1d_flush_state {
    VMENTER_L1D_FLUSH_AUTO,
    VMENTER_L1D_FLUSH_NEVER,
    VMENTER_L1D_FLUSH_COND,
    VMENTER_L1D_FLUSH_ALWAYS,
    VMENTER_L1D_FLUSH_EPT_DISABLED,
    VMENTER_L1D_FLUSH_NOT_REQUIRED,
}

    pub l1tf_vmx_mitigation: extern enum vmx_l1d_flush_state,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmx_ve_information {
    pub exit_reason: u32,
    pub delivery: u32,
    pub exit_qualification: u64,
    pub guest_linear_address: u64,
    pub guest_physical_address: u64,
    pub eptp_index: u16,
}
