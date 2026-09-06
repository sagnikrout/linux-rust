//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/xen/interface.h
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


//
// arch-x86_32.h
//
// Guest OS interface to x86 Xen.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//
// Copyright (c) 2004-2006, K A Fraser
//
// XEN_GUEST_HANDLE represents a guest pointer, when passed as a field
// in a struct in memory.
// XEN_GUEST_HANDLE_PARAM represent a guest pointer, when passed as an
// hypercall argument.
// XEN_GUEST_HANDLE_PARAM and XEN_GUEST_HANDLE are the same on X86 but
// they might not be on other architectures.
//

// (uint64_t *)&(hnd) = 0;	\

// (uint64_t *)&(hnd) = 0;	\

// Explicitly size integers that represent pfns in the public interface
// with Xen so that on ARM we can have one ABI that works for 32 and 64
// bit guests.
pub type xen_pfn_t = c_ulong;

pub type xen_ulong_t = c_ulong;

pub type xen_long_t = c_long;

// Guest handles for primitive C types.

// Maximum number of virtual CPUs in multi-processor guests.
pub const MAX_VIRT_CPUS: c_int = 32;
//
// SEGMENT DESCRIPTOR TABLES
//
// A number of GDT entries are reserved by Xen. These are not situated at the
// start of the GDT because some stupid OSes export hard-coded selector values
// in their ABI. These hard-coded values are always near the start of the GDT,
// so Xen places itself out of the way, at the far end of the GDT.
//
// NB The LDT is set using the MMUEXT_SET_LDT op of HYPERVISOR_mmuext_op
//
pub const FIRST_RESERVED_GDT_PAGE: c_int = 14;

//
// Send an array of these to HYPERVISOR_set_trap_table().
// Terminate the array with a sentinel entry, with traps[].address==0.
// The privilege level specifies which modes may enter a trap via a software
// interrupt. On x86/64, since rings 1 and 2 are unavailable, we allocate
// privilege levels as follows:
// Level == 0: No one may enter
// Level == 1: Kernel may enter
// Level == 2: Kernel may enter
// Level == 3: Everyone may enter
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trap_info {
    pub /: *mut *mut uint8_t vector; / exception vector,
    pub /: *mut *mut uint8_t flags; / 0-3: privilege level; 4: clear event enable?,
    pub /: *mut *mut uint16_t cs; / code selector,
    pub /: *mut *mut unsigned long address; / code offset,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_shared_info {
//
// Number of valid entries in the p2m table(s) anchored at
// pfn_to_mfn_frame_list_list and/or p2m_vaddr.
//
    pub max_pfn: c_ulong,
//
// Frame containing list of mfns containing list of mfns containing p2m.
// A value of 0 indicates it has not yet been set up, ~0 indicates it
// has been set to invalid e.g. due to the p2m being too large for the
// 3-level p2m tree. In this case the linear mapper p2m list anchored
// at p2m_vaddr is to be used.
//
    pub pfn_to_mfn_frame_list_list: xen_pfn_t,
    pub nmi_reason: c_ulong,
//
// Following three fields are valid if p2m_cr3 contains a value
// different from 0.
// p2m_cr3 is the root of the address space where p2m_vaddr is valid.
// p2m_cr3 is in the same format as a cr3 value in the vcpu register
// state and holds the folded machine frame number (via xen_pfn_to_cr3)
// of a L3 or L4 page table.
// p2m_vaddr holds the virtual address of the linear p2m list. All
// entries in the range [0...max_pfn[ are accessible via this pointer.
// p2m_generation will be incremented by the guest before and after each
// change of the mappings of the p2m list. p2m_generation starts at 0
// and a value with the least significant bit set indicates that a
// mapping update is in progress. This allows guest external software
// (e.g. in Dom0) to verify that read mappings are consistent and
// whether they have changed since the last check.
// Modifying a p2m element in the linear p2m list is allowed via an
// atomic write only.
//
    pub /: *mut *mut unsigned long p2m_cr3; / cr3 value of the p2m address space,
    pub /: *mut *mut unsigned long p2m_vaddr; / virtual address of the p2m list,
    pub /: *mut *mut unsigned long p2m_generation; / generation count of p2m mapping,

    pub wc_sec_hi: u32,

}

//
// The following is all CPU context. Note that the fpu_ctxt block is filled
// in by FXSAVE if the CPU has feature FXSR; otherwise FSAVE is used.
//
// Also note that when calling DOMCTL_setvcpucontext and VCPU_initialise
// for HVM and PVH guests, not all information in this structure is updated:
//
// - For HVM guests, the structures read include: fpu_ctxt (if
// VGCT_I387_VALID is set), flags, user_regs, debugreg[*]
//
// - PVH guests are the same as HVM guests, but additionally use ctrlreg[3] to
// set cr3. All other fields not used should be set to 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcpu_guest_context {
// FPU registers come first so they can be aligned for FXSAVE/FXRSTOR.
    pub /: *mut *mut { char x[512]; } fpu_ctxt; / User-level FPU registers,

pub const _VGCF_i387_valid: c_int = 0;

pub const _VGCF_in_kernel: c_int = 2;

pub const _VGCF_failsafe_disables_events: c_int = 3;

pub const _VGCF_syscall_disables_events: c_int = 4;

pub const _VGCF_online: c_int = 5;

    pub /: *mut *mut *mut unsigned long flags; / VGCF_ flags,
    pub /: *mut *mut cpu_user_regs user_regs; / User-level CPU registers,
    pub /: *mut *mut trap_info trap_ctxt[256]; / Virtual IDT,
    pub /: *mut *mut unsigned long ldt_base, ldt_ents; / LDT (linear address, # ents),
    pub /: *mut *mut unsigned long gdt_frames[16], gdt_ents; / GDT (machine frames, # ents),
    pub /: *mut *mut unsigned long kernel_ss, kernel_sp; / Virtual TSS (only SS1/SP1),
// NB. User pagetable on x86/64 is placed in ctrlreg[1].
    pub /: *mut *mut unsigned long ctrlreg[8]; / CR0-CR7 (control registers),
    pub /: *mut *mut unsigned long debugreg[8]; / DB0-DB7 (debug registers),

    pub /: *mut *mut unsigned long event_callback_cs; / CS:EIP of event callback,
    pub event_callback_eip: c_ulong,
    pub /: *mut *mut unsigned long failsafe_callback_cs; / CS:EIP of failsafe callback,
    pub failsafe_callback_eip: c_ulong,

    pub event_callback_eip: c_ulong,
    pub failsafe_callback_eip: c_ulong,
    pub syscall_callback_eip: c_ulong,

    pub /: *mut *mut *mut unsigned long vm_assist; / VMASST_TYPE_ bitmap,

// Segment base addresses.
    pub fs_base: u64,
    pub gs_base_kernel: u64,
    pub gs_base_user: u64,

}

// AMD PMU registers and structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pmu_amd_ctxt {
//
// Offsets to counter and control MSRs (relative to xen_pmu_arch.c.amd).
// For PV(H) guests these fields are RO.
//
    pub counters: u32,
    pub ctrls: u32,
// Counter MSRs
    pub regs: [u64; ],    pub regs: [u64; 0],
}

// Intel PMU registers and structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pmu_cntr_pair {
    pub counter: u64,
    pub control: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pmu_intel_ctxt {
//
// Offsets to fixed and architectural counter MSRs (relative to
// xen_pmu_arch.c.intel).
// For PV(H) guests these fields are RO.
//
    pub fixed_counters: u32,
    pub arch_counters: u32,
// PMU registers
    pub global_ctrl: u64,
    pub global_ovf_ctrl: u64,
    pub global_status: u64,
    pub fixed_ctrl: u64,
    pub ds_area: u64,
    pub pebs_enable: u64,
    pub debugctl: u64,
// Fixed and architectural counter MSRs
    pub regs: [u64; ],    pub regs: [u64; 0],
}

// Sampled domain's registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pmu_regs {
    pub ip: u64,
    pub sp: u64,
    pub flags: u64,
    pub cs: u16,
    pub ss: u16,
    pub cpl: u8,
    pub pad: [u8; 3],
}

// PMU flags

//
// Architecture-specific information describing state of the processor at
// the time of PMU interrupt.
// Fields of this structure marked as RW for guest should only be written by
// the guest when PMU_CACHED bit in pmu_flags is set (which is done by the
// hypervisor during PMU interrupt). Hypervisor will read updated data in
// XENPMU_flush hypercall and clear PMU_CACHED bit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pmu_arch {
//
// Processor's registers at the time of interrupt.
// WO for hypervisor, RO for guests.
//
    pub regs: xen_pmu_regs,
//
// Padding for adding new registers to xen_pmu_regs in
// the future
//
pub const XENPMU_REGS_PAD_SZ: c_int = 64;
    pub pad: [u8; XENPMU_REGS_PAD_SZ],
    pub r: },
// WO for hypervisor, RO for guest
    pub pmu_flags: u64,
//
// APIC LVTPC register.
// RW for both hypervisor and guest.
// Only APIC_LVT_MASKED bit is loaded by the hypervisor into hardware
// during XENPMU_flush or XENPMU_lvtpc_set.
//
    pub lapic_lvtpc: u32,
    pub pad: u64,
    pub l: },
//
// Vendor-specific PMU registers.
// RW for both hypervisor and guest (see exceptions above).
// Guest's updates to this field are verified and then loaded by the
// hypervisor into hardware during XENPMU_flush
//
    pub amd: xen_pmu_amd_ctxt,
    pub intel: xen_pmu_intel_ctxt,
//
// Padding for contexts (fixed parts only, does not include
// MSR banks that are specified by offsets)
//
pub const XENPMU_CTXT_PAD_SZ: c_int = 128;
    pub pad: [u8; XENPMU_CTXT_PAD_SZ],
    pub c: },
}

//
// Prefix forces emulation of some non-trapping instructions.
// Currently only CPUID.
//

