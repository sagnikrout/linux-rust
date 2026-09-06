//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/pcie-iproc-msi.c
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
// Copyright (C) 2015 Broadcom Corporation
//

pub const IPROC_MSI_INTR_EN_SHIFT: c_int = 11;

pub const IPROC_MSI_INT_N_EVENT_SHIFT: c_int = 1;

pub const IPROC_MSI_EQ_EN_SHIFT: c_int = 0;

pub const IPROC_MSI_EQ_MASK: c_uint = 0x3f;
// Max number of GIC interrupts
pub const NR_HW_IRQS: c_int = 6;
// Number of entries in each event queue
pub const EQ_LEN: c_int = 64;
// Size of each event queue memory region

// Size of each MSI address region

    enum iproc_msi_reg {
    IPROC_MSI_EQ_PAGE = 0,
    IPROC_MSI_EQ_PAGE_UPPER,
    IPROC_MSI_PAGE,
    IPROC_MSI_PAGE_UPPER,
    IPROC_MSI_CTRL,
    IPROC_MSI_EQ_HEAD,
    IPROC_MSI_EQ_TAIL,
    IPROC_MSI_INTS_EN,
    IPROC_MSI_REG_SIZE,
    };
    struct iproc_msi;
//
// struct iproc_msi_grp - iProc MSI group
//
// One MSI group is allocated per GIC interrupt, serviced by one iProc MSI
// event queue.
//
// @msi: pointer to iProc MSI data
// @gic_irq: GIC interrupt
// @eq: Event queue number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_msi_grp {
    pub msi: *mut iproc_msi,
    pub gic_irq: c_int,
    pub eq: c_uint,
}

//
// struct iproc_msi - iProc event queue based MSI
//
// Only meant to be used on platforms without MSI support integrated into the
// GIC.
//
// @pcie: pointer to iProc PCIe data
// @reg_offsets: MSI register offsets
// @grps: MSI groups
// @nr_irqs: number of total interrupts connected to GIC
// @nr_cpus: number of toal CPUs
// @has_inten_reg: indicates the MSI interrupt enable register needs to be
// set explicitly (required for some legacy platforms)
// @bitmap: MSI vector bitmap
// @bitmap_lock: lock to protect access to the MSI bitmap
// @nr_msi_vecs: total number of MSI vectors
// @inner_domain: inner IRQ domain
// @nr_eq_region: required number of 4K aligned memory region for MSI event
// queues
// @nr_msi_region: required number of 4K aligned address region for MSI posted
// writes
// @eq_cpu: pointer to allocated memory region for MSI event queues
// @eq_dma: DMA address of MSI event queues
// @msi_addr: MSI address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_msi {
    pub pcie: *mut iproc_pcie,
    pub (*reg_offsets)[IPROC_MSI_REG_SIZE]: *const u16,
    pub grps: *mut iproc_msi_grp,
    pub nr_irqs: c_int,
    pub nr_cpus: c_int,
    pub has_inten_reg: bool,
    pub bitmap: *mut c_ulong,
    pub bitmap_lock: mutex,
    pub nr_msi_vecs: c_uint,
    pub inner_domain: *mut irq_domain,
    pub nr_eq_region: c_uint,
    pub nr_msi_region: c_uint,
    pub eq_cpu: *mut c_void,
    pub eq_dma: dma_addr_t,
    pub msi_addr: phys_addr_t,
}

    static const u16 iproc_msi_reg_paxb[NR_HW_IRQS][IPROC_MSI_REG_SIZE] = {
    { 0x200, 0x2c0, 0x204, 0x2c4, 0x210, 0x250, 0x254, 0x208 },
    { 0x200, 0x2c0, 0x204, 0x2c4, 0x214, 0x258, 0x25c, 0x208 },
    { 0x200, 0x2c0, 0x204, 0x2c4, 0x218, 0x260, 0x264, 0x208 },
    { 0x200, 0x2c0, 0x204, 0x2c4, 0x21c, 0x268, 0x26c, 0x208 },
    { 0x200, 0x2c0, 0x204, 0x2c4, 0x220, 0x270, 0x274, 0x208 },
    { 0x200, 0x2c0, 0x204, 0x2c4, 0x224, 0x278, 0x27c, 0x208 },
    };
    static const u16 iproc_msi_reg_paxc[NR_HW_IRQS][IPROC_MSI_REG_SIZE] = {
    { 0xc00, 0xc04, 0xc08, 0xc0c, 0xc40, 0xc50, 0xc60 },
    { 0xc10, 0xc14, 0xc18, 0xc1c, 0xc44, 0xc54, 0xc64 },
    { 0xc20, 0xc24, 0xc28, 0xc2c, 0xc48, 0xc58, 0xc68 },
    { 0xc30, 0xc34, 0xc38, 0xc3c, 0xc4c, 0xc5c, 0xc6c },
    };
    static inline u32 iproc_msi_read_reg(struct iproc_msi *msi,
    enum iproc_msi_reg reg,
    unsigned int eq)
    {
    struct iproc_pcie *pcie = msi.pcie;
    return readl_relaxed(pcie.base + msi.reg_offsets[eq][reg]);
    }
    static inline void iproc_msi_write_reg(struct iproc_msi *msi,
    enum iproc_msi_reg reg,
    int eq, u32 val)
    {
    struct iproc_pcie *pcie = msi.pcie;
    writel_relaxed(val, pcie.base + msi.reg_offsets[eq][reg]);
    }
#[no_mangle]
pub unsafe extern "C" fn hwirq_to_group(msi: *mut iproc_msi, hwirq: c_ulong) -> u32 {
    static inline u32 hwirq_to_group(struct iproc_msi *msi, unsigned long hwirq)
    {
    return (hwirq % msi.nr_irqs);
    }
    static inline unsigned int iproc_msi_addr_offset(struct iproc_msi *msi,
    unsigned long hwirq)
    {
    if (msi.nr_msi_region > 1)
    return hwirq_to_group(msi, hwirq) * MSI_MEM_REGION_SIZE;
    else
    return hwirq_to_group(msi, hwirq) * sizeof(u32);
    }
#[no_mangle]
pub unsafe extern "C" fn iproc_msi_eq_offset(msi: *mut iproc_msi, eq: u32) -> c_uint {
    static inline unsigned int iproc_msi_eq_offset(struct iproc_msi *msi, u32 eq)
    {
    if (msi.nr_eq_region > 1)
    return eq * EQ_MEM_REGION_SIZE;
    else
    return eq * EQ_LEN * sizeof(u32);
    }

    MSI_FLAG_USE_DEF_CHIP_OPS)

    MSI_FLAG_PCI_MSIX)
    static struct msi_parent_ops iproc_msi_parent_ops = {
    .required_flags		= IPROC_MSI_FLAGS_REQUIRED,
    .supported_flags	= IPROC_MSI_FLAGS_SUPPORTED,
    .bus_select_token	= DOMAIN_BUS_PCI_MSI,
    .prefix			= "iProc-",
    .init_dev_msi_info	= msi_lib_init_dev_msi_info,
    };
//
// In iProc PCIe core, each MSI group is serviced by a GIC interrupt and a
// dedicated event queue.  Each MSI group can support up to 64 MSI vectors.
//
// The number of MSI groups varies between different iProc SoCs.  The total
// number of CPU cores also varies.  To support MSI IRQ affinity, we
// distribute GIC interrupts across all available CPUs.  MSI vector is moved
// from one GIC interrupt to another to steer to the target CPU.
//
// Assuming:
// - the number of MSI groups is M
// - the number of CPU cores is N
// - M is always a multiple of N
//
// Total number of raw MSI vectors = M * 64
// Total number of supported MSI vectors = (M * 64) / N
//
#[no_mangle]
pub unsafe extern "C" fn hwirq_to_cpu(msi: *mut iproc_msi, hwirq: c_ulong) -> c_int {
    static inline int hwirq_to_cpu(struct iproc_msi *msi, unsigned long hwirq)
    {
    return (hwirq % msi.nr_cpus);
    }
    static inline unsigned long hwirq_to_canonical_hwirq(struct iproc_msi *msi,
    unsigned long hwirq)
    {
    return (hwirq - hwirq_to_cpu(msi, hwirq));
    }
    static int iproc_msi_irq_set_affinity(struct irq_data *data,
    const struct cpumask *mask, bool force)
    {
    struct iproc_msi *msi = irq_data_get_irq_chip_data(data);
    let mut target_cpu: c_int = cpumask_first(mask);
    int curr_cpu;
    int ret;
    curr_cpu = hwirq_to_cpu(msi, data.hwirq);
    if (curr_cpu == target_cpu)
    ret = IRQ_SET_MASK_OK_DONE;
    else {
// steer MSI to the target CPU
    data.hwirq = hwirq_to_canonical_hwirq(msi, data.hwirq) + target_cpu;
    ret = IRQ_SET_MASK_OK;
    }
    irq_data_update_effective_affinity(data, cpumask_of(target_cpu));
    return ret;
    }
    static void iproc_msi_irq_compose_msi_msg(struct irq_data *data,
    struct msi_msg *msg)
    {
    struct iproc_msi *msi = irq_data_get_irq_chip_data(data);
    dma_addr_t addr;
    addr = msi.msi_addr + iproc_msi_addr_offset(msi, data.hwirq);
    msg.address_lo = lower_32_bits(addr);
    msg.address_hi = upper_32_bits(addr);
    msg.data = data.hwirq << 5;
    }
    static struct irq_chip iproc_msi_bottom_irq_chip = {
    .name = "MSI",
    .irq_set_affinity = iproc_msi_irq_set_affinity,
    .irq_compose_msi_msg = iproc_msi_irq_compose_msi_msg,
    };
    static int iproc_msi_irq_domain_alloc(struct irq_domain *domain,
    unsigned int virq, unsigned int nr_irqs,
    void *args)
    {
    struct iproc_msi *msi = domain.host_data;
    int hwirq, i;
    if (msi.nr_cpus > 1 && nr_irqs > 1)
    return -EINVAL;
    mutex_lock(&msi.bitmap_lock);
//
// Allocate 'nr_irqs' multiplied by 'nr_cpus' number of MSI vectors
// each time
//
    hwirq = bitmap_find_free_region(msi.bitmap, msi.nr_msi_vecs,
    order_base_2(msi.nr_cpus * nr_irqs));
    mutex_unlock(&msi.bitmap_lock);
    if (hwirq < 0)
    return -ENOSPC;
    for (i = 0; i < nr_irqs; i++) {
    irq_domain_set_info(domain, virq + i, hwirq + i,
    &iproc_msi_bottom_irq_chip,
    domain.host_data, handle_simple_irq,
    core::ptr::null_mut(), core::ptr::null_mut());
    }
    return 0;
    }
    static void iproc_msi_irq_domain_free(struct irq_domain *domain,
    unsigned int virq, unsigned int nr_irqs)
    {
    struct irq_data *data = irq_domain_get_irq_data(domain, virq);
    struct iproc_msi *msi = irq_data_get_irq_chip_data(data);
    unsigned int hwirq;
    mutex_lock(&msi.bitmap_lock);
    hwirq = hwirq_to_canonical_hwirq(msi, data.hwirq);
    bitmap_release_region(msi.bitmap, hwirq,
    order_base_2(msi.nr_cpus * nr_irqs));
    mutex_unlock(&msi.bitmap_lock);
    irq_domain_free_irqs_parent(domain, virq, nr_irqs);
    }
    static const struct irq_domain_ops msi_domain_ops = {
    .alloc = iproc_msi_irq_domain_alloc,
    .free = iproc_msi_irq_domain_free,
    };
#[no_mangle]
pub unsafe extern "C" fn decode_msi_hwirq(msi: *mut iproc_msi, eq: u32, head: u32) -> u32 {
    static inline u32 decode_msi_hwirq(struct iproc_msi *msi, u32 eq, u32 head)
    {
    u32 __iomem *msg;
    u32 hwirq;
    unsigned int offs;
    offs = iproc_msi_eq_offset(msi, eq) + head * sizeof(u32);
    msg = (u32 __iomem *)(msi.eq_cpu + offs);
    hwirq = readl(msg);
    hwirq = (hwirq >> 5) + (hwirq & 0x1f);
//
// Since we have multiple hwirq mapped to a single MSI vector,
// now we need to derive the hwirq at CPU0.  It can then be used to
// mapped back to virq.
//
    return hwirq_to_canonical_hwirq(msi, hwirq);
    }
#[no_mangle]
unsafe extern "C" fn iproc_msi_handler(desc: *mut irq_desc) {
    static void iproc_msi_handler(struct irq_desc *desc)
    {
    struct irq_chip *chip = irq_desc_get_chip(desc);
    struct iproc_msi_grp *grp;
    struct iproc_msi *msi;
    u32 eq, head, tail, nr_events;
    unsigned long hwirq;
    chained_irq_enter(chip, desc);
    grp = irq_desc_get_handler_data(desc);
    msi = grp.msi;
    eq = grp.eq;
//
// iProc MSI event queue is tracked by head and tail pointers.  Head
// pointer indicates the next entry (MSI data) to be consumed by SW in
// the queue and needs to be updated by SW.  iProc MSI core uses the
// tail pointer as the next data insertion point.
//
// Entries between head and tail pointers contain valid MSI data.  MSI
// data is guaranteed to be in the event queue memory before the tail
// pointer is updated by the iProc MSI core.
//
    head = iproc_msi_read_reg(msi, IPROC_MSI_EQ_HEAD,
    eq) & IPROC_MSI_EQ_MASK;
    do {
    tail = iproc_msi_read_reg(msi, IPROC_MSI_EQ_TAIL,
    eq) & IPROC_MSI_EQ_MASK;
//
// Figure out total number of events (MSI data) to be
// processed.
//
    nr_events = (tail < head) ?
    (EQ_LEN - (head - tail)) : (tail - head);
    if (!nr_events)
    break;
// process all outstanding events
    while (nr_events--) {
    hwirq = decode_msi_hwirq(msi, eq, head);
    generic_handle_domain_irq(msi.inner_domain, hwirq);
    head++;
    head %= EQ_LEN;
    }
//
// Now all outstanding events have been processed.  Update the
// head pointer.
//
    iproc_msi_write_reg(msi, IPROC_MSI_EQ_HEAD, eq, head);
//
// Now go read the tail pointer again to see if there are new
// outstanding events that came in during the above window.
//
    } while (true);
    chained_irq_exit(chip, desc);
    }
#[no_mangle]
unsafe extern "C" fn iproc_msi_enable(msi: *mut iproc_msi) {
    static void iproc_msi_enable(struct iproc_msi *msi)
    {
    int i, eq;
    u32 val;
// Program memory region for each event queue
    for (i = 0; i < msi.nr_eq_region; i++) {
    let mut addr: dma_addr_t = msi.eq_dma + (i * EQ_MEM_REGION_SIZE);
    iproc_msi_write_reg(msi, IPROC_MSI_EQ_PAGE, i,
    lower_32_bits(addr));
    iproc_msi_write_reg(msi, IPROC_MSI_EQ_PAGE_UPPER, i,
    upper_32_bits(addr));
    }
// Program address region for MSI posted writes
    for (i = 0; i < msi.nr_msi_region; i++) {
    let mut addr: phys_addr_t = msi.msi_addr + (i * MSI_MEM_REGION_SIZE);
    iproc_msi_write_reg(msi, IPROC_MSI_PAGE, i,
    lower_32_bits(addr));
    iproc_msi_write_reg(msi, IPROC_MSI_PAGE_UPPER, i,
    upper_32_bits(addr));
    }
    for (eq = 0; eq < msi.nr_irqs; eq++) {
// Enable MSI event queue
    val = IPROC_MSI_INTR_EN | IPROC_MSI_INT_N_EVENT |
    IPROC_MSI_EQ_EN;
    iproc_msi_write_reg(msi, IPROC_MSI_CTRL, eq, val);
//
// Some legacy platforms require the MSI interrupt enable
// register to be set explicitly.
//
    if (msi.has_inten_reg) {
    val = iproc_msi_read_reg(msi, IPROC_MSI_INTS_EN, eq);
    val |= BIT(eq);
    iproc_msi_write_reg(msi, IPROC_MSI_INTS_EN, eq, val);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn iproc_msi_disable(msi: *mut iproc_msi) {
    static void iproc_msi_disable(struct iproc_msi *msi)
    {
    u32 eq, val;
    for (eq = 0; eq < msi.nr_irqs; eq++) {
    if (msi.has_inten_reg) {
    val = iproc_msi_read_reg(msi, IPROC_MSI_INTS_EN, eq);
    val &= ~BIT(eq);
    iproc_msi_write_reg(msi, IPROC_MSI_INTS_EN, eq, val);
    }
    val = iproc_msi_read_reg(msi, IPROC_MSI_CTRL, eq);
    val &= ~(IPROC_MSI_INTR_EN | IPROC_MSI_INT_N_EVENT |
    IPROC_MSI_EQ_EN);
    iproc_msi_write_reg(msi, IPROC_MSI_CTRL, eq, val);
    }
    }
    static int iproc_msi_alloc_domains(struct device_node *node,
    struct iproc_msi *msi)
    {
    struct irq_domain_info info = {
    .fwnode		= of_fwnode_handle(node),
    .ops		= &msi_domain_ops,
    .host_data	= msi,
    .size		= msi.nr_msi_vecs,
    };
    msi.inner_domain = msi_create_parent_irq_domain(&info, &iproc_msi_parent_ops);
    if (!msi.inner_domain)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iproc_msi_free_domains(msi: *mut iproc_msi) {
    static void iproc_msi_free_domains(struct iproc_msi *msi)
    {
    if (msi.inner_domain)
    irq_domain_remove(msi.inner_domain);
    }
#[no_mangle]
unsafe extern "C" fn iproc_msi_irq_free(msi: *mut iproc_msi, cpu: c_uint) {
    static void iproc_msi_irq_free(struct iproc_msi *msi, unsigned int cpu)
    {
    int i;
    for (i = cpu; i < msi.nr_irqs; i += msi.nr_cpus) {
    irq_set_chained_handler_and_data(msi.grps[i].gic_irq,
    core::ptr::null_mut(), core::ptr::null_mut());
    }
    }
#[no_mangle]
unsafe extern "C" fn iproc_msi_irq_setup(msi: *mut iproc_msi, cpu: c_uint) -> c_int {
    static int iproc_msi_irq_setup(struct iproc_msi *msi, unsigned int cpu)
    {
    int i, ret;
    cpumask_var_t mask;
    struct iproc_pcie *pcie = msi.pcie;
    for (i = cpu; i < msi.nr_irqs; i += msi.nr_cpus) {
    irq_set_chained_handler_and_data(msi.grps[i].gic_irq,
    iproc_msi_handler,
    &msi.grps[i]);
// Dedicate GIC interrupt to each CPU core
    if (alloc_cpumask_var(&mask, GFP_KERNEL)) {
    cpumask_clear(mask);
    cpumask_set_cpu(cpu, mask);
    ret = irq_set_affinity(msi.grps[i].gic_irq, mask);
    if (ret)
    dev_err(pcie.dev,
    "failed to set affinity for IRQ%d\n",
    msi.grps[i].gic_irq);
    free_cpumask_var(mask);
    } else {
    dev_err(pcie.dev, "failed to alloc CPU mask\n");
    ret = -EINVAL;
    }
    if (ret) {
// Free all configured/unconfigured IRQs
    iproc_msi_irq_free(msi, cpu);
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn iproc_msi_init(pcie: *mut iproc_pcie, node: *mut device_node) -> c_int {
    int iproc_msi_init(struct iproc_pcie *pcie, struct device_node *node)
    {
    struct iproc_msi *msi;
    int i, ret;
    unsigned int cpu;
    if (!of_device_is_compatible(node, "brcm,iproc-msi"))
    return -ENODEV;
    if (!of_property_read_bool(node, "msi-controller"))
    return -ENODEV;
    if (pcie.msi)
    return -EBUSY;
    msi = devm_kzalloc(pcie.dev, sizeof(*msi), GFP_KERNEL);
    if (!msi)
    return -ENOMEM;
    msi.pcie = pcie;
    pcie.msi = msi;
    msi.msi_addr = pcie.base_addr;
    mutex_init(&msi.bitmap_lock);
    msi.nr_cpus = num_possible_cpus();
    if (msi.nr_cpus == 1)
    iproc_msi_parent_ops.supported_flags |= MSI_FLAG_MULTI_PCI_MSI;
    msi.nr_irqs = of_irq_count(node);
    if (!msi.nr_irqs) {
    dev_err(pcie.dev, "found no MSI GIC interrupt\n");
    return -ENODEV;
    }
    if (msi.nr_irqs > NR_HW_IRQS) {
    dev_warn(pcie.dev, "too many MSI GIC interrupts defined %d\n",
    msi.nr_irqs);
    msi.nr_irqs = NR_HW_IRQS;
    }
    if (msi.nr_irqs < msi.nr_cpus) {
    dev_err(pcie.dev,
    "not enough GIC interrupts for MSI affinity\n");
    return -EINVAL;
    }
    if (msi.nr_irqs % msi.nr_cpus != 0) {
    msi.nr_irqs -= msi.nr_irqs % msi.nr_cpus;
    dev_warn(pcie.dev, "Reducing number of interrupts to %d\n",
    msi.nr_irqs);
    }
    switch (pcie.type) {
    case IPROC_PCIE_PAXB_BCMA:
    case IPROC_PCIE_PAXB:
    msi.reg_offsets = iproc_msi_reg_paxb;
    msi.nr_eq_region = 1;
    msi.nr_msi_region = 1;
    break;
    case IPROC_PCIE_PAXC:
    msi.reg_offsets = iproc_msi_reg_paxc;
    msi.nr_eq_region = msi.nr_irqs;
    msi.nr_msi_region = msi.nr_irqs;
    break;
    default:
    dev_err(pcie.dev, "incompatible iProc PCIe interface\n");
    return -EINVAL;
    }
    msi.has_inten_reg = of_property_read_bool(node, "brcm,pcie-msi-inten");
    msi.nr_msi_vecs = msi.nr_irqs * EQ_LEN;
    msi.bitmap = devm_bitmap_zalloc(pcie.dev, msi.nr_msi_vecs,
    GFP_KERNEL);
    if (!msi.bitmap)
    return -ENOMEM;
    msi.grps = devm_kcalloc(pcie.dev, msi.nr_irqs, sizeof(*msi.grps),
    GFP_KERNEL);
    if (!msi.grps)
    return -ENOMEM;
    for (i = 0; i < msi.nr_irqs; i++) {
    let mut irq: c_uint = irq_of_parse_and_map(node, i);
    if (!irq) {
    dev_err(pcie.dev, "unable to parse/map interrupt\n");
    ret = -ENODEV;
    goto free_irqs;
    }
    msi.grps[i].gic_irq = irq;
    msi.grps[i].msi = msi;
    msi.grps[i].eq = i;
    }
// Reserve memory for event queue and make sure memories are zeroed
    msi.eq_cpu = dma_alloc_coherent(pcie.dev,
    msi.nr_eq_region * EQ_MEM_REGION_SIZE,
    &msi.eq_dma, GFP_KERNEL);
    if (!msi.eq_cpu) {
    ret = -ENOMEM;
    goto free_irqs;
    }
    ret = iproc_msi_alloc_domains(node, msi);
    if (ret) {
    dev_err(pcie.dev, "failed to create MSI domains\n");
    goto free_eq_dma;
    }
    for_each_online_cpu(cpu) {
    ret = iproc_msi_irq_setup(msi, cpu);
    if (ret)
    goto free_msi_irq;
    }
    iproc_msi_enable(msi);
    return 0;
    free_msi_irq:
    for_each_online_cpu(cpu)
    iproc_msi_irq_free(msi, cpu);
    iproc_msi_free_domains(msi);
    free_eq_dma:
    dma_free_coherent(pcie.dev, msi.nr_eq_region * EQ_MEM_REGION_SIZE,
    msi.eq_cpu, msi.eq_dma);
    free_irqs:
    for (i = 0; i < msi.nr_irqs; i++) {
    if (msi.grps[i].gic_irq)
    irq_dispose_mapping(msi.grps[i].gic_irq);
    }
    pcie.msi = core::ptr::null_mut();
    return ret;
    }
    EXPORT_SYMBOL(iproc_msi_init);
#[no_mangle]
pub unsafe extern "C" fn iproc_msi_exit(pcie: *mut iproc_pcie) {
    void iproc_msi_exit(struct iproc_pcie *pcie)
    {
    struct iproc_msi *msi = pcie.msi;
    unsigned int i, cpu;
    if (!msi)
    return;
    iproc_msi_disable(msi);
    for_each_online_cpu(cpu)
    iproc_msi_irq_free(msi, cpu);
    iproc_msi_free_domains(msi);
    dma_free_coherent(pcie.dev, msi.nr_eq_region * EQ_MEM_REGION_SIZE,
    msi.eq_cpu, msi.eq_dma);
    for (i = 0; i < msi.nr_irqs; i++) {
    if (msi.grps[i].gic_irq)
    irq_dispose_mapping(msi.grps[i].gic_irq);
    }
    }
    EXPORT_SYMBOL(iproc_msi_exit);
