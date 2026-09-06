//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/cpu_ops.c
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
// CPU kernel entry/exit control
//
// Copyright (C) 2013 ARM Ltd.
//

    extern const struct cpu_operations smp_spin_table_ops;

    extern const struct cpu_operations acpi_parking_protocol_ops;

    extern const struct cpu_operations cpu_psci_ops;
    static const struct cpu_operations *cpu_ops[NR_CPUS] __ro_after_init;
    static const struct cpu_operations *const dt_supported_cpu_ops[] __initconst = {
    &smp_spin_table_ops,
    &cpu_psci_ops,
    core::ptr::null_mut(),
    };
    static const struct cpu_operations *const acpi_supported_cpu_ops[] __initconst = {

    &acpi_parking_protocol_ops,

    &cpu_psci_ops,
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn cpu_get_ops(name: *const c_char) -> *const cpu_operations  __init {
    static const struct cpu_operations * __init cpu_get_ops(const char *name)
    {
    const struct cpu_operations *const *ops;
    ops = acpi_disabled ? dt_supported_cpu_ops : acpi_supported_cpu_ops;
    while (*ops) {
    if (!strcmp(name, (*ops).name))
    return *ops;
    ops++;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn cpu_read_enable_method(cpu: c_int) -> *const char __init {
    static const char *__init cpu_read_enable_method(int cpu)
    {
    const char *enable_method;
    if (acpi_disabled) {
    struct device_node *dn = of_get_cpu_node(cpu, core::ptr::null_mut());
    if (!dn) {
    if (!cpu)
    pr_err("Failed to find device node for boot cpu\n");
    return core::ptr::null_mut();
    }
    enable_method = of_get_property(dn, "enable-method", core::ptr::null_mut());
    if (!enable_method) {
//
// The boot CPU may not have an enable method (e.g.
// when spin-table is used for secondaries).
// Don't warn spuriously.
//
    if (cpu != 0)
    pr_err("%pOF: missing enable-method property\n",
    dn);
    }
    of_node_put(dn);
    } else {
    enable_method = acpi_get_enable_method(cpu);
    if (!enable_method) {
//
// In ACPI systems the boot CPU does not require
// checking the enable method since for some
// boot protocol (ie parking protocol) it need not
// be initialized. Don't warn spuriously.
//
    if (cpu != 0)
    pr_err("Unsupported ACPI enable-method\n");
    }
    }
    return enable_method;
    }
//
// Read a cpu's enable method and record it in cpu_ops.
//
#[no_mangle]
pub unsafe extern "C" fn init_cpu_ops(cpu: c_int) -> int __init {
    int __init init_cpu_ops(int cpu)
    {
    const char *enable_method = cpu_read_enable_method(cpu);
    if (!enable_method)
    return -ENODEV;
    cpu_ops[cpu] = cpu_get_ops(enable_method);
    if (!cpu_ops[cpu]) {
    pr_warn("Unsupported enable-method: %s\n", enable_method);
    return -EOPNOTSUPP;
    }
    return 0;
    }
    const struct cpu_operations *get_cpu_ops(int cpu)
    {
    return cpu_ops[cpu];
    }
