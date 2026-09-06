//! Automatically rewritten from C to Rust
//! Source: drivers/iommu/ipmmu-vmsa.c
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
// IOMMU API for Renesas VMSA-compatible IPMMU
// Author: Laurent Pinchart <laurent.pinchart@ideasonboard.com>
//
// Copyright (C) 2014-2020 Renesas Electronics Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipmmu_features {
    pub use_ns_alias_offset: bool,
    pub has_cache_leaf_nodes: bool,
    pub number_of_contexts: c_uint,
    pub num_utlbs: c_uint,
    pub setup_imbuscr: bool,
    pub twobit_imttbcr_sl0: bool,
    pub reserved_context: bool,
    pub cache_snoop: bool,
    pub ctx_offset_base: c_uint,
    pub ctx_offset_stride: c_uint,
    pub utlb_offset_base: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipmmu_vmsa_device {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub iommu: iommu_device,
    pub root: *mut ipmmu_vmsa_device,
    pub features: *const ipmmu_features,
    pub num_ctx: c_uint,
    pub /: *mut *mut spinlock_t lock; / Protects ctx and domains[],
    pub IPMMU_CTX_MAX): DECLARE_BITMAP(ctx,,
    pub domains: [*mut ipmmu_vmsa_domain; IPMMU_CTX_MAX],
    pub utlb_ctx: [i8; IPMMU_UTLB_MAX],
    pub mapping: *mut dma_iommu_mapping,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipmmu_vmsa_domain {
    pub mmu: *mut ipmmu_vmsa_device,
    pub io_domain: iommu_domain,
    pub cfg: io_pgtable_cfg,
    pub iop: *mut io_pgtable_ops,
    pub context_id: c_uint,
    pub /: *mut *mut mutex mutex; / Protects mappings,
}

    static struct ipmmu_vmsa_domain *to_vmsa_domain(struct iommu_domain *dom)
    {
    return container_of(dom, struct ipmmu_vmsa_domain, io_domain);
    }
    static struct ipmmu_vmsa_device *to_ipmmu(struct device *dev)
    {
    return dev_iommu_priv_get(dev);
    }

// -----------------------------------------------------------------------------
// Registers Definition
//
pub const IM_NS_ALIAS_OFFSET: c_uint = 0x800;
// MMU "context" registers
pub const IMCTR: c_uint = 0x0000		/* R-Car Gen2/3 */;

pub const IMTTBCR: c_uint = 0x0008		/* R-Car Gen2/3 */;

pub const IMBUSCR: c_uint = 0x000c		/* R-Car Gen2 only */;

pub const IMTTLBR0: c_uint = 0x0010		/* R-Car Gen2/3 */;
pub const IMTTUBR0: c_uint = 0x0014		/* R-Car Gen2/3 */;
pub const IMSTR: c_uint = 0x0020		/* R-Car Gen2/3 */;

pub const IMMAIR0: c_uint = 0x0028		/* R-Car Gen2/3 */;
pub const IMELAR: c_uint = 0x0030		/* R-Car Gen2/3, IMEAR on R-Car Gen2 */;
pub const IMEUAR: c_uint = 0x0034		/* R-Car Gen3 only */;
// uTLB registers

// -----------------------------------------------------------------------------
// Root device handling
//
    static struct platform_driver ipmmu_driver;
#[no_mangle]
unsafe extern "C" fn ipmmu_is_root(mmu: *mut ipmmu_vmsa_device) -> bool {
    static bool ipmmu_is_root(struct ipmmu_vmsa_device *mmu)
    {
    return mmu.root == mmu;
    }
#[no_mangle]
unsafe extern "C" fn __ipmmu_check_device(dev: *mut device, data: *mut c_void) -> c_int {
    static int __ipmmu_check_device(struct device *dev, void *data)
    {
    struct ipmmu_vmsa_device *mmu = dev_get_drvdata(dev);
    struct ipmmu_vmsa_device **rootp = data;
    if (ipmmu_is_root(mmu))
// rootp = mmu;
    return 0;
    }
    static struct ipmmu_vmsa_device *ipmmu_find_root(void)
    {
    struct ipmmu_vmsa_device *root = core::ptr::null_mut();
    return driver_for_each_device(&ipmmu_driver.driver, core::ptr::null_mut(), &root,
    __ipmmu_check_device) == 0 ? root : core::ptr::null_mut();
    }
// -----------------------------------------------------------------------------
// Read/Write Access
//
#[no_mangle]
unsafe extern "C" fn ipmmu_read(mmu: *mut ipmmu_vmsa_device, offset: c_uint) -> u32 {
    static u32 ipmmu_read(struct ipmmu_vmsa_device *mmu, unsigned int offset)
    {
    return ioread32(mmu.base + offset);
    }
    static void ipmmu_write(struct ipmmu_vmsa_device *mmu, unsigned int offset,
    u32 data)
    {
    iowrite32(data, mmu.base + offset);
    }
    static unsigned int ipmmu_ctx_reg(struct ipmmu_vmsa_device *mmu,
    unsigned int context_id, unsigned int reg)
    {
    let mut base: c_uint = mmu.features.ctx_offset_base;
    if (context_id > 7)
    base += 0x800 - 8 * 0x40;
    return base + context_id * mmu.features.ctx_offset_stride + reg;
    }
    static u32 ipmmu_ctx_read(struct ipmmu_vmsa_device *mmu,
    unsigned int context_id, unsigned int reg)
    {
    return ipmmu_read(mmu, ipmmu_ctx_reg(mmu, context_id, reg));
    }
    static void ipmmu_ctx_write(struct ipmmu_vmsa_device *mmu,
    unsigned int context_id, unsigned int reg, u32 data)
    {
    ipmmu_write(mmu, ipmmu_ctx_reg(mmu, context_id, reg), data);
    }
    static u32 ipmmu_ctx_read_root(struct ipmmu_vmsa_domain *domain,
    unsigned int reg)
    {
    return ipmmu_ctx_read(domain.mmu.root, domain.context_id, reg);
    }
    static void ipmmu_ctx_write_root(struct ipmmu_vmsa_domain *domain,
    unsigned int reg, u32 data)
    {
    ipmmu_ctx_write(domain.mmu.root, domain.context_id, reg, data);
    }
    static void ipmmu_ctx_write_all(struct ipmmu_vmsa_domain *domain,
    unsigned int reg, u32 data)
    {
    if (domain.mmu != domain.mmu.root)
    ipmmu_ctx_write(domain.mmu, domain.context_id, reg, data);
    ipmmu_ctx_write(domain.mmu.root, domain.context_id, reg, data);
    }
#[no_mangle]
unsafe extern "C" fn ipmmu_utlb_reg(mmu: *mut ipmmu_vmsa_device, reg: c_uint) -> u32 {
    static u32 ipmmu_utlb_reg(struct ipmmu_vmsa_device *mmu, unsigned int reg)
    {
    return mmu.features.utlb_offset_base + reg;
    }
    static void ipmmu_imuasid_write(struct ipmmu_vmsa_device *mmu,
    unsigned int utlb, u32 data)
    {
    ipmmu_write(mmu, ipmmu_utlb_reg(mmu, IMUASID(utlb)), data);
    }
    static void ipmmu_imuctr_write(struct ipmmu_vmsa_device *mmu,
    unsigned int utlb, u32 data)
    {
    ipmmu_write(mmu, ipmmu_utlb_reg(mmu, IMUCTR(utlb)), data);
    }
// -----------------------------------------------------------------------------
// TLB and microTLB Management
//
// Wait for any pending TLB invalidations to complete
#[no_mangle]
unsafe extern "C" fn ipmmu_tlb_sync(domain: *mut ipmmu_vmsa_domain) {
    static void ipmmu_tlb_sync(struct ipmmu_vmsa_domain *domain)
    {
    u32 val;
    if (read_poll_timeout_atomic(ipmmu_ctx_read_root, val,
    !(val & IMCTR_FLUSH), 1, TLB_LOOP_TIMEOUT,
    false, domain, IMCTR))
    dev_err_ratelimited(domain.mmu.dev,
    "TLB sync timed out -- MMU may be deadlocked\n");
    }
#[no_mangle]
unsafe extern "C" fn ipmmu_tlb_invalidate(domain: *mut ipmmu_vmsa_domain) {
    static void ipmmu_tlb_invalidate(struct ipmmu_vmsa_domain *domain)
    {
    u32 reg;
    reg = ipmmu_ctx_read_root(domain, IMCTR);
    reg |= IMCTR_FLUSH;
    ipmmu_ctx_write_all(domain, IMCTR, reg);
    ipmmu_tlb_sync(domain);
    }
//
// Enable MMU translation for the microTLB.
//
    static void ipmmu_utlb_enable(struct ipmmu_vmsa_domain *domain,
    unsigned int utlb)
    {
    struct ipmmu_vmsa_device *mmu = domain.mmu;
//
// TODO: Reference-count the microTLB as several bus masters can be
// connected to the same microTLB.
//
// TODO: What should we set the ASID to ?
    ipmmu_imuasid_write(mmu, utlb, 0);
// TODO: Do we need to flush the microTLB ?
    ipmmu_imuctr_write(mmu, utlb, IMUCTR_TTSEL_MMU(domain.context_id) |
    IMUCTR_FLUSH | IMUCTR_MMUEN);
    mmu.utlb_ctx[utlb] = domain.context_id;
    }
//
// Disable MMU translation for the microTLB.
//
    static void ipmmu_utlb_disable(struct ipmmu_vmsa_domain *domain,
    unsigned int utlb)
    {
    struct ipmmu_vmsa_device *mmu = domain.mmu;
    ipmmu_imuctr_write(mmu, utlb, 0);
    mmu.utlb_ctx[utlb] = IPMMU_CTX_INVALID;
    }
#[no_mangle]
unsafe extern "C" fn ipmmu_tlb_flush_all(cookie: *mut c_void) {
    static void ipmmu_tlb_flush_all(void *cookie)
    {
    struct ipmmu_vmsa_domain *domain = cookie;
    ipmmu_tlb_invalidate(domain);
    }
    static void ipmmu_tlb_flush(unsigned long iova, size_t size,
    size_t granule, void *cookie)
    {
    ipmmu_tlb_flush_all(cookie);
    }
    static const struct iommu_flush_ops ipmmu_flush_ops = {
    .tlb_flush_all = ipmmu_tlb_flush_all,
    .tlb_flush_walk = ipmmu_tlb_flush,
    };
// -----------------------------------------------------------------------------
// Domain/Context Management
//
    static int ipmmu_domain_allocate_context(struct ipmmu_vmsa_device *mmu,
    struct ipmmu_vmsa_domain *domain)
    {
    unsigned long flags;
    int ret;
    spin_lock_irqsave(&mmu.lock, flags);
    ret = find_first_zero_bit(mmu.ctx, mmu.num_ctx);
    if (ret != mmu.num_ctx) {
    mmu.domains[ret] = domain;
    set_bit(ret, mmu.ctx);
    } else
    ret = -EBUSY;
    spin_unlock_irqrestore(&mmu.lock, flags);
    return ret;
    }
    static void ipmmu_domain_free_context(struct ipmmu_vmsa_device *mmu,
    unsigned int context_id)
    {
    unsigned long flags;
    spin_lock_irqsave(&mmu.lock, flags);
    clear_bit(context_id, mmu.ctx);
    mmu.domains[context_id] = core::ptr::null_mut();
    spin_unlock_irqrestore(&mmu.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ipmmu_domain_setup_context(domain: *mut ipmmu_vmsa_domain) {
    static void ipmmu_domain_setup_context(struct ipmmu_vmsa_domain *domain)
    {
    u64 ttbr;
    u32 tmp;
// TTBR0
    ttbr = domain.cfg.arm_lpae_s1_cfg.ttbr;
    ipmmu_ctx_write_root(domain, IMTTLBR0, ttbr);
    ipmmu_ctx_write_root(domain, IMTTUBR0, ttbr >> 32);
//
// TTBCR
// We use long descriptors and allocate the whole 32-bit VA space to
// TTBR0.
//
    if (domain.mmu.features.twobit_imttbcr_sl0)
    tmp = IMTTBCR_SL0_TWOBIT_LVL_1;
    else
    tmp = IMTTBCR_SL0_LVL_1;
    if (domain.mmu.features.cache_snoop)
    tmp |= IMTTBCR_SH0_INNER_SHAREABLE | IMTTBCR_ORGN0_WB_WA |
    IMTTBCR_IRGN0_WB_WA;
    ipmmu_ctx_write_root(domain, IMTTBCR, IMTTBCR_EAE | tmp);
// MAIR0
    ipmmu_ctx_write_root(domain, IMMAIR0,
    domain.cfg.arm_lpae_s1_cfg.mair);
// IMBUSCR
    if (domain.mmu.features.setup_imbuscr)
    ipmmu_ctx_write_root(domain, IMBUSCR,
    ipmmu_ctx_read_root(domain, IMBUSCR) &
    ~(IMBUSCR_DVM | IMBUSCR_BUSSEL_MASK));
//
// IMSTR
// Clear all interrupt flags.
//
    ipmmu_ctx_write_root(domain, IMSTR, ipmmu_ctx_read_root(domain, IMSTR));
//
// IMCTR
// Enable the MMU and interrupt generation. The long-descriptor
// translation table format doesn't use TEX remapping. Don't enable AF
// software management as we have no use for it. Flush the TLB as
// required when modifying the context registers.
//
    ipmmu_ctx_write_all(domain, IMCTR,
    IMCTR_INTEN | IMCTR_FLUSH | IMCTR_MMUEN);
    }
#[no_mangle]
unsafe extern "C" fn ipmmu_domain_init_context(domain: *mut ipmmu_vmsa_domain) -> c_int {
    static int ipmmu_domain_init_context(struct ipmmu_vmsa_domain *domain)
    {
    int ret;
//
// Allocate the page table operations.
//
// VMSA states in section B3.6.3 "Control of Secure or Non-secure memory
// access, Long-descriptor format" that the NStable bit being set in a
// table descriptor will result in the NStable and NS bits of all child
// entries being ignored and considered as being set. The IPMMU seems
// not to comply with this, as it generates a secure access page fault
// if any of the NStable and NS bits isn't set when running in
// non-secure mode.
//
    domain.cfg.quirks = IO_PGTABLE_QUIRK_ARM_NS;
    domain.cfg.pgsize_bitmap = domain.io_domain.pgsize_bitmap;
    domain.cfg.ias = 32;
    domain.cfg.oas = 40;
    domain.cfg.tlb = &ipmmu_flush_ops;
    domain.io_domain.geometry.aperture_end = DMA_BIT_MASK(32);
    domain.io_domain.geometry.force_aperture = true;
//
// TODO: Add support for coherent walk through CCI with DVM and remove
// cache handling. For now, delegate it to the io-pgtable code.
//
    domain.cfg.coherent_walk = false;
    domain.cfg.iommu_dev = domain.mmu.root.dev;
//
// Find an unused context.
//
    ret = ipmmu_domain_allocate_context(domain.mmu.root, domain);
    if (ret < 0)
    return ret;
    domain.context_id = ret;
    domain.iop = alloc_io_pgtable_ops(ARM_32_LPAE_S1, &domain.cfg,
    domain);
    if (!domain.iop) {
    ipmmu_domain_free_context(domain.mmu.root,
    domain.context_id);
    return -EINVAL;
    }
    ipmmu_domain_setup_context(domain);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ipmmu_domain_destroy_context(domain: *mut ipmmu_vmsa_domain) {
    static void ipmmu_domain_destroy_context(struct ipmmu_vmsa_domain *domain)
    {
    if (!domain.mmu)
    return;
//
// Disable the context. Flush the TLB as required when modifying the
// context registers.
//
// TODO: Is TLB flush really needed ?
//
    ipmmu_ctx_write_all(domain, IMCTR, IMCTR_FLUSH);
    ipmmu_tlb_sync(domain);
    ipmmu_domain_free_context(domain.mmu.root, domain.context_id);
    }
// -----------------------------------------------------------------------------
// Fault Handling
//
#[no_mangle]
unsafe extern "C" fn ipmmu_domain_irq(domain: *mut ipmmu_vmsa_domain) -> irqreturn_t {
    static irqreturn_t ipmmu_domain_irq(struct ipmmu_vmsa_domain *domain)
    {
    let mut err_mask: u32 = IMSTR_MHIT | IMSTR_ABORT | IMSTR_PF | IMSTR_TF;
    struct ipmmu_vmsa_device *mmu = domain.mmu;
    unsigned long iova;
    u32 status;
    status = ipmmu_ctx_read_root(domain, IMSTR);
    if (!(status & err_mask))
    return IRQ_NONE;
    iova = ipmmu_ctx_read_root(domain, IMELAR);
    if (IS_ENABLED(CONFIG_64BIT))
    iova |= (u64)ipmmu_ctx_read_root(domain, IMEUAR) << 32;
//
// Clear the error status flags. Unlike traditional interrupt flag
// registers that must be cleared by writing 1, this status register
// seems to require 0. The error address register must be read before,
// otherwise its value will be 0.
//
    ipmmu_ctx_write_root(domain, IMSTR, 0);
// Log fatal errors.
    if (status & IMSTR_MHIT)
    dev_err_ratelimited(mmu.dev, "Multiple TLB hits @0x%lx\n",
    iova);
    if (status & IMSTR_ABORT)
    dev_err_ratelimited(mmu.dev, "Page Table Walk Abort @0x%lx\n",
    iova);
    if (!(status & (IMSTR_PF | IMSTR_TF)))
    return IRQ_NONE;
//
// Try to handle page faults and translation faults.
//
// TODO: We need to look up the faulty device based on the I/O VA. Use
// the IOMMU device for now.
//
    if (!report_iommu_fault(&domain.io_domain, mmu.dev, iova, 0))
    return IRQ_HANDLED;
    dev_err_ratelimited(mmu.dev,
    "Unhandled fault: status 0x%08x iova 0x%lx\n",
    status, iova);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ipmmu_irq(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t ipmmu_irq(int irq, void *dev)
    {
    struct ipmmu_vmsa_device *mmu = dev;
    let mut status: irqreturn_t = IRQ_NONE;
    unsigned int i;
    unsigned long flags;
    spin_lock_irqsave(&mmu.lock, flags);
//
// Check interrupts for all active contexts.
//
    for (i = 0; i < mmu.num_ctx; i++) {
    if (!mmu.domains[i])
    continue;
    if (ipmmu_domain_irq(mmu.domains[i]) == IRQ_HANDLED)
    status = IRQ_HANDLED;
    }
    spin_unlock_irqrestore(&mmu.lock, flags);
    return status;
    }
// -----------------------------------------------------------------------------
// IOMMU Operations
//
    static struct iommu_domain *ipmmu_domain_alloc_paging(struct device *dev)
    {
    struct ipmmu_vmsa_domain *domain;
    domain = kzalloc_obj(*domain);
    if (!domain)
    return core::ptr::null_mut();
    mutex_init(&domain.mutex);
    domain.io_domain.pgsize_bitmap = SZ_1G | SZ_2M | SZ_4K;
    return &domain.io_domain;
    }
#[no_mangle]
unsafe extern "C" fn ipmmu_domain_free(io_domain: *mut iommu_domain) {
    static void ipmmu_domain_free(struct iommu_domain *io_domain)
    {
    struct ipmmu_vmsa_domain *domain = to_vmsa_domain(io_domain);
//
// Free the domain resources. We assume that all devices have already
// been detached.
//
    ipmmu_domain_destroy_context(domain);
    free_io_pgtable_ops(domain.iop);
    kfree(domain);
    }
    static int ipmmu_attach_device(struct iommu_domain *io_domain,
    struct device *dev, struct iommu_domain *old)
    {
    struct iommu_fwspec *fwspec = dev_iommu_fwspec_get(dev);
    struct ipmmu_vmsa_device *mmu = to_ipmmu(dev);
    struct ipmmu_vmsa_domain *domain = to_vmsa_domain(io_domain);
    unsigned int i;
    let mut ret: c_int = 0;
    if (!mmu) {
    dev_err(dev, "Cannot attach to IPMMU\n");
    return -ENXIO;
    }
    mutex_lock(&domain.mutex);
    if (!domain.mmu) {
// The domain hasn't been used yet, initialize it.
    domain.mmu = mmu;
    ret = ipmmu_domain_init_context(domain);
    if (ret < 0) {
    dev_err(dev, "Unable to initialize IPMMU context\n");
    domain.mmu = core::ptr::null_mut();
    } else {
    dev_info(dev, "Using IPMMU context %u\n",
    domain.context_id);
    }
    } else if (domain.mmu != mmu) {
//
// Something is wrong, we can't attach two devices using
// different IOMMUs to the same domain.
//
    ret = -EINVAL;
    } else
    dev_info(dev, "Reusing IPMMU context %u\n", domain.context_id);
    mutex_unlock(&domain.mutex);
    if (ret < 0)
    return ret;
    for (i = 0; i < fwspec.num_ids; ++i)
    ipmmu_utlb_enable(domain, fwspec.ids[i]);
    return 0;
    }
    static int ipmmu_iommu_identity_attach(struct iommu_domain *identity_domain,
    struct device *dev,
    struct iommu_domain *old)
    {
    struct iommu_fwspec *fwspec = dev_iommu_fwspec_get(dev);
    struct ipmmu_vmsa_domain *domain;
    unsigned int i;
    if (old == identity_domain || !old)
    return 0;
    domain = to_vmsa_domain(old);
    for (i = 0; i < fwspec.num_ids; ++i)
    ipmmu_utlb_disable(domain, fwspec.ids[i]);
//
// TODO: Optimize by disabling the context when no device is attached.
//
    return 0;
    }
    static struct iommu_domain_ops ipmmu_iommu_identity_ops = {
    .attach_dev = ipmmu_iommu_identity_attach,
    };
    static struct iommu_domain ipmmu_iommu_identity_domain = {
    .type = IOMMU_DOMAIN_IDENTITY,
    .ops = &ipmmu_iommu_identity_ops,
    };
    static int ipmmu_map(struct iommu_domain *io_domain, unsigned long iova,
    phys_addr_t paddr, size_t pgsize, size_t pgcount,
    int prot, gfp_t gfp, size_t *mapped)
    {
    struct ipmmu_vmsa_domain *domain = to_vmsa_domain(io_domain);
    return domain.iop.map_pages(domain.iop, iova, paddr, pgsize, pgcount,
    prot, gfp, mapped);
    }
    static size_t ipmmu_unmap(struct iommu_domain *io_domain, unsigned long iova,
    size_t pgsize, size_t pgcount,
    struct iommu_iotlb_gather *gather)
    {
    struct ipmmu_vmsa_domain *domain = to_vmsa_domain(io_domain);
    return domain.iop.unmap_pages(domain.iop, iova, pgsize, pgcount, gather);
    }
#[no_mangle]
unsafe extern "C" fn ipmmu_flush_iotlb_all(io_domain: *mut iommu_domain) {
    static void ipmmu_flush_iotlb_all(struct iommu_domain *io_domain)
    {
    struct ipmmu_vmsa_domain *domain = to_vmsa_domain(io_domain);
    if (domain.mmu)
    ipmmu_tlb_flush_all(domain);
    }
    static void ipmmu_iotlb_sync(struct iommu_domain *io_domain,
    struct iommu_iotlb_gather *gather)
    {
    ipmmu_flush_iotlb_all(io_domain);
    }
    static phys_addr_t ipmmu_iova_to_phys(struct iommu_domain *io_domain,
    dma_addr_t iova)
    {
    struct ipmmu_vmsa_domain *domain = to_vmsa_domain(io_domain);
// TODO: Is locking needed ?
    return domain.iop.iova_to_phys(domain.iop, iova);
    }
    static int ipmmu_init_platform_device(struct device *dev,
    const struct of_phandle_args *args)
    {
    struct platform_device *ipmmu_pdev;
    ipmmu_pdev = of_find_device_by_node(args.np);
    if (!ipmmu_pdev)
    return -ENODEV;
    dev_iommu_priv_set(dev, platform_get_drvdata(ipmmu_pdev));
    put_device(&ipmmu_pdev.dev);
    return 0;
    }
    static const struct soc_device_attribute soc_needs_opt_in[] = {
    { .family = "R-Car Gen3", },
    { .family = "R-Car Gen4", },
    { .family = "RZ/G2", },
    { /* sentinel */ }
    };
    static const struct soc_device_attribute soc_denylist[] = {
    { .soc_id = "r8a774a1", },
    { .soc_id = "r8a7795", .revision = "ES2.*" },
    { .soc_id = "r8a7796", },
    { /* sentinel */ }
    };
    static const char * const devices_allowlist[] = {
    "ee100000.mmc",
    "ee120000.mmc",
    "ee140000.mmc",
    "ee160000.mmc"
    };
#[no_mangle]
unsafe extern "C" fn ipmmu_device_is_allowed(dev: *mut device) -> bool {
    static bool ipmmu_device_is_allowed(struct device *dev)
    {
    unsigned int i;
//
// R-Car Gen3/4 and RZ/G2 use the allow list to opt-in devices.
// For Other SoCs, this returns true anyway.
//
    if (!soc_device_match(soc_needs_opt_in))
    return true;
// Check whether this SoC can use the IPMMU correctly or not
    if (soc_device_match(soc_denylist))
    return false;
// Check whether this device is a PCI device
    if (dev_is_pci(dev))
    return true;
// Check whether this device can work with the IPMMU
    for (i = 0; i < ARRAY_SIZE(devices_allowlist); i++) {
    if (!strcmp(dev_name(dev), devices_allowlist[i]))
    return true;
    }
// Otherwise, do not allow use of IPMMU
    return false;
    }
    static int ipmmu_of_xlate(struct device *dev,
    const struct of_phandle_args *spec)
    {
    if (!ipmmu_device_is_allowed(dev))
    return -ENODEV;
    iommu_fwspec_add_ids(dev, spec.args, 1);
// Initialize once - xlate() will call multiple times
    if (to_ipmmu(dev))
    return 0;
    return ipmmu_init_platform_device(dev, spec);
    }
#[no_mangle]
unsafe extern "C" fn ipmmu_init_arm_mapping(dev: *mut device) -> c_int {
    static int ipmmu_init_arm_mapping(struct device *dev)
    {
    struct ipmmu_vmsa_device *mmu = to_ipmmu(dev);
    int ret;
//
// Create the ARM mapping, used by the ARM DMA mapping core to allocate
// VAs. This will allocate a corresponding IOMMU domain.
//
// TODO:
// - Create one mapping per context (TLB).
// - Make the mapping size configurable ? We currently use a 2GB mapping
// at a 1GB offset to ensure that NULL VAs will fault.
//
    if (!mmu.mapping) {
    struct dma_iommu_mapping *mapping;
    mapping = arm_iommu_create_mapping(dev, SZ_1G, SZ_2G);
    if (IS_ERR(mapping)) {
    dev_err(mmu.dev, "failed to create ARM IOMMU mapping\n");
    ret = PTR_ERR(mapping);
    goto error;
    }
    mmu.mapping = mapping;
    }
// Attach the ARM VA mapping to the device.
    ret = arm_iommu_attach_device(dev, mmu.mapping);
    if (ret < 0) {
    dev_err(dev, "Failed to attach device to VA mapping\n");
    goto error;
    }
    return 0;
    error:
    if (mmu.mapping)
    arm_iommu_release_mapping(mmu.mapping);
    return ret;
    }
    static struct iommu_device *ipmmu_probe_device(struct device *dev)
    {
    struct ipmmu_vmsa_device *mmu = to_ipmmu(dev);
//
// Only let through devices that have been verified in xlate()
//
    if (!mmu)
    return ERR_PTR(-ENODEV);
    return &mmu.iommu;
    }
#[no_mangle]
unsafe extern "C" fn ipmmu_probe_finalize(dev: *mut device) {
    static void ipmmu_probe_finalize(struct device *dev)
    {
    let mut ret: c_int = 0;
    if (IS_ENABLED(CONFIG_ARM) && !IS_ENABLED(CONFIG_IOMMU_DMA))
    ret = ipmmu_init_arm_mapping(dev);
    if (ret)
    dev_err(dev, "Can't create IOMMU mapping - DMA-OPS will not work\n");
    }
#[no_mangle]
unsafe extern "C" fn ipmmu_release_device(dev: *mut device) {
    static void ipmmu_release_device(struct device *dev)
    {
    struct iommu_fwspec *fwspec = dev_iommu_fwspec_get(dev);
    struct ipmmu_vmsa_device *mmu = to_ipmmu(dev);
    unsigned int i;
    for (i = 0; i < fwspec.num_ids; ++i) {
    let mut utlb: c_uint = fwspec.ids[i];
    ipmmu_imuctr_write(mmu, utlb, 0);
    mmu.utlb_ctx[utlb] = IPMMU_CTX_INVALID;
    }
    arm_iommu_release_mapping(mmu.mapping);
    }
    static const struct iommu_ops ipmmu_ops = {
    .identity_domain = &ipmmu_iommu_identity_domain,
    .domain_alloc_paging = ipmmu_domain_alloc_paging,
    .probe_device = ipmmu_probe_device,
    .release_device = ipmmu_release_device,
    .probe_finalize = ipmmu_probe_finalize,
//
// FIXME: The device grouping is a fixed property of the hardware's
// ability to isolate and control DMA, it should not depend on kconfig.
//
    .device_group = IS_ENABLED(CONFIG_ARM) && !IS_ENABLED(CONFIG_IOMMU_DMA)
    ? generic_device_group : generic_single_device_group,
    .of_xlate = ipmmu_of_xlate,
    .default_domain_ops = &(const struct iommu_domain_ops) {
    .attach_dev	= ipmmu_attach_device,
    .map_pages	= ipmmu_map,
    .unmap_pages	= ipmmu_unmap,
    .flush_iotlb_all = ipmmu_flush_iotlb_all,
    .iotlb_sync	= ipmmu_iotlb_sync,
    .iova_to_phys	= ipmmu_iova_to_phys,
    .free		= ipmmu_domain_free,
    }
    };
// -----------------------------------------------------------------------------
// Probe/remove and init
//
#[no_mangle]
unsafe extern "C" fn ipmmu_device_reset(mmu: *mut ipmmu_vmsa_device) {
    static void ipmmu_device_reset(struct ipmmu_vmsa_device *mmu)
    {
    unsigned int i;
// Disable all contexts.
    for (i = 0; i < mmu.num_ctx; ++i)
    ipmmu_ctx_write(mmu, i, IMCTR, 0);
    }
    static const struct ipmmu_features ipmmu_features_default = {
    .use_ns_alias_offset = true,
    .has_cache_leaf_nodes = false,
    .number_of_contexts = 1, /* software only tested with one context */
    .num_utlbs = 32,
    .setup_imbuscr = true,
    .twobit_imttbcr_sl0 = false,
    .reserved_context = false,
    .cache_snoop = true,
    .ctx_offset_base = 0,
    .ctx_offset_stride = 0x40,
    .utlb_offset_base = 0,
    };
    static const struct ipmmu_features ipmmu_features_rcar_gen3 = {
    .use_ns_alias_offset = false,
    .has_cache_leaf_nodes = true,
    .number_of_contexts = 8,
    .num_utlbs = 48,
    .setup_imbuscr = false,
    .twobit_imttbcr_sl0 = true,
    .reserved_context = true,
    .cache_snoop = false,
    .ctx_offset_base = 0,
    .ctx_offset_stride = 0x40,
    .utlb_offset_base = 0,
    };
    static const struct ipmmu_features ipmmu_features_rcar_gen4 = {
    .use_ns_alias_offset = false,
    .has_cache_leaf_nodes = true,
    .number_of_contexts = 16,
    .num_utlbs = 64,
    .setup_imbuscr = false,
    .twobit_imttbcr_sl0 = true,
    .reserved_context = true,
    .cache_snoop = false,
    .ctx_offset_base = 0x10000,
    .ctx_offset_stride = 0x1040,
    .utlb_offset_base = 0x3000,
    };
    static const struct of_device_id ipmmu_of_ids[] = {
    {
    .compatible = "renesas,ipmmu-vmsa",
    .data = &ipmmu_features_default,
    }, {
    .compatible = "renesas,ipmmu-r8a774a1",
    .data = &ipmmu_features_rcar_gen3,
    }, {
    .compatible = "renesas,ipmmu-r8a774b1",
    .data = &ipmmu_features_rcar_gen3,
    }, {
    .compatible = "renesas,ipmmu-r8a774c0",
    .data = &ipmmu_features_rcar_gen3,
    }, {
    .compatible = "renesas,ipmmu-r8a774e1",
    .data = &ipmmu_features_rcar_gen3,
    }, {
    .compatible = "renesas,ipmmu-r8a7795",
    .data = &ipmmu_features_rcar_gen3,
    }, {
    .compatible = "renesas,ipmmu-r8a7796",
    .data = &ipmmu_features_rcar_gen3,
    }, {
    .compatible = "renesas,ipmmu-r8a77961",
    .data = &ipmmu_features_rcar_gen3,
    }, {
    .compatible = "renesas,ipmmu-r8a77965",
    .data = &ipmmu_features_rcar_gen3,
    }, {
    .compatible = "renesas,ipmmu-r8a77970",
    .data = &ipmmu_features_rcar_gen3,
    }, {
    .compatible = "renesas,ipmmu-r8a77980",
    .data = &ipmmu_features_rcar_gen3,
    }, {
    .compatible = "renesas,ipmmu-r8a77990",
    .data = &ipmmu_features_rcar_gen3,
    }, {
    .compatible = "renesas,ipmmu-r8a77995",
    .data = &ipmmu_features_rcar_gen3,
    }, {
    .compatible = "renesas,ipmmu-r8a779a0",
    .data = &ipmmu_features_rcar_gen4,
    }, {
    .compatible = "renesas,rcar-gen4-ipmmu-vmsa",
    .data = &ipmmu_features_rcar_gen4,
    }, {
// Terminator
    },
    };
#[no_mangle]
unsafe extern "C" fn ipmmu_probe(pdev: *mut platform_device) -> c_int {
    static int ipmmu_probe(struct platform_device *pdev)
    {
    struct ipmmu_vmsa_device *mmu;
    int irq;
    int ret;
    mmu = devm_kzalloc(&pdev.dev, sizeof(*mmu), GFP_KERNEL);
    if (!mmu) {
    dev_err(&pdev.dev, "cannot allocate device data\n");
    return -ENOMEM;
    }
    mmu.dev = &pdev.dev;
    spin_lock_init(&mmu.lock);
    bitmap_zero(mmu.ctx, IPMMU_CTX_MAX);
    mmu.features = of_device_get_match_data(&pdev.dev);
    memset(mmu.utlb_ctx, IPMMU_CTX_INVALID, mmu.features.num_utlbs);
    ret = dma_set_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(40));
    if (ret)
    return ret;
// Map I/O memory and request IRQ.
    mmu.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mmu.base))
    return PTR_ERR(mmu.base);
//
// The IPMMU has two register banks, for secure and non-secure modes.
// The bank mapped at the beginning of the IPMMU address space
// corresponds to the running mode of the CPU. When running in secure
// mode the non-secure register bank is also available at an offset.
//
// Secure mode operation isn't clearly documented and is thus currently
// not implemented in the driver. Furthermore, preliminary tests of
// non-secure operation with the main register bank were not successful.
// Offset the registers base unconditionally to point to the non-secure
// alias space for now.
//
    if (mmu.features.use_ns_alias_offset)
    mmu.base += IM_NS_ALIAS_OFFSET;
    mmu.num_ctx = min(IPMMU_CTX_MAX, mmu.features.number_of_contexts);
//
// Determine if this IPMMU instance is a root device by checking for
// the lack of has_cache_leaf_nodes flag or renesas,ipmmu-main property.
//
    if (!mmu.features.has_cache_leaf_nodes ||
    !of_property_present(pdev.dev.of_node, "renesas,ipmmu-main"))
    mmu.root = mmu;
    else
    mmu.root = ipmmu_find_root();
//
// Wait until the root device has been registered for sure.
//
    if (!mmu.root)
    return -EPROBE_DEFER;
// Root devices have mandatory IRQs
    if (ipmmu_is_root(mmu)) {
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = devm_request_irq(&pdev.dev, irq, ipmmu_irq, 0,
    dev_name(&pdev.dev), mmu);
    if (ret < 0) {
    dev_err(&pdev.dev, "failed to request IRQ %d\n", irq);
    return ret;
    }
    ipmmu_device_reset(mmu);
    if (mmu.features.reserved_context) {
    dev_info(&pdev.dev, "IPMMU context 0 is reserved\n");
    set_bit(0, mmu.ctx);
    }
    }
    platform_set_drvdata(pdev, mmu);
//
// Register the IPMMU to the IOMMU subsystem in the following cases:
// - R-Car Gen2 IPMMU (all devices registered)
// - R-Car Gen3 IPMMU (leaf devices only - skip root IPMMU-MM device)
//
    if (mmu.features.has_cache_leaf_nodes && ipmmu_is_root(mmu))
    return 0;
    ret = iommu_device_sysfs_add(&mmu.iommu, &pdev.dev, core::ptr::null_mut(), "%s",
    dev_name(&pdev.dev));
    if (ret)
    return ret;
    ret = iommu_device_register(&mmu.iommu, &ipmmu_ops, &pdev.dev);
    if (ret)
    iommu_device_sysfs_remove(&mmu.iommu);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ipmmu_remove(pdev: *mut platform_device) {
    static void ipmmu_remove(struct platform_device *pdev)
    {
    struct ipmmu_vmsa_device *mmu = platform_get_drvdata(pdev);
    iommu_device_sysfs_remove(&mmu.iommu);
    iommu_device_unregister(&mmu.iommu);
    arm_iommu_release_mapping(mmu.mapping);
    ipmmu_device_reset(mmu);
    }
#[no_mangle]
unsafe extern "C" fn ipmmu_resume_noirq(dev: *mut device) -> c_int {
    static int ipmmu_resume_noirq(struct device *dev)
    {
    struct ipmmu_vmsa_device *mmu = dev_get_drvdata(dev);
    unsigned int i;
// Reset root MMU and restore contexts
    if (ipmmu_is_root(mmu)) {
    ipmmu_device_reset(mmu);
    for (i = 0; i < mmu.num_ctx; i++) {
    if (!mmu.domains[i])
    continue;
    ipmmu_domain_setup_context(mmu.domains[i]);
    }
    }
// Re-enable active micro-TLBs
    for (i = 0; i < mmu.features.num_utlbs; i++) {
    if (mmu.utlb_ctx[i] == IPMMU_CTX_INVALID)
    continue;
    ipmmu_utlb_enable(mmu.root.domains[mmu.utlb_ctx[i]], i);
    }
    return 0;
    }
    static const struct dev_pm_ops ipmmu_pm  = {
    NOIRQ_SYSTEM_SLEEP_PM_OPS(core::ptr::null_mut(), ipmmu_resume_noirq)
    };
    static struct platform_driver ipmmu_driver = {
    .driver = {
    .name = "ipmmu-vmsa",
    .of_match_table = ipmmu_of_ids,
    .pm = pm_sleep_ptr(&ipmmu_pm),
    },
    .probe = ipmmu_probe,
    .remove = ipmmu_remove,
    };
    builtin_platform_driver(ipmmu_driver);
