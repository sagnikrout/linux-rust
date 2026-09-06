//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/loongson/lsdc_debugfs.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2023 Loongson Technology Corporation Limited
//

// device level debugfs
#[no_mangle]
unsafe extern "C" fn lsdc_identify(m: *mut seq_file, arg: *mut c_void) -> c_int {
    static int lsdc_identify(struct seq_file *m, void *arg)
    {
    struct drm_info_node *node = (struct drm_info_node *)m.private;
    struct lsdc_device *ldev = (struct lsdc_device *)node.info_ent.data;
    const struct loongson_gfx_desc *gfx = to_loongson_gfx(ldev.descp);
    u8 impl, rev;
    loongson_cpu_get_prid(&impl, &rev);
    seq_printf(m, "Running on cpu 0x%x, cpu revision: 0x%x\n",
    impl, rev);
    seq_printf(m, "Contained in: %s\n", gfx.model);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lsdc_show_mm(m: *mut seq_file, arg: *mut c_void) -> c_int {
    static int lsdc_show_mm(struct seq_file *m, void *arg)
    {
    struct drm_info_node *node = (struct drm_info_node *)m.private;
    struct drm_device *ddev = node.minor.dev;
    let mut p: drm_printer = drm_seq_file_printer(m);
    drm_mm_print(&ddev.vma_offset_manager.vm_addr_space_mm, &p);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lsdc_show_gfxpll_clock(m: *mut seq_file, arg: *mut c_void) -> c_int {
    static int lsdc_show_gfxpll_clock(struct seq_file *m, void *arg)
    {
    struct drm_info_node *node = (struct drm_info_node *)m.private;
    struct lsdc_device *ldev = (struct lsdc_device *)node.info_ent.data;
    let mut printer: drm_printer = drm_seq_file_printer(m);
    struct loongson_gfxpll *gfxpll = ldev.gfxpll;
    gfxpll.funcs.print(gfxpll, &printer, true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lsdc_show_benchmark(m: *mut seq_file, arg: *mut c_void) -> c_int {
    static int lsdc_show_benchmark(struct seq_file *m, void *arg)
    {
    struct drm_info_node *node = (struct drm_info_node *)m.private;
    struct lsdc_device *ldev = (struct lsdc_device *)node.info_ent.data;
    let mut printer: drm_printer = drm_seq_file_printer(m);
    lsdc_show_benchmark_copy(ldev, &printer);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lsdc_pdev_enable_io_mem(m: *mut seq_file, arg: *mut c_void) -> c_int {
    static int lsdc_pdev_enable_io_mem(struct seq_file *m, void *arg)
    {
    struct drm_info_node *node = (struct drm_info_node *)m.private;
    struct lsdc_device *ldev = (struct lsdc_device *)node.info_ent.data;
    u16 cmd;
    pci_read_config_word(ldev.dc, PCI_COMMAND, &cmd);
    seq_printf(m, "PCI_COMMAND: 0x%x\n", cmd);
    cmd |= PCI_COMMAND_MEMORY | PCI_COMMAND_IO;
    pci_write_config_word(ldev.dc, PCI_COMMAND, cmd);
    pci_read_config_word(ldev.dc, PCI_COMMAND, &cmd);
    seq_printf(m, "PCI_COMMAND: 0x%x\n", cmd);
    return 0;
    }
    static struct drm_info_list lsdc_debugfs_list[] = {
    { "benchmark",   lsdc_show_benchmark, 0, core::ptr::null_mut() },
    { "bos",         lsdc_show_buffer_object, 0, core::ptr::null_mut() },
    { "chips",       lsdc_identify, 0, core::ptr::null_mut() },
    { "clocks",      lsdc_show_gfxpll_clock, 0, core::ptr::null_mut() },
    { "dc_enable",   lsdc_pdev_enable_io_mem, 0, core::ptr::null_mut() },
    { "mm",          lsdc_show_mm, 0, core::ptr::null_mut() },
    };
#[no_mangle]
pub unsafe extern "C" fn lsdc_debugfs_init(minor: *mut drm_minor) {
    void lsdc_debugfs_init(struct drm_minor *minor)
    {
    struct drm_device *ddev = minor.dev;
    struct lsdc_device *ldev = to_lsdc(ddev);
    let mut n: c_uint = ARRAY_SIZE(lsdc_debugfs_list);
    unsigned int i;
    for (i = 0; i < n; ++i)
    lsdc_debugfs_list[i].data = ldev;
    drm_debugfs_create_files(lsdc_debugfs_list, n, minor.debugfs_root, minor);
    lsdc_ttm_debugfs_init(ldev);
    }
