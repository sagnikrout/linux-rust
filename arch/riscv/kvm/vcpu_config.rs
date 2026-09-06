//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kvm/vcpu_config.c
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
// Copyright (c) 2026 Qualcomm Technologies, Inc.
//

    BIT(EXC_INST_ILLEGAL)     | \
    BIT(EXC_BREAKPOINT)      | \
    BIT(EXC_SYSCALL)         | \
    BIT(EXC_INST_PAGE_FAULT) | \
    BIT(EXC_LOAD_PAGE_FAULT) | \
    BIT(EXC_STORE_PAGE_FAULT))

    BIT(IRQ_VS_TIMER) | \
    BIT(IRQ_VS_EXT))
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_vcpu_config_init(vcpu: *mut kvm_vcpu) {
    void kvm_riscv_vcpu_config_init(struct kvm_vcpu *vcpu)
    {
    vcpu.arch.cfg.hedeleg = KVM_HEDELEG_DEFAULT;
    vcpu.arch.cfg.hideleg = KVM_HIDELEG_DEFAULT;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_vcpu_config_guest_debug(vcpu: *mut kvm_vcpu) {
    void kvm_riscv_vcpu_config_guest_debug(struct kvm_vcpu *vcpu)
    {
    struct kvm_vcpu_config *cfg = &vcpu.arch.cfg;
    if (vcpu.guest_debug)
    cfg.hedeleg &= ~BIT(EXC_BREAKPOINT);
    else
    cfg.hedeleg |= BIT(EXC_BREAKPOINT);
    vcpu.arch.csr_dirty = true;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_vcpu_config_ran_once(vcpu: *mut kvm_vcpu) {
    void kvm_riscv_vcpu_config_ran_once(struct kvm_vcpu *vcpu)
    {
    const unsigned long *isa = vcpu.arch.isa;
    struct kvm_vcpu_config *cfg = &vcpu.arch.cfg;
    if (riscv_isa_extension_available(isa, SVPBMT))
    cfg.henvcfg |= ENVCFG_PBMTE;
    if (riscv_isa_extension_available(isa, SSTC))
    cfg.henvcfg |= ENVCFG_STCE;
    if (riscv_isa_extension_available(isa, ZICBOM))
    cfg.henvcfg |= (ENVCFG_CBIE | ENVCFG_CBCFE);
    if (riscv_isa_extension_available(isa, ZICBOZ))
    cfg.henvcfg |= ENVCFG_CBZE;
    if (riscv_isa_extension_available(isa, SVADU) &&
    !riscv_isa_extension_available(isa, SVADE))
    cfg.henvcfg |= ENVCFG_ADUE;
    if (riscv_has_extension_unlikely(RISCV_ISA_EXT_SMSTATEEN)) {
    cfg.hstateen0 |= SMSTATEEN0_HSENVCFG;
    if (riscv_isa_extension_available(isa, SSAIA))
    cfg.hstateen0 |= SMSTATEEN0_AIA_IMSIC |
    SMSTATEEN0_AIA |
    SMSTATEEN0_AIA_ISEL;
    if (riscv_isa_extension_available(isa, SMSTATEEN))
    cfg.hstateen0 |= SMSTATEEN0_SSTATEEN0;
    }
    if (vcpu.guest_debug)
    cfg.hedeleg &= ~BIT(EXC_BREAKPOINT);
    kvm_riscv_vcpu_sbi_validate(vcpu);
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_vcpu_config_load(vcpu: *mut kvm_vcpu) {
    void kvm_riscv_vcpu_config_load(struct kvm_vcpu *vcpu)
    {
    struct kvm_vcpu_config *cfg = &vcpu.arch.cfg;
    void *nsh;
    if (kvm_riscv_nacl_sync_csr_available()) {
    nsh = nacl_shmem();
    nacl_csr_write(nsh, CSR_HEDELEG, cfg.hedeleg);
    nacl_csr_write(nsh, CSR_HIDELEG, cfg.hideleg);
    nacl_csr_write(nsh, CSR_HENVCFG, cfg.henvcfg);
    if (IS_ENABLED(CONFIG_32BIT))
    nacl_csr_write(nsh, CSR_HENVCFGH, cfg.henvcfg >> 32);
    if (riscv_has_extension_unlikely(RISCV_ISA_EXT_SMSTATEEN)) {
    nacl_csr_write(nsh, CSR_HSTATEEN0, cfg.hstateen0);
    if (IS_ENABLED(CONFIG_32BIT))
    nacl_csr_write(nsh, CSR_HSTATEEN0H, cfg.hstateen0 >> 32);
    }
    } else {
    csr_write(CSR_HEDELEG, cfg.hedeleg);
    csr_write(CSR_HIDELEG, cfg.hideleg);
    csr_write(CSR_HENVCFG, cfg.henvcfg);
    if (IS_ENABLED(CONFIG_32BIT))
    csr_write(CSR_HENVCFGH, cfg.henvcfg >> 32);
    if (riscv_has_extension_unlikely(RISCV_ISA_EXT_SMSTATEEN)) {
    csr_write(CSR_HSTATEEN0, cfg.hstateen0);
    if (IS_ENABLED(CONFIG_32BIT))
    csr_write(CSR_HSTATEEN0H, cfg.hstateen0 >> 32);
    }
    }
    }
