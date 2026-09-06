//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/svm/svm.h
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
// Kernel-based Virtual Machine driver for Linux
//
// AMD SVM support
//
// Copyright (C) 2006 Qumranet, Inc.
// Copyright 2010 Red Hat, Inc. and/or its affiliates.
//
// Authors:
// Yaniv Kamay  <yaniv@qumranet.com>
// Avi Kivity   <avi@qumranet.com>
//

//
// Helpers to convert to/from physical addresses for pages whose address is
// consumed directly by hardware.  Even though it's a physical address, SVM
// often restricts the address to the natural width, hence 'unsigned long'
// instead of 'hpa_t'.
//
extern "C" {
    pub fn __sme_set(PAGE_SHIFT: page_to_pfn(page) <<) -> return;
}
extern "C" {
    pub fn pfn_to_page(PAGE_SHIFT: __sme_clr(pa) >>) -> return;
}

//
// Clean bits in VMCB.
// VMCB_ALL_CLEAN_MASK might also need to
// be updated if this enum is modified.
//
// AVIC PHYSICAL_TABLE pointer,
// AVIC LOGICAL_TABLE pointer
//

// TPR and CR2 are always written before VMRUN

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sev_info {
    pub /: *mut *mut bool active; / SEV enabled guest,
    pub /: *mut *mut bool es_active; / SEV-ES enabled guest,
    pub /: *mut *mut bool need_init; / waiting for SEV_INIT2,
    pub /: *mut *mut unsigned int asid; / ASID used for this guest,
    pub /: *mut *mut unsigned int handle; / SEV firmware handle,
    pub /: *mut *mut int fd; / SEV device fd,
    pub policy: c_ulong,
    pub /: *mut *mut unsigned long pages_locked; / Number of pages locked,
    pub /: *mut *mut list_head regions_list; / List of registered regions,
    pub /: *mut *mut u64 ap_jump_table; / SEV-ES AP Jump Table address,
    pub vmsa_features: u64,
    pub /: *mut *mut u16 ghcb_version; / Highest guest GHCB protocol version allowed,
// The three fields below are protected by sev_mirror_lock
    pub /: *mut *mut *mut kvm enc_context_owner; / Owner of copied encryption context,
    pub /: *mut *mut list_head mirror_vms; / List of VMs mirroring,
    pub /: *mut *mut list_head mirror_entry; / Use as a list entry of mirrors,
    pub /: *mut *mut *mut misc_cg misc_cg; / For misc cgroup accounting,
    pub migration_in_progress: core::sync::atomic::AtomicI32,
    pub /: *mut *mut *mut void snp_context; / SNP guest context page,
    pub /: *mut *mut *mut void guest_req_buf; / Bounce buffer for SNP Guest Request input,
    pub /: *mut *mut *mut void guest_resp_buf; / Bounce buffer for SNP Guest Request output,
    pub /: *mut *mut mutex guest_req_mutex; / Must acquire before using bounce buffers,
    pub /: *mut *mut cpumask_var_t have_run_cpus; / CPUs that have done VMRUN for this VM.,
    pub /: *mut *mut bool snp_certs_enabled; / SNP certificate-fetching support.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_svm {
    pub kvm: kvm,
// Struct members for AVIC
    pub avic_vm_id: u32,
    pub avic_logical_id_table: *mut u32,
    pub avic_physical_id_table: *mut u64,
    pub hnode: hlist_node,

    pub sev_info: kvm_sev_info,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vmcb_info {
    pub ptr: *mut vmcb,
    pub pa: c_ulong,
    pub cpu: c_int,
    pub asid_generation: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmcb_save_area_cached {
    pub es: vmcb_seg,
    pub cs: vmcb_seg,
    pub ss: vmcb_seg,
    pub ds: vmcb_seg,
    pub gdtr: vmcb_seg,
    pub idtr: vmcb_seg,
    pub cpl: u8,
    pub efer: u64,
    pub cr4: u64,
    pub cr3: u64,
    pub cr0: u64,
    pub dr7: u64,
    pub dr6: u64,
    pub rflags: u64,
    pub rip: u64,
    pub rsp: u64,
    pub s_cet: u64,
    pub ssp: u64,
    pub isst_addr: u64,
    pub rax: u64,
    pub cr2: u64,
    pub g_pat: u64,
    pub dbgctl: u64,
    pub br_from: u64,
    pub br_to: u64,
    pub last_excp_from: u64,
    pub last_excp_to: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmcb_ctrl_area_cached {
    pub intercepts: [u32; MAX_INTERCEPT],
    pub pause_filter_thresh: u16,
    pub pause_filter_count: u16,
    pub iopm_base_pa: u64,
    pub msrpm_base_pa: u64,
    pub tsc_offset: u64,
    pub asid: u32,
    pub tlb_ctl: u8,
    pub erap_ctl: u8,
    pub int_ctl: u32,
    pub int_vector: u32,
    pub int_state: u32,
    pub exit_code: u64,
    pub exit_info_1: u64,
    pub exit_info_2: u64,
    pub exit_int_info: u32,
    pub exit_int_info_err: u32,
    pub misc_ctl: u64,
    pub event_inj: u32,
    pub event_inj_err: u32,
    pub next_rip: u64,
    pub nested_cr3: u64,
    pub misc_ctl2: u64,
    pub clean: u32,

    pub hv_enlightenments: hv_vmcb_enlightenments,
    pub reserved_sw: [u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct svm_nested_state {
    pub vmcb02: kvm_vmcb_info,
    pub hsave_msr: u64,
    pub vm_cr_msr: u64,
    pub vmcb12_gpa: u64,
    pub last_vmcb12_gpa: u64,
    pub last_bus_lock_rip: u64,
//
// The MSR permissions map used for vmcb02, which is the merge result
// of vmcb01 and vmcb12
//
    pub msrpm: *mut c_void,
// cache for control fields of the guest
    pub ctl: vmcb_ctrl_area_cached,
//
// Note: this struct is not kept up-to-date while L2 runs; it is only
// valid within nested_svm_vmrun.
//
    pub save: vmcb_save_area_cached,
    pub initialized: bool,
//
// Indicates whether MSR bitmap for L2 needs to be rebuilt due to
// changes in MSR bitmap for L1 or switching to a different L2. Note,
// this flag can only be used reliably in conjunction with a paravirt L1
// which informs L0 whether any changes to MSR bitmap for L2 were done
// on its side.
//
    pub force_msr_bitmap_recalc: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcpu_sev_es_state {
// SEV-ES support
    pub vmsa: *mut sev_es_save_area,
    pub ghcb: *mut ghcb,
    pub valid_bitmap: [u8; 16],
    pub ghcb_map: kvm_host_map,
    pub received_first_sipi: bool,
    pub ap_reset_hold_type: c_uint,
// SEV-ES scratch area support
    pub sw_scratch: u64,
    pub ghcb_sa: *mut c_void,
    pub ghcb_sa_len: u32,
    pub ghcb_sa_sync: bool,
    pub ghcb_sa_free: bool,
// SNP Page-State-Change buffer entries currently being processed
    pub cur_idx: u16,
    pub end_idx: u16,
    pub batch_size: u16,
    pub is_2m: bool,
    pub psc: },
    pub ghcb_registered_gpa: u64,
    pub /: *mut *mut mutex snp_vmsa_mutex; / Used to handle concurrent updates of VMSA.,
    pub snp_pending_vmsa_gpa: gpa_t,
    pub snp_guest_vmsa_gpa: gpa_t,
    pub snp_ap_waiting_for_reset: bool,
    pub snp_has_guest_vmsa: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcpu_svm {
    pub vcpu: kvm_vcpu,
// vmcb always points at current_vmcb->ptr, it's purely a shorthand.
    pub vmcb: *mut vmcb,
    pub vmcb01: kvm_vmcb_info,
    pub current_vmcb: *mut kvm_vmcb_info,
    pub asid: u32,
    pub sysenter_esp_hi: u32,
    pub sysenter_eip_hi: u32,
    pub tsc_aux: u64,
    pub msr_decfg: u64,
    pub next_rip: u64,
    pub spec_ctrl: u64,
    pub tsc_ratio_msr: u64,
//
// Contains guest-controlled bits of VIRT_SPEC_CTRL, which will be
// translated into the appropriate L2_CFG bits on the host to
// perform speculative control.
//
    pub virt_spec_ctrl: u64,
    pub msrpm: *mut c_void,
    pub nmi_iret_rip: c_ulong,
    pub nested: svm_nested_state,
// NMI mask value, used when vNMI is not enabled
    pub nmi_masked: bool,
//
// True when NMIs are still masked but guest IRET was just intercepted
// and KVM is waiting for RIP to change, which will signal that the
// intercepted IRET was retired and thus NMI can be unmasked.
//
    pub awaiting_iret_completion: bool,
//
// Set when KVM is awaiting IRET completion and needs to inject NMIs as
// soon as the IRET completes (e.g. NMI is pending injection).  KVM
// temporarily steals RFLAGS.TF to single-step the guest in this case
// in order to regain control as soon as the NMI-blocking condition
// goes away.
//
    pub nmi_singlestep: bool,
    pub nmi_singlestep_guest_rflags: u64,
    pub nmi_l1_to_l2: bool,
    pub soft_int_csbase: c_ulong,
    pub soft_int_old_rip: c_ulong,
    pub soft_int_next_rip: c_ulong,
    pub soft_int_injected: bool,
    pub ldr_reg: u32,
    pub dfr_reg: u32,
// This is essentially a shadow of the vCPU's actual entry in the
// Physical ID table that is programmed into the VMCB, i.e. that is
// seen by the CPU.  If IPI virtualization is disabled, IsRunning is
// only ever set in the shadow, i.e. is never propagated to the "real"
// table, so that hardware never sees IsRunning=1.
//
    pub avic_physical_id_entry: u64,
//
// Per-vCPU list of irqfds that are eligible to post IRQs directly to
// the vCPU (a.k.a. device posted IRQs, a.k.a. IRQ bypass).  The list
// is used to reconfigure IRTEs when the vCPU is loaded/put (to set the
// target pCPU), when AVIC is toggled on/off (to (de)activate bypass),
// and if the irqfd becomes ineligible for posting (to put the IRTE
// back into remapped mode).
//
    pub ir_list: list_head,
    pub ir_list_lock: raw_spinlock_t,
    pub sev_es: vcpu_sev_es_state,
    pub guest_state_loaded: bool,
    pub avic_irq_window: bool,
    pub x2avic_msrs_intercepted: bool,
    pub lbr_msrs_intercepted: bool,
// Guest GIF value, used when vGIF is not enabled
    pub guest_gif: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct svm_cpu_data {
    pub asid_generation: u64,
    pub max_asid: u32,
    pub next_asid: u32,
    pub min_asid: u32,
    pub bp_spec_reduce_set: bool,
    pub save_area: *mut vmcb,
    pub save_area_pa: c_ulong,
// index = sev_asid, value = vmcb pointer
    pub sev_vmcbs: *mut vmcb,
}

extern "C" {
    pub fn container_of(_arg: kvm, kvm_svm: struct, _arg: kvm) -> return;
}

extern "C" {
    pub fn ____sev_guest(_arg: vcpu->kvm) -> return;
}
extern "C" {
    pub fn ____sev_es_guest(_arg: vcpu->kvm) -> return;
}
extern "C" {
    pub fn ____sev_snp_guest(_arg: vcpu->kvm) -> return;
}

extern "C" {
    pub fn container_of(_arg: vcpu, vcpu_svm: struct, _arg: vcpu) -> return;
}
//
// Only the PDPTRs are loaded on demand into the shadow MMU.  All other
// fields are synchronized on VM-Exit, because accessing the VMCB is cheap.
//
// CR3 might be out of date in the VMCB but it is not marked dirty; instead,
// KVM_REQ_LOAD_MMU_PGD is always requested when the cached vcpu->arch.cr3
// is changed.  svm_load_mmu_pgd() then syncs the new CR3 value into the VMCB.
//

extern "C" {
    pub fn test_bit(_arg: bit, _arg: intercepts) -> return;
}
extern "C" {
    pub fn __vmcb_is_intercept()&control->intercepts: *mut (unsigned long, _arg: bit) -> return;
}
extern "C" {
    pub fn __vmcb_is_intercept()&control->intercepts: *mut (unsigned long, _arg: bit) -> return;
}
extern "C" {
    pub fn nested_vmcb02_recalc_intercepts(svm: *mut vcpu_svm);
}
//
// If L2 is active, recalculate the intercepts for vmcb02 to account
// for the changes made to vmcb01.  All intercept configuration is done
// for vmcb01 and then propagated to vmcb02 to combine KVM's intercepts
// with L1's intercepts (from the vmcb12 snapshot).
//
extern "C" {
    pub fn vmcb_is_intercept(_arg: &svm->vmcb->control, _arg: bit) -> return;
}
//
// If KVM_X86_QUIRK_NESTED_SVM_SHARED_PAT is disabled while a vCPU
// is running, the L2 IA32_PAT semantics for that vCPU are undefined.
//
// 4 msrs per u8, and 4 u8 in u32
//
// The MSRPM is 8KiB in size, divided into four 2KiB ranges (the fourth range
// is reserved).  Each MSR within a range is covered by two bits, one each for
// read (bit 0) and write (bit 1), where a bit value of '1' means intercepted.
//
pub const SVM_MSRPM_BYTES_PER_RANGE: c_int = 2048;
pub const SVM_BITS_PER_MSR: c_int = 2;

// svm.c
extern "C" {
    pub fn svm_alloc_permissions_map(_arg: MSRPM_SIZE, _arg: GFP_KERNEL_ACCOUNT) -> return;
}

extern "C" {
    pub fn svm_vcpu_free_msrpm(msrpm: *mut c_void);
}
extern "C" {
    pub fn svm_enable_lbrv(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn svm_update_lbrv(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn svm_set_efer(vcpu: *mut kvm_vcpu, efer: u64) -> c_int;
}
extern "C" {
    pub fn svm_set_cr0(vcpu: *mut kvm_vcpu, cr0: c_ulong);
}
extern "C" {
    pub fn svm_set_cr4(vcpu: *mut kvm_vcpu, cr4: c_ulong);
}
extern "C" {
    pub fn disable_nmi_singlestep(svm: *mut vcpu_svm);
}
extern "C" {
    pub fn svm_smi_blocked(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn svm_nmi_blocked(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn svm_interrupt_blocked(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn svm_set_gif(svm: *mut vcpu_svm, value: bool);
}
extern "C" {
    pub fn svm_invoke_exit_handler(vcpu: *mut kvm_vcpu, exit_code: u64) -> c_int;
}
extern "C" {
    pub fn svm_set_intercept_for_msr(vcpu: *mut kvm_vcpu, msr: u32, type: c_int, set: bool);
}
extern "C" {
    pub fn svm_skip_emulated_instruction(vcpu: *mut kvm_vcpu) -> c_int;
}
// nested.c

extern "C" {
    pub fn is_guest_mode(V_INTR_MASKING_MASK: vcpu) && (svm->nested.ctl.int_ctl &) -> return;
}
extern "C" {
    pub fn vmcb12_is_intercept(_arg: &svm->nested.ctl, _arg: INTERCEPT_SMI) -> return;
}
extern "C" {
    pub fn vmcb12_is_intercept(_arg: &svm->nested.ctl, _arg: INTERCEPT_INTR) -> return;
}
extern "C" {
    pub fn vmcb12_is_intercept(_arg: &svm->nested.ctl, _arg: INTERCEPT_NMI) -> return;
}
extern "C" {
    pub fn nested_svm_init_msrpm_merge_offsets() -> int __init;
}
extern "C" {
    pub fn enter_svm_guest_mode(vcpu: *mut kvm_vcpu, vmcb_gpa: u64, from_vmrun: bool) -> c_int;
}
extern "C" {
    pub fn svm_leave_nested(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn svm_free_nested(svm: *mut vcpu_svm);
}
extern "C" {
    pub fn svm_allocate_nested(svm: *mut vcpu_svm) -> c_int;
}
extern "C" {
    pub fn nested_svm_vmrun(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn svm_copy_vmloadsave_state(to_vmcb: *mut vmcb, from_vmcb: *mut vmcb);
}
extern "C" {
    pub fn nested_svm_vmexit(svm: *mut vcpu_svm);
}
extern "C" {
    pub fn nested_svm_exit_handled(svm: *mut vcpu_svm) -> c_int;
}
extern "C" {
    pub fn nested_svm_check_permissions(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn nested_svm_check_cached_vmcb12(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn nested_svm_exit_special(svm: *mut vcpu_svm) -> c_int;
}
extern "C" {
    pub fn nested_svm_update_tsc_ratio_msr(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn svm_write_tsc_multiplier(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn nested_sync_control_from_vmcb02(svm: *mut vcpu_svm);
}
extern "C" {
    pub fn svm_switch_vmcb(svm: *mut vcpu_svm, target_vmcb: *mut kvm_vmcb_info);
}
//
// Do NOT defer reprogramming the counters by default.  Instructions
// causing a state change are counted based on the _new_ CPU state
// (e.g. a successful VMRUN is counted in guest mode). Hence, the
// counters should be reprogrammed with the new state _before_ the
// instruction is potentially counted upon emulation completion.
//
// avic.c

extern "C" {
    pub fn avic_hardware_setup() -> bool __init;
}
extern "C" {
    pub fn avic_hardware_unsetup();
}
extern "C" {
    pub fn avic_vcpu_precreate(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn avic_vm_pre_destroy(kvm: *mut kvm);
}
extern "C" {
    pub fn avic_vm_destroy(kvm: *mut kvm);
}
extern "C" {
    pub fn avic_init_vmcb(svm: *mut vcpu_svm, vmcb: *mut vmcb);
}
extern "C" {
    pub fn avic_incomplete_ipi_interception(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn avic_unaccelerated_access_interception(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn avic_init_vcpu(svm: *mut vcpu_svm) -> c_int;
}
extern "C" {
    pub fn avic_vcpu_load(vcpu: *mut kvm_vcpu, cpu: c_int);
}
extern "C" {
    pub fn avic_vcpu_put(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn avic_apicv_post_state_restore(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn avic_refresh_apicv_exec_ctrl(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn avic_vcpu_blocking(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn avic_vcpu_unblocking(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn avic_ring_doorbell(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn avic_vcpu_get_apicv_inhibit_reasons(vcpu: *mut kvm_vcpu) -> c_ulong;
}
extern "C" {
    pub fn avic_refresh_virtual_apic_mode(vcpu: *mut kvm_vcpu);
}
// sev.c
extern "C" {
    pub fn pre_sev_run(svm: *mut vcpu_svm, cpu: c_int) -> c_int;
}
extern "C" {
    pub fn sev_init_vmcb(svm: *mut vcpu_svm, init_event: bool);
}
extern "C" {
    pub fn sev_vcpu_after_set_cpuid(svm: *mut vcpu_svm);
}
extern "C" {
    pub fn sev_es_string_io(svm: *mut vcpu_svm, size: c_int, port: c_uint, in: c_int) -> c_int;
}
extern "C" {
    pub fn sev_es_recalc_msr_intercepts(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn sev_vcpu_deliver_sipi_vector(vcpu: *mut kvm_vcpu, vector: u8);
}
extern "C" {
    pub fn sev_es_prepare_switch_to_guest(svm: *mut vcpu_svm, hostsa: *mut sev_es_save_area);
}
extern "C" {
    pub fn sev_es_unmap_ghcb(svm: *mut vcpu_svm);
}

extern "C" {
    pub fn sev_vcpu_needs_initialization(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn sev_mem_enc_ioctl(kvm: *mut kvm, argp: *mut void __user) -> c_int;
}
extern "C" {
    pub fn sev_vm_copy_enc_context_from(kvm: *mut kvm, source_fd: c_uint) -> c_int;
}
extern "C" {
    pub fn sev_vm_move_enc_context_from(kvm: *mut kvm, source_fd: c_uint) -> c_int;
}
extern "C" {
    pub fn sev_guest_memory_reclaimed(kvm: *mut kvm);
}
extern "C" {
    pub fn sev_handle_vmgexit(vcpu: *mut kvm_vcpu) -> c_int;
}
// These symbols are used in common code and are stubbed below.
extern "C" {
    pub fn snp_safe_alloc_page_node(_arg: numa_node_id(), _arg: GFP_KERNEL_ACCOUNT) -> return;
}
extern "C" {
    pub fn sev_snp_reload_vmsa(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn sev_vcpu_create(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn sev_free_vcpu(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn sev_vm_init(kvm: *mut kvm);
}
extern "C" {
    pub fn sev_vm_destroy(kvm: *mut kvm);
}
extern "C" {
    pub fn sev_set_cpu_caps() -> void __init;
}
extern "C" {
    pub fn sev_hardware_setup() -> void __init;
}
extern "C" {
    pub fn sev_hardware_unsetup();
}
extern "C" {
    pub fn sev_cpu_init(sd: *mut svm_cpu_data) -> c_int;
}
extern "C" {
    pub fn sev_dev_get_attr(group: u32, attr: u64, val: *mut u64) -> c_int;
}
extern "C" {
    pub fn sev_handle_rmp_fault(vcpu: *mut kvm_vcpu, gpa: gpa_t, error_code: u64);
}
extern "C" {
    pub fn sev_gmem_make_private(kvm: *mut kvm, gfn: gfn_t, pfn: kvm_pfn_t, nr_pages: kvm_pfn_t) -> c_int;
}
extern "C" {
    pub fn sev_gmem_make_shared(pfn: kvm_pfn_t, nr_pages: kvm_pfn_t);
}
extern "C" {
    pub fn sev_gmem_invalidate_range(kvm: *mut kvm, range: *mut kvm_gfn_range);
}
extern "C" {
    pub fn sev_gmem_max_mapping_level(kvm: *mut kvm, pfn: kvm_pfn_t, is_private: bool) -> c_int;
}
extern "C" {
    pub fn sev_free_decrypted_vmsa(vcpu: *mut kvm_vcpu, vmsa: *mut vmcb_save_area);
}

extern "C" {
    pub fn alloc_pages_node(_arg: node, __GFP_ZERO: gfp |, _arg: 0) -> return;
}
extern "C" {
    pub fn snp_safe_alloc_page_node(_arg: numa_node_id(), _arg: GFP_KERNEL_ACCOUNT) -> return;
}
pub const max_sev_asid: c_int = 0;

// vmenter.S
extern "C" {
    pub fn __svm_vcpu_run(svm: *mut vcpu_svm, flags: c_uint);
}

