//! Automatically rewritten from C to Rust
//! Source: drivers/soc/tegra/cbb/tegra-cbb.c
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
// Copyright (c) 2021-2022, NVIDIA CORPORATION. All rights reserved
//

#[no_mangle]
pub unsafe extern "C" fn tegra_cbb_print_err(file: *mut seq_file, fmt: *const c_char, ...) {
    void tegra_cbb_print_err(struct seq_file *file, const char *fmt, ...)
    {
    struct va_format vaf;
    va_list args;
    va_start(args, fmt);
    if (file) {
    seq_vprintf(file, fmt, args);
    } else {
    vaf.fmt = fmt;
    vaf.va = &args;
    pr_crit("%pV", &vaf);
    }
    va_end(args);
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_cbb_print_cache(file: *mut seq_file, cache: u32) {
    void tegra_cbb_print_cache(struct seq_file *file, u32 cache)
    {
    const char *buff_str, *mod_str, *rd_str, *wr_str;
    buff_str = (cache & BIT(0)) ? "Bufferable " : "";
    mod_str = (cache & BIT(1)) ? "Modifiable " : "";
    rd_str = (cache & BIT(2)) ? "Read-Allocate " : "";
    wr_str = (cache & BIT(3)) ? "Write-Allocate" : "";
    if (cache == 0x0)
    buff_str = "Device Non-Bufferable";
    tegra_cbb_print_err(file, "\t  Cache\t\t\t: 0x%x -- %s%s%s%s\n",
    cache, buff_str, mod_str, rd_str, wr_str);
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_cbb_print_prot(file: *mut seq_file, prot: u32) {
    void tegra_cbb_print_prot(struct seq_file *file, u32 prot)
    {
    const char *data_str, *secure_str, *priv_str;
    data_str = (prot & 0x4) ? "Instruction" : "Data";
    secure_str = (prot & 0x2) ? "Non-Secure" : "Secure";
    priv_str = (prot & 0x1) ? "Privileged" : "Unprivileged";
    tegra_cbb_print_err(file, "\t  Protection\t\t: 0x%x -- %s, %s, %s Access\n",
    prot, priv_str, secure_str, data_str);
    }
#[no_mangle]
unsafe extern "C" fn tegra_cbb_err_show(file: *mut seq_file, data: *mut c_void) -> c_int {
    static int tegra_cbb_err_show(struct seq_file *file, void *data)
    {
    struct tegra_cbb *cbb = file.private;
    return cbb.ops.debugfs_show(cbb, file, data);
    }
    DEFINE_SHOW_ATTRIBUTE(tegra_cbb_err);
#[no_mangle]
unsafe extern "C" fn tegra_cbb_err_debugfs_init(cbb: *mut tegra_cbb) {
    static void tegra_cbb_err_debugfs_init(struct tegra_cbb *cbb)
    {
    static struct dentry *root;
    if (!root)
    root = debugfs_create_file("tegra_cbb_err", 0444, core::ptr::null_mut(), cbb, &tegra_cbb_err_fops);
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_cbb_stall_enable(cbb: *mut tegra_cbb) {
    void tegra_cbb_stall_enable(struct tegra_cbb *cbb)
    {
    if (cbb.ops.stall_enable)
    cbb.ops.stall_enable(cbb);
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_cbb_fault_enable(cbb: *mut tegra_cbb) {
    void tegra_cbb_fault_enable(struct tegra_cbb *cbb)
    {
    if (cbb.ops.fault_enable)
    cbb.ops.fault_enable(cbb);
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_cbb_error_clear(cbb: *mut tegra_cbb) {
    void tegra_cbb_error_clear(struct tegra_cbb *cbb)
    {
    if (cbb.ops.error_clear)
    cbb.ops.error_clear(cbb);
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_cbb_get_status(cbb: *mut tegra_cbb) -> u32 {
    u32 tegra_cbb_get_status(struct tegra_cbb *cbb)
    {
    if (cbb.ops.get_status)
    return cbb.ops.get_status(cbb);
    return 0;
    }
    int tegra_cbb_get_irq(struct platform_device *pdev, unsigned int *nonsec_irq,
    unsigned int *sec_irq)
    {
    let mut index: c_uint = 0;
    let mut num_intr: c_int = 0, irq;
    num_intr = platform_irq_count(pdev);
    if (!num_intr)
    return -EINVAL;
    if (num_intr == 2) {
    irq = platform_get_irq(pdev, index);
    if (irq <= 0)
    return -ENOENT;
// nonsec_irq = irq;
    index++;
    }
    irq = platform_get_irq(pdev, index);
    if (irq <= 0)
    return -ENOENT;
// sec_irq = irq;
    if (num_intr == 1)
    dev_dbg(&pdev.dev, "secure IRQ: %u\n", *sec_irq);
    if (num_intr == 2)
    dev_dbg(&pdev.dev, "secure IRQ: %u, non-secure IRQ: %u\n", *sec_irq, *nonsec_irq);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_cbb_register(cbb: *mut tegra_cbb) -> c_int {
    int tegra_cbb_register(struct tegra_cbb *cbb)
    {
    int ret;
    if (IS_ENABLED(CONFIG_DEBUG_FS))
    tegra_cbb_err_debugfs_init(cbb);
// register interrupt handler for errors due to different initiators
    ret = cbb.ops.interrupt_enable(cbb);
    if (ret < 0) {
    dev_err(cbb.dev, "Failed to register CBB Interrupt ISR");
    return ret;
    }
    cbb.ops.error_enable(cbb);
    dsb(sy);
    return 0;
    }
