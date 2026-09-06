//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/lpddr/qinfo_probe.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Probing flash chips with QINFO records.
// (C) 2008 Korolev Alexey <akorolev@infradead.org>
// (C) 2008 Vasiliy Leonenko <vasiliy.leonenko@gmail.com>
//

    static int lpddr_chip_setup(struct map_info *map, struct lpddr_private *lpddr);
    struct mtd_info *lpddr_probe(struct map_info *map);
    static struct lpddr_private *lpddr_probe_chip(struct map_info *map);
    static int lpddr_pfow_present(struct map_info *map,
    struct lpddr_private *lpddr);
    static struct qinfo_query_info qinfo_array[] = {
// General device info
    {0, 0, "DevSizeShift", "Device size 2^n bytes"},
    {0, 3, "BufSizeShift", "Program buffer size 2^n bytes"},
// Erase block information
    {1, 1, "TotalBlocksNum", "Total number of blocks"},
    {1, 2, "UniformBlockSizeShift", "Uniform block size 2^n bytes"},
// Partition information
    {2, 1, "HWPartsNum", "Number of hardware partitions"},
// Optional features
    {5, 1, "SuspEraseSupp", "Suspend erase supported"},
// Operation typical time
    {10, 0, "SingleWordProgTime", "Single word program 2^n u-sec"},
    {10, 1, "ProgBufferTime", "Program buffer write 2^n u-sec"},
    {10, 2, "BlockEraseTime", "Block erase 2^n m-sec"},
    {10, 3, "FullChipEraseTime", "Full chip erase 2^n m-sec"},
    };
#[no_mangle]
unsafe extern "C" fn lpddr_get_qinforec_pos(map: *mut map_info, id_str: *mut c_char) -> c_long {
    static long lpddr_get_qinforec_pos(struct map_info *map, char *id_str)
    {
    let mut qinfo_lines: c_int = ARRAY_SIZE(qinfo_array);
    int i;
    let mut bankwidth: c_int = map_bankwidth(map) * 8;
    int major, minor;
    for (i = 0; i < qinfo_lines; i++) {
    if (strcmp(id_str, qinfo_array[i].id_str) == 0) {
    major = qinfo_array[i].major & ((1 << bankwidth) - 1);
    minor = qinfo_array[i].minor & ((1 << bankwidth) - 1);
    return minor | (major << bankwidth);
    }
    }
    printk(KERN_ERR"%s qinfo id string is wrong!\n", map.name);
    BUG();
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn lpddr_info_query(map: *mut map_info, id_str: *mut c_char) -> u16 {
    static uint16_t lpddr_info_query(struct map_info *map, char *id_str)
    {
    unsigned int dsr, val;
    let mut bits_per_chip: c_int = map_bankwidth(map) * 8;
    let mut adr: c_ulong = lpddr_get_qinforec_pos(map, id_str);
    let mut attempts: c_int = 20;
// Write a request for the PFOW record
    map_write(map, CMD(LPDDR_INFO_QUERY),
    map.pfow_base + PFOW_COMMAND_CODE);
    map_write(map, CMD(adr & ((1 << bits_per_chip) - 1)),
    map.pfow_base + PFOW_COMMAND_ADDRESS_L);
    map_write(map, CMD(adr >> bits_per_chip),
    map.pfow_base + PFOW_COMMAND_ADDRESS_H);
    map_write(map, CMD(LPDDR_START_EXECUTION),
    map.pfow_base + PFOW_COMMAND_EXECUTE);
    while ((attempts--) > 0) {
    dsr = CMDVAL(map_read(map, map.pfow_base + PFOW_DSR));
    if (dsr & DSR_READY_STATUS)
    break;
    udelay(10);
    }
    val = CMDVAL(map_read(map, map.pfow_base + PFOW_COMMAND_DATA));
    return val;
    }
#[no_mangle]
unsafe extern "C" fn lpddr_pfow_present(map: *mut map_info, lpddr: *mut lpddr_private) -> c_int {
    static int lpddr_pfow_present(struct map_info *map, struct lpddr_private *lpddr)
    {
    map_word pfow_val[4];
// Check identification string
    pfow_val[0] = map_read(map, map.pfow_base + PFOW_QUERY_STRING_P);
    pfow_val[1] = map_read(map, map.pfow_base + PFOW_QUERY_STRING_F);
    pfow_val[2] = map_read(map, map.pfow_base + PFOW_QUERY_STRING_O);
    pfow_val[3] = map_read(map, map.pfow_base + PFOW_QUERY_STRING_W);
    if (!map_word_equal(map, CMD('P'), pfow_val[0]))
    goto out;
    if (!map_word_equal(map, CMD('F'), pfow_val[1]))
    goto out;
    if (!map_word_equal(map, CMD('O'), pfow_val[2]))
    goto out;
    if (!map_word_equal(map, CMD('W'), pfow_val[3]))
    goto out;
    return 1;	/* "PFOW" is found */
    out:
    printk(KERN_WARNING"%s: PFOW string at 0x%lx is not found\n",
    map.name, map.pfow_base);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpddr_chip_setup(map: *mut map_info, lpddr: *mut lpddr_private) -> c_int {
    static int lpddr_chip_setup(struct map_info *map, struct lpddr_private *lpddr)
    {
    lpddr.qinfo = kzalloc_obj(struct qinfo_chip);
    if (!lpddr.qinfo)
    return 0;
// Get the ManuID
    lpddr.ManufactId = CMDVAL(map_read(map, map.pfow_base + PFOW_MANUFACTURER_ID));
// Get the DeviceID
    lpddr.DevId = CMDVAL(map_read(map, map.pfow_base + PFOW_DEVICE_ID));
// read parameters from chip qinfo table
    lpddr.qinfo.DevSizeShift = lpddr_info_query(map, "DevSizeShift");
    lpddr.qinfo.TotalBlocksNum = lpddr_info_query(map, "TotalBlocksNum");
    lpddr.qinfo.BufSizeShift = lpddr_info_query(map, "BufSizeShift");
    lpddr.qinfo.HWPartsNum = lpddr_info_query(map, "HWPartsNum");
    lpddr.qinfo.UniformBlockSizeShift =
    lpddr_info_query(map, "UniformBlockSizeShift");
    lpddr.qinfo.SuspEraseSupp = lpddr_info_query(map, "SuspEraseSupp");
    lpddr.qinfo.SingleWordProgTime =
    lpddr_info_query(map, "SingleWordProgTime");
    lpddr.qinfo.ProgBufferTime = lpddr_info_query(map, "ProgBufferTime");
    lpddr.qinfo.BlockEraseTime = lpddr_info_query(map, "BlockEraseTime");
    return 1;
    }
    static struct lpddr_private *lpddr_probe_chip(struct map_info *map)
    {
    struct lpddr_private lpddr;
    struct lpddr_private *retlpddr;
    int numvirtchips;
    if ((map.pfow_base + 0x1000) >= map.size) {
    printk(KERN_NOTICE"%s Probe at base (0x%08lx) past the end of"
    "the map(0x%08lx)\n", map.name,
    (unsigned long)map.pfow_base, map.size - 1);
    return core::ptr::null_mut();
    }
    memset(&lpddr, 0, sizeof(struct lpddr_private));
    if (!lpddr_pfow_present(map, &lpddr))
    return core::ptr::null_mut();
    if (!lpddr_chip_setup(map, &lpddr))
    return core::ptr::null_mut();
// Ok so we found a chip
    lpddr.chipshift = lpddr.qinfo.DevSizeShift;
    lpddr.numchips = 1;
    numvirtchips = lpddr.numchips * lpddr.qinfo.HWPartsNum;
    retlpddr = kzalloc_flex(*retlpddr, chips, numvirtchips);
    if (!retlpddr)
    return core::ptr::null_mut();
    memcpy(retlpddr, &lpddr, sizeof(struct lpddr_private));
    retlpddr.numchips = numvirtchips;
    retlpddr.chipshift = retlpddr.qinfo.DevSizeShift -
    __ffs(retlpddr.qinfo.HWPartsNum);
    return retlpddr;
    }
    struct mtd_info *lpddr_probe(struct map_info *map)
    {
    struct mtd_info *mtd = core::ptr::null_mut();
    struct lpddr_private *lpddr;
// First probe the map to see if we havecan open PFOW here
    lpddr = lpddr_probe_chip(map);
    if (!lpddr)
    return core::ptr::null_mut();
    map.fldrv_priv = lpddr;
    mtd = lpddr_cmdset(map);
    if (mtd) {
    if (mtd.size > map.size) {
    printk(KERN_WARNING "Reducing visibility of %ldKiB chip"
    "to %ldKiB\n", (unsigned long)mtd.size >> 10,
    (unsigned long)map.size >> 10);
    mtd.size = map.size;
    }
    return mtd;
    }
    kfree(lpddr.qinfo);
    kfree(lpddr);
    map.fldrv_priv = core::ptr::null_mut();
    return core::ptr::null_mut();
    }
    static struct mtd_chip_driver lpddr_chipdrv = {
    .probe		= lpddr_probe,
    .name		= "qinfo_probe",
    .module		= THIS_MODULE
    };
#[no_mangle]
unsafe extern "C" fn lpddr_probe_init() -> int __init {
    static int __init lpddr_probe_init(void)
    {
    register_mtd_chip_driver(&lpddr_chipdrv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpddr_probe_exit() -> void __exit {
    static void __exit lpddr_probe_exit(void)
    {
    unregister_mtd_chip_driver(&lpddr_chipdrv);
    }
    module_init(lpddr_probe_init);
    module_exit(lpddr_probe_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Vasiliy Leonenko <vasiliy.leonenko@gmail.com>");
    MODULE_DESCRIPTION("Driver to probe qinfo flash chips");
