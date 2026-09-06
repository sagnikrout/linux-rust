//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/devices/slram.c
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
// ======================================================================
    This driver provides a method to access memory not used by the kernel
    itself (i.e. if the kernel commandline mem=xxx is used). To actually
    use slram at least mtdblock or mtdchar is required (for block or
    character device access).
    Usage:
    if compiled as loadable module:
    modprobe slram map=<name>,<start>,<end/offset>
    if statically linked into the kernel use the following kernel cmd.line
    slram=<name>,<start>,<end/offset>
    <name>: name of the device that will be listed in /proc/mtd
    <start>: start of the memory region, decimal or hex (0xabcdef)
    <end/offset>: end of the memory region. It's possible to use +0x1234
    to specify the offset instead of the absolute address
    NOTE:
    With slram it's only possible to map a contiguous memory region. Therefore
    if there's a device mapped somewhere in the region specified slram will
    fail to load (see kernel log if modprobe fails).
    -
    Jochen Schaeuble <psionic@psionic.de>
    ======================================================================*/

pub const SLRAM_BLK_SZ: c_uint = 0x4000;

    typedef struct slram_priv {
    u_char *start;
    u_char *end;
    } slram_priv_t;
    typedef struct slram_mtd_list {
    struct mtd_info *mtdinfo;
    struct slram_mtd_list *next;
    } slram_mtd_list_t;

    static char *map[SLRAM_MAX_DEVICES_PARAMS];
    module_param_array(map, charp, core::ptr::null_mut(), 0);
    MODULE_PARM_DESC(map, "List of memory regions to map. \"map=<name>, <start>, <length / end>\"");

    static char *map;

    static slram_mtd_list_t *slram_mtdlist = core::ptr::null_mut();
    static int slram_erase(struct mtd_info *, struct erase_info *);
    static int slram_point(struct mtd_info *, loff_t, size_t, size_t *, void **,
    resource_size_t *);
    static int slram_unpoint(struct mtd_info *, loff_t, size_t);
    static int slram_read(struct mtd_info *, loff_t, size_t, size_t *, u_char *);
    static int slram_write(struct mtd_info *, loff_t, size_t, size_t *, const u_char *);
#[no_mangle]
unsafe extern "C" fn slram_erase(mtd: *mut mtd_info, instr: *mut erase_info) -> c_int {
    static int slram_erase(struct mtd_info *mtd, struct erase_info *instr)
    {
    slram_priv_t *priv = mtd.priv;
    memset(priv.start + instr.addr, 0xff, instr.len);
    return(0);
    }
    static int slram_point(struct mtd_info *mtd, loff_t from, size_t len,
    size_t *retlen, void **virt, resource_size_t *phys)
    {
    slram_priv_t *priv = mtd.priv;
// virt = priv->start + from;
// retlen = len;
    return(0);
    }
#[no_mangle]
unsafe extern "C" fn slram_unpoint(mtd: *mut mtd_info, from: loff_t, len: usize) -> c_int {
    static int slram_unpoint(struct mtd_info *mtd, loff_t from, size_t len)
    {
    return 0;
    }
    static int slram_read(struct mtd_info *mtd, loff_t from, size_t len,
    size_t *retlen, u_char *buf)
    {
    slram_priv_t *priv = mtd.priv;
    memcpy(buf, priv.start + from, len);
// retlen = len;
    return(0);
    }
    static int slram_write(struct mtd_info *mtd, loff_t to, size_t len,
    size_t *retlen, const u_char *buf)
    {
    slram_priv_t *priv = mtd.priv;
    memcpy(priv.start + to, buf, len);
// retlen = len;
    return(0);
    }
// ====================================================================
#[no_mangle]
unsafe extern "C" fn register_device(name: *mut c_char, start: c_ulong, length: c_ulong) -> c_int {
    static int register_device(char *name, unsigned long start, unsigned long length)
    {
    slram_mtd_list_t **curmtd;
    slram_mtd_list_t *new_mtd;
    struct mtd_info *mtdinfo;
    slram_priv_t *priv;
    let mut ret: c_int = -ENOMEM;
    curmtd = &slram_mtdlist;
    while (*curmtd) {
    curmtd = &(*curmtd).next;
    }
    new_mtd = kmalloc_obj(slram_mtd_list_t);
    if (!new_mtd) {
    E("slram: Cannot allocate new MTD device.\n");
    return(-ENOMEM);
    }
    new_mtd.next = core::ptr::null_mut();
    mtdinfo = kzalloc_obj(struct mtd_info);
    if (!mtdinfo) {
    E("slram: Cannot allocate new MTD device.\n");
    goto err_free_list;
    }
    new_mtd.mtdinfo = mtdinfo;
    priv = kzalloc_obj(slram_priv_t);
    if (!priv) {
    E("slram: Cannot allocate new MTD device.\n");
    goto err_free_mtdinfo;
    }
    mtdinfo.priv = priv;
    priv.start = memremap(start, length,
    MEMREMAP_WB | MEMREMAP_WT | MEMREMAP_WC);
    if (!priv.start) {
    E("slram: memremap failed\n");
    ret = -EIO;
    goto err_free_priv;
    }
    priv.end = priv.start + length;
    mtdinfo.name = name;
    mtdinfo.size = length;
    mtdinfo.flags = MTD_CAP_RAM;
    mtdinfo._erase = slram_erase;
    mtdinfo._point = slram_point;
    mtdinfo._unpoint = slram_unpoint;
    mtdinfo._read = slram_read;
    mtdinfo._write = slram_write;
    mtdinfo.owner = THIS_MODULE;
    mtdinfo.type = MTD_RAM;
    mtdinfo.erasesize = SLRAM_BLK_SZ;
    mtdinfo.writesize = 1;
    if (mtd_device_register(mtdinfo, core::ptr::null_mut(), 0)) {
    E("slram: Failed to register new device\n");
    ret = -EAGAIN;
    goto err_unmap;
    }
// curmtd = new_mtd;
    T("slram: Registered device %s from %luKiB to %luKiB\n", name,
    (start / 1024), ((start + length) / 1024));
    T("slram: Mapped from 0x%p to 0x%p\n", priv.start, priv.end);
    return 0;
    err_unmap:
    memunmap(priv.start);
    err_free_priv:
    kfree(priv);
    err_free_mtdinfo:
    kfree(mtdinfo);
    err_free_list:
    kfree(new_mtd);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn unregister_devices() {
    static void unregister_devices(void)
    {
    slram_mtd_list_t *nextitem;
    while (slram_mtdlist) {
    nextitem = slram_mtdlist.next;
    mtd_device_unregister(slram_mtdlist.mtdinfo);
    memunmap(((slram_priv_t *)slram_mtdlist.mtdinfo.priv).start);
    kfree(slram_mtdlist.mtdinfo.priv);
    kfree(slram_mtdlist.mtdinfo);
    kfree(slram_mtdlist);
    slram_mtdlist = nextitem;
    }
    }
#[no_mangle]
unsafe extern "C" fn handle_unit(value: c_ulong, unit: *mut c_char) -> c_ulong {
    static unsigned long handle_unit(unsigned long value, char *unit)
    {
    if ((*unit == 'M') || (*unit == 'm')) {
    return(value * 1024 * 1024);
    } else if ((*unit == 'K') || (*unit == 'k')) {
    return(value * 1024);
    }
    return(value);
    }
#[no_mangle]
unsafe extern "C" fn parse_cmdline(devname: *mut c_char, szstart: *mut c_char, szlength: *mut c_char) -> c_int {
    static int parse_cmdline(char *devname, char *szstart, char *szlength)
    {
    char *buffer;
    unsigned long devstart;
    unsigned long devlength;
    if ((!devname) || (!szstart) || (!szlength)) {
    unregister_devices();
    return(-EINVAL);
    }
    devstart = simple_strtoul(szstart, &buffer, 0);
    devstart = handle_unit(devstart, buffer);
    if (*(szlength) != '+') {
    devlength = simple_strtoul(szlength, &buffer, 0);
    devlength = handle_unit(devlength, buffer);
    if (devlength < devstart)
    goto err_out;
    devlength -= devstart;
    } else {
    devlength = simple_strtoul(szlength + 1, &buffer, 0);
    devlength = handle_unit(devlength, buffer);
    }
    T("slram: devname=%s, devstart=0x%lx, devlength=0x%lx\n",
    devname, devstart, devlength);
    if (devlength % SLRAM_BLK_SZ != 0)
    goto err_out;
    if ((devstart = register_device(devname, devstart, devlength))){
    unregister_devices();
    return((int)devstart);
    }
    return(0);
    err_out:
    E("slram: Illegal length parameter.\n");
    return(-EINVAL);
    }

#[no_mangle]
unsafe extern "C" fn mtd_slram_setup(str: *mut c_char) -> int __init {
    static int __init mtd_slram_setup(char *str)
    {
    map = str;
    return(1);
    }
    __setup("slram=", mtd_slram_setup);

#[no_mangle]
unsafe extern "C" fn init_slram() -> int __init {
    static int __init init_slram(void)
    {
    char *devname;

    char *devstart;
    char *devlength;
    if (!map) {
    E("slram: not enough parameters.\n");
    return(-EINVAL);
    }
    while (map) {
    devname = devstart = devlength = core::ptr::null_mut();
    if (!(devname = strsep(&map, ","))) {
    E("slram: No devicename specified.\n");
    break;
    }
    T("slram: devname = %s\n", devname);
    if ((!map) || (!(devstart = strsep(&map, ",")))) {
    E("slram: No devicestart specified.\n");
    break;
    }
    T("slram: devstart = %s\n", devstart);
    if ((!map) || (!(devlength = strsep(&map, ",")))) {
    E("slram: No devicelength / -end specified.\n");
    break;
    }
    T("slram: devlength = %s\n", devlength);
    if (parse_cmdline(devname, devstart, devlength) != 0) {
    return(-EINVAL);
    }
    }

    int count;
    int i;
    for (count = 0; count < SLRAM_MAX_DEVICES_PARAMS && map[count];
    count++) {
    }
    if ((count % 3 != 0) || (count == 0)) {
    E("slram: not enough parameters.\n");
    return(-EINVAL);
    }
    for (i = 0; i < (count / 3); i++) {
    devname = map[i * 3];
    if (parse_cmdline(devname, map[i * 3 + 1], map[i * 3 + 2])!=0) {
    return(-EINVAL);
    }
    }

    return(0);
    }
#[no_mangle]
unsafe extern "C" fn cleanup_slram() -> void __exit {
    static void __exit cleanup_slram(void)
    {
    unregister_devices();
    }
    module_init(init_slram);
    module_exit(cleanup_slram);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Jochen Schaeuble <psionic@psionic.de>");
    MODULE_DESCRIPTION("MTD driver for uncached system RAM");
