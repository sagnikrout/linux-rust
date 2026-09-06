//! Automatically rewritten from C to Rust
//! Source: drivers/soc/fsl/dpaa2-console.c
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
//
// Freescale DPAA2 Platforms Console Driver
//
// Copyright 2015-2016 Freescale Semiconductor Inc.
// Copyright 2018 NXP
//

// MC firmware base low/high registers indexes
pub const MCFBALR_OFFSET: c_int = 0;
pub const MCFBAHR_OFFSET: c_int = 1;
// Bit masks used to get the most/least significant part of the MC base addr
pub const MC_FW_ADDR_MASK_HIGH: c_uint = 0x1FFFF;
pub const MC_FW_ADDR_MASK_LOW: c_uint = 0xE0000000;
pub const MC_BUFFER_OFFSET: c_uint = 0x01000000;

pub const AIOP_BUFFER_OFFSET: c_uint = 0x06000000;

pub const AIOP_OFFSET_DELTA: c_int = 0;
pub const LOG_HEADER_FLAG_BUFFER_WRAPAROUND: c_uint = 0x80000000;

// MC and AIOP Magic words
pub const MAGIC_MC: c_uint = 0x4d430100;
pub const MAGIC_AIOP: c_uint = 0x41494F50;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct log_header {
    pub magic_word: __le32,
    pub reserved: [c_char; 4],
    pub buf_start: __le32,
    pub buf_length: __le32,
    pub last_byte: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct console_data {
    pub map_addr: *mut void __iomem,
    pub hdr: *mut log_header __iomem,
    pub start_addr: *mut void __iomem,
    pub end_addr: *mut void __iomem,
    pub end_of_data: *mut void __iomem,
    pub cur_ptr: *mut void __iomem,
}

    static struct resource mc_base_addr;
#[no_mangle]
pub unsafe extern "C" fn adjust_end(cd: *mut console_data) {
    static inline void adjust_end(struct console_data *cd)
    {
    let mut last_byte: u32 = readl(&cd.hdr.last_byte);
    cd.end_of_data = cd.start_addr + LAST_BYTE(last_byte);
    }
#[no_mangle]
unsafe extern "C" fn get_mc_fw_base_address() -> u64 {
    static u64 get_mc_fw_base_address(void)
    {
    let mut mcfwbase: u64 = 0ULL;
    u32 __iomem *mcfbaregs;
    mcfbaregs = ioremap(mc_base_addr.start, resource_size(&mc_base_addr));
    if (!mcfbaregs) {
    pr_err("could not map MC Firmware Base registers\n");
    return 0;
    }
    mcfwbase  = readl(mcfbaregs + MCFBAHR_OFFSET) &
    MC_FW_ADDR_MASK_HIGH;
    mcfwbase <<= 32;
    mcfwbase |= readl(mcfbaregs + MCFBALR_OFFSET) & MC_FW_ADDR_MASK_LOW;
    iounmap(mcfbaregs);
    pr_debug("MC base address at 0x%016llx\n", mcfwbase);
    return mcfwbase;
    }
#[no_mangle]
unsafe extern "C" fn dpaa2_console_size(cd: *mut console_data) -> isize {
    static ssize_t dpaa2_console_size(struct console_data *cd)
    {
    ssize_t size;
    if (cd.cur_ptr <= cd.end_of_data)
    size = cd.end_of_data - cd.cur_ptr;
    else
    size = (cd.end_addr - cd.cur_ptr) +
    (cd.end_of_data - cd.start_addr);
    return size;
    }
    static int dpaa2_generic_console_open(struct inode *node, struct file *fp,
    u64 offset, u64 size,
    u32 expected_magic,
    u32 offset_delta)
    {
    u32 read_magic, wrapped, last_byte, buf_start, buf_length;
    struct console_data *cd;
    u64 base_addr;
    int err;
    cd = kmalloc_obj(*cd);
    if (!cd)
    return -ENOMEM;
    base_addr = get_mc_fw_base_address();
    if (!base_addr) {
    err = -EIO;
    goto err_fwba;
    }
    cd.map_addr = ioremap(base_addr + offset, size);
    if (!cd.map_addr) {
    pr_err("cannot map console log memory\n");
    err = -EIO;
    goto err_ioremap;
    }
    cd.hdr = (struct log_header __iomem *)cd.map_addr;
    read_magic = readl(&cd.hdr.magic_word);
    last_byte =  readl(&cd.hdr.last_byte);
    buf_start =  readl(&cd.hdr.buf_start);
    buf_length = readl(&cd.hdr.buf_length);
    if (read_magic != expected_magic) {
    pr_warn("expected = %08x, read = %08x\n",
    expected_magic, read_magic);
    err = -EIO;
    goto err_magic;
    }
    cd.start_addr = cd.map_addr + buf_start - offset_delta;
    cd.end_addr = cd.start_addr + buf_length;
    wrapped = last_byte & LOG_HEADER_FLAG_BUFFER_WRAPAROUND;
    adjust_end(cd);
    if (wrapped && cd.end_of_data != cd.end_addr)
    cd.cur_ptr = cd.end_of_data + 1;
    else
    cd.cur_ptr = cd.start_addr;
    fp.private_data = cd;
    return 0;
    err_magic:
    iounmap(cd.map_addr);
    err_ioremap:
    err_fwba:
    kfree(cd);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn dpaa2_mc_console_open(node: *mut inode, fp: *mut file) -> c_int {
    static int dpaa2_mc_console_open(struct inode *node, struct file *fp)
    {
    return dpaa2_generic_console_open(node, fp,
    MC_BUFFER_OFFSET, MC_BUFFER_SIZE,
    MAGIC_MC, MC_OFFSET_DELTA);
    }
#[no_mangle]
unsafe extern "C" fn dpaa2_aiop_console_open(node: *mut inode, fp: *mut file) -> c_int {
    static int dpaa2_aiop_console_open(struct inode *node, struct file *fp)
    {
    return dpaa2_generic_console_open(node, fp,
    AIOP_BUFFER_OFFSET, AIOP_BUFFER_SIZE,
    MAGIC_AIOP, AIOP_OFFSET_DELTA);
    }
#[no_mangle]
unsafe extern "C" fn dpaa2_console_close(node: *mut inode, fp: *mut file) -> c_int {
    static int dpaa2_console_close(struct inode *node, struct file *fp)
    {
    struct console_data *cd = fp.private_data;
    iounmap(cd.map_addr);
    kfree(cd);
    return 0;
    }
    static ssize_t dpaa2_console_read(struct file *fp, char __user *buf,
    size_t count, loff_t *f_pos)
    {
    struct console_data *cd = fp.private_data;
    let mut bytes: usize = dpaa2_console_size(cd);
    let mut bytes_end: usize = cd.end_addr - cd.cur_ptr;
    let mut written: usize = 0;
    void *kbuf;
    int err;
// Check if we need to adjust the end of data addr
    adjust_end(cd);
    if (cd.end_of_data == cd.cur_ptr)
    return 0;
    if (count < bytes)
    bytes = count;
    kbuf = kmalloc(bytes, GFP_KERNEL);
    if (!kbuf)
    return -ENOMEM;
    if (bytes > bytes_end) {
    memcpy_fromio(kbuf, cd.cur_ptr, bytes_end);
    if (copy_to_user(buf, kbuf, bytes_end)) {
    err = -EFAULT;
    goto err_free_buf;
    }
    buf += bytes_end;
    cd.cur_ptr = cd.start_addr;
    bytes -= bytes_end;
    written += bytes_end;
    }
    memcpy_fromio(kbuf, cd.cur_ptr, bytes);
    if (copy_to_user(buf, kbuf, bytes)) {
    err = -EFAULT;
    goto err_free_buf;
    }
    cd.cur_ptr += bytes;
    written += bytes;
    kfree(kbuf);
    return written;
    err_free_buf:
    kfree(kbuf);
    return err;
    }
    static const struct file_operations dpaa2_mc_console_fops = {
    .owner          = THIS_MODULE,
    .open           = dpaa2_mc_console_open,
    .release        = dpaa2_console_close,
    .read           = dpaa2_console_read,
    };
    static struct miscdevice dpaa2_mc_console_dev = {
    .minor = MISC_DYNAMIC_MINOR,
    .name = "dpaa2_mc_console",
    .fops = &dpaa2_mc_console_fops
    };
    static const struct file_operations dpaa2_aiop_console_fops = {
    .owner          = THIS_MODULE,
    .open           = dpaa2_aiop_console_open,
    .release        = dpaa2_console_close,
    .read           = dpaa2_console_read,
    };
    static struct miscdevice dpaa2_aiop_console_dev = {
    .minor = MISC_DYNAMIC_MINOR,
    .name = "dpaa2_aiop_console",
    .fops = &dpaa2_aiop_console_fops
    };
#[no_mangle]
unsafe extern "C" fn dpaa2_console_probe(pdev: *mut platform_device) -> c_int {
    static int dpaa2_console_probe(struct platform_device *pdev)
    {
    int error;
    error = of_address_to_resource(pdev.dev.of_node, 0, &mc_base_addr);
    if (error < 0) {
    pr_err("of_address_to_resource() failed for %pOF with %d\n",
    pdev.dev.of_node, error);
    return error;
    }
    error = misc_register(&dpaa2_mc_console_dev);
    if (error) {
    pr_err("cannot register device %s\n",
    dpaa2_mc_console_dev.name);
    goto err_register_mc;
    }
    error = misc_register(&dpaa2_aiop_console_dev);
    if (error) {
    pr_err("cannot register device %s\n",
    dpaa2_aiop_console_dev.name);
    goto err_register_aiop;
    }
    return 0;
    err_register_aiop:
    misc_deregister(&dpaa2_mc_console_dev);
    err_register_mc:
    return error;
    }
#[no_mangle]
unsafe extern "C" fn dpaa2_console_remove(pdev: *mut platform_device) {
    static void dpaa2_console_remove(struct platform_device *pdev)
    {
    misc_deregister(&dpaa2_mc_console_dev);
    misc_deregister(&dpaa2_aiop_console_dev);
    }
    static const struct of_device_id dpaa2_console_match_table[] = {
    { .compatible = "fsl,dpaa2-console",},
    {},
    };
    MODULE_DEVICE_TABLE(of, dpaa2_console_match_table);
    static struct platform_driver dpaa2_console_driver = {
    .driver = {
    .name = "dpaa2-console",
    .pm = core::ptr::null_mut(),
    .of_match_table = dpaa2_console_match_table,
    },
    .probe = dpaa2_console_probe,
    .remove = dpaa2_console_remove,
    };
    module_platform_driver(dpaa2_console_driver);
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_AUTHOR("Roy Pledge <roy.pledge@nxp.com>");
    MODULE_DESCRIPTION("DPAA2 console driver");
