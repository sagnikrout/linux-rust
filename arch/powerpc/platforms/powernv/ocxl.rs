//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powernv/ocxl.c
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright 2017 IBM Corp.

pub const PNV_OCXL_TL_P9_RECV_CAP: c_uint = 0x000000000000000Full;
pub const PNV_OCXL_ACTAG_MAX: c_int = 64;
// PASIDs are 20-bit, but on P9, NPU can only handle 15 bits
pub const PNV_OCXL_PASID_BITS: c_int = 15;

pub const AFU_INDEX_MASK: c_uint = 0x3F000000;
pub const AFU_INDEX_SHIFT: c_int = 24;
pub const ACTAG_MASK: c_uint = 0xFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct actag_range {
    pub start: u16,
    pub count: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npu_link {
    pub list: list_head,
    pub domain: c_int,
    pub bus: c_int,
    pub dev: c_int,
    pub fn_desired_actags: [u16; 8],
    pub fn_actags: [actag_range; 8],
    pub assignment_done: bool,
}

    let mut links_list: static struct list_head = LIST_HEAD_INIT(links_list);
    static DEFINE_MUTEX(links_list_lock);
//
// opencapi actags handling:
//
// When sending commands, the opencapi device references the memory
// context it's targeting with an 'actag', which is really an alias
// for a (BDF, pasid) combination. When it receives a command, the NPU
// must do a lookup of the actag to identify the memory context. The
// hardware supports a finite number of actags per link (64 for
// POWER9).
//
// The device can carry multiple functions, and each function can have
// multiple AFUs. Each AFU advertises in its config space the number
// of desired actags. The host must configure in the config space of
// the AFU how many actags the AFU is really allowed to use (which can
// be less than what the AFU desires).
//
// When a PCI function is probed by the driver, it has no visibility
// about the other PCI functions and how many actags they'd like,
// which makes it impossible to distribute actags fairly among AFUs.
//
// Unfortunately, the only way to know how many actags a function
// desires is by looking at the data for each AFU in the config space
// and add them up. Similarly, the only way to know how many actags
// all the functions of the physical device desire is by adding the
// previously computed function counts. Then we can match that against
// what the hardware supports.
//
// To get a comprehensive view, we use a 'pci fixup': at the end of
// PCI enumeration, each function counts how many actags its AFUs
// desire and we save it in a 'npu_link' structure, shared between all
// the PCI functions of a same device. Therefore, when the first
// function is probed by the driver, we can get an idea of the total
// count of desired actags for the device, and assign the actags to
// the AFUs, by pro-rating if needed.
//
#[no_mangle]
unsafe extern "C" fn find_dvsec_from_pos(dev: *mut pci_dev, dvsec_id: c_int, pos: c_int) -> c_int {
    static int find_dvsec_from_pos(struct pci_dev *dev, int dvsec_id, int pos)
    {
    let mut vsec: c_int = pos;
    u16 vendor, id;
    while ((vsec = pci_find_next_ext_capability(dev, vsec,
    OCXL_EXT_CAP_ID_DVSEC))) {
    pci_read_config_word(dev, vsec + OCXL_DVSEC_VENDOR_OFFSET,
    &vendor);
    pci_read_config_word(dev, vsec + OCXL_DVSEC_ID_OFFSET, &id);
    if (vendor == PCI_VENDOR_ID_IBM && id == dvsec_id)
    return vsec;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn find_dvsec_afu_ctrl(dev: *mut pci_dev, afu_idx: u8) -> c_int {
    static int find_dvsec_afu_ctrl(struct pci_dev *dev, u8 afu_idx)
    {
    let mut vsec: c_int = 0;
    u8 idx;
    while ((vsec = find_dvsec_from_pos(dev, OCXL_DVSEC_AFU_CTRL_ID,
    vsec))) {
    pci_read_config_byte(dev, vsec + OCXL_DVSEC_AFU_CTRL_AFU_IDX,
    &idx);
    if (idx == afu_idx)
    return vsec;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_max_afu_index(dev: *mut pci_dev, afu_idx: *mut c_int) -> c_int {
    static int get_max_afu_index(struct pci_dev *dev, int *afu_idx)
    {
    int pos;
    u32 val;
    pos = pci_find_dvsec_capability(dev, PCI_VENDOR_ID_IBM,
    OCXL_DVSEC_FUNC_ID);
    if (!pos)
    return -ESRCH;
    pci_read_config_dword(dev, pos + OCXL_DVSEC_FUNC_OFF_INDEX, &val);
    if (val & AFU_PRESENT)
// afu_idx = (val & AFU_INDEX_MASK) >> AFU_INDEX_SHIFT;
    else
// afu_idx = -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_actag_count(dev: *mut pci_dev, afu_idx: c_int, actag: *mut c_int) -> c_int {
    static int get_actag_count(struct pci_dev *dev, int afu_idx, int *actag)
    {
    int pos;
    u16 actag_sup;
    pos = find_dvsec_afu_ctrl(dev, afu_idx);
    if (!pos)
    return -ESRCH;
    pci_read_config_word(dev, pos + OCXL_DVSEC_AFU_CTRL_ACTAG_SUP,
    &actag_sup);
// actag = actag_sup & ACTAG_MASK;
    return 0;
    }
    static struct npu_link *find_link(struct pci_dev *dev)
    {
    struct npu_link *link;
    list_for_each_entry(link, &links_list, list) {
// The functions of a device all share the same link
    if (link.domain == pci_domain_nr(dev.bus) &&
    link.bus == dev.bus.number &&
    link.dev == PCI_SLOT(dev.devfn)) {
    return link;
    }
    }
// link doesn't exist yet. Allocate one
    link = kzalloc_obj(struct npu_link);
    if (!link)
    return core::ptr::null_mut();
    link.domain = pci_domain_nr(dev.bus);
    link.bus = dev.bus.number;
    link.dev = PCI_SLOT(dev.devfn);
    list_add(&link.list, &links_list);
    return link;
    }
#[no_mangle]
unsafe extern "C" fn pnv_ocxl_fixup_actag(dev: *mut pci_dev) {
    static void pnv_ocxl_fixup_actag(struct pci_dev *dev)
    {
    struct pci_controller *hose = pci_bus_to_host(dev.bus);
    struct pnv_phb *phb = hose.private_data;
    struct npu_link *link;
    int rc, afu_idx = -1, i, actag;
    if (!machine_is(powernv))
    return;
    if (phb.type != PNV_PHB_NPU_OCAPI)
    return;
    guard(mutex)(&links_list_lock);
    link = find_link(dev);
    if (!link) {
    dev_warn(&dev.dev, "couldn't update actag information\n");
    return;
    }
//
// Check how many actags are desired for the AFUs under that
// function and add it to the count for the link
//
    rc = get_max_afu_index(dev, &afu_idx);
    if (rc) {
// Most likely an invalid config space
    dev_dbg(&dev.dev, "couldn't find AFU information\n");
    afu_idx = -1;
    }
    link.fn_desired_actags[PCI_FUNC(dev.devfn)] = 0;
    for (i = 0; i <= afu_idx; i++) {
//
// AFU index 'holes' are allowed. So don't fail if we
// can't read the actag info for an index
//
    rc = get_actag_count(dev, i, &actag);
    if (rc)
    continue;
    link.fn_desired_actags[PCI_FUNC(dev.devfn)] += actag;
    }
    dev_dbg(&dev.dev, "total actags for function: %d\n",
    link.fn_desired_actags[PCI_FUNC(dev.devfn)]);
    }
    DECLARE_PCI_FIXUP_HEADER(PCI_ANY_ID, PCI_ANY_ID, pnv_ocxl_fixup_actag);
#[no_mangle]
unsafe extern "C" fn assign_fn_actags(desired: u16, total: u16) -> u16 {
    static u16 assign_fn_actags(u16 desired, u16 total)
    {
    u16 count;
    if (total <= PNV_OCXL_ACTAG_MAX)
    count = desired;
    else
    count = PNV_OCXL_ACTAG_MAX * desired / total;
    return count;
    }
#[no_mangle]
unsafe extern "C" fn assign_actags(link: *mut npu_link) {
    static void assign_actags(struct npu_link *link)
    {
    u16 actag_count, range_start = 0, total_desired = 0;
    int i;
    for (i = 0; i < 8; i++)
    total_desired += link.fn_desired_actags[i];
    for (i = 0; i < 8; i++) {
    if (link.fn_desired_actags[i]) {
    actag_count = assign_fn_actags(
    link.fn_desired_actags[i],
    total_desired);
    link.fn_actags[i].start = range_start;
    link.fn_actags[i].count = actag_count;
    range_start += actag_count;
    WARN_ON(range_start >= PNV_OCXL_ACTAG_MAX);
    }
    pr_debug("link %x:%x:%x fct %d actags: start=%d count=%d (desired=%d)\n",
    link.domain, link.bus, link.dev, i,
    link.fn_actags[i].start, link.fn_actags[i].count,
    link.fn_desired_actags[i]);
    }
    link.assignment_done = true;
    }
    int pnv_ocxl_get_actag(struct pci_dev *dev, u16 *base, u16 *enabled,
    u16 *supported)
    {
    struct npu_link *link;
    guard(mutex)(&links_list_lock);
    link = find_link(dev);
    if (!link) {
    dev_err(&dev.dev, "actag information not found\n");
    return -ENODEV;
    }
//
// On p9, we only have 64 actags per link, so they must be
// shared by all the functions of the same adapter. We counted
// the desired actag counts during PCI enumeration, so that we
// can allocate a pro-rated number of actags to each function.
//
    if (!link.assignment_done)
    assign_actags(link);
// base      = link->fn_actags[PCI_FUNC(dev->devfn)].start;
// enabled   = link->fn_actags[PCI_FUNC(dev->devfn)].count;
// supported = link->fn_desired_actags[PCI_FUNC(dev->devfn)];
    return 0;
    }
    EXPORT_SYMBOL_GPL(pnv_ocxl_get_actag);
#[no_mangle]
pub unsafe extern "C" fn pnv_ocxl_get_pasid_count(dev: *mut pci_dev, count: *mut c_int) -> c_int {
    int pnv_ocxl_get_pasid_count(struct pci_dev *dev, int *count)
    {
    struct npu_link *link;
    int i, rc = -EINVAL;
//
// The number of PASIDs (process address space ID) which can
// be used by a function depends on how many functions exist
// on the device. The NPU needs to be configured to know how
// many bits are available to PASIDs and how many are to be
// used by the function BDF identifier.
//
// We only support one AFU-carrying function for now.
//
    guard(mutex)(&links_list_lock);
    link = find_link(dev);
    if (!link) {
    dev_err(&dev.dev, "actag information not found\n");
    return -ENODEV;
    }
    for (i = 0; i < 8; i++)
    if (link.fn_desired_actags[i] && (i == PCI_FUNC(dev.devfn))) {
// count = PNV_OCXL_PASID_MAX;
    rc = 0;
    break;
    }
    dev_dbg(&dev.dev, "%d PASIDs available for function\n",
    rc ? 0 : *count);
    return rc;
    }
    EXPORT_SYMBOL_GPL(pnv_ocxl_get_pasid_count);
#[no_mangle]
unsafe extern "C" fn set_templ_rate(templ: c_uint, rate: c_uint, buf: *mut c_char) {
    static void set_templ_rate(unsigned int templ, unsigned int rate, char *buf)
    {
    int shift, idx;
    WARN_ON(templ > PNV_OCXL_TL_MAX_TEMPLATE);
    idx = (PNV_OCXL_TL_MAX_TEMPLATE - templ) / 2;
    shift = 4 * (1 - ((PNV_OCXL_TL_MAX_TEMPLATE - templ) % 2));
    buf[idx] |= rate << shift;
    }
    int pnv_ocxl_get_tl_cap(struct pci_dev *dev, long *cap,
    char *rate_buf, int rate_buf_size)
    {
    if (rate_buf_size != PNV_OCXL_TL_RATE_BUF_SIZE)
    return -EINVAL;
//
// The TL capabilities are a characteristic of the NPU, so
// we go with hard-coded values.
//
// The receiving rate of each template is encoded on 4 bits.
//
// On P9:
// - templates 0 -> 3 are supported
// - templates 0, 1 and 3 have a 0 receiving rate
// - template 2 has receiving rate of 1 (extra cycle)
//
    memset(rate_buf, 0, rate_buf_size);
    set_templ_rate(2, 1, rate_buf);
// cap = PNV_OCXL_TL_P9_RECV_CAP;
    return 0;
    }
    EXPORT_SYMBOL_GPL(pnv_ocxl_get_tl_cap);
    int pnv_ocxl_set_tl_conf(struct pci_dev *dev, long cap,
    uint64_t rate_buf_phys, int rate_buf_size)
    {
    struct pci_controller *hose = pci_bus_to_host(dev.bus);
    struct pnv_phb *phb = hose.private_data;
    int rc;
    if (rate_buf_size != PNV_OCXL_TL_RATE_BUF_SIZE)
    return -EINVAL;
    rc = opal_npu_tl_set(phb.opal_id, dev.devfn, cap,
    rate_buf_phys, rate_buf_size);
    if (rc) {
    dev_err(&dev.dev, "Can't configure host TL: %d\n", rc);
    return -EINVAL;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(pnv_ocxl_set_tl_conf);
#[no_mangle]
pub unsafe extern "C" fn pnv_ocxl_get_xsl_irq(dev: *mut pci_dev, hwirq: *mut c_int) -> c_int {
    int pnv_ocxl_get_xsl_irq(struct pci_dev *dev, int *hwirq)
    {
    int rc;
    rc = of_property_read_u32(dev.dev.of_node, "ibm,opal-xsl-irq", hwirq);
    if (rc) {
    dev_err(&dev.dev,
    "Can't get translation interrupt for device\n");
    return rc;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(pnv_ocxl_get_xsl_irq);
    void pnv_ocxl_unmap_xsl_regs(void __iomem *dsisr, void __iomem *dar,
    void __iomem *tfc, void __iomem *pe_handle)
    {
    iounmap(dsisr);
    iounmap(dar);
    iounmap(tfc);
    iounmap(pe_handle);
    }
    EXPORT_SYMBOL_GPL(pnv_ocxl_unmap_xsl_regs);
    int pnv_ocxl_map_xsl_regs(struct pci_dev *dev, void __iomem **dsisr,
    void __iomem **dar, void __iomem **tfc,
    void __iomem **pe_handle)
    {
    u64 reg;
    int i, j, rc = 0;
    void __iomem *regs[4];
//
// opal stores the mmio addresses of the DSISR, DAR, TFC and
// PE_HANDLE registers in a device tree property, in that
// order
//
    for (i = 0; i < 4; i++) {
    rc = of_property_read_u64_index(dev.dev.of_node,
    "ibm,opal-xsl-mmio", i, &reg);
    if (rc)
    break;
    regs[i] = ioremap(reg, 8);
    if (!regs[i]) {
    rc = -EINVAL;
    break;
    }
    }
    if (rc) {
    dev_err(&dev.dev, "Can't map translation mmio registers\n");
    for (j = i - 1; j >= 0; j--)
    iounmap(regs[j]);
    } else {
// dsisr = regs[0];
// dar = regs[1];
// tfc = regs[2];
// pe_handle = regs[3];
    }
    return rc;
    }
    EXPORT_SYMBOL_GPL(pnv_ocxl_map_xsl_regs);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spa_data {
    pub phb_opal_id: u64,
    pub bdfn: u32,
}

    int pnv_ocxl_spa_setup(struct pci_dev *dev, void *spa_mem, int PE_mask,
    void **platform_data)
    {
    struct pci_controller *hose = pci_bus_to_host(dev.bus);
    struct pnv_phb *phb = hose.private_data;
    struct spa_data *data;
    u32 bdfn;
    int rc;
    data = kzalloc_obj(*data);
    if (!data)
    return -ENOMEM;
    bdfn = pci_dev_id(dev);
    rc = opal_npu_spa_setup(phb.opal_id, bdfn, virt_to_phys(spa_mem),
    PE_mask);
    if (rc) {
    dev_err(&dev.dev, "Can't setup Shared Process Area: %d\n", rc);
    kfree(data);
    return rc;
    }
    data.phb_opal_id = phb.opal_id;
    data.bdfn = bdfn;
// platform_data = (void *) data;
    return 0;
    }
    EXPORT_SYMBOL_GPL(pnv_ocxl_spa_setup);
#[no_mangle]
pub unsafe extern "C" fn pnv_ocxl_spa_release(platform_data: *mut c_void) {
    void pnv_ocxl_spa_release(void *platform_data)
    {
    struct spa_data *data = (struct spa_data *) platform_data;
    int rc;
    rc = opal_npu_spa_setup(data.phb_opal_id, data.bdfn, 0, 0);
    WARN_ON(rc);
    kfree(data);
    }
    EXPORT_SYMBOL_GPL(pnv_ocxl_spa_release);
#[no_mangle]
pub unsafe extern "C" fn pnv_ocxl_spa_remove_pe_from_cache(platform_data: *mut c_void, pe_handle: c_int) -> c_int {
    int pnv_ocxl_spa_remove_pe_from_cache(void *platform_data, int pe_handle)
    {
    struct spa_data *data = (struct spa_data *) platform_data;
    return opal_npu_spa_clear_cache(data.phb_opal_id, data.bdfn, pe_handle);
    }
    EXPORT_SYMBOL_GPL(pnv_ocxl_spa_remove_pe_from_cache);
    int pnv_ocxl_map_lpar(struct pci_dev *dev, uint64_t lparid,
    uint64_t lpcr, void __iomem **arva)
    {
    struct pci_controller *hose = pci_bus_to_host(dev.bus);
    struct pnv_phb *phb = hose.private_data;
    u64 mmio_atsd;
    int rc;
// ATSD physical address.
// ATSD LAUNCH register: write access initiates a shoot down to
// initiate the TLB Invalidate command.
//
    rc = of_property_read_u64_index(hose.dn, "ibm,mmio-atsd",
    0, &mmio_atsd);
    if (rc) {
    dev_info(&dev.dev, "No available ATSD found\n");
    return rc;
    }
// Assign a register set to a Logical Partition and MMIO ATSD
// LPARID register to the required value.
//
    rc = opal_npu_map_lpar(phb.opal_id, pci_dev_id(dev),
    lparid, lpcr);
    if (rc) {
    dev_err(&dev.dev, "Error mapping device to LPAR: %d\n", rc);
    return rc;
    }
// arva = ioremap(mmio_atsd, 24);
    if (!(*arva)) {
    dev_warn(&dev.dev, "ioremap failed - mmio_atsd: %#llx\n", mmio_atsd);
    rc = -ENOMEM;
    }
    return rc;
    }
    EXPORT_SYMBOL_GPL(pnv_ocxl_map_lpar);
#[no_mangle]
pub unsafe extern "C" fn pnv_ocxl_unmap_lpar(arva: *mut void __iomem) {
    void pnv_ocxl_unmap_lpar(void __iomem *arva)
    {
    iounmap(arva);
    }
    EXPORT_SYMBOL_GPL(pnv_ocxl_unmap_lpar);
    void pnv_ocxl_tlb_invalidate(void __iomem *arva,
    unsigned long pid,
    unsigned long addr,
    unsigned long page_size)
    {
    let mut timeout: c_ulong = jiffies + (HZ * PNV_OCXL_ATSD_TIMEOUT);
    let mut val: u64 = 0ull;
    int pend;
    u8 size;
    if (!(arva))
    return;
    if (addr) {
// load Abbreviated Virtual Address register with
// the necessary value
//
    val |= FIELD_PREP(PNV_OCXL_ATSD_AVA_AVA, addr >> (63-51));
    out_be64(arva + PNV_OCXL_ATSD_AVA, val);
    }
// Write access initiates a shoot down to initiate the
// TLB Invalidate command
//
    val = PNV_OCXL_ATSD_LNCH_R;
    val |= FIELD_PREP(PNV_OCXL_ATSD_LNCH_RIC, 0b10);
    if (addr)
    val |= FIELD_PREP(PNV_OCXL_ATSD_LNCH_IS, 0b00);
    else {
    val |= FIELD_PREP(PNV_OCXL_ATSD_LNCH_IS, 0b01);
    val |= PNV_OCXL_ATSD_LNCH_OCAPI_SINGLETON;
    }
    val |= PNV_OCXL_ATSD_LNCH_PRS;
// Actual Page Size to be invalidated
// 000 4KB
// 101 64KB
// 001 2MB
// 010 1GB
//
    size = 0b101;
    if (page_size == 0x1000)
    size = 0b000;
    if (page_size == 0x200000)
    size = 0b001;
    if (page_size == 0x40000000)
    size = 0b010;
    val |= FIELD_PREP(PNV_OCXL_ATSD_LNCH_AP, size);
    val |= FIELD_PREP(PNV_OCXL_ATSD_LNCH_PID, pid);
    out_be64(arva + PNV_OCXL_ATSD_LNCH, val);
// Poll the ATSD status register to determine when the
// TLB Invalidate has been completed.
//
    val = in_be64(arva + PNV_OCXL_ATSD_STAT);
    pend = val >> 63;
    while (pend) {
    if (time_after_eq(jiffies, timeout)) {
    pr_err("%s - Timeout while reading XTS MMIO ATSD status register (val=%#llx, pidr=0x%lx)\n",
    __func__, val, pid);
    return;
    }
    cpu_relax();
    val = in_be64(arva + PNV_OCXL_ATSD_STAT);
    pend = val >> 63;
    }
    }
    EXPORT_SYMBOL_GPL(pnv_ocxl_tlb_invalidate);
