//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/ps3/device-init.c
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
// PS3 device registration routines.
//
// Copyright (C) 2007 Sony Computer Entertainment Inc.
// Copyright 2007 Sony Corp.
//

#[no_mangle]
unsafe extern "C" fn ps3_register_lpm_devices() -> int __init {
    static int __init ps3_register_lpm_devices(void)
    {
    int result;
    u64 tmp1;
    u64 tmp2;
    struct ps3_system_bus_device *dev;
    pr_debug(" . %s:%d\n", __func__, __LINE__);
    dev = kzalloc_obj(*dev);
    if (!dev)
    return -ENOMEM;
    dev.match_id = PS3_MATCH_ID_LPM;
    dev.dev_type = PS3_DEVICE_TYPE_LPM;
// The current lpm driver only supports a single BE processor.
    result = ps3_repository_read_be_node_id(0, &dev.lpm.node_id);
    if (result) {
    pr_debug("%s:%d: ps3_repository_read_be_node_id failed \n",
    __func__, __LINE__);
    goto fail_read_repo;
    }
    result = ps3_repository_read_lpm_privileges(dev.lpm.node_id, &tmp1,
    &dev.lpm.rights);
    if (result) {
    pr_debug("%s:%d: ps3_repository_read_lpm_privileges failed\n",
    __func__, __LINE__);
    goto fail_read_repo;
    }
    lv1_get_logical_partition_id(&tmp2);
    if (tmp1 != tmp2) {
    pr_debug("%s:%d: wrong lpar\n",
    __func__, __LINE__);
    result = -ENODEV;
    goto fail_rights;
    }
    if (!(dev.lpm.rights & PS3_LPM_RIGHTS_USE_LPM)) {
    pr_debug("%s:%d: don't have rights to use lpm\n",
    __func__, __LINE__);
    result = -EPERM;
    goto fail_rights;
    }
    pr_debug("%s:%d: pu_id %llu, rights %llu(%llxh)\n",
    __func__, __LINE__, dev.lpm.pu_id, dev.lpm.rights,
    dev.lpm.rights);
    result = ps3_repository_read_pu_id(0, &dev.lpm.pu_id);
    if (result) {
    pr_debug("%s:%d: ps3_repository_read_pu_id failed \n",
    __func__, __LINE__);
    goto fail_read_repo;
    }
    result = ps3_system_bus_device_register(dev);
    if (result) {
    pr_debug("%s:%d ps3_system_bus_device_register failed\n",
    __func__, __LINE__);
    goto fail_register;
    }
    pr_debug(" <- %s:%d\n", __func__, __LINE__);
    return 0;
    fail_register:
    fail_rights:
    fail_read_repo:
    kfree(dev);
    pr_debug(" <- %s:%d: failed\n", __func__, __LINE__);
    return result;
    }
//
// ps3_setup_gelic_device - Setup and register a gelic device instance.
//
// Allocates memory for a struct ps3_system_bus_device instance, initialises the
// structure members, and registers the device instance with the system bus.
//
    static int __init ps3_setup_gelic_device(
    const struct ps3_repository_device *repo)
    {
    int result;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct layout {
    pub dev: ps3_system_bus_device,
    pub d_region: ps3_dma_region,
    pub p: *mut },
    pub __LINE__): pr_debug(" -> %s:%d\n", __func__,,
    pub PS3_BUS_TYPE_SB): BUG_ON(repo->bus_type !=,
    pub PS3_DEV_TYPE_SB_GELIC): BUG_ON(repo->dev_type !=,
    pub layout): p = kzalloc_obj(struct,
    if (!p) {
    pub -ENOMEM: result =,
    pub fail_malloc: goto,
    }
    pub PS3_MATCH_ID_GELIC: p->dev.match_id =,
    pub PS3_DEVICE_TYPE_SB: p->dev.dev_type =,
    pub repo->bus_id: p->dev.bus_id =,
    pub repo->dev_id: p->dev.dev_id =,
    pub &p->d_region: p->dev.d_region =,
    result = ps3_repository_find_interrupt(repo,
    pub &p->dev.interrupt_id): PS3_INTERRUPT_TYPE_EVENT_PORT,,
    if (result) {
    pr_debug("%s:%d ps3_repository_find_interrupt failed\n",
    pub __LINE__): __func__,,
    pub fail_find_interrupt: goto,
    }
    pub 0): BUG_ON(p->dev.interrupt_id !=,
    result = ps3_dma_region_init(&p.dev, p.dev.d_region, PS3_DMA_64K,
    pub 0): PS3_DMA_OTHER, NULL,,
    if (result) {
    pr_debug("%s:%d ps3_dma_region_init failed\n",
    pub __LINE__): __func__,,
    pub fail_dma_init: goto,
    }
    pub ps3_system_bus_device_register(&p->dev): result =,
    if (result) {
    pr_debug("%s:%d ps3_system_bus_device_register failed\n",
    pub __LINE__): __func__,,
    pub fail_device_register: goto,
    }
    pub __LINE__): pr_debug(" <- %s:%d\n", __func__,,
    pub result: return,
    fail_device_register:
    fail_dma_init:
    fail_find_interrupt:
    fail_malloc:
    pub __LINE__): pr_debug(" <- %s:%d: fail.\n", __func__,,
    pub result: return,
    }
    static int __init ps3_setup_uhc_device(
    const struct ps3_repository_device *repo, enum ps3_match_id match_id,
    enum ps3_interrupt_type interrupt_type, enum ps3_reg_type reg_type)
    {
    pub result: c_int,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct layout {
    pub dev: ps3_system_bus_device,
    pub d_region: ps3_dma_region,
    pub m_region: ps3_mmio_region,
    pub p: *mut },
    pub bus_addr: u64,
    pub len: u64,
    pub __LINE__): pr_debug(" -> %s:%d\n", __func__,,
    pub PS3_BUS_TYPE_SB): BUG_ON(repo->bus_type !=,
    pub PS3_DEV_TYPE_SB_USB): BUG_ON(repo->dev_type !=,
    pub layout): p = kzalloc_obj(struct,
    if (!p) {
    pub -ENOMEM: result =,
    pub fail_malloc: goto,
    }
    pub match_id: p->dev.match_id =,
    pub PS3_DEVICE_TYPE_SB: p->dev.dev_type =,
    pub repo->bus_id: p->dev.bus_id =,
    pub repo->dev_id: p->dev.dev_id =,
    pub &p->d_region: p->dev.d_region =,
    pub &p->m_region: p->dev.m_region =,
    result = ps3_repository_find_interrupt(repo,
    pub &p->dev.interrupt_id): interrupt_type,,
    if (result) {
    pr_debug("%s:%d ps3_repository_find_interrupt failed\n",
    pub __LINE__): __func__,,
    pub fail_find_interrupt: goto,
    }
    result = ps3_repository_find_reg(repo, reg_type,
    pub &len): &bus_addr,,
    if (result) {
    pr_debug("%s:%d ps3_repository_find_reg failed\n",
    pub __LINE__): __func__,,
    pub fail_find_reg: goto,
    }
    result = ps3_dma_region_init(&p.dev, p.dev.d_region, PS3_DMA_64K,
    pub 0): PS3_DMA_INTERNAL, NULL,,
    if (result) {
    pr_debug("%s:%d ps3_dma_region_init failed\n",
    pub __LINE__): __func__,,
    pub fail_dma_init: goto,
    }
    result = ps3_mmio_region_init(&p.dev, p.dev.m_region, bus_addr, len,
    if (result) {
    pr_debug("%s:%d ps3_mmio_region_init failed\n",
    pub __LINE__): __func__,,
    pub fail_mmio_init: goto,
    }
    pub ps3_system_bus_device_register(&p->dev): result =,
    if (result) {
    pr_debug("%s:%d ps3_system_bus_device_register failed\n",
    pub __LINE__): __func__,,
    pub fail_device_register: goto,
    }
    pub __LINE__): pr_debug(" <- %s:%d\n", __func__,,
    pub result: return,
    fail_device_register:
    fail_mmio_init:
    fail_dma_init:
    fail_find_reg:
    fail_find_interrupt:
    fail_malloc:
    pub __LINE__): pr_debug(" <- %s:%d: fail.\n", __func__,,
    pub result: return,
    }
    static int __init ps3_setup_ehci_device(
    const struct ps3_repository_device *repo)
    {
    return ps3_setup_uhc_device(repo, PS3_MATCH_ID_EHCI,
    pub PS3_REG_TYPE_SB_EHCI): PS3_INTERRUPT_TYPE_SB_EHCI,,
    }
    static int __init ps3_setup_ohci_device(
    const struct ps3_repository_device *repo)
    {
    return ps3_setup_uhc_device(repo, PS3_MATCH_ID_OHCI,
    pub PS3_REG_TYPE_SB_OHCI): PS3_INTERRUPT_TYPE_SB_OHCI,,
    }
    static int __init ps3_setup_vuart_device(enum ps3_match_id match_id,
    unsigned int port_number)
    {
    pub result: c_int,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct layout {
    pub dev: ps3_system_bus_device,
    pub p: *mut },
    pr_debug(" . %s:%d: match_id %u, port %u\n", __func__, __LINE__,
    pub port_number): match_id,,
    pub layout): p = kzalloc_obj(struct,
    if (!p)
    pub -ENOMEM: return,
    pub match_id: p->dev.match_id =,
    pub PS3_DEVICE_TYPE_VUART: p->dev.dev_type =,
    pub port_number: p->dev.port_number =,
    pub ps3_system_bus_device_register(&p->dev): result =,
    if (result) {
    pr_debug("%s:%d ps3_system_bus_device_register failed\n",
    pub __LINE__): __func__,,
    pub fail_device_register: goto,
    }
    pub __LINE__): pr_debug(" <- %s:%d\n", __func__,,
    pub 0: return,
    fail_device_register:
    pub __LINE__): pr_debug(" <- %s:%d fail\n", __func__,,
    pub result: return,
    }
    static int ps3_setup_storage_dev(const struct ps3_repository_device *repo,
    enum ps3_match_id match_id)
    {
    pub result: c_int,
    pub p: *mut ps3_storage_device,
    pub num_blocks: u64 port, blk_size,,
    pub i: unsigned int num_regions,,
    pub match_id): pr_debug(" -> %s:%u: match_id %u\n", __func__, __LINE__,,
    result = ps3_repository_read_stor_dev_info(repo.bus_index,
    repo.dev_index, &port,
    &blk_size, &num_blocks,
    if (result) {
    printk(KERN_ERR "%s:%u: _read_stor_dev_info failed %d\n",
    pub result): __func__, __LINE__,,
    pub -ENODEV: return,
    }
    pr_debug("%s:%u: (%u:%u:%u): port %llu blk_size %llu num_blocks %llu "
    "num_regions %u\n", __func__, __LINE__, repo.bus_index,
    repo.dev_index, repo.dev_type, port, blk_size, num_blocks,
    pub num_regions): *mut *mut p = kzalloc_flex(p, regions,,
    if (!p) {
    pub -ENOMEM: result =,
    pub fail_malloc: goto,
    }
    pub match_id: p->sbd.match_id =,
    pub PS3_DEVICE_TYPE_SB: p->sbd.dev_type =,
    pub repo->bus_id: p->sbd.bus_id =,
    pub repo->dev_id: p->sbd.dev_id =,
    pub &p->dma_region: p->sbd.d_region =,
    pub blk_size: p->blk_size =,
    pub num_regions: p->num_regions =,
    result = ps3_repository_find_interrupt(repo,
    PS3_INTERRUPT_TYPE_EVENT_PORT,
    if (result) {
    printk(KERN_ERR "%s:%u: find_interrupt failed %d\n", __func__,
    pub result): __LINE__,,
    pub -ENODEV: result =,
    pub fail_find_interrupt: goto,
    }
    pub {: for (i = 0; i < num_regions; i++),
    pub id: c_uint,
    pub size: u64 start,,
    result = ps3_repository_read_stor_dev_region(repo.bus_index,
    repo.dev_index,
    i, &id, &start,
    if (result) {
    printk(KERN_ERR
    "%s:%u: read_stor_dev_region failed %d\n",
    pub result): __func__, __LINE__,,
    pub -ENODEV: result =,
    pub fail_read_region: goto,
    }
    pr_debug("%s:%u: region %u: id %u start %llu size %llu\n",
    pub size): __func__, __LINE__, i, id, start,,
    pub id: p->regions[i].id =,
    pub start: p->regions[i].start =,
    pub size: p->regions[i].size =,
    }
    pub ps3_system_bus_device_register(&p->sbd): result =,
    if (result) {
    pr_debug("%s:%u ps3_system_bus_device_register failed\n",
    pub __LINE__): __func__,,
    pub fail_device_register: goto,
    }
    pub __LINE__): pr_debug(" <- %s:%u\n", __func__,,
    pub 0: return,
    fail_device_register:
    fail_read_region:
    fail_find_interrupt:
    fail_malloc:
    pub __LINE__): pr_debug(" <- %s:%u: fail.\n", __func__,,
    pub result: return,
    }
#[no_mangle]
unsafe extern "C" fn ps3_register_vuart_devices() -> int __init {
    static int __init ps3_register_vuart_devices(void)
    {
    pub result: c_int,
    pub port_number: c_uint,
    pub __LINE__): pr_debug(" -> %s:%d\n", __func__,,
    pub ps3_repository_read_vuart_av_port(&port_number): result =,
    if (result)
    pub /: *mut *mut port_number = 0; / av default,
    pub port_number): result = ps3_setup_vuart_device(PS3_MATCH_ID_AV_SETTINGS,,
    pub ps3_repository_read_vuart_sysmgr_port(&port_number): result =,
    if (result)
    pub /: *mut *mut port_number = 2; / sysmgr default,
    result = ps3_setup_vuart_device(PS3_MATCH_ID_SYSTEM_MANAGER,
    pub __LINE__): pr_debug(" <- %s:%d\n", __func__,,
    pub result: return,
    }
#[no_mangle]
unsafe extern "C" fn ps3_register_sound_devices() -> int __init {
    static int __init ps3_register_sound_devices(void)
    {
    pub result: c_int,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct layout {
    pub dev: ps3_system_bus_device,
    pub d_region: ps3_dma_region,
    pub m_region: ps3_mmio_region,
    pub p: *mut },
    pub __LINE__): pr_debug(" -> %s:%d\n", __func__,,
    pub kzalloc_obj(*p): *mut p =,
    if (!p)
    pub -ENOMEM: return,
    pub PS3_MATCH_ID_SOUND: p->dev.match_id =,
    pub PS3_DEVICE_TYPE_IOC0: p->dev.dev_type =,
    pub &p->d_region: p->dev.d_region =,
    pub &p->m_region: p->dev.m_region =,
    pub ps3_system_bus_device_register(&p->dev): result =,
    if (result) {
    pr_debug("%s:%d ps3_system_bus_device_register failed\n",
    pub __LINE__): __func__,,
    pub fail_device_register: goto,
    }
    pub __LINE__): pr_debug(" <- %s:%d\n", __func__,,
    pub 0: return,
    fail_device_register:
    pub __LINE__): pr_debug(" <- %s:%d failed\n", __func__,,
    pub result: return,
    }
#[no_mangle]
unsafe extern "C" fn ps3_register_graphics_devices() -> int __init {
    static int __init ps3_register_graphics_devices(void)
    {
    pub result: c_int,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct layout {
    pub dev: ps3_system_bus_device,
    pub p: *mut },
    pub __LINE__): pr_debug(" -> %s:%d\n", __func__,,
    pub layout): p = kzalloc_obj(struct,
    if (!p)
    pub -ENOMEM: return,
    pub PS3_MATCH_ID_GPU: p->dev.match_id =,
    pub PS3_MATCH_SUB_ID_GPU_FB: p->dev.match_sub_id =,
    pub PS3_DEVICE_TYPE_IOC0: p->dev.dev_type =,
    pub ps3_system_bus_device_register(&p->dev): result =,
    if (result) {
    pr_debug("%s:%d ps3_system_bus_device_register failed\n",
    pub __LINE__): __func__,,
    pub fail_device_register: goto,
    }
    pub __LINE__): pr_debug(" <- %s:%d\n", __func__,,
    pub 0: return,
    fail_device_register:
    pub __LINE__): pr_debug(" <- %s:%d failed\n", __func__,,
    pub result: return,
    }
#[no_mangle]
unsafe extern "C" fn ps3_register_ramdisk_device() -> int __init {
    static int __init ps3_register_ramdisk_device(void)
    {
    pub result: c_int,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct layout {
    pub dev: ps3_system_bus_device,
    pub p: *mut },
    pub __LINE__): pr_debug(" -> %s:%d\n", __func__,,
    pub layout): p = kzalloc_obj(struct,
    if (!p)
    pub -ENOMEM: return,
    pub PS3_MATCH_ID_GPU: p->dev.match_id =,
    pub PS3_MATCH_SUB_ID_GPU_RAMDISK: p->dev.match_sub_id =,
    pub PS3_DEVICE_TYPE_IOC0: p->dev.dev_type =,
    pub ps3_system_bus_device_register(&p->dev): result =,
    if (result) {
    pr_debug("%s:%d ps3_system_bus_device_register failed\n",
    pub __LINE__): __func__,,
    pub fail_device_register: goto,
    }
    pub __LINE__): pr_debug(" <- %s:%d\n", __func__,,
    pub 0: return,
    fail_device_register:
    pub __LINE__): pr_debug(" <- %s:%d failed\n", __func__,,
    pub result: return,
    }
//
// ps3_setup_dynamic_device - Setup a dynamic device from the repository
//
#[no_mangle]
unsafe extern "C" fn ps3_setup_dynamic_device(repo: *const ps3_repository_device) -> c_int {
    static int ps3_setup_dynamic_device(const struct ps3_repository_device *repo)
    {
    pub result: c_int,
    switch (repo.dev_type) {
    case PS3_DEV_TYPE_STOR_DISK:
    pub PS3_MATCH_ID_STOR_DISK): result = ps3_setup_storage_dev(repo,,
// Some devices are not accessible from the Other OS lpar.
    if (result == -ENODEV) {
    pub 0: result =,
    pr_debug("%s:%u: not accessible\n", __func__,
    }
    if (result)
    pr_debug("%s:%u ps3_setup_storage_dev failed\n",
    pub __LINE__): __func__,,
    case PS3_DEV_TYPE_STOR_ROM:
    pub PS3_MATCH_ID_STOR_ROM): result = ps3_setup_storage_dev(repo,,
    if (result)
    pr_debug("%s:%u ps3_setup_storage_dev failed\n",
    pub __LINE__): __func__,,
    case PS3_DEV_TYPE_STOR_FLASH:
    pub PS3_MATCH_ID_STOR_FLASH): result = ps3_setup_storage_dev(repo,,
    if (result)
    pr_debug("%s:%u ps3_setup_storage_dev failed\n",
    pub __LINE__): __func__,,
    default:
    pub 0: result =,
    pr_debug("%s:%u: unsupported dev_type %u\n", __func__, __LINE__,
    }
    pub result: return,
    }
//
// ps3_setup_static_device - Setup a static device from the repository
//
#[no_mangle]
unsafe extern "C" fn ps3_setup_static_device(repo: *const ps3_repository_device) -> int __init {
    static int __init ps3_setup_static_device(const struct ps3_repository_device *repo)
    {
    pub result: c_int,
    switch (repo.dev_type) {
    case PS3_DEV_TYPE_SB_GELIC:
    pub ps3_setup_gelic_device(repo): result =,
    if (result) {
    pr_debug("%s:%d ps3_setup_gelic_device failed\n",
    pub __LINE__): __func__,,
    }
    case PS3_DEV_TYPE_SB_USB:
// Each USB device has both an EHCI and an OHCI HC
    pub ps3_setup_ehci_device(repo): result =,
    if (result) {
    pr_debug("%s:%d ps3_setup_ehci_device failed\n",
    pub __LINE__): __func__,,
    }
    pub ps3_setup_ohci_device(repo): result =,
    if (result) {
    pr_debug("%s:%d ps3_setup_ohci_device failed\n",
    pub __LINE__): __func__,,
    }
    default:
    pub ps3_setup_dynamic_device(repo): return,
    }
    pub result: return,
    }
#[no_mangle]
unsafe extern "C" fn ps3_find_and_add_device(bus_id: u64, dev_id: u64) {
    static void ps3_find_and_add_device(u64 bus_id, u64 dev_id)
    {
    pub repo: ps3_repository_device,
    pub res: c_int,
    pub retries: c_uint,
    pub rem: c_ulong,
//
// On some firmware versions (e.g. 1.90), the device may not show up
// in the repository immediately
//
    pub {: for (retries = 0; retries < 10; retries++),
    pub dev_id): res = ps3_repository_find_device_by_id(&repo, bus_id,,
    if (!res)
    pub found: goto,
    pub msleep_interruptible(100): rem =,
    if (rem)
    }
    pr_warn("%s:%u: device %llu:%llu not found\n",
    pub dev_id): __func__, __LINE__, bus_id,,
    found:
    if (retries)
    pr_debug("%s:%u: device %llu:%llu found after %u retries\n",
    pub retries): __func__, __LINE__, bus_id, dev_id,,
    }

pub const PS3_NOTIFICATION_INTERRUPT_ID: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3_notification_device {
    pub sbd: ps3_system_bus_device,
    pub lock: spinlock_t,
    pub tag: u64,
    pub lv1_status: u64,
    pub wait: rcuwait,
    pub done: bool,
}

    enum ps3_notify_type {
    notify_device_ready = 0,
    notify_region_probe = 1,
    notify_region_update = 2,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3_notify_cmd {
    pub /: *mut *mut u64 operation_code; / must be zero,
    pub /: *mut *mut u64 event_mask; / OR of 1UL << enum ps3_notify_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3_notify_event {
    pub /: *mut *mut u64 event_type; / enum ps3_notify_type,
    pub bus_id: u64,
    pub dev_id: u64,
    pub dev_type: u64,
    pub dev_port: u64,
}

#[no_mangle]
unsafe extern "C" fn ps3_notification_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t ps3_notification_interrupt(int irq, void *data)
    {
    struct ps3_notification_device *dev = data;
    int res;
    u64 tag, status;
    spin_lock(&dev.lock);
    res = lv1_storage_get_async_status(PS3_NOTIFICATION_DEV_ID, &tag,
    &status);
    if (tag != dev.tag)
    pr_err("%s:%u: tag mismatch, got %llx, expected %llx\n",
    __func__, __LINE__, tag, dev.tag);
    if (res) {
    pr_err("%s:%u: res %d status 0x%llx\n", __func__, __LINE__, res,
    status);
    } else {
    pr_debug("%s:%u: completed, status 0x%llx\n", __func__,
    __LINE__, status);
    dev.lv1_status = status;
    dev.done = true;
    rcuwait_wake_up(&dev.wait);
    }
    spin_unlock(&dev.lock);
    return IRQ_HANDLED;
    }
    static int ps3_notification_read_write(struct ps3_notification_device *dev,
    u64 lpar, int write)
    {
    const char *op = str_write_read(write);
    unsigned long flags;
    int res;
    spin_lock_irqsave(&dev.lock, flags);
    res = write ? lv1_storage_write(dev.sbd.dev_id, 0, 0, 1, 0, lpar,
    &dev.tag)
    : lv1_storage_read(dev.sbd.dev_id, 0, 0, 1, 0, lpar,
    &dev.tag);
    dev.done = false;
    spin_unlock_irqrestore(&dev.lock, flags);
    if (res) {
    pr_err("%s:%u: %s failed %d\n", __func__, __LINE__, op, res);
    return -EPERM;
    }
    pr_debug("%s:%u: notification %s issued\n", __func__, __LINE__, op);
    rcuwait_wait_event(&dev.wait, dev.done || kthread_should_stop(), TASK_IDLE);
    if (kthread_should_stop())
    res = -EINTR;
    if (dev.lv1_status) {
    pr_err("%s:%u: %s not completed, status 0x%llx\n", __func__,
    __LINE__, op, dev.lv1_status);
    return -EIO;
    }
    pr_debug("%s:%u: notification %s completed\n", __func__, __LINE__, op);
    return 0;
    }
    static struct task_struct *probe_task;
//
// ps3_probe_thread - Background repository probing at system startup.
//
// This implementation only supports background probing on a single bus.
// It uses the hypervisor's storage device notification mechanism to wait until
// a storage device is ready.  The device notification mechanism uses a
// pseudo device to asynchronously notify the guest when storage devices become
// ready.  The notification device has a block size of 512 bytes.
//
#[no_mangle]
unsafe extern "C" fn ps3_probe_thread(data: *mut c_void) -> c_int {
    static int ps3_probe_thread(void *data)
    {
    struct {
    struct ps3_notification_device dev;
    u8 buf[512];
    } *local;
    struct ps3_notify_cmd *notify_cmd;
    struct ps3_notify_event *notify_event;
    int res;
    unsigned int irq;
    u64 lpar;
    pr_debug(" . %s:%u: kthread started\n", __func__, __LINE__);
    local = kzalloc_obj(*local);
    if (!local)
    return -ENOMEM;
    lpar = ps3_mm_phys_to_lpar(__pa(&local.buf));
    notify_cmd = (struct ps3_notify_cmd *)&local.buf;
    notify_event = (struct ps3_notify_event *)&local.buf;
// dummy system bus device
    local.dev.sbd.bus_id = (u64)data;
    local.dev.sbd.dev_id = PS3_NOTIFICATION_DEV_ID;
    local.dev.sbd.interrupt_id = PS3_NOTIFICATION_INTERRUPT_ID;
    res = lv1_open_device(local.dev.sbd.bus_id, local.dev.sbd.dev_id, 0);
    if (res) {
    pr_err("%s:%u: lv1_open_device failed %s\n", __func__,
    __LINE__, ps3_result(res));
    goto fail_free;
    }
    res = ps3_sb_event_receive_port_setup(&local.dev.sbd,
    PS3_BINDING_CPU_ANY, &irq);
    if (res) {
    pr_err("%s:%u: ps3_sb_event_receive_port_setup failed %d\n",
    __func__, __LINE__, res);
    goto fail_close_device;
    }
    spin_lock_init(&local.dev.lock);
    rcuwait_init(&local.dev.wait);
    res = request_irq(irq, ps3_notification_interrupt, 0,
    "ps3_notification", &local.dev);
    if (res) {
    pr_err("%s:%u: request_irq failed %d\n", __func__, __LINE__,
    res);
    goto fail_sb_event_receive_port_destroy;
    }
// Setup and write the request for device notification.
    notify_cmd.operation_code = 0; /* must be zero */
    notify_cmd.event_mask = 1UL << notify_region_probe;
    res = ps3_notification_read_write(&local.dev, lpar, 1);
    if (res)
    goto fail_free_irq;
    set_freezable();
// Loop here processing the requested notification events.
    do {
    try_to_freeze();
    memset(notify_event, 0, sizeof(*notify_event));
    res = ps3_notification_read_write(&local.dev, lpar, 0);
    if (res)
    break;
    pr_debug("%s:%u: notify event type 0x%llx bus id %llu dev id %llu"
    " type %llu port %llu\n", __func__, __LINE__,
    notify_event.event_type, notify_event.bus_id,
    notify_event.dev_id, notify_event.dev_type,
    notify_event.dev_port);
    if (notify_event.event_type != notify_region_probe ||
    notify_event.bus_id != local.dev.sbd.bus_id) {
    pr_warn("%s:%u: bad notify_event: event %llu, dev_id %llu, dev_type %llu\n",
    __func__, __LINE__, notify_event.event_type,
    notify_event.dev_id, notify_event.dev_type);
    continue;
    }
    ps3_find_and_add_device(local.dev.sbd.bus_id,
    notify_event.dev_id);
    } while (!kthread_should_stop());
    fail_free_irq:
    free_irq(irq, &local.dev);
    fail_sb_event_receive_port_destroy:
    ps3_sb_event_receive_port_destroy(&local.dev.sbd, irq);
    fail_close_device:
    lv1_close_device(local.dev.sbd.bus_id, local.dev.sbd.dev_id);
    fail_free:
    kfree(local);
    probe_task = core::ptr::null_mut();
    pr_debug(" <- %s:%u: kthread finished\n", __func__, __LINE__);
    return 0;
    }
//
// ps3_stop_probe_thread - Stops the background probe thread.
//
    static int ps3_stop_probe_thread(struct notifier_block *nb, unsigned long code,
    void *data)
    {
    if (probe_task)
    kthread_stop(probe_task);
    return 0;
    }
    static struct notifier_block nb = {
    .notifier_call = ps3_stop_probe_thread
    };
//
// ps3_start_probe_thread - Starts the background probe thread.
//
#[no_mangle]
unsafe extern "C" fn ps3_start_probe_thread(bus_type: enum ps3_bus_type) -> int __init {
    static int __init ps3_start_probe_thread(enum ps3_bus_type bus_type)
    {
    int result;
    struct task_struct *task;
    struct ps3_repository_device repo;
    pr_debug(" . %s:%d\n", __func__, __LINE__);
    memset(&repo, 0, sizeof(repo));
    repo.bus_type = bus_type;
    result = ps3_repository_find_bus(repo.bus_type, 0, &repo.bus_index);
    if (result) {
    printk(KERN_ERR "%s: Cannot find bus (%d)\n", __func__, result);
    return -ENODEV;
    }
    result = ps3_repository_read_bus_id(repo.bus_index, &repo.bus_id);
    if (result) {
    printk(KERN_ERR "%s: read_bus_id failed %d\n", __func__,
    result);
    return -ENODEV;
    }
    task = kthread_run(ps3_probe_thread, (void *)repo.bus_id,
    "ps3-probe-%u", bus_type);
    if (IS_ERR(task)) {
    result = PTR_ERR(task);
    printk(KERN_ERR "%s: kthread_run failed %d\n", __func__,
    result);
    return result;
    }
    probe_task = task;
    register_reboot_notifier(&nb);
    pr_debug(" <- %s:%d\n", __func__, __LINE__);
    return 0;
    }
//
// ps3_register_devices - Probe the system and register devices found.
//
// A device_initcall() routine.
//
#[no_mangle]
unsafe extern "C" fn ps3_register_devices() -> int __init {
    static int __init ps3_register_devices(void)
    {
    if (!firmware_has_feature(FW_FEATURE_PS3_LV1))
    return -ENODEV;
    pr_debug(" . %s:%d\n", __func__, __LINE__);
// ps3_repository_dump_bus_info();
    ps3_start_probe_thread(PS3_BUS_TYPE_STORAGE);
    ps3_register_vuart_devices();
    ps3_register_graphics_devices();
    ps3_repository_find_devices(PS3_BUS_TYPE_SB, ps3_setup_static_device);
    ps3_register_sound_devices();
    ps3_register_lpm_devices();
    ps3_register_ramdisk_device();
    pr_debug(" <- %s:%d\n", __func__, __LINE__);
    return 0;
    }
    device_initcall(ps3_register_devices);
