//! Automatically rewritten from C to Rust
//! Source: drivers/accel/habanalabs/common/pci/pci.c
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
// Copyright 2016-2019 HabanaLabs, Ltd.
// All Rights Reserved.
//

//
// hl_pci_bars_map() - Map PCI BARs.
// @hdev: Pointer to hl_device structure.
// @name: Array of BAR names.
// @is_wc: Array with flag per BAR whether a write-combined mapping is needed.
//
// Request PCI regions and map them to kernel virtual addresses.
//
// Return: 0 on success, non-zero for failure.
//
    int hl_pci_bars_map(struct hl_device *hdev, const char * const name[3],
    bool is_wc[3])
    {
    struct pci_dev *pdev = hdev.pdev;
    int rc, i, bar;
    rc = pci_request_regions(pdev, HL_NAME);
    if (rc) {
    dev_err(hdev.dev, "Cannot obtain PCI resources\n");
    return rc;
    }
    for (i = 0 ; i < 3 ; i++) {
    bar = i * 2; /* 64-bit BARs */
    hdev.pcie_bar[bar] = is_wc[i] ?
    pci_ioremap_wc_bar(pdev, bar) :
    pci_ioremap_bar(pdev, bar);
    if (!hdev.pcie_bar[bar]) {
    dev_err(hdev.dev, "pci_ioremap%s_bar failed for %s\n",
    is_wc[i] ? "_wc" : "", name[i]);
    rc = -ENODEV;
    goto err;
    }
    }
    return 0;
    err:
    for (i = 2 ; i >= 0 ; i--) {
    bar = i * 2; /* 64-bit BARs */
    if (hdev.pcie_bar[bar])
    iounmap(hdev.pcie_bar[bar]);
    }
    pci_release_regions(pdev);
    return rc;
    }
//
// hl_pci_bars_unmap() - Unmap PCI BARS.
// @hdev: Pointer to hl_device structure.
//
// Release all PCI BARs and unmap their virtual addresses.
//
#[no_mangle]
unsafe extern "C" fn hl_pci_bars_unmap(hdev: *mut hl_device) {
    static void hl_pci_bars_unmap(struct hl_device *hdev)
    {
    struct pci_dev *pdev = hdev.pdev;
    int i, bar;
    for (i = 2 ; i >= 0 ; i--) {
    bar = i * 2; /* 64-bit BARs */
    iounmap(hdev.pcie_bar[bar]);
    }
    pci_release_regions(pdev);
    }
#[no_mangle]
pub unsafe extern "C" fn hl_pci_elbi_read(hdev: *mut hl_device, addr: u64, data: *mut u32) -> c_int {
    int hl_pci_elbi_read(struct hl_device *hdev, u64 addr, u32 *data)
    {
    struct pci_dev *pdev = hdev.pdev;
    ktime_t timeout;
    u64 msec;
    u32 val;
    if (hdev.pldm)
    msec = HL_PLDM_PCI_ELBI_TIMEOUT_MSEC;
    else
    msec = HL_PCI_ELBI_TIMEOUT_MSEC;
// Clear previous status
    pci_write_config_dword(pdev, mmPCI_CONFIG_ELBI_STS, 0);
    pci_write_config_dword(pdev, mmPCI_CONFIG_ELBI_ADDR, (u32) addr);
    pci_write_config_dword(pdev, mmPCI_CONFIG_ELBI_CTRL, 0);
    timeout = ktime_add_ms(ktime_get(), msec);
    for (;;) {
    pci_read_config_dword(pdev, mmPCI_CONFIG_ELBI_STS, &val);
    if (val & PCI_CONFIG_ELBI_STS_MASK)
    break;
    if (ktime_compare(ktime_get(), timeout) > 0) {
    pci_read_config_dword(pdev, mmPCI_CONFIG_ELBI_STS,
    &val);
    break;
    }
    usleep_range(300, 500);
    }
    if ((val & PCI_CONFIG_ELBI_STS_MASK) == PCI_CONFIG_ELBI_STS_DONE) {
    pci_read_config_dword(pdev, mmPCI_CONFIG_ELBI_DATA, data);
    if (unlikely(trace_habanalabs_elbi_read_enabled()))
    trace_habanalabs_elbi_read(&hdev.pdev.dev, (u32) addr, val);
    return 0;
    }
    if (val & PCI_CONFIG_ELBI_STS_ERR) {
    dev_err(hdev.dev, "Error reading from ELBI\n");
    return -EIO;
    }
    if (!(val & PCI_CONFIG_ELBI_STS_MASK)) {
    dev_err(hdev.dev, "ELBI read didn't finish in time\n");
    return -EIO;
    }
    dev_err(hdev.dev, "ELBI read has undefined bits in status\n");
    return -EIO;
    }
//
// hl_pci_elbi_write() - Write through the ELBI interface.
// @hdev: Pointer to hl_device structure.
// @addr: Address to write to
// @data: Data to write
//
// Return: 0 on success, negative value for failure.
//
#[no_mangle]
unsafe extern "C" fn hl_pci_elbi_write(hdev: *mut hl_device, addr: u64, data: u32) -> c_int {
    static int hl_pci_elbi_write(struct hl_device *hdev, u64 addr, u32 data)
    {
    struct pci_dev *pdev = hdev.pdev;
    ktime_t timeout;
    u64 msec;
    u32 val;
    if (hdev.pldm)
    msec = HL_PLDM_PCI_ELBI_TIMEOUT_MSEC;
    else
    msec = HL_PCI_ELBI_TIMEOUT_MSEC;
// Clear previous status
    pci_write_config_dword(pdev, mmPCI_CONFIG_ELBI_STS, 0);
    pci_write_config_dword(pdev, mmPCI_CONFIG_ELBI_ADDR, (u32) addr);
    pci_write_config_dword(pdev, mmPCI_CONFIG_ELBI_DATA, data);
    pci_write_config_dword(pdev, mmPCI_CONFIG_ELBI_CTRL,
    PCI_CONFIG_ELBI_CTRL_WRITE);
    timeout = ktime_add_ms(ktime_get(), msec);
    for (;;) {
    pci_read_config_dword(pdev, mmPCI_CONFIG_ELBI_STS, &val);
    if (val & PCI_CONFIG_ELBI_STS_MASK)
    break;
    if (ktime_compare(ktime_get(), timeout) > 0) {
    pci_read_config_dword(pdev, mmPCI_CONFIG_ELBI_STS,
    &val);
    break;
    }
    usleep_range(300, 500);
    }
    if ((val & PCI_CONFIG_ELBI_STS_MASK) == PCI_CONFIG_ELBI_STS_DONE) {
    if (unlikely(trace_habanalabs_elbi_write_enabled()))
    trace_habanalabs_elbi_write(&hdev.pdev.dev, (u32) addr, val);
    return 0;
    }
    if (val & PCI_CONFIG_ELBI_STS_ERR)
    return -EIO;
    if (!(val & PCI_CONFIG_ELBI_STS_MASK)) {
    dev_err(hdev.dev, "ELBI write didn't finish in time\n");
    return -EIO;
    }
    dev_err(hdev.dev, "ELBI write has undefined bits in status\n");
    return -EIO;
    }
//
// hl_pci_iatu_write() - iatu write routine.
// @hdev: Pointer to hl_device structure.
// @addr: Address to write to
// @data: Data to write
//
// Return: 0 on success, negative value for failure.
//
#[no_mangle]
pub unsafe extern "C" fn hl_pci_iatu_write(hdev: *mut hl_device, addr: u32, data: u32) -> c_int {
    int hl_pci_iatu_write(struct hl_device *hdev, u32 addr, u32 data)
    {
    struct asic_fixed_properties *prop = &hdev.asic_prop;
    u32 dbi_offset;
    int rc;
    dbi_offset = addr & 0xFFF;
// Ignore result of writing to pcie_aux_dbi_reg_addr as it could fail
// in case the firmware security is enabled
//
    hl_pci_elbi_write(hdev, prop.pcie_aux_dbi_reg_addr, 0x00300000);
    rc = hl_pci_elbi_write(hdev, prop.pcie_dbi_base_address + dbi_offset,
    data);
    if (rc)
    return -EIO;
    return 0;
    }
//
// hl_pci_set_inbound_region() - Configure inbound region
// @hdev: Pointer to hl_device structure.
// @region: Inbound region number.
// @pci_region: Inbound region parameters.
//
// Configure the iATU inbound region.
//
// Return: 0 on success, negative value for failure.
//
    int hl_pci_set_inbound_region(struct hl_device *hdev, u8 region,
    struct hl_inbound_pci_region *pci_region)
    {
    struct asic_fixed_properties *prop = &hdev.asic_prop;
    u64 bar_phys_base, region_base, region_end_address;
    u32 offset, ctrl_reg_val;
    let mut rc: c_int = 0;
// region offset
    offset = (0x200 * region) + 0x100;
    if (pci_region.mode == PCI_ADDRESS_MATCH_MODE) {
    bar_phys_base = hdev.pcie_bar_phys[pci_region.bar];
    region_base = bar_phys_base + pci_region.offset_in_bar;
    region_end_address = region_base + pci_region.size - 1;
    rc |= hl_pci_iatu_write(hdev, offset + 0x8,
    lower_32_bits(region_base));
    rc |= hl_pci_iatu_write(hdev, offset + 0xC,
    upper_32_bits(region_base));
    rc |= hl_pci_iatu_write(hdev, offset + 0x10,
    lower_32_bits(region_end_address));
    }
// Point to the specified address
    rc |= hl_pci_iatu_write(hdev, offset + 0x14, lower_32_bits(pci_region.addr));
    rc |= hl_pci_iatu_write(hdev, offset + 0x18, upper_32_bits(pci_region.addr));
// Set bar type as memory
    rc |= hl_pci_iatu_write(hdev, offset + 0x0, 0);
// Enable + bar/address match + match enable + bar number
    ctrl_reg_val = FIELD_PREP(IATU_REGION_CTRL_REGION_EN_MASK, 1);
    ctrl_reg_val |= FIELD_PREP(IATU_REGION_CTRL_MATCH_MODE_MASK, pci_region.mode);
    ctrl_reg_val |= FIELD_PREP(IATU_REGION_CTRL_NUM_MATCH_EN_MASK, 1);
    if (pci_region.mode == PCI_BAR_MATCH_MODE)
    ctrl_reg_val |= FIELD_PREP(IATU_REGION_CTRL_BAR_NUM_MASK, pci_region.bar);
    rc |= hl_pci_iatu_write(hdev, offset + 0x4, ctrl_reg_val);
// Return the DBI window to the default location
// Ignore result of writing to pcie_aux_dbi_reg_addr as it could fail
// in case the firmware security is enabled
//
    hl_pci_elbi_write(hdev, prop.pcie_aux_dbi_reg_addr, 0);
    if (rc)
    dev_err(hdev.dev, "failed to map bar %u to 0x%08llx\n",
    pci_region.bar, pci_region.addr);
    return rc;
    }
//
// hl_pci_set_outbound_region() - Configure outbound region 0
// @hdev: Pointer to hl_device structure.
// @pci_region: Outbound region parameters.
//
// Configure the iATU outbound region 0.
//
// Return: 0 on success, negative value for failure.
//
    int hl_pci_set_outbound_region(struct hl_device *hdev,
    struct hl_outbound_pci_region *pci_region)
    {
    struct asic_fixed_properties *prop = &hdev.asic_prop;
    u64 outbound_region_end_address;
    let mut rc: c_int = 0;
// Outbound Region 0
    outbound_region_end_address =
    pci_region.addr + pci_region.size - 1;
    rc |= hl_pci_iatu_write(hdev, 0x008,
    lower_32_bits(pci_region.addr));
    rc |= hl_pci_iatu_write(hdev, 0x00C,
    upper_32_bits(pci_region.addr));
    rc |= hl_pci_iatu_write(hdev, 0x010,
    lower_32_bits(outbound_region_end_address));
    rc |= hl_pci_iatu_write(hdev, 0x014, 0);
    rc |= hl_pci_iatu_write(hdev, 0x018, 0);
    rc |= hl_pci_iatu_write(hdev, 0x020,
    upper_32_bits(outbound_region_end_address));
// Increase region size
    rc |= hl_pci_iatu_write(hdev, 0x000, 0x00002000);
// Enable
    rc |= hl_pci_iatu_write(hdev, 0x004, 0x80000000);
// Return the DBI window to the default location
// Ignore result of writing to pcie_aux_dbi_reg_addr as it could fail
// in case the firmware security is enabled
//
    hl_pci_elbi_write(hdev, prop.pcie_aux_dbi_reg_addr, 0);
    return rc;
    }
//
// hl_get_pci_memory_region() - get PCI region for given address
// @hdev: Pointer to hl_device structure.
// @addr: device address
//
// @return region index on success, otherwise PCI_REGION_NUMBER (invalid
// region index)
//
#[no_mangle]
pub unsafe extern "C" fn hl_get_pci_memory_region(hdev: *mut hl_device, addr: u64) -> enum pci_region {
    enum pci_region hl_get_pci_memory_region(struct hl_device *hdev, u64 addr)
    {
    int i;
    for  (i = 0 ; i < PCI_REGION_NUMBER ; i++) {
    struct pci_mem_region *region = &hdev.pci_mem_region[i];
    if (!region.used)
    continue;
    if ((addr >= region.region_base) &&
    (addr < region.region_base + region.region_size))
    return i;
    }
    return PCI_REGION_NUMBER;
    }
//
// hl_pci_init() - PCI initialization code.
// @hdev: Pointer to hl_device structure.
//
// Set DMA masks, initialize the PCI controller and map the PCI BARs.
//
// Return: 0 on success, non-zero for failure.
//
#[no_mangle]
pub unsafe extern "C" fn hl_pci_init(hdev: *mut hl_device) -> c_int {
    int hl_pci_init(struct hl_device *hdev)
    {
    struct asic_fixed_properties *prop = &hdev.asic_prop;
    struct pci_dev *pdev = hdev.pdev;
    int rc;
    rc = pci_enable_device_mem(pdev);
    if (rc) {
    dev_err(hdev.dev, "can't enable PCI device\n");
    return rc;
    }
    pci_set_master(pdev);
    rc = hdev.asic_funcs.pci_bars_map(hdev);
    if (rc) {
    dev_err(hdev.dev, "Failed to map PCI BAR addresses\n");
    goto disable_device;
    }
    rc = hdev.asic_funcs.init_iatu(hdev);
    if (rc) {
    dev_err(hdev.dev, "PCI controller was not initialized successfully\n");
    goto unmap_pci_bars;
    }
// Driver must sleep in order for FW to finish the iATU configuration
    if (hdev.asic_prop.iatu_done_by_fw)
    usleep_range(2000, 3000);
    rc = dma_set_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(prop.dma_mask));
    if (rc) {
    dev_err(hdev.dev,
    "Failed to set dma mask to %d bits, error %d\n",
    prop.dma_mask, rc);
    goto unmap_pci_bars;
    }
    dma_set_max_seg_size(&pdev.dev, U32_MAX);
    return 0;
    unmap_pci_bars:
    hl_pci_bars_unmap(hdev);
    disable_device:
    pci_disable_device(pdev);
    return rc;
    }
//
// hl_pci_fini() - PCI finalization code.
// @hdev: Pointer to hl_device structure
//
// Unmap PCI bars and disable PCI device.
//
#[no_mangle]
pub unsafe extern "C" fn hl_pci_fini(hdev: *mut hl_device) {
    void hl_pci_fini(struct hl_device *hdev)
    {
    hl_pci_bars_unmap(hdev);
    pci_disable_device(hdev.pdev);
    }
