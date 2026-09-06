//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/maps/ichxrom.c
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
// ichxrom.c
//
// Normal mappings of chips in physical memory
//

pub const ADDRESS_NAME_LEN: c_int = 18;

pub const BIOS_CNTL: c_uint = 0x4e;
pub const FWH_DEC_EN1: c_uint = 0xE3;
pub const FWH_DEC_EN2: c_uint = 0xF0;
pub const FWH_SEL1: c_uint = 0xE8;
pub const FWH_SEL2: c_uint = 0xEE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ichxrom_window {
    pub virt: *mut *mut void __iomem,
    pub phys: c_ulong,
    pub size: c_ulong,
    pub maps: list_head,
    pub rsrc: resource,
    pub pdev: *mut pci_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ichxrom_map_info {
    pub list: list_head,
    pub map: map_info,
    pub mtd: *mut mtd_info,
    pub rsrc: resource,
    pub ADDRESS_NAME_LEN]: char map_name[sizeof(MOD_NAME) + 2 +,
}

    static struct ichxrom_window ichxrom_window = {
    .maps = LIST_HEAD_INIT(ichxrom_window.maps),
    };
#[no_mangle]
unsafe extern "C" fn ichxrom_cleanup(window: *mut ichxrom_window) {
    static void ichxrom_cleanup(struct ichxrom_window *window)
    {
    struct ichxrom_map_info *map, *scratch;
    u16 word;
    int ret;
// Disable writes through the rom window
    ret = pci_read_config_word(window.pdev, BIOS_CNTL, &word);
    if (!ret)
    pci_write_config_word(window.pdev, BIOS_CNTL, word & ~1);
    pci_dev_put(window.pdev);
// Free all of the mtd devices
    list_for_each_entry_safe(map, scratch, &window.maps, list) {
    if (map.rsrc.parent)
    release_resource(&map.rsrc);
    mtd_device_unregister(map.mtd);
    map_destroy(map.mtd);
    list_del(&map.list);
    kfree(map);
    }
    if (window.rsrc.parent)
    release_resource(&window.rsrc);
    if (window.virt) {
    iounmap(window.virt);
    window.virt = core::ptr::null_mut();
    window.phys = 0;
    window.size = 0;
    window.pdev = core::ptr::null_mut();
    }
    }
    static int __init ichxrom_init_one(struct pci_dev *pdev,
    const struct pci_device_id *ent)
    {
    static char *rom_probe_types[] = { "cfi_probe", "jedec_probe", core::ptr::null_mut() };
    struct ichxrom_window *window = &ichxrom_window;
    struct ichxrom_map_info *map = core::ptr::null_mut();
    unsigned long map_top;
    u8 byte;
    u16 word;
// For now I just handle the ichx and I assume there
// are not a lot of resources up at the top of the address
// space.  It is possible to handle other devices in the
// top 16MB but it is very painful.  Also since
// you can only really attach a FWH to an ICHX there
// a number of simplifications you can make.
//
// Also you can page firmware hubs if an 8MB window isn't enough
// but don't currently handle that case either.
//
    window.pdev = pdev;
// Find a region continuous to the end of the ROM window
    window.phys = 0;
    pci_read_config_byte(pdev, FWH_DEC_EN1, &byte);
    if (byte == 0xff) {
    window.phys = 0xffc00000;
    pci_read_config_byte(pdev, FWH_DEC_EN2, &byte);
    if ((byte & 0x0f) == 0x0f) {
    window.phys = 0xff400000;
    }
#[no_mangle]
pub unsafe extern "C" fn if(0x0e: (byte & 0x0e) ==) -> else {
    window.phys = 0xff500000;
    }
#[no_mangle]
pub unsafe extern "C" fn if(0x0c: (byte & 0x0c) ==) -> else {
    window.phys = 0xff600000;
    }
#[no_mangle]
pub unsafe extern "C" fn if(0x08: (byte & 0x08) ==) -> else {
    window.phys = 0xff700000;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn if(0xfe: (byte & 0xfe) ==) -> else {
    window.phys = 0xffc80000;
    }
#[no_mangle]
pub unsafe extern "C" fn if(0xfc: (byte & 0xfc) ==) -> else {
    window.phys = 0xffd00000;
    }
#[no_mangle]
pub unsafe extern "C" fn if(0xf8: (byte & 0xf8) ==) -> else {
    window.phys = 0xffd80000;
    }
#[no_mangle]
pub unsafe extern "C" fn if(0xf0: (byte & 0xf0) ==) -> else {
    window.phys = 0xffe00000;
    }
#[no_mangle]
pub unsafe extern "C" fn if(0xe0: (byte & 0xe0) ==) -> else {
    window.phys = 0xffe80000;
    }
#[no_mangle]
pub unsafe extern "C" fn if(0xc0: (byte & 0xc0) ==) -> else {
    window.phys = 0xfff00000;
    }
#[no_mangle]
pub unsafe extern "C" fn if(0x80: (byte & 0x80) ==) -> else {
    window.phys = 0xfff80000;
    }
    if (window.phys == 0) {
    printk(KERN_ERR MOD_NAME ": Rom window is closed\n");
    goto out;
    }
    window.phys -= 0x400000UL;
    window.size = (0xffffffffUL - window.phys) + 1UL;
// Enable writes through the rom window
    pci_read_config_word(pdev, BIOS_CNTL, &word);
    if (!(word & 1)  && (word & (1<<1))) {
// The BIOS will generate an error if I enable
// this device, so don't even try.
//
    printk(KERN_ERR MOD_NAME ": firmware access control, I can't enable writes\n");
    goto out;
    }
    pci_write_config_word(pdev, BIOS_CNTL, word | 1);
//
// Try to reserve the window mem region.  If this fails then
// it is likely due to the window being "reserved" by the BIOS.
//
    window.rsrc.name = MOD_NAME;
    window.rsrc.start = window.phys;
    window.rsrc.end   = window.phys + window.size - 1;
    window.rsrc.flags = IORESOURCE_MEM | IORESOURCE_BUSY;
    if (request_resource(&iomem_resource, &window.rsrc)) {
    window.rsrc.parent = core::ptr::null_mut();
    printk(KERN_DEBUG MOD_NAME ": "
    "%s(): Unable to register resource %pR - kernel bug?\n",
    __func__, &window.rsrc);
    }
// Map the firmware hub into my address space.
    window.virt = ioremap(window.phys, window.size);
    if (!window.virt) {
    printk(KERN_ERR MOD_NAME ": ioremap(%08lx, %08lx) failed\n",
    window.phys, window.size);
    goto out;
    }
// Get the first address to look for an rom chip at
    map_top = window.phys;
    if ((window.phys & 0x3fffff) != 0) {
    map_top = window.phys + 0x400000;
    }

// The probe sequence run over the firmware hub lock
// registers sets them to 0x7 (no access).
// Probe at most the last 4M of the address space.
//
    if (map_top < 0xffc00000) {
    map_top = 0xffc00000;
    }

// Loop through and look for rom chips
    while((map_top - 1) < 0xffffffffUL) {
    struct cfi_private *cfi;
    unsigned long offset;
    int i;
    if (!map) {
    map = kmalloc_obj(*map);
    if (!map)
    goto out;
    }
    memset(map, 0, sizeof(*map));
    INIT_LIST_HEAD(&map.list);
    map.map.name = map.map_name;
    map.map.phys = map_top;
    offset = map_top - window.phys;
    map.map.virt = (void __iomem *)
    (((unsigned long)(window.virt)) + offset);
    map.map.size = 0xffffffffUL - map_top + 1UL;
// Set the name of the map to the address I am trying
    sprintf(map.map_name, "%s @%08Lx",
    MOD_NAME, (unsigned long long)map.map.phys);
// Firmware hubs only use vpp when being programmed
// in a factory setting.  So in-place programming
// needs to use a different method.
//
    for(map.map.bankwidth = 32; map.map.bankwidth;
    map.map.bankwidth >>= 1)
    {
    char **probe_type;
// Skip bankwidths that are not supported
    if (!map_bankwidth_supported(map.map.bankwidth))
    continue;
// Setup the map methods
    simple_map_init(&map.map);
// Try all of the probe methods
    probe_type = rom_probe_types;
    for(; *probe_type; probe_type++) {
    map.mtd = do_map_probe(*probe_type, &map.map);
    if (map.mtd)
    goto found;
    }
    }
    map_top += ROM_PROBE_STEP_SIZE;
    continue;
    found:
// Trim the size if we are larger than the map
    if (map.mtd.size > map.map.size) {
    printk(KERN_WARNING MOD_NAME
    " rom(%llu) larger than window(%lu). fixing...\n",
    (unsigned long long)map.mtd.size, map.map.size);
    map.mtd.size = map.map.size;
    }
    if (window.rsrc.parent) {
//
// Registering the MTD device in iomem may not be possible
// if there is a BIOS "reserved" and BUSY range.  If this
// fails then continue anyway.
//
    map.rsrc.name  = map.map_name;
    map.rsrc.start = map.map.phys;
    map.rsrc.end   = map.map.phys + map.mtd.size - 1;
    map.rsrc.flags = IORESOURCE_MEM | IORESOURCE_BUSY;
    if (request_resource(&window.rsrc, &map.rsrc)) {
    printk(KERN_ERR MOD_NAME
    ": cannot reserve MTD resource\n");
    map.rsrc.parent = core::ptr::null_mut();
    }
    }
// Make the whole region visible in the map
    map.map.virt = window.virt;
    map.map.phys = window.phys;
    cfi = map.map.fldrv_priv;
    for(i = 0; i < cfi.numchips; i++) {
    cfi.chips[i].start += offset;
    }
// Now that the mtd devices is complete claim and export it
    map.mtd.owner = THIS_MODULE;
    if (mtd_device_register(map.mtd, core::ptr::null_mut(), 0)) {
    map_destroy(map.mtd);
    map.mtd = core::ptr::null_mut();
    goto out;
    }
// Calculate the new value of map_top
    map_top += map.mtd.size;
// File away the map structure
    list_add(&map.list, &window.maps);
    map = core::ptr::null_mut();
    }
    out:
// Free any left over map structures
    kfree(map);
// See if I have any map structures
    if (list_empty(&window.maps)) {
    ichxrom_cleanup(window);
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ichxrom_remove_one(pdev: *mut pci_dev) {
    static void ichxrom_remove_one(struct pci_dev *pdev)
    {
    struct ichxrom_window *window = &ichxrom_window;
    ichxrom_cleanup(window);
    }
    static const struct pci_device_id ichxrom_pci_tbl[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_82801BA_0), },
    { PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_82801CA_0), },
    { PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_82801DB_0), },
    { PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_82801EB_0), },
    { PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_ESB_1), },
    { },
    };

    MODULE_DEVICE_TABLE(pci, ichxrom_pci_tbl);
    static struct pci_driver ichxrom_driver = {
    .name =		MOD_NAME,
    .id_table =	ichxrom_pci_tbl,
    .probe =	ichxrom_init_one,
    .remove =	ichxrom_remove_one,
    };

#[no_mangle]
unsafe extern "C" fn init_ichxrom() -> int __init {
    static int __init init_ichxrom(void)
    {
    struct pci_dev *pdev;
    const struct pci_device_id *id;
    pdev = core::ptr::null_mut();
    for (id = ichxrom_pci_tbl; id.vendor; id++) {
    pdev = pci_get_device(id.vendor, id.device, core::ptr::null_mut());
    if (pdev) {
    break;
    }
    }
    if (pdev) {
    return ichxrom_init_one(pdev, &ichxrom_pci_tbl[0]);
    }
    return -ENXIO;

    return pci_register_driver(&ichxrom_driver);

    }
#[no_mangle]
unsafe extern "C" fn cleanup_ichxrom() -> void __exit {
    static void __exit cleanup_ichxrom(void)
    {
    ichxrom_remove_one(ichxrom_window.pdev);
    }
    module_init(init_ichxrom);
    module_exit(cleanup_ichxrom);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Eric Biederman <ebiederman@lnxi.com>");
    MODULE_DESCRIPTION("MTD map driver for BIOS chips on the ICHX southbridge");
