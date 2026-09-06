//! Automatically rewritten from C to Rust
//! Source: drivers/dma/ioat/dca.c
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
// Intel I/OAT DMA Linux driver
// Copyright(c) 2007 - 2009 Intel Corporation.
//

// either a kernel change is needed, or we need something like this in kernel

//
// Bit 7 of a tag map entry is the "valid" bit, if it is set then bits 0:6
// contain the bit number of the APIC ID to map into the DCA tag.  If the valid
// bit is not set, then the value must be 0 or 1 and defines the bit in the tag.
//
pub const DCA_TAG_MAP_VALID: c_uint = 0x80;
pub const DCA3_TAG_MAP_BIT_TO_INV: c_uint = 0x80;
pub const DCA3_TAG_MAP_BIT_TO_SEL: c_uint = 0x40;
pub const DCA3_TAG_MAP_LITERAL_VAL: c_uint = 0x1;
pub const DCA_TAG_MAP_MASK: c_uint = 0xDF;
// expected tag map bytes for I/OAT ver.2
pub const DCA2_TAG_MAP_BYTE0: c_uint = 0x80;
pub const DCA2_TAG_MAP_BYTE1: c_uint = 0x0;
pub const DCA2_TAG_MAP_BYTE2: c_uint = 0x81;
pub const DCA2_TAG_MAP_BYTE3: c_uint = 0x82;
pub const DCA2_TAG_MAP_BYTE4: c_uint = 0x82;
//
// "Legacy" DCA systems do not implement the DCA register set in the
// I/OAT device.  Software needs direct support for their tag mappings.
//

pub const IOAT_TAG_MAP_LEN: c_int = 8;
// pack PCI B/D/F into a u16
#[no_mangle]
pub unsafe extern "C" fn dcaid_from_pcidev(pci: *mut pci_dev) -> u16 {
    static inline u16 dcaid_from_pcidev(struct pci_dev *pci)
    {
    return pci_dev_id(pci);
    }
#[no_mangle]
unsafe extern "C" fn dca_enabled_in_bios(pdev: *mut pci_dev) -> c_int {
    static int dca_enabled_in_bios(struct pci_dev *pdev)
    {
// CPUID level 9 returns DCA configuration
// Bit 0 indicates DCA enabled by the BIOS
    u32 eax;
    int res;
    eax = cpuid_eax(CPUID_LEAF_DCA);
    res = eax & BIT(0);
    if (!res)
    dev_dbg(&pdev.dev, "DCA is disabled in BIOS\n");
    return res;
    }
#[no_mangle]
pub unsafe extern "C" fn system_has_dca_enabled(pdev: *mut pci_dev) -> c_int {
    int system_has_dca_enabled(struct pci_dev *pdev)
    {
    if (boot_cpu_has(X86_FEATURE_DCA))
    return dca_enabled_in_bios(pdev);
    dev_dbg(&pdev.dev, "boot cpu doesn't have X86_FEATURE_DCA\n");
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioat_dca_slot {
    pub /: *mut *mut *mut pci_dev pdev; / requester device,
    pub /: *mut *mut u16 rid; / requester id, as used by IOAT,
}

pub const IOAT_DCA_MAX_REQ: c_int = 6;
pub const IOAT3_DCA_MAX_REQ: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioat_dca_priv {
    pub iobase: *mut void __iomem,
    pub dca_base: *mut void __iomem,
    pub max_requesters: c_int,
    pub requester_count: c_int,
    pub tag_map: [u8; IOAT_TAG_MAP_LEN],
    pub req_slots: [ioat_dca_slot; ],
}

    static int ioat_dca_dev_managed(struct dca_provider *dca,
    struct device *dev)
    {
    struct ioat_dca_priv *ioatdca = dca_priv(dca);
    struct pci_dev *pdev;
    int i;
    pdev = to_pci_dev(dev);
    for (i = 0; i < ioatdca.max_requesters; i++) {
    if (ioatdca.req_slots[i].pdev == pdev)
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ioat_dca_add_requester(dca: *mut dca_provider, dev: *mut device) -> c_int {
    static int ioat_dca_add_requester(struct dca_provider *dca, struct device *dev)
    {
    struct ioat_dca_priv *ioatdca = dca_priv(dca);
    struct pci_dev *pdev;
    int i;
    u16 id;
    u16 global_req_table;
// This implementation only supports PCI-Express
    if (!dev_is_pci(dev))
    return -ENODEV;
    pdev = to_pci_dev(dev);
    id = dcaid_from_pcidev(pdev);
    if (ioatdca.requester_count == ioatdca.max_requesters)
    return -ENODEV;
    for (i = 0; i < ioatdca.max_requesters; i++) {
    if (ioatdca.req_slots[i].pdev == core::ptr::null_mut()) {
// found an empty slot
    ioatdca.requester_count++;
    ioatdca.req_slots[i].pdev = pdev;
    ioatdca.req_slots[i].rid = id;
    global_req_table =
    readw(ioatdca.dca_base + IOAT3_DCA_GREQID_OFFSET);
    writel(id | IOAT_DCA_GREQID_VALID,
    ioatdca.iobase + global_req_table + (i * 4));
    return i;
    }
    }
// Error, ioatdma->requester_count is out of whack
    return -EFAULT;
    }
    static int ioat_dca_remove_requester(struct dca_provider *dca,
    struct device *dev)
    {
    struct ioat_dca_priv *ioatdca = dca_priv(dca);
    struct pci_dev *pdev;
    int i;
    u16 global_req_table;
// This implementation only supports PCI-Express
    if (!dev_is_pci(dev))
    return -ENODEV;
    pdev = to_pci_dev(dev);
    for (i = 0; i < ioatdca.max_requesters; i++) {
    if (ioatdca.req_slots[i].pdev == pdev) {
    global_req_table =
    readw(ioatdca.dca_base + IOAT3_DCA_GREQID_OFFSET);
    writel(0, ioatdca.iobase + global_req_table + (i * 4));
    ioatdca.req_slots[i].pdev = core::ptr::null_mut();
    ioatdca.req_slots[i].rid = 0;
    ioatdca.requester_count--;
    return i;
    }
    }
    return -ENODEV;
    }
    static u8 ioat_dca_get_tag(struct dca_provider *dca,
    struct device *dev,
    int cpu)
    {
    u8 tag;
    struct ioat_dca_priv *ioatdca = dca_priv(dca);
    int i, apic_id, bit, value;
    u8 entry;
    tag = 0;
    apic_id = cpu_physical_id(cpu);
    for (i = 0; i < IOAT_TAG_MAP_LEN; i++) {
    entry = ioatdca.tag_map[i];
    if (entry & DCA3_TAG_MAP_BIT_TO_SEL) {
    bit = entry &
    ~(DCA3_TAG_MAP_BIT_TO_SEL | DCA3_TAG_MAP_BIT_TO_INV);
    value = (apic_id & (1 << bit)) ? 1 : 0;
    } else if (entry & DCA3_TAG_MAP_BIT_TO_INV) {
    bit = entry & ~DCA3_TAG_MAP_BIT_TO_INV;
    value = (apic_id & (1 << bit)) ? 0 : 1;
    } else {
    value = (entry & DCA3_TAG_MAP_LITERAL_VAL) ? 1 : 0;
    }
    tag |= (value << i);
    }
    return tag;
    }
    static const struct dca_ops ioat_dca_ops = {
    .add_requester		= ioat_dca_add_requester,
    .remove_requester	= ioat_dca_remove_requester,
    .get_tag		= ioat_dca_get_tag,
    .dev_managed		= ioat_dca_dev_managed,
    };
#[no_mangle]
unsafe extern "C" fn ioat_dca_count_dca_slots(iobase: *mut c_void, dca_offset: u16) -> c_int {
    static int ioat_dca_count_dca_slots(void *iobase, u16 dca_offset)
    {
    let mut slots: c_int = 0;
    u32 req;
    u16 global_req_table;
    global_req_table = readw(iobase + dca_offset + IOAT3_DCA_GREQID_OFFSET);
    if (global_req_table == 0)
    return 0;
    do {
    req = readl(iobase + global_req_table + (slots * sizeof(u32)));
    slots++;
    } while ((req & IOAT_DCA_GREQID_LASTID) == 0);
    return slots;
    }
#[no_mangle]
pub unsafe extern "C" fn dca3_tag_map_invalid(tag_map: *mut u8) -> c_int {
    static inline int dca3_tag_map_invalid(u8 *tag_map)
    {
//
// If the tag map is not programmed by the BIOS the default is:
// 0x80 0x80 0x80 0x80 0x80 0x00 0x00 0x00
//
// This an invalid map and will result in only 2 possible tags
// 0x1F and 0x00.  0x00 is an invalid DCA tag so we know that
// this entire definition is invalid.
//
    return ((tag_map[0] == DCA_TAG_MAP_VALID) &&
    (tag_map[1] == DCA_TAG_MAP_VALID) &&
    (tag_map[2] == DCA_TAG_MAP_VALID) &&
    (tag_map[3] == DCA_TAG_MAP_VALID) &&
    (tag_map[4] == DCA_TAG_MAP_VALID));
    }
    struct dca_provider *ioat_dca_init(struct pci_dev *pdev, void __iomem *iobase)
    {
    struct dca_provider *dca;
    struct ioat_dca_priv *ioatdca;
    int slots;
    int i;
    int err;
    u16 dca_offset;
    u16 csi_fsb_control;
    u16 pcie_control;
    u8 bit;
    union {
    u64 full;
    struct {
    u32 low;
    u32 high;
    };
    } tag_map;
    if (!system_has_dca_enabled(pdev))
    return core::ptr::null_mut();
    dca_offset = readw(iobase + IOAT_DCAOFFSET_OFFSET);
    if (dca_offset == 0)
    return core::ptr::null_mut();
    slots = ioat_dca_count_dca_slots(iobase, dca_offset);
    if (slots == 0)
    return core::ptr::null_mut();
    dca = alloc_dca_provider(&ioat_dca_ops,
    struct_size(ioatdca, req_slots, slots));
    if (!dca)
    return core::ptr::null_mut();
    ioatdca = dca_priv(dca);
    ioatdca.iobase = iobase;
    ioatdca.dca_base = iobase + dca_offset;
    ioatdca.max_requesters = slots;
// some bios might not know to turn these on
    csi_fsb_control = readw(ioatdca.dca_base + IOAT3_CSI_CONTROL_OFFSET);
    if ((csi_fsb_control & IOAT3_CSI_CONTROL_PREFETCH) == 0) {
    csi_fsb_control |= IOAT3_CSI_CONTROL_PREFETCH;
    writew(csi_fsb_control,
    ioatdca.dca_base + IOAT3_CSI_CONTROL_OFFSET);
    }
    pcie_control = readw(ioatdca.dca_base + IOAT3_PCI_CONTROL_OFFSET);
    if ((pcie_control & IOAT3_PCI_CONTROL_MEMWR) == 0) {
    pcie_control |= IOAT3_PCI_CONTROL_MEMWR;
    writew(pcie_control,
    ioatdca.dca_base + IOAT3_PCI_CONTROL_OFFSET);
    }
// TODO version, compatibility and configuration checks
// copy out the APIC to DCA tag map
    tag_map.low =
    readl(ioatdca.dca_base + IOAT3_APICID_TAG_MAP_OFFSET_LOW);
    tag_map.high =
    readl(ioatdca.dca_base + IOAT3_APICID_TAG_MAP_OFFSET_HIGH);
    for (i = 0; i < 8; i++) {
    bit = tag_map.full >> (8 * i);
    ioatdca.tag_map[i] = bit & DCA_TAG_MAP_MASK;
    }
    if (dca3_tag_map_invalid(ioatdca.tag_map)) {
    add_taint(TAINT_FIRMWARE_WORKAROUND, LOCKDEP_STILL_OK);
    pr_warn_once("%s %s: APICID_TAG_MAP set incorrectly by BIOS, disabling DCA\n",
    dev_driver_string(&pdev.dev),
    dev_name(&pdev.dev));
    free_dca_provider(dca);
    return core::ptr::null_mut();
    }
    err = register_dca_provider(dca, &pdev.dev);
    if (err) {
    free_dca_provider(dca);
    return core::ptr::null_mut();
    }
    return dca;
    }
