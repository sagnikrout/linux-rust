//! Automatically rewritten from C Header to Rust Module
//! Source: include/hyperv/hvhdk_mini.h
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
// Type definitions for the Microsoft Hypervisor.
//

pub const HV_MAX_CONTIGUOUS_ALLOCATION_PAGES: c_int = 8;
//
// Doorbell connection_info flags.
//
pub const HV_DOORBELL_FLAG_TRIGGER_SIZE_MASK: c_uint = 0x00000007;
pub const HV_DOORBELL_FLAG_TRIGGER_SIZE_ANY: c_uint = 0x00000000;
pub const HV_DOORBELL_FLAG_TRIGGER_SIZE_BYTE: c_uint = 0x00000001;
pub const HV_DOORBELL_FLAG_TRIGGER_SIZE_WORD: c_uint = 0x00000002;
pub const HV_DOORBELL_FLAG_TRIGGER_SIZE_DWORD: c_uint = 0x00000003;
pub const HV_DOORBELL_FLAG_TRIGGER_SIZE_QWORD: c_uint = 0x00000004;
pub const HV_DOORBELL_FLAG_TRIGGER_ANY_VALUE: c_uint = 0x80000000;
// Each generic set contains 64 elements

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_generic_set_format {
    HV_GENERIC_SET_SPARSE_4K,
    HV_GENERIC_SET_ALL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_scheduler_type {
    HV_SCHEDULER_TYPE_LP		= 1, /* Classic scheduler w/o SMT */
    HV_SCHEDULER_TYPE_LP_SMT	= 2, /* Classic scheduler w/ SMT */
    HV_SCHEDULER_TYPE_CORE_SMT	= 3, /* Core scheduler */
    HV_SCHEDULER_TYPE_ROOT		= 4, /* Root / integrated scheduler */
    HV_SCHEDULER_TYPE_MAX
}

// HV_STATS_AREA_TYPE
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_stats_area_type {
    HV_STATS_AREA_SELF = 0,
    HV_STATS_AREA_PARENT = 1,
    HV_STATS_AREA_INTERNAL = 2,
    HV_STATS_AREA_COUNT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_stats_object_type {
    HV_STATS_OBJECT_HYPERVISOR		= 0x00000001,
    HV_STATS_OBJECT_LOGICAL_PROCESSOR	= 0x00000002,
    HV_STATS_OBJECT_PARTITION		= 0x00010001,
    HV_STATS_OBJECT_VP			= 0x00010002
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_stats_object_identity {
// hv_stats_hypervisor
    pub reserved: [u8; 15],
    pub stats_area_type: u8,
    pub hv: } __packed,
// hv_stats_logical_processor
    pub lp_index: u32,
    pub reserved: [u8; 11],
    pub stats_area_type: u8,
    pub lp: } __packed,
// hv_stats_partition
    pub partition_id: u64,
    pub reserved: [u8; 7],
    pub stats_area_type: u8,
    pub partition: } __packed,
// hv_stats_vp
    pub partition_id: u64,
    pub vp_index: u32,
    pub flags: u16,
    pub reserved: u8,
    pub stats_area_type: u8,
    pub vp: } __packed,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_partition_property_code {
// Privilege properties
    HV_PARTITION_PROPERTY_PRIVILEGE_FLAGS			= 0x00010000,
    HV_PARTITION_PROPERTY_SYNTHETIC_PROC_FEATURES		= 0x00010001,

// Integrated scheduling properties
    HV_PARTITION_PROPERTY_INTEGRATED_SCHEDULER_ENABLED	= 0x00020005,

// Resource properties
    HV_PARTITION_PROPERTY_GPA_PAGE_ACCESS_TRACKING		= 0x00050005,
    HV_PARTITION_PROPERTY_UNIMPLEMENTED_MSR_ACTION		= 0x00050017,

// Compatibility properties
    HV_PARTITION_PROPERTY_PROCESSOR_XSAVE_FEATURES		= 0x00060002,
    HV_PARTITION_PROPERTY_XSAVE_STATES                      = 0x00060007,
    HV_PARTITION_PROPERTY_MAX_XSAVE_DATA_SIZE		= 0x00060008,
    HV_PARTITION_PROPERTY_PROCESSOR_CLOCK_FREQUENCY		= 0x00060009,

// Extended properties with larger property values
    HV_PARTITION_PROPERTY_VMM_CAPABILITIES			= 0x00090007,
}

pub const HV_PARTITION_VMM_CAPABILITIES_BANK_COUNT: c_int = 1;
pub const HV_PARTITION_VMM_CAPABILITIES_RESERVED_BITFIELD_COUNT: c_int = 57;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_partition_property_vmm_capabilities {
    pub bank_count: u16,
    pub reserved: [u16; 3],
    pub as_uint64: [u64; HV_PARTITION_VMM_CAPABILITIES_BANK_COUNT],
    pub 1: u64 map_gpa_preserve_adjustable:,
    pub 1: u64 vmm_can_provide_overlay_gpfn:,
    pub 1: u64 vp_affinity_property:,

    pub 1: u64 vmm_can_provide_gic_overlay_locations:,

    pub 1: u64 reservedbit3:,

    pub 1: u64 assignable_synthetic_proc_features:,
    pub 1: u64 reservedbit5:,
    pub 1: u64 vmm_enable_integrated_scheduler :,
    pub HV_PARTITION_VMM_CAPABILITIES_RESERVED_BITFIELD_COUNT: u64 reserved0:,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_snp_status {
    HV_SNP_STATUS_NONE = 0,
    HV_SNP_STATUS_AVAILABLE = 1,
    HV_SNP_STATUS_INCOMPATIBLE = 2,
    HV_SNP_STATUS_PSP_UNAVAILABLE = 3,
    HV_SNP_STATUS_PSP_INIT_FAILED = 4,
    HV_SNP_STATUS_PSP_BAD_FW_VERSION = 5,
    HV_SNP_STATUS_BAD_CONFIGURATION = 6,
    HV_SNP_STATUS_PSP_FW_UPDATE_IN_PROGRESS = 7,
    HV_SNP_STATUS_PSP_RB_INIT_FAILED = 8,
    HV_SNP_STATUS_PSP_PLATFORM_STATUS_FAILED = 9,
    HV_SNP_STATUS_PSP_INIT_LATE_FAILED = 10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_system_property {
// Add more values when needed
    HV_SYSTEM_PROPERTY_SLEEP_STATE = 3,
    HV_SYSTEM_PROPERTY_SCHEDULER_TYPE = 15,
    HV_DYNAMIC_PROCESSOR_FEATURE_PROPERTY = 21,
    HV_SYSTEM_PROPERTY_CRASHDUMPAREA = 47,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_pfn_range {
    pub as_uint64: u64,
// 39:0: base pfn.  63:40: additional pages
    pub HV_PFN_RANGE_PGBITS: u64 base_pfn : 64 -,
    pub HV_PFN_RANGE_PGBITS: u64 add_pfns :,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_sleep_state {
    HV_SLEEP_STATE_S1 = 1,
    HV_SLEEP_STATE_S2 = 2,
    HV_SLEEP_STATE_S3 = 3,
    HV_SLEEP_STATE_S4 = 4,
    HV_SLEEP_STATE_S5 = 5,
//
// After hypervisor has received this, any follow up sleep
// state registration requests will be rejected.
//
    HV_SLEEP_STATE_LOCK = 6
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_dynamic_processor_feature_property {
// Add more values when needed
    HV_X64_DYNAMIC_PROCESSOR_FEATURE_MAX_ENCRYPTED_PARTITIONS = 13,
    HV_X64_DYNAMIC_PROCESSOR_FEATURE_SNP_STATUS = 16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_get_system_property {
    pub /: *mut *mut u32 property_id; / enum hv_system_property,
    pub reserved: u32,
    pub as_uint64: u64,

// enum hv_dynamic_processor_feature_property
    pub hv_processor_feature: u32,

// More fields to be filled in when needed
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_output_get_system_property {
    pub /: *mut *mut u32 scheduler_type; / enum hv_scheduler_type,

    pub hv_processor_feature_value: u64,

    pub /: *mut *mut hv_pfn_range hv_cda_info; / CrashdumpAreaAddress,
    pub /: *mut *mut u64 hv_tramp_pa; / CrashdumpTrampolineAddress,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_sleep_state_info {
    pub /: *mut *mut u32 sleep_state; / enum hv_sleep_state,
    pub pm1a_slp_typ: u8,
    pub pm1b_slp_typ: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_set_system_property {
    pub /: *mut *mut u32 property_id; / enum hv_system_property,
    pub reserved: u32,
// More fields to be filled in when needed
    pub set_sleep_state_info: hv_sleep_state_info,
//
// Add a reserved field to ensure the union is 8-byte aligned as
// existing members may not be. This is a temporary measure
// until all remaining members are added.
//
    pub reserved0: [u64; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_enter_sleep_state {
    pub /: *mut *mut u32 sleep_state; / enum hv_sleep_state,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_map_stats_page {
    pub /: *mut *mut u32 type; / enum hv_stats_object_type,
    pub padding: u32,
    pub identity: hv_stats_object_identity,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_map_stats_page2 {
    pub /: *mut *mut u32 type; / enum hv_stats_object_type,
    pub padding: u32,
    pub identity: hv_stats_object_identity,
    pub map_location: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_output_map_stats_page {
    pub map_location: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_unmap_stats_page {
    pub /: *mut *mut u32 type; / enum hv_stats_object_type,
    pub padding: u32,
    pub identity: hv_stats_object_identity,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_proximity_domain_flags {
    pub 1: u32 proximity_preferred :,
    pub 30: u32 reserved :,
    pub 1: u32 proximity_info_valid :,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_proximity_domain_info {
    pub domain_id: u32,
    pub flags: hv_proximity_domain_flags,
    pub __packed: },
// HvDepositMemory hypercall
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_deposit_memory {
    pub partition_id: u64,
    pub gpa_page_list: [u64; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_withdraw_memory {
    pub partition_id: u64,
    pub proximity_domain_info: hv_proximity_domain_info,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_output_withdraw_memory {
    pub gpa_page_list): DECLARE_FLEX_ARRAY(u64,,
    pub __packed: },
// HV Map GPA (Guest Physical Address) Flags
pub const HV_MAP_GPA_PERMISSIONS_NONE: c_uint = 0x0;
pub const HV_MAP_GPA_READABLE: c_uint = 0x1;
pub const HV_MAP_GPA_WRITABLE: c_uint = 0x2;
pub const HV_MAP_GPA_KERNEL_EXECUTABLE: c_uint = 0x4;
pub const HV_MAP_GPA_USER_EXECUTABLE: c_uint = 0x8;
pub const HV_MAP_GPA_EXECUTABLE: c_uint = 0xC;
pub const HV_MAP_GPA_PERMISSIONS_MASK: c_uint = 0xF;
pub const HV_MAP_GPA_ADJUSTABLE: c_uint = 0x8000;
pub const HV_MAP_GPA_NO_ACCESS: c_uint = 0x10000;
pub const HV_MAP_GPA_NOT_CACHED: c_uint = 0x200000;
pub const HV_MAP_GPA_LARGE_PAGE: c_uint = 0x80000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_map_gpa_pages {
    pub target_partition_id: u64,
    pub target_gpa_base: u64,
    pub map_flags: u32,
    pub padding: u32,
    pub source_gpa_page_list: [u64; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_gpa_page_access_state_flags {
    pub 1: u64 clear_accessed :,
    pub 1: u64 set_accessed :,
    pub 1: u64 clear_dirty :,
    pub 1: u64 set_dirty :,
    pub 60: u64 reserved :,
    pub __packed: },
    pub as_uint64: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_get_gpa_pages_access_state {
    pub partition_id: u64,
    pub flags: hv_gpa_page_access_state_flags,
    pub hv_gpa_page_number: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_gpa_page_access_state {
    pub 1: u8 accessed :,
    pub 1: u8 dirty :,
    pub 6: u8 reserved:,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_crashdump_action {
    HV_CRASHDUMP_NONE = 0,
    HV_CRASHDUMP_SUSPEND_ALL_VPS,
    HV_CRASHDUMP_PREPARE_FOR_STATE_SAVE,
    HV_CRASHDUMP_STATE_SAVED,
    HV_CRASHDUMP_ENTRY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_partition_event_root_crashdump_input {
    pub /: *mut *mut u32 crashdump_action; / enum hv_crashdump_action,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_disable_hyp_ex {
    pub rip: u64,
    pub arg: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_crashdump_area {
    pub version: u32,
    pub flags_as_uint32: u32,
    pub 1: u32 cda_valid :,
    pub 31: u32 cda_unused :,
    pub __packed: },
}

// more unused fields
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_partition_event_input {
    pub crashdump_input: hv_partition_event_root_crashdump_input,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_partition_event {
    HV_PARTITION_EVENT_ROOT_CRASHDUMP = 2,
    HV_PARTITION_ALL_LOGICAL_PROCESSORS_STARTED = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_notify_partition_event {
    pub /: *mut *mut u32 event; / enum hv_partition_event,
    pub input: hv_partition_event_input,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_get_logical_processor_run_time {
    pub lp_index: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_output_get_logical_processor_run_time {
    pub global_time: u64,
    pub local_run_time: u64,
    pub rsvdz0: u64,
    pub hypervisor_time: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_lp_startup_status {
    pub hv_status: u64,
    pub substatus1: u64,
    pub substatus2: u64,
    pub substatus3: u64,
    pub substatus4: u64,
    pub substatus5: u64,
    pub substatus6: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_add_logical_processor {
    pub lp_index: u32,
    pub apic_id: u32,
    pub proximity_domain_info: hv_proximity_domain_info,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_output_add_logical_processor {
    pub startup_status: hv_lp_startup_status,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_create_vp {
    pub partition_id: u64,
    pub vp_index: u32,
    pub padding: [u8; 3],
    pub subnode_type: u8,
    pub subnode_id: u64,
    pub proximity_domain_info: hv_proximity_domain_info,
    pub flags: u64,
    pub __packed: },
// HV_INTERRUPT_TRIGGER_MODE
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_interrupt_trigger_mode {
    HV_INTERRUPT_TRIGGER_MODE_EDGE	= 0,
    HV_INTERRUPT_TRIGGER_MODE_LEVEL	= 1,
}

// HV_DEVICE_INTERRUPT_DESCRIPTOR
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_device_interrupt_descriptor {
    pub interrupt_type: u32,
    pub trigger_mode: u32,
    pub vector_count: u32,
    pub reserved: u32,
    pub target: hv_device_interrupt_target,
    pub __packed: },
// HV_INPUT_MAP_DEVICE_INTERRUPT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_map_device_interrupt {
    pub partition_id: u64,
    pub device_id: u64,
    pub flags: u32,
    pub base_irt_idx: u32,
    pub logical_interrupt_entry: hv_interrupt_entry,
    pub interrupt_descriptor: hv_device_interrupt_descriptor,
    pub __packed: },
// HV_OUTPUT_MAP_DEVICE_INTERRUPT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_output_map_device_interrupt {
    pub interrupt_entry: hv_interrupt_entry,
    pub ext_status_deprecated: [u64; 5],
    pub __packed: },
// HV_INPUT_UNMAP_DEVICE_INTERRUPT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_unmap_device_interrupt {
    pub partition_id: u64,
    pub device_id: u64,
    pub interrupt_entry: hv_interrupt_entry,
    pub flags: u32,
    pub __packed: },
pub const HV_SOURCE_SHADOW_NONE: c_uint = 0x0;
pub const HV_SOURCE_SHADOW_BRIDGE_BUS_RANGE: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_send_ipi_ex {
    pub vector: u32,
    pub reserved: u32,
    pub vp_set: hv_vpset,
    pub __packed: },
    pub /: *mut *mut typedef u16 hv_pci_rid; / HV_PCI_RID,
    pub /: *mut *mut typedef u16 hv_pci_segment; / HV_PCI_SEGMENT,
pub type hv_logical_device_id = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_pci_bdf {
    pub as_uint16: u16,
    pub 3: u8 function :,
    pub 5: u8 device :,
    pub bus: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_pci_bus_range {
    pub as_uint16: u16,
    pub subordinate_bus: u8,
    pub secondary_bus: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_device_type {
    HV_DEVICE_TYPE_LOGICAL	= 0,
    HV_DEVICE_TYPE_PCI	= 1,
    HV_DEVICE_TYPE_IOAPIC	= 2,
    HV_DEVICE_TYPE_ACPI	= 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_device_id {
    pub as_uint64: u64,
    pub 62: u64 reserved0 :,
    pub 2: u64 device_type :,
}

// HV_DEVICE_TYPE_LOGICAL
// HV_DEVICE_TYPE_PCI
// HV_DEVICE_TYPE_IOAPIC
// HV_DEVICE_TYPE_ACPI
