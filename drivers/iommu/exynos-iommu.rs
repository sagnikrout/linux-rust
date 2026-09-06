//! Automatically rewritten from C to Rust
//! Source: drivers/iommu/exynos-iommu.c
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
// Copyright (c) 2011,2016 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//

// Macro flag: #define DEBUG

    typedef u32 sysmmu_iova_t;
    typedef u32 sysmmu_pte_t;
    static struct iommu_domain exynos_identity_domain;
// We do not consider super section mapping (16MB)
pub const SECT_ORDER: c_int = 20;
pub const LPAGE_ORDER: c_int = 16;
pub const SPAGE_ORDER: c_int = 12;

    ((*(sent) & 3) == 0) || ((*(sent) & 3) == 3))

    ((*(sent) & 3) == 1))

//
// v1.x - v3.x SYSMMU supports 32bit physical and 32bit virtual address spaces
// v5.0 introduced support for 36bit physical address space by shifting
// all page entry values by 4 bits.
// All SYSMMU controllers in the system support the address spaces of the same
// size, so PG_ENT_SHIFT can be initialized on first SYSMMU probe to proper
// value (0 or 4).
//
    let mut PG_ENT_SHIFT: static short = -1;
pub const SYSMMU_PG_ENT_SHIFT: c_int = 0;
pub const SYSMMU_V5_PG_ENT_SHIFT: c_int = 4;
    static const sysmmu_pte_t *LV1_PROT;
    static const sysmmu_pte_t SYSMMU_LV1_PROT[] = {
    ((0 << 15) | (0 << 10)), /* no access */
    ((1 << 15) | (1 << 10)), /* IOMMU_READ only */
    ((0 << 15) | (1 << 10)), /* IOMMU_WRITE not supported, use read/write */
    ((0 << 15) | (1 << 10)), /* IOMMU_READ | IOMMU_WRITE */
    };
    static const sysmmu_pte_t SYSMMU_V5_LV1_PROT[] = {
    (0 << 4), /* no access */
    (1 << 4), /* IOMMU_READ only */
    (2 << 4), /* IOMMU_WRITE only */
    (3 << 4), /* IOMMU_READ | IOMMU_WRITE */
    };
    static const sysmmu_pte_t *LV2_PROT;
    static const sysmmu_pte_t SYSMMU_LV2_PROT[] = {
    ((0 << 9) | (0 << 4)), /* no access */
    ((1 << 9) | (1 << 4)), /* IOMMU_READ only */
    ((0 << 9) | (1 << 4)), /* IOMMU_WRITE not supported, use read/write */
    ((0 << 9) | (1 << 4)), /* IOMMU_READ | IOMMU_WRITE */
    };
    static const sysmmu_pte_t SYSMMU_V5_LV2_PROT[] = {
    (0 << 2), /* no access */
    (1 << 2), /* IOMMU_READ only */
    (2 << 2), /* IOMMU_WRITE only */
    (3 << 2), /* IOMMU_READ | IOMMU_WRITE */
    };

pub const NUM_LV1ENTRIES: c_int = 4096;

#[no_mangle]
unsafe extern "C" fn lv1ent_offset(iova: sysmmu_iova_t) -> u32 {
    static u32 lv1ent_offset(sysmmu_iova_t iova)
    {
    return iova >> SECT_ORDER;
    }
#[no_mangle]
unsafe extern "C" fn lv2ent_offset(iova: sysmmu_iova_t) -> u32 {
    static u32 lv2ent_offset(sysmmu_iova_t iova)
    {
    return (iova >> SPAGE_ORDER) & (NUM_LV2ENTRIES - 1);
    }

pub const CTRL_ENABLE: c_uint = 0x5;
pub const CTRL_BLOCK: c_uint = 0x7;
pub const CTRL_DISABLE: c_uint = 0x0;
pub const CFG_LRU: c_uint = 0x1;

// common registers
pub const REG_MMU_CTRL: c_uint = 0x000;
pub const REG_MMU_CFG: c_uint = 0x004;
pub const REG_MMU_STATUS: c_uint = 0x008;
pub const REG_MMU_VERSION: c_uint = 0x034;

// v1.x - v3.x registers
pub const REG_PAGE_FAULT_ADDR: c_uint = 0x024;
pub const REG_AW_FAULT_ADDR: c_uint = 0x028;
pub const REG_AR_FAULT_ADDR: c_uint = 0x02C;
pub const REG_DEFAULT_SLAVE_ADDR: c_uint = 0x030;
// v5.x registers
pub const REG_V5_FAULT_AR_VA: c_uint = 0x070;
pub const REG_V5_FAULT_AW_VA: c_uint = 0x080;
// v7.x registers
pub const REG_V7_CAPA0: c_uint = 0x870;
pub const REG_V7_CAPA1: c_uint = 0x874;
pub const REG_V7_CTRL_VM: c_uint = 0x8000;

    static struct device *dma_dev;
    static struct kmem_cache *lv2table_kmem_cache;
    static sysmmu_pte_t *zero_lv2_table;

    static sysmmu_pte_t *section_entry(sysmmu_pte_t *pgtable, sysmmu_iova_t iova)
    {
    return pgtable + lv1ent_offset(iova);
    }
    static sysmmu_pte_t *page_entry(sysmmu_pte_t *sent, sysmmu_iova_t iova)
    {
    return (sysmmu_pte_t *)phys_to_virt(
    lv2table_base(sent)) + lv2ent_offset(iova);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysmmu_fault {
    pub /: *mut *mut sysmmu_iova_t addr; / IOVA address that caused fault,
    pub /: *const *const *const char name; / human readable fault name,
    pub /: *mut *mut unsigned int type; / fault type for report_iommu_fault(),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysmmu_v1_fault_info {
    pub /: *mut *mut unsigned short addr_reg; / register to read IOVA fault address,
    pub /: *const *const *const char name; / human readable fault name,
    pub /: *mut *mut unsigned int type; / fault type for report_iommu_fault,
}

    static const struct sysmmu_v1_fault_info sysmmu_v1_faults[] = {
    { REG_PAGE_FAULT_ADDR, "PAGE", IOMMU_FAULT_READ },
    { REG_AR_FAULT_ADDR, "MULTI-HIT", IOMMU_FAULT_READ },
    { REG_AW_FAULT_ADDR, "MULTI-HIT", IOMMU_FAULT_WRITE },
    { REG_DEFAULT_SLAVE_ADDR, "BUS ERROR", IOMMU_FAULT_READ },
    { REG_AR_FAULT_ADDR, "SECURITY PROTECTION", IOMMU_FAULT_READ },
    { REG_AR_FAULT_ADDR, "ACCESS PROTECTION", IOMMU_FAULT_READ },
    { REG_AW_FAULT_ADDR, "SECURITY PROTECTION", IOMMU_FAULT_WRITE },
    { REG_AW_FAULT_ADDR, "ACCESS PROTECTION", IOMMU_FAULT_WRITE },
    };
// SysMMU v5 has the same faults for AR (0..4 bits) and AW (16..20 bits)
    static const char * const sysmmu_v5_fault_names[] = {
    "PTW",
    "PAGE",
    "MULTI-HIT",
    "ACCESS PROTECTION",
    "SECURITY PROTECTION"
    };
    static const char * const sysmmu_v7_fault_names[] = {
    "PTW",
    "PAGE",
    "ACCESS PROTECTION",
    "RESERVED"
    };
//
// This structure is attached to dev->iommu->priv of the master device
// on device add, contains a list of SYSMMU controllers defined by device tree,
// which are bound to given master device. It is usually referenced by 'owner'
// pointer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_iommu_owner {
    pub /: *mut *mut list_head controllers; / list of sysmmu_drvdata.owner_node,
    pub /: *mut *mut *mut iommu_domain domain; / domain this device is attached,
    pub /: *mut *mut mutex rpm_lock; / for runtime pm of all sysmmus,
}

//
// This structure exynos specific generalization of struct iommu_domain.
// It contains list of SYSMMU controllers from all master devices, which has
// been attached to this domain and page tables of IO address space defined by
// it. It is usually referenced by 'domain' pointer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_iommu_domain {
    pub /: *mut *mut list_head clients; / list of sysmmu_drvdata.domain_node,
    pub /: *mut *mut *mut sysmmu_pte_t pgtable; / lv1 page table, 16KB,
    pub /: *mut *mut *mut short lv2entcnt; / free lv2 entry counter for each section,
    pub /: *mut *mut spinlock_t lock; / lock for modifying list of clients,
    pub /: *mut *mut spinlock_t pgtablelock; / lock for modifying page table @ pgtable,
    pub /: *mut *mut iommu_domain domain; / generic domain data structure,
}

    struct sysmmu_drvdata;
//
// SysMMU version specific data. Contains offsets for the registers which can
// be found in different SysMMU variants, but have different offset values.
// Also contains version specific callbacks to abstract the hardware.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysmmu_variant {
    pub /: *mut *mut u32 pt_base; / page table base address (physical),
    pub /: *mut *mut u32 flush_all; / invalidate all TLB entries,
    pub /: *mut *mut u32 flush_entry; / invalidate specific TLB entry,
    pub /: *mut *mut u32 flush_range; / invalidate TLB entries in specified range,
    pub /: *mut *mut u32 flush_start; / start address of range invalidation,
    pub /: *mut *mut u32 flush_end; / end address of range invalidation,
    pub /: *mut *mut u32 int_status; / interrupt status information,
    pub /: *mut *mut u32 int_clear; / clear the interrupt,
    pub /: *mut *mut u32 fault_va; / IOVA address that caused fault,
    pub /: *mut *mut u32 fault_info; / fault transaction info,
    int (*get_fault_info)(struct sysmmu_drvdata *data, unsigned int itype,
    pub fault): *mut sysmmu_fault,
}

//
// This structure hold all data of a single SYSMMU controller, this includes
// hw resources like registers and clocks, pointers and list nodes to connect
// it to all other structures, internal state and parameters read from device
// tree. It is usually referenced by 'data' pointer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysmmu_drvdata {
    pub /: *mut *mut *mut device sysmmu; / SYSMMU controller device,
    pub /: *mut *mut *mut device master; / master device (owner),
    pub /: *mut *mut *mut device_link link; / runtime PM link to master,
    pub /: *mut *mut *mut void __iomem sfrbase; / our registers,
    pub /: *mut *mut *mut clk clk; / SYSMMU's clock,
    pub /: *mut *mut *mut clk aclk; / SYSMMU's aclk clock,
    pub /: *mut *mut *mut clk pclk; / SYSMMU's pclk clock,
    pub /: *mut *mut *mut clk clk_master; / master's device clock,
    pub /: *mut *mut spinlock_t lock; / lock for modifying state,
    pub /: *mut *mut bool active; / current status,
    pub /: *mut *mut *mut exynos_iommu_domain domain; / domain we belong to,
    pub /: *mut *mut list_head domain_node; / node for domain clients list,
    pub /: *mut *mut list_head owner_node; / node for owner controllers list,
    pub /: *mut *mut phys_addr_t pgtable; / assigned page table structure,
    pub /: *mut *mut unsigned int version; / our version,
    pub /: *mut *mut iommu_device iommu; / IOMMU core handle,
    pub /: *const *const *const sysmmu_variant variant; / version specific data,
// v7 fields
    pub /: *mut *mut bool has_vcr; / virtual machine control register,
}

    static int exynos_sysmmu_v1_get_fault_info(struct sysmmu_drvdata *data,
    unsigned int itype,
    struct sysmmu_fault *fault)
    {
    const struct sysmmu_v1_fault_info *finfo;
    if (itype >= ARRAY_SIZE(sysmmu_v1_faults))
    return -ENXIO;
    finfo = &sysmmu_v1_faults[itype];
    fault.addr = readl(data.sfrbase + finfo.addr_reg);
    fault.name = finfo.name;
    fault.type = finfo.type;
    return 0;
    }
    static int exynos_sysmmu_v5_get_fault_info(struct sysmmu_drvdata *data,
    unsigned int itype,
    struct sysmmu_fault *fault)
    {
    unsigned int addr_reg;
    if (itype < ARRAY_SIZE(sysmmu_v5_fault_names)) {
    fault.type = IOMMU_FAULT_READ;
    addr_reg = REG_V5_FAULT_AR_VA;
    } else if (itype >= 16 && itype <= 20) {
    fault.type = IOMMU_FAULT_WRITE;
    addr_reg = REG_V5_FAULT_AW_VA;
    itype -= 16;
    } else {
    return -ENXIO;
    }
    fault.name = sysmmu_v5_fault_names[itype];
    fault.addr = readl(data.sfrbase + addr_reg);
    return 0;
    }
    static int exynos_sysmmu_v7_get_fault_info(struct sysmmu_drvdata *data,
    unsigned int itype,
    struct sysmmu_fault *fault)
    {
    let mut info: u32 = readl(SYSMMU_REG(data, fault_info));
    fault.addr = readl(SYSMMU_REG(data, fault_va));
    fault.name = sysmmu_v7_fault_names[itype % 4];
    fault.type = (info & BIT(20)) ? IOMMU_FAULT_WRITE : IOMMU_FAULT_READ;
    return 0;
    }
// SysMMU v1..v3
    static const struct sysmmu_variant sysmmu_v1_variant = {
    .flush_all	= 0x0c,
    .flush_entry	= 0x10,
    .pt_base	= 0x14,
    .int_status	= 0x18,
    .int_clear	= 0x1c,
    .get_fault_info	= exynos_sysmmu_v1_get_fault_info,
    };
// SysMMU v5
    static const struct sysmmu_variant sysmmu_v5_variant = {
    .pt_base	= 0x0c,
    .flush_all	= 0x10,
    .flush_entry	= 0x14,
    .flush_range	= 0x18,
    .flush_start	= 0x20,
    .flush_end	= 0x24,
    .int_status	= 0x60,
    .int_clear	= 0x64,
    .get_fault_info	= exynos_sysmmu_v5_get_fault_info,
    };
// SysMMU v7: non-VM capable register layout
    static const struct sysmmu_variant sysmmu_v7_variant = {
    .pt_base	= 0x0c,
    .flush_all	= 0x10,
    .flush_entry	= 0x14,
    .flush_range	= 0x18,
    .flush_start	= 0x20,
    .flush_end	= 0x24,
    .int_status	= 0x60,
    .int_clear	= 0x64,
    .fault_va	= 0x70,
    .fault_info	= 0x78,
    .get_fault_info	= exynos_sysmmu_v7_get_fault_info,
    };
// SysMMU v7: VM capable register layout
    static const struct sysmmu_variant sysmmu_v7_vm_variant = {
    .pt_base	= 0x800c,
    .flush_all	= 0x8010,
    .flush_entry	= 0x8014,
    .flush_range	= 0x8018,
    .flush_start	= 0x8020,
    .flush_end	= 0x8024,
    .int_status	= 0x60,
    .int_clear	= 0x64,
    .fault_va	= 0x1000,
    .fault_info	= 0x1004,
    .get_fault_info	= exynos_sysmmu_v7_get_fault_info,
    };
    static struct exynos_iommu_domain *to_exynos_domain(struct iommu_domain *dom)
    {
    return container_of(dom, struct exynos_iommu_domain, domain);
    }
#[no_mangle]
unsafe extern "C" fn sysmmu_unblock(data: *mut sysmmu_drvdata) {
    static void sysmmu_unblock(struct sysmmu_drvdata *data)
    {
    writel(CTRL_ENABLE, data.sfrbase + REG_MMU_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn sysmmu_block(data: *mut sysmmu_drvdata) -> bool {
    static bool sysmmu_block(struct sysmmu_drvdata *data)
    {
    let mut i: c_int = 120;
    writel(CTRL_BLOCK, data.sfrbase + REG_MMU_CTRL);
    while ((i > 0) && !(readl(data.sfrbase + REG_MMU_STATUS) & 1))
    --i;
    if (!(readl(data.sfrbase + REG_MMU_STATUS) & 1)) {
    sysmmu_unblock(data);
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn __sysmmu_tlb_invalidate(data: *mut sysmmu_drvdata) {
    static void __sysmmu_tlb_invalidate(struct sysmmu_drvdata *data)
    {
    writel(0x1, SYSMMU_REG(data, flush_all));
    }
    static void __sysmmu_tlb_invalidate_entry(struct sysmmu_drvdata *data,
    sysmmu_iova_t iova, unsigned int num_inv)
    {
    unsigned int i;
    if (MMU_MAJ_VER(data.version) < 5 || num_inv == 1) {
    for (i = 0; i < num_inv; i++) {
    writel((iova & SPAGE_MASK) | 1,
    SYSMMU_REG(data, flush_entry));
    iova += SPAGE_SIZE;
    }
    } else {
    writel(iova & SPAGE_MASK, SYSMMU_REG(data, flush_start));
    writel((iova & SPAGE_MASK) + (num_inv - 1) * SPAGE_SIZE,
    SYSMMU_REG(data, flush_end));
    writel(0x1, SYSMMU_REG(data, flush_range));
    }
    }
#[no_mangle]
unsafe extern "C" fn __sysmmu_set_ptbase(data: *mut sysmmu_drvdata, pgd: phys_addr_t) {
    static void __sysmmu_set_ptbase(struct sysmmu_drvdata *data, phys_addr_t pgd)
    {
    u32 pt_base;
    if (MMU_MAJ_VER(data.version) < 5)
    pt_base = pgd;
    else
    pt_base = pgd >> SPAGE_ORDER;
    writel(pt_base, SYSMMU_REG(data, pt_base));
    __sysmmu_tlb_invalidate(data);
    }
#[no_mangle]
unsafe extern "C" fn __sysmmu_enable_clocks(data: *mut sysmmu_drvdata) {
    static void __sysmmu_enable_clocks(struct sysmmu_drvdata *data)
    {
    BUG_ON(clk_prepare_enable(data.clk_master));
    BUG_ON(clk_prepare_enable(data.clk));
    BUG_ON(clk_prepare_enable(data.pclk));
    BUG_ON(clk_prepare_enable(data.aclk));
    }
#[no_mangle]
unsafe extern "C" fn __sysmmu_disable_clocks(data: *mut sysmmu_drvdata) {
    static void __sysmmu_disable_clocks(struct sysmmu_drvdata *data)
    {
    clk_disable_unprepare(data.aclk);
    clk_disable_unprepare(data.pclk);
    clk_disable_unprepare(data.clk);
    clk_disable_unprepare(data.clk_master);
    }
#[no_mangle]
unsafe extern "C" fn __sysmmu_has_capa1(data: *mut sysmmu_drvdata) -> bool {
    static bool __sysmmu_has_capa1(struct sysmmu_drvdata *data)
    {
    let mut capa0: u32 = readl(data.sfrbase + REG_V7_CAPA0);
    return capa0 & CAPA0_CAPA1_EXIST;
    }
#[no_mangle]
unsafe extern "C" fn __sysmmu_get_vcr(data: *mut sysmmu_drvdata) {
    static void __sysmmu_get_vcr(struct sysmmu_drvdata *data)
    {
    let mut capa1: u32 = readl(data.sfrbase + REG_V7_CAPA1);
    data.has_vcr = capa1 & CAPA1_VCR_ENABLED;
    }
#[no_mangle]
unsafe extern "C" fn __sysmmu_get_version(data: *mut sysmmu_drvdata) {
    static void __sysmmu_get_version(struct sysmmu_drvdata *data)
    {
    u32 ver;
    __sysmmu_enable_clocks(data);
    ver = readl(data.sfrbase + REG_MMU_VERSION);
// controllers on some SoCs don't report proper version
    if (ver == 0x80000001u)
    data.version = MAKE_MMU_VER(1, 0);
    else
    data.version = MMU_RAW_VER(ver);
    dev_dbg(data.sysmmu, "hardware version: %d.%d\n",
    MMU_MAJ_VER(data.version), MMU_MIN_VER(data.version));
    if (MMU_MAJ_VER(data.version) < 5) {
    data.variant = &sysmmu_v1_variant;
    } else if (MMU_MAJ_VER(data.version) < 7) {
    data.variant = &sysmmu_v5_variant;
    } else {
    if (__sysmmu_has_capa1(data))
    __sysmmu_get_vcr(data);
    if (data.has_vcr)
    data.variant = &sysmmu_v7_vm_variant;
    else
    data.variant = &sysmmu_v7_variant;
    }
    __sysmmu_disable_clocks(data);
    }
    static void show_fault_information(struct sysmmu_drvdata *data,
    const struct sysmmu_fault *fault)
    {
    sysmmu_pte_t *ent;
    dev_err(data.sysmmu, "%s: [%s] %s FAULT occurred at %#x\n",
    dev_name(data.master),
    fault.type == IOMMU_FAULT_READ ? "READ" : "WRITE",
    fault.name, fault.addr);
    dev_dbg(data.sysmmu, "Page table base: %pa\n", &data.pgtable);
    ent = section_entry(phys_to_virt(data.pgtable), fault.addr);
    dev_dbg(data.sysmmu, "\tLv1 entry: %#x\n", *ent);
    if (lv1ent_page(ent)) {
    ent = page_entry(ent, fault.addr);
    dev_dbg(data.sysmmu, "\t Lv2 entry: %#x\n", *ent);
    }
    }
#[no_mangle]
unsafe extern "C" fn exynos_sysmmu_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t exynos_sysmmu_irq(int irq, void *dev_id)
    {
    struct sysmmu_drvdata *data = dev_id;
    unsigned int itype;
    struct sysmmu_fault fault;
    let mut ret: c_int = -ENOSYS;
    WARN_ON(!data.active);
    spin_lock(&data.lock);
    clk_enable(data.clk_master);
    itype = __ffs(readl(SYSMMU_REG(data, int_status)));
    ret = data.variant.get_fault_info(data, itype, &fault);
    if (ret) {
    dev_err(data.sysmmu, "Unhandled interrupt bit %u\n", itype);
    goto out;
    }
    show_fault_information(data, &fault);
    if (data.domain) {
    ret = report_iommu_fault(&data.domain.domain, data.master,
    fault.addr, fault.type);
    }
    if (ret)
    panic("Unrecoverable System MMU Fault!");
    out:
    writel(1 << itype, SYSMMU_REG(data, int_clear));
// SysMMU is in blocked state when interrupt occurred
    sysmmu_unblock(data);
    clk_disable(data.clk_master);
    spin_unlock(&data.lock);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn __sysmmu_disable(data: *mut sysmmu_drvdata) {
    static void __sysmmu_disable(struct sysmmu_drvdata *data)
    {
    unsigned long flags;
    clk_enable(data.clk_master);
    spin_lock_irqsave(&data.lock, flags);
    writel(CTRL_DISABLE, data.sfrbase + REG_MMU_CTRL);
    writel(0, data.sfrbase + REG_MMU_CFG);
    data.active = false;
    spin_unlock_irqrestore(&data.lock, flags);
    __sysmmu_disable_clocks(data);
    }
#[no_mangle]
unsafe extern "C" fn __sysmmu_init_config(data: *mut sysmmu_drvdata) {
    static void __sysmmu_init_config(struct sysmmu_drvdata *data)
    {
    unsigned int cfg;
    if (data.version <= MAKE_MMU_VER(3, 1))
    cfg = CFG_LRU | CFG_QOS(15);
#[no_mangle]
pub unsafe extern "C" fn if(MAKE_MMU_VER(3: data->version <=, _arg: 2)) -> else {
    else if (data.version <= MAKE_MMU_VER(3, 2))
    cfg = CFG_LRU | CFG_QOS(15) | CFG_FLPDCACHE | CFG_SYSSEL;
    else
    cfg = CFG_QOS(15) | CFG_FLPDCACHE | CFG_ACGEN;
    cfg |= CFG_EAP; /* enable access protection bits check */
    writel(cfg, data.sfrbase + REG_MMU_CFG);
    }
#[no_mangle]
unsafe extern "C" fn __sysmmu_enable_vid(data: *mut sysmmu_drvdata) {
    static void __sysmmu_enable_vid(struct sysmmu_drvdata *data)
    {
    u32 ctrl;
    if (MMU_MAJ_VER(data.version) < 7 || !data.has_vcr)
    return;
    ctrl = readl(data.sfrbase + REG_V7_CTRL_VM);
    ctrl |= CTRL_VM_ENABLE | CTRL_VM_FAULT_MODE_STALL;
    writel(ctrl, data.sfrbase + REG_V7_CTRL_VM);
    }
#[no_mangle]
unsafe extern "C" fn __sysmmu_enable(data: *mut sysmmu_drvdata) {
    static void __sysmmu_enable(struct sysmmu_drvdata *data)
    {
    unsigned long flags;
    __sysmmu_enable_clocks(data);
    spin_lock_irqsave(&data.lock, flags);
    writel(CTRL_BLOCK, data.sfrbase + REG_MMU_CTRL);
    __sysmmu_init_config(data);
    __sysmmu_set_ptbase(data, data.pgtable);
    __sysmmu_enable_vid(data);
    writel(CTRL_ENABLE, data.sfrbase + REG_MMU_CTRL);
    data.active = true;
    spin_unlock_irqrestore(&data.lock, flags);
//
// SYSMMU driver keeps master's clock enabled only for the short
// time, while accessing the registers. For performing address
// translation during DMA transaction it relies on the client
// driver to enable it.
//
    clk_disable(data.clk_master);
    }
    static void sysmmu_tlb_invalidate_flpdcache(struct sysmmu_drvdata *data,
    sysmmu_iova_t iova)
    {
    unsigned long flags;
    spin_lock_irqsave(&data.lock, flags);
    if (data.active && data.version >= MAKE_MMU_VER(3, 3)) {
    clk_enable(data.clk_master);
    if (sysmmu_block(data)) {
    if (data.version >= MAKE_MMU_VER(5, 0))
    __sysmmu_tlb_invalidate(data);
    else
    __sysmmu_tlb_invalidate_entry(data, iova, 1);
    sysmmu_unblock(data);
    }
    clk_disable(data.clk_master);
    }
    spin_unlock_irqrestore(&data.lock, flags);
    }
    static void sysmmu_tlb_invalidate_entry(struct sysmmu_drvdata *data,
    sysmmu_iova_t iova, size_t size)
    {
    unsigned long flags;
    spin_lock_irqsave(&data.lock, flags);
    if (data.active) {
    let mut num_inv: c_uint = 1;
    clk_enable(data.clk_master);
//
// L2TLB invalidation required
// 4KB page: 1 invalidation
// 64KB page: 16 invalidations
// 1MB page: 64 invalidations
// because it is set-associative TLB
// with 8-way and 64 sets.
// 1MB page can be cached in one of all sets.
// 64KB page can be one of 16 consecutive sets.
//
    if (MMU_MAJ_VER(data.version) == 2)
    num_inv = min_t(unsigned int, size / SPAGE_SIZE, 64);
    if (sysmmu_block(data)) {
    __sysmmu_tlb_invalidate_entry(data, iova, num_inv);
    sysmmu_unblock(data);
    }
    clk_disable(data.clk_master);
    }
    spin_unlock_irqrestore(&data.lock, flags);
    }
    static const struct iommu_ops exynos_iommu_ops;
#[no_mangle]
unsafe extern "C" fn exynos_sysmmu_probe(pdev: *mut platform_device) -> c_int {
    static int exynos_sysmmu_probe(struct platform_device *pdev)
    {
    int irq, ret;
    struct device *dev = &pdev.dev;
    struct sysmmu_drvdata *data;
    struct resource *res;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    data.sfrbase = devm_ioremap_resource(dev, res);
    if (IS_ERR(data.sfrbase))
    return PTR_ERR(data.sfrbase);
    irq = platform_get_irq(pdev, 0);
    if (irq <= 0)
    return irq;
    ret = devm_request_irq(dev, irq, exynos_sysmmu_irq, 0,
    dev_name(dev), data);
    if (ret) {
    dev_err(dev, "Unable to register handler of irq %d\n", irq);
    return ret;
    }
    data.clk = devm_clk_get_optional(dev, "sysmmu");
    if (IS_ERR(data.clk))
    return PTR_ERR(data.clk);
    data.aclk = devm_clk_get_optional(dev, "aclk");
    if (IS_ERR(data.aclk))
    return PTR_ERR(data.aclk);
    data.pclk = devm_clk_get_optional(dev, "pclk");
    if (IS_ERR(data.pclk))
    return PTR_ERR(data.pclk);
    if (!data.clk && (!data.aclk || !data.pclk)) {
    dev_err(dev, "Failed to get device clock(s)!\n");
    return -ENOSYS;
    }
    data.clk_master = devm_clk_get_optional(dev, "master");
    if (IS_ERR(data.clk_master))
    return PTR_ERR(data.clk_master);
    data.sysmmu = dev;
    spin_lock_init(&data.lock);
    __sysmmu_get_version(data);
    ret = iommu_device_sysfs_add(&data.iommu, &pdev.dev, core::ptr::null_mut(),
    dev_name(data.sysmmu));
    if (ret)
    return ret;
    platform_set_drvdata(pdev, data);
    if (PG_ENT_SHIFT < 0) {
    if (MMU_MAJ_VER(data.version) < 5) {
    PG_ENT_SHIFT = SYSMMU_PG_ENT_SHIFT;
    LV1_PROT = SYSMMU_LV1_PROT;
    LV2_PROT = SYSMMU_LV2_PROT;
    } else {
    PG_ENT_SHIFT = SYSMMU_V5_PG_ENT_SHIFT;
    LV1_PROT = SYSMMU_V5_LV1_PROT;
    LV2_PROT = SYSMMU_V5_LV2_PROT;
    }
    }
    if (MMU_MAJ_VER(data.version) >= 5) {
    ret = dma_set_mask(dev, DMA_BIT_MASK(36));
    if (ret) {
    dev_err(dev, "Unable to set DMA mask: %d\n", ret);
    goto err_dma_set_mask;
    }
    }
//
// use the first registered sysmmu device for performing
// dma mapping operations on iommu page tables (cpu cache flush)
//
    if (!dma_dev)
    dma_dev = &pdev.dev;
    pm_runtime_enable(dev);
    ret = iommu_device_register(&data.iommu, &exynos_iommu_ops, dev);
    if (ret)
    goto err_dma_set_mask;
    return 0;
    err_dma_set_mask:
    iommu_device_sysfs_remove(&data.iommu);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn exynos_sysmmu_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused exynos_sysmmu_suspend(struct device *dev)
    {
    struct sysmmu_drvdata *data = dev_get_drvdata(dev);
    struct device *master = data.master;
    if (master) {
    struct exynos_iommu_owner *owner = dev_iommu_priv_get(master);
    mutex_lock(&owner.rpm_lock);
    if (data.domain) {
    dev_dbg(data.sysmmu, "saving state\n");
    __sysmmu_disable(data);
    }
    mutex_unlock(&owner.rpm_lock);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos_sysmmu_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused exynos_sysmmu_resume(struct device *dev)
    {
    struct sysmmu_drvdata *data = dev_get_drvdata(dev);
    struct device *master = data.master;
    if (master) {
    struct exynos_iommu_owner *owner = dev_iommu_priv_get(master);
    mutex_lock(&owner.rpm_lock);
    if (data.domain) {
    dev_dbg(data.sysmmu, "restoring state\n");
    __sysmmu_enable(data);
    }
    mutex_unlock(&owner.rpm_lock);
    }
    return 0;
    }
    static const struct dev_pm_ops sysmmu_pm_ops = {
    SET_RUNTIME_PM_OPS(exynos_sysmmu_suspend, exynos_sysmmu_resume, core::ptr::null_mut())
    SET_SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend,
    pm_runtime_force_resume)
    };
    static const struct of_device_id sysmmu_of_match[] = {
    { .compatible	= "samsung,exynos-sysmmu", },
    { },
    };
    static struct platform_driver exynos_sysmmu_driver __refdata = {
    .probe	= exynos_sysmmu_probe,
    .driver	= {
    .name		= "exynos-sysmmu",
    .of_match_table	= sysmmu_of_match,
    .pm		= &sysmmu_pm_ops,
    .suppress_bind_attrs = true,
    }
    };
#[no_mangle]
pub unsafe extern "C" fn exynos_iommu_set_pte(ent: *mut sysmmu_pte_t, val: sysmmu_pte_t) {
    static inline void exynos_iommu_set_pte(sysmmu_pte_t *ent, sysmmu_pte_t val)
    {
    dma_sync_single_for_cpu(dma_dev, virt_to_phys(ent), sizeof(*ent),
    DMA_TO_DEVICE);
// ent = cpu_to_le32(val);
    dma_sync_single_for_device(dma_dev, virt_to_phys(ent), sizeof(*ent),
    DMA_TO_DEVICE);
    }
    static struct iommu_domain *exynos_iommu_domain_alloc_paging(struct device *dev)
    {
    struct exynos_iommu_domain *domain;
    dma_addr_t handle;
    int i;
// Check if correct PTE offsets are initialized
    BUG_ON(PG_ENT_SHIFT < 0 || !dma_dev);
    domain = kzalloc_obj(*domain);
    if (!domain)
    return core::ptr::null_mut();
    domain.pgtable = iommu_alloc_pages_sz(GFP_KERNEL, SZ_16K);
    if (!domain.pgtable)
    goto err_pgtable;
    domain.lv2entcnt = iommu_alloc_pages_sz(GFP_KERNEL, SZ_8K);
    if (!domain.lv2entcnt)
    goto err_counter;
// Workaround for System MMU v3.3 to prevent caching 1MiB mapping
    for (i = 0; i < NUM_LV1ENTRIES; i++)
    domain.pgtable[i] = ZERO_LV2LINK;
    handle = dma_map_single(dma_dev, domain.pgtable, LV1TABLE_SIZE,
    DMA_TO_DEVICE);
// For mapping page table entries we rely on dma == phys
    BUG_ON(handle != virt_to_phys(domain.pgtable));
    if (dma_mapping_error(dma_dev, handle))
    goto err_lv2ent;
    spin_lock_init(&domain.lock);
    spin_lock_init(&domain.pgtablelock);
    INIT_LIST_HEAD(&domain.clients);
    domain.domain.pgsize_bitmap = SECT_SIZE | LPAGE_SIZE | SPAGE_SIZE;
    domain.domain.geometry.aperture_start = 0;
    domain.domain.geometry.aperture_end   = ~0UL;
    domain.domain.geometry.force_aperture = true;
    return &domain.domain;
    err_lv2ent:
    iommu_free_pages(domain.lv2entcnt);
    err_counter:
    iommu_free_pages(domain.pgtable);
    err_pgtable:
    kfree(domain);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn exynos_iommu_domain_free(iommu_domain: *mut iommu_domain) {
    static void exynos_iommu_domain_free(struct iommu_domain *iommu_domain)
    {
    struct exynos_iommu_domain *domain = to_exynos_domain(iommu_domain);
    struct sysmmu_drvdata *data, *next;
    unsigned long flags;
    int i;
    WARN_ON(!list_empty(&domain.clients));
    spin_lock_irqsave(&domain.lock, flags);
    list_for_each_entry_safe(data, next, &domain.clients, domain_node) {
    spin_lock(&data.lock);
    __sysmmu_disable(data);
    data.pgtable = 0;
    data.domain = core::ptr::null_mut();
    list_del_init(&data.domain_node);
    spin_unlock(&data.lock);
    }
    spin_unlock_irqrestore(&domain.lock, flags);
    dma_unmap_single(dma_dev, virt_to_phys(domain.pgtable), LV1TABLE_SIZE,
    DMA_TO_DEVICE);
    for (i = 0; i < NUM_LV1ENTRIES; i++)
    if (lv1ent_page(domain.pgtable + i)) {
    let mut base: phys_addr_t = lv2table_base(domain.pgtable + i);
    dma_unmap_single(dma_dev, base, LV2TABLE_SIZE,
    DMA_TO_DEVICE);
    kmem_cache_free(lv2table_kmem_cache,
    phys_to_virt(base));
    }
    iommu_free_pages(domain.pgtable);
    iommu_free_pages(domain.lv2entcnt);
    kfree(domain);
    }
    static int exynos_iommu_identity_attach(struct iommu_domain *identity_domain,
    struct device *dev,
    struct iommu_domain *old)
    {
    struct exynos_iommu_owner *owner = dev_iommu_priv_get(dev);
    struct exynos_iommu_domain *domain;
    phys_addr_t pagetable;
    struct sysmmu_drvdata *data, *next;
    unsigned long flags;
    if (owner.domain == identity_domain)
    return 0;
    domain = to_exynos_domain(owner.domain);
    pagetable = virt_to_phys(domain.pgtable);
    mutex_lock(&owner.rpm_lock);
    list_for_each_entry(data, &owner.controllers, owner_node) {
    pm_runtime_get_noresume(data.sysmmu);
    if (pm_runtime_active(data.sysmmu))
    __sysmmu_disable(data);
    pm_runtime_put(data.sysmmu);
    }
    spin_lock_irqsave(&domain.lock, flags);
    list_for_each_entry_safe(data, next, &domain.clients, domain_node) {
    spin_lock(&data.lock);
    data.pgtable = 0;
    data.domain = core::ptr::null_mut();
    list_del_init(&data.domain_node);
    spin_unlock(&data.lock);
    }
    owner.domain = identity_domain;
    spin_unlock_irqrestore(&domain.lock, flags);
    mutex_unlock(&owner.rpm_lock);
    dev_dbg(dev, "%s: Restored IOMMU to IDENTITY from pgtable %pa\n",
    __func__, &pagetable);
    return 0;
    }
    static struct iommu_domain_ops exynos_identity_ops = {
    .attach_dev = exynos_iommu_identity_attach,
    };
    static struct iommu_domain exynos_identity_domain = {
    .type = IOMMU_DOMAIN_IDENTITY,
    .ops = &exynos_identity_ops,
    };
    static int exynos_iommu_attach_device(struct iommu_domain *iommu_domain,
    struct device *dev,
    struct iommu_domain *old)
    {
    struct exynos_iommu_domain *domain = to_exynos_domain(iommu_domain);
    struct exynos_iommu_owner *owner = dev_iommu_priv_get(dev);
    struct sysmmu_drvdata *data;
    let mut pagetable: phys_addr_t = virt_to_phys(domain.pgtable);
    unsigned long flags;
    int err;
    err = exynos_iommu_identity_attach(&exynos_identity_domain, dev, old);
    if (err)
    return err;
    mutex_lock(&owner.rpm_lock);
    spin_lock_irqsave(&domain.lock, flags);
    list_for_each_entry(data, &owner.controllers, owner_node) {
    spin_lock(&data.lock);
    data.pgtable = pagetable;
    data.domain = domain;
    list_add_tail(&data.domain_node, &domain.clients);
    spin_unlock(&data.lock);
    }
    owner.domain = iommu_domain;
    spin_unlock_irqrestore(&domain.lock, flags);
    list_for_each_entry(data, &owner.controllers, owner_node) {
    pm_runtime_get_noresume(data.sysmmu);
    if (pm_runtime_active(data.sysmmu))
    __sysmmu_enable(data);
    pm_runtime_put(data.sysmmu);
    }
    mutex_unlock(&owner.rpm_lock);
    dev_dbg(dev, "%s: Attached IOMMU with pgtable %pa\n", __func__,
    &pagetable);
    return 0;
    }
    static sysmmu_pte_t *alloc_lv2entry(struct exynos_iommu_domain *domain,
    sysmmu_pte_t *sent, sysmmu_iova_t iova, short *pgcounter)
    {
    if (lv1ent_section(sent)) {
    WARN(1, "Trying mapping on %#08x mapped with 1MiB page", iova);
    return ERR_PTR(-EADDRINUSE);
    }
    if (lv1ent_fault(sent)) {
    dma_addr_t handle;
    sysmmu_pte_t *pent;
    let mut need_flush_flpd_cache: bool = lv1ent_zero(sent);
    pent = kmem_cache_zalloc(lv2table_kmem_cache, GFP_ATOMIC);
    BUG_ON((uintptr_t)pent & (LV2TABLE_SIZE - 1));
    if (!pent)
    return ERR_PTR(-ENOMEM);
    exynos_iommu_set_pte(sent, mk_lv1ent_page(virt_to_phys(pent)));
    kmemleak_ignore(pent);
// pgcounter = NUM_LV2ENTRIES;
    handle = dma_map_single(dma_dev, pent, LV2TABLE_SIZE,
    DMA_TO_DEVICE);
    if (dma_mapping_error(dma_dev, handle)) {
    kmem_cache_free(lv2table_kmem_cache, pent);
    return ERR_PTR(-EADDRINUSE);
    }
//
// If pre-fetched SLPD is a faulty SLPD in zero_l2_table,
// FLPD cache may cache the address of zero_l2_table. This
// function replaces the zero_l2_table with new L2 page table
// to write valid mappings.
// Accessing the valid area may cause page fault since FLPD
// cache may still cache zero_l2_table for the valid area
// instead of new L2 page table that has the mapping
// information of the valid area.
// Thus any replacement of zero_l2_table with other valid L2
// page table must involve FLPD cache invalidation for System
// MMU v3.3.
// FLPD cache invalidation is performed with TLB invalidation
// by VPN without blocking. It is safe to invalidate TLB without
// blocking because the target address of TLB invalidation is
// not currently mapped.
//
    if (need_flush_flpd_cache) {
    struct sysmmu_drvdata *data;
    spin_lock(&domain.lock);
    list_for_each_entry(data, &domain.clients, domain_node)
    sysmmu_tlb_invalidate_flpdcache(data, iova);
    spin_unlock(&domain.lock);
    }
    }
    return page_entry(sent, iova);
    }
    static int lv1set_section(struct exynos_iommu_domain *domain,
    sysmmu_pte_t *sent, sysmmu_iova_t iova,
    phys_addr_t paddr, int prot, short *pgcnt)
    {
    if (lv1ent_section(sent)) {
    WARN(1, "Trying mapping on 1MiB@%#08x that is mapped",
    iova);
    return -EADDRINUSE;
    }
    if (lv1ent_page(sent)) {
    if (*pgcnt != NUM_LV2ENTRIES) {
    WARN(1, "Trying mapping on 1MiB@%#08x that is mapped",
    iova);
    return -EADDRINUSE;
    }
    kmem_cache_free(lv2table_kmem_cache, page_entry(sent, 0));
// pgcnt = 0;
    }
    exynos_iommu_set_pte(sent, mk_lv1ent_sect(paddr, prot));
    spin_lock(&domain.lock);
    if (lv1ent_page_zero(sent)) {
    struct sysmmu_drvdata *data;
//
// Flushing FLPD cache in System MMU v3.3 that may cache a FLPD
// entry by speculative prefetch of SLPD which has no mapping.
//
    list_for_each_entry(data, &domain.clients, domain_node)
    sysmmu_tlb_invalidate_flpdcache(data, iova);
    }
    spin_unlock(&domain.lock);
    return 0;
    }
    static int lv2set_page(sysmmu_pte_t *pent, phys_addr_t paddr, size_t size,
    int prot, short *pgcnt)
    {
    if (size == SPAGE_SIZE) {
    if (WARN_ON(!lv2ent_fault(pent)))
    return -EADDRINUSE;
    exynos_iommu_set_pte(pent, mk_lv2ent_spage(paddr, prot));
// pgcnt -= 1;
    } else { /* size == LPAGE_SIZE */
    int i;
    let mut pent_base: dma_addr_t = virt_to_phys(pent);
    dma_sync_single_for_cpu(dma_dev, pent_base,
    sizeof(*pent) * SPAGES_PER_LPAGE,
    DMA_TO_DEVICE);
    for (i = 0; i < SPAGES_PER_LPAGE; i++, pent++) {
    if (WARN_ON(!lv2ent_fault(pent))) {
    if (i > 0)
    memset(pent - i, 0, sizeof(*pent) * i);
    return -EADDRINUSE;
    }
// pent = mk_lv2ent_lpage(paddr, prot);
    }
    dma_sync_single_for_device(dma_dev, pent_base,
    sizeof(*pent) * SPAGES_PER_LPAGE,
    DMA_TO_DEVICE);
// pgcnt -= SPAGES_PER_LPAGE;
    }
    return 0;
    }
//
// *CAUTION* to the I/O virtual memory managers that support exynos-iommu:
//
// System MMU v3.x has advanced logic to improve address translation
// performance with caching more page table entries by a page table walk.
// However, the logic has a bug that while caching faulty page table entries,
// System MMU reports page fault if the cached fault entry is hit even though
// the fault entry is updated to a valid entry after the entry is cached.
// To prevent caching faulty page table entries which may be updated to valid
// entries later, the virtual memory manager should care about the workaround
// for the problem. The following describes the workaround.
//
// Any two consecutive I/O virtual address regions must have a hole of 128KiB
// at maximum to prevent misbehavior of System MMU 3.x (workaround for h/w bug).
//
// Precisely, any start address of I/O virtual region must be aligned with
// the following sizes for System MMU v3.1 and v3.2.
// System MMU v3.1: 128KiB
// System MMU v3.2: 256KiB
//
// Because System MMU v3.3 caches page table entries more aggressively, it needs
// more workarounds.
// - Any two consecutive I/O virtual regions must have a hole of size larger
// than or equal to 128KiB.
// - Start address of an I/O virtual region must be aligned by 128KiB.
//
    static int exynos_iommu_map(struct iommu_domain *iommu_domain,
    unsigned long l_iova, phys_addr_t paddr, size_t size,
    size_t count, int prot, gfp_t gfp, size_t *mapped)
    {
    struct exynos_iommu_domain *domain = to_exynos_domain(iommu_domain);
    sysmmu_pte_t *entry;
    let mut iova: sysmmu_iova_t = (sysmmu_iova_t)l_iova;
    unsigned long flags;
    let mut ret: c_int = -ENOMEM;
    BUG_ON(domain.pgtable == core::ptr::null_mut());
    prot &= SYSMMU_SUPPORTED_PROT_BITS;
    spin_lock_irqsave(&domain.pgtablelock, flags);
    entry = section_entry(domain.pgtable, iova);
    if (size == SECT_SIZE) {
    ret = lv1set_section(domain, entry, iova, paddr, prot,
    &domain.lv2entcnt[lv1ent_offset(iova)]);
    } else {
    sysmmu_pte_t *pent;
    pent = alloc_lv2entry(domain, entry, iova,
    &domain.lv2entcnt[lv1ent_offset(iova)]);
    if (IS_ERR(pent))
    ret = PTR_ERR(pent);
    else
    ret = lv2set_page(pent, paddr, size, prot,
    &domain.lv2entcnt[lv1ent_offset(iova)]);
    }
    if (ret)
    pr_err("%s: Failed(%d) to map %#zx bytes @ %#x\n",
    __func__, ret, size, iova);
    else
// mapped = size;
    spin_unlock_irqrestore(&domain.pgtablelock, flags);
    return ret;
    }
    static void exynos_iommu_tlb_invalidate_entry(struct exynos_iommu_domain *domain,
    sysmmu_iova_t iova, size_t size)
    {
    struct sysmmu_drvdata *data;
    unsigned long flags;
    spin_lock_irqsave(&domain.lock, flags);
    list_for_each_entry(data, &domain.clients, domain_node)
    sysmmu_tlb_invalidate_entry(data, iova, size);
    spin_unlock_irqrestore(&domain.lock, flags);
    }
    static size_t exynos_iommu_unmap(struct iommu_domain *iommu_domain,
    unsigned long l_iova, size_t size, size_t count,
    struct iommu_iotlb_gather *gather)
    {
    struct exynos_iommu_domain *domain = to_exynos_domain(iommu_domain);
    let mut iova: sysmmu_iova_t = (sysmmu_iova_t)l_iova;
    sysmmu_pte_t *ent;
    size_t err_pgsize;
    unsigned long flags;
    BUG_ON(domain.pgtable == core::ptr::null_mut());
    spin_lock_irqsave(&domain.pgtablelock, flags);
    ent = section_entry(domain.pgtable, iova);
    if (lv1ent_section(ent)) {
    if (WARN_ON(size < SECT_SIZE)) {
    err_pgsize = SECT_SIZE;
    goto err;
    }
// workaround for h/w bug in System MMU v3.3
    exynos_iommu_set_pte(ent, ZERO_LV2LINK);
    size = SECT_SIZE;
    goto done;
    }
    if (unlikely(lv1ent_fault(ent))) {
    if (size > SECT_SIZE)
    size = SECT_SIZE;
    goto done;
    }
// lv1ent_page(sent) == true here
    ent = page_entry(ent, iova);
    if (unlikely(lv2ent_fault(ent))) {
    size = SPAGE_SIZE;
    goto done;
    }
    if (lv2ent_small(ent)) {
    exynos_iommu_set_pte(ent, 0);
    size = SPAGE_SIZE;
    domain.lv2entcnt[lv1ent_offset(iova)] += 1;
    goto done;
    }
// lv1ent_large(ent) == true here
    if (WARN_ON(size < LPAGE_SIZE)) {
    err_pgsize = LPAGE_SIZE;
    goto err;
    }
    dma_sync_single_for_cpu(dma_dev, virt_to_phys(ent),
    sizeof(*ent) * SPAGES_PER_LPAGE,
    DMA_TO_DEVICE);
    memset(ent, 0, sizeof(*ent) * SPAGES_PER_LPAGE);
    dma_sync_single_for_device(dma_dev, virt_to_phys(ent),
    sizeof(*ent) * SPAGES_PER_LPAGE,
    DMA_TO_DEVICE);
    size = LPAGE_SIZE;
    domain.lv2entcnt[lv1ent_offset(iova)] += SPAGES_PER_LPAGE;
    done:
    spin_unlock_irqrestore(&domain.pgtablelock, flags);
    exynos_iommu_tlb_invalidate_entry(domain, iova, size);
    return size;
    err:
    spin_unlock_irqrestore(&domain.pgtablelock, flags);
    pr_err("%s: Failed: size(%#zx) @ %#x is smaller than page size %#zx\n",
    __func__, size, iova, err_pgsize);
    return 0;
    }
    static phys_addr_t exynos_iommu_iova_to_phys(struct iommu_domain *iommu_domain,
    dma_addr_t iova)
    {
    struct exynos_iommu_domain *domain = to_exynos_domain(iommu_domain);
    sysmmu_pte_t *entry;
    unsigned long flags;
    let mut phys: phys_addr_t = 0;
    spin_lock_irqsave(&domain.pgtablelock, flags);
    entry = section_entry(domain.pgtable, iova);
    if (lv1ent_section(entry)) {
    phys = section_phys(entry) + section_offs(iova);
    } else if (lv1ent_page(entry)) {
    entry = page_entry(entry, iova);
    if (lv2ent_large(entry))
    phys = lpage_phys(entry) + lpage_offs(iova);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: lv2ent_small(entry)) -> else {
    else if (lv2ent_small(entry))
    phys = spage_phys(entry) + spage_offs(iova);
    }
    spin_unlock_irqrestore(&domain.pgtablelock, flags);
    return phys;
    }
    static struct iommu_device *exynos_iommu_probe_device(struct device *dev)
    {
    struct exynos_iommu_owner *owner = dev_iommu_priv_get(dev);
    struct sysmmu_drvdata *data;
    if (!has_sysmmu(dev))
    return ERR_PTR(-ENODEV);
    list_for_each_entry(data, &owner.controllers, owner_node) {
//
// SYSMMU will be runtime activated via device link
// (dependency) to its master device, so there are no
// direct calls to pm_runtime_get/put in this driver.
//
    data.link = device_link_add(dev, data.sysmmu,
    DL_FLAG_STATELESS |
    DL_FLAG_PM_RUNTIME);
    }
// There is always at least one entry, see exynos_iommu_of_xlate()
    data = list_first_entry(&owner.controllers,
    struct sysmmu_drvdata, owner_node);
    return &data.iommu;
    }
#[no_mangle]
unsafe extern "C" fn exynos_iommu_release_device(dev: *mut device) {
    static void exynos_iommu_release_device(struct device *dev)
    {
    struct exynos_iommu_owner *owner = dev_iommu_priv_get(dev);
    struct sysmmu_drvdata *data;
    list_for_each_entry(data, &owner.controllers, owner_node)
    device_link_del(data.link);
    }
    static int exynos_iommu_of_xlate(struct device *dev,
    const struct of_phandle_args *spec)
    {
    struct platform_device *sysmmu = of_find_device_by_node(spec.np);
    struct exynos_iommu_owner *owner = dev_iommu_priv_get(dev);
    struct sysmmu_drvdata *data, *entry;
    if (!sysmmu)
    return -ENODEV;
    data = platform_get_drvdata(sysmmu);
    put_device(&sysmmu.dev);
    if (!data)
    return -ENODEV;
    if (!owner) {
    owner = kzalloc_obj(*owner);
    if (!owner)
    return -ENOMEM;
    INIT_LIST_HEAD(&owner.controllers);
    mutex_init(&owner.rpm_lock);
    owner.domain = &exynos_identity_domain;
    dev_iommu_priv_set(dev, owner);
    }
    list_for_each_entry(entry, &owner.controllers, owner_node)
    if (entry == data)
    return 0;
    list_add_tail(&data.owner_node, &owner.controllers);
    data.master = dev;
    return 0;
    }
    static const struct iommu_ops exynos_iommu_ops = {
    .identity_domain = &exynos_identity_domain,
    .release_domain = &exynos_identity_domain,
    .domain_alloc_paging = exynos_iommu_domain_alloc_paging,
    .device_group = generic_device_group,
    .probe_device = exynos_iommu_probe_device,
    .release_device = exynos_iommu_release_device,
    .get_resv_regions = iommu_dma_get_resv_regions,
    .of_xlate = exynos_iommu_of_xlate,
    .default_domain_ops = &(const struct iommu_domain_ops) {
    .attach_dev	= exynos_iommu_attach_device,
    .map_pages	= exynos_iommu_map,
    .unmap_pages	= exynos_iommu_unmap,
    .iova_to_phys	= exynos_iommu_iova_to_phys,
    .free		= exynos_iommu_domain_free,
    }
    };
#[no_mangle]
unsafe extern "C" fn exynos_iommu_init() -> int __init {
    static int __init exynos_iommu_init(void)
    {
    struct device_node *np;
    int ret;
    np = of_find_matching_node(core::ptr::null_mut(), sysmmu_of_match);
    if (!np)
    return 0;
    of_node_put(np);
    lv2table_kmem_cache = kmem_cache_create("exynos-iommu-lv2table",
    LV2TABLE_SIZE, LV2TABLE_SIZE, 0, core::ptr::null_mut());
    if (!lv2table_kmem_cache) {
    pr_err("%s: Failed to create kmem cache\n", __func__);
    return -ENOMEM;
    }
    zero_lv2_table = kmem_cache_zalloc(lv2table_kmem_cache, GFP_KERNEL);
    if (zero_lv2_table == core::ptr::null_mut()) {
    pr_err("%s: Failed to allocate zero level2 page table\n",
    __func__);
    ret = -ENOMEM;
    goto err_zero_lv2;
    }
    ret = platform_driver_register(&exynos_sysmmu_driver);
    if (ret) {
    pr_err("%s: Failed to register driver\n", __func__);
    goto err_reg_driver;
    }
    return 0;
    err_reg_driver:
    kmem_cache_free(lv2table_kmem_cache, zero_lv2_table);
    err_zero_lv2:
    kmem_cache_destroy(lv2table_kmem_cache);
    return ret;
    }
    core_initcall(exynos_iommu_init);
