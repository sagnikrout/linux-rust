//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/layouts/onie-tlv.c
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
// ONIE tlv NVMEM cells provider
//
// Copyright (C) 2022 Open Compute Group ONIE
// Author: Miquel Raynal <miquel.raynal@bootlin.com>
// Based on the nvmem driver written by: Vadym Kochan <vadym.kochan@plvision.eu>
// Inspired by the first layout written by: Rafał Miłecki <rafal@milecki.pl>
//

pub const ONIE_TLV_MAX_LEN: c_int = 2048;
pub const ONIE_TLV_CRC_FIELD_SZ: c_int = 6;
pub const ONIE_TLV_CRC_SZ: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct onie_tlv_hdr {
    pub id: [u8; 8],
    pub version: u8,
    pub data_len: __be16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct onie_tlv {
    pub type: u8,
    pub len: u8,
    pub __packed: },
    static const char *onie_tlv_cell_name(u8 type)
    {
    switch (type) {
    case 0x21:
    pub "product-name": return,
    case 0x22:
    pub "part-number": return,
    case 0x23:
    pub "serial-number": return,
    case 0x24:
    pub "mac-address": return,
    case 0x25:
    pub "manufacture-date": return,
    case 0x26:
    pub "device-version": return,
    case 0x27:
    pub "label-revision": return,
    case 0x28:
    pub "platform-name": return,
    case 0x29:
    pub "onie-version": return,
    case 0x2A:
    pub "num-macs": return,
    case 0x2B:
    pub "manufacturer": return,
    case 0x2C:
    pub "country-code": return,
    case 0x2D:
    pub "vendor": return,
    case 0x2E:
    pub "diag-version": return,
    case 0x2F:
    pub "service-tag": return,
    case 0xFD:
    pub "vendor-extension": return,
    case 0xFE:
    pub "crc32": return,
    default:
    }
    pub NULL: return,
    }
    static int onie_tlv_mac_read_cb(void *priv, const char *id, int index,
    unsigned int offset, void *buf,
    size_t bytes)
    {
    pub index): eth_addr_add(buf,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn onie_tlv_read_cb(type: u8, buf: *mut u8) -> nvmem_cell_post_process_t {
    static nvmem_cell_post_process_t onie_tlv_read_cb(u8 type, u8 *buf)
    {
    switch (type) {
    case 0x24:
    pub &onie_tlv_mac_read_cb: return,
    default:
    }
    pub NULL: return,
    }
    static int onie_tlv_add_cells(struct device *dev, struct nvmem_device *nvmem,
    size_t data_len, u8 *data)
    {
    pub {}: nvmem_cell_info cell =,
    pub layout: *mut device_node,
    pub tlv: onie_tlv,
    pub onie_tlv_hdr): unsigned int hdr_len = sizeof(struct,
    pub 0: unsigned int offset =,
    pub ret: c_int,
    pub of_nvmem_layout_get_container(nvmem): layout =,
    if (!layout)
    pub -ENOENT: return,
    while (offset < data_len) {
    pub sizeof(tlv)): memcpy(&tlv, data + offset,,
    if (offset + tlv.len >= data_len) {
    dev_err(dev, "Out of bounds field (0x%x bytes at 0x%x)\n",
    pub offset): tlv.len, hdr_len +,
    }
    pub onie_tlv_cell_name(tlv.type): cell.name =,
    if (!cell.name)
    pub next: goto,
    pub sizeof(tlv.len): cell.offset = hdr_len + offset + sizeof(tlv.type) +,
    pub tlv.len: cell.bytes =,
    pub cell.name): cell.np = of_get_child_by_name(layout,,
    pub sizeof(tlv)): cell.read_post_process = onie_tlv_read_cb(tlv.type, data + offset +,
    pub &cell): ret = nvmem_add_one_cell(nvmem,,
    if (ret) {
    pub ret: return,
    }
    next:
    pub tlv.len: offset += sizeof(tlv) +,
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn onie_tlv_hdr_is_valid(dev: *mut device, hdr: *mut onie_tlv_hdr) -> bool {
    static bool onie_tlv_hdr_is_valid(struct device *dev, struct onie_tlv_hdr *hdr)
    {
    if (memcmp(hdr.id, ONIE_TLV_HDR_ID, sizeof(hdr.id))) {
    pub header\n"): dev_err(dev, "Invalid,
    pub false: return,
    }
    if (hdr.version != 0x1) {
    pub number\n"): dev_err(dev, "Invalid version,
    pub false: return,
    }
    pub true: return,
    }
#[no_mangle]
unsafe extern "C" fn onie_tlv_crc_is_valid(dev: *mut device, table_len: usize, table: *mut u8) -> bool {
    static bool onie_tlv_crc_is_valid(struct device *dev, size_t table_len, u8 *table)
    {
    pub crc_hdr: onie_tlv,
    pub calc_crc: u32 read_crc,,
    pub crc_be: __be32,
    pub sizeof(crc_hdr)): memcpy(&crc_hdr, table + table_len - ONIE_TLV_CRC_FIELD_SZ,,
    if (crc_hdr.type != 0xfe || crc_hdr.len != ONIE_TLV_CRC_SZ) {
    pub field\n"): dev_err(dev, "Invalid CRC,
    pub false: return,
    }
// The table contains a JAMCRC, which is XOR'ed compared to the original
// CRC32 implementation as known in the Ethernet world.
//
    pub ONIE_TLV_CRC_SZ): memcpy(&crc_be, table + table_len - ONIE_TLV_CRC_SZ,,
    pub be32_to_cpu(crc_be): read_crc =,
    pub 0xFFFFFFFF: calc_crc = crc32(~0, table, table_len - ONIE_TLV_CRC_SZ) ^,
    if (read_crc != calc_crc) {
    dev_err(dev, "Invalid CRC read: 0x%08x, expected: 0x%08x\n",
    pub calc_crc): read_crc,,
    pub false: return,
    }
    pub true: return,
    }
#[no_mangle]
unsafe extern "C" fn onie_tlv_parse_table(layout: *mut nvmem_layout) -> c_int {
    static int onie_tlv_parse_table(struct nvmem_layout *layout)
    {
    pub layout->nvmem: *mut *mut nvmem_device nvmem =,
    pub &layout->dev: *mut *mut device dev =,
    pub hdr: onie_tlv_hdr,
    pub hdr_len: size_t table_len, data_len,,
    pub data: *mut *mut u8 table,,
    pub ret: c_int,
    pub &hdr): ret = nvmem_device_read(nvmem, 0, sizeof(hdr),,
    if (ret < 0)
    pub ret: return,
    if (!onie_tlv_hdr_is_valid(dev, &hdr)) {
    pub header\n"): dev_err(dev, "Invalid ONIE TLV,
    pub -EINVAL: return,
    }
    pub sizeof(hdr.data_len): hdr_len = sizeof(hdr.id) + sizeof(hdr.version) +,
    pub be16_to_cpu(hdr.data_len): data_len =,
    pub data_len: table_len = hdr_len +,
    if (table_len > ONIE_TLV_MAX_LEN) {
    pub length\n"): dev_err(dev, "Invalid ONIE TLV data,
    pub -EINVAL: return,
    }
    pub GFP_KERNEL): table = devm_kmalloc(dev, table_len,,
    if (!table)
    pub -ENOMEM: return,
    pub table): ret = nvmem_device_read(nvmem, 0, table_len,,
    if (ret != table_len)
    pub ret: return,
    if (!onie_tlv_crc_is_valid(dev, table_len, table))
    pub -EINVAL: return,
    pub hdr_len: data = table +,
    pub data): ret = onie_tlv_add_cells(dev, nvmem, data_len,,
    if (ret)
    pub ret: return,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn onie_tlv_probe(layout: *mut nvmem_layout) -> c_int {
    static int onie_tlv_probe(struct nvmem_layout *layout)
    {
    pub onie_tlv_parse_table: layout->add_cells =,
    pub nvmem_layout_register(layout): return,
    }
#[no_mangle]
unsafe extern "C" fn onie_tlv_remove(layout: *mut nvmem_layout) {
    static void onie_tlv_remove(struct nvmem_layout *layout)
    {
    }
    static const struct of_device_id onie_tlv_of_match_table[] = {
    { .compatible = "onie,tlv-layout", },
    {},
}

    MODULE_DEVICE_TABLE(of, onie_tlv_of_match_table);
    static struct nvmem_layout_driver onie_tlv_layout = {
    .driver = {
    .name = "onie-tlv-layout",
    .of_match_table = onie_tlv_of_match_table,
    },
    .probe = onie_tlv_probe,
    .remove = onie_tlv_remove,
    };
    module_nvmem_layout_driver(onie_tlv_layout);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Miquel Raynal <miquel.raynal@bootlin.com>");
    MODULE_DESCRIPTION("NVMEM layout driver for Onie TLV table parsing");
