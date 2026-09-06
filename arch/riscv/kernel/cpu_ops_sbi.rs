//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/cpu_ops_sbi.c
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
// HSM extension and cpu_ops implementation.
//
// Copyright (c) 2020 Western Digital Corporation or its affiliates.
//

    extern char secondary_start_sbi[];
    const struct cpu_operations cpu_ops_sbi;
//
// Ordered booting via HSM brings one cpu at a time. However, cpu hotplug can
// be invoked from multiple threads in parallel. Define an array of boot data
// to handle that.
//
    static struct sbi_hart_boot_data boot_data[NR_CPUS];
    static int sbi_hsm_hart_start(unsigned long hartid, unsigned long saddr,
    unsigned long priv)
    {
    struct sbiret ret;
    ret = sbi_ecall(SBI_EXT_HSM, SBI_EXT_HSM_HART_START,
    hartid, saddr, priv, 0, 0, 0);
    return sbi_err_map_linux_errno(ret.error);
    }

#[no_mangle]
unsafe extern "C" fn sbi_hsm_hart_stop() -> c_int {
    static int sbi_hsm_hart_stop(void)
    {
    struct sbiret ret;
    ret = sbi_ecall(SBI_EXT_HSM, SBI_EXT_HSM_HART_STOP, 0, 0, 0, 0, 0, 0);
    return sbi_err_map_linux_errno(ret.error);
    }
#[no_mangle]
unsafe extern "C" fn sbi_hsm_hart_get_status(hartid: c_ulong) -> c_int {
    static int sbi_hsm_hart_get_status(unsigned long hartid)
    {
    struct sbiret ret;
    ret = sbi_ecall(SBI_EXT_HSM, SBI_EXT_HSM_HART_STATUS,
    hartid, 0, 0, 0, 0, 0);
    if (ret.error)
    return sbi_err_map_linux_errno(ret.error);
    else
    return ret.value;
    }

#[no_mangle]
unsafe extern "C" fn sbi_cpu_start(cpuid: c_uint, tidle: *mut task_struct) -> c_int {
    static int sbi_cpu_start(unsigned int cpuid, struct task_struct *tidle)
    {
    let mut boot_addr: c_ulong = __pa_symbol(secondary_start_sbi);
    let mut hartid: c_ulong = cpuid_to_hartid_map(cpuid);
    unsigned long hsm_data;
    struct sbi_hart_boot_data *bdata = &boot_data[cpuid];
// Make sure tidle is updated
    smp_mb();
    bdata.task_ptr = tidle;
    bdata.stack_ptr = task_pt_regs(tidle);
// Make sure boot data is updated
    smp_mb();
    hsm_data = __pa(bdata);
    return sbi_hsm_hart_start(hartid, boot_addr, hsm_data);
    }

#[no_mangle]
unsafe extern "C" fn sbi_cpu_stop() {
    static void sbi_cpu_stop(void)
    {
    int ret;
    ret = sbi_hsm_hart_stop();
    pr_crit("Unable to stop the cpu %d (%d)\n", smp_processor_id(), ret);
    }
#[no_mangle]
unsafe extern "C" fn sbi_cpu_is_stopped(cpuid: c_uint) -> bool {
    static bool sbi_cpu_is_stopped(unsigned int cpuid)
    {
    int rc;
    let mut hartid: c_ulong = cpuid_to_hartid_map(cpuid);
    rc = sbi_hsm_hart_get_status(hartid);
    if (rc != SBI_HSM_STATE_STOPPED) {
    pr_warn("HART%lu isn't stopped; status %d\n", hartid, rc);
    return false;
    }
    return true;
    }

    const struct cpu_operations cpu_ops_sbi = {
    .cpu_start	= sbi_cpu_start,

    .cpu_stop	= sbi_cpu_stop,
    .cpu_is_stopped	= sbi_cpu_is_stopped,

    };
