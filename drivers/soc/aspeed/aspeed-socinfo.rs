//! Automatically rewritten from C to Rust
//! Source: drivers/soc/aspeed/aspeed-socinfo.c
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
// Copyright 2019 IBM Corp.

    static struct {
    const char *name;
    const u32 id;
    } const rev_table[] = {
// AST2400
    { "AST2400", 0x02000303 },
    { "AST1400", 0x02010103 },
    { "AST1250", 0x02010303 },
// AST2500
    { "AST2500", 0x04000303 },
    { "AST2510", 0x04000103 },
    { "AST2520", 0x04000203 },
    { "AST2530", 0x04000403 },
// AST2600
    { "AST2600", 0x05000303 },
    { "AST2620", 0x05010203 },
    { "AST2605", 0x05030103 },
    { "AST2625", 0x05030403 },
// AST2700
    { "AST2750", 0x06000003 },
    { "AST2700", 0x06000103 },
    { "AST2720", 0x06000203 },
    };
    static const char *siliconid_to_name(u32 siliconid)
    {
    let mut id: c_uint = siliconid & 0xff00ffff;
    unsigned int i;
    for (i = 0 ; i < ARRAY_SIZE(rev_table) ; ++i) {
    if ((rev_table[i].id & 0xff00ffff) == id)
    return rev_table[i].name;
    }
    return "Unknown";
    }
    static const char *siliconid_to_rev(u32 siliconid)
    {
    let mut rev: c_uint = (siliconid >> 16) & 0xff;
    let mut gen: c_uint = (siliconid >> 24) & 0xff;
    if (gen < 0x5) {
// AST2500 and below
    switch (rev) {
    case 0:
    return "A0";
    case 1:
    return "A1";
    case 3:
    return "A2";
    }
    } else {
// AST2600
    switch (rev) {
    case 0:
    return "A0";
    case 1:
    return "A1";
    case 2:
    return "A2";
    case 3:
    return "A3";
    }
    }
    return "??";
    }
#[no_mangle]
unsafe extern "C" fn aspeed_socinfo_init() -> int __init {
    static int __init aspeed_socinfo_init(void)
    {
    struct soc_device_attribute *attrs;
    struct soc_device *soc_dev;
    struct device_node *np;
    void __iomem *reg;
    let mut has_chipid: bool = false;
    u32 siliconid;
    u32 chipid[2];
    const char *machine = core::ptr::null_mut();
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "aspeed,silicon-id");
    if (!of_device_is_available(np)) {
    of_node_put(np);
    return -ENODEV;
    }
    reg = of_iomap(np, 0);
    if (!reg) {
    of_node_put(np);
    return -ENODEV;
    }
    siliconid = readl(reg);
    iounmap(reg);
// This is optional, the ast2400 does not have it
    reg = of_iomap(np, 1);
    if (reg) {
    has_chipid = true;
    chipid[0] = readl(reg);
    chipid[1] = readl(reg + 4);
    iounmap(reg);
    }
    of_node_put(np);
    attrs = kzalloc_obj(*attrs);
    if (!attrs)
    return -ENODEV;
//
// Machine: Romulus BMC
// Family: AST2500
// Revision: A1
// SoC ID: raw silicon revision id
// Serial Number: 64-bit chipid
//
    np = of_find_node_by_path("/");
    of_property_read_string(np, "model", &machine);
    if (machine)
    attrs.machine = kstrdup(machine, GFP_KERNEL);
    of_node_put(np);
    attrs.family = siliconid_to_name(siliconid);
    attrs.revision = siliconid_to_rev(siliconid);
    attrs.soc_id = kasprintf(GFP_KERNEL, "%08x", siliconid);
    if (has_chipid)
    attrs.serial_number = kasprintf(GFP_KERNEL, "%08x%08x",
    chipid[1], chipid[0]);
    soc_dev = soc_device_register(attrs);
    if (IS_ERR(soc_dev)) {
    kfree(attrs.machine);
    kfree(attrs.soc_id);
    kfree(attrs.serial_number);
    kfree(attrs);
    return PTR_ERR(soc_dev);
    }
    pr_info("ASPEED %s rev %s (%s)\n",
    attrs.family,
    attrs.revision,
    attrs.soc_id);
    return 0;
    }
    early_initcall(aspeed_socinfo_init);
