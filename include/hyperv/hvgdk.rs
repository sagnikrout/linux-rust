//! Automatically rewritten from C Header to Rust Module
//! Source: include/hyperv/hvgdk.h
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

//
// The guest OS needs to register the guest ID with the hypervisor.
// The guest ID is a 64-bit entity and the structure of this ID is
// specified in the Hyper-V TLFS specification.
//
// Bit(s)
// 63 - Indicates if the OS is Open Source or not; 1 is Open Source
// 62:56 - OS Type; Linux is 0x1
// 55:48 - Distro specific identification
// 47:16 - Linux kernel version number
// 15:0  - Distro specific identification
//
pub const HV_LINUX_VENDOR_ID: c_uint = 0x8100;
// HV_VMX_ENLIGHTENED_VMCS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_enlightened_vmcs {
    pub revision_id: u32,
    pub abort: u32,
    pub host_es_selector: u16,
    pub host_cs_selector: u16,
    pub host_ss_selector: u16,
    pub host_ds_selector: u16,
    pub host_fs_selector: u16,
    pub host_gs_selector: u16,
    pub host_tr_selector: u16,
    pub padding16_1: u16,
    pub host_ia32_pat: u64,
    pub host_ia32_efer: u64,
    pub host_cr0: u64,
    pub host_cr3: u64,
    pub host_cr4: u64,
    pub host_ia32_sysenter_esp: u64,
    pub host_ia32_sysenter_eip: u64,
    pub host_rip: u64,
    pub host_ia32_sysenter_cs: u32,
    pub pin_based_vm_exec_control: u32,
    pub vm_exit_controls: u32,
    pub secondary_vm_exec_control: u32,
    pub io_bitmap_a: u64,
    pub io_bitmap_b: u64,
    pub msr_bitmap: u64,
    pub guest_es_selector: u16,
    pub guest_cs_selector: u16,
    pub guest_ss_selector: u16,
    pub guest_ds_selector: u16,
    pub guest_fs_selector: u16,
    pub guest_gs_selector: u16,
    pub guest_ldtr_selector: u16,
    pub guest_tr_selector: u16,
    pub guest_es_limit: u32,
    pub guest_cs_limit: u32,
    pub guest_ss_limit: u32,
    pub guest_ds_limit: u32,
    pub guest_fs_limit: u32,
    pub guest_gs_limit: u32,
    pub guest_ldtr_limit: u32,
    pub guest_tr_limit: u32,
    pub guest_gdtr_limit: u32,
    pub guest_idtr_limit: u32,
    pub guest_es_ar_bytes: u32,
    pub guest_cs_ar_bytes: u32,
    pub guest_ss_ar_bytes: u32,
    pub guest_ds_ar_bytes: u32,
    pub guest_fs_ar_bytes: u32,
    pub guest_gs_ar_bytes: u32,
    pub guest_ldtr_ar_bytes: u32,
    pub guest_tr_ar_bytes: u32,
    pub guest_es_base: u64,
    pub guest_cs_base: u64,
    pub guest_ss_base: u64,
    pub guest_ds_base: u64,
    pub guest_fs_base: u64,
    pub guest_gs_base: u64,
    pub guest_ldtr_base: u64,
    pub guest_tr_base: u64,
    pub guest_gdtr_base: u64,
    pub guest_idtr_base: u64,
    pub padding64_1: [u64; 3],
    pub vm_exit_msr_store_addr: u64,
    pub vm_exit_msr_load_addr: u64,
    pub vm_entry_msr_load_addr: u64,
    pub cr3_target_value0: u64,
    pub cr3_target_value1: u64,
    pub cr3_target_value2: u64,
    pub cr3_target_value3: u64,
    pub page_fault_error_code_mask: u32,
    pub page_fault_error_code_match: u32,
    pub cr3_target_count: u32,
    pub vm_exit_msr_store_count: u32,
    pub vm_exit_msr_load_count: u32,
    pub vm_entry_msr_load_count: u32,
    pub tsc_offset: u64,
    pub virtual_apic_page_addr: u64,
    pub vmcs_link_pointer: u64,
    pub guest_ia32_debugctl: u64,
    pub guest_ia32_pat: u64,
    pub guest_ia32_efer: u64,
    pub guest_pdptr0: u64,
    pub guest_pdptr1: u64,
    pub guest_pdptr2: u64,
    pub guest_pdptr3: u64,
    pub guest_pending_dbg_exceptions: u64,
    pub guest_sysenter_esp: u64,
    pub guest_sysenter_eip: u64,
    pub guest_activity_state: u32,
    pub guest_sysenter_cs: u32,
    pub cr0_guest_host_mask: u64,
    pub cr4_guest_host_mask: u64,
    pub cr0_read_shadow: u64,
    pub cr4_read_shadow: u64,
    pub guest_cr0: u64,
    pub guest_cr3: u64,
    pub guest_cr4: u64,
    pub guest_dr7: u64,
    pub host_fs_base: u64,
    pub host_gs_base: u64,
    pub host_tr_base: u64,
    pub host_gdtr_base: u64,
    pub host_idtr_base: u64,
    pub host_rsp: u64,
    pub ept_pointer: u64,
    pub virtual_processor_id: u16,
    pub padding16_2: [u16; 3],
    pub padding64_2: [u64; 5],
    pub guest_physical_address: u64,
    pub vm_instruction_error: u32,
    pub vm_exit_reason: u32,
    pub vm_exit_intr_info: u32,
    pub vm_exit_intr_error_code: u32,
    pub idt_vectoring_info_field: u32,
    pub idt_vectoring_error_code: u32,
    pub vm_exit_instruction_len: u32,
    pub vmx_instruction_info: u32,
    pub exit_qualification: u64,
    pub exit_io_instruction_ecx: u64,
    pub exit_io_instruction_esi: u64,
    pub exit_io_instruction_edi: u64,
    pub exit_io_instruction_eip: u64,
    pub guest_linear_address: u64,
    pub guest_rsp: u64,
    pub guest_rflags: u64,
    pub guest_interruptibility_info: u32,
    pub cpu_based_vm_exec_control: u32,
    pub exception_bitmap: u32,
    pub vm_entry_controls: u32,
    pub vm_entry_intr_info_field: u32,
    pub vm_entry_exception_error_code: u32,
    pub vm_entry_instruction_len: u32,
    pub tpr_threshold: u32,
    pub guest_rip: u64,
    pub hv_clean_fields: u32,
    pub padding32_1: u32,
    pub hv_synthetic_controls: u32,
    pub nested_flush_hypercall:1: u32,
    pub msr_bitmap:1: u32,
    pub reserved:30: u32,
    pub hv_enlightenments_control: } __packed,
    pub hv_vp_id: u32,
    pub padding32_2: u32,
    pub hv_vm_id: u64,
    pub partition_assist_page: u64,
    pub padding64_4: [u64; 4],
    pub guest_bndcfgs: u64,
    pub guest_ia32_perf_global_ctrl: u64,
    pub guest_ia32_s_cet: u64,
    pub guest_ssp: u64,
    pub guest_ia32_int_ssp_table_addr: u64,
    pub guest_ia32_lbr_ctl: u64,
    pub padding64_5: [u64; 2],
    pub xss_exit_bitmap: u64,
    pub encls_exiting_bitmap: u64,
    pub host_ia32_perf_global_ctrl: u64,
    pub tsc_multiplier: u64,
    pub host_ia32_s_cet: u64,
    pub host_ssp: u64,
    pub host_ia32_int_ssp_table_addr: u64,
    pub padding64_6: u64,
    pub __packed: },
pub const HV_VMX_ENLIGHTENED_CLEAN_FIELD_NONE: c_int = 0;

pub const HV_VMX_ENLIGHTENED_CLEAN_FIELD_ALL: c_uint = 0xFFFF;
//
// Note, Hyper-V isn't actually stealing bit 28 from Intel, just abusing it by
// pairing it with architecturally impossible exit reasons.  Bit 28 is set only
// on SMI exits to a SMI transfer monitor (STM) and if and only if a MTF VM-Exit
// is pending.  I.e. it will never be set by hardware for non-SMI exits (there
// are only three), nor will it ever be set unless the VMM is an STM.
//
pub const HV_VMX_SYNTHETIC_EXIT_REASON_TRAP_AFTER_FLUSH: c_uint = 0x10000031;
//
// Hyper-V uses the software reserved 32 bytes in VMCB control area to expose
// SVM enlightenments to guests. This is documented in the TLFS doc.
// Note on naming: SVM_NESTED_ENLIGHTENED_VMCB_FIELDS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_vmcb_enlightenments {
    pub 1: u32 nested_flush_hypercall :,
    pub 1: u32 msr_bitmap :,
    pub 1: u32 enlightened_npt_tlb:,
    pub 29: u32 reserved :,
    pub hv_enlightenments_control: } __packed,
    pub hv_vp_id: u32,
    pub hv_vm_id: u64,
    pub partition_assist_page: u64,
    pub reserved: u64,
    pub __packed: },
//
// Hyper-V uses the software reserved clean bit in VMCB.
//
pub const HV_VMCB_NESTED_ENLIGHTENMENTS: c_int = 31;
// Synthetic VM-Exit
pub const HV_SVM_EXITCODE_ENL: c_uint = 0xf0000000ull;

// VM_PARTITION_ASSIST_PAGE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_partition_assist_pg {
    pub tlb_lock_count: u32,
}

// Define connection identifier type.
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_connection_id {
    pub asu32: u32,
    pub 24: u32 id :,
    pub 8: u32 reserved :,
    pub u: } __packed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_unmap_gpa_pages {
    pub target_partition_id: u64,
    pub target_gpa_base: u64,
    pub unmap_flags: u32,
    pub padding: u32,
    pub __packed: },
