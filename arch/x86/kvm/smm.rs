//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/smm.h
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
// 32 bit KVM's emulated SMM layout. Based on Intel P6 layout
// (https://www.sandpile.org/x86/smm.htm).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_smm_seg_state_32 {
    pub flags: u32,
    pub limit: u32,
    pub base: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_smram_state_32 {
    pub reserved1: [u32; 62],
    pub smbase: u32,
    pub smm_revision: u32,
    pub io_inst_restart: u16,
    pub auto_hlt_restart: u16,
    pub io_restart_rdi: u32,
    pub io_restart_rcx: u32,
    pub io_restart_rsi: u32,
    pub io_restart_rip: u32,
    pub cr4: u32,
// A20M#, CPL, shutdown and other reserved/undocumented fields
    pub reserved2: u16,
    pub /: *mut *mut u8 int_shadow; / KVM extension,
    pub reserved3: [u8; 17],
    pub ds: kvm_smm_seg_state_32,
    pub fs: kvm_smm_seg_state_32,
    pub gs: kvm_smm_seg_state_32,
    pub /: *mut *mut kvm_smm_seg_state_32 idtr; / IDTR has only base and limit,
    pub tr: kvm_smm_seg_state_32,
    pub reserved: u32,
    pub /: *mut *mut kvm_smm_seg_state_32 gdtr; / GDTR has only base and limit,
    pub ldtr: kvm_smm_seg_state_32,
    pub es: kvm_smm_seg_state_32,
    pub cs: kvm_smm_seg_state_32,
    pub ss: kvm_smm_seg_state_32,
    pub es_sel: u32,
    pub cs_sel: u32,
    pub ss_sel: u32,
    pub ds_sel: u32,
    pub fs_sel: u32,
    pub gs_sel: u32,
    pub ldtr_sel: u32,
    pub tr_sel: u32,
    pub dr7: u32,
    pub dr6: u32,
    pub /: *mut *mut u32 gprs[8]; / GPRS in the "natural" X86 order (EAX/ECX/EDX.../EDI),
    pub eip: u32,
    pub eflags: u32,
    pub cr3: u32,
    pub cr0: u32,
    pub __packed: },
// 64 bit KVM's emulated SMM layout. Based on AMD64 layout
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_smm_seg_state_64 {
    pub selector: u16,
    pub attributes: u16,
    pub limit: u32,
    pub base: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_smram_state_64 {
    pub es: kvm_smm_seg_state_64,
    pub cs: kvm_smm_seg_state_64,
    pub ss: kvm_smm_seg_state_64,
    pub ds: kvm_smm_seg_state_64,
    pub fs: kvm_smm_seg_state_64,
    pub gs: kvm_smm_seg_state_64,
    pub limit*/: *mut *mut kvm_smm_seg_state_64 gdtr; / GDTR has only base and,
    pub ldtr: kvm_smm_seg_state_64,
    pub limit*/: *mut *mut kvm_smm_seg_state_64 idtr; / IDTR has only base and,
    pub tr: kvm_smm_seg_state_64,
// I/O restart and auto halt restart are not implemented by KVM
    pub io_restart_rip: u64,
    pub io_restart_rcx: u64,
    pub io_restart_rsi: u64,
    pub io_restart_rdi: u64,
    pub io_restart_dword: u32,
    pub reserved1: u32,
    pub io_inst_restart: u8,
    pub auto_hlt_restart: u8,
    pub /: *mut *mut u8 amd_nmi_mask; / Documented in AMD BKDG as NMI mask, not used by KVM,
    pub int_shadow: u8,
    pub reserved2: u32,
    pub efer: u64,
//
// Two fields below are implemented on AMD only, to store
// SVM guest vmcb address if the #SMI was received while in the guest mode.
//
    pub svm_guest_flag: u64,
    pub svm_guest_vmcb_gpa: u64,
    pub /: *mut *mut u64 svm_guest_virtual_int; / unknown purpose, not implemented,
    pub reserved3: [u32; 3],
    pub smm_revison: u32,
    pub smbase: u32,
    pub reserved4: [u32; 5],
    pub ssp: u64,
// svm_* fields below are not implemented by KVM
    pub svm_guest_pat: u64,
    pub svm_host_efer: u64,
    pub svm_host_cr4: u64,
    pub svm_host_cr3: u64,
    pub svm_host_cr0: u64,
    pub cr4: u64,
    pub cr3: u64,
    pub cr0: u64,
    pub dr7: u64,
    pub dr6: u64,
    pub rflags: u64,
    pub rip: u64,
    pub /: *mut *mut u64 gprs[16]; / GPRS in a reversed "natural" X86 order (R15/R14/../RCX/RAX.),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_smram {
    pub smram64: kvm_smram_state_64,
    pub smram32: kvm_smram_state_32,
    pub bytes: [u8; 512],
}

extern "C" {
    pub fn kvm_smm_changed(vcpu: *mut kvm_vcpu, in_smm: bool);
}
extern "C" {
    pub fn enter_smm(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn emulator_leave_smm(ctxt: *mut x86_emulate_ctxt) -> c_int;
}
extern "C" {
    pub fn process_smi(vcpu: *mut kvm_vcpu);
}

//
// emulator_leave_smm is used as a function pointer, so the
// stub is defined in x86.c.
//

