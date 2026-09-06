//! Automatically rewritten from C to Rust
//! Source: drivers/iommu/tegra-smmu.c
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
// Copyright (C) 2011-2014 NVIDIA CORPORATION.  All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_smmu_group {
    pub list: list_head,
    pub smmu: *mut tegra_smmu,
    pub soc: *const tegra_smmu_group_soc,
    pub group: *mut iommu_group,
    pub swgroup: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_smmu {
    pub regs: *mut void __iomem,
    pub dev: *mut device,
    pub mc: *mut tegra_mc,
    pub soc: *const tegra_smmu_soc,
    pub groups: list_head,
    pub pfn_mask: c_ulong,
    pub tlb_mask: c_ulong,
    pub asids: *mut c_ulong,
    pub lock: mutex,
    pub list: list_head,
    pub debugfs: *mut dentry,
    pub /: *mut *mut iommu_device iommu; / IOMMU Core code handle,
}

    struct tegra_pd;
    struct tegra_pt;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_smmu_as {
    pub domain: iommu_domain,
    pub smmu: *mut tegra_smmu,
    pub use_count: c_uint,
    pub lock: spinlock_t,
    pub count: *mut u32,
    pub pts: *mut tegra_pt,
    pub pd: *mut tegra_pd,
    pub pd_dma: dma_addr_t,
    pub id: unsigned,
    pub attr: u32,
}

    static struct tegra_smmu_as *to_smmu_as(struct iommu_domain *dom)
    {
    return container_of(dom, struct tegra_smmu_as, domain);
    }
    static inline void smmu_writel(struct tegra_smmu *smmu, u32 value,
    unsigned long offset)
    {
    writel(value, smmu.regs + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn smmu_readl(smmu: *mut tegra_smmu, offset: c_ulong) -> u32 {
    static inline u32 smmu_readl(struct tegra_smmu *smmu, unsigned long offset)
    {
    return readl(smmu.regs + offset);
    }
pub const SMMU_CONFIG: c_uint = 0x010;

pub const SMMU_TLB_CONFIG: c_uint = 0x14;

    ((smmu).soc.num_tlb_lines & (smmu).tlb_mask)
pub const SMMU_PTC_CONFIG: c_uint = 0x18;

pub const SMMU_PTB_ASID: c_uint = 0x01c;

pub const SMMU_PTB_DATA: c_uint = 0x020;

pub const SMMU_TLB_FLUSH: c_uint = 0x030;

    SMMU_TLB_FLUSH_VA_MATCH_SECTION)

    SMMU_TLB_FLUSH_VA_MATCH_GROUP)

pub const SMMU_PTC_FLUSH: c_uint = 0x034;

pub const SMMU_PTC_FLUSH_HI: c_uint = 0x9b8;
pub const SMMU_PTC_FLUSH_HI_MASK: c_uint = 0x3;
// per-SWGROUP SMMU_*_ASID register

pub const SMMU_ASID_MASK: c_uint = 0x7f;

// page table definitions
pub const SMMU_NUM_PDE: c_int = 1024;
pub const SMMU_NUM_PTE: c_int = 1024;

pub const SMMU_PDE_SHIFT: c_int = 22;
pub const SMMU_PTE_SHIFT: c_int = 12;

    SMMU_PDE_NONSECURE)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_pd {
    pub val: [u32; SMMU_NUM_PDE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_pt {
    pub val: [u32; SMMU_NUM_PTE],
}

#[no_mangle]
unsafe extern "C" fn iova_pd_index(iova: c_ulong) -> c_uint {
    static unsigned int iova_pd_index(unsigned long iova)
    {
    return (iova >> SMMU_PDE_SHIFT) & (SMMU_NUM_PDE - 1);
    }
#[no_mangle]
unsafe extern "C" fn iova_pt_index(iova: c_ulong) -> c_uint {
    static unsigned int iova_pt_index(unsigned long iova)
    {
    return (iova >> SMMU_PTE_SHIFT) & (SMMU_NUM_PTE - 1);
    }
#[no_mangle]
unsafe extern "C" fn smmu_dma_addr_valid(smmu: *mut tegra_smmu, addr: dma_addr_t) -> bool {
    static bool smmu_dma_addr_valid(struct tegra_smmu *smmu, dma_addr_t addr)
    {
    addr >>= 12;
    return (addr & smmu.pfn_mask) == addr;
    }
#[no_mangle]
unsafe extern "C" fn smmu_pde_to_dma(smmu: *mut tegra_smmu, pde: u32) -> dma_addr_t {
    static dma_addr_t smmu_pde_to_dma(struct tegra_smmu *smmu, u32 pde)
    {
    return (dma_addr_t)(pde & smmu.pfn_mask) << 12;
    }
#[no_mangle]
unsafe extern "C" fn smmu_flush_ptc_all(smmu: *mut tegra_smmu) {
    static void smmu_flush_ptc_all(struct tegra_smmu *smmu)
    {
    smmu_writel(smmu, SMMU_PTC_FLUSH_TYPE_ALL, SMMU_PTC_FLUSH);
    }
    static inline void smmu_flush_ptc(struct tegra_smmu *smmu, dma_addr_t dma,
    unsigned long offset)
    {
    u32 value;
    offset &= ~(smmu.mc.soc.atom_size - 1);
    if (smmu.mc.soc.num_address_bits > 32) {

    value = (dma >> 32) & SMMU_PTC_FLUSH_HI_MASK;

    value = 0;

    smmu_writel(smmu, value, SMMU_PTC_FLUSH_HI);
    }
    value = (dma + offset) | SMMU_PTC_FLUSH_TYPE_ADR;
    smmu_writel(smmu, value, SMMU_PTC_FLUSH);
    }
#[no_mangle]
pub unsafe extern "C" fn smmu_flush_tlb(smmu: *mut tegra_smmu) {
    static inline void smmu_flush_tlb(struct tegra_smmu *smmu)
    {
    smmu_writel(smmu, SMMU_TLB_FLUSH_VA_MATCH_ALL, SMMU_TLB_FLUSH);
    }
    static inline void smmu_flush_tlb_asid(struct tegra_smmu *smmu,
    unsigned long asid)
    {
    u32 value;
    if (smmu.soc.num_asids == 4)
    value = (asid & 0x3) << 29;
    else
    value = (asid & 0x7f) << 24;
    value |= SMMU_TLB_FLUSH_ASID_MATCH | SMMU_TLB_FLUSH_VA_MATCH_ALL;
    smmu_writel(smmu, value, SMMU_TLB_FLUSH);
    }
    static inline void smmu_flush_tlb_section(struct tegra_smmu *smmu,
    unsigned long asid,
    unsigned long iova)
    {
    u32 value;
    if (smmu.soc.num_asids == 4)
    value = (asid & 0x3) << 29;
    else
    value = (asid & 0x7f) << 24;
    value |= SMMU_TLB_FLUSH_ASID_MATCH | SMMU_TLB_FLUSH_VA_SECTION(iova);
    smmu_writel(smmu, value, SMMU_TLB_FLUSH);
    }
    static inline void smmu_flush_tlb_group(struct tegra_smmu *smmu,
    unsigned long asid,
    unsigned long iova)
    {
    u32 value;
    if (smmu.soc.num_asids == 4)
    value = (asid & 0x3) << 29;
    else
    value = (asid & 0x7f) << 24;
    value |= SMMU_TLB_FLUSH_ASID_MATCH | SMMU_TLB_FLUSH_VA_GROUP(iova);
    smmu_writel(smmu, value, SMMU_TLB_FLUSH);
    }
#[no_mangle]
pub unsafe extern "C" fn smmu_flush(smmu: *mut tegra_smmu) {
    static inline void smmu_flush(struct tegra_smmu *smmu)
    {
    smmu_readl(smmu, SMMU_PTB_ASID);
    }
#[no_mangle]
unsafe extern "C" fn tegra_smmu_alloc_asid(smmu: *mut tegra_smmu, idp: *mut c_uint) -> c_int {
    static int tegra_smmu_alloc_asid(struct tegra_smmu *smmu, unsigned int *idp)
    {
    unsigned long id;
    id = find_first_zero_bit(smmu.asids, smmu.soc.num_asids);
    if (id >= smmu.soc.num_asids)
    return -ENOSPC;
    set_bit(id, smmu.asids);
// idp = id;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_smmu_free_asid(smmu: *mut tegra_smmu, id: c_uint) {
    static void tegra_smmu_free_asid(struct tegra_smmu *smmu, unsigned int id)
    {
    clear_bit(id, smmu.asids);
    }
    static struct iommu_domain *tegra_smmu_domain_alloc_paging(struct device *dev)
    {
    struct tegra_smmu_as *as;
    as = kzalloc_obj(*as);
    if (!as)
    return core::ptr::null_mut();
    as.attr = SMMU_PD_READABLE | SMMU_PD_WRITABLE | SMMU_PD_NONSECURE;
    as.pd = iommu_alloc_pages_sz(GFP_KERNEL | __GFP_DMA, SMMU_SIZE_PD);
    if (!as.pd) {
    kfree(as);
    return core::ptr::null_mut();
    }
    as.count = kcalloc(SMMU_NUM_PDE, sizeof(u32), GFP_KERNEL);
    if (!as.count) {
    iommu_free_pages(as.pd);
    kfree(as);
    return core::ptr::null_mut();
    }
    as.pts = kzalloc_objs(*as.pts, SMMU_NUM_PDE);
    if (!as.pts) {
    kfree(as.count);
    iommu_free_pages(as.pd);
    kfree(as);
    return core::ptr::null_mut();
    }
    spin_lock_init(&as.lock);
    as.domain.pgsize_bitmap = SZ_4K;
// setup aperture
    as.domain.geometry.aperture_start = 0;
    as.domain.geometry.aperture_end = 0xffffffff;
    as.domain.geometry.force_aperture = true;
    return &as.domain;
    }
#[no_mangle]
unsafe extern "C" fn tegra_smmu_domain_free(domain: *mut iommu_domain) {
    static void tegra_smmu_domain_free(struct iommu_domain *domain)
    {
    struct tegra_smmu_as *as = to_smmu_as(domain);
// TODO: free page directory and page tables
    WARN_ON_ONCE(as.use_count);
    kfree(as.count);
    kfree(as.pts);
    kfree(as);
    }
    static const struct tegra_smmu_swgroup *
    tegra_smmu_find_swgroup(struct tegra_smmu *smmu, unsigned int swgroup)
    {
    const struct tegra_smmu_swgroup *group = core::ptr::null_mut();
    unsigned int i;
    for (i = 0; i < smmu.soc.num_swgroups; i++) {
    if (smmu.soc.swgroups[i].swgroup == swgroup) {
    group = &smmu.soc.swgroups[i];
    break;
    }
    }
    return group;
    }
    static void tegra_smmu_enable(struct tegra_smmu *smmu, unsigned int swgroup,
    unsigned int asid)
    {
    const struct tegra_smmu_swgroup *group;
    unsigned int i;
    u32 value;
    group = tegra_smmu_find_swgroup(smmu, swgroup);
    if (group) {
    value = smmu_readl(smmu, group.reg);
    value &= ~SMMU_ASID_MASK;
    value |= SMMU_ASID_VALUE(asid);
    value |= SMMU_ASID_ENABLE;
    smmu_writel(smmu, value, group.reg);
    } else {
    pr_warn("%s group from swgroup %u not found\n", __func__,
    swgroup);
// No point moving ahead if group was not found
    return;
    }
    for (i = 0; i < smmu.soc.num_clients; i++) {
    const struct tegra_mc_client *client = &smmu.soc.clients[i];
    if (client.swgroup != swgroup)
    continue;
    value = smmu_readl(smmu, client.regs.smmu.reg);
    value |= BIT(client.regs.smmu.bit);
    smmu_writel(smmu, value, client.regs.smmu.reg);
    }
    }
    static void tegra_smmu_disable(struct tegra_smmu *smmu, unsigned int swgroup,
    unsigned int asid)
    {
    const struct tegra_smmu_swgroup *group;
    unsigned int i;
    u32 value;
    group = tegra_smmu_find_swgroup(smmu, swgroup);
    if (group) {
    value = smmu_readl(smmu, group.reg);
    value &= ~SMMU_ASID_MASK;
    value |= SMMU_ASID_VALUE(asid);
    value &= ~SMMU_ASID_ENABLE;
    smmu_writel(smmu, value, group.reg);
    }
    for (i = 0; i < smmu.soc.num_clients; i++) {
    const struct tegra_mc_client *client = &smmu.soc.clients[i];
    if (client.swgroup != swgroup)
    continue;
    value = smmu_readl(smmu, client.regs.smmu.reg);
    value &= ~BIT(client.regs.smmu.bit);
    smmu_writel(smmu, value, client.regs.smmu.reg);
    }
    }
    static int tegra_smmu_as_prepare(struct tegra_smmu *smmu,
    struct tegra_smmu_as *as)
    {
    u32 value;
    let mut err: c_int = 0;
    mutex_lock(&smmu.lock);
    if (as.use_count > 0) {
    as.use_count++;
    goto unlock;
    }
    as.pd_dma =
    dma_map_single(smmu.dev, as.pd, SMMU_SIZE_PD, DMA_TO_DEVICE);
    if (dma_mapping_error(smmu.dev, as.pd_dma)) {
    err = -ENOMEM;
    goto unlock;
    }
// We can't handle 64-bit DMA addresses
    if (!smmu_dma_addr_valid(smmu, as.pd_dma)) {
    err = -ENOMEM;
    goto err_unmap;
    }
    err = tegra_smmu_alloc_asid(smmu, &as.id);
    if (err < 0)
    goto err_unmap;
    smmu_flush_ptc(smmu, as.pd_dma, 0);
    smmu_flush_tlb_asid(smmu, as.id);
    smmu_writel(smmu, as.id & 0x7f, SMMU_PTB_ASID);
    value = SMMU_PTB_DATA_VALUE(as.pd_dma, as.attr);
    smmu_writel(smmu, value, SMMU_PTB_DATA);
    smmu_flush(smmu);
    as.smmu = smmu;
    as.use_count++;
    mutex_unlock(&smmu.lock);
    return 0;
    err_unmap:
    dma_unmap_single(smmu.dev, as.pd_dma, SMMU_SIZE_PD, DMA_TO_DEVICE);
    unlock:
    mutex_unlock(&smmu.lock);
    return err;
    }
    static void tegra_smmu_as_unprepare(struct tegra_smmu *smmu,
    struct tegra_smmu_as *as)
    {
    mutex_lock(&smmu.lock);
    if (--as.use_count > 0) {
    mutex_unlock(&smmu.lock);
    return;
    }
    tegra_smmu_free_asid(smmu, as.id);
    dma_unmap_single(smmu.dev, as.pd_dma, SMMU_SIZE_PD, DMA_TO_DEVICE);
    as.smmu = core::ptr::null_mut();
    mutex_unlock(&smmu.lock);
    }
    static int tegra_smmu_attach_dev(struct iommu_domain *domain,
    struct device *dev, struct iommu_domain *old)
    {
    struct iommu_fwspec *fwspec = dev_iommu_fwspec_get(dev);
    struct tegra_smmu *smmu = dev_iommu_priv_get(dev);
    struct tegra_smmu_as *as = to_smmu_as(domain);
    unsigned int index;
    int err;
    if (!fwspec)
    return -ENOENT;
    for (index = 0; index < fwspec.num_ids; index++) {
    err = tegra_smmu_as_prepare(smmu, as);
    if (err)
    goto disable;
    tegra_smmu_enable(smmu, fwspec.ids[index], as.id);
    }
    if (index == 0)
    return -ENODEV;
    return 0;
    disable:
    while (index--) {
    tegra_smmu_disable(smmu, fwspec.ids[index], as.id);
    tegra_smmu_as_unprepare(smmu, as);
    }
    return err;
    }
    static int tegra_smmu_identity_attach(struct iommu_domain *identity_domain,
    struct device *dev,
    struct iommu_domain *old)
    {
    struct iommu_fwspec *fwspec = dev_iommu_fwspec_get(dev);
    struct tegra_smmu_as *as;
    struct tegra_smmu *smmu;
    unsigned int index;
    if (!fwspec)
    return -ENODEV;
    if (old == identity_domain || !old)
    return 0;
    as = to_smmu_as(old);
    smmu = as.smmu;
    for (index = 0; index < fwspec.num_ids; index++) {
    tegra_smmu_disable(smmu, fwspec.ids[index], as.id);
    tegra_smmu_as_unprepare(smmu, as);
    }
    return 0;
    }
    static struct iommu_domain_ops tegra_smmu_identity_ops = {
    .attach_dev = tegra_smmu_identity_attach,
    };
    static struct iommu_domain tegra_smmu_identity_domain = {
    .type = IOMMU_DOMAIN_IDENTITY,
    .ops = &tegra_smmu_identity_ops,
    };
    static void tegra_smmu_set_pde(struct tegra_smmu_as *as, unsigned long iova,
    u32 value)
    {
    let mut pd_index: c_uint = iova_pd_index(iova);
    struct tegra_smmu *smmu = as.smmu;
    u32 *pd = &as.pd.val[pd_index];
    let mut offset: c_ulong = pd_index * sizeof(*pd);
// Set the page directory entry first
// pd = value;
// The flush the page directory entry from caches
    dma_sync_single_range_for_device(smmu.dev, as.pd_dma, offset,
    sizeof(*pd), DMA_TO_DEVICE);
// And flush the iommu
    smmu_flush_ptc(smmu, as.pd_dma, offset);
    smmu_flush_tlb_section(smmu, as.id, iova);
    smmu_flush(smmu);
    }
    static u32 *tegra_smmu_pte_offset(struct tegra_pt *pt, unsigned long iova)
    {
    return &pt.val[iova_pt_index(iova)];
    }
    static u32 *tegra_smmu_pte_lookup(struct tegra_smmu_as *as, unsigned long iova,
    dma_addr_t *dmap)
    {
    let mut pd_index: c_uint = iova_pd_index(iova);
    struct tegra_smmu *smmu = as.smmu;
    struct tegra_pt *pt;
    pt = as.pts[pd_index];
    if (!pt)
    return core::ptr::null_mut();
// dmap = smmu_pde_to_dma(smmu, as->pd->val[pd_index]);
    return tegra_smmu_pte_offset(pt, iova);
    }
    static u32 *as_get_pte(struct tegra_smmu_as *as, dma_addr_t iova,
    dma_addr_t *dmap, struct tegra_pt *pt)
    {
    let mut pde: c_uint = iova_pd_index(iova);
    struct tegra_smmu *smmu = as.smmu;
    if (!as.pts[pde]) {
    dma_addr_t dma;
    dma = dma_map_single(smmu.dev, pt, SMMU_SIZE_PT,
    DMA_TO_DEVICE);
    if (dma_mapping_error(smmu.dev, dma)) {
    iommu_free_pages(pt);
    return core::ptr::null_mut();
    }
    if (!smmu_dma_addr_valid(smmu, dma)) {
    dma_unmap_single(smmu.dev, dma, SMMU_SIZE_PT,
    DMA_TO_DEVICE);
    iommu_free_pages(pt);
    return core::ptr::null_mut();
    }
    as.pts[pde] = pt;
    tegra_smmu_set_pde(as, iova, SMMU_MK_PDE(dma, SMMU_PDE_ATTR |
    SMMU_PDE_NEXT));
// dmap = dma;
    } else {
// dmap = smmu_pde_to_dma(smmu, as->pd->val[pde]);
    }
    return tegra_smmu_pte_offset(as.pts[pde], iova);
    }
#[no_mangle]
unsafe extern "C" fn tegra_smmu_pte_get_use(as: *mut tegra_smmu_as, iova: c_ulong) {
    static void tegra_smmu_pte_get_use(struct tegra_smmu_as *as, unsigned long iova)
    {
    let mut pd_index: c_uint = iova_pd_index(iova);
    as.count[pd_index]++;
    }
#[no_mangle]
unsafe extern "C" fn tegra_smmu_pte_put_use(as: *mut tegra_smmu_as, iova: c_ulong) {
    static void tegra_smmu_pte_put_use(struct tegra_smmu_as *as, unsigned long iova)
    {
    let mut pde: c_uint = iova_pd_index(iova);
    struct tegra_pt *pt = as.pts[pde];
//
// When no entries in this page table are used anymore, return the
// memory page to the system.
//
    if (--as.count[pde] == 0) {
    struct tegra_smmu *smmu = as.smmu;
    let mut pte_dma: dma_addr_t = smmu_pde_to_dma(smmu, as.pd.val[pde]);
    tegra_smmu_set_pde(as, iova, 0);
    dma_unmap_single(smmu.dev, pte_dma, SMMU_SIZE_PT,
    DMA_TO_DEVICE);
    iommu_free_pages(pt);
    as.pts[pde] = core::ptr::null_mut();
    }
    }
    static void tegra_smmu_set_pte(struct tegra_smmu_as *as, unsigned long iova,
    u32 *pte, dma_addr_t pte_dma, u32 val)
    {
    struct tegra_smmu *smmu = as.smmu;
    let mut offset: c_ulong = SMMU_OFFSET_IN_PAGE(pte);
// pte = val;
    dma_sync_single_range_for_device(smmu.dev, pte_dma, offset,
    4, DMA_TO_DEVICE);
    smmu_flush_ptc(smmu, pte_dma, offset);
    smmu_flush_tlb_group(smmu, as.id, iova);
    smmu_flush(smmu);
    }
    static struct tegra_pt *as_get_pde_page(struct tegra_smmu_as *as,
    unsigned long iova, gfp_t gfp,
    unsigned long *flags)
    {
    let mut pde: c_uint = iova_pd_index(iova);
    struct tegra_pt *pt = as.pts[pde];
// at first check whether allocation needs to be done at all
    if (pt)
    return pt;
//
// In order to prevent exhaustion of the atomic memory pool, we
// allocate page in a sleeping context if GFP flags permit. Hence
// spinlock needs to be unlocked and re-locked after allocation.
//
    if (gfpflags_allow_blocking(gfp))
    spin_unlock_irqrestore(&as.lock, *flags);
    pt = iommu_alloc_pages_sz(gfp | __GFP_DMA, SMMU_SIZE_PT);
    if (gfpflags_allow_blocking(gfp))
    spin_lock_irqsave(&as.lock, *flags);
//
// In a case of blocking allocation, a concurrent mapping may win
// the PDE allocation. In this case the allocated page isn't needed
// if allocation succeeded and the allocation failure isn't fatal.
//
    if (as.pts[pde]) {
    if (pt)
    iommu_free_pages(pt);
    pt = as.pts[pde];
    }
    return pt;
    }
    static int
    __tegra_smmu_map(struct iommu_domain *domain, unsigned long iova,
    phys_addr_t paddr, size_t size, int prot, gfp_t gfp,
    unsigned long *flags)
    {
    struct tegra_smmu_as *as = to_smmu_as(domain);
    dma_addr_t pte_dma;
    struct tegra_pt *pt;
    u32 pte_attrs;
    u32 *pte;
    pt = as_get_pde_page(as, iova, gfp, flags);
    if (!pt)
    return -ENOMEM;
    pte = as_get_pte(as, iova, &pte_dma, pt);
    if (!pte)
    return -ENOMEM;
// If we aren't overwriting a pre-existing entry, increment use
    if (*pte == 0)
    tegra_smmu_pte_get_use(as, iova);
    pte_attrs = SMMU_PTE_NONSECURE;
    if (prot & IOMMU_READ)
    pte_attrs |= SMMU_PTE_READABLE;
    if (prot & IOMMU_WRITE)
    pte_attrs |= SMMU_PTE_WRITABLE;
    tegra_smmu_set_pte(as, iova, pte, pte_dma,
    SMMU_PHYS_PFN(paddr) | pte_attrs);
    return 0;
    }
    static size_t
    __tegra_smmu_unmap(struct iommu_domain *domain, unsigned long iova,
    size_t size, struct iommu_iotlb_gather *gather)
    {
    struct tegra_smmu_as *as = to_smmu_as(domain);
    dma_addr_t pte_dma;
    u32 *pte;
    pte = tegra_smmu_pte_lookup(as, iova, &pte_dma);
    if (!pte || !*pte)
    return 0;
    tegra_smmu_set_pte(as, iova, pte, pte_dma, 0);
    tegra_smmu_pte_put_use(as, iova);
    return size;
    }
    static int tegra_smmu_map(struct iommu_domain *domain, unsigned long iova,
    phys_addr_t paddr, size_t size, size_t count,
    int prot, gfp_t gfp, size_t *mapped)
    {
    struct tegra_smmu_as *as = to_smmu_as(domain);
    unsigned long flags;
    int ret;
    spin_lock_irqsave(&as.lock, flags);
    ret = __tegra_smmu_map(domain, iova, paddr, size, prot, gfp, &flags);
    spin_unlock_irqrestore(&as.lock, flags);
    if (!ret)
// mapped = size;
    return ret;
    }
    static size_t tegra_smmu_unmap(struct iommu_domain *domain, unsigned long iova,
    size_t size, size_t count, struct iommu_iotlb_gather *gather)
    {
    struct tegra_smmu_as *as = to_smmu_as(domain);
    unsigned long flags;
    spin_lock_irqsave(&as.lock, flags);
    size = __tegra_smmu_unmap(domain, iova, size, gather);
    spin_unlock_irqrestore(&as.lock, flags);
    return size;
    }
    static phys_addr_t tegra_smmu_iova_to_phys(struct iommu_domain *domain,
    dma_addr_t iova)
    {
    struct tegra_smmu_as *as = to_smmu_as(domain);
    unsigned long pfn;
    dma_addr_t pte_dma;
    u32 *pte;
    pte = tegra_smmu_pte_lookup(as, iova, &pte_dma);
    if (!pte || !*pte)
    return 0;
    pfn = *pte & as.smmu.pfn_mask;
    return SMMU_PFN_PHYS(pfn) + SMMU_OFFSET_IN_PAGE(iova);
    }
    static struct tegra_smmu *tegra_smmu_find(struct device_node *np)
    {
    struct platform_device *pdev;
    struct tegra_mc *mc;
    pdev = of_find_device_by_node(np);
    if (!pdev)
    return core::ptr::null_mut();
    mc = platform_get_drvdata(pdev);
    put_device(&pdev.dev);
    if (!mc)
    return core::ptr::null_mut();
    return mc.smmu;
    }
    static int tegra_smmu_configure(struct tegra_smmu *smmu, struct device *dev,
    const struct of_phandle_args *args)
    {
    const struct iommu_ops *ops = smmu.iommu.ops;
    int err;
    err = iommu_fwspec_init(dev, dev_fwnode(smmu.dev));
    if (err < 0) {
    dev_err(dev, "failed to initialize fwspec: %d\n", err);
    return err;
    }
    err = ops.of_xlate(dev, args);
    if (err < 0) {
    dev_err(dev, "failed to parse SW group ID: %d\n", err);
    return err;
    }
    return 0;
    }
    static struct iommu_device *tegra_smmu_probe_device(struct device *dev)
    {
    struct device_node *np = dev.of_node;
    struct tegra_smmu *smmu = core::ptr::null_mut();
    struct of_phandle_args args;
    let mut index: c_uint = 0;
    int err;
    while (of_parse_phandle_with_args(np, "iommus", "#iommu-cells", index,
    &args) == 0) {
    smmu = tegra_smmu_find(args.np);
    if (smmu) {
    err = tegra_smmu_configure(smmu, dev, &args);
    if (err < 0) {
    of_node_put(args.np);
    return ERR_PTR(err);
    }
    }
    of_node_put(args.np);
    index++;
    }
    smmu = dev_iommu_priv_get(dev);
    if (!smmu)
    return ERR_PTR(-ENODEV);
    return &smmu.iommu;
    }
    static const struct tegra_smmu_group_soc *
    tegra_smmu_find_group(struct tegra_smmu *smmu, unsigned int swgroup)
    {
    unsigned int i, j;
    for (i = 0; i < smmu.soc.num_groups; i++)
    for (j = 0; j < smmu.soc.groups[i].num_swgroups; j++)
    if (smmu.soc.groups[i].swgroups[j] == swgroup)
    return &smmu.soc.groups[i];
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn tegra_smmu_group_release(iommu_data: *mut c_void) {
    static void tegra_smmu_group_release(void *iommu_data)
    {
    struct tegra_smmu_group *group = iommu_data;
    struct tegra_smmu *smmu = group.smmu;
    mutex_lock(&smmu.lock);
    list_del(&group.list);
    mutex_unlock(&smmu.lock);
    }
    static struct iommu_group *tegra_smmu_device_group(struct device *dev)
    {
    struct iommu_fwspec *fwspec = dev_iommu_fwspec_get(dev);
    struct tegra_smmu *smmu = dev_iommu_priv_get(dev);
    const struct tegra_smmu_group_soc *soc;
    let mut swgroup: c_uint = fwspec.ids[0];
    struct tegra_smmu_group *group;
    struct iommu_group *grp;
// Find group_soc associating with swgroup
    soc = tegra_smmu_find_group(smmu, swgroup);
    mutex_lock(&smmu.lock);
// Find existing iommu_group associating with swgroup or group_soc
    list_for_each_entry(group, &smmu.groups, list)
    if ((group.swgroup == swgroup) || (soc && group.soc == soc)) {
    grp = iommu_group_ref_get(group.group);
    mutex_unlock(&smmu.lock);
    return grp;
    }
    group = devm_kzalloc(smmu.dev, sizeof(*group), GFP_KERNEL);
    if (!group) {
    mutex_unlock(&smmu.lock);
    return core::ptr::null_mut();
    }
    INIT_LIST_HEAD(&group.list);
    group.swgroup = swgroup;
    group.smmu = smmu;
    group.soc = soc;
    if (dev_is_pci(dev))
    group.group = pci_device_group(dev);
    else
    group.group = generic_device_group(dev);
    if (IS_ERR(group.group)) {
    devm_kfree(smmu.dev, group);
    mutex_unlock(&smmu.lock);
    return core::ptr::null_mut();
    }
    iommu_group_set_iommudata(group.group, group, tegra_smmu_group_release);
    if (soc)
    iommu_group_set_name(group.group, soc.name);
    list_add_tail(&group.list, &smmu.groups);
    mutex_unlock(&smmu.lock);
    return group.group;
    }
    static int tegra_smmu_of_xlate(struct device *dev,
    const struct of_phandle_args *args)
    {
    struct platform_device *iommu_pdev = of_find_device_by_node(args.np);
    struct tegra_mc *mc = platform_get_drvdata(iommu_pdev);
    let mut id: u32 = args.args[0];
//
// Note: we are here releasing the reference of &iommu_pdev->dev, which
// is mc->dev. Although some functions in tegra_smmu_ops may keep using
// its private data beyond this point, it's still safe to do so because
// the SMMU parent device is the same as the MC, so the reference count
// isn't strictly necessary.
//
    put_device(&iommu_pdev.dev);
    dev_iommu_priv_set(dev, mc.smmu);
    return iommu_fwspec_add_ids(dev, &id, 1);
    }
#[no_mangle]
unsafe extern "C" fn tegra_smmu_def_domain_type(dev: *mut device) -> c_int {
    static int tegra_smmu_def_domain_type(struct device *dev)
    {
//
// FIXME: For now we want to run all translation in IDENTITY mode, due
// to some device quirks. Better would be to just quirk the troubled
// devices.
//
    return IOMMU_DOMAIN_IDENTITY;
    }
    static const struct iommu_ops tegra_smmu_ops = {
    .identity_domain = &tegra_smmu_identity_domain,
    .def_domain_type = &tegra_smmu_def_domain_type,
    .domain_alloc_paging = tegra_smmu_domain_alloc_paging,
    .probe_device = tegra_smmu_probe_device,
    .device_group = tegra_smmu_device_group,
    .of_xlate = tegra_smmu_of_xlate,
    .default_domain_ops = &(const struct iommu_domain_ops) {
    .attach_dev	= tegra_smmu_attach_dev,
    .map_pages	= tegra_smmu_map,
    .unmap_pages	= tegra_smmu_unmap,
    .iova_to_phys	= tegra_smmu_iova_to_phys,
    .free		= tegra_smmu_domain_free,
    }
    };
#[no_mangle]
unsafe extern "C" fn tegra_smmu_ahb_enable() {
    static void tegra_smmu_ahb_enable(void)
    {
    static const struct of_device_id ahb_match[] = {
    { .compatible = "nvidia,tegra30-ahb", },
    { }
    };
    struct device_node *ahb;
    ahb = of_find_matching_node(core::ptr::null_mut(), ahb_match);
    if (ahb) {
    tegra_ahb_enable_smmu(ahb);
    of_node_put(ahb);
    }
    }
#[no_mangle]
unsafe extern "C" fn tegra_smmu_swgroups_show(s: *mut seq_file, data: *mut c_void) -> c_int {
    static int tegra_smmu_swgroups_show(struct seq_file *s, void *data)
    {
    struct tegra_smmu *smmu = s.private;
    unsigned int i;
    u32 value;
    seq_printf(s, "swgroup    enabled  ASID\n");
    seq_printf(s, "------------------------\n");
    for (i = 0; i < smmu.soc.num_swgroups; i++) {
    const struct tegra_smmu_swgroup *group = &smmu.soc.swgroups[i];
    const char *status;
    unsigned int asid;
    value = smmu_readl(smmu, group.reg);
    if (value & SMMU_ASID_ENABLE)
    status = "yes";
    else
    status = "no";
    asid = value & SMMU_ASID_MASK;
    seq_printf(s, "%-9s  %-7s  %#04x\n", group.name, status,
    asid);
    }
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(tegra_smmu_swgroups);
#[no_mangle]
unsafe extern "C" fn tegra_smmu_clients_show(s: *mut seq_file, data: *mut c_void) -> c_int {
    static int tegra_smmu_clients_show(struct seq_file *s, void *data)
    {
    struct tegra_smmu *smmu = s.private;
    unsigned int i;
    u32 value;
    seq_printf(s, "client       enabled\n");
    seq_printf(s, "--------------------\n");
    for (i = 0; i < smmu.soc.num_clients; i++) {
    const struct tegra_mc_client *client = &smmu.soc.clients[i];
    const char *status;
    value = smmu_readl(smmu, client.regs.smmu.reg);
    if (value & BIT(client.regs.smmu.bit))
    status = "yes";
    else
    status = "no";
    seq_printf(s, "%-12s %s\n", client.name, status);
    }
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(tegra_smmu_clients);
#[no_mangle]
unsafe extern "C" fn tegra_smmu_debugfs_init(smmu: *mut tegra_smmu) {
    static void tegra_smmu_debugfs_init(struct tegra_smmu *smmu)
    {
    smmu.debugfs = debugfs_create_dir("smmu", core::ptr::null_mut());
    debugfs_create_file("swgroups", S_IRUGO, smmu.debugfs, smmu,
    &tegra_smmu_swgroups_fops);
    debugfs_create_file("clients", S_IRUGO, smmu.debugfs, smmu,
    &tegra_smmu_clients_fops);
    }
#[no_mangle]
unsafe extern "C" fn tegra_smmu_debugfs_exit(smmu: *mut tegra_smmu) {
    static void tegra_smmu_debugfs_exit(struct tegra_smmu *smmu)
    {
    debugfs_remove_recursive(smmu.debugfs);
    }
    struct tegra_smmu *tegra_smmu_probe(struct device *dev,
    const struct tegra_smmu_soc *soc,
    struct tegra_mc *mc)
    {
    struct tegra_smmu *smmu;
    u32 value;
    int err;
    smmu = devm_kzalloc(dev, sizeof(*smmu), GFP_KERNEL);
    if (!smmu)
    return ERR_PTR(-ENOMEM);
//
// This is a bit of a hack. Ideally we'd want to simply return this
// value. However iommu_device_register() will attempt to add
// all devices to the IOMMU before we get that far. In order
// not to rely on global variables to track the IOMMU instance, we
// set it here so that it can be looked up from the .probe_device()
// callback via the IOMMU device's .drvdata field.
//
    mc.smmu = smmu;
    smmu.asids = devm_bitmap_zalloc(dev, soc.num_asids, GFP_KERNEL);
    if (!smmu.asids)
    return ERR_PTR(-ENOMEM);
    INIT_LIST_HEAD(&smmu.groups);
    mutex_init(&smmu.lock);
    smmu.regs = mc.regs;
    smmu.soc = soc;
    smmu.dev = dev;
    smmu.mc = mc;
    smmu.pfn_mask =
    BIT_MASK(mc.soc.num_address_bits - SMMU_PTE_SHIFT) - 1;
    dev_dbg(dev, "address bits: %u, PFN mask: %#lx\n",
    mc.soc.num_address_bits, smmu.pfn_mask);
    smmu.tlb_mask = (1 << fls(smmu.soc.num_tlb_lines)) - 1;
    dev_dbg(dev, "TLB lines: %u, mask: %#lx\n", smmu.soc.num_tlb_lines,
    smmu.tlb_mask);
    value = SMMU_PTC_CONFIG_ENABLE | SMMU_PTC_CONFIG_INDEX_MAP(0x3f);
    if (soc.supports_request_limit)
    value |= SMMU_PTC_CONFIG_REQ_LIMIT(8);
    smmu_writel(smmu, value, SMMU_PTC_CONFIG);
    value = SMMU_TLB_CONFIG_HIT_UNDER_MISS |
    SMMU_TLB_CONFIG_ACTIVE_LINES(smmu);
    if (soc.supports_round_robin_arbitration)
    value |= SMMU_TLB_CONFIG_ROUND_ROBIN_ARBITRATION;
    smmu_writel(smmu, value, SMMU_TLB_CONFIG);
    smmu_flush_ptc_all(smmu);
    smmu_flush_tlb(smmu);
    smmu_writel(smmu, SMMU_CONFIG_ENABLE, SMMU_CONFIG);
    smmu_flush(smmu);
    tegra_smmu_ahb_enable();
    err = iommu_device_sysfs_add(&smmu.iommu, dev, core::ptr::null_mut(), dev_name(dev));
    if (err)
    return ERR_PTR(err);
    err = iommu_device_register(&smmu.iommu, &tegra_smmu_ops, dev);
    if (err) {
    iommu_device_sysfs_remove(&smmu.iommu);
    return ERR_PTR(err);
    }
    if (IS_ENABLED(CONFIG_DEBUG_FS))
    tegra_smmu_debugfs_init(smmu);
    return smmu;
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_smmu_remove(smmu: *mut tegra_smmu) {
    void tegra_smmu_remove(struct tegra_smmu *smmu)
    {
    iommu_device_unregister(&smmu.iommu);
    iommu_device_sysfs_remove(&smmu.iommu);
    if (IS_ENABLED(CONFIG_DEBUG_FS))
    tegra_smmu_debugfs_exit(smmu);
    }
