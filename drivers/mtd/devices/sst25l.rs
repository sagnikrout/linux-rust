//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/devices/sst25l.c
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
// sst25l.c
//
// Driver for SST25L SPI Flash chips
//
// Copyright © 2009 Bluewater Systems Ltd
// Author: Andre Renaud <andre@bluewatersys.com>
// Author: Ryan Mallon
//
// Based on m25p80.c
//

// Erases can take up to 3 seconds!

pub const SST25L_CMD_WRSR: c_uint = 0x01	/* Write status register */;
pub const SST25L_CMD_WRDI: c_uint = 0x04	/* Write disable */;
pub const SST25L_CMD_RDSR: c_uint = 0x05	/* Read status register */;
pub const SST25L_CMD_WREN: c_uint = 0x06	/* Write enable */;
pub const SST25L_CMD_READ: c_uint = 0x03	/* High speed read */;
pub const SST25L_CMD_EWSR: c_uint = 0x50	/* Enable write status register */;
pub const SST25L_CMD_SECTOR_ERASE: c_uint = 0x20	/* Erase sector */;
pub const SST25L_CMD_READ_ID: c_uint = 0x90	/* Read device ID */;
pub const SST25L_CMD_AAI_PROGRAM: c_uint = 0xaf	/* Auto address increment */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst25l_flash {
    pub spi: *mut spi_device,
    pub lock: mutex,
    pub mtd: mtd_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flash_info {
    pub name: *const c_char,
    pub device_id: u16,
    pub page_size: unsigned,
    pub nr_pages: unsigned,
    pub erase_size: unsigned,
}

    static struct flash_info sst25l_flash_info[] = {
    {"sst25lf020a", 0xbf43, 256, 1024, 4096},
    {"sst25lf040a",	0xbf44,	256, 2048, 4096},
    };
#[no_mangle]
unsafe extern "C" fn sst25l_status(flash: *mut sst25l_flash, status: *mut c_int) -> c_int {
    static int sst25l_status(struct sst25l_flash *flash, int *status)
    {
    struct spi_message m;
    struct spi_transfer t;
    unsigned char cmd_resp[2];
    int err;
    spi_message_init(&m);
    memset(&t, 0, sizeof(struct spi_transfer));
    cmd_resp[0] = SST25L_CMD_RDSR;
    cmd_resp[1] = 0xff;
    t.tx_buf = cmd_resp;
    t.rx_buf = cmd_resp;
    t.len = sizeof(cmd_resp);
    spi_message_add_tail(&t, &m);
    err = spi_sync(flash.spi, &m);
    if (err < 0)
    return err;
// status = cmd_resp[1];
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sst25l_write_enable(flash: *mut sst25l_flash, enable: c_int) -> c_int {
    static int sst25l_write_enable(struct sst25l_flash *flash, int enable)
    {
    unsigned char command[2];
    int status, err;
    command[0] = enable ? SST25L_CMD_WREN : SST25L_CMD_WRDI;
    err = spi_write(flash.spi, command, 1);
    if (err)
    return err;
    command[0] = SST25L_CMD_EWSR;
    err = spi_write(flash.spi, command, 1);
    if (err)
    return err;
    command[0] = SST25L_CMD_WRSR;
    command[1] = enable ? 0 : SST25L_STATUS_BP0 | SST25L_STATUS_BP1;
    err = spi_write(flash.spi, command, 2);
    if (err)
    return err;
    if (enable) {
    err = sst25l_status(flash, &status);
    if (err)
    return err;
    if (!(status & SST25L_STATUS_WREN))
    return -EROFS;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sst25l_wait_till_ready(flash: *mut sst25l_flash) -> c_int {
    static int sst25l_wait_till_ready(struct sst25l_flash *flash)
    {
    unsigned long deadline;
    int status, err;
    deadline = jiffies + MAX_READY_WAIT_JIFFIES;
    do {
    err = sst25l_status(flash, &status);
    if (err)
    return err;
    if (!(status & SST25L_STATUS_BUSY))
    return 0;
    cond_resched();
    } while (!time_after_eq(jiffies, deadline));
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn sst25l_erase_sector(flash: *mut sst25l_flash, offset: u32) -> c_int {
    static int sst25l_erase_sector(struct sst25l_flash *flash, uint32_t offset)
    {
    unsigned char command[4];
    int err;
    err = sst25l_write_enable(flash, 1);
    if (err)
    return err;
    command[0] = SST25L_CMD_SECTOR_ERASE;
    command[1] = offset >> 16;
    command[2] = offset >> 8;
    command[3] = offset;
    err = spi_write(flash.spi, command, 4);
    if (err)
    return err;
    err = sst25l_wait_till_ready(flash);
    if (err)
    return err;
    return sst25l_write_enable(flash, 0);
    }
#[no_mangle]
unsafe extern "C" fn sst25l_erase(mtd: *mut mtd_info, instr: *mut erase_info) -> c_int {
    static int sst25l_erase(struct mtd_info *mtd, struct erase_info *instr)
    {
    struct sst25l_flash *flash = to_sst25l_flash(mtd);
    uint32_t addr, end;
    int err;
// Sanity checks
    if ((uint32_t)instr.len % mtd.erasesize)
    return -EINVAL;
    if ((uint32_t)instr.addr % mtd.erasesize)
    return -EINVAL;
    addr = instr.addr;
    end = addr + instr.len;
    mutex_lock(&flash.lock);
    err = sst25l_wait_till_ready(flash);
    if (err) {
    mutex_unlock(&flash.lock);
    return err;
    }
    while (addr < end) {
    err = sst25l_erase_sector(flash, addr);
    if (err) {
    mutex_unlock(&flash.lock);
    dev_err(&flash.spi.dev, "Erase failed\n");
    return err;
    }
    addr += mtd.erasesize;
    }
    mutex_unlock(&flash.lock);
    return 0;
    }
    static int sst25l_read(struct mtd_info *mtd, loff_t from, size_t len,
    size_t *retlen, unsigned char *buf)
    {
    struct sst25l_flash *flash = to_sst25l_flash(mtd);
    struct spi_transfer transfer[2];
    struct spi_message message;
    unsigned char command[4];
    int ret;
    spi_message_init(&message);
    memset(&transfer, 0, sizeof(transfer));
    command[0] = SST25L_CMD_READ;
    command[1] = from >> 16;
    command[2] = from >> 8;
    command[3] = from;
    transfer[0].tx_buf = command;
    transfer[0].len = sizeof(command);
    spi_message_add_tail(&transfer[0], &message);
    transfer[1].rx_buf = buf;
    transfer[1].len = len;
    spi_message_add_tail(&transfer[1], &message);
    mutex_lock(&flash.lock);
// Wait for previous write/erase to complete
    ret = sst25l_wait_till_ready(flash);
    if (ret) {
    mutex_unlock(&flash.lock);
    return ret;
    }
    spi_sync(flash.spi, &message);
    if (retlen && message.actual_length > sizeof(command))
// retlen += message.actual_length - sizeof(command);
    mutex_unlock(&flash.lock);
    return 0;
    }
    static int sst25l_write(struct mtd_info *mtd, loff_t to, size_t len,
    size_t *retlen, const unsigned char *buf)
    {
    struct sst25l_flash *flash = to_sst25l_flash(mtd);
    int i, j, ret, bytes, copied = 0;
    unsigned char command[5];
    if ((uint32_t)to % mtd.writesize)
    return -EINVAL;
    mutex_lock(&flash.lock);
    ret = sst25l_write_enable(flash, 1);
    if (ret)
    goto out;
    for (i = 0; i < len; i += mtd.writesize) {
    ret = sst25l_wait_till_ready(flash);
    if (ret)
    goto out;
// Write the first byte of the page
    command[0] = SST25L_CMD_AAI_PROGRAM;
    command[1] = (to + i) >> 16;
    command[2] = (to + i) >> 8;
    command[3] = (to + i);
    command[4] = buf[i];
    ret = spi_write(flash.spi, command, 5);
    if (ret < 0)
    goto out;
    copied++;
//
// Write the remaining bytes using auto address
// increment mode
//
    bytes = min_t(uint32_t, mtd.writesize, len - i);
    for (j = 1; j < bytes; j++, copied++) {
    ret = sst25l_wait_till_ready(flash);
    if (ret)
    goto out;
    command[1] = buf[i + j];
    ret = spi_write(flash.spi, command, 2);
    if (ret)
    goto out;
    }
    }
    out:
    ret = sst25l_write_enable(flash, 0);
    if (retlen)
// retlen = copied;
    mutex_unlock(&flash.lock);
    return ret;
    }
    static struct flash_info *sst25l_match_device(struct spi_device *spi)
    {
    struct flash_info *flash_info = core::ptr::null_mut();
    struct spi_message m;
    struct spi_transfer t;
    unsigned char cmd_resp[6];
    int i, err;
    uint16_t id;
    spi_message_init(&m);
    memset(&t, 0, sizeof(struct spi_transfer));
    cmd_resp[0] = SST25L_CMD_READ_ID;
    cmd_resp[1] = 0;
    cmd_resp[2] = 0;
    cmd_resp[3] = 0;
    cmd_resp[4] = 0xff;
    cmd_resp[5] = 0xff;
    t.tx_buf = cmd_resp;
    t.rx_buf = cmd_resp;
    t.len = sizeof(cmd_resp);
    spi_message_add_tail(&t, &m);
    err = spi_sync(spi, &m);
    if (err < 0) {
    dev_err(&spi.dev, "error reading device id\n");
    return core::ptr::null_mut();
    }
    id = (cmd_resp[4] << 8) | cmd_resp[5];
    for (i = 0; i < ARRAY_SIZE(sst25l_flash_info); i++)
    if (sst25l_flash_info[i].device_id == id)
    flash_info = &sst25l_flash_info[i];
    if (!flash_info)
    dev_err(&spi.dev, "unknown id %.4x\n", id);
    return flash_info;
    }
#[no_mangle]
unsafe extern "C" fn sst25l_probe(spi: *mut spi_device) -> c_int {
    static int sst25l_probe(struct spi_device *spi)
    {
    struct flash_info *flash_info;
    struct sst25l_flash *flash;
    struct flash_platform_data *data;
    int ret;
    flash_info = sst25l_match_device(spi);
    if (!flash_info)
    return -ENODEV;
    flash = devm_kzalloc(&spi.dev, sizeof(*flash), GFP_KERNEL);
    if (!flash)
    return -ENOMEM;
    flash.spi = spi;
    mutex_init(&flash.lock);
    spi_set_drvdata(spi, flash);
    data = dev_get_platdata(&spi.dev);
    if (data && data.name)
    flash.mtd.name = data.name;
    flash.mtd.dev.parent   = &spi.dev;
    flash.mtd.type		= MTD_NORFLASH;
    flash.mtd.flags	= MTD_CAP_NORFLASH;
    flash.mtd.erasesize	= flash_info.erase_size;
    flash.mtd.writesize	= flash_info.page_size;
    flash.mtd.writebufsize	= flash_info.page_size;
    flash.mtd.size		= flash_info.page_size * flash_info.nr_pages;
    flash.mtd._erase	= sst25l_erase;
    flash.mtd._read		= sst25l_read;
    flash.mtd._write 	= sst25l_write;
    dev_info(&spi.dev, "%s (%lld KiB)\n", flash_info.name,
    (long long)flash.mtd.size >> 10);
    pr_debug("mtd .name = %s, .size = 0x%llx (%lldMiB) "
    ".erasesize = 0x%.8x (%uKiB) .numeraseregions = %d\n",
    flash.mtd.name,
    (long long)flash.mtd.size, (long long)(flash.mtd.size >> 20),
    flash.mtd.erasesize, flash.mtd.erasesize / 1024,
    flash.mtd.numeraseregions);
    ret = mtd_device_register(&flash.mtd, data ? data.parts : core::ptr::null_mut(),
    data ? data.nr_parts : 0);
    if (ret)
    return -ENODEV;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sst25l_remove(spi: *mut spi_device) {
    static void sst25l_remove(struct spi_device *spi)
    {
    struct sst25l_flash *flash = spi_get_drvdata(spi);
    WARN_ON(mtd_device_unregister(&flash.mtd));
    }
    static struct spi_driver sst25l_driver = {
    .driver = {
    .name	= "sst25l",
    },
    .probe		= sst25l_probe,
    .remove		= sst25l_remove,
    };
    module_spi_driver(sst25l_driver);
    MODULE_DESCRIPTION("MTD SPI driver for SST25L Flash chips");
    MODULE_AUTHOR("Andre Renaud <andre@bluewatersys.com>, "
    "Ryan Mallon");
    MODULE_LICENSE("GPL");
