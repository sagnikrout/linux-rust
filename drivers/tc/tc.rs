//! Automatically rewritten from C to Rust
//! Source: drivers/tc/tc.c
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


//
// TURBOchannel bus services.
//
// Copyright (c) Harald Koerfgen, 1998
// Copyright (c) 2001, 2003, 2005, 2006, 2018  Maciej W. Rozycki
// Copyright (c) 2005  James Simmons
//
// This file is subject to the terms and conditions of the GNU
// General Public License.  See the file "COPYING" in the main
// directory of this archive for more details.
//

    static struct tc_bus tc_bus = {
    .name = "TURBOchannel",
    };
//
// Probing for TURBOchannel modules.
//
#[no_mangle]
unsafe extern "C" fn tc_bus_add_devices(tbus: *mut tc_bus) -> void __init {
    static void __init tc_bus_add_devices(struct tc_bus *tbus)
    {
    let mut slotsize: resource_size_t = tbus.info.slot_size << 20;
    let mut extslotsize: resource_size_t = tbus.ext_slot_size;
    resource_size_t slotaddr;
    resource_size_t extslotaddr;
    resource_size_t devsize;
    void __iomem *module;
    struct tc_dev *tdev;
    int i, slot, err;
    u8 pattern[4];
    long offset;
    for (slot = 0; slot < tbus.num_tcslots; slot++) {
    slotaddr = tbus.slot_base + slot * slotsize;
    extslotaddr = tbus.ext_slot_base + slot * extslotsize;
    module = ioremap(slotaddr, slotsize);
    BUG_ON(!module);
    offset = TC_OLDCARD;
    err = 0;
    err |= tc_preadb(pattern + 0, module + offset + TC_PATTERN0);
    err |= tc_preadb(pattern + 1, module + offset + TC_PATTERN1);
    err |= tc_preadb(pattern + 2, module + offset + TC_PATTERN2);
    err |= tc_preadb(pattern + 3, module + offset + TC_PATTERN3);
    if (err)
    goto out_err;
    if (pattern[0] != 0x55 || pattern[1] != 0x00 ||
    pattern[2] != 0xaa || pattern[3] != 0xff) {
    offset = TC_NEWCARD;
    err = 0;
    err |= tc_preadb(pattern + 0,
    module + offset + TC_PATTERN0);
    err |= tc_preadb(pattern + 1,
    module + offset + TC_PATTERN1);
    err |= tc_preadb(pattern + 2,
    module + offset + TC_PATTERN2);
    err |= tc_preadb(pattern + 3,
    module + offset + TC_PATTERN3);
    if (err)
    goto out_err;
    }
    if (pattern[0] != 0x55 || pattern[1] != 0x00 ||
    pattern[2] != 0xaa || pattern[3] != 0xff)
    goto out_err;
// Found a board, allocate it an entry in the list
    tdev = kzalloc_obj(*tdev);
    if (!tdev) {
    pr_err("tc%x: unable to allocate tc_dev\n", slot);
    goto out_err;
    }
    dev_set_name(&tdev.dev, "tc%x", slot);
    tdev.bus = tbus;
    tdev.dev.parent = &tbus.dev;
    tdev.dev.bus = &tc_bus_type;
    tdev.slot = slot;
// TURBOchannel has 34-bit DMA addressing (16GiB space).
    tdev.dma_mask = DMA_BIT_MASK(34);
    tdev.dev.dma_mask = &tdev.dma_mask;
    tdev.dev.coherent_dma_mask = DMA_BIT_MASK(34);
    for (i = 0; i < 8; i++) {
    tdev.firmware[i] =
    readb(module + offset + TC_FIRM_VER + 4 * i);
    tdev.vendor[i] =
    readb(module + offset + TC_VENDOR + 4 * i);
    tdev.name[i] =
    readb(module + offset + TC_MODULE + 4 * i);
    }
    tdev.firmware[8] = 0;
    tdev.vendor[8] = 0;
    tdev.name[8] = 0;
    pr_info("%s: %s %s %s\n", dev_name(&tdev.dev), tdev.vendor,
    tdev.name, tdev.firmware);
    devsize = readb(module + offset + TC_SLOT_SIZE);
    devsize <<= 22;
    if (devsize <= slotsize) {
    tdev.resource.start = slotaddr;
    tdev.resource.end = slotaddr + devsize - 1;
    } else if (devsize <= extslotsize) {
    tdev.resource.start = extslotaddr;
    tdev.resource.end = extslotaddr + devsize - 1;
    } else {
    pr_err("%s: Cannot provide slot space "
    "(%ldMiB required, up to %ldMiB supported)\n",
    dev_name(&tdev.dev), (long)(devsize >> 20),
    (long)(max(slotsize, extslotsize) >> 20));
    kfree(tdev);
    goto out_err;
    }
    tdev.resource.name = tdev.name;
    tdev.resource.flags = IORESOURCE_MEM;
    tc_device_get_irq(tdev);
    if (device_register(&tdev.dev)) {
    put_device(&tdev.dev);
    goto out_err;
    }
    list_add_tail(&tdev.node, &tbus.devices);
    out_err:
    iounmap(module);
    }
    }
//
// The main entry.
//
#[no_mangle]
unsafe extern "C" fn tc_init() -> int __init {
    static int __init tc_init(void)
    {
// Initialize the TURBOchannel bus
    if (tc_bus_get_info(&tc_bus))
    goto out_err;
    INIT_LIST_HEAD(&tc_bus.devices);
    dev_set_name(&tc_bus.dev, "tc");
    if (device_register(&tc_bus.dev))
    goto out_err_device;
    if (tc_bus.info.slot_size) {
    let mut tc_clock: c_uint = tc_get_speed(&tc_bus) / 100000;
    pr_info("tc: TURBOchannel rev. %d at %u.%u MHz "
    "(with%s parity)\n", tc_bus.info.revision,
    tc_clock / 10, tc_clock % 10,
    tc_bus.info.parity ? "" : "out");
    tc_bus.resource[0].start = tc_bus.slot_base;
    tc_bus.resource[0].end = tc_bus.slot_base +
    (tc_bus.info.slot_size << 20) *
    tc_bus.num_tcslots - 1;
    tc_bus.resource[0].name = tc_bus.name;
    tc_bus.resource[0].flags = IORESOURCE_MEM;
    if (request_resource(&iomem_resource,
    &tc_bus.resource[0]) < 0) {
    pr_err("tc: Cannot reserve resource\n");
    goto out_err_device;
    }
    if (tc_bus.ext_slot_size) {
    tc_bus.resource[1].start = tc_bus.ext_slot_base;
    tc_bus.resource[1].end = tc_bus.ext_slot_base +
    tc_bus.ext_slot_size *
    tc_bus.num_tcslots - 1;
    tc_bus.resource[1].name = tc_bus.name;
    tc_bus.resource[1].flags = IORESOURCE_MEM;
    if (request_resource(&iomem_resource,
    &tc_bus.resource[1]) < 0) {
    pr_err("tc: Cannot reserve resource\n");
    goto out_err_resource;
    }
    }
    tc_bus_add_devices(&tc_bus);
    }
    return 0;
    out_err_resource:
    release_resource(&tc_bus.resource[0]);
    out_err_device:
    put_device(&tc_bus.dev);
    out_err:
    return 0;
    }
    subsys_initcall(tc_init);
