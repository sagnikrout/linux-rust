//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/broadcom/bcm47xx_nvram.c
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
// BCM947xx nvram variable access
//
// Copyright (C) 2005 Broadcom Corporation
// Copyright (C) 2006 Felix Fietkau <nbd@openwrt.org>
// Copyright (C) 2010-2012 Hauke Mehrtens <hauke@hauke-m.de>
//

pub const NVRAM_MAGIC: c_uint = 0x48534C46	/* 'FLSH' */;
pub const NVRAM_SPACE: c_uint = 0x10000;
pub const NVRAM_MAX_GPIO_ENTRIES: c_int = 32;
pub const NVRAM_MAX_GPIO_VALUE_LEN: c_int = 30;
pub const FLASH_MIN: c_uint = 0x00020000	/* Minimum flash size */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvram_header {
    pub magic: u32,
    pub len: u32,
    pub /: *mut *mut u32 crc_ver_init; / 0:7 crc, 8:15 ver, 16:31 sdram_init,
    pub /: *mut *mut u32 config_refresh; / 0:15 sdram_config, 16:31 sdram_refresh,
    pub /: *mut *mut u32 config_ncdl; / ncdl values for memc,
}

    static char nvram_buf[NVRAM_SPACE];
    static size_t nvram_len;
    static const u32 nvram_sizes[] = {0x6000, 0x8000, 0xF000, 0x10000};
//
// bcm47xx_nvram_is_valid - check for a valid NVRAM at specified memory
//
#[no_mangle]
unsafe extern "C" fn bcm47xx_nvram_is_valid(nvram: *mut void __iomem) -> bool {
    static bool bcm47xx_nvram_is_valid(void __iomem *nvram)
    {
    return ((struct nvram_header *)nvram).magic == NVRAM_MAGIC;
    }
//
// bcm47xx_nvram_copy - copy NVRAM to internal buffer
//
#[no_mangle]
unsafe extern "C" fn bcm47xx_nvram_copy(nvram_start: *mut void __iomem, res_size: usize) {
    static void bcm47xx_nvram_copy(void __iomem *nvram_start, size_t res_size)
    {
    struct nvram_header __iomem *header = nvram_start;
    size_t copy_size;
    copy_size = header.len;
    if (copy_size > res_size) {
    pr_err("The nvram size according to the header seems to be bigger than the partition on flash\n");
    copy_size = res_size;
    }
    if (copy_size >= NVRAM_SPACE) {
    pr_err("nvram on flash (%zu bytes) is bigger than the reserved space in memory, will just copy the first %i bytes\n",
    copy_size, NVRAM_SPACE - 1);
    copy_size = NVRAM_SPACE - 1;
    }
    __ioread32_copy(nvram_buf, nvram_start, DIV_ROUND_UP(copy_size, 4));
    nvram_buf[NVRAM_SPACE - 1] = '\0';
    nvram_len = copy_size;
    }
//
// bcm47xx_nvram_find_and_copy - find NVRAM on flash mapping & copy it
//
#[no_mangle]
unsafe extern "C" fn bcm47xx_nvram_find_and_copy(flash_start: *mut void __iomem, res_size: usize) -> c_int {
    static int bcm47xx_nvram_find_and_copy(void __iomem *flash_start, size_t res_size)
    {
    size_t flash_size;
    size_t offset;
    int i;
    if (nvram_len) {
    pr_warn("nvram already initialized\n");
    return -EEXIST;
    }
// TODO: when nvram is on nand flash check for bad blocks first.
// Try every possible flash size and check for NVRAM at its end
    for (flash_size = FLASH_MIN; flash_size <= res_size; flash_size <<= 1) {
    for (i = 0; i < ARRAY_SIZE(nvram_sizes); i++) {
    offset = flash_size - nvram_sizes[i];
    if (bcm47xx_nvram_is_valid(flash_start + offset))
    goto found;
    }
    }
// Try embedded NVRAM at 4 KB and 1 KB as last resorts
    offset = 4096;
    if (bcm47xx_nvram_is_valid(flash_start + offset))
    goto found;
    offset = 1024;
    if (bcm47xx_nvram_is_valid(flash_start + offset))
    goto found;
    pr_err("no nvram found\n");
    return -ENXIO;
    found:
    bcm47xx_nvram_copy(flash_start + offset, res_size - offset);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bcm47xx_nvram_init_from_iomem(nvram_start: *mut void __iomem, res_size: usize) -> c_int {
    int bcm47xx_nvram_init_from_iomem(void __iomem *nvram_start, size_t res_size)
    {
    if (nvram_len) {
    pr_warn("nvram already initialized\n");
    return -EEXIST;
    }
    if (!bcm47xx_nvram_is_valid(nvram_start)) {
    pr_err("No valid NVRAM found\n");
    return -ENOENT;
    }
    bcm47xx_nvram_copy(nvram_start, res_size);
    return 0;
    }
    EXPORT_SYMBOL_GPL(bcm47xx_nvram_init_from_iomem);
//
// On bcm47xx we need access to the NVRAM very early, so we can't use mtd
// subsystem to access flash. We can't even use platform device / driver to
// store memory offset.
// To handle this we provide following symbol. It's supposed to be called as
// soon as we get info about flash device, before any NVRAM entry is needed.
//
#[no_mangle]
pub unsafe extern "C" fn bcm47xx_nvram_init_from_mem(base: u32, lim: u32) -> c_int {
    int bcm47xx_nvram_init_from_mem(u32 base, u32 lim)
    {
    void __iomem *iobase;
    int err;
    iobase = ioremap(base, lim);
    if (!iobase)
    return -ENOMEM;
    err = bcm47xx_nvram_find_and_copy(iobase, lim);
    iounmap(iobase);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn nvram_init() -> c_int {
    static int nvram_init(void)
    {

    struct mtd_info *mtd;
    struct nvram_header header;
    size_t bytes_read;
    int err;
    mtd = get_mtd_device_nm("nvram");
    if (IS_ERR(mtd))
    return -ENODEV;
    err = mtd_read(mtd, 0, sizeof(header), &bytes_read, (uint8_t *)&header);
    if (!err && header.magic == NVRAM_MAGIC &&
    header.len > sizeof(header)) {
    nvram_len = header.len;
    if (nvram_len >= NVRAM_SPACE) {
    pr_err("nvram on flash (%zu bytes) is bigger than the reserved space in memory, will just copy the first %i bytes\n",
    nvram_len, NVRAM_SPACE);
    nvram_len = NVRAM_SPACE - 1;
    }
    err = mtd_read(mtd, 0, nvram_len, &nvram_len,
    (u8 *)nvram_buf);
    return err;
    }

    return -ENXIO;
    }
#[no_mangle]
pub unsafe extern "C" fn bcm47xx_nvram_getenv(name: *const c_char, val: *mut c_char, val_len: usize) -> c_int {
    int bcm47xx_nvram_getenv(const char *name, char *val, size_t val_len)
    {
    char *var, *value, *end, *eq;
    int err;
    if (!name)
    return -EINVAL;
    if (!nvram_len) {
    err = nvram_init();
    if (err)
    return err;
    }
// Look for name=value and return value
    var = &nvram_buf[sizeof(struct nvram_header)];
    end = nvram_buf + sizeof(nvram_buf);
    while (var < end && *var) {
    eq = strchr(var, '=');
    if (!eq)
    break;
    value = eq + 1;
    if (eq - var == strlen(name) &&
    strncmp(var, name, eq - var) == 0)
    return snprintf(val, val_len, "%s", value);
    var = value + strlen(value) + 1;
    }
    return -ENOENT;
    }
    EXPORT_SYMBOL(bcm47xx_nvram_getenv);
#[no_mangle]
pub unsafe extern "C" fn bcm47xx_nvram_gpio_pin(name: *const c_char) -> c_int {
    int bcm47xx_nvram_gpio_pin(const char *name)
    {
    int i, err;
    char nvram_var[] = "gpioXX";
    char buf[NVRAM_MAX_GPIO_VALUE_LEN];
// TODO: Optimize it to don't call getenv so many times
    for (i = 0; i < NVRAM_MAX_GPIO_ENTRIES; i++) {
    err = snprintf(nvram_var, sizeof(nvram_var), "gpio%i", i);
    if (err <= 0)
    continue;
    err = bcm47xx_nvram_getenv(nvram_var, buf, sizeof(buf));
    if (err <= 0)
    continue;
    if (!strcmp(name, buf))
    return i;
    }
    return -ENOENT;
    }
    EXPORT_SYMBOL(bcm47xx_nvram_gpio_pin);
    char *bcm47xx_nvram_get_contents(size_t *nvram_size)
    {
    int err;
    char *nvram;
    if (!nvram_len) {
    err = nvram_init();
    if (err)
    return core::ptr::null_mut();
    }
// nvram_size = nvram_len - sizeof(struct nvram_header);
    nvram = vmalloc(*nvram_size);
    if (!nvram)
    return core::ptr::null_mut();
    memcpy(nvram, &nvram_buf[sizeof(struct nvram_header)], *nvram_size);
    return nvram;
    }
    EXPORT_SYMBOL(bcm47xx_nvram_get_contents);
