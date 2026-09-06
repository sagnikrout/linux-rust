//! Automatically rewritten from C to Rust
//! Source: drivers/soc/mediatek/mtk-socinfo.c
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
// Copyright (c) 2023 MediaTek Inc.
//

    .soc_name = _soc_name,									\
    .segment_name = _segment_name,								\
    .marketing_name = _marketing_name,							\
    .cell_data = {_cell_data1, _cell_data2}							\
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_socinfo {
    pub dev: *mut device,
    pub name_data: *mut name_data,
    pub socinfo_data: *mut socinfo_data,
    pub soc_dev: *mut soc_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct socinfo_data {
    pub soc_name: *mut c_char,
    pub segment_name: *mut c_char,
    pub marketing_name: *mut c_char,
    pub cell_data: [u32; MAX_CELLS],
}

    static const char *cell_names[MAX_CELLS] = {"socinfo-data1", "socinfo-data2"};
    static struct socinfo_data socinfo_data_table[] = {
    MTK_SOCINFO_ENTRY("MT8173", "MT8173V/AC", "MT8173", 0x6CA20004, 0x10000000),
    MTK_SOCINFO_ENTRY("MT8183", "MT8183V/AZA", "Kompanio 500", 0x00010043, 0x00000840),
    MTK_SOCINFO_ENTRY("MT8183", "MT8183V/AZA", "Kompanio 500", 0x00010043, 0x00000940),
    MTK_SOCINFO_ENTRY("MT8186", "MT8186GV/AZA", "Kompanio 520", 0x81861001, CELL_NOT_USED),
    MTK_SOCINFO_ENTRY("MT8186T", "MT8186TV/AZA", "Kompanio 528", 0x81862001, CELL_NOT_USED),
    MTK_SOCINFO_ENTRY("MT8188", "MT8188GV/AZA", "Kompanio 838", 0x81880000, 0x00000010),
    MTK_SOCINFO_ENTRY("MT8188", "MT8188GV/HZA", "Kompanio 838", 0x81880000, 0x00000011),
    MTK_SOCINFO_ENTRY("MT8189", "MT8189GV/AZA", "Kompanio 540", 0x81890000, 0x00000020),
    MTK_SOCINFO_ENTRY("MT8189", "MT8189HV/AZA", "Kompanio 540", 0x81890000, 0x00000021),
    MTK_SOCINFO_ENTRY("MT8192", "MT8192V/AZA", "Kompanio 820", 0x00001100, 0x00040080),
    MTK_SOCINFO_ENTRY("MT8192T", "MT8192V/ATZA", "Kompanio 828", 0x00000100, 0x000400C0),
    MTK_SOCINFO_ENTRY("MT8195", "MT8195GV/EZA", "Kompanio 1200", 0x81950300, CELL_NOT_USED),
    MTK_SOCINFO_ENTRY("MT8195", "MT8195GV/EHZA", "Kompanio 1200", 0x81950304, CELL_NOT_USED),
    MTK_SOCINFO_ENTRY("MT8195", "MT8195TV/EZA", "Kompanio 1380", 0x81950400, CELL_NOT_USED),
    MTK_SOCINFO_ENTRY("MT8195", "MT8195TV/EHZA", "Kompanio 1380", 0x81950404, CELL_NOT_USED),
    MTK_SOCINFO_ENTRY("MT8370", "MT8370AV/AZA", "Genio 510", 0x83700000, 0x00000081),
    MTK_SOCINFO_ENTRY("MT8371", "MT8371AV/AZA", "Genio 520", 0x83710000, 0x00000081),
    MTK_SOCINFO_ENTRY("MT8390", "MT8390AV/AZA", "Genio 700", 0x83900000, 0x00000080),
    MTK_SOCINFO_ENTRY("MT8391", "MT8391AV/AZA", "Genio 720", 0x83910000, 0x00000080),
    MTK_SOCINFO_ENTRY("MT8395", "MT8395AV/ZA", "Genio 1200", 0x83950100, CELL_NOT_USED),
    MTK_SOCINFO_ENTRY("MT8395", "MT8395AV/ZA", "Genio 1200", 0x83950800, CELL_NOT_USED),
    };
#[no_mangle]
unsafe extern "C" fn mtk_socinfo_create_socinfo_node(mtk_socinfop: *mut mtk_socinfo) -> c_int {
    static int mtk_socinfo_create_socinfo_node(struct mtk_socinfo *mtk_socinfop)
    {
    struct soc_device_attribute *attrs;
    struct socinfo_data *data = mtk_socinfop.socinfo_data;
    static const char *soc_manufacturer = "MediaTek";
    attrs = devm_kzalloc(mtk_socinfop.dev, sizeof(*attrs), GFP_KERNEL);
    if (!attrs)
    return -ENOMEM;
    if (data.marketing_name != core::ptr::null_mut() && data.marketing_name[0] != '\0') {
    attrs.family = devm_kasprintf(mtk_socinfop.dev, GFP_KERNEL, "MediaTek %s",
    data.marketing_name);
    if (!attrs.family)
    return -ENOMEM;
    } else {
    attrs.family = soc_manufacturer;
    }
    attrs.soc_id = data.soc_name;
//
// The "machine" field will be populated automatically with the model
// name from board DTS (if available).
//
    mtk_socinfop.soc_dev = soc_device_register(attrs);
    if (IS_ERR(mtk_socinfop.soc_dev))
    return PTR_ERR(mtk_socinfop.soc_dev);
    dev_info(mtk_socinfop.dev, "%s (%s) SoC detected.\n", attrs.family, attrs.soc_id);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_socinfo_read_cell(dev: *mut device, name: *const c_char) -> u32 {
    static u32 mtk_socinfo_read_cell(struct device *dev, const char *name)
    {
    struct nvmem_device *nvmemp;
    struct device_node *np, *nvmem_node = dev.parent.of_node;
    u32 offset;
    let mut cell_val: u32 = CELL_NOT_USED;
// should never fail since the nvmem driver registers this child
    nvmemp = nvmem_device_find(nvmem_node, device_match_of_node);
    if (IS_ERR(nvmemp))
    goto out;
    np = of_get_child_by_name(nvmem_node, name);
    if (!np)
    goto put_device;
    if (of_property_read_u32_index(np, "reg", 0, &offset))
    goto put_node;
    nvmem_device_read(nvmemp, offset, sizeof(cell_val), &cell_val);
    put_node:
    of_node_put(np);
    put_device:
    nvmem_device_put(nvmemp);
    out:
    return cell_val;
    }
#[no_mangle]
unsafe extern "C" fn mtk_socinfo_get_socinfo_data(mtk_socinfop: *mut mtk_socinfo) -> c_int {
    static int mtk_socinfo_get_socinfo_data(struct mtk_socinfo *mtk_socinfop)
    {
    unsigned int i, j;
    let mut num_cell_data: c_uint = 0;
    u32 cell_data[MAX_CELLS] = {0};
    bool match_socinfo;
    let mut match_socinfo_index: c_int = -1;
    for (i = 0; i < MAX_CELLS; i++) {
    cell_data[i] = mtk_socinfo_read_cell(mtk_socinfop.dev, cell_names[i]);
    if (cell_data[i] != CELL_NOT_USED)
    num_cell_data++;
    else
    break;
    }
    if (!num_cell_data)
    return -ENOENT;
    for (i = 0; i < ARRAY_SIZE(socinfo_data_table); i++) {
    match_socinfo = true;
    for (j = 0; j < num_cell_data; j++) {
    if (cell_data[j] != socinfo_data_table[i].cell_data[j]) {
    match_socinfo = false;
    break;
    }
    }
    if (match_socinfo) {
    mtk_socinfop.socinfo_data = &(socinfo_data_table[i]);
    match_socinfo_index = i;
    break;
    }
    }
    if (match_socinfo_index < 0) {
    dev_warn(mtk_socinfop.dev,
    "Unknown MediaTek SoC with ID 0x%08x 0x%08x\n",
    cell_data[0], cell_data[1]);
    return -ENOENT;
    }
    return match_socinfo_index;
    }
#[no_mangle]
unsafe extern "C" fn mtk_socinfo_probe(pdev: *mut platform_device) -> c_int {
    static int mtk_socinfo_probe(struct platform_device *pdev)
    {
    struct mtk_socinfo *mtk_socinfop;
    int ret;
    mtk_socinfop = devm_kzalloc(&pdev.dev, sizeof(*mtk_socinfop), GFP_KERNEL);
    if (!mtk_socinfop)
    return -ENOMEM;
    mtk_socinfop.dev = &pdev.dev;
    ret = mtk_socinfo_get_socinfo_data(mtk_socinfop);
    if (ret < 0)
    return dev_err_probe(mtk_socinfop.dev, ret, "Failed to get socinfo data\n");
    ret = mtk_socinfo_create_socinfo_node(mtk_socinfop);
    if (ret)
    return dev_err_probe(mtk_socinfop.dev, ret, "Cannot create node\n");
    platform_set_drvdata(pdev, mtk_socinfop);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_socinfo_remove(pdev: *mut platform_device) {
    static void mtk_socinfo_remove(struct platform_device *pdev)
    {
    struct mtk_socinfo *mtk_socinfop = platform_get_drvdata(pdev);
    soc_device_unregister(mtk_socinfop.soc_dev);
    }
    static struct platform_driver mtk_socinfo = {
    .probe = mtk_socinfo_probe,
    .remove = mtk_socinfo_remove,
    .driver = {
    .name = "mtk-socinfo",
    },
    };
    module_platform_driver(mtk_socinfo);
    MODULE_AUTHOR("William-TW LIN <william-tw.lin@mediatek.com>");
    MODULE_DESCRIPTION("MediaTek socinfo driver");
    MODULE_LICENSE("GPL");
