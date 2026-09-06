//! Automatically rewritten from C to Rust
//! Source: drivers/cxl/core/atl.c
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
// Copyright (C) 2025 Advanced Micro Devices, Inc.
//

//
// PRM Address Translation - CXL DPA to System Physical Address
//
// Reference:
//
// AMD Family 1Ah Models 00h–0Fh and Models 10h–1Fh
// ACPI v6.5 Porting Guide, Publication # 58088
//
    static const guid_t prm_cxl_dpa_spa_guid =
    GUID_INIT(0xee41b397, 0x25d4, 0x452c, 0xad, 0x54, 0x48, 0xc6, 0xe3,
    0x48, 0x0b, 0x94);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prm_cxl_dpa_spa_data {
    pub dpa: u64,
    pub reserved: u8,
    pub devfn: u8,
    pub bus: u8,
    pub segment: u8,
    pub spa: *mut u64,
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn prm_cxl_dpa_spa(pci_dev: *mut pci_dev, dpa: u64) -> u64 {
    static u64 prm_cxl_dpa_spa(struct pci_dev *pci_dev, u64 dpa)
    {
    pub data: prm_cxl_dpa_spa_data,
    pub spa: u64,
    pub rc: c_int,
    data = (struct prm_cxl_dpa_spa_data) {
    .dpa     = dpa,
    .devfn   = pci_dev.devfn,
    .bus     = pci_dev.bus.number,
    .segment = pci_domain_nr(pci_dev.bus),
    .spa     = &spa,
}

    rc = acpi_call_prm_handler(prm_cxl_dpa_spa_guid, &data);
    if (rc) {
    pci_dbg(pci_dev, "failed to get SPA for %#llx: %d\n", dpa, rc);
    return ULLONG_MAX;
    }
    pci_dbg(pci_dev, "PRM address translation: DPA . SPA: %#llx . %#llx\n", dpa, spa);
    return spa;
    }
#[no_mangle]
unsafe extern "C" fn cxl_prm_setup_root(cxl_root: *mut cxl_root, data: *mut c_void) -> c_int {
    static int cxl_prm_setup_root(struct cxl_root *cxl_root, void *data)
    {
    struct cxl_region_context *ctx = data;
    struct cxl_endpoint_decoder *cxled = ctx.cxled;
    struct cxl_decoder *cxld = &cxled.cxld;
    struct cxl_memdev *cxlmd = cxled_to_memdev(cxled);
    let mut hpa_range: range = ctx.hpa_range;
    struct pci_dev *pci_dev;
    u64 spa_len, len;
    u64 addr, base_spa, base;
    int ways, gran;
//
// When Normalized Addressing is enabled, the endpoint maintains a 1:1
// mapping between HPA and DPA. If disabled, skip address translation
// and perform only a range check.
//
    if (hpa_range.start != cxled.dpa_res.start)
    return 0;
//
// Endpoints are programmed passthrough in Normalized Addressing mode.
//
    if (ctx.interleave_ways != 1) {
    dev_dbg(&cxld.dev, "unexpected interleaving config: ways: %d granularity: %d\n",
    ctx.interleave_ways, ctx.interleave_granularity);
    return -ENXIO;
    }
    if (!cxlmd || !dev_is_pci(cxlmd.dev.parent)) {
    dev_dbg(&cxld.dev, "No endpoint found: %s, range %#llx-%#llx\n",
    dev_name(cxld.dev.parent), hpa_range.start,
    hpa_range.end);
    return -ENXIO;
    }
    pci_dev = to_pci_dev(cxlmd.dev.parent);
// Translate HPA range to SPA.
    base = hpa_range.start;
    hpa_range.start = prm_cxl_dpa_spa(pci_dev, hpa_range.start);
    hpa_range.end = prm_cxl_dpa_spa(pci_dev, hpa_range.end);
    base_spa = hpa_range.start;
    if (hpa_range.start == ULLONG_MAX || hpa_range.end == ULLONG_MAX) {
    dev_dbg(cxld.dev.parent,
    "CXL address translation: Failed to translate HPA range: %#llx-%#llx:%#llx-%#llx(%s)\n",
    hpa_range.start, hpa_range.end, ctx.hpa_range.start,
    ctx.hpa_range.end, dev_name(&cxld.dev));
    return -ENXIO;
    }
//
// Since translated addresses include the interleaving offsets, align
// the range to 256 MB.
//
    hpa_range.start = ALIGN_DOWN(hpa_range.start, SZ_256M);
    hpa_range.end = ALIGN(hpa_range.end, SZ_256M) - 1;
    len = range_len(&ctx.hpa_range);
    spa_len = range_len(&hpa_range);
    if (!len || !spa_len || spa_len % len) {
    dev_dbg(cxld.dev.parent,
    "CXL address translation: HPA range not contiguous: %#llx-%#llx:%#llx-%#llx(%s)\n",
    hpa_range.start, hpa_range.end, ctx.hpa_range.start,
    ctx.hpa_range.end, dev_name(&cxld.dev));
    return -ENXIO;
    }
    ways = spa_len / len;
    gran = SZ_256;
//
// Determine interleave granularity
//
// Note: The position of the chunk from one interleaving block to the
// next may vary and thus cannot be considered constant. Address offsets
// larger than the interleaving block size cannot be used to calculate
// the granularity.
//
    if (ways > 1) {
    while (gran <= SZ_16M) {
    addr = prm_cxl_dpa_spa(pci_dev, base + gran);
    if (addr != base_spa + gran)
    break;
    gran <<= 1;
    }
    }
    if (gran > SZ_16M) {
    dev_dbg(cxld.dev.parent,
    "CXL address translation: Cannot determine granularity: %#llx-%#llx:%#llx-%#llx(%s)\n",
    hpa_range.start, hpa_range.end, ctx.hpa_range.start,
    ctx.hpa_range.end, dev_name(&cxld.dev));
    return -ENXIO;
    }
//
// The current kernel implementation does not support endpoint
// setup with Normalized Addressing. It only translates an
// endpoint's DPA to the SPA range of the host bridge.
// Therefore, the endpoint address range cannot be determined,
// making a non-auto setup impossible. If a decoder requires
// address translation, reprogramming should be disabled and
// the decoder locked.
//
// The BIOS, however, provides all the necessary address
// translation data, which the kernel can use to reconfigure
// endpoint decoders with normalized addresses. Locking the
// decoders in the BIOS would prevent a capable kernel (or
// other operating systems) from shutting down auto-generated
// regions and managing resources dynamically.
//
// Indicate that Normalized Addressing is enabled.
//
    cxld.flags |= CXL_DECODER_F_LOCK;
    cxld.flags |= CXL_DECODER_F_NORMALIZED_ADDRESSING;
    ctx.hpa_range = hpa_range;
    ctx.interleave_ways = ways;
    ctx.interleave_granularity = gran;
    dev_dbg(&cxld.dev,
    "address mapping found for %s (hpa . spa): %#llx+%#llx . %#llx+%#llx ways:%d granularity:%d\n",
    dev_name(cxlmd.dev.parent), base, len, hpa_range.start,
    spa_len, ways, gran);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cxl_setup_prm_address_translation(cxl_root: *mut cxl_root) {
    void cxl_setup_prm_address_translation(struct cxl_root *cxl_root)
    {
    struct device *host = cxl_root.port.uport_dev;
    u64 spa;
    let mut data: prm_cxl_dpa_spa_data = { .spa = &spa };
    int rc;
//
// Applies only to PCIe Host Bridges which are children of the CXL Root
// Device (HID=“ACPI0017”). Check this and drop cxl_test instances.
//
    if (!acpi_match_device(host.driver.acpi_match_table, host))
    return;
// Check kernel (-EOPNOTSUPP) and firmware support (-ENODEV)
    rc = acpi_call_prm_handler(prm_cxl_dpa_spa_guid, &data);
    if (rc == -EOPNOTSUPP || rc == -ENODEV)
    return;
    cxl_root.ops.translation_setup_root = cxl_prm_setup_root;
    }
    EXPORT_SYMBOL_NS_GPL(cxl_setup_prm_address_translation, "CXL");
