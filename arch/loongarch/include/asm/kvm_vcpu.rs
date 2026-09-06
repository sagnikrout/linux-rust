//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/kvm_vcpu.h
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
// Copyright (C) 2020-2023 Loongson Technology Corporation Limited
//

// Controlled by 0x5 guest estat

// Controlled by 0x52 guest exception VIP aligned to estat bit 5~12

// KVM_IRQ_LINE irq field index values
pub const KVM_LOONGSON_IRQ_TYPE_SHIFT: c_int = 24;
pub const KVM_LOONGSON_IRQ_TYPE_MASK: c_uint = 0xff;
pub const KVM_LOONGSON_IRQ_VCPU_SHIFT: c_int = 16;
pub const KVM_LOONGSON_IRQ_VCPU_MASK: c_uint = 0xff;
pub const KVM_LOONGSON_IRQ_NUM_SHIFT: c_int = 0;
pub const KVM_LOONGSON_IRQ_NUM_MASK: c_uint = 0xffff;
pub type larch_inst = loongarch_instruction;
extern "C" {
    pub fn int(: *mut *mut exit_handle_fn)(struct kvm_vcpu, _arg: c_int) -> typedef;
}
extern "C" {
    pub fn kvm_emu_mmio_read(vcpu: *mut kvm_vcpu, inst: larch_inst) -> c_int;
}
extern "C" {
    pub fn kvm_emu_mmio_write(vcpu: *mut kvm_vcpu, inst: larch_inst) -> c_int;
}
extern "C" {
    pub fn kvm_complete_mmio_read(vcpu: *mut kvm_vcpu, run: *mut kvm_run) -> c_int;
}
extern "C" {
    pub fn kvm_complete_iocsr_read(vcpu: *mut kvm_vcpu, run: *mut kvm_run) -> c_int;
}
extern "C" {
    pub fn kvm_complete_user_service(vcpu: *mut kvm_vcpu, run: *mut kvm_run) -> c_int;
}
extern "C" {
    pub fn kvm_emu_idle(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_pending_timer(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_handle_fault(vcpu: *mut kvm_vcpu, fault: c_int) -> c_int;
}
extern "C" {
    pub fn kvm_deliver_intr(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_deliver_exception(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_own_fpu(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_lose_fpu(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_save_fpu(fpu: *mut loongarch_fpu);
}
extern "C" {
    pub fn kvm_restore_fpu(fpu: *mut loongarch_fpu);
}
extern "C" {
    pub fn kvm_restore_fcsr(fpu: *mut loongarch_fpu);
}

extern "C" {
    pub fn kvm_own_lsx(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_save_lsx(fpu: *mut loongarch_fpu);
}
extern "C" {
    pub fn kvm_restore_lsx(fpu: *mut loongarch_fpu);
}

extern "C" {
    pub fn kvm_own_lasx(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_save_lasx(fpu: *mut loongarch_fpu);
}
extern "C" {
    pub fn kvm_restore_lasx(fpu: *mut loongarch_fpu);
}

extern "C" {
    pub fn kvm_own_lbt(vcpu: *mut kvm_vcpu) -> c_int;
}

extern "C" {
    pub fn kvm_init_timer(vcpu: *mut kvm_vcpu, hz: c_ulong);
}
extern "C" {
    pub fn kvm_save_timer(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_restore_timer(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_vcpu_ioctl_interrupt(vcpu: *mut kvm_vcpu, irq: *mut kvm_interrupt) -> c_int;
}
//
// Loongarch KVM guest interrupt handling
//
// only one exception can be injected
