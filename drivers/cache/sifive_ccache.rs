//! Automatically rewritten from C to Rust
//! Source: drivers/cache/sifive_ccache.c
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
// SiFive composable cache controller Driver
//
// Copyright (C) 2018-2022 SiFive, Inc.
//

pub const SIFIVE_CCACHE_DIRECCFIX_LOW: c_uint = 0x100;
pub const SIFIVE_CCACHE_DIRECCFIX_HIGH: c_uint = 0x104;
pub const SIFIVE_CCACHE_DIRECCFIX_COUNT: c_uint = 0x108;
pub const SIFIVE_CCACHE_DIRECCFAIL_LOW: c_uint = 0x120;
pub const SIFIVE_CCACHE_DIRECCFAIL_HIGH: c_uint = 0x124;
pub const SIFIVE_CCACHE_DIRECCFAIL_COUNT: c_uint = 0x128;
pub const SIFIVE_CCACHE_DATECCFIX_LOW: c_uint = 0x140;
pub const SIFIVE_CCACHE_DATECCFIX_HIGH: c_uint = 0x144;
pub const SIFIVE_CCACHE_DATECCFIX_COUNT: c_uint = 0x148;
pub const SIFIVE_CCACHE_DATECCFAIL_LOW: c_uint = 0x160;
pub const SIFIVE_CCACHE_DATECCFAIL_HIGH: c_uint = 0x164;
pub const SIFIVE_CCACHE_DATECCFAIL_COUNT: c_uint = 0x168;
pub const SIFIVE_CCACHE_CONFIG: c_uint = 0x00;

pub const SIFIVE_CCACHE_FLUSH64: c_uint = 0x200;
pub const SIFIVE_CCACHE_FLUSH32: c_uint = 0x240;
pub const SIFIVE_CCACHE_WAYENABLE: c_uint = 0x08;
pub const SIFIVE_CCACHE_ECCINJECTERR: c_uint = 0x40;
pub const SIFIVE_CCACHE_MAX_ECCINTR: c_int = 4;
pub const SIFIVE_CCACHE_LINE_SIZE: c_int = 64;
    static void __iomem *ccache_base;
    static int g_irq[SIFIVE_CCACHE_MAX_ECCINTR];
    static struct riscv_cacheinfo_ops ccache_cache_ops;
    static int level;
    enum {
    DIR_CORR = 0,
    DATA_CORR,
    DATA_UNCORR,
    DIR_UNCORR,
    };
    enum {
    QUIRK_NONSTANDARD_CACHE_OPS	= BIT(0),
    QUIRK_BROKEN_DATA_UNCORR	= BIT(1),
    };

    static struct dentry *sifive_test;
    static ssize_t ccache_write(struct file *file, const char __user *data,
    size_t count, loff_t *ppos)
    {
    unsigned int val;
    if (kstrtouint_from_user(data, count, 0, &val))
    return -EINVAL;
    if ((val < 0xFF) || (val >= 0x10000 && val < 0x100FF))
    writel(val, ccache_base + SIFIVE_CCACHE_ECCINJECTERR);
    else
    return -EINVAL;
    return count;
    }
    static const struct file_operations ccache_fops = {
    .owner = THIS_MODULE,
    .open = simple_open,
    .write = ccache_write
    };
#[no_mangle]
unsafe extern "C" fn setup_sifive_debug() {
    static void setup_sifive_debug(void)
    {
    sifive_test = debugfs_create_dir("sifive_ccache_cache", core::ptr::null_mut());
    debugfs_create_file("sifive_debug_inject_error", 0200,
    sifive_test, core::ptr::null_mut(), &ccache_fops);
    }

#[no_mangle]
unsafe extern "C" fn ccache_config_read() {
    static void ccache_config_read(void)
    {
    u32 cfg;
    cfg = readl(ccache_base + SIFIVE_CCACHE_CONFIG);
    pr_info("%llu banks, %llu ways, sets/bank=%llu, bytes/block=%llu\n",
    FIELD_GET(SIFIVE_CCACHE_CONFIG_BANK_MASK, cfg),
    FIELD_GET(SIFIVE_CCACHE_CONFIG_WAYS_MASK, cfg),
    BIT_ULL(FIELD_GET(SIFIVE_CCACHE_CONFIG_SETS_MASK, cfg)),
    BIT_ULL(FIELD_GET(SIFIVE_CCACHE_CONFIG_BLKS_MASK, cfg)));
    cfg = readl(ccache_base + SIFIVE_CCACHE_WAYENABLE);
    pr_info("Index of the largest way enabled: %u\n", cfg);
    }
    static const struct of_device_id sifive_ccache_ids[] = {
    { .compatible = "eswin,eic7700-l3-cache",
    .data = (void *)(QUIRK_NONSTANDARD_CACHE_OPS) },
    { .compatible = "sifive,fu540-c000-ccache" },
    { .compatible = "sifive,fu740-c000-ccache" },
    { .compatible = "starfive,jh7100-ccache",
    .data = (void *)(QUIRK_NONSTANDARD_CACHE_OPS | QUIRK_BROKEN_DATA_UNCORR) },
    { .compatible = "starfive,jh7110-ccache",
    .data = (void *)(QUIRK_NONSTANDARD_CACHE_OPS) },
    { .compatible = "sifive,ccache0" },
    { /* end of table */ }
    };
    static ATOMIC_NOTIFIER_HEAD(ccache_err_chain);
#[no_mangle]
pub unsafe extern "C" fn register_sifive_ccache_error_notifier(nb: *mut notifier_block) -> c_int {
    int register_sifive_ccache_error_notifier(struct notifier_block *nb)
    {
    return atomic_notifier_chain_register(&ccache_err_chain, nb);
    }
    EXPORT_SYMBOL_GPL(register_sifive_ccache_error_notifier);
#[no_mangle]
pub unsafe extern "C" fn unregister_sifive_ccache_error_notifier(nb: *mut notifier_block) -> c_int {
    int unregister_sifive_ccache_error_notifier(struct notifier_block *nb)
    {
    return atomic_notifier_chain_unregister(&ccache_err_chain, nb);
    }
    EXPORT_SYMBOL_GPL(unregister_sifive_ccache_error_notifier);

#[no_mangle]
unsafe extern "C" fn ccache_flush_range(start: phys_addr_t, len: usize) {
    static void ccache_flush_range(phys_addr_t start, size_t len)
    {
    let mut end: phys_addr_t = start + len;
    phys_addr_t line;
    if (!len)
    return;
    mb(); /* complete earlier memory accesses before the cache flush */
    for (line = ALIGN_DOWN(start, SIFIVE_CCACHE_LINE_SIZE); line < end;
    line += SIFIVE_CCACHE_LINE_SIZE) {

    writel_relaxed(line >> 4, ccache_base + SIFIVE_CCACHE_FLUSH32);

    writeq_relaxed(line, ccache_base + SIFIVE_CCACHE_FLUSH64);

    }
    mb(); /* issue later memory accesses after the cache flush */
    }
    static const struct riscv_nonstd_cache_ops ccache_mgmt_ops __initconst = {
    .wback = &ccache_flush_range,
    .inv = &ccache_flush_range,
    .wback_inv = &ccache_flush_range,
    };

#[no_mangle]
unsafe extern "C" fn ccache_largest_wayenabled() -> c_int {
    static int ccache_largest_wayenabled(void)
    {
    return readl(ccache_base + SIFIVE_CCACHE_WAYENABLE) & 0xFF;
    }
    static ssize_t number_of_ways_enabled_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    return sprintf(buf, "%u\n", ccache_largest_wayenabled());
    }
    static DEVICE_ATTR_RO(number_of_ways_enabled);
    static struct attribute *priv_attrs[] = {
    &dev_attr_number_of_ways_enabled.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group priv_attr_group = {
    .attrs = priv_attrs,
    };
    static const struct attribute_group *ccache_get_priv_group(struct cacheinfo
// this_leaf)
    {
// We want to use private group for composable cache only
    if (this_leaf.level == level)
    return &priv_attr_group;
    else
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn ccache_int_handler(irq: c_int, device: *mut c_void) -> irqreturn_t {
    static irqreturn_t ccache_int_handler(int irq, void *device)
    {
    unsigned int add_h, add_l;
    if (irq == g_irq[DIR_CORR]) {
    add_h = readl(ccache_base + SIFIVE_CCACHE_DIRECCFIX_HIGH);
    add_l = readl(ccache_base + SIFIVE_CCACHE_DIRECCFIX_LOW);
    pr_err("DirError @ 0x%08X.%08X\n", add_h, add_l);
// Reading this register clears the DirError interrupt sig
    readl(ccache_base + SIFIVE_CCACHE_DIRECCFIX_COUNT);
    atomic_notifier_call_chain(&ccache_err_chain,
    SIFIVE_CCACHE_ERR_TYPE_CE,
    "DirECCFix");
    }
    if (irq == g_irq[DIR_UNCORR]) {
    add_h = readl(ccache_base + SIFIVE_CCACHE_DIRECCFAIL_HIGH);
    add_l = readl(ccache_base + SIFIVE_CCACHE_DIRECCFAIL_LOW);
// Reading this register clears the DirFail interrupt sig
    readl(ccache_base + SIFIVE_CCACHE_DIRECCFAIL_COUNT);
    atomic_notifier_call_chain(&ccache_err_chain,
    SIFIVE_CCACHE_ERR_TYPE_UE,
    "DirECCFail");
    panic("CCACHE: DirFail @ 0x%08X.%08X\n", add_h, add_l);
    }
    if (irq == g_irq[DATA_CORR]) {
    add_h = readl(ccache_base + SIFIVE_CCACHE_DATECCFIX_HIGH);
    add_l = readl(ccache_base + SIFIVE_CCACHE_DATECCFIX_LOW);
    pr_err("DataError @ 0x%08X.%08X\n", add_h, add_l);
// Reading this register clears the DataError interrupt sig
    readl(ccache_base + SIFIVE_CCACHE_DATECCFIX_COUNT);
    atomic_notifier_call_chain(&ccache_err_chain,
    SIFIVE_CCACHE_ERR_TYPE_CE,
    "DatECCFix");
    }
    if (irq == g_irq[DATA_UNCORR]) {
    add_h = readl(ccache_base + SIFIVE_CCACHE_DATECCFAIL_HIGH);
    add_l = readl(ccache_base + SIFIVE_CCACHE_DATECCFAIL_LOW);
    pr_err("DataFail @ 0x%08X.%08X\n", add_h, add_l);
// Reading this register clears the DataFail interrupt sig
    readl(ccache_base + SIFIVE_CCACHE_DATECCFAIL_COUNT);
    atomic_notifier_call_chain(&ccache_err_chain,
    SIFIVE_CCACHE_ERR_TYPE_UE,
    "DatECCFail");
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn sifive_ccache_probe(pdev: *mut platform_device) -> c_int {
    static int sifive_ccache_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    unsigned long quirks;
    int intr_num, rc;
    quirks = (unsigned long)device_get_match_data(dev);
    intr_num = platform_irq_count(pdev);
    if (!intr_num)
    return dev_err_probe(dev, -ENODEV, "No interrupts property\n");
    for (int i = 0; i < intr_num; i++) {
    if (i == DATA_UNCORR && (quirks & QUIRK_BROKEN_DATA_UNCORR))
    continue;
    g_irq[i] = platform_get_irq(pdev, i);
    if (g_irq[i] < 0)
    return g_irq[i];
    rc = devm_request_irq(dev, g_irq[i], ccache_int_handler, 0, "ccache_ecc", core::ptr::null_mut());
    if (rc)
    return dev_err_probe(dev, rc, "Could not request IRQ %d\n", g_irq[i]);
    }
    return 0;
    }
    static struct platform_driver sifive_ccache_driver = {
    .probe	= sifive_ccache_probe,
    .driver	= {
    .name		= "sifive_ccache",
    .of_match_table	= sifive_ccache_ids,
    },
    };
#[no_mangle]
unsafe extern "C" fn sifive_ccache_init() -> int __init {
    static int __init sifive_ccache_init(void)
    {
    struct device_node *np;
    struct resource res;
    const struct of_device_id *match;
    unsigned long quirks __maybe_unused;
    int rc;
    np = of_find_matching_node_and_match(core::ptr::null_mut(), sifive_ccache_ids, &match);
    if (!np)
    return -ENODEV;
    quirks = (uintptr_t)match.data;
    if (of_address_to_resource(np, 0, &res)) {
    rc = -ENODEV;
    goto err_node_put;
    }
    ccache_base = ioremap(res.start, resource_size(&res));
    if (!ccache_base) {
    rc = -ENOMEM;
    goto err_node_put;
    }
    if (of_property_read_u32(np, "cache-level", &level)) {
    rc = -ENOENT;
    goto err_unmap;
    }

    if (quirks & QUIRK_NONSTANDARD_CACHE_OPS) {
    riscv_cbom_block_size = SIFIVE_CCACHE_LINE_SIZE;
    riscv_noncoherent_supported();
    riscv_noncoherent_register_cache_ops(&ccache_mgmt_ops);
    }

    ccache_config_read();
    ccache_cache_ops.get_priv_group = ccache_get_priv_group;
    riscv_set_cacheinfo_ops(&ccache_cache_ops);

    setup_sifive_debug();

    rc = platform_driver_register(&sifive_ccache_driver);
    if (rc)
    goto err_unmap;
    of_node_put(np);
    return 0;
    err_unmap:
    iounmap(ccache_base);
    err_node_put:
    of_node_put(np);
    return rc;
    }
    arch_initcall(sifive_ccache_init);
