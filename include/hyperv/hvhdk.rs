//! Automatically rewritten from C Header to Rust Module
//! Source: include/hyperv/hvhdk.h
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

//
// Hypervisor statistics page format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_stats_page {
    pub sizeof(u64)]: u64 data[HV_HYP_PAGE_SIZE /,
    pub __packed: },
// Bits for dirty mask of hv_vp_register_page
pub const HV_X64_REGISTER_CLASS_GENERAL: c_int = 0;
pub const HV_X64_REGISTER_CLASS_IP: c_int = 1;
pub const HV_X64_REGISTER_CLASS_XMM: c_int = 2;
pub const HV_X64_REGISTER_CLASS_SEGMENT: c_int = 3;
pub const HV_X64_REGISTER_CLASS_FLAGS: c_int = 4;

pub const HV_VP_REGISTER_PAGE_MAX_VECTOR_COUNT: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_vp_register_page_interrupt_vectors {
    pub as_uint64: u64,
    pub vector_count: u8,
    pub vector: [u8; HV_VP_REGISTER_PAGE_MAX_VECTOR_COUNT],
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_vp_register_page {
    pub version: u16,
    pub isvalid: u8,
    pub rsvdz: u8,
    pub dirty: u32,

// General purpose registers
// (HV_X64_REGISTER_CLASS_GENERAL)
//
    pub rax: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rbx: u64,
    pub rsp: u64,
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
    pub __packed: },
    pub gp_registers: [u64; 16],
}

// Instruction pointer (HV_X64_REGISTER_CLASS_IP)
// Flags (HV_X64_REGISTER_CLASS_FLAGS)
// Volatile XMM registers (HV_X64_REGISTER_CLASS_XMM)
// Segment registers (HV_X64_REGISTER_CLASS_SEGMENT)
// Misc. control registers (cannot be set via this interface)
//
// Fields from this point are not included in the register page save chunk.
// The reserved field is intended to maintain alignment for unsaved fields.
//
// Interrupts injected as part of HvCallDispatchVp.
//

// Not yet supported in ARM

pub const HV_PARTITION_PROCESSOR_FEATURES_BANKS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_partition_processor_features {
    pub as_uint64: [u64; HV_PARTITION_PROCESSOR_FEATURES_BANKS],
    pub 1: u64 sse3_support :,
    pub 1: u64 lahf_sahf_support :,
    pub 1: u64 ssse3_support :,
    pub 1: u64 sse4_1_support :,
    pub 1: u64 sse4_2_support :,
    pub 1: u64 sse4a_support :,
    pub 1: u64 xop_support :,
    pub 1: u64 pop_cnt_support :,
    pub 1: u64 cmpxchg16b_support :,
    pub 1: u64 altmovcr8_support :,
    pub 1: u64 lzcnt_support :,
    pub 1: u64 mis_align_sse_support :,
    pub 1: u64 mmx_ext_support :,
    pub 1: u64 amd3dnow_support :,
    pub 1: u64 extended_amd3dnow_support :,
    pub 1: u64 page_1gb_support :,
    pub 1: u64 aes_support :,
    pub 1: u64 pclmulqdq_support :,
    pub 1: u64 pcid_support :,
    pub 1: u64 fma4_support :,
    pub 1: u64 f16c_support :,
    pub 1: u64 rd_rand_support :,
    pub 1: u64 rd_wr_fs_gs_support :,
    pub 1: u64 smep_support :,
    pub 1: u64 enhanced_fast_string_support :,
    pub 1: u64 bmi1_support :,
    pub 1: u64 bmi2_support :,
    pub 1: u64 hle_support_deprecated :,
    pub 1: u64 rtm_support_deprecated :,
    pub 1: u64 movbe_support :,
    pub 1: u64 npiep1_support :,
    pub 1: u64 dep_x87_fpu_save_support :,
    pub 1: u64 rd_seed_support :,
    pub 1: u64 adx_support :,
    pub 1: u64 intel_prefetch_support :,
    pub 1: u64 smap_support :,
    pub 1: u64 hle_support :,
    pub 1: u64 rtm_support :,
    pub 1: u64 rdtscp_support :,
    pub 1: u64 clflushopt_support :,
    pub 1: u64 clwb_support :,
    pub 1: u64 sha_support :,
    pub 1: u64 x87_pointers_saved_support :,
    pub 1: u64 invpcid_support :,
    pub 1: u64 ibrs_support :,
    pub 1: u64 stibp_support :,
    pub 1: u64 ibpb_support:,
    pub 1: u64 unrestricted_guest_support :,
    pub 1: u64 mdd_support :,
    pub 1: u64 fast_short_rep_mov_support :,
    pub 1: u64 l1dcache_flush_support :,
    pub 1: u64 rdcl_no_support :,
    pub 1: u64 ibrs_all_support :,
    pub 1: u64 skip_l1df_support :,
    pub 1: u64 ssb_no_support :,
    pub 1: u64 rsb_a_no_support :,
    pub 1: u64 virt_spec_ctrl_support :,
    pub 1: u64 rd_pid_support :,
    pub 1: u64 umip_support :,
    pub 1: u64 mbs_no_support :,
    pub 1: u64 mb_clear_support :,
    pub 1: u64 taa_no_support :,
    pub 1: u64 tsx_ctrl_support :,
//
// N.B. The final processor feature bit in bank 0 is reserved to
// simplify potential downlevel backports.
//
    pub 1: u64 reserved_bank0 :,
// N.B. Begin bank 1 processor features.
    pub 1: u64 acount_mcount_support :,
    pub 1: u64 tsc_invariant_support :,
    pub 1: u64 cl_zero_support :,
    pub 1: u64 rdpru_support :,
    pub 1: u64 la57_support :,
    pub 1: u64 mbec_support :,
    pub 1: u64 nested_virt_support :,
    pub 1: u64 psfd_support :,
    pub 1: u64 cet_ss_support :,
    pub 1: u64 cet_ibt_support :,
    pub 1: u64 vmx_exception_inject_support :,
    pub 1: u64 enqcmd_support :,
    pub 1: u64 umwait_tpause_support :,
    pub 1: u64 movdiri_support :,
    pub 1: u64 movdir64b_support :,
    pub 1: u64 cldemote_support :,
    pub 1: u64 serialize_support :,
    pub 1: u64 tsc_deadline_tmr_support :,
    pub 1: u64 tsc_adjust_support :,
    pub 1: u64 fzlrep_movsb :,
    pub 1: u64 fsrep_stosb :,
    pub 1: u64 fsrep_cmpsb :,
    pub 42: u64 reserved_bank1 :,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_partition_processor_xsave_features {
    pub 1: u64 xsave_support :,
    pub 1: u64 xsaveopt_support :,
    pub 1: u64 avx_support :,
    pub 61: u64 reserved1 :,
    pub __packed: },
    pub as_uint64: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_partition_creation_properties {
    pub disabled_processor_features: hv_partition_processor_features,
    pub __packed: },
pub const HV_PARTITION_SYNTHETIC_PROCESSOR_FEATURES_BANKS: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_partition_synthetic_processor_features {
    pub as_uint64: [u64; HV_PARTITION_SYNTHETIC_PROCESSOR_FEATURES_BANKS],
    pub 1: u64 hypervisor_present :,
// Support for HV#1: (CPUID leaves 0x40000000 - 0x40000006)
    pub 1: u64 hv1 :,
    pub /: *mut *mut u64 access_vp_run_time_reg : 1; / HV_X64_MSR_VP_RUNTIME,
    pub /: *mut *mut u64 access_partition_reference_counter : 1; / HV_X64_MSR_TIME_REF_COUNT,
    pub /: *mut *mut u64 access_synic_regs : 1; / SINT-related registers,
//
// Access to HV_X64_MSR_STIMER0_CONFIG through
// HV_X64_MSR_STIMER3_COUNT.
//
    pub 1: u64 access_synthetic_timer_regs :,
    pub page*/: *mut *mut u64 access_intr_ctrl_regs : 1; / APIC MSRs and VP assist,
// HV_X64_MSR_GUEST_OS_ID and HV_X64_MSR_HYPERCALL
    pub 1: u64 access_hypercall_regs :,
    pub 1: u64 access_vp_index :,
    pub 1: u64 access_partition_reference_tsc :,
    pub 1: u64 access_guest_idle_reg :,
    pub 1: u64 access_frequency_regs :,
    pub 1: u64 reserved_z12 :,
    pub 1: u64 reserved_z13 :,
    pub 1: u64 reserved_z14 :,
    pub 1: u64 enable_extended_gva_ranges_for_flush_virtual_address_list :,
    pub 1: u64 reserved_z16 :,
    pub 1: u64 reserved_z17 :,
// Use fast hypercall output. Corresponds to privilege.
    pub 1: u64 fast_hypercall_output :,
    pub 1: u64 reserved_z19 :,
    pub /: *mut *mut u64 start_virtual_processor : 1; / Can start VPs,
    pub 1: u64 reserved_z21 :,
// Synthetic timers in direct mode.
    pub 1: u64 direct_synthetic_timers :,
    pub 1: u64 reserved_z23 :,
    pub 1: u64 extended_processor_masks :,
// Enable various hypercalls
    pub 1: u64 tb_flush_hypercalls :,
    pub 1: u64 synthetic_cluster_ipi :,
    pub 1: u64 notify_long_spin_wait :,
    pub 1: u64 query_numa_distance :,
    pub 1: u64 signal_events :,
    pub 1: u64 retarget_device_interrupt :,
    pub 1: u64 restore_time :,
// EnlightenedVmcs nested enlightenment is supported.
    pub 1: u64 enlightened_vmcs :,
    pub 31: u64 reserved :,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_partition_isolation_properties {
    pub as_uint64: u64,
    pub 5: u64 isolation_type:,
    pub 2: u64 isolation_host_type :,
    pub 5: u64 rsvd_z:,
    pub 52: u64 shared_gpa_boundary_page_number:,
    pub __packed: },
}

//
// Various isolation types supported by MSHV.
//
pub const HV_PARTITION_ISOLATION_TYPE_NONE: c_int = 0;
pub const HV_PARTITION_ISOLATION_TYPE_SNP: c_int = 2;
pub const HV_PARTITION_ISOLATION_TYPE_TDX: c_int = 3;
//
// Various host isolation types supported by MSHV.
//
pub const HV_PARTITION_ISOLATION_HOST_TYPE_NONE: c_uint = 0x0;
pub const HV_PARTITION_ISOLATION_HOST_TYPE_HARDWARE: c_uint = 0x1;
pub const HV_PARTITION_ISOLATION_HOST_TYPE_RESERVED: c_uint = 0x2;
// Note: Exo partition is enabled by default

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_create_partition {
    pub flags: u64,
    pub proximity_domain_info: hv_proximity_domain_info,
    pub compatibility_version: u32,
    pub padding: u32,
    pub partition_creation_properties: hv_partition_creation_properties,
    pub isolation_properties: hv_partition_isolation_properties,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_output_create_partition {
    pub partition_id: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_initialize_partition {
    pub partition_id: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_finalize_partition {
    pub partition_id: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_delete_partition {
    pub partition_id: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_get_partition_property {
    pub partition_id: u64,
    pub /: *mut *mut u32 property_code; / enum hv_partition_property_code,
    pub padding: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_output_get_partition_property {
    pub property_value: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_set_partition_property {
    pub partition_id: u64,
    pub /: *mut *mut u32 property_code; / enum hv_partition_property_code,
    pub padding: u32,
    pub property_value: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_partition_property_arg {
    pub as_uint64: u64,
    pub arg: u32,
    pub vp_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_get_partition_property_ex {
    pub partition_id: u64,
    pub /: *mut *mut u32 property_code; / enum hv_partition_property_code,
    pub padding: u32,
    pub arg_data: hv_partition_property_arg,
    pub arg: u64,
}

//
// NOTE: Should use hv_input_set_partition_property_ex_header to compute this
// size, but hv_input_get_partition_property_ex is identical so it suffices
//

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_partition_property_ex {
    pub buffer: [u8; HV_PARTITION_PROPERTY_EX_MAX_VAR_SIZE],
    pub vmm_capabilities: hv_partition_property_vmm_capabilities,
// More fields to be filled in when needed
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_output_get_partition_property_ex {
    pub property_value: hv_partition_property_ex,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_vp_state_page_type {
    HV_VP_STATE_PAGE_REGISTERS = 0,
    HV_VP_STATE_PAGE_INTERCEPT_MESSAGE = 1,
    HV_VP_STATE_PAGE_GHCB = 2,
    HV_VP_STATE_PAGE_COUNT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_map_vp_state_page {
    pub partition_id: u64,
    pub vp_index: u32,
    pub /: *mut *mut u16 type; / enum hv_vp_state_page_type,
    pub input_vtl: hv_input_vtl,
    pub as_uint8: u8,
    pub 1: u8 map_location_provided :,
    pub 7: u8 reserved :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_output_map_vp_state_page {
    pub /: *mut *mut u64 map_location; / GPA page number,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_unmap_vp_state_page {
    pub partition_id: u64,
    pub vp_index: u32,
    pub /: *mut *mut u16 type; / enum hv_vp_state_page_type,
    pub input_vtl: hv_input_vtl,
    pub reserved0: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_x64_apic_eoi_message {
    pub vp_index: u32,
    pub interrupt_vector: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_opaque_intercept_message {
    pub vp_index: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_port_type {
    HV_PORT_TYPE_MESSAGE = 1,
    HV_PORT_TYPE_EVENT   = 2,
    HV_PORT_TYPE_MONITOR = 3,
    HV_PORT_TYPE_DOORBELL = 4	/* Root Partition only */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_port_info {
    pub /: *mut *mut u32 port_type; / enum hv_port_type,
    pub padding: u32,
    pub target_sint: u32,
    pub target_vp: u32,
    pub rsvdz: u64,
    pub message_port_info: },
    pub target_sint: u32,
    pub target_vp: u32,
    pub base_flag_number: u16,
    pub flag_count: u16,
    pub rsvdz: u32,
    pub event_port_info: },
    pub monitor_address: u64,
    pub rsvdz: u64,
    pub monitor_port_info: },
    pub target_sint: u32,
    pub target_vp: u32,
    pub rsvdz: u64,
    pub doorbell_port_info: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_connection_info {
    pub port_type: u32,
    pub padding: u32,
    pub rsvdz: u64,
    pub message_connection_info: },
    pub rsvdz: u64,
    pub event_connection_info: },
    pub monitor_address: u64,
    pub monitor_connection_info: },
    pub gpa: u64,
    pub trigger_value: u64,
    pub flags: u64,
    pub doorbell_connection_info: },
}

// Define synthetic interrupt controller flag constants.

// linux side we create long version of flags to use long bit ops on flags

// Define the synthetic interrupt controller event flags format.
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_synic_event_flags {
    pub flags8: [c_uchar; HV_EVENT_FLAGS_BYTE_COUNT],
    pub flags32: [u32; HV_EVENT_FLAGS32_COUNT],
    pub /: *mut *mut ulong flags[HV_EVENT_FLAGS_UL_COUNT]; / linux only,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_synic_event_flags_page {
    pub event_flags: [volatile union hv_synic_event_flags; HV_SYNIC_SINT_COUNT],
}

pub const HV_SYNIC_EVENT_RING_MESSAGE_COUNT: c_int = 63;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_synic_event_ring {
    pub signal_masked: u8,
    pub ring_full: u8,
    pub reserved_z: u16,
    pub data: [u32; HV_SYNIC_EVENT_RING_MESSAGE_COUNT],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_synic_event_ring_page {
    pub sint_event_ring: [hv_synic_event_ring; HV_SYNIC_SINT_COUNT],
}

// Define SynIC control register.
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_synic_scontrol {
    pub as_uint64: u64,
    pub 1: u64 enable :,
    pub 63: u64 reserved :,
    pub __packed: },
}

// Define the format of the SIEFP register
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_synic_siefp {
    pub as_uint64: u64,
    pub 1: u64 siefp_enabled :,
    pub 11: u64 preserved :,
    pub 52: u64 base_siefp_gpa :,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_synic_sirbp {
    pub as_uint64: u64,
    pub 1: u64 sirbp_enabled :,
    pub 11: u64 preserved :,
    pub 52: u64 base_sirbp_gpa :,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_interrupt_control {
    pub as_uint64: u64,
    pub /: *mut *mut u32 interrupt_type; / enum hv_interrupt_type,

    pub 1: u32 level_triggered :,
    pub 1: u32 logical_dest_mode :,
    pub 30: u32 rsvd :,

    pub 2: u32 rsvd1 :,
    pub 1: u32 asserted :,
    pub 29: u32 rsvd2 :,

    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_stimer_state {
    pub 1: u32 undelivered_msg_pending :,
    pub 31: u32 reserved :,
    pub flags: } __packed,
    pub resvd: u32,
    pub config: u64,
    pub count: u64,
    pub adjustment: u64,
    pub undelivered_exp_time: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_synthetic_timers_state {
    pub timers: [hv_stimer_state; HV_SYNIC_STIMER_COUNT],
    pub reserved: [u64; 5],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_async_completion_message_payload {
    pub partition_id: u64,
    pub status: u32,
    pub completion_count: u32,
    pub sub_status: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_input_delete_vp {
    pub as_uint64: [u64; 2],
    pub partition_id: u64,
    pub vp_index: u32,
    pub reserved: [u8; 4],
    pub __packed: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_assert_virtual_interrupt {
    pub partition_id: u64,
    pub control: hv_interrupt_control,
    pub /: *mut *mut u64 dest_addr; / cpu's apic id,
    pub vector: u32,
    pub target_vtl: u8,
    pub rsvd_z0: u8,
    pub rsvd_z1: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_create_port {
    pub port_partition_id: u64,
    pub port_id: hv_port_id,
    pub port_vtl: u8,
    pub min_connection_vtl: u8,
    pub padding: u16,
    pub connection_partition_id: u64,
    pub port_info: hv_port_info,
    pub proximity_domain_info: hv_proximity_domain_info,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_input_delete_port {
    pub as_uint64: [u64; 2],
    pub port_partition_id: u64,
    pub port_id: hv_port_id,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_connect_port {
    pub connection_partition_id: u64,
    pub connection_id: hv_connection_id,
    pub connection_vtl: u8,
    pub rsvdz0: u8,
    pub rsvdz1: u16,
    pub port_partition_id: u64,
    pub port_id: hv_port_id,
    pub reserved2: u32,
    pub connection_info: hv_connection_info,
    pub proximity_domain_info: hv_proximity_domain_info,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_input_disconnect_port {
    pub as_uint64: [u64; 2],
    pub connection_partition_id: u64,
    pub connection_id: hv_connection_id,
    pub 1: u32 is_doorbell:,
    pub 31: u32 reserved:,
    pub __packed: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_input_notify_port_ring_empty {
    pub as_uint64: u64,
    pub sint_index: u32,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_vp_state_data_xsave {
    pub flags: u64,
    pub states: hv_x64_xsave_xfem_register,
    pub __packed: },
//
// For getting and setting VP state, there are two options based on the state type:
//
// 1.) Data that is accessed by PFNs in the input hypercall page. This is used
// for state which may not fit into the hypercall pages.
// 2.) Data that is accessed directly in the input\output hypercall pages.
// This is used for state that will always fit into the hypercall pages.
//
// In the future this could be dynamic based on the size if needed.
//
// Note these hypercalls have an 8-byte aligned variable header size as per the tlfs
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_get_set_vp_state_type {
// HvGetSetVpStateLocalInterruptControllerState - APIC/GIC state
    HV_GET_SET_VP_STATE_LAPIC_STATE	     = 0 | HV_GET_SET_VP_STATE_TYPE_PFN,
    HV_GET_SET_VP_STATE_XSAVE	     = 1 | HV_GET_SET_VP_STATE_TYPE_PFN,
    HV_GET_SET_VP_STATE_SIM_PAGE	     = 2 | HV_GET_SET_VP_STATE_TYPE_PFN,
    HV_GET_SET_VP_STATE_SIEF_PAGE	     = 3 | HV_GET_SET_VP_STATE_TYPE_PFN,
    HV_GET_SET_VP_STATE_SYNTHETIC_TIMERS = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_vp_state_data {
    pub type: u32,
    pub rsvd: u32,
    pub xsave: hv_vp_state_data_xsave,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_get_vp_state {
    pub partition_id: u64,
    pub vp_index: u32,
    pub input_vtl: u8,
    pub rsvd0: u8,
    pub rsvd1: u16,
    pub state_data: hv_vp_state_data,
    pub output_data_pfns: [u64; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_output_get_vp_state {
    pub synthetic_timers_state: hv_synthetic_timers_state,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_input_set_vp_state_data {
    pub pfns: u64,
    pub bytes: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_set_vp_state {
    pub partition_id: u64,
    pub vp_index: u32,
    pub input_vtl: u8,
    pub rsvd0: u8,
    pub rsvd1: u16,
    pub state_data: hv_vp_state_data,
    pub data: [hv_input_set_vp_state_data; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_x64_vp_execution_state {
    pub as_uint16: u16,
    pub cpl:2: u16,
    pub cr0_pe:1: u16,
    pub cr0_am:1: u16,
    pub efer_lma:1: u16,
    pub debug_active:1: u16,
    pub interruption_pending:1: u16,
    pub vtl:4: u16,
    pub enclave_mode:1: u16,
    pub interrupt_shadow:1: u16,
    pub virtualization_fault_active:1: u16,
    pub reserved:2: u16,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_x64_intercept_message_header {
    pub vp_index: u32,
    pub instruction_length:4: u8,
    pub /: *mut *mut u8 cr8:4; / Only set for exo partitions,
    pub /: *mut *mut u8 intercept_access_type; / enum hv_intercept_access_type,
    pub execution_state: hv_x64_vp_execution_state,
    pub cs_segment: hv_x64_segment_register,
    pub rip: u64,
    pub rflags: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_x64_memory_access_info {
    pub as_uint8: u8,
    pub gva_valid:1: u8,
    pub gva_gpa_valid:1: u8,
    pub hypercall_output_pending:1: u8,
    pub tlb_locked_no_overlay:1: u8,
    pub reserved:4: u8,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_x64_memory_intercept_message {
    pub header: hv_x64_intercept_message_header,
    pub /: *mut *mut u32 cache_type; / enum hv_cache_type,
    pub instruction_byte_count: u8,
    pub memory_access_info: hv_x64_memory_access_info,
    pub tpr_priority: u8,
    pub reserved1: u8,
    pub guest_virtual_address: u64,
    pub guest_physical_address: u64,
    pub instruction_bytes: [u8; 16],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_arm64_vp_execution_state {
    pub as_uint16: u16,
    pub /: *mut *mut u16 cpl:2; / Exception Level (EL),
    pub debug_active:1: u16,
    pub interruption_pending:1: u16,
    pub vtl:4: u16,
    pub virtualization_fault_active:1: u16,
    pub reserved:7: u16,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_arm64_intercept_message_header {
    pub vp_index: u32,
    pub instruction_length: u8,
    pub /: *mut *mut u8 intercept_access_type; / enum hv_intercept_access_type,
    pub execution_state: hv_arm64_vp_execution_state,
    pub pc: u64,
    pub cpsr: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_arm64_memory_access_info {
    pub as_uint8: u8,
    pub gva_valid:1: u8,
    pub gva_gpa_valid:1: u8,
    pub hypercall_output_pending:1: u8,
    pub reserved:5: u8,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_arm64_memory_intercept_message {
    pub header: hv_arm64_intercept_message_header,
    pub /: *mut *mut u32 cache_type; / enum hv_cache_type,
    pub instruction_byte_count: u8,
    pub memory_access_info: hv_arm64_memory_access_info,
    pub reserved1: u16,
    pub instruction_bytes: [u8; 4],
    pub reserved2: u32,
    pub guest_virtual_address: u64,
    pub guest_physical_address: u64,
    pub syndrome: u64,
    pub __packed: },

//
// Dispatch state for the VP communicated by the hypervisor to the
// VP-dispatching thread in the root on return from HVCALL_DISPATCH_VP.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_vp_dispatch_state {
    HV_VP_DISPATCH_STATE_INVALID	= 0,
    HV_VP_DISPATCH_STATE_BLOCKED	= 1,
    HV_VP_DISPATCH_STATE_READY	= 2,
}

//
// Dispatch event that caused the current dispatch state on return from
// HVCALL_DISPATCH_VP.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_vp_dispatch_event {
    HV_VP_DISPATCH_EVENT_INVALID	= 0x00000000,
    HV_VP_DISPATCH_EVENT_SUSPEND	= 0x00000001,
    HV_VP_DISPATCH_EVENT_INTERCEPT	= 0x00000002,
}

pub const HV_ROOT_SCHEDULER_MAX_VPS_PER_CHILD_PARTITION: c_int = 1024;
// The maximum array size of HV_GENERIC_SET (vp_set) buffer

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_vp_signal_bitset_scheduler_message {
    pub partition_id: u64,
    pub overflow_count: u32,
    pub vp_count: u16,
    pub reserved: u16,

    pub bitset: hv_vpset,
    pub bitset_buffer: [u64; BITSET_BUFFER_SIZE],
    pub vp_bitset: },

    pub __packed: },
    pub hv_message_header))): (sizeof(struct hv_message) - sizeof(struct,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_vp_signal_pair_scheduler_message {
    pub overflow_count: u32,
    pub vp_count: u8,
    pub reserved1: [u8; 3],
    pub partition_ids: [u64; HV_MESSAGE_MAX_PARTITION_VP_PAIR_COUNT],
    pub vp_indexes: [u32; HV_MESSAGE_MAX_PARTITION_VP_PAIR_COUNT],
    pub reserved2: [u8; 4],
    pub __packed: },
    pub hv_message_header))): (sizeof(struct hv_message) - sizeof(struct,
// Input and output structures for HVCALL_DISPATCH_VP
pub const HV_DISPATCH_VP_FLAG_CLEAR_INTERCEPT_SUSPEND: c_uint = 0x1;
pub const HV_DISPATCH_VP_FLAG_ENABLE_CALLER_INTERRUPTS: c_uint = 0x2;
pub const HV_DISPATCH_VP_FLAG_SET_CALLER_SPEC_CTRL: c_uint = 0x4;
pub const HV_DISPATCH_VP_FLAG_SKIP_VP_SPEC_FLUSH: c_uint = 0x8;
pub const HV_DISPATCH_VP_FLAG_SKIP_CALLER_SPEC_FLUSH: c_uint = 0x10;
pub const HV_DISPATCH_VP_FLAG_SKIP_CALLER_USER_SPEC_FLUSH: c_uint = 0x20;
pub const HV_DISPATCH_VP_FLAG_SCAN_INTERRUPT_INJECTION: c_uint = 0x40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_dispatch_vp {
    pub partition_id: u64,
    pub vp_index: u32,
    pub flags: u32,
    pub /: *mut *mut u64 time_slice; / in 100ns,
    pub spec_ctrl: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_output_dispatch_vp {
    pub /: *mut *mut u32 dispatch_state; / enum hv_vp_dispatch_state,
    pub /: *mut *mut u32 dispatch_event; / enum hv_vp_dispatch_event,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_modify_sparse_spa_page_host_access {
    pub 2: u32 host_access :,
    pub 30: u32 reserved :,
    pub flags: u32,
    pub partition_id: u64,
    pub spa_page_list: [u64; ],
    pub __packed: },
// hv_input_modify_sparse_spa_page_host_access flags
pub const HV_MODIFY_SPA_PAGE_HOST_ACCESS_MAKE_EXCLUSIVE: c_uint = 0x1;
pub const HV_MODIFY_SPA_PAGE_HOST_ACCESS_MAKE_SHARED: c_uint = 0x2;
pub const HV_MODIFY_SPA_PAGE_HOST_ACCESS_LARGE_PAGE: c_uint = 0x4;
pub const HV_MODIFY_SPA_PAGE_HOST_ACCESS_HUGE_PAGE: c_uint = 0x8;
