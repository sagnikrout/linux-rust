//! Automatically rewritten from C to Rust
//! Source: drivers/dma/dw-edma/dw-edma-pcie.c
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
// Copyright (c) 2018-2019 Synopsys, Inc. and/or its affiliates.
// Synopsys DesignWare eDMA PCIe driver
//
// Author: Gustavo Pimentel <gustavo.pimentel@synopsys.com>
//

// Synopsys
pub const DW_PCIE_SYNOPSYS_VSEC_DMA_ID: c_uint = 0x6;

// AMD MDB (Xilinx) specific defines
pub const PCI_DEVICE_ID_XILINX_B054: c_uint = 0xb054;
pub const PCI_DEVICE_ID_XILINX_B00F: c_uint = 0xb00f;
pub const DW_PCIE_XILINX_MDB_VSEC_DMA_ID: c_uint = 0x6;
pub const DW_PCIE_XILINX_MDB_VSEC_ID: c_uint = 0x20;

pub const DW_PCIE_XILINX_MDB_DEVMEM_OFF_REG_HIGH: c_uint = 0xc;
pub const DW_PCIE_XILINX_MDB_DEVMEM_OFF_REG_LOW: c_uint = 0x8;

pub const DW_PCIE_XILINX_MDB_LL_OFF_GAP: c_uint = 0x200000;
pub const DW_PCIE_XILINX_MDB_LL_SIZE: c_uint = 0x800;
pub const DW_PCIE_XILINX_MDB_DT_OFF_GAP: c_uint = 0x100000;
pub const DW_PCIE_XILINX_MDB_DT_SIZE: c_uint = 0x800;

    { \
    .bar = a, \
    .off = b, \
    .sz = c, \
    },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_edma_block {
    pub bar: enum pci_barno,
    pub off: off_t,
    pub paddr: u64,
    pub paddr_valid: bool,
    pub sz: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_edma_pcie_data {
// eDMA registers location
    pub rg: dw_edma_block,
// eDMA memory linked list location
    pub ll_wr: [dw_edma_block; HDMA_MAX_WR_CH],
    pub ll_rd: [dw_edma_block; HDMA_MAX_RD_CH],
// eDMA memory data location
    pub dt_wr: [dw_edma_block; HDMA_MAX_WR_CH],
    pub dt_rd: [dw_edma_block; HDMA_MAX_RD_CH],
// Other
    pub mf: enum dw_edma_map_format,
    pub irqs: u8,
    pub wr_ch_cnt: u16,
    pub rd_ch_cnt: u16,
    pub devmem_phys_off: u64,
    pub cfg_non_ll: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_edma_pcie_match_data {
    pub data: *const dw_edma_pcie_data,
    pub plat_ops: *const dw_edma_plat_ops,
//
// Mandatory callback. It may leave @pdata unchanged when the static
// template already describes the device.
//
    int (*parse_caps)(struct pci_dev *pdev,
    pub pdata): *mut dw_edma_pcie_data,
    pub flags: c_ulong,
    pub chip_flags: u32,
}

    static const struct dw_edma_pcie_data snps_edda_data = {
// eDMA registers location
    .rg.bar				= BAR_0,
    .rg.off				= 0x00001000,	/*  4 Kbytes */
    .rg.sz				= 0x00002000,	/*  8 Kbytes */
// eDMA memory linked list location
    .ll_wr = {
// Channel 0 - BAR 2, offset 0 Mbytes, size 2 Kbytes
    DW_BLOCK(BAR_2, 0x00000000, 0x00000800)
// Channel 1 - BAR 2, offset 2 Mbytes, size 2 Kbytes
    DW_BLOCK(BAR_2, 0x00200000, 0x00000800)
    },
    .ll_rd = {
// Channel 0 - BAR 2, offset 4 Mbytes, size 2 Kbytes
    DW_BLOCK(BAR_2, 0x00400000, 0x00000800)
// Channel 1 - BAR 2, offset 6 Mbytes, size 2 Kbytes
    DW_BLOCK(BAR_2, 0x00600000, 0x00000800)
    },
// eDMA memory data location
    .dt_wr = {
// Channel 0 - BAR 2, offset 8 Mbytes, size 2 Kbytes
    DW_BLOCK(BAR_2, 0x00800000, 0x00000800)
// Channel 1 - BAR 2, offset 9 Mbytes, size 2 Kbytes
    DW_BLOCK(BAR_2, 0x00900000, 0x00000800)
    },
    .dt_rd = {
// Channel 0 - BAR 2, offset 10 Mbytes, size 2 Kbytes
    DW_BLOCK(BAR_2, 0x00a00000, 0x00000800)
// Channel 1 - BAR 2, offset 11 Mbytes, size 2 Kbytes
    DW_BLOCK(BAR_2, 0x00b00000, 0x00000800)
    },
// Other
    .mf				= EDMA_MF_EDMA_UNROLL,
    .irqs				= 1,
    .wr_ch_cnt			= 2,
    .rd_ch_cnt			= 2,
    };
    static const struct dw_edma_pcie_data xilinx_mdb_data = {
// MDB registers location
    .rg.bar				= BAR_0,
    .rg.off				= SZ_4K,	/*  4 Kbytes */
    .rg.sz				= SZ_8K,	/*  8 Kbytes */
// Other
    .mf				= EDMA_MF_HDMA_NATIVE,
    .irqs				= 1,
    .wr_ch_cnt			= 8,
    .rd_ch_cnt			= 8,
    };
    static const struct dw_edma_pcie_data xilinx_cpm6_dma_data = {
// MDB registers location
    .rg.bar				= BAR_0,
    .rg.off				= SZ_4K,	/*  4 Kbytes */
    .rg.sz				= SZ_8K,	/*  8 Kbytes */
// Other
    .mf				= EDMA_MF_HDMA_NATIVE,
    .irqs				= 1,
    .wr_ch_cnt			= 8,
    .rd_ch_cnt			= 8,
    };
    static void dw_edma_set_chan_region_offset(struct dw_edma_pcie_data *pdata,
    enum pci_barno bar, off_t start_off,
    off_t ll_off_gap, size_t ll_size,
    off_t dt_off_gap, size_t dt_size)
    {
    let mut wr_ch: u16 = pdata.wr_ch_cnt;
    let mut rd_ch: u16 = pdata.rd_ch_cnt;
    off_t off;
    u16 i;
    off = start_off;
// Write channel LL region
    for (i = 0; i < wr_ch; i++) {
    pdata.ll_wr[i].bar = bar;
    pdata.ll_wr[i].off = off;
    pdata.ll_wr[i].sz = ll_size;
    off += ll_off_gap;
    }
// Read channel LL region
    for (i = 0; i < rd_ch; i++) {
    pdata.ll_rd[i].bar = bar;
    pdata.ll_rd[i].off = off;
    pdata.ll_rd[i].sz = ll_size;
    off += ll_off_gap;
    }
// Write channel data region
    for (i = 0; i < wr_ch; i++) {
    pdata.dt_wr[i].bar = bar;
    pdata.dt_wr[i].off = off;
    pdata.dt_wr[i].sz = dt_size;
    off += dt_off_gap;
    }
// Read channel data region
    for (i = 0; i < rd_ch; i++) {
    pdata.dt_rd[i].bar = bar;
    pdata.dt_rd[i].off = off;
    pdata.dt_rd[i].sz = dt_size;
    off += dt_off_gap;
    }
    }
#[no_mangle]
unsafe extern "C" fn dw_edma_pcie_irq_vector(dev: *mut device, nr: c_uint) -> c_int {
    static int dw_edma_pcie_irq_vector(struct device *dev, unsigned int nr)
    {
    return pci_irq_vector(to_pci_dev(dev), nr);
    }
#[no_mangle]
unsafe extern "C" fn dw_edma_pcie_address(dev: *mut device, cpu_addr: phys_addr_t) -> u64 {
    static u64 dw_edma_pcie_address(struct device *dev, phys_addr_t cpu_addr)
    {
    struct pci_dev *pdev = to_pci_dev(dev);
    struct pci_bus_region region;
    struct resource res = {
    .flags = IORESOURCE_MEM,
    .start = cpu_addr,
    .end = cpu_addr,
    };
    pcibios_resource_to_bus(pdev.bus, &region, &res);
    return region.start;
    }
    static const struct dw_edma_plat_ops dw_edma_pcie_plat_ops = {
    .irq_vector = dw_edma_pcie_irq_vector,
    .pci_address = dw_edma_pcie_address,
    };
    static void dw_edma_pcie_get_synopsys_dma_data(struct pci_dev *pdev,
    struct dw_edma_pcie_data *pdata)
    {
    u32 val, map;
    u16 vsec;
    u64 off;
    vsec = pci_find_vsec_capability(pdev, PCI_VENDOR_ID_SYNOPSYS,
    DW_PCIE_SYNOPSYS_VSEC_DMA_ID);
    if (!vsec)
    return;
    pci_read_config_dword(pdev, vsec + PCI_VNDR_HEADER, &val);
    if (PCI_VNDR_HEADER_REV(val) != 0x00 ||
    PCI_VNDR_HEADER_LEN(val) != 0x18)
    return;
    pci_dbg(pdev, "Detected Synopsys PCIe Vendor-Specific Extended Capability DMA\n");
    pci_read_config_dword(pdev, vsec + 0x8, &val);
    map = FIELD_GET(DW_PCIE_SYNOPSYS_VSEC_DMA_MAP, val);
    if (map != EDMA_MF_EDMA_LEGACY &&
    map != EDMA_MF_EDMA_UNROLL &&
    map != EDMA_MF_HDMA_COMPAT &&
    map != EDMA_MF_HDMA_NATIVE)
    return;
    pdata.mf = map;
    pdata.rg.bar = FIELD_GET(DW_PCIE_SYNOPSYS_VSEC_DMA_BAR, val);
    pci_read_config_dword(pdev, vsec + 0xc, &val);
    pdata.wr_ch_cnt = min_t(u16, pdata.wr_ch_cnt,
    FIELD_GET(DW_PCIE_SYNOPSYS_VSEC_DMA_WR_CH, val));
    pdata.rd_ch_cnt = min_t(u16, pdata.rd_ch_cnt,
    FIELD_GET(DW_PCIE_SYNOPSYS_VSEC_DMA_RD_CH, val));
    pci_read_config_dword(pdev, vsec + 0x14, &val);
    off = val;
    pci_read_config_dword(pdev, vsec + 0x10, &val);
    off <<= 32;
    off |= val;
    pdata.rg.off = off;
    }
    static void dw_edma_pcie_get_xilinx_dma_data(struct pci_dev *pdev,
    struct dw_edma_pcie_data *pdata)
    {
    u32 val, map;
    u16 vsec;
    u64 off;
    pdata.devmem_phys_off = DW_PCIE_XILINX_MDB_INVALID_ADDR;
    vsec = pci_find_vsec_capability(pdev, PCI_VENDOR_ID_XILINX,
    DW_PCIE_XILINX_MDB_VSEC_DMA_ID);
    if (!vsec)
    return;
    pci_read_config_dword(pdev, vsec + PCI_VNDR_HEADER, &val);
    if (PCI_VNDR_HEADER_REV(val) != 0x00 ||
    PCI_VNDR_HEADER_LEN(val) != 0x18)
    return;
    pci_dbg(pdev, "Detected Xilinx PCIe Vendor-Specific Extended Capability DMA\n");
    pci_read_config_dword(pdev, vsec + 0x8, &val);
    map = FIELD_GET(DW_PCIE_XILINX_MDB_VSEC_DMA_MAP, val);
    if (map != EDMA_MF_HDMA_NATIVE)
    return;
    pdata.mf = map;
    pdata.rg.bar = FIELD_GET(DW_PCIE_XILINX_MDB_VSEC_DMA_BAR, val);
    pci_read_config_dword(pdev, vsec + 0xc, &val);
    pdata.wr_ch_cnt = min(pdata.wr_ch_cnt,
    FIELD_GET(DW_PCIE_XILINX_MDB_VSEC_DMA_WR_CH, val));
    pdata.rd_ch_cnt = min(pdata.rd_ch_cnt,
    FIELD_GET(DW_PCIE_XILINX_MDB_VSEC_DMA_RD_CH, val));
    pci_read_config_dword(pdev, vsec + 0x14, &val);
    off = val;
    pci_read_config_dword(pdev, vsec + 0x10, &val);
    off <<= 32;
    off |= val;
    pdata.rg.off = off;
    vsec = pci_find_vsec_capability(pdev, PCI_VENDOR_ID_XILINX,
    DW_PCIE_XILINX_MDB_VSEC_ID);
    if (!vsec)
    return;
    pci_read_config_dword(pdev,
    vsec + DW_PCIE_XILINX_MDB_DEVMEM_OFF_REG_HIGH,
    &val);
    off = val;
    pci_read_config_dword(pdev,
    vsec + DW_PCIE_XILINX_MDB_DEVMEM_OFF_REG_LOW,
    &val);
    off <<= 32;
    off |= val;
    pdata.devmem_phys_off = off;
    }
    static int
    dw_edma_pcie_parse_synopsys_caps(struct pci_dev *pdev,
    struct dw_edma_pcie_data *pdata)
    {
    dw_edma_pcie_get_synopsys_dma_data(pdev, pdata);
    return 0;
    }
    static int
    dw_edma_pcie_parse_xilinx_caps(struct pci_dev *pdev,
    struct dw_edma_pcie_data *pdata)
    {
    dw_edma_pcie_get_xilinx_dma_data(pdev, pdata);
//
// There is no valid address found for the LL memory space on the
// device side. In the absence of LL base address use the non-LL mode or
// simple mode supported by the HDMA IP.
//
    if (pdata.devmem_phys_off == DW_PCIE_XILINX_MDB_INVALID_ADDR) {
    pdata.cfg_non_ll = true;
    return 0;
    }
//
// Configure the channel LL and data blocks if number of channels
// enabled in VSEC capability are more than the channels configured in
// xilinx_mdb_data.
//
    dw_edma_set_chan_region_offset(pdata, BAR_2, 0,
    DW_PCIE_XILINX_MDB_LL_OFF_GAP,
    DW_PCIE_XILINX_MDB_LL_SIZE,
    DW_PCIE_XILINX_MDB_DT_OFF_GAP,
    DW_PCIE_XILINX_MDB_DT_SIZE);
    return 0;
    }
    static u64 dw_edma_get_phys_addr(struct pci_dev *pdev,
    const struct dw_edma_pcie_match_data *match,
    struct dw_edma_pcie_data *pdata,
    enum pci_barno bar)
    {
    if (match.flags & DW_EDMA_PCIE_F_DEVMEM_PHYS_OFF)
    return pdata.devmem_phys_off;
    return pci_bus_address(pdev, bar);
    }
    static u64 dw_edma_get_block_addr(struct pci_dev *pdev,
    const struct dw_edma_pcie_match_data *match,
    struct dw_edma_pcie_data *pdata,
    const struct dw_edma_block *block)
    {
    if (block.paddr_valid)
    return block.paddr;
    return dw_edma_get_phys_addr(pdev, match, pdata, block.bar) +
    block.off;
    }
    static int dw_edma_pcie_probe(struct pci_dev *pdev,
    const struct pci_device_id *pid)
    {
    const struct dw_edma_pcie_match_data *match = (void *)pid.driver_data;
    const struct dw_edma_pcie_data *pdata;
    struct device *dev = &pdev.dev;
    struct dw_edma_chip *chip;
    int err, nr_irqs;
    int i, mask;
    if (!match)
    return -ENODEV;
    pdata = match.data;
    if (!pdata)
    return -ENODEV;
    struct dw_edma_pcie_data *dma_data __free(kfree) =
    kmemdup(pdata, sizeof(*dma_data), GFP_KERNEL);
    if (!dma_data)
    return -ENOMEM;
// Enable PCI device
    err = pcim_enable_device(pdev);
    if (err) {
    pci_err(pdev, "enabling device failed\n");
    return err;
    }
// Let device-specific discovery override the static template data.
    if (!match.parse_caps || !match.plat_ops)
    return -EINVAL;
    err = match.parse_caps(pdev, dma_data);
    if (err)
    return err;
// Mapping PCI BAR regions
    mask = BIT(dma_data.rg.bar);
    for (i = 0; i < dma_data.wr_ch_cnt; i++) {
    mask |= BIT(dma_data.ll_wr[i].bar);
    if (dma_data.dt_wr[i].sz)
    mask |= BIT(dma_data.dt_wr[i].bar);
    }
    for (i = 0; i < dma_data.rd_ch_cnt; i++) {
    mask |= BIT(dma_data.ll_rd[i].bar);
    if (dma_data.dt_rd[i].sz)
    mask |= BIT(dma_data.dt_rd[i].bar);
    }
    err = pcim_iomap_regions(pdev, mask, pci_name(pdev));
    if (err) {
    pci_err(pdev, "eDMA BAR I/O remapping failed\n");
    return err;
    }
    pci_set_master(pdev);
// DMA configuration
    err = dma_set_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(64));
    if (err) {
    pci_err(pdev, "DMA mask 64 set failed\n");
    return err;
    }
// Data structure allocation
    chip = devm_kzalloc(dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
// IRQs allocation
    nr_irqs = pci_alloc_irq_vectors(pdev, 1, dma_data.irqs,
    PCI_IRQ_MSI | PCI_IRQ_MSIX);
    if (nr_irqs < 1) {
    pci_err(pdev, "fail to alloc IRQ vector (number of IRQs=%u)\n",
    nr_irqs);
    return -EPERM;
    }
// Data structure initialization
    chip.dev = dev;
    chip.mf = dma_data.mf;
    chip.flags = match.chip_flags;
    chip.func_no = PCI_FUNC(pdev.devfn);
    chip.nr_irqs = nr_irqs;
    chip.ops = match.plat_ops;
    chip.cfg_non_ll = dma_data.cfg_non_ll;
    chip.ll_wr_cnt = dma_data.wr_ch_cnt;
    chip.ll_rd_cnt = dma_data.rd_ch_cnt;
    chip.reg_base = pcim_iomap_table(pdev)[dma_data.rg.bar];
    if (!chip.reg_base)
    return -ENOMEM;
    if (match.flags & DW_EDMA_PCIE_F_REG_OFFSET)
    chip.reg_base += dma_data.rg.off;
    for (i = 0; i < chip.ll_wr_cnt && !dma_data.cfg_non_ll; i++) {
    struct dw_edma_region *ll_region = &chip.ll_region_wr[i];
    struct dw_edma_region *dt_region = &chip.dt_region_wr[i];
    struct dw_edma_block *ll_block = &dma_data.ll_wr[i];
    struct dw_edma_block *dt_block = &dma_data.dt_wr[i];
    ll_region.vaddr.io = pcim_iomap_table(pdev)[ll_block.bar];
    if (!ll_region.vaddr.io)
    return -ENOMEM;
    ll_region.vaddr.io += ll_block.off;
    ll_region.paddr = dw_edma_get_block_addr(pdev, match, dma_data,
    ll_block);
    ll_region.sz = ll_block.sz;
    if (!dt_block.sz)
    continue;
    dt_region.vaddr.io = pcim_iomap_table(pdev)[dt_block.bar];
    if (!dt_region.vaddr.io)
    return -ENOMEM;
    dt_region.vaddr.io += dt_block.off;
    dt_region.paddr = dw_edma_get_block_addr(pdev, match, dma_data,
    dt_block);
    dt_region.sz = dt_block.sz;
    }
    for (i = 0; i < chip.ll_rd_cnt && !dma_data.cfg_non_ll; i++) {
    struct dw_edma_region *ll_region = &chip.ll_region_rd[i];
    struct dw_edma_region *dt_region = &chip.dt_region_rd[i];
    struct dw_edma_block *ll_block = &dma_data.ll_rd[i];
    struct dw_edma_block *dt_block = &dma_data.dt_rd[i];
    ll_region.vaddr.io = pcim_iomap_table(pdev)[ll_block.bar];
    if (!ll_region.vaddr.io)
    return -ENOMEM;
    ll_region.vaddr.io += ll_block.off;
    ll_region.paddr = dw_edma_get_block_addr(pdev, match, dma_data,
    ll_block);
    ll_region.sz = ll_block.sz;
    if (!dt_block.sz)
    continue;
    dt_region.vaddr.io = pcim_iomap_table(pdev)[dt_block.bar];
    if (!dt_region.vaddr.io)
    return -ENOMEM;
    dt_region.vaddr.io += dt_block.off;
    dt_region.paddr = dw_edma_get_block_addr(pdev, match, dma_data,
    dt_block);
    dt_region.sz = dt_block.sz;
    }
// Debug info
    if (chip.mf == EDMA_MF_EDMA_LEGACY)
    pci_dbg(pdev, "Version:\teDMA Port Logic (0x%x)\n", chip.mf);
#[no_mangle]
pub unsafe extern "C" fn if(EDMA_MF_EDMA_UNROLL: chip->mf ==) -> else {
    else if (chip.mf == EDMA_MF_EDMA_UNROLL)
    pci_dbg(pdev, "Version:\teDMA Unroll (0x%x)\n", chip.mf);
#[no_mangle]
pub unsafe extern "C" fn if(EDMA_MF_HDMA_COMPAT: chip->mf ==) -> else {
    else if (chip.mf == EDMA_MF_HDMA_COMPAT)
    pci_dbg(pdev, "Version:\tHDMA Compatible (0x%x)\n", chip.mf);
#[no_mangle]
pub unsafe extern "C" fn if(EDMA_MF_HDMA_NATIVE: chip->mf ==) -> else {
    else if (chip.mf == EDMA_MF_HDMA_NATIVE)
    pci_dbg(pdev, "Version:\tHDMA Native (0x%x)\n", chip.mf);
    else
    pci_dbg(pdev, "Version:\tUnknown (0x%x)\n", chip.mf);
    pci_dbg(pdev, "Registers:\tBAR=%u, off=0x%.8lx, sz=0x%zx bytes, addr(v=%p)\n",
    dma_data.rg.bar, dma_data.rg.off, dma_data.rg.sz,
    chip.reg_base);
    for (i = 0; i < chip.ll_wr_cnt; i++) {
    pci_dbg(pdev, "L. List:\tWRITE CH%.2u, BAR=%u, off=0x%.8lx, sz=0x%zx bytes, addr(v=%p, p=%pa)\n",
    i, dma_data.ll_wr[i].bar,
    dma_data.ll_wr[i].off, chip.ll_region_wr[i].sz,
    chip.ll_region_wr[i].vaddr.io, &chip.ll_region_wr[i].paddr);
    if (!dma_data.dt_wr[i].sz)
    continue;
    pci_dbg(pdev, "Data:\tWRITE CH%.2u, BAR=%u, off=0x%.8lx, sz=0x%zx bytes, addr(v=%p, p=%pa)\n",
    i, dma_data.dt_wr[i].bar,
    dma_data.dt_wr[i].off, chip.dt_region_wr[i].sz,
    chip.dt_region_wr[i].vaddr.io,
    &chip.dt_region_wr[i].paddr);
    }
    for (i = 0; i < chip.ll_rd_cnt; i++) {
    pci_dbg(pdev, "L. List:\tREAD CH%.2u, BAR=%u, off=0x%.8lx, sz=0x%zx bytes, addr(v=%p, p=%pa)\n",
    i, dma_data.ll_rd[i].bar,
    dma_data.ll_rd[i].off, chip.ll_region_rd[i].sz,
    chip.ll_region_rd[i].vaddr.io, &chip.ll_region_rd[i].paddr);
    if (!dma_data.dt_rd[i].sz)
    continue;
    pci_dbg(pdev, "Data:\tREAD CH%.2u, BAR=%u, off=0x%.8lx, sz=0x%zx bytes, addr(v=%p, p=%pa)\n",
    i, dma_data.dt_rd[i].bar,
    dma_data.dt_rd[i].off, chip.dt_region_rd[i].sz,
    chip.dt_region_rd[i].vaddr.io,
    &chip.dt_region_rd[i].paddr);
    }
    pci_dbg(pdev, "Nr. IRQs:\t%u\n", chip.nr_irqs);
// Validating if PCI interrupts were enabled
    if (!pci_dev_msi_enabled(pdev)) {
    pci_err(pdev, "enable interrupt failed\n");
    return -EPERM;
    }
// Starting eDMA driver
    err = dw_edma_probe(chip);
    if (err) {
    pci_err(pdev, "eDMA probe failed\n");
    return err;
    }
// Saving data structure reference
    pci_set_drvdata(pdev, chip);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw_edma_pcie_remove(pdev: *mut pci_dev) {
    static void dw_edma_pcie_remove(struct pci_dev *pdev)
    {
    struct dw_edma_chip *chip = pci_get_drvdata(pdev);
    int err;
// Stopping eDMA driver
    err = dw_edma_remove(chip);
    if (err)
    pci_warn(pdev, "can't remove device properly: %d\n", err);
    }
    static const struct dw_edma_pcie_match_data snps_edda_match_data = {
    .data = &snps_edda_data,
    .plat_ops = &dw_edma_pcie_plat_ops,
    .parse_caps = dw_edma_pcie_parse_synopsys_caps,
    };
    static const struct dw_edma_pcie_match_data xilinx_mdb_match_data = {
    .data = &xilinx_mdb_data,
    .plat_ops = &dw_edma_pcie_plat_ops,
    .parse_caps = dw_edma_pcie_parse_xilinx_caps,
    .flags = DW_EDMA_PCIE_F_DEVMEM_PHYS_OFF,
    };
    static const struct dw_edma_pcie_match_data xilinx_cpm6_dma_match_data = {
    .data = &xilinx_cpm6_dma_data,
    .plat_ops = &dw_edma_pcie_plat_ops,
    .parse_caps = dw_edma_pcie_parse_xilinx_caps,
    .flags = DW_EDMA_PCIE_F_DEVMEM_PHYS_OFF,
    };
    static const struct pci_device_id dw_edma_pcie_id_table[] = {
    { PCI_DEVICE_DATA(SYNOPSYS, EDDA, &snps_edda_match_data) },
    { PCI_VDEVICE(XILINX, PCI_DEVICE_ID_XILINX_B054),
    .driver_data = (kernel_ulong_t)&xilinx_mdb_match_data },
    { PCI_VDEVICE(XILINX, PCI_DEVICE_ID_XILINX_B00F),
    .driver_data = (kernel_ulong_t)&xilinx_cpm6_dma_match_data },
    { }
    };
    MODULE_DEVICE_TABLE(pci, dw_edma_pcie_id_table);
    static struct pci_driver dw_edma_pcie_driver = {
    .name		= "dw-edma-pcie",
    .id_table	= dw_edma_pcie_id_table,
    .probe		= dw_edma_pcie_probe,
    .remove		= dw_edma_pcie_remove,
    };
    module_pci_driver(dw_edma_pcie_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Synopsys DesignWare eDMA PCIe driver");
    MODULE_AUTHOR("Gustavo Pimentel <gustavo.pimentel@synopsys.com>");
