//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/rmem.c
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
//
// Copyright (C) 2020 Nicolas Saenz Julienne <nsaenzjulienne@suse.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmem {
    pub dev: *mut device,
    pub nvmem: *mut nvmem_device,
    pub mem: *mut reserved_mem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmem_match_data {
    pub priv): *mut *mut int (checksum)(struct rmem,
}

    struct __packed rmem_eyeq5_header {
    u32 magic;
    u32 version;
    u32 size;
    };

    static int rmem_read(void *context, unsigned int offset,
    void *val, size_t bytes)
    {
    struct rmem *priv = context;
    void *addr;
    if ((phys_addr_t)offset + bytes > priv.mem.size)
    return -EIO;
//
// Only map the reserved memory at this point to avoid potential rogue
// kernel threads inadvertently modifying it. Based on the current
// uses-cases for this driver, the performance hit isn't a concern.
// Nor is likely to be, given the nature of the subsystem. Most nvmem
// devices operate over slow buses to begin with.
//
// An alternative would be setting the memory as RO, set_memory_ro(),
// but as of Dec 2020 this isn't possible on arm64.
//
    addr = memremap(priv.mem.base, priv.mem.size, MEMREMAP_WB);
    if (!addr) {
    dev_err(priv.dev, "Failed to remap memory region\n");
    return -ENOMEM;
    }
    memcpy(val, addr + offset, bytes);
    memunmap(addr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rmem_eyeq5_checksum(priv: *mut rmem) -> c_int {
    static int rmem_eyeq5_checksum(struct rmem *priv)
    {
    void *buf __free(kfree) = core::ptr::null_mut();
    struct rmem_eyeq5_header header;
    u32 computed_crc, *target_crc;
    size_t data_size;
    int ret;
    ret = rmem_read(priv, 0, &header, sizeof(header));
    if (ret)
    return ret;
    if (header.magic != RMEM_EYEQ5_MAGIC)
    return -EINVAL;
//
// Avoid massive kmalloc() if header read is invalid;
// the check would be done by the next rmem_read() anyway.
//
    if (header.size > priv.mem.size)
    return -EINVAL;
//
// 0 +-------------------+
// | Header (12 bytes) | \
// +-------------------+ |
// |                   | | data to be CRCed
// |        ...        | |
// |                   |
// data_size +-------------------+
// |   CRC (4 bytes)   |
// header.size +-------------------+
//
    buf = kmalloc(header.size, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    ret = rmem_read(priv, 0, buf, header.size);
    if (ret)
    return ret;
    data_size = header.size - sizeof(*target_crc);
    target_crc = buf + data_size;
    computed_crc = crc32(U32_MAX, buf, data_size) ^ U32_MAX;
    if (computed_crc == *target_crc)
    return 0;
    dev_err(priv.dev,
    "checksum failed: computed %#x, expected %#x, header (%#x, %#x, %#x)\n",
    computed_crc, *target_crc, header.magic, header.version, header.size);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn rmem_probe(pdev: *mut platform_device) -> c_int {
    static int rmem_probe(struct platform_device *pdev)
    {
    let mut config: nvmem_config = { };
    struct device *dev = &pdev.dev;
    const struct rmem_match_data *match_data = device_get_match_data(dev);
    struct reserved_mem *mem;
    struct rmem *priv;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = dev;
    mem = of_reserved_mem_lookup(dev.of_node);
    if (!mem) {
    dev_err(dev, "Failed to lookup reserved memory\n");
    return -EINVAL;
    }
    priv.mem = mem;
    config.dev = dev;
    config.priv = priv;
    config.name = "rmem";
    config.id = NVMEM_DEVID_AUTO;
    config.size = mem.size;
    config.reg_read = rmem_read;
    if (match_data && match_data.checksum) {
    let mut ret: c_int = match_data.checksum(priv);
    if (ret)
    return ret;
    }
    return PTR_ERR_OR_ZERO(devm_nvmem_register(dev, &config));
    }
    static const struct rmem_match_data rmem_eyeq5_match_data = {
    .checksum = rmem_eyeq5_checksum,
    };
    static const struct of_device_id rmem_match[] = {
    { .compatible = "mobileye,eyeq5-bootloader-config", .data = &rmem_eyeq5_match_data },
    { .compatible = "nvmem-rmem", },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, rmem_match);
    static struct platform_driver rmem_driver = {
    .probe = rmem_probe,
    .driver = {
    .name = "rmem",
    .of_match_table = rmem_match,
    },
    };
    module_platform_driver(rmem_driver);
    MODULE_AUTHOR("Nicolas Saenz Julienne <nsaenzjulienne@suse.de>");
    MODULE_DESCRIPTION("Reserved Memory Based nvmem Driver");
    MODULE_LICENSE("GPL");
