//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/lpddr/lpddr2_nvm.c
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
// LPDDR2-NVM MTD driver. This module provides read, write, erase, lock/unlock
// support for LPDDR2-NVM PCM memories
//
// Copyright © 2012 Micron Technology, Inc.
//
// Vincenzo Aliberti <vincenzo.aliberti@gmail.com>
// Domenico Manna <domenico.manna@gmail.com>
// Many thanks to Andrea Vigilante for initial enabling
//

// Parameters

pub const OW_BASE_ADDRESS: c_uint = 0x00000000	/* OW offset */;
pub const BUS_WIDTH: c_uint = 0x00000020	/* x32 devices */;
// PFOW symbols address offset

// OW registers address

// Datamask
pub const MR_CFGMASK: c_uint = 0x8000;
pub const SR_OK_DATAMASK: c_uint = 0x0080;
// LPDDR2-NVM Commands
pub const LPDDR2_NVM_LOCK: c_uint = 0x0061;
pub const LPDDR2_NVM_UNLOCK: c_uint = 0x0062;
pub const LPDDR2_NVM_SW_PROGRAM: c_uint = 0x0041;
pub const LPDDR2_NVM_SW_OVERWRITE: c_uint = 0x0042;
pub const LPDDR2_NVM_BUF_PROGRAM: c_uint = 0x00E9;
pub const LPDDR2_NVM_BUF_OVERWRITE: c_uint = 0x00EA;
pub const LPDDR2_NVM_ERASE: c_uint = 0x0020;
// LPDDR2-NVM Registers offset
pub const LPDDR2_MODE_REG_DATA: c_uint = 0x0040;
pub const LPDDR2_MODE_REG_CFG: c_uint = 0x0050;
//
// Internal Type Definitions
// pcm_int_data contains memory controller details:
// @reg_data : LPDDR2_MODE_REG_DATA register address after remapping
// @reg_cfg  : LPDDR2_MODE_REG_CFG register address after remapping
// &bus_width: memory bus-width (eg: x16 2 Bytes, x32 4 Bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcm_int_data {
    pub ctl_regs: *mut void __iomem,
    pub bus_width: c_int,
}

    static DEFINE_MUTEX(lpdd2_nvm_mutex);
//
// Build a map_word starting from an u_long
//
#[no_mangle]
pub unsafe extern "C" fn build_map_word(myword: u_long) -> map_word {
    static inline map_word build_map_word(u_long myword)
    {
    let mut val: map_word = { {0} };
    val.x[0] = myword;
    return val;
    }
//
// Build Mode Register Configuration DataMask based on device bus-width
//
#[no_mangle]
pub unsafe extern "C" fn build_mr_cfgmask(bus_width: u_int) -> u_int {
    static inline u_int build_mr_cfgmask(u_int bus_width)
    {
    let mut val: u_int = MR_CFGMASK;
    if (bus_width == 0x0004)		/* x32 device */
    val = val << 16;
    return val;
    }
//
// Build Status Register OK DataMask based on device bus-width
//
#[no_mangle]
pub unsafe extern "C" fn build_sr_ok_datamask(bus_width: u_int) -> u_int {
    static inline u_int build_sr_ok_datamask(u_int bus_width)
    {
    let mut val: u_int = SR_OK_DATAMASK;
    if (bus_width == 0x0004)		/* x32 device */
    val = (val << 16)+val;
    return val;
    }
//
// Evaluates Overlay Window Control Registers address
//
#[no_mangle]
pub unsafe extern "C" fn ow_reg_add(map: *mut map_info, offset: u_long) -> u_long {
    static inline u_long ow_reg_add(struct map_info *map, u_long offset)
    {
    let mut val: u_long = 0;
    struct pcm_int_data *pcm_data = map.fldrv_priv;
    val = map.pfow_base + offset*pcm_data.bus_width;
    return val;
    }
//
// Enable lpddr2-nvm Overlay Window
// Overlay Window is a memory mapped area containing all LPDDR2-NVM registers
// used by device commands as well as uservisible resources like Device Status
// Register, Device ID, etc
//
#[no_mangle]
pub unsafe extern "C" fn ow_enable(map: *mut map_info) {
    static inline void ow_enable(struct map_info *map)
    {
    struct pcm_int_data *pcm_data = map.fldrv_priv;
    writel_relaxed(build_mr_cfgmask(pcm_data.bus_width) | 0x18,
    pcm_data.ctl_regs + LPDDR2_MODE_REG_CFG);
    writel_relaxed(0x01, pcm_data.ctl_regs + LPDDR2_MODE_REG_DATA);
    }
//
// Disable lpddr2-nvm Overlay Window
// Overlay Window is a memory mapped area containing all LPDDR2-NVM registers
// used by device commands as well as uservisible resources like Device Status
// Register, Device ID, etc
//
#[no_mangle]
pub unsafe extern "C" fn ow_disable(map: *mut map_info) {
    static inline void ow_disable(struct map_info *map)
    {
    struct pcm_int_data *pcm_data = map.fldrv_priv;
    writel_relaxed(build_mr_cfgmask(pcm_data.bus_width) | 0x18,
    pcm_data.ctl_regs + LPDDR2_MODE_REG_CFG);
    writel_relaxed(0x02, pcm_data.ctl_regs + LPDDR2_MODE_REG_DATA);
    }
//
// Execute lpddr2-nvm operations
//
    static int lpddr2_nvm_do_op(struct map_info *map, u_long cmd_code,
    u_long cmd_data, u_long cmd_add, u_long cmd_mpr, u_char *buf)
    {
    map_word add_l = { {0} }, add_h = { {0} }, mpr_l = { {0} },
    mpr_h = { {0} }, data_l = { {0} }, cmd = { {0} },
    exec_cmd = { {0} }, sr;
    map_word data_h = { {0} };	/* only for 2x x16 devices stacked */
    u_long i, status_reg, prg_buff_ofs;
    struct pcm_int_data *pcm_data = map.fldrv_priv;
    let mut sr_ok_datamask: u_int = build_sr_ok_datamask(pcm_data.bus_width);
// Builds low and high words for OW Control Registers
    add_l.x[0]	= cmd_add & 0x0000FFFF;
    add_h.x[0]	= (cmd_add >> 16) & 0x0000FFFF;
    mpr_l.x[0]	= cmd_mpr & 0x0000FFFF;
    mpr_h.x[0]	= (cmd_mpr >> 16) & 0x0000FFFF;
    cmd.x[0]	= cmd_code & 0x0000FFFF;
    exec_cmd.x[0]	= 0x0001;
    data_l.x[0]	= cmd_data & 0x0000FFFF;
    data_h.x[0]	= (cmd_data >> 16) & 0x0000FFFF; /* only for 2x x16 */
// Set Overlay Window Control Registers
    map_write(map, cmd, ow_reg_add(map, CMD_CODE_OFS));
    map_write(map, data_l, ow_reg_add(map, CMD_DATA_OFS));
    map_write(map, add_l, ow_reg_add(map, CMD_ADD_L_OFS));
    map_write(map, add_h, ow_reg_add(map, CMD_ADD_H_OFS));
    map_write(map, mpr_l, ow_reg_add(map, MPR_L_OFS));
    map_write(map, mpr_h, ow_reg_add(map, MPR_H_OFS));
    if (pcm_data.bus_width == 0x0004) {	/* 2x16 devices stacked */
    map_write(map, cmd, ow_reg_add(map, CMD_CODE_OFS) + 2);
    map_write(map, data_h, ow_reg_add(map, CMD_DATA_OFS) + 2);
    map_write(map, add_l, ow_reg_add(map, CMD_ADD_L_OFS) + 2);
    map_write(map, add_h, ow_reg_add(map, CMD_ADD_H_OFS) + 2);
    map_write(map, mpr_l, ow_reg_add(map, MPR_L_OFS) + 2);
    map_write(map, mpr_h, ow_reg_add(map, MPR_H_OFS) + 2);
    }
// Fill Program Buffer
    if ((cmd_code == LPDDR2_NVM_BUF_PROGRAM) ||
    (cmd_code == LPDDR2_NVM_BUF_OVERWRITE)) {
    prg_buff_ofs = (map_read(map,
    ow_reg_add(map, PRG_BUFFER_OFS))).x[0];
    for (i = 0; i < cmd_mpr; i++) {
    map_write(map, build_map_word(buf[i]), map.pfow_base +
    prg_buff_ofs + i);
    }
    }
// Command Execute
    map_write(map, exec_cmd, ow_reg_add(map, CMD_EXEC_OFS));
    if (pcm_data.bus_width == 0x0004)	/* 2x16 devices stacked */
    map_write(map, exec_cmd, ow_reg_add(map, CMD_EXEC_OFS) + 2);
// Status Register Check
    do {
    sr = map_read(map, ow_reg_add(map, STATUS_REG_OFS));
    status_reg = sr.x[0];
    if (pcm_data.bus_width == 0x0004) {/* 2x16 devices stacked */
    sr = map_read(map, ow_reg_add(map,
    STATUS_REG_OFS) + 2);
    status_reg += sr.x[0] << 16;
    }
    } while ((status_reg & sr_ok_datamask) != sr_ok_datamask);
    return (((status_reg & sr_ok_datamask) == sr_ok_datamask) ? 0 : -EIO);
    }
//
// Execute lpddr2-nvm operations @ block level
//
    static int lpddr2_nvm_do_block_op(struct mtd_info *mtd, loff_t start_add,
    uint64_t len, u_char block_op)
    {
    struct map_info *map = mtd.priv;
    u_long add, end_add;
    let mut ret: c_int = 0;
    mutex_lock(&lpdd2_nvm_mutex);
    ow_enable(map);
    add = start_add;
    end_add = add + len;
    do {
    ret = lpddr2_nvm_do_op(map, block_op, 0x00, add, add, core::ptr::null_mut());
    if (ret)
    goto out;
    add += mtd.erasesize;
    } while (add < end_add);
    out:
    ow_disable(map);
    mutex_unlock(&lpdd2_nvm_mutex);
    return ret;
    }
//
// verify presence of PFOW string
//
#[no_mangle]
unsafe extern "C" fn lpddr2_nvm_pfow_present(map: *mut map_info) -> c_int {
    static int lpddr2_nvm_pfow_present(struct map_info *map)
    {
    map_word pfow_val[4];
    let mut found: c_uint = 1;
    mutex_lock(&lpdd2_nvm_mutex);
    ow_enable(map);
// Load string from array
    pfow_val[0] = map_read(map, ow_reg_add(map, PFOW_QUERY_STRING_P));
    pfow_val[1] = map_read(map, ow_reg_add(map, PFOW_QUERY_STRING_F));
    pfow_val[2] = map_read(map, ow_reg_add(map, PFOW_QUERY_STRING_O));
    pfow_val[3] = map_read(map, ow_reg_add(map, PFOW_QUERY_STRING_W));
// Verify the string loaded vs expected
    if (!map_word_equal(map, build_map_word('P'), pfow_val[0]))
    found = 0;
    if (!map_word_equal(map, build_map_word('F'), pfow_val[1]))
    found = 0;
    if (!map_word_equal(map, build_map_word('O'), pfow_val[2]))
    found = 0;
    if (!map_word_equal(map, build_map_word('W'), pfow_val[3]))
    found = 0;
    ow_disable(map);
    mutex_unlock(&lpdd2_nvm_mutex);
    return found;
    }
//
// lpddr2_nvm driver read method
//
    static int lpddr2_nvm_read(struct mtd_info *mtd, loff_t start_add,
    size_t len, size_t *retlen, u_char *buf)
    {
    struct map_info *map = mtd.priv;
    mutex_lock(&lpdd2_nvm_mutex);
// retlen = len;
    map_copy_from(map, buf, start_add, *retlen);
    mutex_unlock(&lpdd2_nvm_mutex);
    return 0;
    }
//
// lpddr2_nvm driver write method
//
    static int lpddr2_nvm_write(struct mtd_info *mtd, loff_t start_add,
    size_t len, size_t *retlen, const u_char *buf)
    {
    struct map_info *map = mtd.priv;
    struct pcm_int_data *pcm_data = map.fldrv_priv;
    u_long add, current_len, tot_len, target_len, my_data;
    u_char *write_buf = (u_char *)buf;
    let mut ret: c_int = 0;
    mutex_lock(&lpdd2_nvm_mutex);
    ow_enable(map);
// Set start value for the variables
    add = start_add;
    target_len = len;
    tot_len = 0;
    while (tot_len < target_len) {
    if (!(IS_ALIGNED(add, mtd.writesize))) { /* do sw program */
    my_data = write_buf[tot_len];
    my_data += (write_buf[tot_len+1]) << 8;
    if (pcm_data.bus_width == 0x0004) {/* 2x16 devices */
    my_data += (write_buf[tot_len+2]) << 16;
    my_data += (write_buf[tot_len+3]) << 24;
    }
    ret = lpddr2_nvm_do_op(map, LPDDR2_NVM_SW_OVERWRITE,
    my_data, add, 0x00, core::ptr::null_mut());
    if (ret)
    goto out;
    add += pcm_data.bus_width;
    tot_len += pcm_data.bus_width;
    } else {		/* do buffer program */
    current_len = min(target_len - tot_len,
    (u_long) mtd.writesize);
    ret = lpddr2_nvm_do_op(map, LPDDR2_NVM_BUF_OVERWRITE,
    0x00, add, current_len, write_buf + tot_len);
    if (ret)
    goto out;
    add += current_len;
    tot_len += current_len;
    }
    }
    out:
// retlen = tot_len;
    ow_disable(map);
    mutex_unlock(&lpdd2_nvm_mutex);
    return ret;
    }
//
// lpddr2_nvm driver erase method
//
#[no_mangle]
unsafe extern "C" fn lpddr2_nvm_erase(mtd: *mut mtd_info, instr: *mut erase_info) -> c_int {
    static int lpddr2_nvm_erase(struct mtd_info *mtd, struct erase_info *instr)
    {
    return lpddr2_nvm_do_block_op(mtd, instr.addr, instr.len,
    LPDDR2_NVM_ERASE);
    }
//
// lpddr2_nvm driver unlock method
//
    static int lpddr2_nvm_unlock(struct mtd_info *mtd, loff_t start_add,
    uint64_t len)
    {
    return lpddr2_nvm_do_block_op(mtd, start_add, len, LPDDR2_NVM_UNLOCK);
    }
//
// lpddr2_nvm driver lock method
//
    static int lpddr2_nvm_lock(struct mtd_info *mtd, loff_t start_add,
    uint64_t len)
    {
    return lpddr2_nvm_do_block_op(mtd, start_add, len, LPDDR2_NVM_LOCK);
    }
    static const struct mtd_info lpddr2_nvm_mtd_info = {
    .type		= MTD_RAM,
    .writesize	= 1,
    .flags		= (MTD_CAP_NVRAM | MTD_POWERUP_LOCK),
    ._read		= lpddr2_nvm_read,
    ._write		= lpddr2_nvm_write,
    ._erase		= lpddr2_nvm_erase,
    ._unlock	= lpddr2_nvm_unlock,
    ._lock		= lpddr2_nvm_lock,
    };
//
// lpddr2_nvm driver probe method
//
#[no_mangle]
unsafe extern "C" fn lpddr2_nvm_probe(pdev: *mut platform_device) -> c_int {
    static int lpddr2_nvm_probe(struct platform_device *pdev)
    {
    struct map_info *map;
    struct mtd_info *mtd;
    struct resource *add_range;
    struct pcm_int_data *pcm_data;
// Allocate memory control_regs data structures
    pcm_data = devm_kzalloc(&pdev.dev, sizeof(*pcm_data), GFP_KERNEL);
    if (!pcm_data)
    return -ENOMEM;
    pcm_data.bus_width = BUS_WIDTH;
// Allocate memory for map_info & mtd_info data structures
    map = devm_kzalloc(&pdev.dev, sizeof(*map), GFP_KERNEL);
    if (!map)
    return -ENOMEM;
    mtd = devm_kzalloc(&pdev.dev, sizeof(*mtd), GFP_KERNEL);
    if (!mtd)
    return -ENOMEM;
// lpddr2_nvm address range
    add_range = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!add_range)
    return -ENODEV;
// Populate map_info data structure
// map = (struct map_info) {
    .virt		= devm_ioremap_resource(&pdev.dev, add_range),
    .name		= pdev.dev.init_name,
    .phys		= add_range.start,
    .size		= resource_size(add_range),
    .bankwidth	= pcm_data.bus_width / 2,
    .pfow_base	= OW_BASE_ADDRESS,
    .fldrv_priv	= pcm_data,
    };
    if (IS_ERR(map.virt))
    return PTR_ERR(map.virt);
    simple_map_init(map);	/* fill with default methods */
    pcm_data.ctl_regs = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(pcm_data.ctl_regs))
    return PTR_ERR(pcm_data.ctl_regs);
// Populate mtd_info data structure
// mtd = lpddr2_nvm_mtd_info;
    mtd.dev.parent		= &pdev.dev;
    mtd.name		= pdev.dev.init_name;
    mtd.priv		= map;
    mtd.size		= resource_size(add_range);
    mtd.erasesize		= ERASE_BLOCKSIZE * pcm_data.bus_width;
    mtd.writebufsize	= WRITE_BUFFSIZE * pcm_data.bus_width;
// Verify the presence of the device looking for PFOW string
    if (!lpddr2_nvm_pfow_present(map)) {
    pr_err("device not recognized\n");
    return -EINVAL;
    }
// Parse partitions and register the MTD device
    return mtd_device_register(mtd, core::ptr::null_mut(), 0);
    }
//
// lpddr2_nvm driver remove method
//
#[no_mangle]
unsafe extern "C" fn lpddr2_nvm_remove(pdev: *mut platform_device) {
    static void lpddr2_nvm_remove(struct platform_device *pdev)
    {
    WARN_ON(mtd_device_unregister(dev_get_drvdata(&pdev.dev)));
    }
// Initialize platform_driver data structure for lpddr2_nvm
    static struct platform_driver lpddr2_nvm_drv = {
    .driver		= {
    .name	= "lpddr2_nvm",
    },
    .probe		= lpddr2_nvm_probe,
    .remove		= lpddr2_nvm_remove,
    };
    module_platform_driver(lpddr2_nvm_drv);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Vincenzo Aliberti <vincenzo.aliberti@gmail.com>");
    MODULE_DESCRIPTION("MTD driver for LPDDR2-NVM PCM memories");
