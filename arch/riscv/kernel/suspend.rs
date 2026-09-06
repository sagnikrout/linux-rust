//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/suspend.c
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
// Copyright (c) 2021 Western Digital Corporation or its affiliates.
// Copyright (c) 2022 Ventana Micro Systems Inc.
//

#[no_mangle]
pub unsafe extern "C" fn suspend_save_csrs(context: *mut suspend_context) {
    void suspend_save_csrs(struct suspend_context *context)
    {
    if (riscv_has_extension_unlikely(RISCV_ISA_EXT_XLINUXENVCFG))
    context.envcfg = csr_read(CSR_ENVCFG);
    context.tvec = csr_read(CSR_TVEC);
    context.ie = csr_read(CSR_IE);
//
// No need to save/restore IP CSR (i.e. MIP or SIP) because:
//
// 1. For no-MMU (M-mode) kernel, the bits in MIP are set by
// external devices (such as interrupt controller, timer, etc).
// 2. For MMU (S-mode) kernel, the bits in SIP are set by
// M-mode firmware and external devices (such as interrupt
// controller, etc).
//

    if (riscv_has_extension_unlikely(RISCV_ISA_EXT_SSTC)) {
    context.stimecmp = csr_read(CSR_STIMECMP);

    context.stimecmph = csr_read(CSR_STIMECMPH);

    }
    context.satp = csr_read(CSR_SATP);

    }
#[no_mangle]
pub unsafe extern "C" fn suspend_restore_csrs(context: *mut suspend_context) {
    void suspend_restore_csrs(struct suspend_context *context)
    {
    csr_write(CSR_SCRATCH, 0);
    if (riscv_has_extension_unlikely(RISCV_ISA_EXT_XLINUXENVCFG))
    csr_write(CSR_ENVCFG, context.envcfg);
    csr_write(CSR_TVEC, context.tvec);
    csr_write(CSR_IE, context.ie);

    if (riscv_has_extension_unlikely(RISCV_ISA_EXT_SSTC)) {

    csr_write(CSR_STIMECMP, ULONG_MAX);
    csr_write(CSR_STIMECMPH, context.stimecmph);

    csr_write(CSR_STIMECMP, context.stimecmp);
    }
    csr_write(CSR_SATP, context.satp);

    }
    int cpu_suspend(unsigned long arg,
    int (*finish)(unsigned long arg,
    unsigned long entry,
    unsigned long context))
    {
    let mut rc: c_int = 0;
    let mut context: suspend_context = { 0 };
// Finisher should be non-NULL
    if (!finish)
    return -EINVAL;
// Save additional CSRs
    suspend_save_csrs(&context);
//
// Function graph tracer state gets inconsistent when the kernel
// calls functions that never return (aka finishers) hence disable
// graph tracing during their execution.
//
    pause_graph_tracing();
// Save context on stack
    if (__cpu_suspend_enter(&context)) {
// Call the finisher
    rc = finish(arg, __pa_symbol(__cpu_resume_enter),
    (ulong)&context);
//
// Should never reach here, unless the suspend finisher
// fails. Successful cpu_suspend() should return from
// __cpu_resume_entry()
//
    if (!rc)
    rc = -EOPNOTSUPP;
    }
// Enable function graph tracer
    unpause_graph_tracing();
// Restore additional CSRs
    suspend_restore_csrs(&context);
    return rc;
    }

    static int sbi_system_suspend(unsigned long sleep_type,
    unsigned long resume_addr,
    unsigned long opaque)
    {
    struct sbiret ret;
    ret = sbi_ecall(SBI_EXT_SUSP, SBI_EXT_SUSP_SYSTEM_SUSPEND,
    sleep_type, resume_addr, opaque, 0, 0, 0);
    if (ret.error)
    return sbi_err_map_linux_errno(ret.error);
    return ret.value;
    }
#[no_mangle]
unsafe extern "C" fn sbi_system_suspend_enter(state: suspend_state_t) -> c_int {
    static int sbi_system_suspend_enter(suspend_state_t state)
    {
    return cpu_suspend(SBI_SUSP_SLEEP_TYPE_SUSPEND_TO_RAM, sbi_system_suspend);
    }
    static const struct platform_suspend_ops sbi_system_suspend_ops = {
    .valid = suspend_valid_only_mem,
    .enter = sbi_system_suspend_enter,
    };
#[no_mangle]
unsafe extern "C" fn sbi_system_suspend_init() -> int __init {
    static int __init sbi_system_suspend_init(void)
    {
    if (sbi_spec_version >= sbi_mk_version(2, 0) &&
    sbi_probe_extension(SBI_EXT_SUSP) > 0) {
    pr_info("SBI SUSP extension detected\n");
    if (IS_ENABLED(CONFIG_SUSPEND))
    suspend_set_ops(&sbi_system_suspend_ops);
    }
    return 0;
    }
    arch_initcall(sbi_system_suspend_init);
    static int sbi_suspend_finisher(unsigned long suspend_type,
    unsigned long resume_addr,
    unsigned long opaque)
    {
    struct sbiret ret;
    ret = sbi_ecall(SBI_EXT_HSM, SBI_EXT_HSM_HART_SUSPEND,
    suspend_type, resume_addr, opaque, 0, 0, 0);
    return (ret.error) ? sbi_err_map_linux_errno(ret.error) : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn riscv_sbi_hart_suspend(state: u32) -> c_int {
    int riscv_sbi_hart_suspend(u32 state)
    {
    if (state & SBI_HSM_SUSP_NON_RET_BIT)
    return cpu_suspend(state, sbi_suspend_finisher);
    else
    return sbi_suspend_finisher(state, 0, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn riscv_sbi_suspend_state_is_valid(state: u32) -> bool {
    bool riscv_sbi_suspend_state_is_valid(u32 state)
    {
    if (state > SBI_HSM_SUSPEND_RET_DEFAULT &&
    state < SBI_HSM_SUSPEND_RET_PLATFORM)
    return false;
    if (state > SBI_HSM_SUSPEND_NON_RET_DEFAULT &&
    state < SBI_HSM_SUSPEND_NON_RET_PLATFORM)
    return false;
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn riscv_sbi_hsm_is_supported() -> bool {
    bool riscv_sbi_hsm_is_supported(void)
    {
//
// The SBI HSM suspend function is only available when:
// 1) SBI version is 0.3 or higher
// 2) SBI HSM extension is available
//
    if (sbi_spec_version < sbi_mk_version(0, 3) ||
    !sbi_probe_extension(SBI_EXT_HSM)) {
    pr_info("HSM suspend not available\n");
    return false;
    }
    return true;
    }
