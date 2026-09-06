//! Automatically rewritten from C to Rust
//! Source: drivers/soc/vt8500/wmt-socinfo.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2025 Alexey Charkov <alchark@gmail.com>
// Based on aspeed-socinfo.c
//

    static const struct {
    const char *name;
    const u32 id;
    } chip_id_table[] = {
// VIA
    { "VT8420", 0x3300 },
    { "VT8430", 0x3357 },
    { "VT8500", 0x3400 },
// WonderMedia
    { "WM8425", 0x3429 },
    { "WM8435", 0x3437 },
    { "WM8440", 0x3451 },
    { "WM8505", 0x3426 },
    { "WM8650", 0x3465 },
    { "WM8750", 0x3445 },
    { "WM8850", 0x3481 },
    { "WM8880", 0x3498 },
    };
    static const char *sccid_to_name(u32 sccid)
    {
    let mut id: u32 = sccid >> 16;
    unsigned int i;
    for (i = 0 ; i < ARRAY_SIZE(chip_id_table) ; ++i) {
    if (chip_id_table[i].id == id)
    return chip_id_table[i].name;
    }
    return "Unknown";
    }
#[no_mangle]
unsafe extern "C" fn wmt_socinfo_probe(pdev: *mut platform_device) -> c_int {
    static int wmt_socinfo_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct soc_device_attribute *attrs;
    struct soc_device *soc_dev;
    char letter, digit;
    void __iomem *reg;
    u32 sccid;
    reg = devm_of_iomap(&pdev.dev, np, 0, core::ptr::null_mut());
    if (IS_ERR(reg))
    return PTR_ERR(reg);
    sccid = readl(reg);
    attrs = devm_kzalloc(&pdev.dev, sizeof(*attrs), GFP_KERNEL);
    if (!attrs)
    return -ENOMEM;
//
// Machine: VIA APC Rock
// Family: WM8850
// Revision: A2
// SoC ID: raw silicon revision id (34810103 in hexadecimal)
//
    attrs.family = sccid_to_name(sccid);
    letter = (sccid >> 8) & 0xf;
    letter = (letter - 1) + 'A';
    digit = sccid & 0xff;
    digit = (digit - 1) + '0';
    attrs.revision = devm_kasprintf(&pdev.dev, GFP_KERNEL,
    "%c%c", letter, digit);
    attrs.soc_id = devm_kasprintf(&pdev.dev, GFP_KERNEL, "%08x", sccid);
    if (!attrs.revision || !attrs.soc_id)
    return -ENOMEM;
    soc_dev = soc_device_register(attrs);
    if (IS_ERR(soc_dev))
    return PTR_ERR(soc_dev);
    dev_info(&pdev.dev,
    "VIA/WonderMedia %s rev %s (%s)\n",
    attrs.family,
    attrs.revision,
    attrs.soc_id);
    platform_set_drvdata(pdev, soc_dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wmt_socinfo_remove(pdev: *mut platform_device) {
    static void wmt_socinfo_remove(struct platform_device *pdev)
    {
    struct soc_device *soc_dev = platform_get_drvdata(pdev);
    soc_device_unregister(soc_dev);
    }
    static const struct of_device_id wmt_socinfo_ids[] = {
    { .compatible = "via,vt8500-scc-id" },
    { /* Sentinel */ },
    };
    static struct platform_driver wmt_socinfo = {
    .probe = wmt_socinfo_probe,
    .remove = wmt_socinfo_remove,
    .driver = {
    .name = "wmt-socinfo",
    .of_match_table = wmt_socinfo_ids,
    },
    };
    module_platform_driver(wmt_socinfo);
    MODULE_AUTHOR("Alexey Charkov <alchark@gmail.com>");
    MODULE_DESCRIPTION("VIA/WonderMedia socinfo driver");
    MODULE_LICENSE("GPL");
