//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/layouts/sl28vpd.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sl28vpd_header {
    pub magic: u8,
    pub version: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sl28vpd_v1 {
    pub header: sl28vpd_header,
    pub serial_number: [c_char; 15],
    pub base_mac_address: [u8; ETH_ALEN],
    pub crc8: u8,
    pub __packed: },
    static int sl28vpd_mac_address_pp(void *priv, const char *id, int index,
    unsigned int offset, void *buf,
    size_t bytes)
    {
    if (bytes != ETH_ALEN)
    pub -EINVAL: return,
    if (index < 0)
    pub -EINVAL: return,
    if (!is_valid_ether_addr(buf))
    pub -EINVAL: return,
    pub index): eth_addr_add(buf,,
    pub 0: return,
    }
    static const struct nvmem_cell_info sl28vpd_v1_entries[] = {
    {
    .name = "serial-number",
    .offset = offsetof(struct sl28vpd_v1, serial_number),
    .bytes = sizeof_field(struct sl28vpd_v1, serial_number),
    },
    {
    .name = "base-mac-address",
    .offset = offsetof(struct sl28vpd_v1, base_mac_address),
    .bytes = sizeof_field(struct sl28vpd_v1, base_mac_address),
    .read_post_process = sl28vpd_mac_address_pp,
    },
}

#[no_mangle]
unsafe extern "C" fn sl28vpd_v1_check_crc(dev: *mut device, nvmem: *mut nvmem_device) -> c_int {
    static int sl28vpd_v1_check_crc(struct device *dev, struct nvmem_device *nvmem)
    {
    struct sl28vpd_v1 data_v1;
    u8 table[CRC8_TABLE_SIZE];
    int ret;
    u8 crc;
    crc8_populate_msb(table, 0x07);
    ret = nvmem_device_read(nvmem, 0, sizeof(data_v1), &data_v1);
    if (ret < 0)
    return ret;
#[no_mangle]
pub unsafe extern "C" fn if(sizeof(data_v1): ret !=) -> else {
    else if (ret != sizeof(data_v1))
    return -EIO;
    crc = crc8(table, (void *)&data_v1, sizeof(data_v1) - 1, 0);
    if (crc != data_v1.crc8) {
    dev_err(dev,
    "Checksum is invalid (got %02x, expected %02x).\n",
    crc, data_v1.crc8);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sl28vpd_add_cells(layout: *mut nvmem_layout) -> c_int {
    static int sl28vpd_add_cells(struct nvmem_layout *layout)
    {
    struct nvmem_device *nvmem = layout.nvmem;
    struct device *dev = &layout.dev;
    const struct nvmem_cell_info *pinfo;
    let mut info: nvmem_cell_info = {0};
    struct device_node *layout_np;
    struct sl28vpd_header hdr;
    int ret, i;
// check header
    ret = nvmem_device_read(nvmem, 0, sizeof(hdr), &hdr);
    if (ret < 0)
    return ret;
#[no_mangle]
pub unsafe extern "C" fn if(sizeof(hdr): ret !=) -> else {
    else if (ret != sizeof(hdr))
    return -EIO;
    if (hdr.magic != SL28VPD_MAGIC) {
    dev_err(dev, "Invalid magic value (%02x)\n", hdr.magic);
    return -EINVAL;
    }
    if (hdr.version != 1) {
    dev_err(dev, "Version %d is unsupported.\n", hdr.version);
    return -EINVAL;
    }
    ret = sl28vpd_v1_check_crc(dev, nvmem);
    if (ret)
    return ret;
    layout_np = of_nvmem_layout_get_container(nvmem);
    if (!layout_np)
    return -ENOENT;
    for (i = 0; i < ARRAY_SIZE(sl28vpd_v1_entries); i++) {
    pinfo = &sl28vpd_v1_entries[i];
    info.name = pinfo.name;
    info.offset = pinfo.offset;
    info.bytes = pinfo.bytes;
    info.read_post_process = pinfo.read_post_process;
    info.np = of_get_child_by_name(layout_np, pinfo.name);
    ret = nvmem_add_one_cell(nvmem, &info);
    if (ret) {
    of_node_put(layout_np);
    return ret;
    }
    }
    of_node_put(layout_np);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sl28vpd_probe(layout: *mut nvmem_layout) -> c_int {
    static int sl28vpd_probe(struct nvmem_layout *layout)
    {
    layout.add_cells = sl28vpd_add_cells;
    return nvmem_layout_register(layout);
    }
#[no_mangle]
unsafe extern "C" fn sl28vpd_remove(layout: *mut nvmem_layout) {
    static void sl28vpd_remove(struct nvmem_layout *layout)
    {
    nvmem_layout_unregister(layout);
    }
    static const struct of_device_id sl28vpd_of_match_table[] = {
    { .compatible = "kontron,sl28-vpd" },
    {},
    };
    MODULE_DEVICE_TABLE(of, sl28vpd_of_match_table);
    static struct nvmem_layout_driver sl28vpd_layout = {
    .driver = {
    .name = "kontron-sl28vpd-layout",
    .of_match_table = sl28vpd_of_match_table,
    },
    .probe = sl28vpd_probe,
    .remove = sl28vpd_remove,
    };
    module_nvmem_layout_driver(sl28vpd_layout);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Michael Walle <michael@walle.cc>");
    MODULE_DESCRIPTION("NVMEM layout driver for the VPD of Kontron sl28 boards");
