//! Automatically rewritten from C to Rust
//! Source: drivers/w1/slaves/w1_ds250x.c
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
// w1_ds250x.c - w1 family 09/0b/89/91 (DS250x) driver
//

pub const W1_DS2501_UNW_FAMILY: c_uint = 0x91;
pub const W1_DS2501_SIZE: c_int = 64;
pub const W1_DS2502_FAMILY: c_uint = 0x09;
pub const W1_DS2502_UNW_FAMILY: c_uint = 0x89;
pub const W1_DS2502_SIZE: c_int = 128;
pub const W1_DS2505_FAMILY: c_uint = 0x0b;
pub const W1_DS2505_SIZE: c_int = 2048;
pub const W1_PAGE_SIZE: c_int = 32;
pub const W1_EXT_READ_MEMORY: c_uint = 0xA5;
pub const W1_READ_DATA_CRC: c_uint = 0xC3;

pub const CRC16_INIT: c_int = 0;
pub const CRC16_VALID: c_uint = 0xb001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct w1_eprom_data {
    pub size: usize,
    pub pageno): *mut *mut *mut int (read)(struct w1_slave sl, int,
    pub eprom: [u8; W1_DS2505_SIZE],
    pub W1_PAGE_SIZE): DECLARE_BITMAP(page_present, W1_DS2505_SIZE /,
    pub nvmem_name: [c_char; 64],
}

#[no_mangle]
unsafe extern "C" fn w1_ds2502_read_page(sl: *mut w1_slave, pageno: c_int) -> c_int {
    static int w1_ds2502_read_page(struct w1_slave *sl, int pageno)
    {
    struct w1_eprom_data *data = sl.family_data;
    let mut pgoff: c_int = pageno * W1_PAGE_SIZE;
    let mut ret: c_int = -EIO;
    u8 buf[3];
    u8 crc8;
    if (test_bit(pageno, data.page_present))
    return 0; /* page already present */
    mutex_lock(&sl.master.bus_mutex);
    if (w1_reset_select_slave(sl))
    goto err;
    buf[0] = W1_READ_DATA_CRC;
    buf[1] = pgoff & 0xff;
    buf[2] = pgoff >> 8;
    w1_write_block(sl.master, buf, 3);
    crc8 = w1_read_8(sl.master);
    if (w1_calc_crc8(buf, 3) != crc8)
    goto err;
    w1_read_block(sl.master, &data.eprom[pgoff], W1_PAGE_SIZE);
    crc8 = w1_read_8(sl.master);
    if (w1_calc_crc8(&data.eprom[pgoff], W1_PAGE_SIZE) != crc8)
    goto err;
    set_bit(pageno, data.page_present); /* mark page present */
    ret = 0;
    err:
    mutex_unlock(&sl.master.bus_mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn w1_ds2505_read_page(sl: *mut w1_slave, pageno: c_int) -> c_int {
    static int w1_ds2505_read_page(struct w1_slave *sl, int pageno)
    {
    struct w1_eprom_data *data = sl.family_data;
    let mut redir_retries: c_int = 16;
    int pgoff, epoff;
    let mut ret: c_int = -EIO;
    u8 buf[6];
    u8 redir;
    u16 crc;
    if (test_bit(pageno, data.page_present))
    return 0; /* page already present */
    epoff = pgoff = pageno * W1_PAGE_SIZE;
    mutex_lock(&sl.master.bus_mutex);
    retry:
    if (w1_reset_select_slave(sl))
    goto err;
    buf[0] = W1_EXT_READ_MEMORY;
    buf[1] = pgoff & 0xff;
    buf[2] = pgoff >> 8;
    w1_write_block(sl.master, buf, 3);
    w1_read_block(sl.master, buf + 3, 3); /* redir, crc16 */
    redir = buf[3];
    crc = crc16(CRC16_INIT, buf, 6);
    if (crc != CRC16_VALID)
    goto err;
    if (redir != 0xff) {
    redir_retries--;
    if (redir_retries < 0)
    goto err;
    pgoff = (redir ^ 0xff) * W1_PAGE_SIZE;
    goto retry;
    }
    w1_read_block(sl.master, &data.eprom[epoff], W1_PAGE_SIZE);
    w1_read_block(sl.master, buf, 2); /* crc16 */
    crc = crc16(CRC16_INIT, &data.eprom[epoff], W1_PAGE_SIZE);
    crc = crc16(crc, buf, 2);
    if (crc != CRC16_VALID)
    goto err;
    set_bit(pageno, data.page_present);
    ret = 0;
    err:
    mutex_unlock(&sl.master.bus_mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn w1_nvmem_read(priv: *mut c_void, off: c_uint, buf: *mut c_void, count: usize) -> c_int {
    static int w1_nvmem_read(void *priv, unsigned int off, void *buf, size_t count)
    {
    struct w1_slave *sl = priv;
    struct w1_eprom_data *data = sl.family_data;
    let mut eprom_size: usize = data.size;
    int ret;
    int i;
    if (off > eprom_size)
    return -EINVAL;
    if ((off + count) > eprom_size)
    count = eprom_size - off;
    i = OFF2PG(off);
    do {
    ret = data.read(sl, i++);
    if (ret < 0)
    return ret;
    } while (i < OFF2PG(off + count));
    memcpy(buf, &data.eprom[off], count);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn w1_eprom_add_slave(sl: *mut w1_slave) -> c_int {
    static int w1_eprom_add_slave(struct w1_slave *sl)
    {
    struct w1_eprom_data *data;
    struct nvmem_device *nvmem;
    struct nvmem_config nvmem_cfg = {
    .dev = &sl.dev,
    .add_legacy_fixed_of_cells = true,
    .reg_read = w1_nvmem_read,
    .type = NVMEM_TYPE_OTP,
    .read_only = true,
    .word_size = 1,
    .priv = sl,
    .id = -1
    };
    data = devm_kzalloc(&sl.dev, sizeof(struct w1_eprom_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    sl.family_data = data;
    switch (sl.family.fid) {
    case W1_DS2501_UNW_FAMILY:
    data.size = W1_DS2501_SIZE;
    data.read = w1_ds2502_read_page;
    break;
    case W1_DS2502_FAMILY:
    case W1_DS2502_UNW_FAMILY:
    data.size = W1_DS2502_SIZE;
    data.read = w1_ds2502_read_page;
    break;
    case W1_DS2505_FAMILY:
    data.size = W1_DS2505_SIZE;
    data.read = w1_ds2505_read_page;
    break;
    }
    if (sl.master.bus_master.dev_id)
    snprintf(data.nvmem_name, sizeof(data.nvmem_name),
    "%s-%02x-%012llx",
    sl.master.bus_master.dev_id, sl.reg_num.family,
    (unsigned long long)sl.reg_num.id);
    else
    snprintf(data.nvmem_name, sizeof(data.nvmem_name),
    "%02x-%012llx",
    sl.reg_num.family,
    (unsigned long long)sl.reg_num.id);
    nvmem_cfg.name = data.nvmem_name;
    nvmem_cfg.size = data.size;
    nvmem = devm_nvmem_register(&sl.dev, &nvmem_cfg);
    return PTR_ERR_OR_ZERO(nvmem);
    }
    static const struct w1_family_ops w1_eprom_fops = {
    .add_slave	= w1_eprom_add_slave,
    };
    static struct w1_family w1_family_09 = {
    .fid = W1_DS2502_FAMILY,
    .fops = &w1_eprom_fops,
    };
    static struct w1_family w1_family_0b = {
    .fid = W1_DS2505_FAMILY,
    .fops = &w1_eprom_fops,
    };
    static struct w1_family w1_family_89 = {
    .fid = W1_DS2502_UNW_FAMILY,
    .fops = &w1_eprom_fops,
    };
    static struct w1_family w1_family_91 = {
    .fid = W1_DS2501_UNW_FAMILY,
    .fops = &w1_eprom_fops,
    };
#[no_mangle]
unsafe extern "C" fn w1_ds250x_init() -> int __init {
    static int __init w1_ds250x_init(void)
    {
    int err;
    err = w1_register_family(&w1_family_09);
    if (err)
    return err;
    err = w1_register_family(&w1_family_0b);
    if (err)
    goto err_0b;
    err = w1_register_family(&w1_family_89);
    if (err)
    goto err_89;
    err = w1_register_family(&w1_family_91);
    if (err)
    goto err_91;
    return 0;
    err_91:
    w1_unregister_family(&w1_family_89);
    err_89:
    w1_unregister_family(&w1_family_0b);
    err_0b:
    w1_unregister_family(&w1_family_09);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn w1_ds250x_exit() -> void __exit {
    static void __exit w1_ds250x_exit(void)
    {
    w1_unregister_family(&w1_family_09);
    w1_unregister_family(&w1_family_0b);
    w1_unregister_family(&w1_family_89);
    w1_unregister_family(&w1_family_91);
    }
    module_init(w1_ds250x_init);
    module_exit(w1_ds250x_exit);
    MODULE_AUTHOR("Thomas Bogendoerfer <tbogendoerfe@suse.de>");
    MODULE_DESCRIPTION("w1 family driver for DS250x Add Only Memory");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("w1-family-" __stringify(W1_DS2502_FAMILY));
    MODULE_ALIAS("w1-family-" __stringify(W1_DS2505_FAMILY));
    MODULE_ALIAS("w1-family-" __stringify(W1_DS2501_UNW_FAMILY));
    MODULE_ALIAS("w1-family-" __stringify(W1_DS2502_UNW_FAMILY));
