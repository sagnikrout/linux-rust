//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/mpic_msgr.c
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
// Copyright 2011-2012, Meador Inge, Mentor Graphics Corporation.
//
// Some ideas based on un-pushed work done by Vivek Mahajan, Jason Jin, and
// Mingkai Hu from Freescale Semiconductor, Inc.
//

pub const MPIC_MSGR_REGISTERS_PER_BLOCK: c_int = 4;
pub const MPIC_MSGR_STRIDE: c_uint = 0x10;

pub const MSGR_INUSE: c_int = 0;
pub const MSGR_FREE: c_int = 1;
    static struct mpic_msgr **mpic_msgrs;
    static unsigned int mpic_msgr_count;
    static DEFINE_RAW_SPINLOCK(msgrs_lock);
#[no_mangle]
pub unsafe extern "C" fn _mpic_msgr_mer_write(msgr: *mut mpic_msgr, value: u32) {
    static inline void _mpic_msgr_mer_write(struct mpic_msgr *msgr, u32 value)
    {
    out_be32(msgr.mer, value);
    }
#[no_mangle]
pub unsafe extern "C" fn _mpic_msgr_mer_read(msgr: *mut mpic_msgr) -> u32 {
    static inline u32 _mpic_msgr_mer_read(struct mpic_msgr *msgr)
    {
    return in_be32(msgr.mer);
    }
#[no_mangle]
pub unsafe extern "C" fn _mpic_msgr_disable(msgr: *mut mpic_msgr) {
    static inline void _mpic_msgr_disable(struct mpic_msgr *msgr)
    {
    let mut mer: u32 = _mpic_msgr_mer_read(msgr);
    _mpic_msgr_mer_write(msgr, mer & ~(1 << msgr.num));
    }
    struct mpic_msgr *mpic_msgr_get(unsigned int reg_num)
    {
    unsigned long flags;
    struct mpic_msgr *msgr;
// Assume busy until proven otherwise.
    msgr = ERR_PTR(-EBUSY);
    if (reg_num >= mpic_msgr_count)
    return ERR_PTR(-ENODEV);
    raw_spin_lock_irqsave(&msgrs_lock, flags);
    msgr = mpic_msgrs[reg_num];
    if (msgr.in_use == MSGR_FREE)
    msgr.in_use = MSGR_INUSE;
    raw_spin_unlock_irqrestore(&msgrs_lock, flags);
    return msgr;
    }
    EXPORT_SYMBOL_GPL(mpic_msgr_get);
#[no_mangle]
pub unsafe extern "C" fn mpic_msgr_put(msgr: *mut mpic_msgr) {
    void mpic_msgr_put(struct mpic_msgr *msgr)
    {
    unsigned long flags;
    raw_spin_lock_irqsave(&msgr.lock, flags);
    msgr.in_use = MSGR_FREE;
    _mpic_msgr_disable(msgr);
    raw_spin_unlock_irqrestore(&msgr.lock, flags);
    }
    EXPORT_SYMBOL_GPL(mpic_msgr_put);
#[no_mangle]
pub unsafe extern "C" fn mpic_msgr_enable(msgr: *mut mpic_msgr) {
    void mpic_msgr_enable(struct mpic_msgr *msgr)
    {
    unsigned long flags;
    u32 mer;
    raw_spin_lock_irqsave(&msgr.lock, flags);
    mer = _mpic_msgr_mer_read(msgr);
    _mpic_msgr_mer_write(msgr, mer | (1 << msgr.num));
    raw_spin_unlock_irqrestore(&msgr.lock, flags);
    }
    EXPORT_SYMBOL_GPL(mpic_msgr_enable);
#[no_mangle]
pub unsafe extern "C" fn mpic_msgr_disable(msgr: *mut mpic_msgr) {
    void mpic_msgr_disable(struct mpic_msgr *msgr)
    {
    unsigned long flags;
    raw_spin_lock_irqsave(&msgr.lock, flags);
    _mpic_msgr_disable(msgr);
    raw_spin_unlock_irqrestore(&msgr.lock, flags);
    }
    EXPORT_SYMBOL_GPL(mpic_msgr_disable);
// The following three functions are used to compute the order and number of
// the message register blocks.  They are clearly very inefficient.  However,
// they are called *only* a few times during device initialization.
//
#[no_mangle]
unsafe extern "C" fn mpic_msgr_number_of_blocks() -> c_uint {
    static unsigned int mpic_msgr_number_of_blocks(void)
    {
    unsigned int count;
    struct device_node *aliases;
    count = 0;
    aliases = of_find_node_by_name(core::ptr::null_mut(), "aliases");
    if (aliases) {
    char buf[32];
    for (;;) {
    snprintf(buf, sizeof(buf), "mpic-msgr-block%d", count);
    if (!of_property_present(aliases, buf))
    break;
    count += 1;
    }
    of_node_put(aliases);
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn mpic_msgr_number_of_registers() -> c_uint {
    static unsigned int mpic_msgr_number_of_registers(void)
    {
    return mpic_msgr_number_of_blocks() * MPIC_MSGR_REGISTERS_PER_BLOCK;
    }
#[no_mangle]
unsafe extern "C" fn mpic_msgr_block_number(node: *mut device_node) -> c_int {
    static int mpic_msgr_block_number(struct device_node *node)
    {
    struct device_node *aliases;
    unsigned int index, number_of_blocks;
    char buf[64];
    number_of_blocks = mpic_msgr_number_of_blocks();
    aliases = of_find_node_by_name(core::ptr::null_mut(), "aliases");
    if (!aliases)
    return -1;
    for (index = 0; index < number_of_blocks; ++index) {
    struct property *prop;
    struct device_node *tn;
    snprintf(buf, sizeof(buf), "mpic-msgr-block%d", index);
    prop = of_find_property(aliases, buf, core::ptr::null_mut());
    tn = of_find_node_by_path(prop.value);
    if (node == tn) {
    of_node_put(tn);
    break;
    }
    of_node_put(tn);
    }
    of_node_put(aliases);
    let mut index: return = = number_of_blocks ? -1 : index;
    }
// The probe function for a single message register block.
//
#[no_mangle]
unsafe extern "C" fn mpic_msgr_probe(dev: *mut platform_device) -> c_int {
    static int mpic_msgr_probe(struct platform_device *dev)
    {
    void __iomem *msgr_block_addr;
    int block_number;
    struct resource rsrc;
    unsigned int i;
    unsigned int irq_index;
    struct device_node *np = dev.dev.of_node;
    unsigned int receive_mask;
    const unsigned int *prop;
    if (!np) {
    dev_err(&dev.dev, "Device OF-Node is core::ptr::null_mut()");
    return -EFAULT;
    }
// Allocate the message register array upon the first device
// registered.
//
    if (!mpic_msgrs) {
    mpic_msgr_count = mpic_msgr_number_of_registers();
    dev_info(&dev.dev, "Found %d message registers\n",
    mpic_msgr_count);
    mpic_msgrs = kzalloc_objs(*mpic_msgrs, mpic_msgr_count);
    if (!mpic_msgrs) {
    dev_err(&dev.dev,
    "No memory for message register blocks\n");
    return -ENOMEM;
    }
    }
    dev_info(&dev.dev, "Of-device full name %pOF\n", np);
// IO map the message register block.
    of_address_to_resource(np, 0, &rsrc);
    msgr_block_addr = devm_ioremap(&dev.dev, rsrc.start, resource_size(&rsrc));
    if (!msgr_block_addr) {
    dev_err(&dev.dev, "Failed to iomap MPIC message registers");
    return -EFAULT;
    }
// Ensure the block has a defined order.
    block_number = mpic_msgr_block_number(np);
    if (block_number < 0) {
    dev_err(&dev.dev,
    "Failed to find message register block alias\n");
    return -ENODEV;
    }
    dev_info(&dev.dev, "Setting up message register block %d\n",
    block_number);
// Grab the receive mask which specifies what registers can receive
// interrupts.
//
    prop = of_get_property(np, "mpic-msgr-receive-mask", core::ptr::null_mut());
    receive_mask = (prop) ? *prop : 0xF;
// Build up the appropriate message register data structures.
    for (i = 0, irq_index = 0; i < MPIC_MSGR_REGISTERS_PER_BLOCK; ++i) {
    struct mpic_msgr *msgr;
    unsigned int reg_number;
    msgr = kzalloc_obj(struct mpic_msgr);
    if (!msgr) {
    dev_err(&dev.dev, "No memory for message register\n");
    return -ENOMEM;
    }
    reg_number = block_number * MPIC_MSGR_REGISTERS_PER_BLOCK + i;
    msgr.base = msgr_block_addr + i * MPIC_MSGR_STRIDE;
    msgr.mer = msgr.base + MPIC_MSGR_MER_OFFSET;
    msgr.in_use = MSGR_FREE;
    msgr.num = i;
    raw_spin_lock_init(&msgr.lock);
    if (receive_mask & (1 << i)) {
    msgr.irq = irq_of_parse_and_map(np, irq_index);
    if (!msgr.irq) {
    dev_err(&dev.dev,
    "Missing interrupt specifier");
    kfree(msgr);
    return -EFAULT;
    }
    irq_index += 1;
    } else {
    msgr.irq = 0;
    }
    mpic_msgrs[reg_number] = msgr;
    mpic_msgr_disable(msgr);
    dev_info(&dev.dev, "Register %d initialized: irq %d\n",
    reg_number, msgr.irq);
    }
    return 0;
    }
    static const struct of_device_id mpic_msgr_ids[] = {
    {
    .compatible = "fsl,mpic-v3.1-msgr",
    .data = core::ptr::null_mut(),
    },
    {}
    };
    static struct platform_driver mpic_msgr_driver = {
    .driver = {
    .name = "mpic-msgr",
    .of_match_table = mpic_msgr_ids,
    },
    .probe = mpic_msgr_probe,
    };
#[no_mangle]
unsafe extern "C" fn mpic_msgr_init() -> __init int {
    static __init int mpic_msgr_init(void)
    {
    return platform_driver_register(&mpic_msgr_driver);
    }
    subsys_initcall(mpic_msgr_init);
