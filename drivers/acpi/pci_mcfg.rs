//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/pci_mcfg.c
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
// Copyright (C) 2016 Broadcom
// Author: Jayachandran C <jchandra@broadcom.com>
// Copyright (C) 2016 Semihalf
// Author: Tomasz Nowicki <tn@semihalf.com>
//

// Structure to hold entries from the MCFG table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcfg_entry {
    pub list: list_head,
    pub addr: phys_addr_t,
    pub segment: u16,
    pub bus_start: u8,
    pub bus_end: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcfg_fixup {
    pub 1]: char oem_id[ACPI_OEM_ID_SIZE +,
    pub 1]: char oem_table_id[ACPI_OEM_TABLE_ID_SIZE +,
    pub oem_revision: u32,
    pub segment: u16,
    pub bus_range: resource,
    pub ops: *const pci_ecam_ops,
    pub cfgres: resource,
}

    ((end) - (start) + 1),	\
    core::ptr::null_mut(), IORESOURCE_BUS)

    static struct mcfg_fixup mcfg_quirks[] = {
// { OEM_ID, OEM_TABLE_ID, REV, SEGMENT, BUS_RANGE, ops, cfgres },

    { "AMAZON", table_id, rev, seg, MCFG_BUS_ANY, ops }
    AL_ECAM("GRAVITON", 0, 0, &al_pcie_ops),
    AL_ECAM("GRAVITON", 0, 1, &al_pcie_ops),
    AL_ECAM("GRAVITON", 0, 2, &al_pcie_ops),
    AL_ECAM("GRAVITON", 0, 3, &al_pcie_ops),
    AL_ECAM("GRAVITON", 0, 4, &al_pcie_ops),
    AL_ECAM("GRAVITON", 0, 5, &al_pcie_ops),
    AL_ECAM("GRAVITON", 0, 6, &al_pcie_ops),
    AL_ECAM("GRAVITON", 0, 7, &al_pcie_ops),

    { "QCOM  ", "QDF2432 ", 1, seg, MCFG_BUS_ANY, &pci_32b_ops }
    QCOM_ECAM32(0),
    QCOM_ECAM32(1),
    QCOM_ECAM32(2),
    QCOM_ECAM32(3),
    QCOM_ECAM32(4),
    QCOM_ECAM32(5),
    QCOM_ECAM32(6),
    QCOM_ECAM32(7),

    { "HISI  ", table_id, 0, (seg) + 0, MCFG_BUS_ANY, ops }, \
    { "HISI  ", table_id, 0, (seg) + 1, MCFG_BUS_ANY, ops }, \
    { "HISI  ", table_id, 0, (seg) + 2, MCFG_BUS_ANY, ops }, \
    { "HISI  ", table_id, 0, (seg) + 3, MCFG_BUS_ANY, ops }
    HISI_QUAD_DOM("HIP05   ",  0, &hisi_pcie_ops),
    HISI_QUAD_DOM("HIP06   ",  0, &hisi_pcie_ops),
    HISI_QUAD_DOM("HIP07   ",  0, &hisi_pcie_ops),
    HISI_QUAD_DOM("HIP07   ",  4, &hisi_pcie_ops),
    HISI_QUAD_DOM("HIP07   ",  8, &hisi_pcie_ops),
    HISI_QUAD_DOM("HIP07   ", 12, &hisi_pcie_ops),

    DEFINE_RES_MEM((addr) + ((u64) (node) << 44), 0x39 * SZ_16M)

    { "CAVIUM", "THUNDERX", rev, 4 + (10 * (node)), MCFG_BUS_ANY,	    \
    &thunder_pem_ecam_ops, THUNDER_PEM_RES(0x88001f000000UL, node) },  \
    { "CAVIUM", "THUNDERX", rev, 5 + (10 * (node)), MCFG_BUS_ANY,	    \
    &thunder_pem_ecam_ops, THUNDER_PEM_RES(0x884057000000UL, node) },  \
    { "CAVIUM", "THUNDERX", rev, 6 + (10 * (node)), MCFG_BUS_ANY,	    \
    &thunder_pem_ecam_ops, THUNDER_PEM_RES(0x88808f000000UL, node) },  \
    { "CAVIUM", "THUNDERX", rev, 7 + (10 * (node)), MCFG_BUS_ANY,	    \
    &thunder_pem_ecam_ops, THUNDER_PEM_RES(0x89001f000000UL, node) },  \
    { "CAVIUM", "THUNDERX", rev, 8 + (10 * (node)), MCFG_BUS_ANY,	    \
    &thunder_pem_ecam_ops, THUNDER_PEM_RES(0x894057000000UL, node) },  \
    { "CAVIUM", "THUNDERX", rev, 9 + (10 * (node)), MCFG_BUS_ANY,	    \
    &thunder_pem_ecam_ops, THUNDER_PEM_RES(0x89808f000000UL, node) }

    { "CAVIUM", "THUNDERX", rev, seg, MCFG_BUS_ANY,			\
    &pci_thunder_ecam_ops }
// SoC pass2.x
    THUNDER_PEM_QUIRK(1, 0),
    THUNDER_PEM_QUIRK(1, 1),
    THUNDER_ECAM_QUIRK(1, 10),
// SoC pass1.x
    THUNDER_PEM_QUIRK(2, 0),	/* off-chip devices */
    THUNDER_PEM_QUIRK(2, 1),	/* off-chip devices */
    THUNDER_ECAM_QUIRK(2,  0),
    THUNDER_ECAM_QUIRK(2,  1),
    THUNDER_ECAM_QUIRK(2,  2),
    THUNDER_ECAM_QUIRK(2,  3),
    THUNDER_ECAM_QUIRK(2, 10),
    THUNDER_ECAM_QUIRK(2, 11),
    THUNDER_ECAM_QUIRK(2, 12),
    THUNDER_ECAM_QUIRK(2, 13),
    { "NVIDIA", "TEGRA194", 1, 0, MCFG_BUS_ANY, &tegra194_pcie_ops},
    { "NVIDIA", "TEGRA194", 1, 1, MCFG_BUS_ANY, &tegra194_pcie_ops},
    { "NVIDIA", "TEGRA194", 1, 2, MCFG_BUS_ANY, &tegra194_pcie_ops},
    { "NVIDIA", "TEGRA194", 1, 3, MCFG_BUS_ANY, &tegra194_pcie_ops},
    { "NVIDIA", "TEGRA194", 1, 4, MCFG_BUS_ANY, &tegra194_pcie_ops},
    { "NVIDIA", "TEGRA194", 1, 5, MCFG_BUS_ANY, &tegra194_pcie_ops},

    {"APM   ", "XGENE   ", rev, seg, MCFG_BUS_ANY, \
    &xgene_v1_pcie_ecam_ops }

    {"APM   ", "XGENE   ", rev, seg, MCFG_BUS_ANY, \
    &xgene_v2_pcie_ecam_ops }
// X-Gene SoC with v1 PCIe controller
    XGENE_V1_ECAM_MCFG(1, 0),
    XGENE_V1_ECAM_MCFG(1, 1),
    XGENE_V1_ECAM_MCFG(1, 2),
    XGENE_V1_ECAM_MCFG(1, 3),
    XGENE_V1_ECAM_MCFG(1, 4),
    XGENE_V1_ECAM_MCFG(2, 0),
    XGENE_V1_ECAM_MCFG(2, 1),
    XGENE_V1_ECAM_MCFG(2, 2),
    XGENE_V1_ECAM_MCFG(2, 3),
    XGENE_V1_ECAM_MCFG(2, 4),
// X-Gene SoC with v2.1 PCIe controller
    XGENE_V2_ECAM_MCFG(3, 0),
    XGENE_V2_ECAM_MCFG(3, 1),
// X-Gene SoC with v2.2 PCIe controller
    XGENE_V2_ECAM_MCFG(4, 0),
    XGENE_V2_ECAM_MCFG(4, 1),
    XGENE_V2_ECAM_MCFG(4, 2),

    { "Ampere", "Altra   ", rev, seg, MCFG_BUS_ANY, &pci_32b_read_ops }
    ALTRA_ECAM_QUIRK(1, 0),
    ALTRA_ECAM_QUIRK(1, 1),
    ALTRA_ECAM_QUIRK(1, 2),
    ALTRA_ECAM_QUIRK(1, 3),
    ALTRA_ECAM_QUIRK(1, 4),
    ALTRA_ECAM_QUIRK(1, 5),
    ALTRA_ECAM_QUIRK(1, 6),
    ALTRA_ECAM_QUIRK(1, 7),
    ALTRA_ECAM_QUIRK(1, 8),
    ALTRA_ECAM_QUIRK(1, 9),
    ALTRA_ECAM_QUIRK(1, 10),
    ALTRA_ECAM_QUIRK(1, 11),
    ALTRA_ECAM_QUIRK(1, 12),
    ALTRA_ECAM_QUIRK(1, 13),
    ALTRA_ECAM_QUIRK(1, 14),
    ALTRA_ECAM_QUIRK(1, 15),

    { "LOONGS", table_id, 1, seg, MCFG_BUS_ANY, &loongson_pci_ecam_ops }
    LOONGSON_ECAM_MCFG("\0", 0),
    LOONGSON_ECAM_MCFG("LOONGSON", 0),
    LOONGSON_ECAM_MCFG("\0", 1),
    LOONGSON_ECAM_MCFG("LOONGSON", 1),
    LOONGSON_ECAM_MCFG("\0", 2),
    LOONGSON_ECAM_MCFG("LOONGSON", 2),
    LOONGSON_ECAM_MCFG("\0", 3),
    LOONGSON_ECAM_MCFG("LOONGSON", 3),
    LOONGSON_ECAM_MCFG("\0", 4),
    LOONGSON_ECAM_MCFG("LOONGSON", 4),
    LOONGSON_ECAM_MCFG("\0", 5),
    LOONGSON_ECAM_MCFG("LOONGSON", 5),
    LOONGSON_ECAM_MCFG("\0", 6),
    LOONGSON_ECAM_MCFG("LOONGSON", 6),
    LOONGSON_ECAM_MCFG("\0", 7),
    LOONGSON_ECAM_MCFG("LOONGSON", 7),

    };
    static char mcfg_oem_id[ACPI_OEM_ID_SIZE];
    static char mcfg_oem_table_id[ACPI_OEM_TABLE_ID_SIZE];
    static u32 mcfg_oem_revision;
    static int pci_mcfg_quirk_matches(struct mcfg_fixup *f, u16 segment,
    struct resource *bus_range)
    {
    if (!memcmp(f.oem_id, mcfg_oem_id, ACPI_OEM_ID_SIZE) &&
    !memcmp(f.oem_table_id, mcfg_oem_table_id,
    ACPI_OEM_TABLE_ID_SIZE) &&
    f.oem_revision == mcfg_oem_revision &&
    f.segment == segment &&
    resource_contains(&f.bus_range, bus_range))
    return 1;
    return 0;
    }

    static void pci_mcfg_apply_quirks(struct acpi_pci_root *root,
    struct resource *cfgres,
    const struct pci_ecam_ops **ecam_ops)
    {

    let mut segment: u16 = root.segment;
    struct resource *bus_range = &root.secondary;
    struct mcfg_fixup *f;
    int i;
    for (i = 0, f = mcfg_quirks; i < ARRAY_SIZE(mcfg_quirks); i++, f++) {
    if (pci_mcfg_quirk_matches(f, segment, bus_range)) {
    if (f.cfgres.start)
// cfgres = f->cfgres;
    if (f.ops)
// ecam_ops =  f->ops;
    dev_info(&root.device.dev, "MCFG quirk: ECAM at %pR for %pR with %ps\n",
    cfgres, bus_range, *ecam_ops);
    return;
    }
    }

    }
// List to save MCFG entries
    static LIST_HEAD(pci_mcfg_list);
    int pci_mcfg_lookup(struct acpi_pci_root *root, struct resource *cfgres,
    const struct pci_ecam_ops **ecam_ops)
    {
    const struct pci_ecam_ops *ops = &pci_generic_ecam_ops;
    struct resource *bus_res = &root.secondary;
    let mut seg: u16 = root.segment;
    struct mcfg_entry *e;
    struct resource res;
// Use address from _CBA if present, otherwise lookup MCFG
    if (root.mcfg_addr)
    goto skip_lookup;
//
// We expect the range in bus_res in the coverage of MCFG bus range.
//
    list_for_each_entry(e, &pci_mcfg_list, list) {
    if (e.segment == seg && e.bus_start <= bus_res.start &&
    e.bus_end >= bus_res.end) {
    root.mcfg_addr = e.addr;
    }
    }
    skip_lookup:
    memset(&res, 0, sizeof(res));
    if (root.mcfg_addr) {
    res.start = root.mcfg_addr + (bus_res.start << 20);
    res.end = res.start + (resource_size(bus_res) << 20) - 1;
    res.flags = IORESOURCE_MEM;
    }
//
// Allow quirks to override default ECAM ops and CFG resource
// range.  This may even fabricate a CFG resource range in case
// MCFG does not have it.  Invalid CFG start address means MCFG
// firmware bug or we need another quirk in array.
//
    pci_mcfg_apply_quirks(root, &res, &ops);
    if (!res.start)
    return -ENXIO;
// cfgres = res;
// ecam_ops = ops;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci_mcfg_parse(header: *mut acpi_table_header) -> __init int {
    static __init int pci_mcfg_parse(struct acpi_table_header *header)
    {
    struct acpi_table_mcfg *mcfg;
    struct acpi_mcfg_allocation *mptr;
    struct mcfg_entry *e, *arr;
    int i, n;
    if (header.length < sizeof(struct acpi_table_mcfg))
    return -EINVAL;
    n = (header.length - sizeof(struct acpi_table_mcfg)) /
    sizeof(struct acpi_mcfg_allocation);
    mcfg = (struct acpi_table_mcfg *)header;
    mptr = (struct acpi_mcfg_allocation *) &mcfg[1];
    arr = kzalloc_objs(*arr, n);
    if (!arr)
    return -ENOMEM;
    for (i = 0, e = arr; i < n; i++, mptr++, e++) {
    e.segment = mptr.pci_segment;
    e.addr =  mptr.address;
    e.bus_start = mptr.start_bus_number;
    e.bus_end = mptr.end_bus_number;
    list_add(&e.list, &pci_mcfg_list);
    }

// Save MCFG IDs and revision for quirks matching
    memcpy(mcfg_oem_id, header.oem_id, ACPI_OEM_ID_SIZE);
    memcpy(mcfg_oem_table_id, header.oem_table_id, ACPI_OEM_TABLE_ID_SIZE);
    mcfg_oem_revision = header.oem_revision;

    pr_info("MCFG table detected, %d entries\n", n);
    return 0;
    }
// Interface called by ACPI - parse and save MCFG table
#[no_mangle]
pub unsafe extern "C" fn pci_mmcfg_late_init() -> void __init {
    void __init pci_mmcfg_late_init(void)
    {
    let mut err: c_int = acpi_table_parse(ACPI_SIG_MCFG, pci_mcfg_parse);
    if (err)
    pr_debug("Failed to parse MCFG (%d)\n", err);
    }
