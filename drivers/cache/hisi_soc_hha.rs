//! Automatically rewritten from C to Rust
//! Source: drivers/cache/hisi_soc_hha.c
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
// Driver for HiSilicon Hydra Home Agent (HHA).
//
// Copyright (c) 2025 HiSilicon Technologies Co., Ltd.
// Author: Yicong Yang <yangyicong@hisilicon.com>
// Yushan Wang <wangyushan12@huawei.com>
//
// A system typically contains multiple HHAs. Each is responsible for a subset
// of the physical addresses in the system, but interleave can make the mapping
// from a particular cache line to a responsible HHA complex. As such no
// filtering is done in the driver, with the hardware being responsible for
// responding with success for even if it was not responsible for any addresses
// in the range on which the operation was requested.
//

pub const HISI_HHA_CTRL: c_uint = 0x5004;

pub const HISI_HHA_START_L: c_uint = 0x5008;
pub const HISI_HHA_START_H: c_uint = 0x500c;
pub const HISI_HHA_LEN_L: c_uint = 0x5010;
pub const HISI_HHA_LEN_H: c_uint = 0x5014;
// The maintain operation performs in a 128 Byte granularity
pub const HISI_HHA_MAINT_ALIGN: c_int = 128;
pub const HISI_HHA_POLL_GAP_US: c_int = 10;
pub const HISI_HHA_POLL_TIMEOUT_US: c_int = 50000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_soc_hha {
// Must be first element
    pub cci: cache_coherency_ops_inst,
// Locks HHA instance to forbid overlapping access.
    pub lock: mutex,
    pub base: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn hisi_hha_cache_maintain_wait_finished(soc_hha: *mut hisi_soc_hha) -> bool {
    static bool hisi_hha_cache_maintain_wait_finished(struct hisi_soc_hha *soc_hha)
    {
    u32 val;
    return !readl_poll_timeout_atomic(soc_hha.base + HISI_HHA_CTRL, val,
    !(val & HISI_HHA_CTRL_EN),
    HISI_HHA_POLL_GAP_US,
    HISI_HHA_POLL_TIMEOUT_US);
    }
    static int hisi_soc_hha_wbinv(struct cache_coherency_ops_inst *cci,
    struct cc_inval_params *invp)
    {
    struct hisi_soc_hha *soc_hha =
    container_of(cci, struct hisi_soc_hha, cci);
    phys_addr_t top, addr = invp.addr;
    let mut size: usize = invp.size;
    u32 reg;
    if (!size)
    return -EINVAL;
    addr = ALIGN_DOWN(addr, HISI_HHA_MAINT_ALIGN);
    top = ALIGN(addr + size, HISI_HHA_MAINT_ALIGN);
    size = top - addr;
    guard(mutex)(&soc_hha.lock);
    if (!hisi_hha_cache_maintain_wait_finished(soc_hha))
    return -EBUSY;
//
// Hardware will search for addresses ranging [addr, addr + size - 1],
// last byte included, and perform maintenance in 128 byte granules
// on those cachelines which contain the addresses. If a given instance
// is either not responsible for a cacheline or that cacheline is not
// currently present then the search will fail, no operation will be
// necessary and the device will report success.
//
    size -= 1;
    writel(lower_32_bits(addr), soc_hha.base + HISI_HHA_START_L);
    writel(upper_32_bits(addr), soc_hha.base + HISI_HHA_START_H);
    writel(lower_32_bits(size), soc_hha.base + HISI_HHA_LEN_L);
    writel(upper_32_bits(size), soc_hha.base + HISI_HHA_LEN_H);
    reg = FIELD_PREP(HISI_HHA_CTRL_TYPE, 1); /* Clean Invalid */
    reg |= HISI_HHA_CTRL_RANGE | HISI_HHA_CTRL_EN;
    writel(reg, soc_hha.base + HISI_HHA_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hisi_soc_hha_done(cci: *mut cache_coherency_ops_inst) -> c_int {
    static int hisi_soc_hha_done(struct cache_coherency_ops_inst *cci)
    {
    struct hisi_soc_hha *soc_hha =
    container_of(cci, struct hisi_soc_hha, cci);
    guard(mutex)(&soc_hha.lock);
    if (!hisi_hha_cache_maintain_wait_finished(soc_hha))
    return -ETIMEDOUT;
    return 0;
    }
    static const struct cache_coherency_ops hha_ops = {
    .wbinv = hisi_soc_hha_wbinv,
    .done = hisi_soc_hha_done,
    };
#[no_mangle]
unsafe extern "C" fn hisi_soc_hha_probe(pdev: *mut platform_device) -> c_int {
    static int hisi_soc_hha_probe(struct platform_device *pdev)
    {
    struct hisi_soc_hha *soc_hha;
    struct resource *mem;
    int ret;
    soc_hha = cache_coherency_ops_instance_alloc(&hha_ops,
    struct hisi_soc_hha, cci);
    if (!soc_hha)
    return -ENOMEM;
    platform_set_drvdata(pdev, soc_hha);
    mutex_init(&soc_hha.lock);
    mem = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!mem) {
    ret = -ENOMEM;
    goto err_free_cci;
    }
    soc_hha.base = ioremap(mem.start, resource_size(mem));
    if (!soc_hha.base) {
    ret = dev_err_probe(&pdev.dev, -ENOMEM,
    "failed to remap io memory");
    goto err_free_cci;
    }
    ret = cache_coherency_ops_instance_register(&soc_hha.cci);
    if (ret)
    goto err_iounmap;
    return 0;
    err_iounmap:
    iounmap(soc_hha.base);
    err_free_cci:
    cache_coherency_ops_instance_put(&soc_hha.cci);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hisi_soc_hha_remove(pdev: *mut platform_device) {
    static void hisi_soc_hha_remove(struct platform_device *pdev)
    {
    struct hisi_soc_hha *soc_hha = platform_get_drvdata(pdev);
    cache_coherency_ops_instance_unregister(&soc_hha.cci);
    iounmap(soc_hha.base);
    cache_coherency_ops_instance_put(&soc_hha.cci);
    }
    static const struct acpi_device_id hisi_soc_hha_ids[] = {
    { "HISI0511", },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, hisi_soc_hha_ids);
    static struct platform_driver hisi_soc_hha_driver = {
    .driver = {
    .name = "hisi_soc_hha",
    .acpi_match_table = hisi_soc_hha_ids,
    },
    .probe = hisi_soc_hha_probe,
    .remove = hisi_soc_hha_remove,
    };
    module_platform_driver(hisi_soc_hha_driver);
    MODULE_IMPORT_NS("CACHE_COHERENCY");
    MODULE_DESCRIPTION("HiSilicon Hydra Home Agent driver supporting cache maintenance");
    MODULE_AUTHOR("Yicong Yang <yangyicong@hisilicon.com>");
    MODULE_AUTHOR("Yushan Wang <wangyushan12@huawei.com>");
    MODULE_LICENSE("GPL");
