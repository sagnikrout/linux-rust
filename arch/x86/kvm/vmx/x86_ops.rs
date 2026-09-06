//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/vmx/x86_ops.h
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

extern "C" {
    pub fn vmx_hardware_setup() -> __init int;
}
extern "C" {
    pub fn vmx_hardware_unsetup();
}
extern "C" {
    pub fn vmx_check_processor_compat() -> c_int;
}
extern "C" {
    pub fn vmx_enable_virtualization_cpu() -> c_int;
}
extern "C" {
    pub fn vmx_disable_virtualization_cpu();
}
extern "C" {
    pub fn vmx_emergency_disable_virtualization_cpu();
}
extern "C" {
    pub fn vmx_vm_init(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn vmx_vm_destroy(kvm: *mut kvm);
}
extern "C" {
    pub fn vmx_vcpu_precreate(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn vmx_vcpu_create(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn vmx_vcpu_run(vcpu: *mut kvm_vcpu, run_flags: u64) -> fastpath_t;
}
extern "C" {
    pub fn vmx_vcpu_free(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_vcpu_reset(vcpu: *mut kvm_vcpu, init_event: bool);
}
extern "C" {
    pub fn vmx_vcpu_load(vcpu: *mut kvm_vcpu, cpu: c_int);
}
extern "C" {
    pub fn vmx_vcpu_put(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_handle_exit(vcpu: *mut kvm_vcpu, exit_fastpath: fastpath_t) -> c_int;
}
extern "C" {
    pub fn vmx_handle_exit_irqoff(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_skip_emulated_instruction(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn vmx_update_emulated_instruction(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_unhandleable_emulation_required(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn vmx_set_msr(vcpu: *mut kvm_vcpu, msr_info: *mut msr_data) -> c_int;
}

extern "C" {
    pub fn vmx_smi_allowed(vcpu: *mut kvm_vcpu, for_injection: bool) -> c_int;
}
extern "C" {
    pub fn vmx_enter_smm(vcpu: *mut kvm_vcpu, smram: *mut kvm_smram) -> c_int;
}
extern "C" {
    pub fn vmx_leave_smm(vcpu: *mut kvm_vcpu, smram: *const kvm_smram) -> c_int;
}
extern "C" {
    pub fn vmx_enable_smi_window(vcpu: *mut kvm_vcpu);
}

extern "C" {
    pub fn vmx_apic_init_signal_blocked(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn vmx_migrate_timers(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_set_virtual_apic_mode(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_hwapic_isr_update(vcpu: *mut kvm_vcpu, max_isr: c_int);
}
extern "C" {
    pub fn vmx_sync_pir_to_irr(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn vmx_vcpu_after_set_cpuid(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_has_emulated_msr(kvm: *mut kvm, index: u32) -> bool;
}
extern "C" {
    pub fn vmx_recalc_intercepts(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_prepare_switch_to_guest(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_update_exception_bitmap(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_get_feature_msr(msr: u32, data: *mut u64) -> c_int;
}
extern "C" {
    pub fn vmx_get_msr(vcpu: *mut kvm_vcpu, msr_info: *mut msr_data) -> c_int;
}

extern "C" {
    pub fn vmx_get_segment_base(vcpu: *mut kvm_vcpu, seg: c_int) -> u64;
}
extern "C" {
    pub fn vmx_get_segment(vcpu: *mut kvm_vcpu, var: *mut kvm_segment, seg: c_int);
}
extern "C" {
    pub fn vmx_set_segment(vcpu: *mut kvm_vcpu, var: *mut kvm_segment, seg: c_int);
}
extern "C" {
    pub fn vmx_get_cpl(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn vmx_get_cs_db_l_bits(vcpu: *mut kvm_vcpu, db: *mut c_int, l: *mut c_int);
}
extern "C" {
    pub fn vmx_is_valid_cr0(vcpu: *mut kvm_vcpu, cr0: c_ulong) -> bool;
}
extern "C" {
    pub fn vmx_set_cr0(vcpu: *mut kvm_vcpu, cr0: c_ulong);
}
extern "C" {
    pub fn vmx_load_mmu_pgd(vcpu: *mut kvm_vcpu, root_hpa: hpa_t, root_level: c_int);
}
extern "C" {
    pub fn vmx_set_cr4(vcpu: *mut kvm_vcpu, cr4: c_ulong);
}
extern "C" {
    pub fn vmx_is_valid_cr4(vcpu: *mut kvm_vcpu, cr4: c_ulong) -> bool;
}
extern "C" {
    pub fn vmx_set_efer(vcpu: *mut kvm_vcpu, efer: u64) -> c_int;
}
extern "C" {
    pub fn vmx_get_idt(vcpu: *mut kvm_vcpu, dt: *mut desc_ptr);
}
extern "C" {
    pub fn vmx_set_idt(vcpu: *mut kvm_vcpu, dt: *mut desc_ptr);
}
extern "C" {
    pub fn vmx_get_gdt(vcpu: *mut kvm_vcpu, dt: *mut desc_ptr);
}
extern "C" {
    pub fn vmx_set_gdt(vcpu: *mut kvm_vcpu, dt: *mut desc_ptr);
}
extern "C" {
    pub fn vmx_set_dr7(vcpu: *mut kvm_vcpu, val: c_ulong);
}
extern "C" {
    pub fn vmx_sync_dirty_debug_regs(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_cache_reg(vcpu: *mut kvm_vcpu, reg: kvm_reg);
}
extern "C" {
    pub fn vmx_get_rflags(vcpu: *mut kvm_vcpu) -> c_ulong;
}
extern "C" {
    pub fn vmx_set_rflags(vcpu: *mut kvm_vcpu, rflags: c_ulong);
}
extern "C" {
    pub fn vmx_get_if_flag(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn vmx_flush_tlb_all(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_flush_tlb_current(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_flush_tlb_gva(vcpu: *mut kvm_vcpu, addr: gva_t, full: *mut bool);
}
extern "C" {
    pub fn vmx_flush_tlb_guest(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_set_interrupt_shadow(vcpu: *mut kvm_vcpu, mask: c_int);
}
extern "C" {
    pub fn vmx_get_interrupt_shadow(vcpu: *mut kvm_vcpu) -> u32;
}
extern "C" {
    pub fn vmx_patch_hypercall(vcpu: *mut kvm_vcpu, hypercall: *mut c_uchar);
}
extern "C" {
    pub fn vmx_inject_irq(vcpu: *mut kvm_vcpu, reinjected: bool);
}
extern "C" {
    pub fn vmx_inject_nmi(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_inject_exception(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_cancel_injection(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_interrupt_allowed(vcpu: *mut kvm_vcpu, for_injection: bool) -> c_int;
}
extern "C" {
    pub fn vmx_nmi_allowed(vcpu: *mut kvm_vcpu, for_injection: bool) -> c_int;
}
extern "C" {
    pub fn vmx_get_nmi_mask(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn vmx_set_nmi_mask(vcpu: *mut kvm_vcpu, masked: bool);
}
extern "C" {
    pub fn vmx_enable_nmi_window(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_enable_irq_window(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_update_cr8_intercept(vcpu: *mut kvm_vcpu, tpr: c_int, irr: c_int);
}
extern "C" {
    pub fn vmx_set_apic_access_page_addr(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_refresh_apicv_exec_ctrl(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_load_eoi_exitmap(vcpu: *mut kvm_vcpu, eoi_exit_bitmap: *mut u64);
}
extern "C" {
    pub fn vmx_set_tss_addr(kvm: *mut kvm, addr: c_uint) -> c_int;
}
extern "C" {
    pub fn vmx_set_identity_map_addr(kvm: *mut kvm, ident_addr: u64) -> c_int;
}
extern "C" {
    pub fn vmx_get_mt_mask(vcpu: *mut kvm_vcpu, gfn: gfn_t, is_mmio: bool) -> u8;
}
extern "C" {
    pub fn vmx_get_entry_info(vcpu: *mut kvm_vcpu, intr_info: *mut u32, error_code: *mut u32);
}
extern "C" {
    pub fn vmx_get_l2_tsc_offset(vcpu: *mut kvm_vcpu) -> u64;
}
extern "C" {
    pub fn vmx_get_l2_tsc_multiplier(vcpu: *mut kvm_vcpu) -> u64;
}
extern "C" {
    pub fn vmx_write_tsc_offset(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_write_tsc_multiplier(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_update_cpu_dirty_logging(vcpu: *mut kvm_vcpu);
}

extern "C" {
    pub fn vmx_cancel_hv_timer(vcpu: *mut kvm_vcpu);
}

extern "C" {
    pub fn vmx_setup_mce(vcpu: *mut kvm_vcpu);
}

extern "C" {
    pub fn tdx_disable_virtualization_cpu();
}
extern "C" {
    pub fn tdx_vm_init(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn tdx_mmu_release_hkid(kvm: *mut kvm);
}
extern "C" {
    pub fn tdx_vm_destroy(kvm: *mut kvm);
}
extern "C" {
    pub fn tdx_vm_ioctl(kvm: *mut kvm, argp: *mut void __user) -> c_int;
}
extern "C" {
    pub fn tdx_vcpu_create(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn tdx_vcpu_reset(vcpu: *mut kvm_vcpu, init_event: bool);
}
extern "C" {
    pub fn tdx_vcpu_free(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn tdx_vcpu_load(vcpu: *mut kvm_vcpu, cpu: c_int);
}
extern "C" {
    pub fn tdx_vcpu_needs_initialization(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn tdx_vcpu_run(vcpu: *mut kvm_vcpu, run_flags: u64) -> fastpath_t;
}
extern "C" {
    pub fn tdx_prepare_switch_to_guest(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn tdx_vcpu_put(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn tdx_inject_nmi(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn tdx_has_emulated_msr(index: u32) -> bool;
}
extern "C" {
    pub fn tdx_get_msr(vcpu: *mut kvm_vcpu, msr: *mut msr_data) -> c_int;
}
extern "C" {
    pub fn tdx_set_msr(vcpu: *mut kvm_vcpu, msr: *mut msr_data) -> c_int;
}
extern "C" {
    pub fn tdx_vcpu_ioctl(vcpu: *mut kvm_vcpu, argp: *mut void __user) -> c_int;
}
extern "C" {
    pub fn tdx_vcpu_unlocked_ioctl(vcpu: *mut kvm_vcpu, argp: *mut void __user) -> c_int;
}
extern "C" {
    pub fn tdx_flush_tlb_current(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn tdx_flush_tlb_all(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn tdx_load_mmu_pgd(vcpu: *mut kvm_vcpu, root_hpa: hpa_t, root_level: c_int);
}
extern "C" {
    pub fn tdx_gmem_max_mapping_level(kvm: *mut kvm, pfn: kvm_pfn_t, is_private: bool) -> c_int;
}

