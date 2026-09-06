//! Automatically rewritten from C to Rust
//! Source: arch/x86/kvm/kvm-asm-offsets.c
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
// Generate definitions needed by assembly language modules.
// This code generates raw asm output which is post-processed to extract
// and format the required data.
//
// Macro flag: #define COMPILE_OFFSETS

#[no_mangle]
unsafe extern "C" fn common() -> void __used {
    static void __used common(void)
    {
    if (IS_ENABLED(CONFIG_KVM_AMD)) {
    BLANK();
    OFFSET(SVM_vcpu_arch_regs, vcpu_svm, vcpu.arch.regs);
    OFFSET(SVM_current_vmcb, vcpu_svm, current_vmcb);
    OFFSET(SVM_spec_ctrl, vcpu_svm, spec_ctrl);
    OFFSET(SVM_vmcb01, vcpu_svm, vmcb01);
    OFFSET(KVM_VMCB_pa, kvm_vmcb_info, pa);
    OFFSET(SD_save_area_pa, svm_cpu_data, save_area_pa);
    }
    if (IS_ENABLED(CONFIG_KVM_INTEL)) {
    BLANK();
    OFFSET(VMX_vcpu_arch_regs, vcpu_vmx, vcpu.arch.regs);
    OFFSET(VMX_spec_ctrl, vcpu_vmx, spec_ctrl);
    }
    }
