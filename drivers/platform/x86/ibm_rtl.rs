//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/ibm_rtl.c
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
// IBM Real-Time Linux driver
//
// Copyright (C) IBM Corporation, 2010
//
// Author: Keith Mannthey <kmannth@us.ibm.com>
// Vernon Mauery <vernux@us.ibm.com>
//

    static bool force;
    module_param(force, bool, 0);
    MODULE_PARM_DESC(force, "Force driver load, ignore DMI data");
    static bool debug;
    module_param(debug, bool, 0644);
    MODULE_PARM_DESC(debug, "Show debug output");
    MODULE_DESCRIPTION("IBM Premium Real Time Mode (PRTM) driver");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Keith Mannthey <kmmanth@us.ibm.com>");
    MODULE_AUTHOR("Vernon Mauery <vernux@us.ibm.com>");
pub const RTL_ADDR_TYPE_IO: c_int = 1;
pub const RTL_ADDR_TYPE_MMIO: c_int = 2;
pub const RTL_CMD_ENTER_PRTM: c_int = 1;
pub const RTL_CMD_EXIT_PRTM: c_int = 2;
// The RTL table as presented by the EBDA:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibm_rtl_table {
    pub /: *mut *mut char signature[5]; / signature should be "_RTL_",
    pub version: u8,
    pub rt_status: u8,
    pub command: u8,
    pub command_status: u8,
    pub cmd_address_type: u8,
    pub cmd_granularity: u8,
    pub cmd_offset: u8,
    pub reserve1: u16,
    pub /: *mut *mut u32 cmd_port_address; / platform dependent address,
    pub /: *mut *mut u32 cmd_port_value; / platform dependent value,
    pub __attribute__((packed)): },
// to locate "_RTL_" signature do a masked 5-byte integer compare
pub const RTL_SIGNATURE: c_uint = 0x0000005f4c54525fULL;
pub const RTL_MASK: c_uint = 0x000000ffffffffffULL;

    do {							\
    if (debug)					\
    pub \: pr_info(fmt, ##__VA_ARGS__);,
    } while (0)
    pub DEFINE_MUTEX(rtl_lock): static,
    pub rtl_table: *mut static struct ibm_rtl_table __iomem,
    pub ebda_map: *mut static void __iomem,
    pub rtl_cmd_addr: *mut static void __iomem,
    pub rtl_cmd_type: static u8,
    pub rtl_cmd_width: static u8,
    static void __iomem *rtl_port_map(phys_addr_t addr, unsigned long len)
    {
    if (rtl_cmd_type == RTL_ADDR_TYPE_MMIO)
    pub len): return ioremap(addr,,
    pub len): return ioport_map(addr,,
    }
#[no_mangle]
unsafe extern "C" fn rtl_port_unmap(addr: *mut void __iomem) {
    static void rtl_port_unmap(void __iomem *addr)
    {
    if (addr && rtl_cmd_type == RTL_ADDR_TYPE_MMIO)
    else
    }
#[no_mangle]
unsafe extern "C" fn ibm_rtl_write(value: u8) -> c_int {
    static int ibm_rtl_write(u8 value)
    {
    pub 0: int ret = 0, count =,
    pub cmd_port_val: u32,
    pub value): RTL_DEBUG("%s(%d)\n", __func__,,
    pub RTL_CMD_EXIT_PRTM: value = value == 1 ? RTL_CMD_ENTER_PRTM :,
    if (ioread8(&rtl_table.rt_status) != value) {
    pub &rtl_table->command): iowrite8(value,,
    switch (rtl_cmd_width) {
    case 8:
    pub ioread8(&rtl_table->cmd_port_value): cmd_port_val =,
    pub cmd_port_val): RTL_DEBUG("cmd_port_val = %u\n",,
    pub rtl_cmd_addr): iowrite8((u8)cmd_port_val,,
    case 16:
    pub ioread16(&rtl_table->cmd_port_value): cmd_port_val =,
    pub cmd_port_val): RTL_DEBUG("cmd_port_val = %u\n",,
    pub rtl_cmd_addr): iowrite16((u16)cmd_port_val,,
    case 32:
    pub ioread32(&rtl_table->cmd_port_value): cmd_port_val =,
    pub cmd_port_val): RTL_DEBUG("cmd_port_val = %u\n",,
    pub rtl_cmd_addr): iowrite32(cmd_port_val,,
    }
    while (ioread8(&rtl_table.command)) {
    if (count++ > 500) {
    pr_err("Hardware not responding to "
    pub request\n"): "mode switch,
    pub -EIO: ret =,
    }
    }
    if (ioread8(&rtl_table.command_status)) {
    pub command\n"): RTL_DEBUG("command_status reports failed,
    pub -EIO: ret =,
    }
    }
    pub ret: return,
    }
    static ssize_t rtl_show_version(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    pub (int)ioread8(&rtl_table->version)): return sprintf(buf, "%d\n",,
    }
    static ssize_t rtl_show_state(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    pub ioread8(&rtl_table->rt_status)): return sprintf(buf, "%d\n",,
    }
    static ssize_t rtl_set_state(struct device *dev,
    struct device_attribute *attr,
    const char *buf,
    size_t count)
    {
    pub ret: isize,
    if (count < 1 || count > 2)
    pub -EINVAL: return,
    switch (buf[0]) {
    case '0':
    pub ibm_rtl_write(0): ret =,
    case '1':
    pub ibm_rtl_write(1): ret =,
    default:
    pub -EINVAL: ret =,
    }
    if (ret >= 0)
    pub count: ret =,
    pub ret: return,
    }
    static const struct bus_type rtl_subsys = {
    .name = "ibm_rtl",
    .dev_name = "ibm_rtl",
}

    static DEVICE_ATTR(version, S_IRUGO, rtl_show_version, core::ptr::null_mut());
    static DEVICE_ATTR(state, 0600, rtl_show_state, rtl_set_state);
    static struct device_attribute *rtl_attributes[] = {
    &dev_attr_version,
    &dev_attr_state,
    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn rtl_setup_sysfs() -> c_int {
    int ret, i;
    ret = subsys_system_register(&rtl_subsys, core::ptr::null_mut());
    if (!ret) {
    struct device *dev_root = bus_get_dev_root(&rtl_subsys);
    if (dev_root) {
    for (i = 0; rtl_attributes[i]; i ++)
    device_create_file(dev_root, rtl_attributes[i]);
    put_device(dev_root);
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rtl_teardown_sysfs() {
    struct device *dev_root = bus_get_dev_root(&rtl_subsys);
    int i;
    if (dev_root) {
    for (i = 0; rtl_attributes[i]; i ++)
    device_remove_file(dev_root, rtl_attributes[i]);
    put_device(dev_root);
    }
    bus_unregister(&rtl_subsys);
    }
    static const struct dmi_system_id ibm_rtl_dmi_table[] __initconst = {
    {                                                  \
    .matches = {                               \
    DMI_MATCH(DMI_SYS_VENDOR, "IBM"),  \
    },                                         \
    },
    { }
    };
#[no_mangle]
unsafe extern "C" fn ibm_rtl_init() -> int __init {
    unsigned long ebda_addr, ebda_size;
    unsigned int ebda_kb;
    let mut ret: c_int = -ENODEV, i;
    if (force)
    pr_warn("module loaded by force\n");
// first ensure that we are running on IBM HW
#[no_mangle]
pub unsafe extern "C" fn if(!dmi_check_system(ibm_rtl_dmi_table): efi_enabled(EFI_BOOT) ||) -> else {
    else if (efi_enabled(EFI_BOOT) || !dmi_check_system(ibm_rtl_dmi_table))
    return -ENODEV;
// Get the address for the Extended BIOS Data Area
    ebda_addr = get_bios_ebda();
    if (!ebda_addr) {
    RTL_DEBUG("no BIOS EBDA found\n");
    return -ENODEV;
    }
    ebda_map = ioremap(ebda_addr, 4);
    if (!ebda_map)
    return -ENOMEM;
// First word in the EDBA is the Size in KB
    ebda_kb = ioread16(ebda_map);
    RTL_DEBUG("EBDA is %d kB\n", ebda_kb);
    if (ebda_kb == 0)
    goto out;
    iounmap(ebda_map);
    ebda_size = ebda_kb*1024;
// Remap the whole table
    ebda_map = ioremap(ebda_addr, ebda_size);
    if (!ebda_map)
    return -ENOMEM;
// search for the _RTL_ signature at the start of the table
    for (i = 0 ; i < ebda_size/sizeof(unsigned int); i++) {
    struct ibm_rtl_table __iomem * tmp;
    tmp = (struct ibm_rtl_table __iomem *) (ebda_map + i*sizeof(unsigned int));
    if ((readq(&tmp.signature) & RTL_MASK) == RTL_SIGNATURE) {
    phys_addr_t addr;
    unsigned int plen;
    RTL_DEBUG("found RTL_SIGNATURE at %p\n", tmp);
    rtl_table = tmp;
// The address, value, width and offset are platform
// dependent and found in the ibm_rtl_table
    rtl_cmd_width = ioread8(&rtl_table.cmd_granularity);
    rtl_cmd_type = ioread8(&rtl_table.cmd_address_type);
    RTL_DEBUG("rtl_cmd_width = %u, rtl_cmd_type = %u\n",
    rtl_cmd_width, rtl_cmd_type);
    addr = ioread32(&rtl_table.cmd_port_address);
    RTL_DEBUG("addr = %#llx\n", (unsigned long long)addr);
    plen = rtl_cmd_width/sizeof(char);
    rtl_cmd_addr = rtl_port_map(addr, plen);
    RTL_DEBUG("rtl_cmd_addr = %p\n", rtl_cmd_addr);
    if (!rtl_cmd_addr) {
    ret = -ENOMEM;
    break;
    }
    ret = rtl_setup_sysfs();
    break;
    }
    }
    out:
    if (ret) {
    iounmap(ebda_map);
    rtl_port_unmap(rtl_cmd_addr);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ibm_rtl_exit() -> void __exit {
    static void __exit ibm_rtl_exit(void)
    {
    if (rtl_table) {
    RTL_DEBUG("cleaning up");
// do not leave the machine in SMI-free mode
    ibm_rtl_write(0);
// unmap, unlink and remove all traces
    rtl_teardown_sysfs();
    iounmap(ebda_map);
    rtl_port_unmap(rtl_cmd_addr);
    }
    }
    module_init(ibm_rtl_init);
    module_exit(ibm_rtl_exit);
