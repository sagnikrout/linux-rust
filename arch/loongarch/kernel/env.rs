//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kernel/env.c
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
// Author: Huacai Chen <chenhuacai@loongson.cn>
//
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

    u64 efi_system_table;
    struct loongson_system_configuration loongson_sysconf;
    EXPORT_SYMBOL(loongson_sysconf);
#[no_mangle]
pub unsafe extern "C" fn init_environ() -> void __init {
    void __init init_environ(void)
    {
    let mut efi_boot: c_int = fw_arg0;
    char *cmdline = early_memremap_ro(fw_arg1, COMMAND_LINE_SIZE);
    if (efi_boot)
    set_bit(EFI_BOOT, &efi.flags);
    else
    clear_bit(EFI_BOOT, &efi.flags);
    strscpy(boot_command_line, cmdline, COMMAND_LINE_SIZE);
    strscpy(init_command_line, cmdline, COMMAND_LINE_SIZE);
    early_memunmap(cmdline, COMMAND_LINE_SIZE);
    efi_system_table = fw_arg2;
    }
#[no_mangle]
unsafe extern "C" fn init_cpu_fullname() -> int __init {
    static int __init init_cpu_fullname(void)
    {
    int cpu, ret;
    char *cpuname;
    const char *model;
// Parsing cpuname from DTS model property
    ret = of_property_read_string(of_root, "model", &model);
    if (ret == 0) {
    cpuname = kstrdup(model, GFP_KERNEL);
    if (!cpuname)
    return -ENOMEM;
    loongson_sysconf.cpuname = strsep(&cpuname, " ");
    }
    if (loongson_sysconf.cpuname && !strncmp(loongson_sysconf.cpuname, "Loongson", 8)) {
    for (cpu = 0; cpu < NR_CPUS; cpu++)
    __cpu_full_name[cpu] = loongson_sysconf.cpuname;
    }
    return 0;
    }
    arch_initcall(init_cpu_fullname);
#[no_mangle]
unsafe extern "C" fn fdt_cpu_clk_init() -> int __init {
    static int __init fdt_cpu_clk_init(void)
    {
    struct clk *clk;
    struct device_node *np;
    np = of_get_cpu_node(0, core::ptr::null_mut());
    if (!np)
    return -ENODEV;
    clk = of_clk_get(np, 0);
    of_node_put(np);
    cpu_clock_freq = 200 * 1000 * 1000;
    if (IS_ERR(clk)) {
    pr_warn("No valid CPU clock freq, assume 200MHz.\n");
    return -ENODEV;
    }
    cpu_clock_freq = clk_get_rate(clk);
    clk_put(clk);
    return 0;
    }
    late_initcall(fdt_cpu_clk_init);
    static ssize_t boardinfo_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    return sysfs_emit(buf,
    "BIOS Information\n"
    "Vendor\t\t\t: %s\n"
    "Version\t\t\t: %s\n"
    "ROM Size\t\t: %d KB\n"
    "Release Date\t\t: %s\n\n"
    "Board Information\n"
    "Manufacturer\t\t: %s\n"
    "Board Name\t\t: %s\n"
    "Family\t\t\t: LOONGSON64\n\n",
    b_info.bios_vendor, b_info.bios_version,
    b_info.bios_size, b_info.bios_release_date,
    b_info.board_vendor, b_info.board_name);
    }
    static struct kobj_attribute boardinfo_attr = __ATTR(boardinfo, 0444,
    boardinfo_show, core::ptr::null_mut());
#[no_mangle]
unsafe extern "C" fn boardinfo_init() -> int __init {
    static int __init boardinfo_init(void)
    {
    struct kobject *loongson_kobj;
    loongson_kobj = kobject_create_and_add("loongson", firmware_kobj);
    if (!loongson_kobj)
    return -ENOMEM;
    return sysfs_create_file(loongson_kobj, &boardinfo_attr.attr);
    }
    late_initcall(boardinfo_init);
