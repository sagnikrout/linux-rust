//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/devices/phram.c
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
// Copyright (c) ????		Jochen Schäuble <psionic@psionic.de>
// Copyright (c) 2003-2004	Joern Engel <joern@wh.fh-wedel.de>
//
// Usage:
//
// one commend line parameter per device, each in the form:
// phram=<name>,<start>,<len>[,<erasesize>]
// <name> may be up to 63 characters.
// <start>, <len>, and <erasesize> can be octal, decimal or hexadecimal.  If followed
// by "ki", "Mi" or "Gi", the numbers will be interpreted as kilo, mega or
// gigabytes. <erasesize> is optional and defaults to PAGE_SIZE.
//
// Example:
// phram=swap,64Mi,128Mi phram=test,900Mi,1Mi,64Ki
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phram_mtd_list {
    pub mtd: mtd_info,
    pub list: list_head,
    pub cached: bool,
}

    static LIST_HEAD(phram_list);
#[no_mangle]
unsafe extern "C" fn phram_erase(mtd: *mut mtd_info, instr: *mut erase_info) -> c_int {
    static int phram_erase(struct mtd_info *mtd, struct erase_info *instr)
    {
    u_char *start = mtd.priv;
    memset(start + instr.addr, 0xff, instr.len);
    return 0;
    }
    static int phram_point(struct mtd_info *mtd, loff_t from, size_t len,
    size_t *retlen, void **virt, resource_size_t *phys)
    {
// virt = mtd->priv + from;
// retlen = len;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn phram_unpoint(mtd: *mut mtd_info, from: loff_t, len: usize) -> c_int {
    static int phram_unpoint(struct mtd_info *mtd, loff_t from, size_t len)
    {
    return 0;
    }
    static int phram_read(struct mtd_info *mtd, loff_t from, size_t len,
    size_t *retlen, u_char *buf)
    {
    u_char *start = mtd.priv;
    memcpy(buf, start + from, len);
// retlen = len;
    return 0;
    }
    static int phram_write(struct mtd_info *mtd, loff_t to, size_t len,
    size_t *retlen, const u_char *buf)
    {
    u_char *start = mtd.priv;
    memcpy(start + to, buf, len);
// retlen = len;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn phram_map(phram: *mut phram_mtd_list, start: phys_addr_t, len: usize) -> c_int {
    static int phram_map(struct phram_mtd_list *phram, phys_addr_t start, size_t len)
    {
    void *addr = core::ptr::null_mut();
    if (phram.cached)
    addr = memremap(start, len, MEMREMAP_WB);
    else
    addr = (void  *)ioremap(start, len);
    if (!addr)
    return -EIO;
    phram.mtd.priv = addr;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn phram_unmap(phram: *mut phram_mtd_list) {
    static void phram_unmap(struct phram_mtd_list *phram)
    {
    void *addr = phram.mtd.priv;
    if (phram.cached) {
    memunmap(addr);
    return;
    }
    iounmap((void __iomem *)addr);
    }
#[no_mangle]
unsafe extern "C" fn unregister_devices() {
    static void unregister_devices(void)
    {
    struct phram_mtd_list *this, *safe;
    list_for_each_entry_safe(this, safe, &phram_list, list) {
    mtd_device_unregister(&this.mtd);
    phram_unmap(this);
    kfree(this.mtd.name);
    kfree(this);
    }
    }
    static int register_device(struct platform_device *pdev, const char *name,
    phys_addr_t start, size_t len, uint32_t erasesize)
    {
    struct device_node *np = pdev ? pdev.dev.of_node : core::ptr::null_mut();
    let mut cached: bool = np ? !of_property_read_bool(np, "no-map") : false;
    struct phram_mtd_list *new;
    let mut ret: c_int = -ENOMEM;
    new = kzalloc_obj(*new);
    if (!new)
    goto out0;
    new.cached = cached;
    ret = phram_map(new, start, len);
    if (ret) {
    pr_err("ioremap failed\n");
    goto out1;
    }
    new.mtd.name = name;
    new.mtd.size = len;
    new.mtd.flags = MTD_CAP_RAM;
    new.mtd._erase = phram_erase;
    new.mtd._point = phram_point;
    new.mtd._unpoint = phram_unpoint;
    new.mtd._read = phram_read;
    new.mtd._write = phram_write;
    new.mtd.owner = THIS_MODULE;
    new.mtd.type = MTD_RAM;
    new.mtd.erasesize = erasesize;
    new.mtd.writesize = 1;
    mtd_set_of_node(&new.mtd, np);
    ret = -EAGAIN;
    if (mtd_device_register(&new.mtd, core::ptr::null_mut(), 0)) {
    pr_err("Failed to register new device\n");
    goto out2;
    }
    if (pdev)
    platform_set_drvdata(pdev, new);
    else
    list_add_tail(&new.list, &phram_list);
    return 0;
    out2:
    phram_unmap(new);
    out1:
    kfree(new);
    out0:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn parse_num64(num64: *mut u64, token: *mut c_char) -> c_int {
    static int parse_num64(uint64_t *num64, char *token)
    {
    size_t len;
    let mut shift: c_int = 0;
    int ret;
    len = strlen(token);
// By dwmw2 editorial decree, "ki", "Mi" or "Gi" are to be used.
    if (len > 2) {
    if (token[len - 1] == 'i') {
    switch (token[len - 2]) {
    case 'G':
    shift += 10;
    fallthrough;
    case 'M':
    shift += 10;
    fallthrough;
    case 'k':
    shift += 10;
    token[len - 2] = 0;
    break;
    default:
    return -EINVAL;
    }
    }
    }
    ret = kstrtou64(token, 0, num64);
// num64 <<= shift;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn parse_name(pname: *mut c_char, token: *const c_char) -> c_int {
    static int parse_name(char **pname, const char *token)
    {
    size_t len;
    char *name;
    len = strlen(token) + 1;
    if (len > 64)
    return -ENOSPC;
    name = kstrdup(token, GFP_KERNEL);
    if (!name)
    return -ENOMEM;
// pname = name;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kill_final_newline(str: *mut c_char) {
    static inline void kill_final_newline(char *str)
    {
    char *newline = strrchr(str, '\n');
    if (newline && !newline[1])
// newline = 0;
    }

    pr_err(fmt , ## args);	\
    return 1;		\
    } while (0)

    static int phram_init_called;
//
// This shall contain the module parameter if any. It is of the form:
// - phram=<device>,<address>,<size>[,<erasesize>] for module case
// - phram.phram=<device>,<address>,<size>[,<erasesize>] for built-in case
// We leave 64 bytes for the device name, 20 for the address , 20 for the
// size and 20 for the erasesize.
// Example: phram.phram=rootfs,0xa0000000,512Mi,65536
//
    static char phram_paramline[64 + 20 + 20 + 20];

#[no_mangle]
unsafe extern "C" fn phram_setup(val: *const c_char) -> c_int {
    static int phram_setup(const char *val)
    {
    char buf[64 + 20 + 20 + 20], *str = buf;
    char *token[4];
    char *name;
    uint64_t start;
    uint64_t len;
    let mut erasesize: u64 = PAGE_SIZE;
    uint32_t rem;
    int i, ret;
    if (strnlen(val, sizeof(buf)) >= sizeof(buf))
    parse_err("parameter too long\n");
    strcpy(str, val);
    kill_final_newline(str);
    for (i = 0; i < 4; i++)
    token[i] = strsep(&str, ",");
    if (str)
    parse_err("too many arguments\n");
    if (!token[2])
    parse_err("not enough arguments\n");
    ret = parse_name(&name, token[0]);
    if (ret)
    return ret;
    ret = parse_num64(&start, token[1]);
    if (ret) {
    parse_err("illegal start address\n");
    goto error;
    }
    ret = parse_num64(&len, token[2]);
    if (ret) {
    parse_err("illegal device length\n");
    goto error;
    }
    if (token[3]) {
    ret = parse_num64(&erasesize, token[3]);
    if (ret) {
    parse_err("illegal erasesize\n");
    goto error;
    }
    }
    if (len == 0 || erasesize == 0 || erasesize > len
    || erasesize > UINT_MAX) {
    parse_err("illegal erasesize or len\n");
    ret = -EINVAL;
    goto error;
    }
    div_u64_rem(len, (uint32_t)erasesize, &rem);
    if (rem) {
    parse_err("len is not multiple of erasesize\n");
    ret = -EINVAL;
    goto error;
    }
    ret = register_device(core::ptr::null_mut(), name, start, len, (uint32_t)erasesize);
    if (ret)
    goto error;
    pr_info("%s device: %#llx at %#llx for erasesize %#llx\n", name, len, start, erasesize);
    return 0;
    error:
    kfree(name);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn phram_param_call(val: *const c_char, kp: *const kernel_param) -> c_int {
    static int phram_param_call(const char *val, const struct kernel_param *kp)
    {

    return phram_setup(val);

//
// If more parameters are later passed in via
// /sys/module/phram/parameters/phram
// and init_phram() has already been called,
// we can parse the argument now.
//
    if (phram_init_called)
    return phram_setup(val);
//
// During early boot stage, we only save the parameters
// here. We must parse them later: if the param passed
// from kernel boot command line, phram_param_call() is
// called so early that it is not possible to resolve
// the device (even kmalloc() fails). Defer that work to
// phram_setup().
//
    if (strlen(val) >= sizeof(phram_paramline))
    return -ENOSPC;
    strcpy(phram_paramline, val);
    return 0;

    }
    module_param_call(phram, phram_param_call, core::ptr::null_mut(), core::ptr::null_mut(), 0200);
    MODULE_PARM_DESC(phram, "Memory region to map. \"phram=<name>,<start>,<length>[,<erasesize>]\"");

    static const struct of_device_id phram_of_match[] = {
    { .compatible = "phram" },
    {}
    };
    MODULE_DEVICE_TABLE(of, phram_of_match);

#[no_mangle]
unsafe extern "C" fn phram_probe(pdev: *mut platform_device) -> c_int {
    static int phram_probe(struct platform_device *pdev)
    {
    struct resource *res;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -ENOMEM;
// mtd_set_of_node() reads name from "label"
    return register_device(pdev, core::ptr::null_mut(), res.start, resource_size(res),
    PAGE_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn phram_remove(pdev: *mut platform_device) {
    static void phram_remove(struct platform_device *pdev)
    {
    struct phram_mtd_list *phram = platform_get_drvdata(pdev);
    mtd_device_unregister(&phram.mtd);
    phram_unmap(phram);
    kfree(phram);
    }
    static struct platform_driver phram_driver = {
    .probe		= phram_probe,
    .remove		= phram_remove,
    .driver		= {
    .name		= "phram",
    .of_match_table	= of_match_ptr(phram_of_match),
    },
    };
#[no_mangle]
unsafe extern "C" fn init_phram() -> int __init {
    static int __init init_phram(void)
    {
    int ret;
    ret = security_locked_down(LOCKDOWN_DEV_MEM);
    if (ret)
    return ret;
    ret = platform_driver_register(&phram_driver);
    if (ret)
    return ret;

    if (phram_paramline[0]) {
    ret = phram_setup(phram_paramline);
    if (ret)
    platform_driver_unregister(&phram_driver);
    }
    phram_init_called = 1;

    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cleanup_phram() -> void __exit {
    static void __exit cleanup_phram(void)
    {
    unregister_devices();
    platform_driver_unregister(&phram_driver);
    }
    module_init(init_phram);
    module_exit(cleanup_phram);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Joern Engel <joern@wh.fh-wedel.de>");
    MODULE_DESCRIPTION("MTD driver for physical RAM");
