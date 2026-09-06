//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kexec/core.c
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
// Code to handle transition of Linux booting another kernel.
//
// Copyright (C) 2002-2003 Eric Biederman  <ebiederm@xmission.com>
// GameCube/ppc32 port Copyright (C) 2004 Albert Herranz
// Copyright (C) 2005 IBM Corporation.
//

#[no_mangle]
pub unsafe extern "C" fn machine_crash_shutdown(regs: *mut pt_regs) {
    void machine_crash_shutdown(struct pt_regs *regs)
    {
    default_machine_crash_shutdown(regs);
    }

#[no_mangle]
pub unsafe extern "C" fn machine_kexec_cleanup(image: *mut kimage) {
    void machine_kexec_cleanup(struct kimage *image)
    {
    }
//
// Do not allocate memory (or fail in any way) in machine_kexec().
// We are past the point of no return, committed to rebooting now.
//
#[no_mangle]
pub unsafe extern "C" fn machine_kexec(image: *mut kimage) {
    void machine_kexec(struct kimage *image)
    {
    int save_ftrace_enabled;
    save_ftrace_enabled = __ftrace_enabled_save();
    this_cpu_disable_ftrace();
    if (ppc_md.machine_kexec)
    ppc_md.machine_kexec(image);
    else
    default_machine_kexec(image);
    this_cpu_enable_ftrace();
    __ftrace_enabled_restore(save_ftrace_enabled);
// Fall back to normal restart if we're still alive.
    machine_restart(core::ptr::null_mut());
    for(;;);
    }

    static unsigned long long crashk_cma_size;
#[no_mangle]
unsafe extern "C" fn get_crash_base(crash_base: c_ulonglong) -> unsigned long long __init {
    static unsigned long long __init get_crash_base(unsigned long long crash_base)
    {

    if (crash_base != KDUMP_KERNELBASE)
    printk("Crash kernel location must be 0x%x\n",
    KDUMP_KERNELBASE);
    return KDUMP_KERNELBASE;

    unsigned long long crash_base_align;
    if (!crash_base) {

//
// On the LPAR platform place the crash kernel to mid of
// RMA size (max. of 512MB) to ensure the crash kernel
// gets enough space to place itself and some stack to be
// in the first segment. At the same time normal kernel
// also get enough space to allocate memory for essential
// system resource in the first segment. Keep the crash
// kernel starts at 128MB offset on other platforms.
//
    if (firmware_has_feature(FW_FEATURE_LPAR))
    crash_base = min_t(u64, ppc64_rma_size / 2, SZ_512M);
    else
    crash_base = min_t(u64, ppc64_rma_size / 2, SZ_128M);

    crash_base = KDUMP_KERNELBASE;

    }
    crash_base_align = PAGE_ALIGN(crash_base);
    if (crash_base != crash_base_align)
    pr_warn("Crash kernel base must be aligned to 0x%lx\n", PAGE_SIZE);
    return crash_base_align;

    }
#[no_mangle]
pub unsafe extern "C" fn arch_reserve_crashkernel() -> void __init {
    void __init arch_reserve_crashkernel(void)
    {
    unsigned long long crash_size, crash_base, crash_end;
    unsigned long long kernel_start, kernel_size;
    unsigned long long total_mem_sz;
    int ret;
    total_mem_sz = memory_limit ? memory_limit : memblock_phys_mem_size();
// use common parsing
    ret = parse_crashkernel(boot_command_line, total_mem_sz, &crash_size,
    &crash_base, core::ptr::null_mut(), &crashk_cma_size, core::ptr::null_mut());
    if (ret)
    return;
    crash_base = get_crash_base(crash_base);
    crash_end = crash_base + crash_size - 1;
    kernel_start = __pa(_stext);
    kernel_size = _end - _stext;
// The crash region must not overlap the current kernel
    if ((kernel_start + kernel_size > crash_base) && (kernel_start <= crash_end)) {
    pr_warn("Crash kernel can not overlap current kernel\n");
    return;
    }
    reserve_crashkernel_generic(crash_size, crash_base, 0, false);
    }
#[no_mangle]
pub unsafe extern "C" fn kdump_cma_reserve() -> void __init {
    void __init kdump_cma_reserve(void)
    {
    if (crashk_cma_size)
    reserve_crashkernel_cma(crashk_cma_size);
    }
#[no_mangle]
pub unsafe extern "C" fn overlaps_crashkernel(start: c_ulong, size: c_ulong) -> int __init {
    int __init overlaps_crashkernel(unsigned long start, unsigned long size)
    {
    return (start + size) > crashk_res.start && start <= crashk_res.end;
    }
// Values we need to export to the second kernel via the device tree.
    static __be_word crashk_base;
    static __be_word crashk_size;
    static __be_word mem_limit;
    static struct property crashk_base_prop = {
    .name = "linux,crashkernel-base",
    .length = sizeof(__be_word),
    .value = &crashk_base
    };
    static struct property crashk_size_prop = {
    .name = "linux,crashkernel-size",
    .length = sizeof(__be_word),
    .value = &crashk_size,
    };
    static struct property memory_limit_prop = {
    .name = "linux,memory-limit",
    .length = sizeof(__be_word),
    .value = &mem_limit,
    };
#[no_mangle]
unsafe extern "C" fn export_crashk_values(node: *mut device_node) -> void __init {
    static void __init export_crashk_values(struct device_node *node)
    {
// There might be existing crash kernel properties, but we can't
// be sure what's in them, so remove them.
    of_remove_property(node, of_find_property(node,
    "linux,crashkernel-base", core::ptr::null_mut()));
    of_remove_property(node, of_find_property(node,
    "linux,crashkernel-size", core::ptr::null_mut()));
    if (crashk_res.start != 0) {
    crashk_base = cpu_to_be_ulong(crashk_res.start),
    of_add_property(node, &crashk_base_prop);
    crashk_size = cpu_to_be_ulong(resource_size(&crashk_res));
    of_add_property(node, &crashk_size_prop);
    }
//
// memory_limit is required by the kexec-tools to limit the
// crash regions to the actual memory used.
//
    mem_limit = cpu_to_be_ulong(memory_limit);
    of_update_property(node, &memory_limit_prop);
    }

    static __be_word kernel_end;
    static struct property kernel_end_prop = {
    .name = "linux,kernel-end",
    .length = sizeof(__be_word),
    .value = &kernel_end,
    };
#[no_mangle]
unsafe extern "C" fn kexec_setup() -> int __init {
    static int __init kexec_setup(void)
    {
    struct device_node *node;
    node = of_find_node_by_path("/chosen");
    if (!node)
    return -ENOENT;
// remove any stale properties so ours can be found
    of_remove_property(node, of_find_property(node, kernel_end_prop.name,
    core::ptr::null_mut()));
// information needed by userspace when using default_machine_kexec
    kernel_end = cpu_to_be_ulong(__pa(_end));
    of_add_property(node, &kernel_end_prop);

    export_crashk_values(node);

    of_node_put(node);
    return 0;
    }
    late_initcall(kexec_setup);
