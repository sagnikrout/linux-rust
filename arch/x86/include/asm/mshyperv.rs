//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/mshyperv.h
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
// Hyper-V always provides a single IO-APIC at this MMIO address.
// Ideally, the value should be looked up in ACPI tables, but it
// is needed for mapping the IO-APIC early in boot on Confidential
// VMs, before ACPI functions can be used.
//
pub const HV_IOAPIC_BASE_ADDRESS: c_uint = 0xfec00000;
pub const HV_VTL_NORMAL: c_uint = 0x0;
pub const HV_VTL_SECURE: c_uint = 0x1;
pub const HV_VTL_MGMT: c_uint = 0x2;
extern "C" {
    pub fn hyperv_vector_handler(regs: *mut pt_regs);
}
extern "C" {
    pub fn hv_tdx_hypercall(control: u64, param1: u64, param2: u64) -> u64;
}
extern "C" {
    pub fn hv_snp_hypercall(control: u64, param1: u64, param2: u64) -> u64;
}
extern "C" {
    pub fn hv_std_hypercall(control: u64, param1: u64, param2: u64) -> u64;
}

extern "C" {
    pub fn hv_isolation_type_snp() -> bool;
}
extern "C" {
    pub fn hv_isolation_type_tdx() -> bool;
}

//
// DEFAULT INIT GPAT and SEGMENT LIMIT value in struct VMSA
// to start AP in enlightened SEV guest.
//
pub const HV_AP_INIT_GPAT_DEFAULT: c_uint = 0x0007040600070406ULL;
pub const HV_AP_SEGMENT_LIMIT: c_uint = 0xffffffff;
//
// If the hypercall involves no input or output parameters, the hypervisor
// ignores the corresponding GPA pointer.
//

extern "C" {
    pub fn static_call_mod(_arg: hv_hypercall)(control, _arg: input_address, _arg: output_address) -> return;
}

// Fast hypercall with 8 bytes of input and no output

extern "C" {
    pub fn static_call_mod(_arg: hv_hypercall)(control, _arg: input1, _arg: 0) -> return;
}

extern "C" {
    pub fn _hv_do_fast_hypercall8(_arg: control, _arg: input1) -> return;
}
// Fast hypercall with 16 bytes of input

extern "C" {
    pub fn static_call_mod(_arg: hv_hypercall)(control, _arg: input1, _arg: input2) -> return;
}

extern "C" {
    pub fn _hv_do_fast_hypercall16(_arg: control, _arg: input1, _arg: input2) -> return;
}
extern "C" {
    pub fn hyperv_init() -> void __init;
}
extern "C" {
    pub fn hyperv_setup_mmu_ops();
}
extern "C" {
    pub fn set_hv_tscchange_cb((*cb)(void): *mut c_void);
}
extern "C" {
    pub fn clear_hv_tscchange_cb();
}
extern "C" {
    pub fn hyperv_stop_tsc_emulation();
}
extern "C" {
    pub fn hyperv_flush_guest_mapping(as: u64) -> c_int;
}
extern "C" {
    pub fn hv_sleep_notifiers_register();
}
extern "C" {
    pub fn hv_machine_power_off();
}

extern "C" {
    pub fn hv_apic_init();
}
extern "C" {
    pub fn hv_init_spinlocks() -> void __init;
}
extern "C" {
    pub fn hv_vcpu_is_preempted(vcpu: c_int) -> bool;
}

extern "C" {
    pub fn hv_unmap_ioapic_interrupt(ioapic_id: c_int, entry: *mut hv_interrupt_entry) -> c_int;
}

extern "C" {
    pub fn hv_ghcb_negotiate_protocol() -> bool;
}
extern "C" {
    pub fn hv_ghcb_terminate(set: c_uint, reason: c_uint) -> void __noreturn;
}
extern "C" {
    pub fn hv_snp_boot_ap(apic_id: u32, start_ip: c_ulong, cpu: c_uint) -> c_int;
}

extern "C" {
    pub fn hv_vtom_init();
}
extern "C" {
    pub fn hv_ivm_msr_write(msr: u64, value: u64);
}
extern "C" {
    pub fn hv_ivm_msr_read(msr: u64, value: *mut u64);
}

extern "C" {
    pub fn hv_get_msr(reg: c_uint) -> u64;
}
extern "C" {
    pub fn hv_set_msr(reg: c_uint, value: u64);
}
extern "C" {
    pub fn hv_get_non_nested_msr(reg: c_uint) -> u64;
}
extern "C" {
    pub fn hv_set_non_nested_msr(reg: c_uint, value: u64);
}
extern "C" {
    pub fn native_rdmsrq(_arg: reg) -> return;
}
extern "C" {
    pub fn hv_apicid_to_vp_index(apic_id: u32) -> c_int;
}

extern "C" {
    pub fn hv_root_crash_init();
}
extern "C" {
    pub fn hv_crash_asm32();
}
extern "C" {
    pub fn hv_crash_asm64();
}
extern "C" {
    pub fn hv_crash_asm_end();
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_vtl_cpu_context {
    pub rax: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rbx: u64,
    pub cr2: u64,
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
}

extern "C" {
    pub fn hv_vtl_init_platform() -> void __init;
}
extern "C" {
    pub fn hv_vtl_early_init() -> int __init;
}
extern "C" {
    pub fn mshv_vtl_return_call(vtl0: *mut mshv_vtl_cpu_context);
}
extern "C" {
    pub fn mshv_vtl_return_call_init(vtl_return_offset: u64);
}
extern "C" {
    pub fn mshv_vtl_return_hypercall();
}
extern "C" {
    pub fn __mshv_vtl_return_call(vtl0: *mut mshv_vtl_cpu_context);
}

