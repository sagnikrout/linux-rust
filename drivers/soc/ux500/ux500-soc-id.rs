//! Automatically rewritten from C to Rust
//! Source: drivers/soc/ux500/ux500-soc-id.c
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
// Copyright (C) ST-Ericsson SA 2010
//
// Author: Rabin Vincent <rabin.vincent@stericsson.com> for ST-Ericsson
//

//
// struct dbx500_asic_id - fields of the ASIC ID
// @process: the manufacturing process, 0x40 is 40 nm 0x00 is "standard"
// @partnumber: hithereto 0x8500 for DB8500
// @revision: version code in the series
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbx500_asic_id {
    pub partnumber: u16,
    pub revision: u8,
    pub process: u8,
}

    static struct dbx500_asic_id dbx500_id;
#[no_mangle]
unsafe extern "C" fn ux500_read_asicid(addr: phys_addr_t) -> unsigned int __init {
    static unsigned int __init ux500_read_asicid(phys_addr_t addr)
    {
    void __iomem *virt = ioremap(addr, 4);
    unsigned int asicid;
    if (!virt)
    return 0;
    asicid = readl(virt);
    iounmap(virt);
    return asicid;
    }
#[no_mangle]
unsafe extern "C" fn ux500_print_soc_info(asicid: c_uint) {
    static void ux500_print_soc_info(unsigned int asicid)
    {
    let mut rev: c_uint = dbx500_id.revision;
    pr_info("DB%4x ", dbx500_id.partnumber);
    if (rev == 0x01)
    pr_cont("Early Drop");
#[no_mangle]
pub unsafe extern "C" fn if(0xA0: rev >=) -> else {
    else if (rev >= 0xA0)
    pr_cont("v%d.%d" , (rev >> 4) - 0xA + 1, rev & 0xf);
    else
    pr_cont("Unknown");
    pr_cont(" [%#010x]\n", asicid);
    }
#[no_mangle]
unsafe extern "C" fn partnumber(asicid: c_uint) -> c_uint {
    static unsigned int partnumber(unsigned int asicid)
    {
    return (asicid >> 8) & 0xffff;
    }
//
// SOC		MIDR		ASICID ADDRESS		ASICID VALUE
// DB8500ed	0x410fc090	0x9001FFF4		0x00850001
// DB8500v1	0x411fc091	0x9001FFF4		0x008500A0
// DB8500v1.1	0x411fc091	0x9001FFF4		0x008500A1
// DB8500v2	0x412fc091	0x9001DBF4		0x008500B0
// DB8520v2.2	0x412fc091	0x9001DBF4		0x008500B2
// DB5500v1	0x412fc091	0x9001FFF4		0x005500A0
// DB9540	0x413fc090	0xFFFFDBF4		0x009540xx
//
#[no_mangle]
unsafe extern "C" fn ux500_setup_id() -> void __init {
    static void __init ux500_setup_id(void)
    {
    let mut cpuid: c_uint = read_cpuid_id();
    let mut asicid: c_uint = 0;
    let mut addr: phys_addr_t = 0;
    switch (cpuid) {
    case 0x410fc090: /* DB8500ed */
    case 0x411fc091: /* DB8500v1 */
    addr = 0x9001FFF4;
    break;
    case 0x412fc091: /* DB8520 / DB8500v2 / DB5500v1 */
    asicid = ux500_read_asicid(0x9001DBF4);
    if (partnumber(asicid) == 0x8500 ||
    partnumber(asicid) == 0x8520)
// DB8500v2
    break;
// DB5500v1
    addr = 0x9001FFF4;
    break;
    case 0x413fc090: /* DB9540 */
    addr = 0xFFFFDBF4;
    break;
    }
    if (addr)
    asicid = ux500_read_asicid(addr);
    if (!asicid) {
    pr_err("Unable to identify SoC\n");
    BUG();
    }
    dbx500_id.process = asicid >> 24;
    dbx500_id.partnumber = partnumber(asicid);
    dbx500_id.revision = asicid & 0xff;
    ux500_print_soc_info(asicid);
    }
#[no_mangle]
unsafe extern "C" fn ux500_get_machine() -> *const char  __init {
    static const char * __init ux500_get_machine(void)
    {
    return kasprintf(GFP_KERNEL, "DB%4x", dbx500_id.partnumber);
    }
#[no_mangle]
unsafe extern "C" fn ux500_get_family() -> *const char  __init {
    static const char * __init ux500_get_family(void)
    {
    return kasprintf(GFP_KERNEL, "ux500");
    }
#[no_mangle]
unsafe extern "C" fn ux500_get_revision() -> *const char  __init {
    static const char * __init ux500_get_revision(void)
    {
    let mut rev: c_uint = dbx500_id.revision;
    if (rev == 0x01)
    return kasprintf(GFP_KERNEL, "%s", "ED");
#[no_mangle]
pub unsafe extern "C" fn if(0xA0: rev >=) -> else {
    else if (rev >= 0xA0)
    return kasprintf(GFP_KERNEL, "%d.%d",
    (rev >> 4) - 0xA + 1, rev & 0xf);
    return kasprintf(GFP_KERNEL, "%s", "Unknown");
    }
    static ssize_t
    process_show(struct device *dev, struct device_attribute *attr, char *buf)
    {
    if (dbx500_id.process == 0x00)
    return sprintf(buf, "Standard\n");
    return sprintf(buf, "%02xnm\n", dbx500_id.process);
    }
    static DEVICE_ATTR_RO(process);
    static struct attribute *ux500_soc_attrs[] = {
    &dev_attr_process.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(ux500_soc);
    static const char *db8500_read_soc_id(struct device_node *backupram)
    {
    void __iomem *base;
    const char *retstr;
    u32 uid[5];
    base = of_iomap(backupram, 0);
    if (!base)
    return core::ptr::null_mut();
    memcpy_fromio(uid, base + 0x1fc0, sizeof(uid));
// Throw these device-specific numbers into the entropy pool
    add_device_randomness(uid, sizeof(uid));
    retstr = kasprintf(GFP_KERNEL, "%08x%08x%08x%08x%08x",
    uid[0], uid[1], uid[2], uid[3], uid[4]);
    iounmap(base);
    return retstr;
    }
    static void __init soc_info_populate(struct soc_device_attribute *soc_dev_attr,
    struct device_node *backupram)
    {
    soc_dev_attr.soc_id   = db8500_read_soc_id(backupram);
    soc_dev_attr.machine  = ux500_get_machine();
    soc_dev_attr.family   = ux500_get_family();
    soc_dev_attr.revision = ux500_get_revision();
    soc_dev_attr.custom_attr_group = ux500_soc_groups[0];
    }
#[no_mangle]
unsafe extern "C" fn ux500_soc_device_init() -> int __init {
    static int __init ux500_soc_device_init(void)
    {
    struct soc_device *soc_dev;
    struct soc_device_attribute *soc_dev_attr;
    struct device_node *backupram;
    backupram = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "ste,dbx500-backupram");
    if (!backupram)
    return 0;
    ux500_setup_id();
    soc_dev_attr = kzalloc_obj(*soc_dev_attr);
    if (!soc_dev_attr) {
    of_node_put(backupram);
    return -ENOMEM;
    }
    soc_info_populate(soc_dev_attr, backupram);
    of_node_put(backupram);
    soc_dev = soc_device_register(soc_dev_attr);
    if (IS_ERR(soc_dev)) {
    kfree(soc_dev_attr);
    return PTR_ERR(soc_dev);
    }
    return 0;
    }
    subsys_initcall(ux500_soc_device_init);
