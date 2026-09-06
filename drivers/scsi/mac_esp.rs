//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/mac_esp.c
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
// mac_esp.c: ESP front-end for Macintosh Quadra systems.
//
// Adapted from jazz_esp.c and the old mac_esp.c.
//
// The pseudo DMA algorithm is based on the one used in NetBSD.
// See sys/arch/mac68k/obio/esp.c for some background information.
//
// Copyright (C) 2007-2008 Finn Thain
//

pub const MAC_ESP_IO_BASE: c_uint = 0x50F00000;

pub const MAC_ESP_REGS_SPACING: c_uint = 0x402;
pub const MAC_ESP_PDMA_REG: c_uint = 0xF9800024;
pub const MAC_ESP_PDMA_REG_SPACING: c_uint = 0x4;
pub const MAC_ESP_PDMA_IO_OFFSET: c_uint = 0x100;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_esp_priv {
    pub esp: *mut esp,
    pub pdma_regs: *mut void __iomem,
    pub pdma_io: *mut void __iomem,
}

    static struct esp *esp_chips[2];
    static DEFINE_SPINLOCK(esp_chips_lock);

    dev_get_drvdata((esp).dev))
#[no_mangle]
pub unsafe extern "C" fn mac_esp_write8(esp: *mut esp, val: u8, reg: c_ulong) {
    static inline void mac_esp_write8(struct esp *esp, u8 val, unsigned long reg)
    {
    nubus_writeb(val, esp.regs + reg * 16);
    }
#[no_mangle]
pub unsafe extern "C" fn mac_esp_read8(esp: *mut esp, reg: c_ulong) -> u8 {
    static inline u8 mac_esp_read8(struct esp *esp, unsigned long reg)
    {
    return nubus_readb(esp.regs + reg * 16);
    }
#[no_mangle]
unsafe extern "C" fn mac_esp_reset_dma(esp: *mut esp) {
    static void mac_esp_reset_dma(struct esp *esp)
    {
// Nothing to do.
    }
#[no_mangle]
unsafe extern "C" fn mac_esp_dma_drain(esp: *mut esp) {
    static void mac_esp_dma_drain(struct esp *esp)
    {
// Nothing to do.
    }
#[no_mangle]
unsafe extern "C" fn mac_esp_dma_invalidate(esp: *mut esp) {
    static void mac_esp_dma_invalidate(struct esp *esp)
    {
// Nothing to do.
    }
#[no_mangle]
unsafe extern "C" fn mac_esp_dma_error(esp: *mut esp) -> c_int {
    static int mac_esp_dma_error(struct esp *esp)
    {
    return esp.send_cmd_error;
    }
#[no_mangle]
pub unsafe extern "C" fn mac_esp_wait_for_empty_fifo(esp: *mut esp) -> c_int {
    static inline int mac_esp_wait_for_empty_fifo(struct esp *esp)
    {
    let mut i: c_int = 500000;
    do {
    if (!(esp_read8(ESP_FFLAGS) & ESP_FF_FBYTES))
    return 0;
    if (esp_read8(ESP_STATUS) & ESP_STAT_INTR)
    return 1;
    udelay(2);
    } while (--i);
    printk(KERN_ERR PFX "FIFO is not empty (sreg %02x)\n",
    esp_read8(ESP_STATUS));
    esp.send_cmd_error = 1;
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn mac_esp_wait_for_dreq(esp: *mut esp) -> c_int {
    static inline int mac_esp_wait_for_dreq(struct esp *esp)
    {
    struct mac_esp_priv *mep = MAC_ESP_GET_PRIV(esp);
    let mut i: c_int = 500000;
    do {
    if (mep.pdma_regs == core::ptr::null_mut()) {
    if (via2_scsi_drq_pending())
    return 0;
    } else {
    if (nubus_readl(mep.pdma_regs) & 0x200)
    return 0;
    }
    if (esp_read8(ESP_STATUS) & ESP_STAT_INTR)
    return 1;
    udelay(2);
    } while (--i);
    printk(KERN_ERR PFX "PDMA timeout (sreg %02x)\n",
    esp_read8(ESP_STATUS));
    esp.send_cmd_error = 1;
    return 1;
    }

    asm volatile ( \
    "       tstw %1                   \n" \
    "       jbeq 20f                  \n" \
    "1:     movew " operands "        \n" \
    "2:     movew " operands "        \n" \
    "3:     movew " operands "        \n" \
    "4:     movew " operands "        \n" \
    "5:     movew " operands "        \n" \
    "6:     movew " operands "        \n" \
    "7:     movew " operands "        \n" \
    "8:     movew " operands "        \n" \
    "9:     movew " operands "        \n" \
    "10:    movew " operands "        \n" \
    "11:    movew " operands "        \n" \
    "12:    movew " operands "        \n" \
    "13:    movew " operands "        \n" \
    "14:    movew " operands "        \n" \
    "15:    movew " operands "        \n" \
    "16:    movew " operands "        \n" \
    "       subqw #1,%1               \n" \
    "       jbne 1b                   \n" \
    "20:    tstw %2                   \n" \
    "       jbeq 30f                  \n" \
    "21:    movew " operands "        \n" \
    "       subqw #1,%2               \n" \
    "       jbne 21b                  \n" \
    "30:    tstw %3                   \n" \
    "       jbeq 40f                  \n" \
    "31:    moveb " operands "        \n" \
    "32:    nop                       \n" \
    "40:                              \n" \
    "                                 \n" \
    "       .section __ex_table,\"a\" \n" \
    "       .align  4                 \n" \
    "       .long   1b,40b            \n" \
    "       .long   2b,40b            \n" \
    "       .long   3b,40b            \n" \
    "       .long   4b,40b            \n" \
    "       .long   5b,40b            \n" \
    "       .long   6b,40b            \n" \
    "       .long   7b,40b            \n" \
    "       .long   8b,40b            \n" \
    "       .long   9b,40b            \n" \
    "       .long  10b,40b            \n" \
    "       .long  11b,40b            \n" \
    "       .long  12b,40b            \n" \
    "       .long  13b,40b            \n" \
    "       .long  14b,40b            \n" \
    "       .long  15b,40b            \n" \
    "       .long  16b,40b            \n" \
    "       .long  21b,40b            \n" \
    "       .long  31b,40b            \n" \
    "       .long  32b,40b            \n" \
    "       .previous                 \n" \
    : "+a" (addr), "+r" (count32), "+r" (count2) \
    : "g" (count1), "a" (mep.pdma_io))
    static void mac_esp_send_pdma_cmd(struct esp *esp, u32 addr, u32 esp_count,
    u32 dma_count, int write, u8 cmd)
    {
    struct mac_esp_priv *mep = MAC_ESP_GET_PRIV(esp);
    esp.send_cmd_error = 0;
    if (!write)
    scsi_esp_cmd(esp, ESP_CMD_FLUSH);
    esp_write8((esp_count >> 0) & 0xFF, ESP_TCLOW);
    esp_write8((esp_count >> 8) & 0xFF, ESP_TCMED);
    scsi_esp_cmd(esp, cmd);
    do {
    let mut count32: c_uint = esp_count >> 5;
    let mut count2: c_uint = (esp_count & 0x1F) >> 1;
    let mut count1: c_uint = esp_count & 1;
    let mut start_addr: c_uint = addr;
    if (mac_esp_wait_for_dreq(esp))
    break;
    if (write) {
    MAC_ESP_PDMA_LOOP("%4@,%0@+");
    esp_count -= addr - start_addr;
    } else {
    unsigned int n;
    MAC_ESP_PDMA_LOOP("%0@+,%4@");
    if (mac_esp_wait_for_empty_fifo(esp))
    break;
    n = (esp_read8(ESP_TCMED) << 8) + esp_read8(ESP_TCLOW);
    addr = start_addr + esp_count - n;
    esp_count = n;
    }
    } while (esp_count);
    }
#[no_mangle]
unsafe extern "C" fn mac_esp_irq_pending(esp: *mut esp) -> c_int {
    static int mac_esp_irq_pending(struct esp *esp)
    {
    if (esp_read8(ESP_STATUS) & ESP_STAT_INTR)
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mac_esp_dma_length_limit(esp: *mut esp, dma_addr: u32, dma_len: u32) -> u32 {
    static u32 mac_esp_dma_length_limit(struct esp *esp, u32 dma_addr, u32 dma_len)
    {
    return dma_len > 0xFFFF ? 0xFFFF : dma_len;
    }
#[no_mangle]
unsafe extern "C" fn mac_scsi_esp_intr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mac_scsi_esp_intr(int irq, void *dev_id)
    {
    int got_intr;
//
// This is an edge triggered IRQ, so we have to be careful to
// avoid missing a transition when it is shared by two ESP devices.
//
    do {
    got_intr = 0;
    if (esp_chips[0] &&
    (mac_esp_read8(esp_chips[0], ESP_STATUS) & ESP_STAT_INTR)) {
    (void)scsi_esp_intr(irq, esp_chips[0]);
    got_intr = 1;
    }
    if (esp_chips[1] &&
    (mac_esp_read8(esp_chips[1], ESP_STATUS) & ESP_STAT_INTR)) {
    (void)scsi_esp_intr(irq, esp_chips[1]);
    got_intr = 1;
    }
    } while (got_intr);
    return IRQ_HANDLED;
    }
    static struct esp_driver_ops mac_esp_ops = {
    .esp_write8       = mac_esp_write8,
    .esp_read8        = mac_esp_read8,
    .irq_pending      = mac_esp_irq_pending,
    .dma_length_limit = mac_esp_dma_length_limit,
    .reset_dma        = mac_esp_reset_dma,
    .dma_drain        = mac_esp_dma_drain,
    .dma_invalidate   = mac_esp_dma_invalidate,
    .send_dma_cmd     = mac_esp_send_pdma_cmd,
    .dma_error        = mac_esp_dma_error,
    };
#[no_mangle]
unsafe extern "C" fn esp_mac_probe(dev: *mut platform_device) -> c_int {
    static int esp_mac_probe(struct platform_device *dev)
    {
    const struct scsi_host_template *tpnt = &scsi_esp_template;
    struct Scsi_Host *host;
    struct esp *esp;
    int err;
    struct mac_esp_priv *mep;
    if (!MACH_IS_MAC)
    return -ENODEV;
    if (dev.id > 1)
    return -ENODEV;
    host = scsi_host_alloc(tpnt, sizeof(struct esp));
    err = -ENOMEM;
    if (!host)
    goto fail;
    host.max_id = 8;
    host.dma_boundary = PAGE_SIZE - 1;
    esp = shost_priv(host);
    esp.host = host;
    esp.dev = &dev.dev;
    esp.command_block = kzalloc(16, GFP_KERNEL);
    if (!esp.command_block)
    goto fail_unlink;
    esp.command_block_dma = (dma_addr_t)esp.command_block;
    esp.scsi_id = 7;
    host.this_id = esp.scsi_id;
    esp.scsi_id_mask = 1 << esp.scsi_id;
    mep = kzalloc_obj(struct mac_esp_priv);
    if (!mep)
    goto fail_free_command_block;
    mep.esp = esp;
    platform_set_drvdata(dev, mep);
    switch (macintosh_config.scsi_type) {
    case MAC_SCSI_QUADRA:
    esp.cfreq     = 16500000;
    esp.regs      = (void __iomem *)MAC_ESP_REGS_QUADRA;
    mep.pdma_io   = esp.regs + MAC_ESP_PDMA_IO_OFFSET;
    mep.pdma_regs = core::ptr::null_mut();
    break;
    case MAC_SCSI_QUADRA2:
    esp.cfreq     = 25000000;
    esp.regs      = (void __iomem *)(MAC_ESP_REGS_QUADRA2 +
    dev.id * MAC_ESP_REGS_SPACING);
    mep.pdma_io   = esp.regs + MAC_ESP_PDMA_IO_OFFSET;
    mep.pdma_regs = (void __iomem *)(MAC_ESP_PDMA_REG +
    dev.id * MAC_ESP_PDMA_REG_SPACING);
    nubus_writel(0x1d1, mep.pdma_regs);
    break;
    case MAC_SCSI_QUADRA3:
// These quadras have a real DMA controller (the PSC) but we
// don't know how to drive it so we must use PIO instead.
//
    esp.cfreq     = 25000000;
    esp.regs      = (void __iomem *)MAC_ESP_REGS_QUADRA3;
    mep.pdma_io   = core::ptr::null_mut();
    mep.pdma_regs = core::ptr::null_mut();
    break;
    }
    esp.fifo_reg = esp.regs + ESP_FDATA * 16;
    esp.ops = &mac_esp_ops;
    esp.flags = ESP_FLAG_NO_DMA_MAP;
    if (mep.pdma_io == core::ptr::null_mut()) {
    printk(KERN_INFO PFX "using PIO for controller %d\n", dev.id);
    esp_write8(0, ESP_TCLOW);
    esp_write8(0, ESP_TCMED);
    esp.flags |= ESP_FLAG_DISABLE_SYNC;
    mac_esp_ops.send_dma_cmd = esp_send_pio_cmd;
    } else {
    printk(KERN_INFO PFX "using PDMA for controller %d\n", dev.id);
    }
    host.irq = IRQ_MAC_SCSI;
// The request_irq() call is intended to succeed for the first device
// and fail for the second device.
//
    err = request_irq(host.irq, mac_scsi_esp_intr, 0, "ESP", core::ptr::null_mut());
    spin_lock(&esp_chips_lock);
    if (err < 0 && esp_chips[!dev.id] == core::ptr::null_mut()) {
    spin_unlock(&esp_chips_lock);
    goto fail_free_priv;
    }
    esp_chips[dev.id] = esp;
    spin_unlock(&esp_chips_lock);
    err = scsi_esp_register(esp);
    if (err)
    goto fail_free_irq;
    return 0;
    fail_free_irq:
    spin_lock(&esp_chips_lock);
    esp_chips[dev.id] = core::ptr::null_mut();
    if (esp_chips[!dev.id] == core::ptr::null_mut()) {
    spin_unlock(&esp_chips_lock);
    free_irq(host.irq, core::ptr::null_mut());
    } else
    spin_unlock(&esp_chips_lock);
    fail_free_priv:
    kfree(mep);
    fail_free_command_block:
    kfree(esp.command_block);
    fail_unlink:
    scsi_host_put(host);
    fail:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn esp_mac_remove(dev: *mut platform_device) {
    static void esp_mac_remove(struct platform_device *dev)
    {
    struct mac_esp_priv *mep = platform_get_drvdata(dev);
    struct esp *esp = mep.esp;
    let mut irq: c_uint = esp.host.irq;
    scsi_esp_unregister(esp);
    spin_lock(&esp_chips_lock);
    esp_chips[dev.id] = core::ptr::null_mut();
    if (esp_chips[!dev.id] == core::ptr::null_mut()) {
    spin_unlock(&esp_chips_lock);
    free_irq(irq, core::ptr::null_mut());
    } else
    spin_unlock(&esp_chips_lock);
    kfree(mep);
    kfree(esp.command_block);
    scsi_host_put(esp.host);
    }
    static struct platform_driver esp_mac_driver = {
    .probe    = esp_mac_probe,
    .remove   = esp_mac_remove,
    .driver   = {
    .name	= DRV_MODULE_NAME,
    },
    };
    module_platform_driver(esp_mac_driver);
    MODULE_DESCRIPTION("Mac ESP SCSI driver");
    MODULE_AUTHOR("Finn Thain");
    MODULE_LICENSE("GPL v2");
    MODULE_VERSION(DRV_VERSION);
    MODULE_ALIAS("platform:" DRV_MODULE_NAME);
