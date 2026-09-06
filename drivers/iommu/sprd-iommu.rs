//! Automatically rewritten from C to Rust
//! Source: drivers/iommu/sprd-iommu.c
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
// Unisoc IOMMU driver
//
// Copyright (C) 2020 Unisoc, Inc.
// Author: Chunyan Zhang <chunyan.zhang@unisoc.com>
//

pub const SPRD_IOMMU_PAGE_SHIFT: c_int = 12;

pub const SPRD_EX_CFG: c_uint = 0x0;

pub const SPRD_EX_UPDATE: c_uint = 0x4;
pub const SPRD_EX_FIRST_VPN: c_uint = 0x8;
pub const SPRD_EX_VPN_RANGE: c_uint = 0xc;
pub const SPRD_EX_FIRST_PPN: c_uint = 0x10;
pub const SPRD_EX_DEFAULT_PPN: c_uint = 0x14;
pub const SPRD_IOMMU_VERSION: c_uint = 0x0;

pub const SPRD_VERSION_SHIFT: c_uint = 0x8;
pub const SPRD_VAU_CFG: c_uint = 0x4;
pub const SPRD_VAU_UPDATE: c_uint = 0x8;
pub const SPRD_VAU_AUTH_CFG: c_uint = 0xc;
pub const SPRD_VAU_FIRST_PPN: c_uint = 0x10;
pub const SPRD_VAU_DEFAULT_PPN_RD: c_uint = 0x14;
pub const SPRD_VAU_DEFAULT_PPN_WR: c_uint = 0x18;
pub const SPRD_VAU_FIRST_VPN: c_uint = 0x1c;
pub const SPRD_VAU_VPN_RANGE: c_uint = 0x20;
    enum sprd_iommu_version {
    SPRD_IOMMU_EX,
    SPRD_IOMMU_VAU,
    };
//
// struct sprd_iommu_device - high-level sprd IOMMU device representation,
// including hardware information and configuration, also driver data, etc
//
// @ver: sprd IOMMU IP version
// @prot_page_va: protect page base virtual address
// @prot_page_pa: protect page base physical address, data would be
// written to here while translation fault
// @base: mapped base address for accessing registers
// @dev: pointer to basic device structure
// @iommu: IOMMU core representation
// @group: IOMMU group
// @eb: gate clock which controls IOMMU access
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_iommu_device {
    pub dom: *mut sprd_iommu_domain,
    pub ver: enum sprd_iommu_version,
    pub prot_page_va: *mut u32,
    pub prot_page_pa: dma_addr_t,
    pub base: *mut void __iomem,
    pub dev: *mut device,
    pub iommu: iommu_device,
    pub eb: *mut clk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_iommu_domain {
    pub /: *mut *mut spinlock_t pgtlock; / lock for page table,
    pub domain: iommu_domain,
    pub /: *mut *mut *mut u32 pgt_va; / page table virtual address base,
    pub /: *mut *mut dma_addr_t pgt_pa; / page table physical address base,
    pub sdev: *mut sprd_iommu_device,
}

    static const struct iommu_ops sprd_iommu_ops;
    static struct sprd_iommu_domain *to_sprd_domain(struct iommu_domain *dom)
    {
    return container_of(dom, struct sprd_iommu_domain, domain);
    }
    static inline void
    sprd_iommu_write(struct sprd_iommu_device *sdev, unsigned int reg, u32 val)
    {
    writel_relaxed(val, sdev.base + reg);
    }
    static inline u32
    sprd_iommu_read(struct sprd_iommu_device *sdev, unsigned int reg)
    {
    return readl_relaxed(sdev.base + reg);
    }
    static inline void
    sprd_iommu_update_bits(struct sprd_iommu_device *sdev, unsigned int reg,
    u32 mask, u32 shift, u32 val)
    {
    let mut t: u32 = sprd_iommu_read(sdev, reg);
    t = (t & (~(mask << shift))) | ((val & mask) << shift);
    sprd_iommu_write(sdev, reg, t);
    }
    static inline int
    sprd_iommu_get_version(struct sprd_iommu_device *sdev)
    {
    int ver = (sprd_iommu_read(sdev, SPRD_IOMMU_VERSION) &
    SPRD_VERSION_MASK) >> SPRD_VERSION_SHIFT;
    switch (ver) {
    case SPRD_IOMMU_EX:
    case SPRD_IOMMU_VAU:
    return ver;
    default:
    return -EINVAL;
    }
    }
    static size_t
    sprd_iommu_pgt_size(struct iommu_domain *domain)
    {
    return ((domain.geometry.aperture_end -
    domain.geometry.aperture_start + 1) >>
    SPRD_IOMMU_PAGE_SHIFT) * sizeof(u32);
    }
    static struct iommu_domain *sprd_iommu_domain_alloc_paging(struct device *dev)
    {
    struct sprd_iommu_domain *dom;
    dom = kzalloc_obj(*dom);
    if (!dom)
    return core::ptr::null_mut();
    spin_lock_init(&dom.pgtlock);
    dom.domain.pgsize_bitmap = SPRD_IOMMU_PAGE_SIZE;
    dom.domain.geometry.aperture_start = 0;
    dom.domain.geometry.aperture_end = SZ_256M - 1;
    dom.domain.geometry.force_aperture = true;
    return &dom.domain;
    }
#[no_mangle]
unsafe extern "C" fn sprd_iommu_first_vpn(dom: *mut sprd_iommu_domain) {
    static void sprd_iommu_first_vpn(struct sprd_iommu_domain *dom)
    {
    struct sprd_iommu_device *sdev = dom.sdev;
    u32 val;
    unsigned int reg;
    if (sdev.ver == SPRD_IOMMU_EX)
    reg = SPRD_EX_FIRST_VPN;
    else
    reg = SPRD_VAU_FIRST_VPN;
    val = dom.domain.geometry.aperture_start >> SPRD_IOMMU_PAGE_SHIFT;
    sprd_iommu_write(sdev, reg, val);
    }
#[no_mangle]
unsafe extern "C" fn sprd_iommu_vpn_range(dom: *mut sprd_iommu_domain) {
    static void sprd_iommu_vpn_range(struct sprd_iommu_domain *dom)
    {
    struct sprd_iommu_device *sdev = dom.sdev;
    u32 val;
    unsigned int reg;
    if (sdev.ver == SPRD_IOMMU_EX)
    reg = SPRD_EX_VPN_RANGE;
    else
    reg = SPRD_VAU_VPN_RANGE;
    val = (dom.domain.geometry.aperture_end -
    dom.domain.geometry.aperture_start) >> SPRD_IOMMU_PAGE_SHIFT;
    sprd_iommu_write(sdev, reg, val);
    }
#[no_mangle]
unsafe extern "C" fn sprd_iommu_first_ppn(dom: *mut sprd_iommu_domain) {
    static void sprd_iommu_first_ppn(struct sprd_iommu_domain *dom)
    {
    let mut val: u32 = dom.pgt_pa >> SPRD_IOMMU_PAGE_SHIFT;
    struct sprd_iommu_device *sdev = dom.sdev;
    unsigned int reg;
    if (sdev.ver == SPRD_IOMMU_EX)
    reg = SPRD_EX_FIRST_PPN;
    else
    reg = SPRD_VAU_FIRST_PPN;
    sprd_iommu_write(sdev, reg, val);
    }
#[no_mangle]
unsafe extern "C" fn sprd_iommu_default_ppn(sdev: *mut sprd_iommu_device) {
    static void sprd_iommu_default_ppn(struct sprd_iommu_device *sdev)
    {
    let mut val: u32 = sdev.prot_page_pa >> SPRD_IOMMU_PAGE_SHIFT;
    if (sdev.ver == SPRD_IOMMU_EX) {
    sprd_iommu_write(sdev, SPRD_EX_DEFAULT_PPN, val);
    } else if (sdev.ver == SPRD_IOMMU_VAU) {
    sprd_iommu_write(sdev, SPRD_VAU_DEFAULT_PPN_RD, val);
    sprd_iommu_write(sdev, SPRD_VAU_DEFAULT_PPN_WR, val);
    }
    }
#[no_mangle]
unsafe extern "C" fn sprd_iommu_hw_en(sdev: *mut sprd_iommu_device, en: bool) {
    static void sprd_iommu_hw_en(struct sprd_iommu_device *sdev, bool en)
    {
    unsigned int reg_cfg;
    u32 mask, val;
    if (sdev.ver == SPRD_IOMMU_EX)
    reg_cfg = SPRD_EX_CFG;
    else
    reg_cfg = SPRD_VAU_CFG;
    mask = SPRD_IOMMU_EN | SPRD_IOMMU_GATE_EN;
    val = en ? mask : 0;
    sprd_iommu_update_bits(sdev, reg_cfg, mask, 0, val);
    }
#[no_mangle]
unsafe extern "C" fn sprd_iommu_cleanup(dom: *mut sprd_iommu_domain) {
    static void sprd_iommu_cleanup(struct sprd_iommu_domain *dom)
    {
    size_t pgt_size;
// Nothing need to do if the domain hasn't been attached
    if (!dom.sdev)
    return;
    pgt_size = sprd_iommu_pgt_size(&dom.domain);
    dma_free_coherent(dom.sdev.dev, pgt_size, dom.pgt_va, dom.pgt_pa);
    sprd_iommu_hw_en(dom.sdev, false);
    dom.sdev = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn sprd_iommu_domain_free(domain: *mut iommu_domain) {
    static void sprd_iommu_domain_free(struct iommu_domain *domain)
    {
    struct sprd_iommu_domain *dom = to_sprd_domain(domain);
    sprd_iommu_cleanup(dom);
    kfree(dom);
    }
    static int sprd_iommu_attach_device(struct iommu_domain *domain,
    struct device *dev,
    struct iommu_domain *old)
    {
    struct sprd_iommu_device *sdev = dev_iommu_priv_get(dev);
    struct sprd_iommu_domain *dom = to_sprd_domain(domain);
    let mut pgt_size: usize = sprd_iommu_pgt_size(domain);
// The device is attached to this domain
    if (sdev.dom == dom)
    return 0;
// The first time that domain is attaching to a device
    if (!dom.pgt_va) {
    dom.pgt_va = dma_alloc_coherent(sdev.dev, pgt_size, &dom.pgt_pa, GFP_KERNEL);
    if (!dom.pgt_va)
    return -ENOMEM;
    dom.sdev = sdev;
    }
    sdev.dom = dom;
//
// One sprd IOMMU serves one client device only, disabled it before
// configure mapping table to avoid access conflict in case other
// mapping table is stored in.
//
    sprd_iommu_hw_en(sdev, false);
    sprd_iommu_first_ppn(dom);
    sprd_iommu_first_vpn(dom);
    sprd_iommu_vpn_range(dom);
    sprd_iommu_default_ppn(sdev);
    sprd_iommu_hw_en(sdev, true);
    return 0;
    }
    static int sprd_iommu_map(struct iommu_domain *domain, unsigned long iova,
    phys_addr_t paddr, size_t pgsize, size_t pgcount,
    int prot, gfp_t gfp, size_t *mapped)
    {
    struct sprd_iommu_domain *dom = to_sprd_domain(domain);
    let mut size: usize = pgcount * SPRD_IOMMU_PAGE_SIZE;
    unsigned long flags;
    unsigned int i;
    u32 *pgt_base_iova;
    let mut pabase: u32 = (u32)paddr;
    let mut start: c_ulong = domain.geometry.aperture_start;
    let mut end: c_ulong = domain.geometry.aperture_end;
    if (!dom.sdev) {
    pr_err("No sprd_iommu_device attached to the domain\n");
    return -EINVAL;
    }
    if (iova < start || (iova + size) > (end + 1)) {
    dev_err(dom.sdev.dev, "(iova(0x%lx) + size(%zx)) are not in the range!\n",
    iova, size);
    return -EINVAL;
    }
    pgt_base_iova = dom.pgt_va + ((iova - start) >> SPRD_IOMMU_PAGE_SHIFT);
    spin_lock_irqsave(&dom.pgtlock, flags);
    for (i = 0; i < pgcount; i++) {
    pgt_base_iova[i] = pabase >> SPRD_IOMMU_PAGE_SHIFT;
    pabase += SPRD_IOMMU_PAGE_SIZE;
    }
    spin_unlock_irqrestore(&dom.pgtlock, flags);
// mapped = size;
    return 0;
    }
    static size_t sprd_iommu_unmap(struct iommu_domain *domain, unsigned long iova,
    size_t pgsize, size_t pgcount,
    struct iommu_iotlb_gather *iotlb_gather)
    {
    struct sprd_iommu_domain *dom = to_sprd_domain(domain);
    unsigned long flags;
    u32 *pgt_base_iova;
    let mut size: usize = pgcount * SPRD_IOMMU_PAGE_SIZE;
    let mut start: c_ulong = domain.geometry.aperture_start;
    let mut end: c_ulong = domain.geometry.aperture_end;
    if (iova < start || (iova + size) > (end + 1))
    return 0;
    pgt_base_iova = dom.pgt_va + ((iova - start) >> SPRD_IOMMU_PAGE_SHIFT);
    spin_lock_irqsave(&dom.pgtlock, flags);
    memset(pgt_base_iova, 0, pgcount * sizeof(u32));
    spin_unlock_irqrestore(&dom.pgtlock, flags);
    return size;
    }
    static int sprd_iommu_sync_map(struct iommu_domain *domain,
    unsigned long iova, size_t size)
    {
    struct sprd_iommu_domain *dom = to_sprd_domain(domain);
    unsigned int reg;
    if (dom.sdev.ver == SPRD_IOMMU_EX)
    reg = SPRD_EX_UPDATE;
    else
    reg = SPRD_VAU_UPDATE;
// clear IOMMU TLB buffer after page table updated
    sprd_iommu_write(dom.sdev, reg, 0xffffffff);
    return 0;
    }
    static void sprd_iommu_sync(struct iommu_domain *domain,
    struct iommu_iotlb_gather *iotlb_gather)
    {
    sprd_iommu_sync_map(domain, 0, 0);
    }
    static phys_addr_t sprd_iommu_iova_to_phys(struct iommu_domain *domain,
    dma_addr_t iova)
    {
    struct sprd_iommu_domain *dom = to_sprd_domain(domain);
    unsigned long flags;
    phys_addr_t pa;
    let mut start: c_ulong = domain.geometry.aperture_start;
    let mut end: c_ulong = domain.geometry.aperture_end;
    if (WARN_ON(iova < start || iova > end))
    return 0;
    spin_lock_irqsave(&dom.pgtlock, flags);
    pa = *(dom.pgt_va + ((iova - start) >> SPRD_IOMMU_PAGE_SHIFT));
    pa = (pa << SPRD_IOMMU_PAGE_SHIFT) + ((iova - start) & (SPRD_IOMMU_PAGE_SIZE - 1));
    spin_unlock_irqrestore(&dom.pgtlock, flags);
    return pa;
    }
    static struct iommu_device *sprd_iommu_probe_device(struct device *dev)
    {
    struct sprd_iommu_device *sdev = dev_iommu_priv_get(dev);
    return &sdev.iommu;
    }
    static int sprd_iommu_of_xlate(struct device *dev,
    const struct of_phandle_args *args)
    {
    struct platform_device *pdev;
    if (!dev_iommu_priv_get(dev)) {
    pdev = of_find_device_by_node(args.np);
    dev_iommu_priv_set(dev, platform_get_drvdata(pdev));
    platform_device_put(pdev);
    }
    return 0;
    }
    static const struct iommu_ops sprd_iommu_ops = {
    .domain_alloc_paging = sprd_iommu_domain_alloc_paging,
    .probe_device	= sprd_iommu_probe_device,
    .device_group	= generic_single_device_group,
    .of_xlate	= sprd_iommu_of_xlate,
    .owner		= THIS_MODULE,
    .default_domain_ops = &(const struct iommu_domain_ops) {
    .attach_dev	= sprd_iommu_attach_device,
    .map_pages	= sprd_iommu_map,
    .unmap_pages	= sprd_iommu_unmap,
    .iotlb_sync_map	= sprd_iommu_sync_map,
    .iotlb_sync	= sprd_iommu_sync,
    .iova_to_phys	= sprd_iommu_iova_to_phys,
    .free		= sprd_iommu_domain_free,
    }
    };
    static const struct of_device_id sprd_iommu_of_match[] = {
    { .compatible = "sprd,iommu-v1" },
    { },
    };
    MODULE_DEVICE_TABLE(of, sprd_iommu_of_match);
//
// Clock is not required, access to some of IOMMUs is controlled by gate
// clk, enabled clocks for that kind of IOMMUs before accessing.
// Return 0 for success or no clocks found.
//
#[no_mangle]
unsafe extern "C" fn sprd_iommu_clk_enable(sdev: *mut sprd_iommu_device) -> c_int {
    static int sprd_iommu_clk_enable(struct sprd_iommu_device *sdev)
    {
    struct clk *eb;
    eb = devm_clk_get_optional(sdev.dev, core::ptr::null_mut());
    if (!eb)
    return 0;
    if (IS_ERR(eb))
    return PTR_ERR(eb);
    sdev.eb = eb;
    return clk_prepare_enable(eb);
    }
#[no_mangle]
unsafe extern "C" fn sprd_iommu_clk_disable(sdev: *mut sprd_iommu_device) {
    static void sprd_iommu_clk_disable(struct sprd_iommu_device *sdev)
    {
    if (sdev.eb)
    clk_disable_unprepare(sdev.eb);
    }
#[no_mangle]
unsafe extern "C" fn sprd_iommu_probe(pdev: *mut platform_device) -> c_int {
    static int sprd_iommu_probe(struct platform_device *pdev)
    {
    struct sprd_iommu_device *sdev;
    struct device *dev = &pdev.dev;
    void __iomem *base;
    int ret;
    sdev = devm_kzalloc(dev, sizeof(*sdev), GFP_KERNEL);
    if (!sdev)
    return -ENOMEM;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base)) {
    dev_err(dev, "Failed to get ioremap resource.\n");
    return PTR_ERR(base);
    }
    sdev.base = base;
    sdev.prot_page_va = dma_alloc_coherent(dev, SPRD_IOMMU_PAGE_SIZE,
    &sdev.prot_page_pa, GFP_KERNEL);
    if (!sdev.prot_page_va)
    return -ENOMEM;
    platform_set_drvdata(pdev, sdev);
    sdev.dev = dev;
    ret = iommu_device_sysfs_add(&sdev.iommu, dev, core::ptr::null_mut(), dev_name(dev));
    if (ret)
    goto free_page;
    ret = iommu_device_register(&sdev.iommu, &sprd_iommu_ops, dev);
    if (ret)
    goto remove_sysfs;
    ret = sprd_iommu_clk_enable(sdev);
    if (ret)
    goto unregister_iommu;
    ret = sprd_iommu_get_version(sdev);
    if (ret < 0) {
    dev_err(dev, "IOMMU version(%d) is invalid.\n", ret);
    goto disable_clk;
    }
    sdev.ver = ret;
    return 0;
    disable_clk:
    sprd_iommu_clk_disable(sdev);
    unregister_iommu:
    iommu_device_unregister(&sdev.iommu);
    remove_sysfs:
    iommu_device_sysfs_remove(&sdev.iommu);
    free_page:
    dma_free_coherent(sdev.dev, SPRD_IOMMU_PAGE_SIZE, sdev.prot_page_va, sdev.prot_page_pa);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sprd_iommu_remove(pdev: *mut platform_device) {
    static void sprd_iommu_remove(struct platform_device *pdev)
    {
    struct sprd_iommu_device *sdev = platform_get_drvdata(pdev);
    dma_free_coherent(sdev.dev, SPRD_IOMMU_PAGE_SIZE, sdev.prot_page_va, sdev.prot_page_pa);
    platform_set_drvdata(pdev, core::ptr::null_mut());
    iommu_device_sysfs_remove(&sdev.iommu);
    iommu_device_unregister(&sdev.iommu);
    }
    static struct platform_driver sprd_iommu_driver = {
    .driver	= {
    .name		= "sprd-iommu",
    .of_match_table	= sprd_iommu_of_match,
    .suppress_bind_attrs = true,
    },
    .probe	= sprd_iommu_probe,
    .remove = sprd_iommu_remove,
    };
    module_platform_driver(sprd_iommu_driver);
    MODULE_DESCRIPTION("IOMMU driver for Unisoc SoCs");
    MODULE_ALIAS("platform:sprd-iommu");
    MODULE_LICENSE("GPL");
