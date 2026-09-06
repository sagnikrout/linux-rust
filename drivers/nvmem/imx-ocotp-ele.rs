//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/imx-ocotp-ele.c
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
// i.MX9 OCOTP fusebox driver
//
// Copyright 2023 NXP
//

    enum fuse_type {
    FUSE_FSB = BIT(0),
    FUSE_ELE = BIT(1),
    FUSE_ECC = BIT(2),
    FUSE_INVALID = -1
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocotp_map_entry {
    pub /: *mut *mut u32 start; / start word,
    pub /: *mut *mut u32 num; / num words,
    pub type: enum fuse_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocotp_devtype_data {
    pub reg_off: u32,
    pub name: *mut c_char,
    pub size: u32,
    pub num_entry: u32,
    pub flag: u32,
    pub reg_read: nvmem_reg_read_t,
    pub entry: [ocotp_map_entry; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_ocotp_priv {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub config: nvmem_config,
    pub lock: mutex,
    pub data: *const ocotp_devtype_data,
}

#[no_mangle]
unsafe extern "C" fn imx_ocotp_fuse_type(context: *mut c_void, index: u32) -> enum fuse_type {
    static enum fuse_type imx_ocotp_fuse_type(void *context, u32 index)
    {
    struct imx_ocotp_priv *priv = context;
    const struct ocotp_devtype_data *data = priv.data;
    u32 start, end;
    int i;
    for (i = 0; i < data.num_entry; i++) {
    start = data.entry[i].start;
    end = data.entry[i].start + data.entry[i].num;
    if (index >= start && index < end)
    return data.entry[i].type;
    }
    return FUSE_INVALID;
    }
#[no_mangle]
unsafe extern "C" fn imx_ocotp_reg_read(context: *mut c_void, offset: c_uint, val: *mut c_void, bytes: usize) -> c_int {
    static int imx_ocotp_reg_read(void *context, unsigned int offset, void *val, size_t bytes)
    {
    struct imx_ocotp_priv *priv = context;
    void __iomem *reg = priv.base + priv.data.reg_off;
    u32 count, index, num_bytes;
    enum fuse_type type;
    u32 *buf;
    void *p;
    int i;
    u8 skipbytes;
    if (offset + bytes > priv.data.size)
    bytes = priv.data.size - offset;
    index = offset >> 2;
    skipbytes = offset - (index << 2);
    num_bytes = round_up(bytes + skipbytes, 4);
    count = num_bytes >> 2;
    p = kzalloc(num_bytes, GFP_KERNEL);
    if (!p)
    return -ENOMEM;
    mutex_lock(&priv.lock);
    buf = p;
    for (i = index; i < (index + count); i++) {
    type = imx_ocotp_fuse_type(context, i);
    if (type == FUSE_INVALID || type == FUSE_ELE) {
// buf++ = 0;
    continue;
    }
    if (type & FUSE_ECC)
// buf++ = readl_relaxed(reg + (i << 2)) & GENMASK(15, 0);
    else
// buf++ = readl_relaxed(reg + (i << 2));
    }
    memcpy(val, ((u8 *)p) + skipbytes, bytes);
    mutex_unlock(&priv.lock);
    kfree(p);
    return 0;
    };
    static int imx_ocotp_cell_pp(void *context, const char *id, int index,
    unsigned int offset, void *data, size_t bytes)
    {
    u8 *buf = data;
    int i;
// Deal with some post processing of nvmem cell data
    if (id && !strcmp(id, "mac-address")) {
    bytes = min(bytes, ETH_ALEN);
    for (i = 0; i < bytes / 2; i++)
    swap(buf[i], buf[bytes - i - 1]);
    }
    return 0;
    }
    static void imx_ocotp_fixup_dt_cell_info(struct nvmem_device *nvmem,
    struct nvmem_cell_info *cell)
    {
    cell.raw_len = round_up(cell.bytes, 4);
    cell.read_post_process = imx_ocotp_cell_pp;
    }
#[no_mangle]
unsafe extern "C" fn imx_ele_ocotp_probe(pdev: *mut platform_device) -> c_int {
    static int imx_ele_ocotp_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct imx_ocotp_priv *priv;
    struct nvmem_device *nvmem;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.data = of_device_get_match_data(dev);
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    priv.config.dev = dev;
    priv.config.name = "ELE-OCOTP";
    priv.config.id = NVMEM_DEVID_AUTO;
    priv.config.owner = THIS_MODULE;
    priv.config.size = priv.data.size;
    priv.config.reg_read = priv.data.reg_read;
    priv.config.word_size = 1;
    priv.config.stride = 1;
    priv.config.priv = priv;
    priv.config.read_only = true;
    priv.config.add_legacy_fixed_of_cells = true;
    priv.config.fixup_dt_cell_info = imx_ocotp_fixup_dt_cell_info;
    mutex_init(&priv.lock);
    nvmem = devm_nvmem_register(dev, &priv.config);
    if (IS_ERR(nvmem))
    return PTR_ERR(nvmem);
    return 0;
    }
    static const struct ocotp_devtype_data imx93_ocotp_data = {
    .reg_off = 0x8000,
    .reg_read = imx_ocotp_reg_read,
    .size = 2048,
    .num_entry = 6,
    .entry = {
    { 0, 52, FUSE_FSB },
    { 63, 1, FUSE_ELE},
    { 128, 16, FUSE_ELE },
    { 182, 1, FUSE_ELE },
    { 188, 1, FUSE_ELE },
    { 312, 200, FUSE_FSB }
    },
    };
    static const struct ocotp_devtype_data imx94_ocotp_data = {
    .reg_off = 0x8000,
    .reg_read = imx_ocotp_reg_read,
    .size = 3296, /* 103 Banks */
    .num_entry = 10,
    .entry = {
    { 0, 1, FUSE_FSB | FUSE_ECC },
    { 7, 1, FUSE_FSB | FUSE_ECC },
    { 9, 3, FUSE_FSB | FUSE_ECC },
    { 12, 24, FUSE_FSB },
    { 36, 2, FUSE_FSB  | FUSE_ECC },
    { 38, 14, FUSE_FSB },
    { 59, 1, FUSE_ELE },
    { 525, 2, FUSE_FSB | FUSE_ECC },
    { 528, 7, FUSE_FSB },
    { 536, 280, FUSE_FSB },
    },
    };
    static const struct ocotp_devtype_data imx95_ocotp_data = {
    .reg_off = 0x8000,
    .reg_read = imx_ocotp_reg_read,
    .size = 2048,
    .num_entry = 12,
    .entry = {
    { 0, 1, FUSE_FSB | FUSE_ECC },
    { 7, 1, FUSE_FSB | FUSE_ECC },
    { 9, 3, FUSE_FSB | FUSE_ECC },
    { 12, 24, FUSE_FSB },
    { 36, 2, FUSE_FSB  | FUSE_ECC },
    { 38, 14, FUSE_FSB },
    { 63, 1, FUSE_ELE },
    { 128, 16, FUSE_ELE },
    { 188, 1, FUSE_ELE },
    { 317, 2, FUSE_FSB | FUSE_ECC },
    { 320, 7, FUSE_FSB },
    { 328, 184, FUSE_FSB }
    },
    };
    static const struct of_device_id imx_ele_ocotp_dt_ids[] = {
    { .compatible = "fsl,imx93-ocotp", .data = &imx93_ocotp_data, },
    { .compatible = "fsl,imx94-ocotp", .data = &imx94_ocotp_data, },
    { .compatible = "fsl,imx95-ocotp", .data = &imx95_ocotp_data, },
    {},
    };
    MODULE_DEVICE_TABLE(of, imx_ele_ocotp_dt_ids);
    static struct platform_driver imx_ele_ocotp_driver = {
    .driver = {
    .name = "imx_ele_ocotp",
    .of_match_table = imx_ele_ocotp_dt_ids,
    },
    .probe = imx_ele_ocotp_probe,
    };
    module_platform_driver(imx_ele_ocotp_driver);
    MODULE_DESCRIPTION("i.MX OCOTP/ELE driver");
    MODULE_AUTHOR("Peng Fan <peng.fan@nxp.com>");
    MODULE_LICENSE("GPL");
