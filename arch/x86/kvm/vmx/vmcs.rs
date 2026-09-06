//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/vmx/vmcs.h
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
// Indexing into the vmcs12 uses the VMCS encoding rotated left by 6 as a very
// rudimentary compression of the range of indices.  The compression ratio is
// good enough to allow KVM to use a (very sparsely populated) array without
// wasting too much memory, while the "algorithm" is fast enough to be used to
// lookup vmcs12 fields on-demand, e.g. for emulation.
//

//
// vmcs_host_state tracks registers that are loaded from the VMCS on VMEXIT
// and whose values change infrequently, but are not constant.  I.e. this is
// used as a write-through cache of the corresponding VMCS fields.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmcs_host_state {
    pub /: *mut *mut unsigned long cr3; / May not match real cr3,
    pub /: *mut *mut unsigned long cr4; / May not match real cr4,
    pub gs_base: c_ulong,
    pub fs_base: c_ulong,
    pub rsp: c_ulong,
    pub ldt_sel: u16 fs_sel, gs_sel,,

    pub es_sel: u16 ds_sel,,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmcs_controls_shadow {
    pub vm_entry: u32,
    pub vm_exit: u32,
    pub pin: u32,
    pub exec: u32,
    pub secondary_exec: u32,
    pub tertiary_exec: u64,
}

//
// Track a VMCS that may be loaded on a certain CPU. If it is (cpu!=-1), also
// remember whether it was VMLAUNCHed, and maintain a linked list of all VMCSs
// loaded on this CPU (so we can clear them if the CPU goes down).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loaded_vmcs {
    pub vmcs: *mut vmcs,
    pub shadow_vmcs: *mut vmcs,
    pub cpu: c_int,
    pub launched: bool,
    pub nmi_known_unmasked: bool,
    pub hv_timer_soft_disabled: bool,
// Support for vnmi-less CPUs
    pub soft_vnmi_blocked: c_int,
    pub entry_time: ktime_t,
    pub vnmi_blocked_time: i64,
    pub msr_bitmap: *mut c_ulong,
    pub loaded_vmcss_on_cpu_link: list_head,
    pub host_state: vmcs_host_state,
    pub controls_shadow: vmcs_controls_shadow,
}

extern "C" {
    pub fn is_intr_type_n(_arg: intr_info, _arg: INTR_TYPE_HARD_EXCEPTION, _arg: vector) -> return;
}
extern "C" {
    pub fn is_exception_n(_arg: intr_info, _arg: DB_VECTOR) -> return;
}
extern "C" {
    pub fn is_exception_n(_arg: intr_info, _arg: BP_VECTOR) -> return;
}
extern "C" {
    pub fn is_exception_n(_arg: intr_info, _arg: DF_VECTOR) -> return;
}
extern "C" {
    pub fn is_exception_n(_arg: intr_info, _arg: PF_VECTOR) -> return;
}
extern "C" {
    pub fn is_exception_n(_arg: intr_info, _arg: UD_VECTOR) -> return;
}
extern "C" {
    pub fn is_exception_n(_arg: intr_info, _arg: GP_VECTOR) -> return;
}
extern "C" {
    pub fn is_exception_n(_arg: intr_info, _arg: AC_VECTOR) -> return;
}
extern "C" {
    pub fn is_exception_n(_arg: intr_info, _arg: MC_VECTOR) -> return;
}
extern "C" {
    pub fn is_exception_n(_arg: intr_info, _arg: NM_VECTOR) -> return;
}
extern "C" {
    pub fn is_exception_n(_arg: intr_info, _arg: VE_VECTOR) -> return;
}
// Undocumented: icebp/int1
extern "C" {
    pub fn is_intr_type(_arg: intr_info, _arg: INTR_TYPE_PRIV_SW_EXCEPTION) -> return;
}
extern "C" {
    pub fn is_intr_type(_arg: intr_info, _arg: INTR_TYPE_NMI_INTR) -> return;
}
extern "C" {
    pub fn is_intr_type(_arg: intr_info, _arg: INTR_TYPE_EXT_INTR) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmcs_field_width {
    VMCS_FIELD_WIDTH_U16 = 0,
    VMCS_FIELD_WIDTH_U64 = 1,
    VMCS_FIELD_WIDTH_U32 = 2,
    VMCS_FIELD_WIDTH_NATURAL_WIDTH = 3
}

