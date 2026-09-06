//! Automatically rewritten from C to Rust
//! Source: drivers/dma/bestcomm/sram.c
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
// Simple memory allocator for on-board SRAM
//
// Maintainer : Sylvain Munaut <tnt@246tNt.com>
//
// Copyright (C) 2005 Sylvain Munaut <tnt@246tNt.com>
//

// Struct keeping our 'state'
    struct bcom_sram *bcom_sram = core::ptr::null_mut();
    EXPORT_SYMBOL_GPL(bcom_sram);	/* needed for inline functions */
// ========================================================================
// Public API
// ========================================================================
// DO NOT USE in interrupts, if needed in irq handler, we should use the
    _irqsave version of the spin_locks */
#[no_mangle]
pub unsafe extern "C" fn bcom_sram_init(sram_node: *mut device_node, owner: *mut c_char) -> c_int {
    int bcom_sram_init(struct device_node *sram_node, char *owner)
    {
    int rv;
    const u32 *regaddr_p;
    struct resource res;
    unsigned int psize;
// Create our state struct
    if (bcom_sram) {
    printk(KERN_ERR "%s: bcom_sram_init: "
    "Already initialized !\n", owner);
    return -EBUSY;
    }
    bcom_sram = kmalloc_obj(struct bcom_sram);
    if (!bcom_sram) {
    printk(KERN_ERR "%s: bcom_sram_init: "
    "Couldn't allocate internal state !\n", owner);
    return -ENOMEM;
    }
// Get address and size of the sram
    rv = of_address_to_resource(sram_node, 0, &res);
    if (rv) {
    printk(KERN_ERR "%s: bcom_sram_init: "
    "Invalid device node !\n", owner);
    goto error_free;
    }
    bcom_sram.base_phys = res.start;
    bcom_sram.size = resource_size(&res);
// Request region
    if (!request_mem_region(res.start, resource_size(&res), owner)) {
    printk(KERN_ERR "%s: bcom_sram_init: "
    "Couldn't request region !\n", owner);
    rv = -EBUSY;
    goto error_free;
    }
// Map SRAM
// sram is not really __iomem
    bcom_sram.base_virt = (void *)ioremap(res.start, resource_size(&res));
    if (!bcom_sram.base_virt) {
    printk(KERN_ERR "%s: bcom_sram_init: "
    "Map error SRAM zone 0x%08lx (0x%0x)!\n",
    owner, (long)bcom_sram.base_phys, bcom_sram.size );
    rv = -ENOMEM;
    goto error_release;
    }
// Create an rheap (defaults to 32 bits word alignment)
    bcom_sram.rh = rh_create(4);
// Attach the free zones
    regaddr_p = core::ptr::null_mut();
    psize = 0;
    if (!regaddr_p || !psize) {
// Attach the whole zone
    rh_attach_region(bcom_sram.rh, 0, bcom_sram.size);
    } else {
// Attach each zone independently
    while (psize >= 2 * sizeof(u32)) {
    let mut zbase: phys_addr_t = of_translate_address(sram_node, regaddr_p);
    rh_attach_region(bcom_sram.rh, zbase - bcom_sram.base_phys, regaddr_p[1]);
    regaddr_p += 2;
    psize -= 2 * sizeof(u32);
    }
    }
// Init our spinlock
    spin_lock_init(&bcom_sram.lock);
    return 0;
    error_release:
    release_mem_region(res.start, resource_size(&res));
    error_free:
    kfree(bcom_sram);
    bcom_sram = core::ptr::null_mut();
    return rv;
    }
    EXPORT_SYMBOL_GPL(bcom_sram_init);
#[no_mangle]
pub unsafe extern "C" fn bcom_sram_cleanup() {
    void bcom_sram_cleanup(void)
    {
// Free resources
    if (bcom_sram) {
    rh_destroy(bcom_sram.rh);
    iounmap((void __iomem *)bcom_sram.base_virt);
    release_mem_region(bcom_sram.base_phys, bcom_sram.size);
    kfree(bcom_sram);
    bcom_sram = core::ptr::null_mut();
    }
    }
    EXPORT_SYMBOL_GPL(bcom_sram_cleanup);
#[no_mangle]
pub unsafe extern "C" fn bcom_sram_alloc(size: c_int, align: c_int, phys: *mut phys_addr_t) -> *mut c_void {
    void* bcom_sram_alloc(int size, int align, phys_addr_t *phys)
    {
    unsigned long offset;
    spin_lock(&bcom_sram.lock);
    offset = rh_alloc_align(bcom_sram.rh, size, align, core::ptr::null_mut());
    spin_unlock(&bcom_sram.lock);
    if (IS_ERR_VALUE(offset))
    return core::ptr::null_mut();
// phys = bcom_sram->base_phys + offset;
    return bcom_sram.base_virt + offset;
    }
    EXPORT_SYMBOL_GPL(bcom_sram_alloc);
#[no_mangle]
pub unsafe extern "C" fn bcom_sram_free(ptr: *mut c_void) {
    void bcom_sram_free(void *ptr)
    {
    unsigned long offset;
    if (!ptr)
    return;
    offset = ptr - bcom_sram.base_virt;
    spin_lock(&bcom_sram.lock);
    rh_free(bcom_sram.rh, offset);
    spin_unlock(&bcom_sram.lock);
    }
    EXPORT_SYMBOL_GPL(bcom_sram_free);
