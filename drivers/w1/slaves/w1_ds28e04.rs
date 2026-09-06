//! Automatically rewritten from C to Rust
//! Source: drivers/w1/slaves/w1_ds28e04.c
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
// w1_ds28e04.c - w1 family 1C (DS28E04) driver
//
// Copyright (c) 2012 Markus Franke <franke.m@sebakmt.com>
//

pub const CRC16_INIT: c_int = 0;
pub const CRC16_VALID: c_uint = 0xb001;

pub const W1_FAMILY_DS28E04: c_uint = 0x1C;
// Allow the strong pullup to be disabled, but default to enabled.
// If it was disabled a parasite powered device might not get the required
// current to copy the data from the scratchpad to EEPROM.  If it is enabled
// parasite powered devices have a better chance of getting the current
// required.
//
    let mut w1_strong_pullup: static int = 1;
    module_param_named(strong_pullup, w1_strong_pullup, int, 0);
// enable/disable CRC checking on DS28E04-100 memory accesses
    let mut w1_enable_crccheck: static bool = true;
pub const W1_EEPROM_SIZE: c_int = 512;
pub const W1_PAGE_COUNT: c_int = 16;
pub const W1_PAGE_SIZE: c_int = 32;
pub const W1_PAGE_BITS: c_int = 5;
pub const W1_PAGE_MASK: c_uint = 0x1F;
pub const W1_F1C_READ_EEPROM: c_uint = 0xF0;
pub const W1_F1C_WRITE_SCRATCH: c_uint = 0x0F;
pub const W1_F1C_READ_SCRATCH: c_uint = 0xAA;
pub const W1_F1C_COPY_SCRATCH: c_uint = 0x55;
pub const W1_F1C_ACCESS_WRITE: c_uint = 0x5A;
pub const W1_1C_REG_LOGIC_STATE: c_uint = 0x220;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct w1_f1C_data {
    pub memory: [u8; W1_EEPROM_SIZE],
    pub validcrc: u32,
}

//
// Check the file size bounds and adjusts count as needed.
// This would not be needed if the file size didn't reset to 0 after a write.
//
#[no_mangle]
pub unsafe extern "C" fn w1_f1C_fix_count(off: loff_t, count: usize, size: usize) -> usize {
    static inline size_t w1_f1C_fix_count(loff_t off, size_t count, size_t size)
    {
    if (off > size)
    return 0;
    if ((off + count) > size)
    return size - off;
    return count;
    }
    static int w1_f1C_refresh_block(struct w1_slave *sl, struct w1_f1C_data *data,
    int block)
    {
    u8	wrbuf[3];
    let mut off: c_int = block * W1_PAGE_SIZE;
    if (data.validcrc & (1 << block))
    return 0;
    if (w1_reset_select_slave(sl)) {
    data.validcrc = 0;
    return -EIO;
    }
    wrbuf[0] = W1_F1C_READ_EEPROM;
    wrbuf[1] = off & 0xff;
    wrbuf[2] = off >> 8;
    w1_write_block(sl.master, wrbuf, 3);
    w1_read_block(sl.master, &data.memory[off], W1_PAGE_SIZE);
// cache the block if the CRC is valid
    if (crc16(CRC16_INIT, &data.memory[off], W1_PAGE_SIZE) == CRC16_VALID)
    data.validcrc |= (1 << block);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn w1_f1C_read(sl: *mut w1_slave, addr: c_int, len: c_int, data: *mut c_char) -> c_int {
    static int w1_f1C_read(struct w1_slave *sl, int addr, int len, char *data)
    {
    u8 wrbuf[3];
// read directly from the EEPROM
    if (w1_reset_select_slave(sl))
    return -EIO;
    wrbuf[0] = W1_F1C_READ_EEPROM;
    wrbuf[1] = addr & 0xff;
    wrbuf[2] = addr >> 8;
    w1_write_block(sl.master, wrbuf, sizeof(wrbuf));
    return w1_read_block(sl.master, data, len);
    }
    static ssize_t eeprom_read(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr, char *buf,
    loff_t off, size_t count)
    {
    struct w1_slave *sl = kobj_to_w1_slave(kobj);
    struct w1_f1C_data *data = sl.family_data;
    int i, min_page, max_page;
    count = w1_f1C_fix_count(off, count, W1_EEPROM_SIZE);
    if (count == 0)
    return 0;
    mutex_lock(&sl.master.mutex);
    if (w1_enable_crccheck) {
    min_page = (off >> W1_PAGE_BITS);
    max_page = (off + count - 1) >> W1_PAGE_BITS;
    for (i = min_page; i <= max_page; i++) {
    if (w1_f1C_refresh_block(sl, data, i)) {
    count = -EIO;
    goto out_up;
    }
    }
    memcpy(buf, &data.memory[off], count);
    } else {
    count = w1_f1C_read(sl, off, count, buf);
    }
    out_up:
    mutex_unlock(&sl.master.mutex);
    return count;
    }
//
// w1_f1C_write() - Writes to the scratchpad and reads it back for verification.
// @sl:		The slave structure
// @addr:	Address for the write
// @len:	length must be <= (W1_PAGE_SIZE - (addr & W1_PAGE_MASK))
// @data:	The data to write
//
// Then copies the scratchpad to EEPROM.
// The data must be on one page.
// The master must be locked.
//
// Return:	0=Success, -1=failure
//
#[no_mangle]
unsafe extern "C" fn w1_f1C_write(sl: *mut w1_slave, addr: c_int, len: c_int, data: *const u8) -> c_int {
    static int w1_f1C_write(struct w1_slave *sl, int addr, int len, const u8 *data)
    {
    u8 wrbuf[4];
    u8 rdbuf[W1_PAGE_SIZE + 3];
    let mut es: u8 = (addr + len - 1) & 0x1f;
    let mut tm: c_uint = 10;
    int i;
    struct w1_f1C_data *f1C = sl.family_data;
// Write the data to the scratchpad
    if (w1_reset_select_slave(sl))
    return -1;
    wrbuf[0] = W1_F1C_WRITE_SCRATCH;
    wrbuf[1] = addr & 0xff;
    wrbuf[2] = addr >> 8;
    w1_write_block(sl.master, wrbuf, 3);
    w1_write_block(sl.master, data, len);
// Read the scratchpad and verify
    if (w1_reset_select_slave(sl))
    return -1;
    w1_write_8(sl.master, W1_F1C_READ_SCRATCH);
    w1_read_block(sl.master, rdbuf, len + 3);
// Compare what was read against the data written
    if ((rdbuf[0] != wrbuf[1]) || (rdbuf[1] != wrbuf[2]) ||
    (rdbuf[2] != es) || (memcmp(data, &rdbuf[3], len) != 0))
    return -1;
// Copy the scratchpad to EEPROM
    if (w1_reset_select_slave(sl))
    return -1;
    wrbuf[0] = W1_F1C_COPY_SCRATCH;
    wrbuf[3] = es;
    for (i = 0; i < sizeof(wrbuf); ++i) {
//
// issue 10ms strong pullup (or delay) on the last byte
// for writing the data from the scratchpad to EEPROM
//
    if (w1_strong_pullup && i == sizeof(wrbuf)-1)
    w1_next_pullup(sl.master, tm);
    w1_write_8(sl.master, wrbuf[i]);
    }
    if (!w1_strong_pullup)
    msleep(tm);
    if (w1_enable_crccheck) {
// invalidate cached data
    f1C.validcrc &= ~(1 << (addr >> W1_PAGE_BITS));
    }
// Reset the bus to wake up the EEPROM (this may not be needed)
    w1_reset_bus(sl.master);
    return 0;
    }
    static ssize_t eeprom_write(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr, char *buf,
    loff_t off, size_t count)
    {
    struct w1_slave *sl = kobj_to_w1_slave(kobj);
    int addr, len, idx;
    count = w1_f1C_fix_count(off, count, W1_EEPROM_SIZE);
    if (count == 0)
    return 0;
    if (w1_enable_crccheck) {
// can only write full blocks in cached mode
    if ((off & W1_PAGE_MASK) || (count & W1_PAGE_MASK)) {
    dev_err(&sl.dev, "invalid offset/count off=%d cnt=%zd\n",
    (int)off, count);
    return -EINVAL;
    }
// make sure the block CRCs are valid
    for (idx = 0; idx < count; idx += W1_PAGE_SIZE) {
    if (crc16(CRC16_INIT, &buf[idx], W1_PAGE_SIZE)
    != CRC16_VALID) {
    dev_err(&sl.dev, "bad CRC at offset %d\n",
    (int)off);
    return -EINVAL;
    }
    }
    }
    mutex_lock(&sl.master.mutex);
// Can only write data to one page at a time
    idx = 0;
    while (idx < count) {
    addr = off + idx;
    len = W1_PAGE_SIZE - (addr & W1_PAGE_MASK);
    if (len > (count - idx))
    len = count - idx;
    if (w1_f1C_write(sl, addr, len, &buf[idx]) < 0) {
    count = -EIO;
    goto out_up;
    }
    idx += len;
    }
    out_up:
    mutex_unlock(&sl.master.mutex);
    return count;
    }
    static const BIN_ATTR_RW(eeprom, W1_EEPROM_SIZE);
    static ssize_t pio_read(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr, char *buf, loff_t off,
    size_t count)
    {
    struct w1_slave *sl = kobj_to_w1_slave(kobj);
    int ret;
// check arguments
    if (off != 0 || count != 1 || buf == core::ptr::null_mut())
    return -EINVAL;
    mutex_lock(&sl.master.mutex);
    ret = w1_f1C_read(sl, W1_1C_REG_LOGIC_STATE, count, buf);
    mutex_unlock(&sl.master.mutex);
    return ret;
    }
    static ssize_t pio_write(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr, char *buf,
    loff_t off, size_t count)
    {
    struct w1_slave *sl = kobj_to_w1_slave(kobj);
    u8 wrbuf[3];
    u8 ack;
// check arguments
    if (off != 0 || count != 1 || buf == core::ptr::null_mut())
    return -EINVAL;
    mutex_lock(&sl.master.mutex);
// Write the PIO data
    if (w1_reset_select_slave(sl)) {
    mutex_unlock(&sl.master.mutex);
    return -1;
    }
// set bit 7..2 to value '1'
// buf = *buf | 0xFC;
    wrbuf[0] = W1_F1C_ACCESS_WRITE;
    wrbuf[1] = *buf;
    wrbuf[2] = ~(*buf);
    w1_write_block(sl.master, wrbuf, 3);
    w1_read_block(sl.master, &ack, sizeof(ack));
    mutex_unlock(&sl.master.mutex);
// check for acknowledgement
    if (ack != 0xAA)
    return -EIO;
    return count;
    }
    static const BIN_ATTR_RW(pio, 1);
    static ssize_t crccheck_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    return sysfs_emit(buf, "%d\n", w1_enable_crccheck);
    }
    static ssize_t crccheck_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    let mut err: c_int = kstrtobool(buf, &w1_enable_crccheck);
    if (err)
    return err;
    return count;
    }
    static DEVICE_ATTR_RW(crccheck);
    static struct attribute *w1_f1C_attrs[] = {
    &dev_attr_crccheck.attr,
    core::ptr::null_mut(),
    };
    static const struct bin_attribute *const w1_f1C_bin_attrs[] = {
    &bin_attr_eeprom,
    &bin_attr_pio,
    core::ptr::null_mut(),
    };
    static const struct attribute_group w1_f1C_group = {
    .attrs		= w1_f1C_attrs,
    .bin_attrs	= w1_f1C_bin_attrs,
    };
    static const struct attribute_group *w1_f1C_groups[] = {
    &w1_f1C_group,
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn w1_f1C_add_slave(sl: *mut w1_slave) -> c_int {
    static int w1_f1C_add_slave(struct w1_slave *sl)
    {
    struct w1_f1C_data *data = core::ptr::null_mut();
    if (w1_enable_crccheck) {
    data = kzalloc_obj(struct w1_f1C_data);
    if (!data)
    return -ENOMEM;
    sl.family_data = data;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn w1_f1C_remove_slave(sl: *mut w1_slave) {
    static void w1_f1C_remove_slave(struct w1_slave *sl)
    {
    kfree(sl.family_data);
    sl.family_data = core::ptr::null_mut();
    }
    static const struct w1_family_ops w1_f1C_fops = {
    .add_slave      = w1_f1C_add_slave,
    .remove_slave   = w1_f1C_remove_slave,
    .groups		= w1_f1C_groups,
    };
    static struct w1_family w1_family_1C = {
    .fid = W1_FAMILY_DS28E04,
    .fops = &w1_f1C_fops,
    };
    module_w1_family(w1_family_1C);
    MODULE_AUTHOR("Markus Franke <franke.m@sebakmt.com>, <franm@hrz.tu-chemnitz.de>");
    MODULE_DESCRIPTION("w1 family 1C driver for DS28E04, 4kb EEPROM and PIO");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("w1-family-" __stringify(W1_FAMILY_DS28E04));
