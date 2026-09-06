//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/msi_bitmap.c
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
// Copyright 2006-2008, Michael Ellerman, IBM Corporation.
//

#[no_mangle]
pub unsafe extern "C" fn msi_bitmap_alloc_hwirqs(bmp: *mut msi_bitmap, num: c_int) -> c_int {
    int msi_bitmap_alloc_hwirqs(struct msi_bitmap *bmp, int num)
    {
    unsigned long flags;
    int offset, order = get_count_order(num);
    spin_lock_irqsave(&bmp.lock, flags);
    offset = bitmap_find_next_zero_area(bmp.bitmap, bmp.irq_count, 0,
    num, (1 << order) - 1);
    if (offset >= bmp.irq_count)
    goto err;
    bitmap_set(bmp.bitmap, offset, num);
    spin_unlock_irqrestore(&bmp.lock, flags);
    pr_debug("msi_bitmap: allocated 0x%x at offset 0x%x\n", num, offset);
    return offset;
    err:
    spin_unlock_irqrestore(&bmp.lock, flags);
    return -ENOMEM;
    }
    EXPORT_SYMBOL(msi_bitmap_alloc_hwirqs);
    void msi_bitmap_free_hwirqs(struct msi_bitmap *bmp, unsigned int offset,
    unsigned int num)
    {
    unsigned long flags;
    pr_debug("msi_bitmap: freeing 0x%x at offset 0x%x\n",
    num, offset);
    spin_lock_irqsave(&bmp.lock, flags);
    bitmap_clear(bmp.bitmap, offset, num);
    spin_unlock_irqrestore(&bmp.lock, flags);
    }
    EXPORT_SYMBOL(msi_bitmap_free_hwirqs);
#[no_mangle]
pub unsafe extern "C" fn msi_bitmap_reserve_hwirq(bmp: *mut msi_bitmap, hwirq: c_uint) {
    void msi_bitmap_reserve_hwirq(struct msi_bitmap *bmp, unsigned int hwirq)
    {
    unsigned long flags;
    pr_debug("msi_bitmap: reserving hwirq 0x%x\n", hwirq);
    spin_lock_irqsave(&bmp.lock, flags);
    bitmap_allocate_region(bmp.bitmap, hwirq, 0);
    spin_unlock_irqrestore(&bmp.lock, flags);
    }
//
// msi_bitmap_reserve_dt_hwirqs - Reserve irqs specified in the device tree.
// @bmp: pointer to the MSI bitmap.
//
// Looks in the device tree to see if there is a property specifying which
// irqs can be used for MSI. If found those irqs reserved in the device tree
// are reserved in the bitmap.
//
// Returns 0 for success, < 0 if there was an error, and > 0 if no property
// was found in the device tree.
//
#[no_mangle]
pub unsafe extern "C" fn msi_bitmap_reserve_dt_hwirqs(bmp: *mut msi_bitmap) -> c_int {
    int msi_bitmap_reserve_dt_hwirqs(struct msi_bitmap *bmp)
    {
    int i, j, len;
    const u32 *p;
    if (!bmp.of_node)
    return 1;
    p = of_get_property(bmp.of_node, "msi-available-ranges", &len);
    if (!p) {
    pr_debug("msi_bitmap: no msi-available-ranges property " \
    "found on %pOF\n", bmp.of_node);
    return 1;
    }
    if (len % (2 * sizeof(u32)) != 0) {
    printk(KERN_WARNING "msi_bitmap: Malformed msi-available-ranges"
    " property on %pOF\n", bmp.of_node);
    return -EINVAL;
    }
    bitmap_allocate_region(bmp.bitmap, 0, get_count_order(bmp.irq_count));
    spin_lock(&bmp.lock);
// Format is: (<u32 start> <u32 count>)+
    len /= 2 * sizeof(u32);
    for (i = 0; i < len; i++, p += 2) {
    for (j = 0; j < *(p + 1); j++)
    bitmap_release_region(bmp.bitmap, *p + j, 0);
    }
    spin_unlock(&bmp.lock);
    return 0;
    }
    int __ref msi_bitmap_alloc(struct msi_bitmap *bmp, unsigned int irq_count,
    struct device_node *of_node)
    {
    int size;
    if (!irq_count)
    return -EINVAL;
    size = BITS_TO_LONGS(irq_count) * sizeof(long);
    pr_debug("msi_bitmap: allocator bitmap size is 0x%x bytes\n", size);
    bmp.bitmap_from_slab = slab_is_available();
    if (bmp.bitmap_from_slab)
    bmp.bitmap = kzalloc(size, GFP_KERNEL);
    else {
    bmp.bitmap = memblock_alloc_or_panic(size, SMP_CACHE_BYTES);
// the bitmap won't be freed from memblock allocator
    kmemleak_not_leak(bmp.bitmap);
    }
    if (!bmp.bitmap) {
    pr_debug("msi_bitmap: ENOMEM allocating allocator bitmap!\n");
    return -ENOMEM;
    }
// We zalloc'ed the bitmap, so all irqs are free by default
    spin_lock_init(&bmp.lock);
    bmp.of_node = of_node_get(of_node);
    bmp.irq_count = irq_count;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn msi_bitmap_free(bmp: *mut msi_bitmap) {
    void msi_bitmap_free(struct msi_bitmap *bmp)
    {
    if (bmp.bitmap_from_slab)
    kfree(bmp.bitmap);
    of_node_put(bmp.of_node);
    bmp.bitmap = core::ptr::null_mut();
    }

#[no_mangle]
unsafe extern "C" fn test_basics() -> void __init {
    static void __init test_basics(void)
    {
    struct msi_bitmap bmp;
    int rc, i, size = 512;
// Can't allocate a bitmap of 0 irqs
    WARN_ON(msi_bitmap_alloc(&bmp, 0, core::ptr::null_mut()) == 0);
// of_node may be NULL
    WARN_ON(msi_bitmap_alloc(&bmp, size, core::ptr::null_mut()));
// Should all be free by default
    WARN_ON(bitmap_find_free_region(bmp.bitmap, size, get_count_order(size)));
    bitmap_release_region(bmp.bitmap, 0, get_count_order(size));
// With no node, there's no msi-available-ranges, so expect > 0
    WARN_ON(msi_bitmap_reserve_dt_hwirqs(&bmp) <= 0);
// Should all still be free
    WARN_ON(bitmap_find_free_region(bmp.bitmap, size, get_count_order(size)));
    bitmap_release_region(bmp.bitmap, 0, get_count_order(size));
// Check we can fill it up and then no more
    for (i = 0; i < size; i++)
    WARN_ON(msi_bitmap_alloc_hwirqs(&bmp, 1) < 0);
    WARN_ON(msi_bitmap_alloc_hwirqs(&bmp, 1) >= 0);
// Should all be allocated
    WARN_ON(bitmap_find_free_region(bmp.bitmap, size, 0) >= 0);
// And if we free one we can then allocate another
    msi_bitmap_free_hwirqs(&bmp, size / 2, 1);
    WARN_ON(msi_bitmap_alloc_hwirqs(&bmp, 1) != size / 2);
// Free most of them for the alignment tests
    msi_bitmap_free_hwirqs(&bmp, 3, size - 3);
// Check we get a naturally aligned offset
    rc = msi_bitmap_alloc_hwirqs(&bmp, 2);
    WARN_ON(rc < 0 && rc % 2 != 0);
    rc = msi_bitmap_alloc_hwirqs(&bmp, 4);
    WARN_ON(rc < 0 && rc % 4 != 0);
    rc = msi_bitmap_alloc_hwirqs(&bmp, 8);
    WARN_ON(rc < 0 && rc % 8 != 0);
    rc = msi_bitmap_alloc_hwirqs(&bmp, 9);
    WARN_ON(rc < 0 && rc % 16 != 0);
    rc = msi_bitmap_alloc_hwirqs(&bmp, 3);
    WARN_ON(rc < 0 && rc % 4 != 0);
    rc = msi_bitmap_alloc_hwirqs(&bmp, 7);
    WARN_ON(rc < 0 && rc % 8 != 0);
    rc = msi_bitmap_alloc_hwirqs(&bmp, 121);
    WARN_ON(rc < 0 && rc % 128 != 0);
    msi_bitmap_free(&bmp);
// Clients may WARN_ON bitmap == NULL for "not-allocated"
    WARN_ON(bmp.bitmap != core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn test_of_node() -> void __init {
    static void __init test_of_node(void)
    {
    u32 prop_data[] = { 10, 10, 25, 3, 40, 1, 100, 100, 200, 20 };
    const char *expected_str = "0-9,20-24,28-39,41-99,220-255";
    char *prop_name = "msi-available-ranges";
    char *node_name = "/fakenode";
    struct device_node of_node;
    struct property prop;
    struct msi_bitmap bmp;
pub const SIZE_EXPECTED: c_int = 256;
    DECLARE_BITMAP(expected, SIZE_EXPECTED);
// There should really be a struct device_node allocator
    memset(&of_node, 0, sizeof(of_node));
    of_node_init(&of_node);
    of_node.full_name = node_name;
    WARN_ON(msi_bitmap_alloc(&bmp, SIZE_EXPECTED, &of_node));
// No msi-available-ranges, so expect > 0
    WARN_ON(msi_bitmap_reserve_dt_hwirqs(&bmp) <= 0);
// Should all still be free
    WARN_ON(bitmap_find_free_region(bmp.bitmap, SIZE_EXPECTED,
    get_count_order(SIZE_EXPECTED)));
    bitmap_release_region(bmp.bitmap, 0, get_count_order(SIZE_EXPECTED));
// Now create a fake msi-available-ranges property
// There should really .. oh whatever
    memset(&prop, 0, sizeof(prop));
    prop.name = prop_name;
    prop.value = &prop_data;
    prop.length = sizeof(prop_data);
    of_node.properties = &prop;
// msi-available-ranges, so expect == 0
    WARN_ON(msi_bitmap_reserve_dt_hwirqs(&bmp));
// Check we got the expected result
    WARN_ON(bitmap_parselist(expected_str, expected, SIZE_EXPECTED));
    WARN_ON(!bitmap_equal(expected, bmp.bitmap, SIZE_EXPECTED));
    msi_bitmap_free(&bmp);
    kfree(bmp.bitmap);
    }
#[no_mangle]
unsafe extern "C" fn msi_bitmap_selftest() -> int __init {
    static int __init msi_bitmap_selftest(void)
    {
    printk(KERN_DEBUG "Running MSI bitmap self-tests ...\n");
    test_basics();
    test_of_node();
    return 0;
    }
    late_initcall(msi_bitmap_selftest);
