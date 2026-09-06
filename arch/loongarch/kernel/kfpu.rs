//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kernel/kfpu.c
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
// Copyright (C) 2023 Loongson Technology Corporation Limited
//

    let mut euen_mask: static unsigned int = CSR_EUEN_FPEN;
//
// The critical section between kernel_fpu_begin() and kernel_fpu_end()
// is non-reentrant. It is the caller's responsibility to avoid reentrance.
// See drivers/gpu/drm/amd/display/amdgpu_dm/dc_fpu.c as an example.
//
    static DEFINE_PER_CPU(bool, in_kernel_fpu);
    static DEFINE_PER_CPU(unsigned int, euen_current);
#[no_mangle]
pub unsafe extern "C" fn fpregs_lock() {
    static inline void fpregs_lock(void)
    {
    if (IS_ENABLED(CONFIG_PREEMPT_RT))
    preempt_disable();
    else
    local_bh_disable();
    }
#[no_mangle]
pub unsafe extern "C" fn fpregs_unlock() {
    static inline void fpregs_unlock(void)
    {
    if (IS_ENABLED(CONFIG_PREEMPT_RT))
    preempt_enable();
    else
    local_bh_enable();
    }
#[no_mangle]
pub unsafe extern "C" fn kernel_fpu_begin() {
    void kernel_fpu_begin(void)
    {
    unsigned int *euen_curr;
    if (!irqs_disabled())
    fpregs_lock();
    WARN_ON(this_cpu_read(in_kernel_fpu));
    this_cpu_write(in_kernel_fpu, true);
    euen_curr = this_cpu_ptr(&euen_current);
// euen_curr = csr_xchg32(euen_mask, euen_mask, LOONGARCH_CSR_EUEN);

    if (*euen_curr & CSR_EUEN_LASXEN)
    _save_lasx(&current.thread.fpu);
    else

    if (*euen_curr & CSR_EUEN_LSXEN)
    _save_lsx(&current.thread.fpu);
    else

    if (*euen_curr & CSR_EUEN_FPEN)
    _save_fp(&current.thread.fpu);
    write_fcsr(LOONGARCH_FCSR0, 0);
    }
    EXPORT_SYMBOL_GPL(kernel_fpu_begin);
#[no_mangle]
pub unsafe extern "C" fn kernel_fpu_end() {
    void kernel_fpu_end(void)
    {
    unsigned int *euen_curr;
    WARN_ON(!this_cpu_read(in_kernel_fpu));
    euen_curr = this_cpu_ptr(&euen_current);

    if (*euen_curr & CSR_EUEN_LASXEN)
    _restore_lasx(&current.thread.fpu);
    else

    if (*euen_curr & CSR_EUEN_LSXEN)
    _restore_lsx(&current.thread.fpu);
    else

    if (*euen_curr & CSR_EUEN_FPEN)
    _restore_fp(&current.thread.fpu);
// euen_curr = csr_xchg32(*euen_curr, euen_mask, LOONGARCH_CSR_EUEN);
    this_cpu_write(in_kernel_fpu, false);
    if (!irqs_disabled())
    fpregs_unlock();
    }
    EXPORT_SYMBOL_GPL(kernel_fpu_end);
#[no_mangle]
unsafe extern "C" fn init_euen_mask() -> int __init {
    static int __init init_euen_mask(void)
    {
    if (cpu_has_lsx)
    euen_mask |= CSR_EUEN_LSXEN;
    if (cpu_has_lasx)
    euen_mask |= CSR_EUEN_LASXEN;
    return 0;
    }
    arch_initcall(init_euen_mask);
