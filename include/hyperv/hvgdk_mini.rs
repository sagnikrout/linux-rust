//! Automatically rewritten from C Header to Rust Module
//! Source: include/hyperv/hvgdk_mini.h
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
// Type definitions for the Microsoft hypervisor.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_u128 {
    pub low_part: u64,
    pub high_part: u64,
    pub __packed: },
// NOTE: when adding below, update hv_result_to_string()
pub const HV_STATUS_SUCCESS: c_uint = 0x0;
pub const HV_STATUS_INVALID_HYPERCALL_CODE: c_uint = 0x2;
pub const HV_STATUS_INVALID_HYPERCALL_INPUT: c_uint = 0x3;
pub const HV_STATUS_INVALID_ALIGNMENT: c_uint = 0x4;
pub const HV_STATUS_INVALID_PARAMETER: c_uint = 0x5;
pub const HV_STATUS_ACCESS_DENIED: c_uint = 0x6;
pub const HV_STATUS_INVALID_PARTITION_STATE: c_uint = 0x7;
pub const HV_STATUS_OPERATION_DENIED: c_uint = 0x8;
pub const HV_STATUS_UNKNOWN_PROPERTY: c_uint = 0x9;
pub const HV_STATUS_PROPERTY_VALUE_OUT_OF_RANGE: c_uint = 0xA;
pub const HV_STATUS_INSUFFICIENT_MEMORY: c_uint = 0xB;
pub const HV_STATUS_INVALID_PARTITION_ID: c_uint = 0xD;
pub const HV_STATUS_INVALID_VP_INDEX: c_uint = 0xE;
pub const HV_STATUS_NOT_FOUND: c_uint = 0x10;
pub const HV_STATUS_INVALID_PORT_ID: c_uint = 0x11;
pub const HV_STATUS_INVALID_CONNECTION_ID: c_uint = 0x12;
pub const HV_STATUS_INSUFFICIENT_BUFFERS: c_uint = 0x13;
pub const HV_STATUS_NOT_ACKNOWLEDGED: c_uint = 0x14;
pub const HV_STATUS_INVALID_VP_STATE: c_uint = 0x15;
pub const HV_STATUS_NO_RESOURCES: c_uint = 0x1D;
pub const HV_STATUS_PROCESSOR_FEATURE_NOT_SUPPORTED: c_uint = 0x20;
pub const HV_STATUS_INVALID_LP_INDEX: c_uint = 0x41;
pub const HV_STATUS_INVALID_REGISTER_VALUE: c_uint = 0x50;
pub const HV_STATUS_OPERATION_FAILED: c_uint = 0x71;
pub const HV_STATUS_INSUFFICIENT_ROOT_MEMORY: c_uint = 0x73;
pub const HV_STATUS_INSUFFICIENT_CONTIGUOUS_MEMORY: c_uint = 0x75;
pub const HV_STATUS_TIME_OUT: c_uint = 0x78;
pub const HV_STATUS_CALL_PENDING: c_uint = 0x79;
pub const HV_STATUS_INSUFFICIENT_CONTIGUOUS_ROOT_MEMORY: c_uint = 0x83;
pub const HV_STATUS_VTL_ALREADY_ENABLED: c_uint = 0x86;
//
// The Hyper-V TimeRefCount register and the TSC
// page provide a guest VM clock with 100ns tick rate
//

pub const HV_HYP_PAGE_SHIFT: c_int = 12;

pub const HV_HYP_LARGE_PAGE_SHIFT: c_int = 21;

// Hyper-V specific model specific registers (MSRs)

// HV_X64_SYNTHETIC_MSR
pub const HV_X64_MSR_GUEST_OS_ID: c_uint = 0x40000000;
pub const HV_X64_MSR_HYPERCALL: c_uint = 0x40000001;
pub const HV_X64_MSR_VP_INDEX: c_uint = 0x40000002;
pub const HV_X64_MSR_RESET: c_uint = 0x40000003;
pub const HV_X64_MSR_VP_RUNTIME: c_uint = 0x40000010;
pub const HV_X64_MSR_TIME_REF_COUNT: c_uint = 0x40000020;
pub const HV_X64_MSR_REFERENCE_TSC: c_uint = 0x40000021;
pub const HV_X64_MSR_TSC_FREQUENCY: c_uint = 0x40000022;
pub const HV_X64_MSR_APIC_FREQUENCY: c_uint = 0x40000023;
// Define the virtual APIC registers
pub const HV_X64_MSR_EOI: c_uint = 0x40000070;
pub const HV_X64_MSR_ICR: c_uint = 0x40000071;
pub const HV_X64_MSR_TPR: c_uint = 0x40000072;
pub const HV_X64_MSR_VP_ASSIST_PAGE: c_uint = 0x40000073;
// Define synthetic interrupt controller model specific registers.
pub const HV_X64_MSR_SCONTROL: c_uint = 0x40000080;
pub const HV_X64_MSR_SVERSION: c_uint = 0x40000081;
pub const HV_X64_MSR_SIEFP: c_uint = 0x40000082;
pub const HV_X64_MSR_SIMP: c_uint = 0x40000083;
pub const HV_X64_MSR_EOM: c_uint = 0x40000084;
pub const HV_X64_MSR_SIRBP: c_uint = 0x40000085;
pub const HV_X64_MSR_SINT0: c_uint = 0x40000090;
pub const HV_X64_MSR_SINT1: c_uint = 0x40000091;
pub const HV_X64_MSR_SINT2: c_uint = 0x40000092;
pub const HV_X64_MSR_SINT3: c_uint = 0x40000093;
pub const HV_X64_MSR_SINT4: c_uint = 0x40000094;
pub const HV_X64_MSR_SINT5: c_uint = 0x40000095;
pub const HV_X64_MSR_SINT6: c_uint = 0x40000096;
pub const HV_X64_MSR_SINT7: c_uint = 0x40000097;
pub const HV_X64_MSR_SINT8: c_uint = 0x40000098;
pub const HV_X64_MSR_SINT9: c_uint = 0x40000099;
pub const HV_X64_MSR_SINT10: c_uint = 0x4000009A;
pub const HV_X64_MSR_SINT11: c_uint = 0x4000009B;
pub const HV_X64_MSR_SINT12: c_uint = 0x4000009C;
pub const HV_X64_MSR_SINT13: c_uint = 0x4000009D;
pub const HV_X64_MSR_SINT14: c_uint = 0x4000009E;
pub const HV_X64_MSR_SINT15: c_uint = 0x4000009F;
// Define synthetic interrupt controller model specific registers for nested hypervisor
pub const HV_X64_MSR_NESTED_SCONTROL: c_uint = 0x40001080;
pub const HV_X64_MSR_NESTED_SVERSION: c_uint = 0x40001081;
pub const HV_X64_MSR_NESTED_SIEFP: c_uint = 0x40001082;
pub const HV_X64_MSR_NESTED_SIMP: c_uint = 0x40001083;
pub const HV_X64_MSR_NESTED_EOM: c_uint = 0x40001084;
pub const HV_X64_MSR_NESTED_SINT0: c_uint = 0x40001090;
//
// Synthetic Timer MSRs. Four timers per vcpu.
//
pub const HV_X64_MSR_STIMER0_CONFIG: c_uint = 0x400000B0;
pub const HV_X64_MSR_STIMER0_COUNT: c_uint = 0x400000B1;
pub const HV_X64_MSR_STIMER1_CONFIG: c_uint = 0x400000B2;
pub const HV_X64_MSR_STIMER1_COUNT: c_uint = 0x400000B3;
pub const HV_X64_MSR_STIMER2_CONFIG: c_uint = 0x400000B4;
pub const HV_X64_MSR_STIMER2_COUNT: c_uint = 0x400000B5;
pub const HV_X64_MSR_STIMER3_CONFIG: c_uint = 0x400000B6;
pub const HV_X64_MSR_STIMER3_COUNT: c_uint = 0x400000B7;
// Hyper-V guest idle MSR
pub const HV_X64_MSR_GUEST_IDLE: c_uint = 0x400000F0;
// Hyper-V guest crash notification MSR's
pub const HV_X64_MSR_CRASH_P0: c_uint = 0x40000100;
pub const HV_X64_MSR_CRASH_P1: c_uint = 0x40000101;
pub const HV_X64_MSR_CRASH_P2: c_uint = 0x40000102;
pub const HV_X64_MSR_CRASH_P3: c_uint = 0x40000103;
pub const HV_X64_MSR_CRASH_P4: c_uint = 0x40000104;
pub const HV_X64_MSR_CRASH_CTL: c_uint = 0x40000105;
pub const HV_X64_MSR_HYPERCALL_ENABLE: c_uint = 0x00000001;
pub const HV_X64_MSR_HYPERCALL_PAGE_ADDRESS_SHIFT: c_int = 12;

pub const HV_IPI_LOW_VECTOR: c_uint = 0x10;
pub const HV_IPI_HIGH_VECTOR: c_uint = 0xff;
pub const HV_X64_MSR_VP_ASSIST_PAGE_ENABLE: c_uint = 0x00000001;
pub const HV_X64_MSR_VP_ASSIST_PAGE_ADDRESS_SHIFT: c_int = 12;

// Hyper-V Enlightened VMCS version mask in nested features CPUID
pub const HV_X64_ENLIGHTENED_VMCS_VERSION: c_uint = 0xff;
pub const HV_X64_MSR_TSC_REFERENCE_ENABLE: c_uint = 0x00000001;
pub const HV_X64_MSR_TSC_REFERENCE_ADDRESS_SHIFT: c_int = 12;
// Number of XMM registers used in hypercall input/output
pub const HV_HYPERCALL_MAX_XMM_REGISTERS: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_reenlightenment_control {
    pub 8: u64 vector :,
    pub 8: u64 reserved1 :,
    pub 1: u64 enabled :,
    pub 15: u64 reserved2 :,
    pub 32: u64 target_vp :,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_tsc_emulation_status {
    pub 1: u64 inprogress :,
    pub 63: u64 reserved :,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_tsc_emulation_control {
    pub 1: u64 enabled :,
    pub 63: u64 reserved :,
    pub __packed: },
// TSC emulation after migration
pub const HV_X64_MSR_REENLIGHTENMENT_CONTROL: c_uint = 0x40000106;
pub const HV_X64_MSR_TSC_EMULATION_CONTROL: c_uint = 0x40000107;
pub const HV_X64_MSR_TSC_EMULATION_STATUS: c_uint = 0x40000108;
pub const HV_X64_MSR_TSC_INVARIANT_CONTROL: c_uint = 0x40000118;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_output_get_partition_id {
    pub partition_id: u64,
    pub __packed: },
// HV_CRASH_CTL_REG_CONTENTS

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_reference_tsc_msr {
    pub as_uint64: u64,
    pub 1: u64 enable :,
    pub 11: u64 reserved :,
    pub 52: u64 pfn :,
    pub __packed: },
}

// The maximum number of sparse vCPU banks which can be encoded by 'struct hv_vpset'

// The number of vCPUs in one sparse bank

//
// Some of Hyper-V structs do not use hv_vpset where linux uses them.
//
// struct hv_vpset is usually used as part of hypercall input. The portion
// that counts as "fixed size input header" vs. "variable size input header"
// varies per hypercall. See comments at relevant hypercall call sites as to
// how the "valid_bank_mask" field should be accounted.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_vpset {
    pub format: u64,
    pub valid_bank_mask: u64,
    pub bank_contents: [u64; ],
    pub __packed: },
//
// Version info reported by hypervisor
// Changed to a union for convenience
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_hypervisor_version_info {
    pub build_number: u32,
    pub 16: u32 minor_version :,
    pub 16: u32 major_version :,
    pub service_pack: u32,
    pub 24: u32 service_number :,
    pub 8: u32 service_branch :,
}

// HV_CPUID_FUNCTION
pub const HYPERV_CPUID_VENDOR_AND_MAX_FUNCTIONS: c_uint = 0x40000000;
pub const HYPERV_CPUID_INTERFACE: c_uint = 0x40000001;
pub const HYPERV_CPUID_VERSION: c_uint = 0x40000002;
pub const HYPERV_CPUID_FEATURES: c_uint = 0x40000003;
pub const HYPERV_CPUID_ENLIGHTMENT_INFO: c_uint = 0x40000004;
pub const HYPERV_CPUID_IMPLEMENT_LIMITS: c_uint = 0x40000005;
pub const HYPERV_CPUID_CPU_MANAGEMENT_FEATURES: c_uint = 0x40000007;
pub const HYPERV_CPUID_NESTED_FEATURES: c_uint = 0x4000000A;
pub const HYPERV_CPUID_ISOLATION_CONFIG: c_uint = 0x4000000C;
pub const HYPERV_CPUID_VIRT_STACK_INTERFACE: c_uint = 0x40000081;
pub const HYPERV_VS_INTERFACE_EAX_SIGNATURE: c_uint = 0x31235356  /* "VS#1" */;
pub const HYPERV_CPUID_VIRT_STACK_PROPERTIES: c_uint = 0x40000082;
// Support for the extended IOAPIC RTE format

pub const HYPERV_HYPERVISOR_PRESENT_BIT: c_uint = 0x80000000;
pub const HYPERV_CPUID_MIN: c_uint = 0x40000005;
pub const HYPERV_CPUID_MAX: c_uint = 0x4000ffff;
//
// HV_X64_HYPERVISOR_FEATURES (EAX), or
// HV_PARTITION_PRIVILEGE_MASK [31-0]
//

//
// HV_X64_HYPERVISOR_FEATURES (EBX), or
// HV_PARTITION_PRIVILEGE_MASK [63-32]
//

// HV_X64_HYPERVISOR_FEATURES (EDX)

//
// Support for returning hypercall output block via XMM
// registers is available
//

// stimer Direct Mode is available

//
// Implementation recommendations. Indicates which behaviors the hypervisor
// recommends the OS implement for optimal performance.
// These are HYPERV_CPUID_ENLIGHTMENT_INFO.EAX bits.
//
// HV_X64_ENLIGHTENMENT_INFORMATION

//
// CPU management features identification.
// These are HYPERV_CPUID_CPU_MANAGEMENT_FEATURES.EAX bits.
//

//
// Virtual processor will never share a physical core with another virtual
// processor, except for virtual processors that are reported as sibling SMT
// threads.
//

// Nested features. These are HYPERV_CPUID_NESTED_FEATURES.EAX bits.

// Nested features #2. These are HYPERV_CPUID_NESTED_FEATURES.EBX bits.

//
// This is specific to AMD and specifies that enlightened TLB flush is
// supported. If guest opts in to this feature, ASID invalidations only
// flushes gva -> hpa mapping entries. To flush the TLB entries derived
// from NPT, hypercalls should be used (HvFlushGuestPhysicalAddressSpace
// or HvFlushGuestPhysicalAddressList).
//

// HYPERV_CPUID_ISOLATION_CONFIG.EAX bits.

// HYPERV_CPUID_ISOLATION_CONFIG.EBX bits.

// HYPERV_CPUID_FEATURES.ECX bits.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_isolation_type {
    HV_ISOLATION_TYPE_NONE	= 0,	/* HV_PARTITION_ISOLATION_TYPE_NONE */
    HV_ISOLATION_TYPE_VBS	= 1,
    HV_ISOLATION_TYPE_SNP	= 2,
    HV_ISOLATION_TYPE_TDX	= 3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_x64_msr_hypercall_contents {
    pub as_uint64: u64,
    pub 1: u64 enable :,
    pub 11: u64 reserved :,
    pub 52: u64 guest_physical_address :,
    pub __packed: },
}

pub const HV_MAXIMUM_PROCESSORS: c_int = 2048;

pub const HV_MAXIMUM_PROCESSORS: c_int = 320;

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_vp_assist_msr_contents {
    pub as_uint64: u64,
    pub 1: u64 enable :,
    pub 11: u64 reserved :,
    pub 52: u64 pfn :,
    pub __packed: },
}

// Declare the various hypercall operations.
// HV_CALL_CODE
pub const HVCALL_FLUSH_VIRTUAL_ADDRESS_SPACE: c_uint = 0x0002;
pub const HVCALL_FLUSH_VIRTUAL_ADDRESS_LIST: c_uint = 0x0003;
pub const HVCALL_GET_LOGICAL_PROCESSOR_RUN_TIME: c_uint = 0x0004;
pub const HVCALL_NOTIFY_LONG_SPIN_WAIT: c_uint = 0x0008;
pub const HVCALL_SEND_IPI: c_uint = 0x000b;
pub const HVCALL_ENABLE_VP_VTL: c_uint = 0x000f;
pub const HVCALL_FLUSH_VIRTUAL_ADDRESS_SPACE_EX: c_uint = 0x0013;
pub const HVCALL_FLUSH_VIRTUAL_ADDRESS_LIST_EX: c_uint = 0x0014;
pub const HVCALL_SEND_IPI_EX: c_uint = 0x0015;
pub const HVCALL_CREATE_PARTITION: c_uint = 0x0040;
pub const HVCALL_INITIALIZE_PARTITION: c_uint = 0x0041;
pub const HVCALL_FINALIZE_PARTITION: c_uint = 0x0042;
pub const HVCALL_DELETE_PARTITION: c_uint = 0x0043;
pub const HVCALL_GET_PARTITION_PROPERTY: c_uint = 0x0044;
pub const HVCALL_SET_PARTITION_PROPERTY: c_uint = 0x0045;
pub const HVCALL_GET_PARTITION_ID: c_uint = 0x0046;
pub const HVCALL_DEPOSIT_MEMORY: c_uint = 0x0048;
pub const HVCALL_WITHDRAW_MEMORY: c_uint = 0x0049;
pub const HVCALL_MAP_GPA_PAGES: c_uint = 0x004b;
pub const HVCALL_UNMAP_GPA_PAGES: c_uint = 0x004c;
pub const HVCALL_INSTALL_INTERCEPT: c_uint = 0x004d;
pub const HVCALL_CREATE_VP: c_uint = 0x004e;
pub const HVCALL_DELETE_VP: c_uint = 0x004f;
pub const HVCALL_GET_VP_REGISTERS: c_uint = 0x0050;
pub const HVCALL_SET_VP_REGISTERS: c_uint = 0x0051;
pub const HVCALL_TRANSLATE_VIRTUAL_ADDRESS: c_uint = 0x0052;
pub const HVCALL_CLEAR_VIRTUAL_INTERRUPT: c_uint = 0x0056;
pub const HVCALL_DELETE_PORT: c_uint = 0x0058;
pub const HVCALL_DISCONNECT_PORT: c_uint = 0x005b;
pub const HVCALL_POST_MESSAGE: c_uint = 0x005c;
pub const HVCALL_SIGNAL_EVENT: c_uint = 0x005d;
pub const HVCALL_POST_DEBUG_DATA: c_uint = 0x0069;
pub const HVCALL_RETRIEVE_DEBUG_DATA: c_uint = 0x006a;
pub const HVCALL_RESET_DEBUG_SESSION: c_uint = 0x006b;
pub const HVCALL_MAP_STATS_PAGE: c_uint = 0x006c;
pub const HVCALL_UNMAP_STATS_PAGE: c_uint = 0x006d;
pub const HVCALL_SET_SYSTEM_PROPERTY: c_uint = 0x006f;
pub const HVCALL_ADD_LOGICAL_PROCESSOR: c_uint = 0x0076;
pub const HVCALL_GET_SYSTEM_PROPERTY: c_uint = 0x007b;
pub const HVCALL_MAP_DEVICE_INTERRUPT: c_uint = 0x007c;
pub const HVCALL_UNMAP_DEVICE_INTERRUPT: c_uint = 0x007d;
pub const HVCALL_RETARGET_INTERRUPT: c_uint = 0x007e;
pub const HVCALL_NOTIFY_PARTITION_EVENT: c_uint = 0x0087;
pub const HVCALL_ENTER_SLEEP_STATE: c_uint = 0x0084;
pub const HVCALL_NOTIFY_PORT_RING_EMPTY: c_uint = 0x008b;
pub const HVCALL_REGISTER_INTERCEPT_RESULT: c_uint = 0x0091;
pub const HVCALL_ASSERT_VIRTUAL_INTERRUPT: c_uint = 0x0094;
pub const HVCALL_CREATE_PORT: c_uint = 0x0095;
pub const HVCALL_CONNECT_PORT: c_uint = 0x0096;
pub const HVCALL_START_VP: c_uint = 0x0099;
pub const HVCALL_GET_VP_INDEX_FROM_APIC_ID: c_uint = 0x009a;
pub const HVCALL_FLUSH_GUEST_PHYSICAL_ADDRESS_SPACE: c_uint = 0x00af;
pub const HVCALL_FLUSH_GUEST_PHYSICAL_ADDRESS_LIST: c_uint = 0x00b0;
pub const HVCALL_SIGNAL_EVENT_DIRECT: c_uint = 0x00c0;
pub const HVCALL_POST_MESSAGE_DIRECT: c_uint = 0x00c1;
pub const HVCALL_DISPATCH_VP: c_uint = 0x00c2;
pub const HVCALL_GET_GPA_PAGES_ACCESS_STATES: c_uint = 0x00c9;
pub const HVCALL_ACQUIRE_SPARSE_SPA_PAGE_HOST_ACCESS: c_uint = 0x00d7;
pub const HVCALL_RELEASE_SPARSE_SPA_PAGE_HOST_ACCESS: c_uint = 0x00d8;
pub const HVCALL_MODIFY_SPARSE_GPA_PAGE_HOST_VISIBILITY: c_uint = 0x00db;
pub const HVCALL_MAP_VP_STATE_PAGE: c_uint = 0x00e1;
pub const HVCALL_UNMAP_VP_STATE_PAGE: c_uint = 0x00e2;
pub const HVCALL_GET_VP_STATE: c_uint = 0x00e3;
pub const HVCALL_SET_VP_STATE: c_uint = 0x00e4;
pub const HVCALL_GET_VP_CPUID_VALUES: c_uint = 0x00f4;
pub const HVCALL_GET_PARTITION_PROPERTY_EX: c_uint = 0x0101;
pub const HVCALL_MMIO_READ: c_uint = 0x0106;
pub const HVCALL_MMIO_WRITE: c_uint = 0x0107;
pub const HVCALL_DISABLE_HYP_EX: c_uint = 0x010f;
pub const HVCALL_MAP_STATS_PAGE2: c_uint = 0x0131;
// HV_HYPERCALL_INPUT

pub const HV_HYPERCALL_VARHEAD_OFFSET: c_int = 17;

pub const HV_HYPERCALL_REP_COMP_OFFSET: c_int = 32;

pub const HV_HYPERCALL_REP_START_OFFSET: c_int = 48;

// HvFlushGuestPhysicalAddressSpace hypercalls
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_guest_mapping_flush {
    pub address_space: u64,
    pub flags: u64,
    pub __packed: },
//
// HV_MAX_FLUSH_PAGES = "additional_pages" + 1. It's limited
// by the bitwidth of "additional_pages" in union hv_gpa_page_range.
//

pub const HV_GPA_PAGE_RANGE_PAGE_SIZE_2MB: c_int = 0;
pub const HV_GPA_PAGE_RANGE_PAGE_SIZE_1GB: c_int = 1;

// HvFlushGuestPhysicalAddressList, HvExtCallMemoryHeatHint hypercall
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_gpa_page_range {
    pub address_space: u64,
    pub 11: u64 additional_pages :,
    pub 1: u64 largepage :,
    pub 52: u64 basepfn :,
    pub page: },
    pub 12: u64 reserved :,
    pub 1: u64 page_size :,
    pub 8: u64 reserved1 :,
    pub 43: u64 base_large_pfn :,
}

//
// All input flush parameters should be in single page. The max flush
// count is equal with how many entries of union hv_gpa_page_range can
// be populated into the input parameter page.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_guest_mapping_flush_list {
    pub address_space: u64,
    pub flags: u64,
    pub gpa_list: [hv_gpa_page_range; HV_MAX_FLUSH_REP_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_tlb_flush {
    pub address_space: u64,
    pub flags: u64,
    pub processor_mask: u64,
    pub gva_list: [u64; ],
    pub __packed: },
// HvFlushVirtualAddressSpaceEx, HvFlushVirtualAddressListEx hypercalls
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_tlb_flush_ex {
    pub address_space: u64,
    pub flags: u64,
    pub gva_list: [u64; ],
    pub __packed: },
    pub gva_list)): offsetof(struct hv_tlb_flush_ex,,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_hyperv_tsc_page {
    pub tsc_sequence: volatile u32,
    pub reserved1: u32,
    pub tsc_scale: volatile u64,
    pub tsc_offset: volatile s64,
    pub __packed: },
// Define the number of synthetic interrupt sources.

// Define the expected SynIC version.

// Valid SynIC vectors are 16-255.

// Hyper-V defined statically assigned SINTs
pub const HV_SYNIC_INTERCEPTION_SINT_INDEX: c_uint = 0x00000000;
pub const HV_SYNIC_IOMMU_FAULT_SINT_INDEX: c_uint = 0x00000001;
pub const HV_SYNIC_VMBUS_SINT_INDEX: c_uint = 0x00000002;
pub const HV_SYNIC_FIRST_UNUSED_SINT_INDEX: c_uint = 0x00000005;
// mshv assigned SINT for doorbell

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_interrupt_type {
    HV_X64_INTERRUPT_TYPE_FIXED		= 0x0000,
    HV_X64_INTERRUPT_TYPE_LOWESTPRIORITY	= 0x0001,
    HV_X64_INTERRUPT_TYPE_SMI		= 0x0002,
    HV_X64_INTERRUPT_TYPE_REMOTEREAD	= 0x0003,
    HV_X64_INTERRUPT_TYPE_NMI		= 0x0004,
    HV_X64_INTERRUPT_TYPE_INIT		= 0x0005,
    HV_X64_INTERRUPT_TYPE_SIPI		= 0x0006,
    HV_X64_INTERRUPT_TYPE_EXTINT		= 0x0007,
    HV_X64_INTERRUPT_TYPE_LOCALINT0		= 0x0008,
    HV_X64_INTERRUPT_TYPE_LOCALINT1		= 0x0009,
    HV_X64_INTERRUPT_TYPE_MAXIMUM		= 0x000A,
}

// Define synthetic interrupt source.
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_synic_sint {
    pub as_uint64: u64,
    pub 8: u64 vector :,
    pub 8: u64 reserved1 :,
    pub 1: u64 masked :,
    pub 1: u64 auto_eoi :,
    pub 1: u64 polling :,
    pub 1: u64 as_intercept :,
    pub 1: u64 proxy :,
    pub 43: u64 reserved2 :,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_x64_xsave_xfem_register {
    pub as_uint64: u64,
    pub low_uint32: u32,
    pub high_uint32: u32,
    pub __packed: },
    pub 1: u64 legacy_x87 :,
    pub 1: u64 legacy_sse :,
    pub 1: u64 avx :,
    pub 1: u64 mpx_bndreg :,
    pub 1: u64 mpx_bndcsr :,
    pub 1: u64 avx_512_op_mask :,
    pub 1: u64 avx_512_zmmhi :,
    pub 1: u64 avx_512_zmm16_31 :,
    pub 2: u64 rsvd8_9 :,
    pub 1: u64 pasid :,
    pub 1: u64 cet_u :,
    pub 1: u64 cet_s :,
    pub 4: u64 rsvd13_16 :,
    pub 1: u64 xtile_cfg :,
    pub 1: u64 xtile_data :,
    pub 45: u64 rsvd19_63 :,
    pub __packed: },
}

// Synthetic timer configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_stimer_config {
    pub as_uint64: u64,
    pub 1: u64 enable :,
    pub 1: u64 periodic :,
    pub 1: u64 lazy :,
    pub 1: u64 auto_enable :,
    pub 8: u64 apic_vector :,
    pub 1: u64 direct_mode :,
    pub 3: u64 reserved_z0 :,
    pub 4: u64 sintx :,
    pub 44: u64 reserved_z1 :,
    pub __packed: },
}

// Define the number of synthetic timers

// Define port identifier type.
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_port_id {
    pub asu32: u32,
    pub 24: u32 id :,
    pub 8: u32 reserved :,
    pub u: } __packed,
}

// Define hypervisor message types.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_message_type {
    HVMSG_NONE				= 0x00000000,

// Memory access messages.
    HVMSG_UNMAPPED_GPA			= 0x80000000,
    HVMSG_GPA_INTERCEPT			= 0x80000001,

// Timer notification messages.
    HVMSG_TIMER_EXPIRED			= 0x80000010,

// Error messages.
    HVMSG_INVALID_VP_REGISTER_VALUE		= 0x80000020,
    HVMSG_UNRECOVERABLE_EXCEPTION		= 0x80000021,
    HVMSG_UNSUPPORTED_FEATURE		= 0x80000022,

//
// Opaque intercept message. The original intercept message is only
// accessible from the mapped intercept message page.
//
    HVMSG_OPAQUE_INTERCEPT			= 0x8000003F,

// Trace buffer complete messages.
    HVMSG_EVENTLOG_BUFFERCOMPLETE		= 0x80000040,

// Hypercall intercept
    HVMSG_HYPERCALL_INTERCEPT		= 0x80000050,

// SynIC intercepts
    HVMSG_SYNIC_EVENT_INTERCEPT		= 0x80000060,
    HVMSG_SYNIC_SINT_INTERCEPT		= 0x80000061,
    HVMSG_SYNIC_SINT_DELIVERABLE	= 0x80000062,

// Async call completion intercept
    HVMSG_ASYNC_CALL_COMPLETION		= 0x80000070,

// Root scheduler messages
    HVMSG_SCHEDULER_VP_SIGNAL_BITSET	= 0x80000100,
    HVMSG_SCHEDULER_VP_SIGNAL_PAIR		= 0x80000101,

// Platform-specific processor intercept messages.
    HVMSG_X64_IO_PORT_INTERCEPT		= 0x80010000,
    HVMSG_X64_MSR_INTERCEPT			= 0x80010001,
    HVMSG_X64_CPUID_INTERCEPT		= 0x80010002,
    HVMSG_X64_EXCEPTION_INTERCEPT		= 0x80010003,
    HVMSG_X64_APIC_EOI			= 0x80010004,
    HVMSG_X64_LEGACY_FP_ERROR		= 0x80010005,
    HVMSG_X64_IOMMU_PRQ			= 0x80010006,
    HVMSG_X64_HALT				= 0x80010007,
    HVMSG_X64_INTERRUPTION_DELIVERABLE	= 0x80010008,
    HVMSG_X64_SIPI_INTERCEPT		= 0x80010009,
}

// Define the format of the SIMP register
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_synic_simp {
    pub as_uint64: u64,
    pub 1: u64 simp_enabled :,
    pub 11: u64 preserved :,
    pub 52: u64 base_simp_gpa :,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_message_flags {
    pub asu8: u8,
    pub 1: u8 msg_pending :,
    pub 7: u8 reserved :,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_message_header {
    pub message_type: u32,
    pub payload_size: u8,
    pub message_flags: hv_message_flags,
    pub reserved: [u8; 2],
    pub sender: u64,
    pub port: hv_port_id,
}

//
// Message format for notifications delivered via
// intercept message(as_intercept=1)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_notification_message_payload {
    pub sint_index: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_message {
    pub header: hv_message_header,
    pub payload: [u64; HV_MESSAGE_PAYLOAD_QWORD_COUNT],
    pub u: },
    pub __packed: },
// Define the synthetic interrupt message page layout.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_message_page {
    pub sint_message: [hv_message; HV_SYNIC_SINT_COUNT],
    pub __packed: },
// Define timer message payload structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_timer_message_payload {
    pub timer_index: u32,
    pub reserved: u32,
    pub /: *mut *mut u64 expiration_time; / When the timer expired,
    pub /: *mut *mut u64 delivery_time; / When the message was delivered,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_x64_segment_register {
    pub base: u64,
    pub limit: u32,
    pub selector: u16,
    pub 4: u16 segment_type :,
    pub 1: u16 non_system_segment :,
    pub 2: u16 descriptor_privilege_level :,
    pub 1: u16 present :,
    pub 4: u16 reserved :,
    pub 1: u16 available :,
    pub 1: u16 _long :,
    pub 1: u16 _default :,
    pub 1: u16 granularity :,
    pub __packed: },
    pub attributes: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_x64_table_register {
    pub pad: [u16; 3],
    pub limit: u16,
    pub base: u64,
    pub __packed: },
pub const HV_NORMAL_VTL: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_input_vtl {
    pub as_uint8: u8,
    pub 4: u8 target_vtl :,
    pub 1: u8 use_target_vtl :,
    pub 3: u8 reserved_z :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_init_vp_context {
    pub rip: u64,
    pub rsp: u64,
    pub rflags: u64,
    pub cs: hv_x64_segment_register,
    pub ds: hv_x64_segment_register,
    pub es: hv_x64_segment_register,
    pub fs: hv_x64_segment_register,
    pub gs: hv_x64_segment_register,
    pub ss: hv_x64_segment_register,
    pub tr: hv_x64_segment_register,
    pub ldtr: hv_x64_segment_register,
    pub idtr: hv_x64_table_register,
    pub gdtr: hv_x64_table_register,
    pub efer: u64,
    pub cr0: u64,
    pub cr3: u64,
    pub cr4: u64,
    pub msr_cr_pat: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_enable_vp_vtl {
    pub partition_id: u64,
    pub vp_index: u32,
    pub target_vtl: hv_input_vtl,
    pub mbz0: u8,
    pub mbz1: u16,
    pub vp_context: hv_init_vp_context,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_get_vp_from_apic_id_in {
    pub partition_id: u64,
    pub target_vtl: hv_input_vtl,
    pub res: [u8; 7],
    pub apic_ids: [u32; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_register_vsm_partition_config {
    pub as_uint64: u64,
    pub 1: u64 enable_vtl_protection :,
    pub 4: u64 default_vtl_protection_mask :,
    pub 1: u64 zero_memory_on_reset :,
    pub 1: u64 deny_lower_vtl_startup :,
    pub 1: u64 intercept_acceptance :,
    pub 1: u64 intercept_enable_vtl_protection :,
    pub 1: u64 intercept_vp_startup :,
    pub 1: u64 intercept_cpuid_unimplemented :,
    pub 1: u64 intercept_unrecoverable_exception :,
    pub 1: u64 intercept_page :,
    pub 51: u64 mbz :,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_register_vsm_capabilities {
    pub as_uint64: u64,
    pub 1: u64 dr6_shared:,
    pub 16: u64 mbec_vtl_mask:,
    pub 1: u64 deny_lower_vtl_startup:,
    pub 1: u64 supervisor_shadow_stack:,
    pub 1: u64 hardware_hvpt_available:,
    pub 1: u64 software_hvpt_available:,
    pub 6: u64 hardware_hvpt_range_bits:,
    pub 1: u64 intercept_page_available:,
    pub 1: u64 return_action_available:,
    pub 35: u64 reserved:,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_register_vsm_page_offsets {
    pub 12: u64 vtl_call_offset :,
    pub 12: u64 vtl_return_offset :,
    pub 40: u64 reserved_mbz :,
    pub __packed: },
    pub as_uint64: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_nested_enlightenments_control {
    pub 1: u32 directhypercall :,
    pub 31: u32 reserved :,
    pub features: } __packed,
    pub 1: u32 inter_partition_comm :,
    pub 31: u32 reserved :,
    pub hypercall_controls: } __packed,
    pub __packed: },
// Define virtual processor assist page structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_vp_assist_page {
    pub apic_assist: u32,
    pub reserved1: u32,
    pub vtl_entry_reason: u32,
    pub vtl_reserved: u32,
    pub vtl_ret_x64rax: u64,
    pub vtl_ret_x64rcx: u64,
    pub nested_control: hv_nested_enlightenments_control,
    pub enlighten_vmentry: u8,
    pub reserved2: [u8; 7],
    pub current_nested_vmcs: u64,
    pub synthetic_time_unhalted_timer_expired: u8,
    pub reserved3: [u8; 7],
    pub virtualization_fault_information: [u8; 40],
    pub reserved4: [u8; 8],
    pub intercept_message: [u8; 256],
    pub vtl_ret_actions: [u8; 256],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_register_name {
// Suspend Registers
    HV_REGISTER_EXPLICIT_SUSPEND				= 0x00000000,
    HV_REGISTER_INTERCEPT_SUSPEND				= 0x00000001,
    HV_REGISTER_DISPATCH_SUSPEND				= 0x00000003,

// Version - 128-bit result same as CPUID 0x40000002
    HV_REGISTER_HYPERVISOR_VERSION				= 0x00000100,

// Feature Access (registers are 128 bits) - same as CPUID 0x40000003 - 0x4000000B
    HV_REGISTER_PRIVILEGES_AND_FEATURES_INFO		= 0x00000200,
    HV_REGISTER_FEATURES_INFO				= 0x00000201,
    HV_REGISTER_IMPLEMENTATION_LIMITS_INFO			= 0x00000202,
    HV_REGISTER_HARDWARE_FEATURES_INFO			= 0x00000203,
    HV_REGISTER_CPU_MANAGEMENT_FEATURES_INFO		= 0x00000204,
    HV_REGISTER_SVM_FEATURES_INFO				= 0x00000205,
    HV_REGISTER_SKIP_LEVEL_FEATURES_INFO			= 0x00000206,
    HV_REGISTER_NESTED_VIRT_FEATURES_INFO			= 0x00000207,
    HV_REGISTER_IPT_FEATURES_INFO				= 0x00000208,

// Guest Crash Registers
    HV_REGISTER_GUEST_CRASH_P0				= 0x00000210,
    HV_REGISTER_GUEST_CRASH_P1				= 0x00000211,
    HV_REGISTER_GUEST_CRASH_P2				= 0x00000212,
    HV_REGISTER_GUEST_CRASH_P3				= 0x00000213,
    HV_REGISTER_GUEST_CRASH_P4				= 0x00000214,
    HV_REGISTER_GUEST_CRASH_CTL				= 0x00000215,

// Misc
    HV_REGISTER_VP_RUNTIME					= 0x00090000,
    HV_REGISTER_GUEST_OS_ID					= 0x00090002,
    HV_REGISTER_VP_INDEX					= 0x00090003,
    HV_REGISTER_TIME_REF_COUNT				= 0x00090004,
    HV_REGISTER_CPU_MANAGEMENT_VERSION			= 0x00090007,
    HV_REGISTER_VP_ASSIST_PAGE				= 0x00090013,
    HV_REGISTER_VP_ROOT_SIGNAL_COUNT			= 0x00090014,
    HV_REGISTER_REFERENCE_TSC				= 0x00090017,

// Hypervisor-defined Registers (Synic)
    HV_REGISTER_SINT0					= 0x000A0000,
    HV_REGISTER_SINT1					= 0x000A0001,
    HV_REGISTER_SINT2					= 0x000A0002,
    HV_REGISTER_SINT3					= 0x000A0003,
    HV_REGISTER_SINT4					= 0x000A0004,
    HV_REGISTER_SINT5					= 0x000A0005,
    HV_REGISTER_SINT6					= 0x000A0006,
    HV_REGISTER_SINT7					= 0x000A0007,
    HV_REGISTER_SINT8					= 0x000A0008,
    HV_REGISTER_SINT9					= 0x000A0009,
    HV_REGISTER_SINT10					= 0x000A000A,
    HV_REGISTER_SINT11					= 0x000A000B,
    HV_REGISTER_SINT12					= 0x000A000C,
    HV_REGISTER_SINT13					= 0x000A000D,
    HV_REGISTER_SINT14					= 0x000A000E,
    HV_REGISTER_SINT15					= 0x000A000F,
    HV_REGISTER_SCONTROL					= 0x000A0010,
    HV_REGISTER_SVERSION					= 0x000A0011,
    HV_REGISTER_SIEFP					= 0x000A0012,
    HV_REGISTER_SIMP					= 0x000A0013,
    HV_REGISTER_EOM						= 0x000A0014,
    HV_REGISTER_SIRBP					= 0x000A0015,

    HV_REGISTER_NESTED_SINT0				= 0x000A1000,
    HV_REGISTER_NESTED_SINT1				= 0x000A1001,
    HV_REGISTER_NESTED_SINT2				= 0x000A1002,
    HV_REGISTER_NESTED_SINT3				= 0x000A1003,
    HV_REGISTER_NESTED_SINT4				= 0x000A1004,
    HV_REGISTER_NESTED_SINT5				= 0x000A1005,
    HV_REGISTER_NESTED_SINT6				= 0x000A1006,
    HV_REGISTER_NESTED_SINT7				= 0x000A1007,
    HV_REGISTER_NESTED_SINT8				= 0x000A1008,
    HV_REGISTER_NESTED_SINT9				= 0x000A1009,
    HV_REGISTER_NESTED_SINT10				= 0x000A100A,
    HV_REGISTER_NESTED_SINT11				= 0x000A100B,
    HV_REGISTER_NESTED_SINT12				= 0x000A100C,
    HV_REGISTER_NESTED_SINT13				= 0x000A100D,
    HV_REGISTER_NESTED_SINT14				= 0x000A100E,
    HV_REGISTER_NESTED_SINT15				= 0x000A100F,
    HV_REGISTER_NESTED_SCONTROL				= 0x000A1010,
    HV_REGISTER_NESTED_SVERSION				= 0x000A1011,
    HV_REGISTER_NESTED_SIFP					= 0x000A1012,
    HV_REGISTER_NESTED_SIPP					= 0x000A1013,
    HV_REGISTER_NESTED_EOM					= 0x000A1014,
    HV_REGISTER_NESTED_SIRBP				= 0x000a1015,

// Hypervisor-defined Registers (Synthetic Timers)
    HV_REGISTER_STIMER0_CONFIG				= 0x000B0000,
    HV_REGISTER_STIMER0_COUNT				= 0x000B0001,

// VSM
    HV_REGISTER_VSM_VP_STATUS				= 0x000D0003,

// Synthetic VSM registers
    HV_REGISTER_VSM_CODE_PAGE_OFFSETS	= 0x000D0002,
    HV_REGISTER_VSM_CAPABILITIES		= 0x000D0006,
    HV_REGISTER_VSM_PARTITION_CONFIG	= 0x000D0007,

// X64 Debug Registers
    HV_X64_REGISTER_DR0	= 0x00050000,
    HV_X64_REGISTER_DR1	= 0x00050001,
    HV_X64_REGISTER_DR2	= 0x00050002,
    HV_X64_REGISTER_DR3	= 0x00050003,
    HV_X64_REGISTER_DR6	= 0x00050004,
    HV_X64_REGISTER_DR7	= 0x00050005,

// X64 Cache control MSRs
    HV_X64_REGISTER_MSR_MTRR_CAP		= 0x0008000D,
    HV_X64_REGISTER_MSR_MTRR_DEF_TYPE	= 0x0008000E,
    HV_X64_REGISTER_MSR_MTRR_PHYS_BASE0	= 0x00080010,
    HV_X64_REGISTER_MSR_MTRR_PHYS_BASE1	= 0x00080011,
    HV_X64_REGISTER_MSR_MTRR_PHYS_BASE2	= 0x00080012,
    HV_X64_REGISTER_MSR_MTRR_PHYS_BASE3	= 0x00080013,
    HV_X64_REGISTER_MSR_MTRR_PHYS_BASE4	= 0x00080014,
    HV_X64_REGISTER_MSR_MTRR_PHYS_BASE5	= 0x00080015,
    HV_X64_REGISTER_MSR_MTRR_PHYS_BASE6	= 0x00080016,
    HV_X64_REGISTER_MSR_MTRR_PHYS_BASE7	= 0x00080017,
    HV_X64_REGISTER_MSR_MTRR_PHYS_BASE8	= 0x00080018,
    HV_X64_REGISTER_MSR_MTRR_PHYS_BASE9	= 0x00080019,
    HV_X64_REGISTER_MSR_MTRR_PHYS_BASEA	= 0x0008001A,
    HV_X64_REGISTER_MSR_MTRR_PHYS_BASEB	= 0x0008001B,
    HV_X64_REGISTER_MSR_MTRR_PHYS_BASEC	= 0x0008001C,
    HV_X64_REGISTER_MSR_MTRR_PHYS_BASED	= 0x0008001D,
    HV_X64_REGISTER_MSR_MTRR_PHYS_BASEE	= 0x0008001E,
    HV_X64_REGISTER_MSR_MTRR_PHYS_BASEF	= 0x0008001F,
    HV_X64_REGISTER_MSR_MTRR_PHYS_MASK0	= 0x00080040,
    HV_X64_REGISTER_MSR_MTRR_PHYS_MASK1	= 0x00080041,
    HV_X64_REGISTER_MSR_MTRR_PHYS_MASK2	= 0x00080042,
    HV_X64_REGISTER_MSR_MTRR_PHYS_MASK3	= 0x00080043,
    HV_X64_REGISTER_MSR_MTRR_PHYS_MASK4	= 0x00080044,
    HV_X64_REGISTER_MSR_MTRR_PHYS_MASK5	= 0x00080045,
    HV_X64_REGISTER_MSR_MTRR_PHYS_MASK6	= 0x00080046,
    HV_X64_REGISTER_MSR_MTRR_PHYS_MASK7	= 0x00080047,
    HV_X64_REGISTER_MSR_MTRR_PHYS_MASK8	= 0x00080048,
    HV_X64_REGISTER_MSR_MTRR_PHYS_MASK9	= 0x00080049,
    HV_X64_REGISTER_MSR_MTRR_PHYS_MASKA	= 0x0008004A,
    HV_X64_REGISTER_MSR_MTRR_PHYS_MASKB	= 0x0008004B,
    HV_X64_REGISTER_MSR_MTRR_PHYS_MASKC	= 0x0008004C,
    HV_X64_REGISTER_MSR_MTRR_PHYS_MASKD	= 0x0008004D,
    HV_X64_REGISTER_MSR_MTRR_PHYS_MASKE	= 0x0008004E,
    HV_X64_REGISTER_MSR_MTRR_PHYS_MASKF	= 0x0008004F,
    HV_X64_REGISTER_MSR_MTRR_FIX64K00000	= 0x00080070,
    HV_X64_REGISTER_MSR_MTRR_FIX16K80000	= 0x00080071,
    HV_X64_REGISTER_MSR_MTRR_FIX16KA0000	= 0x00080072,
    HV_X64_REGISTER_MSR_MTRR_FIX4KC0000	= 0x00080073,
    HV_X64_REGISTER_MSR_MTRR_FIX4KC8000	= 0x00080074,
    HV_X64_REGISTER_MSR_MTRR_FIX4KD0000	= 0x00080075,
    HV_X64_REGISTER_MSR_MTRR_FIX4KD8000	= 0x00080076,
    HV_X64_REGISTER_MSR_MTRR_FIX4KE0000	= 0x00080077,
    HV_X64_REGISTER_MSR_MTRR_FIX4KE8000	= 0x00080078,
    HV_X64_REGISTER_MSR_MTRR_FIX4KF0000	= 0x00080079,
    HV_X64_REGISTER_MSR_MTRR_FIX4KF8000	= 0x0008007A,

    HV_X64_REGISTER_REG_PAGE	= 0x0009001C,

    HV_ARM64_REGISTER_SINT_RESERVED_INTERRUPT_ID	= 0x00070001,

}

//
// Arch compatibility regs for use with hv_set/get_register
//

//
// To support arch-generic code calling hv_set/get_register:
// - On x86, HV_MSR_ indicates an MSR accessed via rdmsrq/wrmsrq
// - On ARM, HV_MSR_ indicates a VP register accessed via hypercall
//

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_explicit_suspend_register {
    pub as_uint64: u64,
    pub 1: u64 suspended :,
    pub 63: u64 reserved :,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_intercept_suspend_register {
    pub as_uint64: u64,
    pub 1: u64 suspended :,
    pub 63: u64 reserved :,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_dispatch_suspend_register {
    pub as_uint64: u64,
    pub 1: u64 suspended :,
    pub 63: u64 reserved :,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_arm64_pending_interruption_register {
    pub as_uint64: u64,
    pub 1: u64 interruption_pending :,
    pub 1: u64 interruption_type:,
    pub 30: u64 reserved :,
    pub 32: u64 error_code :,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_arm64_interrupt_state_register {
    pub as_uint64: u64,
    pub 1: u64 interrupt_shadow :,
    pub 63: u64 reserved :,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_arm64_pending_synthetic_exception_event {
    pub as_uint64: [u64; 2],
    pub 1: u8 event_pending :,
    pub 3: u8 event_type :,
    pub 4: u8 reserved :,
    pub rsvd: [u8; 3],
    pub exception_type: u32,
    pub context: u64,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_x64_interrupt_state_register {
    pub as_uint64: u64,
    pub 1: u64 interrupt_shadow :,
    pub 1: u64 nmi_masked :,
    pub 62: u64 reserved :,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_x64_pending_interruption_register {
    pub as_uint64: u64,
    pub 1: u32 interruption_pending :,
    pub 3: u32 interruption_type :,
    pub 1: u32 deliver_error_code :,
    pub 4: u32 instruction_length :,
    pub 1: u32 nested_event :,
    pub 6: u32 reserved :,
    pub 16: u32 interruption_vector :,
    pub error_code: u32,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_register_value {
    pub reg128: hv_u128,
    pub reg64: u64,
    pub reg32: u32,
    pub reg16: u16,
    pub reg8: u8,
    pub segment: hv_x64_segment_register,
    pub table: hv_x64_table_register,
    pub explicit_suspend: hv_explicit_suspend_register,
    pub intercept_suspend: hv_intercept_suspend_register,
    pub dispatch_suspend: hv_dispatch_suspend_register,

    pub interrupt_state: hv_arm64_interrupt_state_register,
    pub pending_interruption: hv_arm64_pending_interruption_register,

    pub interrupt_state: hv_x64_interrupt_state_register,
    pub pending_interruption: hv_x64_pending_interruption_register,

    pub pending_synthetic_exception_event: hv_arm64_pending_synthetic_exception_event,
}

// NOTE: Linux helper struct - NOT from Hyper-V code.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_output_get_vp_registers {
    pub values): DECLARE_FLEX_ARRAY(union hv_register_value,,
}

// HvGetVpRegisters returns an array of these output elements
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_get_vp_registers_output {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: u32,
    pub __packed: } as32,
    pub low: u64,
    pub high: u64,
    pub __packed: } as64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_register_assoc {
    pub /: *mut *mut u32 name; / enum hv_register_name,
    pub reserved1: u32,
    pub reserved2: u64,
    pub value: hv_register_value,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_get_vp_registers {
    pub partition_id: u64,
    pub vp_index: u32,
    pub input_vtl: hv_input_vtl,
    pub rsvd_z8: u8,
    pub rsvd_z16: u16,
    pub names: [u32; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_set_vp_registers {
    pub partition_id: u64,
    pub vp_index: u32,
    pub input_vtl: hv_input_vtl,
    pub rsvd_z8: u8,
    pub rsvd_z16: u16,
    pub elements: [hv_register_assoc; ],
    pub __packed: },
pub const HV_UNMAP_GPA_LARGE_PAGE: c_uint = 0x2;
// HvCallSendSyntheticClusterIpi hypercall
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_send_ipi {
    pub vector: u32,
    pub reserved: u32,
    pub cpu_mask: u64,
    pub __packed: },

// Hyper-V memory host visibility
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_mem_host_visibility {
    VMBUS_PAGE_NOT_VISIBLE		= 0,
    VMBUS_PAGE_VISIBLE_READ_ONLY	= 1,
    VMBUS_PAGE_VISIBLE_READ_WRITE	= 3
}

// HvCallModifySparseGpaPageHostVisibility hypercall

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_gpa_range_for_visibility {
    pub partition_id: u64,
    pub 2: u32 host_visibility :,
    pub 30: u32 reserved0 :,
    pub reserved1: u32,
    pub gpa_page_list: [u64; HV_MAX_MODIFY_GPA_REP_COUNT],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_msi_address_register {
    pub as_uint32: u32,
    pub 2: u32 reserved1 :,
    pub 1: u32 destination_mode :,
    pub 1: u32 redirection_hint :,
    pub 8: u32 reserved2 :,
    pub 8: u32 destination_id :,
    pub 12: u32 msi_base :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_msi_data_register {
    pub as_uint32: u32,
    pub 8: u32 vector :,
    pub 3: u32 delivery_mode :,
    pub 3: u32 reserved1 :,
    pub 1: u32 level_assert :,
    pub 1: u32 trigger_mode :,
    pub 16: u32 reserved2 :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_msi_entry {
    pub as_uint64: u64,
    pub address: hv_msi_address_register,
    pub data: hv_msi_data_register,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_msi_entry {
    pub as_uint64: [u64; 2],
    pub address: u64,
    pub data: u32,
    pub reserved: u32,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_ioapic_rte {
    pub as_uint64: u64,
    pub 8: u32 vector :,
    pub 3: u32 delivery_mode :,
    pub 1: u32 destination_mode :,
    pub 1: u32 delivery_status :,
    pub 1: u32 interrupt_polarity :,
    pub 1: u32 remote_irr :,
    pub 1: u32 trigger_mode :,
    pub 1: u32 interrupt_mask :,
    pub 15: u32 reserved1 :,
    pub 24: u32 reserved2 :,
    pub 8: u32 destination_id :,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_interrupt_source {
    HV_INTERRUPT_SOURCE_MSI = 1, /* MSI and MSI-X */
    HV_INTERRUPT_SOURCE_IOAPIC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_interrupt_entry {
    pub source: u32,
    pub reserved1: u32,
    pub msi_entry: hv_msi_entry,
    pub ioapic_rte: hv_ioapic_rte,
}

pub const HV_DEVICE_INTERRUPT_TARGET_MULTICAST: c_int = 1;
pub const HV_DEVICE_INTERRUPT_TARGET_PROCESSOR_SET: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_device_interrupt_target {
    pub vector: u32,
    pub /: *mut *mut *mut u32 flags; / HV_DEVICE_INTERRUPT_TARGET_ above,
    pub vp_mask: u64,
    pub vp_set: hv_vpset,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_retarget_device_interrupt {
    pub /: *mut *mut u64 partition_id; / use "self",
    pub device_id: u64,
    pub int_entry: hv_interrupt_entry,
    pub reserved2: u64,
    pub int_target: hv_device_interrupt_target,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_intercept_type {

    HV_INTERCEPT_TYPE_X64_IO_PORT			= 0x00000000,
    HV_INTERCEPT_TYPE_X64_MSR			= 0x00000001,
    HV_INTERCEPT_TYPE_X64_CPUID			= 0x00000002,

    HV_INTERCEPT_TYPE_EXCEPTION			= 0x00000003,
// Used to be HV_INTERCEPT_TYPE_REGISTER
    HV_INTERCEPT_TYPE_RESERVED0			= 0x00000004,
    HV_INTERCEPT_TYPE_MMIO				= 0x00000005,

    HV_INTERCEPT_TYPE_X64_GLOBAL_CPUID		= 0x00000006,
    HV_INTERCEPT_TYPE_X64_APIC_SMI			= 0x00000007,

    HV_INTERCEPT_TYPE_HYPERCALL			= 0x00000008,

    HV_INTERCEPT_TYPE_X64_APIC_INIT_SIPI		= 0x00000009,
    HV_INTERCEPT_MC_UPDATE_PATCH_LEVEL_MSR_READ	= 0x0000000A,
    HV_INTERCEPT_TYPE_X64_APIC_WRITE		= 0x0000000B,
    HV_INTERCEPT_TYPE_X64_MSR_INDEX			= 0x0000000C,

    HV_INTERCEPT_TYPE_MAX,
    HV_INTERCEPT_TYPE_INVALID			= 0xFFFFFFFF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_intercept_parameters {
// HV_INTERCEPT_PARAMETERS is defined to be an 8-byte field.
    pub as_uint64: u64,

// HV_INTERCEPT_TYPE_X64_IO_PORT
    pub io_port: u16,
// HV_INTERCEPT_TYPE_X64_CPUID
    pub cpuid_index: u32,
// HV_INTERCEPT_TYPE_X64_APIC_WRITE
    pub apic_write_mask: u32,
// HV_INTERCEPT_TYPE_EXCEPTION
    pub exception_vector: u16,
// HV_INTERCEPT_TYPE_X64_MSR_INDEX
    pub msr_index: u32,

// N.B. Other intercept types do not have any parameters.
}

// Data structures for HVCALL_MMIO_READ and HVCALL_MMIO_WRITE
pub const HV_HYPERCALL_MMIO_MAX_DATA_LENGTH: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_mmio_read_input {
    pub gpa: u64,
    pub size: u32,
    pub reserved: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_mmio_read_output {
    pub data: [u8; HV_HYPERCALL_MMIO_MAX_DATA_LENGTH],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_mmio_write_input {
    pub gpa: u64,
    pub size: u32,
    pub reserved: u32,
    pub data: [u8; HV_HYPERCALL_MMIO_MAX_DATA_LENGTH],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_intercept_access_type {
    HV_INTERCEPT_ACCESS_READ	= 0,
    HV_INTERCEPT_ACCESS_WRITE	= 1,
    HV_INTERCEPT_ACCESS_EXECUTE	= 2
}
