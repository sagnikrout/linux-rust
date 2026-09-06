//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/nmi_selftest.c
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
// Testsuite for NMI: IPIs
//
// Started by Don Zickus:
// (using lib/locking-selftest.c as a guide)
//
// Copyright (C) 2011 Red Hat, Inc., Don Zickus <dzickus@redhat.com>
//

pub const SUCCESS: c_int = 0;
pub const FAILURE: c_int = 1;
pub const TIMEOUT: c_int = 2;
    static int __initdata nmi_fail;
// check to see if NMI IPIs work on this machine
    static DECLARE_BITMAP(nmi_ipi_mask, NR_CPUS) __initdata;
    static int __initdata testcase_total;
    static int __initdata testcase_successes;
    static int __initdata unexpected_testcase_failures;
    static int __initdata unexpected_testcase_unknowns;
#[no_mangle]
unsafe extern "C" fn nmi_unk_cb(val: c_uint, regs: *mut pt_regs) -> int __init {
    static int __init nmi_unk_cb(unsigned int val, struct pt_regs *regs)
    {
    unexpected_testcase_unknowns++;
    return NMI_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn init_nmi_testsuite() -> void __init {
    static void __init init_nmi_testsuite(void)
    {
// trap all the unknown NMIs we may generate
    register_nmi_handler(NMI_UNKNOWN, nmi_unk_cb, 0, "nmi_selftest_unk",
    __initdata);
    }
#[no_mangle]
unsafe extern "C" fn cleanup_nmi_testsuite() -> void __init {
    static void __init cleanup_nmi_testsuite(void)
    {
    unregister_nmi_handler(NMI_UNKNOWN, "nmi_selftest_unk");
    }
#[no_mangle]
unsafe extern "C" fn test_nmi_ipi_callback(val: c_uint, regs: *mut pt_regs) -> int __init {
    static int __init test_nmi_ipi_callback(unsigned int val, struct pt_regs *regs)
    {
    let mut cpu: c_int = raw_smp_processor_id();
    if (cpumask_test_and_clear_cpu(cpu, to_cpumask(nmi_ipi_mask)))
    return NMI_HANDLED;
    return NMI_DONE;
    }
#[no_mangle]
unsafe extern "C" fn test_nmi_ipi(mask: *mut cpumask) -> void __init {
    static void __init test_nmi_ipi(struct cpumask *mask)
    {
    unsigned long timeout;
    if (register_nmi_handler(NMI_LOCAL, test_nmi_ipi_callback,
    NMI_FLAG_FIRST, "nmi_selftest", __initdata)) {
    nmi_fail = FAILURE;
    return;
    }
// sync above data before sending NMI
    wmb();
    __apic_send_IPI_mask(mask, NMI_VECTOR);
// Don't wait longer than a second
    timeout = USEC_PER_SEC;
    while (!cpumask_empty(mask) && --timeout)
    udelay(1);
// What happens if we timeout, do we still unregister??
    unregister_nmi_handler(NMI_LOCAL, "nmi_selftest");
    if (!timeout)
    nmi_fail = TIMEOUT;
    return;
    }
#[no_mangle]
unsafe extern "C" fn remote_ipi() -> void __init {
    static void __init remote_ipi(void)
    {
    cpumask_copy(to_cpumask(nmi_ipi_mask), cpu_online_mask);
    cpumask_clear_cpu(smp_processor_id(), to_cpumask(nmi_ipi_mask));
    if (!cpumask_empty(to_cpumask(nmi_ipi_mask)))
    test_nmi_ipi(to_cpumask(nmi_ipi_mask));
    }
#[no_mangle]
unsafe extern "C" fn local_ipi() -> void __init {
    static void __init local_ipi(void)
    {
    cpumask_clear(to_cpumask(nmi_ipi_mask));
    cpumask_set_cpu(smp_processor_id(), to_cpumask(nmi_ipi_mask));
    test_nmi_ipi(to_cpumask(nmi_ipi_mask));
    }
#[no_mangle]
unsafe extern "C" fn reset_nmi() -> void __init {
    static void __init reset_nmi(void)
    {
    nmi_fail = 0;
    }
#[no_mangle]
unsafe extern "C" fn dotest((*testcase_fn)(void): *mut c_void, expected: c_int) -> void __init {
    static void __init dotest(void (*testcase_fn)(void), int expected)
    {
    testcase_fn();
//
// Filter out expected failures:
//
    if (nmi_fail != expected) {
    unexpected_testcase_failures++;
    if (nmi_fail == FAILURE)
    pr_cont("FAILED |");
#[no_mangle]
pub unsafe extern "C" fn if(TIMEOUT: nmi_fail ==) -> else {
    else if (nmi_fail == TIMEOUT)
    pr_cont("TIMEOUT|");
    else
    pr_cont("ERROR  |");
    dump_stack();
    } else {
    testcase_successes++;
    pr_cont("  ok  |");
    }
    pr_cont("\n");
    testcase_total++;
    reset_nmi();
    }
#[no_mangle]
pub unsafe extern "C" fn nmi_selftest() -> void __init {
    void __init nmi_selftest(void)
    {
    init_nmi_testsuite();
//
// Run the testsuite:
//
    pr_info("----------------\n");
    pr_info("| NMI testsuite:\n");
    pr_info("--------------------\n");
    pr_info("%12s:", "remote IPI");
    dotest(remote_ipi, SUCCESS);
    pr_info("%12s:", "local IPI");
    dotest(local_ipi, SUCCESS);
    cleanup_nmi_testsuite();
    pr_info("--------------------\n");
    if (unexpected_testcase_failures) {
    pr_info("BUG: %3d unexpected failures (out of %3d) - debugging disabled! |\n",
    unexpected_testcase_failures, testcase_total);
    } else {
    pr_info("Good, all %3d testcases passed! |\n",
    testcase_successes);
    }
    pr_info("-----------------------------------------------------------------\n");
    }
