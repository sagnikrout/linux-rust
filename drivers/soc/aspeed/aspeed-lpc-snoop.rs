//! Automatically rewritten from C to Rust
//! Source: drivers/soc/aspeed/aspeed-lpc-snoop.c
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
// Copyright 2017 Google Inc
//
// Provides a simple driver to control the ASPEED LPC snoop interface which
// allows the BMC to listen on and save the data written by
// the host to an arbitrary LPC I/O port.
//
// Typically used by the BMC to "watch" host boot progress via port
// 0x80 writes made by the BIOS during the boot process.
//

pub const SNOOP_FIFO_SIZE: c_int = 2048;
pub const HICR5: c_uint = 0x80;

pub const HICR6: c_uint = 0x84;

pub const SNPWADR: c_uint = 0x90;

pub const SNPWADR_CH0_SHIFT: c_int = 0;

pub const SNPWADR_CH1_SHIFT: c_int = 16;
pub const SNPWDR: c_uint = 0x94;

pub const SNPWDR_CH0_SHIFT: c_int = 0;

pub const SNPWDR_CH1_SHIFT: c_int = 8;
pub const HICRB: c_uint = 0x100;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_lpc_snoop_model_data {
// The ast2400 has bits 14 and 15 as reserved, whereas the ast2500
// can use them.
//
    pub has_hicrb_ensnp: c_uint,
}

    enum aspeed_lpc_snoop_index {
    ASPEED_LPC_SNOOP_INDEX_0 = 0,
    ASPEED_LPC_SNOOP_INDEX_1 = 1,
    ASPEED_LPC_SNOOP_INDEX_MAX = ASPEED_LPC_SNOOP_INDEX_1,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_lpc_snoop_channel_cfg {
    pub index: enum aspeed_lpc_snoop_index,
    pub hicr5_en: u32,
    pub snpwadr_mask: u32,
    pub snpwadr_shift: u32,
    pub hicrb_en: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_lpc_snoop_channel {
    pub cfg: *const aspeed_lpc_snoop_channel_cfg,
    pub enabled: bool,
    pub lock: spinlock_t,
    pub __guarded_by(&lock): kfifo fifo,
    pub wq: wait_queue_head_t,
    pub miscdev: miscdevice,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_lpc_snoop {
    pub regmap: *mut regmap,
    pub irq: c_int,
    pub clk: *mut clk,
    pub 1]: aspeed_lpc_snoop_channel chan[ASPEED_LPC_SNOOP_INDEX_MAX +,
}

    static const struct aspeed_lpc_snoop_channel_cfg channel_cfgs[ASPEED_LPC_SNOOP_INDEX_MAX + 1] = {
    {
    .index = ASPEED_LPC_SNOOP_INDEX_0,
    .hicr5_en = HICR5_EN_SNP0W | HICR5_ENINT_SNP0W,
    .snpwadr_mask = SNPWADR_CH0_MASK,
    .snpwadr_shift = SNPWADR_CH0_SHIFT,
    .hicrb_en = HICRB_ENSNP0D,
    },
    {
    .index = ASPEED_LPC_SNOOP_INDEX_1,
    .hicr5_en = HICR5_EN_SNP1W | HICR5_ENINT_SNP1W,
    .snpwadr_mask = SNPWADR_CH1_MASK,
    .snpwadr_shift = SNPWADR_CH1_SHIFT,
    .hicrb_en = HICRB_ENSNP1D,
    },
    };
    static struct aspeed_lpc_snoop_channel *snoop_file_to_chan(struct file *file)
    {
    return container_of(file.private_data,
    struct aspeed_lpc_snoop_channel,
    miscdev);
    }
    static ssize_t snoop_file_read(struct file *file, char __user *buffer,
    size_t count, loff_t *ppos)
    {
    struct aspeed_lpc_snoop_channel *chan = snoop_file_to_chan(file);
    u8 *buf __free(kfree) = core::ptr::null_mut();
    unsigned int copied;
    let mut ret: c_int = 0;
    if (kfifo_is_empty(&chan.fifo)) {
    if (file.f_flags & O_NONBLOCK)
    return -EAGAIN;
    ret = wait_event_interruptible(chan.wq,
    !kfifo_is_empty(&chan.fifo));
    if (ret == -ERESTARTSYS)
    return -EINTR;
    }
    count = min_t(size_t, count, SNOOP_FIFO_SIZE);
    buf = kmalloc(count, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    copied = kfifo_out_spinlocked(&chan.fifo, buf, count, &chan.lock);
    if (copied && copy_to_user(buffer, buf, copied))
    return -EFAULT;
    return copied;
    }
    static __poll_t snoop_file_poll(struct file *file,
    struct poll_table_struct *pt)
    {
    struct aspeed_lpc_snoop_channel *chan = snoop_file_to_chan(file);
    poll_wait(file, &chan.wq, pt);
    return !kfifo_is_empty(&chan.fifo) ? EPOLLIN : 0;
    }
    static const struct file_operations snoop_fops = {
    .owner  = THIS_MODULE,
    .read   = snoop_file_read,
    .poll   = snoop_file_poll,
    .llseek = noop_llseek,
    };
// Save a byte to a FIFO and discard the oldest byte if FIFO is full
#[no_mangle]
unsafe extern "C" fn put_fifo_with_discard(chan: *mut aspeed_lpc_snoop_channel, val: u8) {
    static void put_fifo_with_discard(struct aspeed_lpc_snoop_channel *chan, u8 val)
    {
    scoped_guard(spinlock, &chan.lock) {
    if (!kfifo_initialized(&chan.fifo))
    return;
    if (kfifo_is_full(&chan.fifo))
    kfifo_skip(&chan.fifo);
    kfifo_put(&chan.fifo, val);
    }
    wake_up_interruptible(&chan.wq);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_lpc_snoop_irq(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t aspeed_lpc_snoop_irq(int irq, void *arg)
    {
    struct aspeed_lpc_snoop *lpc_snoop = arg;
    u32 reg, data;
    if (regmap_read(lpc_snoop.regmap, HICR6, &reg))
    return IRQ_NONE;
// Check if one of the snoop channels is interrupting
    reg &= (HICR6_STR_SNP0W | HICR6_STR_SNP1W);
    if (!reg)
    return IRQ_NONE;
// Ack pending IRQs
    regmap_write(lpc_snoop.regmap, HICR6, reg);
// Read and save most recent snoop'ed data byte to FIFO
    regmap_read(lpc_snoop.regmap, SNPWDR, &data);
    if (reg & HICR6_STR_SNP0W) {
    let mut val: u8 = (data & SNPWDR_CH0_MASK) >> SNPWDR_CH0_SHIFT;
    put_fifo_with_discard(&lpc_snoop.chan[0], val);
    }
    if (reg & HICR6_STR_SNP1W) {
    let mut val: u8 = (data & SNPWDR_CH1_MASK) >> SNPWDR_CH1_SHIFT;
    put_fifo_with_discard(&lpc_snoop.chan[1], val);
    }
    return IRQ_HANDLED;
    }
    static int aspeed_lpc_snoop_config_irq(struct aspeed_lpc_snoop *lpc_snoop,
    struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    int rc;
    lpc_snoop.irq = platform_get_irq(pdev, 0);
    if (lpc_snoop.irq < 0)
    return -ENODEV;
    rc = devm_request_irq(dev, lpc_snoop.irq,
    aspeed_lpc_snoop_irq, IRQF_SHARED,
    DEVICE_NAME, lpc_snoop);
    if (rc < 0) {
    dev_warn(dev, "Unable to request IRQ %d\n", lpc_snoop.irq);
    lpc_snoop.irq = 0;
    return rc;
    }
    return 0;
    }
    __attribute__((nonnull))
    static int aspeed_lpc_enable_snoop(struct device *dev,
    struct aspeed_lpc_snoop *lpc_snoop,
    struct aspeed_lpc_snoop_channel *channel,
    const struct aspeed_lpc_snoop_channel_cfg *cfg,
    u16 lpc_port)
    {
    const struct aspeed_lpc_snoop_model_data *model_data;
    let mut rc: c_int = 0;
    if (WARN_ON(channel.enabled))
    return -EBUSY;
    init_waitqueue_head(&channel.wq);
    channel.cfg = cfg;
    channel.miscdev.minor = MISC_DYNAMIC_MINOR;
    channel.miscdev.fops = &snoop_fops;
    channel.miscdev.parent = dev;
    channel.miscdev.name =
    devm_kasprintf(dev, GFP_KERNEL, "%s%d", DEVICE_NAME, cfg.index);
    if (!channel.miscdev.name)
    return -ENOMEM;
    scoped_guard(spinlock_init, &channel.lock) {
    rc = kfifo_alloc(&channel.fifo, SNOOP_FIFO_SIZE, GFP_KERNEL);
    if (rc)
    return rc;
    }
    rc = misc_register(&channel.miscdev);
    if (rc)
    goto err_free_fifo;
// Enable LPC snoop channel at requested port
    regmap_set_bits(lpc_snoop.regmap, HICR5, cfg.hicr5_en);
    regmap_update_bits(lpc_snoop.regmap, SNPWADR, cfg.snpwadr_mask,
    lpc_port << cfg.snpwadr_shift);
    model_data = of_device_get_match_data(dev);
    if (model_data && model_data.has_hicrb_ensnp)
    regmap_set_bits(lpc_snoop.regmap, HICRB, cfg.hicrb_en);
    channel.enabled = true;
    return 0;
    err_free_fifo:
    kfifo_free(&channel.fifo);
    return rc;
    }
    __attribute__((nonnull))
    static void aspeed_lpc_disable_snoop(struct aspeed_lpc_snoop *lpc_snoop,
    struct aspeed_lpc_snoop_channel *channel)
    {
    if (!channel.enabled)
    return;
// Disable interrupts along with the device
    regmap_clear_bits(lpc_snoop.regmap, HICR5, channel.cfg.hicr5_en);
    channel.enabled = false;
// Consider improving safety wrt concurrent reader(s)
    misc_deregister(&channel.miscdev);
    kfifo_free(&channel.fifo);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_lpc_snoop_remove(pdev: *mut platform_device) {
    static void aspeed_lpc_snoop_remove(struct platform_device *pdev)
    {
    struct aspeed_lpc_snoop *lpc_snoop = dev_get_drvdata(&pdev.dev);
// Disable both snoop channels
    aspeed_lpc_disable_snoop(lpc_snoop, &lpc_snoop.chan[0]);
    aspeed_lpc_disable_snoop(lpc_snoop, &lpc_snoop.chan[1]);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_lpc_snoop_probe(pdev: *mut platform_device) -> c_int {
    static int aspeed_lpc_snoop_probe(struct platform_device *pdev)
    {
    struct aspeed_lpc_snoop *lpc_snoop;
    struct device_node *np;
    struct device *dev;
    int idx;
    int rc;
    dev = &pdev.dev;
    lpc_snoop = devm_kzalloc(dev, sizeof(*lpc_snoop), GFP_KERNEL);
    if (!lpc_snoop)
    return -ENOMEM;
    np = pdev.dev.parent.of_node;
    if (!of_device_is_compatible(np, "aspeed,ast2400-lpc-v2") &&
    !of_device_is_compatible(np, "aspeed,ast2500-lpc-v2") &&
    !of_device_is_compatible(np, "aspeed,ast2600-lpc-v2")) {
    dev_err(dev, "unsupported LPC device binding\n");
    return -ENODEV;
    }
    lpc_snoop.regmap = syscon_node_to_regmap(np);
    if (IS_ERR(lpc_snoop.regmap))
    return dev_err_probe(dev, PTR_ERR(lpc_snoop.regmap), "Couldn't get regmap\n");
    dev_set_drvdata(&pdev.dev, lpc_snoop);
    lpc_snoop.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(lpc_snoop.clk))
    return dev_err_probe(dev, PTR_ERR(lpc_snoop.clk), "couldn't get clock");
    rc = aspeed_lpc_snoop_config_irq(lpc_snoop, pdev);
    if (rc)
    return rc;
    static_assert(ARRAY_SIZE(channel_cfgs) == ARRAY_SIZE(lpc_snoop.chan),
    "Broken implementation assumption regarding cfg count");
    for (idx = ASPEED_LPC_SNOOP_INDEX_0; idx <= ASPEED_LPC_SNOOP_INDEX_MAX; idx++) {
    u32 port;
    rc = of_property_read_u32_index(dev.of_node, "snoop-ports", idx, &port);
    if (rc)
    break;
    rc = aspeed_lpc_enable_snoop(dev, lpc_snoop, &lpc_snoop.chan[idx],
    &channel_cfgs[idx], port);
    if (rc)
    goto cleanup_channels;
    }
    let mut idx: return = = ASPEED_LPC_SNOOP_INDEX_0 ? -ENODEV : 0;
    cleanup_channels:
    aspeed_lpc_snoop_remove(pdev);
    return rc;
    }
    static const struct aspeed_lpc_snoop_model_data ast2400_model_data = {
    .has_hicrb_ensnp = 0,
    };
    static const struct aspeed_lpc_snoop_model_data ast2500_model_data = {
    .has_hicrb_ensnp = 1,
    };
    static const struct of_device_id aspeed_lpc_snoop_match[] = {
    { .compatible = "aspeed,ast2400-lpc-snoop",
    .data = &ast2400_model_data },
    { .compatible = "aspeed,ast2500-lpc-snoop",
    .data = &ast2500_model_data },
    { .compatible = "aspeed,ast2600-lpc-snoop",
    .data = &ast2500_model_data },
    { },
    };
    MODULE_DEVICE_TABLE(of, aspeed_lpc_snoop_match);
    static struct platform_driver aspeed_lpc_snoop_driver = {
    .driver = {
    .name		= DEVICE_NAME,
    .of_match_table = aspeed_lpc_snoop_match,
    },
    .probe = aspeed_lpc_snoop_probe,
    .remove = aspeed_lpc_snoop_remove,
    };
    module_platform_driver(aspeed_lpc_snoop_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Robert Lippert <rlippert@google.com>");
    MODULE_DESCRIPTION("Linux driver to control Aspeed LPC snoop functionality");
