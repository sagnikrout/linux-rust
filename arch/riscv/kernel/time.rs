//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/time.c
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
// Copyright (C) 2012 Regents of the University of California
// Copyright (C) 2017 SiFive
//

    unsigned long riscv_timebase __ro_after_init;
    EXPORT_SYMBOL_GPL(riscv_timebase);
#[no_mangle]
pub unsafe extern "C" fn time_init() -> void __init {
    void __init time_init(void)
    {
    struct device_node *cpu;
    struct acpi_table_rhct *rhct;
    acpi_status status;
    u32 prop;
    if (acpi_disabled) {
    cpu = of_find_node_by_path("/cpus");
    if (!cpu || of_property_read_u32(cpu, "timebase-frequency", &prop))
    panic("RISC-V system with no 'timebase-frequency' in DTS\n");
    of_node_put(cpu);
    riscv_timebase = prop;
    of_clk_init(core::ptr::null_mut());
    } else {
    status = acpi_get_table(ACPI_SIG_RHCT, 0, (struct acpi_table_header **)&rhct);
    if (ACPI_FAILURE(status))
    panic("RISC-V ACPI system with no RHCT table\n");
    riscv_timebase = rhct.time_base_freq;
    acpi_put_table((struct acpi_table_header *)rhct);
    }
    lpj_fine = riscv_timebase / HZ;
    timer_probe();
    tick_setup_hrtimer_broadcast();
    pv_time_init();
    }
