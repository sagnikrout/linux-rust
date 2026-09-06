//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/xen/xen-ops.h
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

// These are code, but not functions.  Defined in entry.S
extern "C" {
    pub fn xen_copy_trap_info(traps: *mut trap_info);
}
extern "C" {
    pub fn xen_entry_SYSENTER_compat();
}
extern "C" {
    pub fn xen_entry_SYSCALL_64();
}
extern "C" {
    pub fn xen_entry_SYSCALL_compat();
}
extern "C" {
    pub fn xen_setup_mfn_list_list();
}
extern "C" {
    pub fn xen_build_mfn_list_list();
}
extern "C" {
    pub fn xen_setup_machphys_mapping();
}
extern "C" {
    pub fn xen_setup_kernel_pagetable(pgd: *mut pgd_t, max_pfn: c_ulong);
}
extern "C" {
    pub fn xen_reserve_special_pages() -> void __init;
}
extern "C" {
    pub fn xen_pt_check_e820() -> void __init;
}
extern "C" {
    pub fn xen_mm_pin_all();
}
extern "C" {
    pub fn xen_mm_unpin_all();
}
extern "C" {
    pub fn xen_relocate_p2m() -> void __init;
}
extern "C" {
    pub fn xen_do_remap_nonram() -> void __init;
}
extern "C" {
    pub fn xen_chk_extra_mem(pfn: c_ulong) -> unsigned long __ref;
}
extern "C" {
    pub fn xen_inv_extra_mem() -> void __init;
}
extern "C" {
    pub fn xen_remap_memory() -> void __init;
}
extern "C" {
    pub fn xen_find_free_area(size: phys_addr_t) -> phys_addr_t __init;
}
extern "C" {
    pub fn xen_memory_setup() -> *mut char  __init;
}
extern "C" {
    pub fn xen_arch_setup() -> void __init;
}
extern "C" {
    pub fn xen_enable_syscall();
}
extern "C" {
    pub fn xen_build_dynamic_phys_to_machine() -> void __init;
}
extern "C" {
    pub fn xen_vmalloc_p2m_tree() -> void __init;
}
extern "C" {
    pub fn xen_init_irq_ops();
}
extern "C" {
    pub fn xen_setup_vcpu_info_placement();
}
extern "C" {
    pub fn xen_init_apic() -> void __init;
}
extern "C" {
    pub fn xen_irq_enable_direct() -> __visible void;
}
extern "C" {
    pub fn xen_irq_disable_direct() -> __visible void;
}
extern "C" {
    pub fn xen_save_fl_direct() -> __visible unsigned long;
}
extern "C" {
    pub fn xen_read_cr2() -> __visible unsigned long;
}
extern "C" {
    pub fn xen_read_cr2_direct() -> __visible unsigned long;
}
// These are not functions, and cannot be called normally
extern "C" {
    pub fn xen_iret() -> __visible void;
}
extern "C" {
    pub fn xen_force_evtchn_callback();
}
extern "C" {
    pub fn xen_pv_pre_suspend();
}
extern "C" {
    pub fn xen_pv_post_suspend(suspend_cancelled: c_int);
}
extern "C" {
    pub fn xen_start_kernel(si: *mut start_info);
}
extern "C" {
    pub fn set_pte_mfn(vaddr: c_ulong, pfn: c_ulong, flags: pgprot_t);
}
extern "C" {
    pub fn xen_init_mmu_ops();
}
// Multicalls
// Allocate room for a multicall and its args
extern "C" {
    pub fn __xen_mc_entry(args: usize) -> multicall_space;
}
// Call to start a batch of multiple __xen_mc_entry()s.  Must be
// need to disable interrupts until this entry is complete
extern "C" {
    pub fn __xen_mc_entry(_arg: args) -> return;
}
// Flush all pending multicalls
extern "C" {
    pub fn xen_mc_flush();
}
// Issue a multicall if we're not in a lazy mode
// restore flags saved in xen_mc_batch
// Set up a callback to be called when the current batch is flushed
extern "C" {
    pub fn xen_mc_callback(): *mut *mut void (fn)(void, data: *mut c_void);
}
//
// Try to extend the arguments of the previous multicall command.  The
// previous command's op must match.  If it does, then it attempts to
// extend the argument space allocated to the multicall entry by
// arg_size bytes.
//
// The returned multicall_space will return with mc pointing to the
// command on success, or NULL on failure, and args pointing to the
// newly allocated space.
//
extern "C" {
    pub fn xen_mc_extend_args(op: c_ulong, arg_size: usize) -> multicall_space;
}
extern "C" {
    pub fn xen_pmu_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn pmu_msr_chk_emulated(msr: u32, val: *mut u64, is_read: bool) -> bool;
}
extern "C" {
    pub fn pmu_apic_update(reg: u32) -> c_int;
}
extern "C" {
    pub fn xen_read_pmc(counter: c_int) -> u64;
}
extern "C" {
    pub fn xen_hypercall_pv();
}

extern "C" {
    pub fn xen_banner();
}
extern "C" {
    pub fn xen_vcpu_restore();
}
extern "C" {
    pub fn xen_hvm_init_shared_info();
}
extern "C" {
    pub fn xen_unplug_emulated_devices();
}
extern "C" {
    pub fn xen_setup_timer(cpu: c_int);
}
extern "C" {
    pub fn xen_setup_runstate_info(cpu: c_int);
}
extern "C" {
    pub fn xen_teardown_timer(cpu: c_int);
}
extern "C" {
    pub fn xen_setup_cpu_clockevents();
}
extern "C" {
    pub fn xen_save_time_memory_area();
}
extern "C" {
    pub fn xen_restore_time_memory_area();
}
extern "C" {
    pub fn xen_init_time_ops();
}
extern "C" {
    pub fn xen_hvm_init_time_ops();
}
extern "C" {
    pub fn xen_vcpu_stolen(vcpu: c_int) -> bool;
}
extern "C" {
    pub fn xen_vcpu_setup(cpu: c_int);
}
extern "C" {
    pub fn xen_vcpu_info_reset(cpu: c_int);
}

extern "C" {
    pub fn xen_smp_init();
}
extern "C" {
    pub fn xen_hvm_smp_init() -> void __init;
}

extern "C" {
    pub fn xen_init_spinlocks() -> void __init;
}
extern "C" {
    pub fn xen_init_lock_cpu(cpu: c_int);
}
extern "C" {
    pub fn xen_uninit_lock_cpu(cpu: c_int);
}

extern "C" {
    pub fn xen_add_preferred_consoles();
}

extern "C" {
    pub fn xen_efi_init(boot_params: *mut boot_params);
}

extern "C" {
    pub fn xen_panic_handler_init() -> c_int;
}
extern "C" {
    pub fn xen_pin_vcpu(cpu: c_int);
}
extern "C" {
    pub fn xen_emergency_restart();
}
extern "C" {
    pub fn xen_hvm_post_suspend(suspend_cancelled: c_int);
}
//
// The maximum amount of extra memory compared to the base size.  The
// main scaling factor is the size of struct page.  At extreme ratios
// of base:extra, all the base memory can be filled with page
// structures for the extra memory, leaving no space for anything
// else.
//
// 10x seems like a reasonable balance between scaling flexibility and
// leaving a practically usable system.
//

extern "C" {
    pub fn xen_add_extra_mem(start_pfn: c_ulong, n_pfns: c_ulong);
}
extern "C" {
    pub fn __set_phys_to_machine(pfn: c_ulong, mfn: c_ulong) -> bool;
}
extern "C" {
    pub fn xen_hvm_init_mmu_ops();
}

extern "C" {
    pub fn xen_pmu_init(cpu: c_int);
}
extern "C" {
    pub fn xen_pmu_finish(cpu: c_int);
}

extern "C" {
    pub fn asm_cpu_bringup_and_idle();
}
extern "C" {
    pub fn cpu_bringup_and_idle() -> asmlinkage void;
}
extern "C" {
    pub fn xen_send_IPI_allbutself(vector: c_int);
}
extern "C" {
    pub fn xen_send_IPI_all(vector: c_int);
}
extern "C" {
    pub fn xen_send_IPI_self(vector: c_int);
}
extern "C" {
    pub fn xen_smp_intr_init(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn xen_smp_intr_free(cpu: c_uint);
}
extern "C" {
    pub fn xen_smp_intr_init_pv(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn xen_smp_intr_free_pv(cpu: c_uint);
}
extern "C" {
    pub fn xen_smp_count_cpus();
}
extern "C" {
    pub fn xen_smp_cpus_done(max_cpus: c_uint);
}
extern "C" {
    pub fn xen_smp_send_reschedule(cpu: c_int);
}
extern "C" {
    pub fn xen_smp_send_call_function_ipi(mask: *const cpumask);
}
extern "C" {
    pub fn xen_smp_send_call_function_single_ipi(cpu: c_int);
}
extern "C" {
    pub fn xen_cpu_bringup_again(stack: c_ulong) -> void __noreturn;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_common_irq {
    pub irq: c_int,
    pub name: *mut c_char,
}

extern "C" {
    pub fn xen_hypercall_hvm();
}
extern "C" {
    pub fn xen_hypercall_amd();
}
extern "C" {
    pub fn xen_hypercall_intel();
}
extern "C" {
    pub fn xen_hypercall_setfunc();
}
