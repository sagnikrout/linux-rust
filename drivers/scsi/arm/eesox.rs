//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/arm/eesox.c
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
// linux/drivers/acorn/scsi/eesox.c
//
// Copyright (C) 1997-2005 Russell King
//
// This driver is based on experimentation.  Hence, it may have made
// assumptions about the particular card that I have available, and
// may not be reliable!
//
// Changelog:
// 01-10-1997	RMK		Created, READONLY version
// 15-02-1998	RMK		READ/WRITE version
// added DMA support and hardware definitions
// 14-03-1998	RMK		Updated DMA support
// Added terminator control
// 15-04-1998	RMK		Only do PIO if FAS216 will allow it.
// 27-06-1998	RMK		Changed asm/delay.h to linux/delay.h
// 02-04-2000	RMK	0.0.3	Fixed NO_IRQ/NO_DMA problem, updated for new
// error handling code.
//

pub const EESOX_FAS216_OFFSET: c_uint = 0x3000;
pub const EESOX_FAS216_SHIFT: c_int = 5;
pub const EESOX_DMASTAT: c_uint = 0x2800;
pub const EESOX_STAT_INTR: c_uint = 0x01;
pub const EESOX_STAT_DMA: c_uint = 0x02;
pub const EESOX_CONTROL: c_uint = 0x2800;
pub const EESOX_INTR_ENABLE: c_uint = 0x04;
pub const EESOX_TERM_ENABLE: c_uint = 0x02;
pub const EESOX_RESET: c_uint = 0x01;
pub const EESOX_DMADATA: c_uint = 0x3800;

//
// Use term=0,1,0,0,0 to turn terminators on/off
//
    static int term[MAX_ECARDS] = { 1, 1, 1, 1, 1, 1, 1, 1 };
pub const NR_SG: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eesoxscsi_info {
    pub info: FAS216_Info,
    pub ec: *mut expansion_card,
    pub base: *mut void __iomem,
    pub ctl_port: *mut void __iomem,
    pub control: c_uint,
    pub /: *mut *mut scatterlist sg[NR_SG]; / Scatter DMA list,
}

// Prototype: void eesoxscsi_irqenable(ec, irqnr)
// Purpose  : Enable interrupts on EESOX SCSI card
// Params   : ec    - expansion card structure
// : irqnr - interrupt number
//
    static void
    eesoxscsi_irqenable(struct expansion_card *ec, int irqnr)
    {
    struct eesoxscsi_info *info = (struct eesoxscsi_info *)ec.irq_data;
    info.control |= EESOX_INTR_ENABLE;
    writeb(info.control, info.ctl_port);
    }
// Prototype: void eesoxscsi_irqdisable(ec, irqnr)
// Purpose  : Disable interrupts on EESOX SCSI card
// Params   : ec    - expansion card structure
// : irqnr - interrupt number
//
    static void
    eesoxscsi_irqdisable(struct expansion_card *ec, int irqnr)
    {
    struct eesoxscsi_info *info = (struct eesoxscsi_info *)ec.irq_data;
    info.control &= ~EESOX_INTR_ENABLE;
    writeb(info.control, info.ctl_port);
    }
    static const expansioncard_ops_t eesoxscsi_ops = {
    .irqenable	= eesoxscsi_irqenable,
    .irqdisable	= eesoxscsi_irqdisable,
    };
// Prototype: void eesoxscsi_terminator_ctl(*host, on_off)
// Purpose  : Turn the EESOX SCSI terminators on or off
// Params   : host   - card to turn on/off
// : on_off - !0 to turn on, 0 to turn off
//
    static void
    eesoxscsi_terminator_ctl(struct Scsi_Host *host, int on_off)
    {
    struct eesoxscsi_info *info = (struct eesoxscsi_info *)host.hostdata;
    unsigned long flags;
    spin_lock_irqsave(host.host_lock, flags);
    if (on_off)
    info.control |= EESOX_TERM_ENABLE;
    else
    info.control &= ~EESOX_TERM_ENABLE;
    writeb(info.control, info.ctl_port);
    spin_unlock_irqrestore(host.host_lock, flags);
    }
// Prototype: void eesoxscsi_intr(irq, *dev_id, *regs)
// Purpose  : handle interrupts from EESOX SCSI card
// Params   : irq    - interrupt number
// dev_id - user-defined (Scsi_Host structure)
//
    static irqreturn_t
    eesoxscsi_intr(int irq, void *dev_id)
    {
    struct eesoxscsi_info *info = dev_id;
    return fas216_intr(&info.info);
    }
// Prototype: fasdmatype_t eesoxscsi_dma_setup(host, SCpnt, direction, min_type)
// Purpose  : initialises DMA/PIO
// Params   : host      - host
// SCpnt     - command
// direction - DMA on to/off of card
// min_type  - minimum DMA support that we must have for this transfer
// Returns  : type of transfer to be performed
//
    static fasdmatype_t
    eesoxscsi_dma_setup(struct Scsi_Host *host, struct scsi_pointer *SCp,
    fasdmadir_t direction, fasdmatype_t min_type)
    {
    struct eesoxscsi_info *info = (struct eesoxscsi_info *)host.hostdata;
    struct device *dev = scsi_get_device(host);
    let mut dmach: c_int = info.info.scsi.dma;
    if (dmach != NO_DMA &&
    (min_type == fasdma_real_all || SCp.this_residual >= 512)) {
    int bufs, map_dir, dma_dir;
    bufs = copy_SCp_to_sg(&info.sg[0], SCp, NR_SG);
    if (direction == DMA_OUT) {
    map_dir = DMA_TO_DEVICE;
    dma_dir = DMA_MODE_WRITE;
    } else {
    map_dir = DMA_FROM_DEVICE;
    dma_dir = DMA_MODE_READ;
    }
    dma_map_sg(dev, info.sg, bufs, map_dir);
    disable_dma(dmach);
    set_dma_sg(dmach, info.sg, bufs);
    set_dma_mode(dmach, dma_dir);
    enable_dma(dmach);
    return fasdma_real_all;
    }
//
// We don't do DMA, we only do slow PIO
//
// Some day, we will do Pseudo DMA
//
    return fasdma_pseudo;
    }
#[no_mangle]
unsafe extern "C" fn eesoxscsi_buffer_in(buf: *mut c_void, length: c_int, base: *mut void __iomem) {
    static void eesoxscsi_buffer_in(void *buf, int length, void __iomem *base)
    {
    const void __iomem *reg_fas = base + EESOX_FAS216_OFFSET;
    const void __iomem *reg_dmastat = base + EESOX_DMASTAT;
    const void __iomem *reg_dmadata = base + EESOX_DMADATA;
    let mut mask: register unsigned long = 0xffff;
    do {
    unsigned int status;
//
// Interrupt request?
//
    status = readb(reg_fas + (REG_STAT << EESOX_FAS216_SHIFT));
    if (status & STAT_INT)
    break;
//
// DMA request active?
//
    status = readb(reg_dmastat);
    if (!(status & EESOX_STAT_DMA))
    continue;
//
// Get number of bytes in FIFO
//
    status = readb(reg_fas + (REG_CFIS << EESOX_FAS216_SHIFT)) & CFIS_CF;
    if (status > 16)
    status = 16;
    if (status > length)
    status = length;
//
// Align buffer.
//
    if (((u32)buf) & 2 && status >= 2) {
// (u16 *)buf = readl(reg_dmadata);
    buf += 2;
    status -= 2;
    length -= 2;
    }
    if (status >= 8) {
    unsigned long l1, l2;
    l1 = readl(reg_dmadata) & mask;
    l1 |= readl(reg_dmadata) << 16;
    l2 = readl(reg_dmadata) & mask;
    l2 |= readl(reg_dmadata) << 16;
// (u32 *)buf = l1;
    buf += 4;
// (u32 *)buf = l2;
    buf += 4;
    length -= 8;
    continue;
    }
    if (status >= 4) {
    unsigned long l1;
    l1 = readl(reg_dmadata) & mask;
    l1 |= readl(reg_dmadata) << 16;
// (u32 *)buf = l1;
    buf += 4;
    length -= 4;
    continue;
    }
    if (status >= 2) {
// (u16 *)buf = readl(reg_dmadata);
    buf += 2;
    length -= 2;
    }
    } while (length);
    }
#[no_mangle]
unsafe extern "C" fn eesoxscsi_buffer_out(buf: *mut c_void, length: c_int, base: *mut void __iomem) {
    static void eesoxscsi_buffer_out(void *buf, int length, void __iomem *base)
    {
    const void __iomem *reg_fas = base + EESOX_FAS216_OFFSET;
    const void __iomem *reg_dmastat = base + EESOX_DMASTAT;
    void __iomem *reg_dmadata = base + EESOX_DMADATA;
    do {
    unsigned int status;
//
// Interrupt request?
//
    status = readb(reg_fas + (REG_STAT << EESOX_FAS216_SHIFT));
    if (status & STAT_INT)
    break;
//
// DMA request active?
//
    status = readb(reg_dmastat);
    if (!(status & EESOX_STAT_DMA))
    continue;
//
// Get number of bytes in FIFO
//
    status = readb(reg_fas + (REG_CFIS << EESOX_FAS216_SHIFT)) & CFIS_CF;
    if (status > 16)
    status = 16;
    status = 16 - status;
    if (status > length)
    status = length;
    status &= ~1;
//
// Align buffer.
//
    if (((u32)buf) & 2 && status >= 2) {
    writel(*(u16 *)buf << 16, reg_dmadata);
    buf += 2;
    status -= 2;
    length -= 2;
    }
    if (status >= 8) {
    unsigned long l1, l2;
    l1 = *(u32 *)buf;
    buf += 4;
    l2 = *(u32 *)buf;
    buf += 4;
    writel(l1 << 16, reg_dmadata);
    writel(l1, reg_dmadata);
    writel(l2 << 16, reg_dmadata);
    writel(l2, reg_dmadata);
    length -= 8;
    continue;
    }
    if (status >= 4) {
    unsigned long l1;
    l1 = *(u32 *)buf;
    buf += 4;
    writel(l1 << 16, reg_dmadata);
    writel(l1, reg_dmadata);
    length -= 4;
    continue;
    }
    if (status >= 2) {
    writel(*(u16 *)buf << 16, reg_dmadata);
    buf += 2;
    length -= 2;
    }
    } while (length);
    }
    static void
    eesoxscsi_dma_pseudo(struct Scsi_Host *host, struct scsi_pointer *SCp,
    fasdmadir_t dir, int transfer_size)
    {
    struct eesoxscsi_info *info = (struct eesoxscsi_info *)host.hostdata;
    if (dir == DMA_IN) {
    eesoxscsi_buffer_in(SCp.ptr, SCp.this_residual, info.base);
    } else {
    eesoxscsi_buffer_out(SCp.ptr, SCp.this_residual, info.base);
    }
    }
// Prototype: int eesoxscsi_dma_stop(host, SCpnt)
// Purpose  : stops DMA/PIO
// Params   : host  - host
// SCpnt - command
//
    static void
    eesoxscsi_dma_stop(struct Scsi_Host *host, struct scsi_pointer *SCp)
    {
    struct eesoxscsi_info *info = (struct eesoxscsi_info *)host.hostdata;
    if (info.info.scsi.dma != NO_DMA)
    disable_dma(info.info.scsi.dma);
    }
// Prototype: const char *eesoxscsi_info(struct Scsi_Host * host)
// Purpose  : returns a descriptive string about this interface,
// Params   : host - driver host structure to return info for.
// Returns  : pointer to a static buffer containing null terminated string.
//
    static const char *eesoxscsi_info(struct Scsi_Host *host)
    {
    struct eesoxscsi_info *info = (struct eesoxscsi_info *)host.hostdata;
    static char string[150];
    sprintf(string, "%s (%s) in slot %d v%s terminators o%s",
    host.hostt.name, info.info.scsi.type, info.ec.slot_no,
    VERSION, info.control & EESOX_TERM_ENABLE ? "n" : "ff");
    return string;
    }
// Prototype: int eesoxscsi_set_proc_info(struct Scsi_Host *host, char *buffer, int length)
// Purpose  : Set a driver specific function
// Params   : host   - host to setup
// : buffer - buffer containing string describing operation
// : length - length of string
// Returns  : -EINVAL, or 0
//
    static int
    eesoxscsi_set_proc_info(struct Scsi_Host *host, char *buffer, int length)
    {
    let mut ret: c_int = length;
    if (length >= 9 && strncmp(buffer, "EESOXSCSI", 9) == 0) {
    buffer += 9;
    length -= 9;
    if (length >= 5 && strncmp(buffer, "term=", 5) == 0) {
    if (buffer[5] == '1')
    eesoxscsi_terminator_ctl(host, 1);
#[no_mangle]
pub unsafe extern "C" fn if('0': buffer[5] ==) -> else {
    else if (buffer[5] == '0')
    eesoxscsi_terminator_ctl(host, 0);
    else
    ret = -EINVAL;
    } else
    ret = -EINVAL;
    } else
    ret = -EINVAL;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn eesoxscsi_show_info(m: *mut seq_file, host: *mut Scsi_Host) -> c_int {
    static int eesoxscsi_show_info(struct seq_file *m, struct Scsi_Host *host)
    {
    struct eesoxscsi_info *info;
    info = (struct eesoxscsi_info *)host.hostdata;
    seq_printf(m, "EESOX SCSI driver v%s\n", VERSION);
    fas216_print_host(&info.info, m);
    seq_printf(m, "Term    : o%s\n",
    info.control & EESOX_TERM_ENABLE ? "n" : "ff");
    fas216_print_stats(&info.info, m);
    fas216_print_devices(&info.info, m);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn eesoxscsi_show_term(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize {
    static ssize_t eesoxscsi_show_term(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct expansion_card *ec = ECARD_DEV(dev);
    struct Scsi_Host *host = ecard_get_drvdata(ec);
    struct eesoxscsi_info *info = (struct eesoxscsi_info *)host.hostdata;
    return sprintf(buf, "%d\n", info.control & EESOX_TERM_ENABLE ? 1 : 0);
    }
#[no_mangle]
unsafe extern "C" fn eesoxscsi_store_term(dev: *mut device, attr: *mut device_attribute, buf: *const c_char, len: usize) -> isize {
    static ssize_t eesoxscsi_store_term(struct device *dev, struct device_attribute *attr, const char *buf, size_t len)
    {
    struct expansion_card *ec = ECARD_DEV(dev);
    struct Scsi_Host *host = ecard_get_drvdata(ec);
    struct eesoxscsi_info *info = (struct eesoxscsi_info *)host.hostdata;
    unsigned long flags;
    if (len > 1) {
    spin_lock_irqsave(host.host_lock, flags);
    if (buf[0] != '0') {
    info.control |= EESOX_TERM_ENABLE;
    } else {
    info.control &= ~EESOX_TERM_ENABLE;
    }
    writeb(info.control, info.ctl_port);
    spin_unlock_irqrestore(host.host_lock, flags);
    }
    return len;
    }
    static DEVICE_ATTR(bus_term, S_IRUGO | S_IWUSR,
    eesoxscsi_show_term, eesoxscsi_store_term);
    static const struct scsi_host_template eesox_template = {
    .module				= THIS_MODULE,
    .show_info			= eesoxscsi_show_info,
    .write_info			= eesoxscsi_set_proc_info,
    .name				= "EESOX SCSI",
    .info				= eesoxscsi_info,
    .queuecommand			= fas216_queue_command,
    .eh_host_reset_handler		= fas216_eh_host_reset,
    .eh_bus_reset_handler		= fas216_eh_bus_reset,
    .eh_device_reset_handler	= fas216_eh_device_reset,
    .eh_abort_handler		= fas216_eh_abort,
    .cmd_size			= sizeof(struct fas216_cmd_priv),
    .can_queue			= 1,
    .this_id			= 7,
    .sg_tablesize			= SG_MAX_SEGMENTS,
    .dma_boundary			= IOMD_DMA_BOUNDARY,
    .proc_name			= "eesox",
    };
#[no_mangle]
unsafe extern "C" fn eesoxscsi_probe(ec: *mut expansion_card, id: *const ecard_id) -> c_int {
    static int eesoxscsi_probe(struct expansion_card *ec, const struct ecard_id *id)
    {
    struct Scsi_Host *host;
    struct eesoxscsi_info *info;
    void __iomem *base;
    int ret;
    ret = ecard_request_resources(ec);
    if (ret)
    goto out;
    base = ecardm_iomap(ec, ECARD_RES_IOCFAST, 0, 0);
    if (!base) {
    ret = -ENOMEM;
    goto out_region;
    }
    host = scsi_host_alloc(&eesox_template,
    sizeof(struct eesoxscsi_info));
    if (!host) {
    ret = -ENOMEM;
    goto out_region;
    }
    ecard_set_drvdata(ec, host);
    info = (struct eesoxscsi_info *)host.hostdata;
    info.ec	= ec;
    info.base	= base;
    info.ctl_port	= base + EESOX_CONTROL;
    info.control	= term[ec.slot_no] ? EESOX_TERM_ENABLE : 0;
    writeb(info.control, info.ctl_port);
    info.info.scsi.io_base		= base + EESOX_FAS216_OFFSET;
    info.info.scsi.io_shift	= EESOX_FAS216_SHIFT;
    info.info.scsi.irq		= ec.irq;
    info.info.scsi.dma		= ec.dma;
    info.info.ifcfg.clockrate	= 40; /* MHz */
    info.info.ifcfg.select_timeout	= 255;
    info.info.ifcfg.asyncperiod	= 200; /* ns */
    info.info.ifcfg.sync_max_depth	= 7;
    info.info.ifcfg.cntl3		= CNTL3_FASTSCSI | CNTL3_FASTCLK;
    info.info.ifcfg.disconnect_ok	= 1;
    info.info.ifcfg.wide_max_size	= 0;
    info.info.ifcfg.capabilities	= FASCAP_PSEUDODMA;
    info.info.dma.setup		= eesoxscsi_dma_setup;
    info.info.dma.pseudo		= eesoxscsi_dma_pseudo;
    info.info.dma.stop		= eesoxscsi_dma_stop;
    ec.irqaddr	= base + EESOX_DMASTAT;
    ec.irqmask	= EESOX_STAT_INTR;
    ecard_setirq(ec, &eesoxscsi_ops, info);
    device_create_file(&ec.dev, &dev_attr_bus_term);
    ret = fas216_init(host);
    if (ret)
    goto out_free;
    ret = request_irq(ec.irq, eesoxscsi_intr, 0, "eesoxscsi", info);
    if (ret) {
    printk("scsi%d: IRQ%d not free: %d\n",
    host.host_no, ec.irq, ret);
    goto out_remove;
    }
    if (info.info.scsi.dma != NO_DMA) {
    if (request_dma(info.info.scsi.dma, "eesox")) {
    printk("scsi%d: DMA%d not free, DMA disabled\n",
    host.host_no, info.info.scsi.dma);
    info.info.scsi.dma = NO_DMA;
    } else {
    set_dma_speed(info.info.scsi.dma, 180);
    info.info.ifcfg.capabilities |= FASCAP_DMA;
    info.info.ifcfg.cntl3 |= CNTL3_BS8;
    }
    }
    ret = fas216_add(host, &ec.dev);
    if (ret == 0)
    goto out;
    if (info.info.scsi.dma != NO_DMA)
    free_dma(info.info.scsi.dma);
    free_irq(ec.irq, info);
    out_remove:
    fas216_remove(host);
    out_free:
    device_remove_file(&ec.dev, &dev_attr_bus_term);
    scsi_host_put(host);
    out_region:
    ecard_release_resources(ec);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn eesoxscsi_remove(ec: *mut expansion_card) {
    static void eesoxscsi_remove(struct expansion_card *ec)
    {
    struct Scsi_Host *host = ecard_get_drvdata(ec);
    struct eesoxscsi_info *info = (struct eesoxscsi_info *)host.hostdata;
    ecard_set_drvdata(ec, core::ptr::null_mut());
    fas216_remove(host);
    if (info.info.scsi.dma != NO_DMA)
    free_dma(info.info.scsi.dma);
    free_irq(ec.irq, info);
    device_remove_file(&ec.dev, &dev_attr_bus_term);
    fas216_release(host);
    scsi_host_put(host);
    ecard_release_resources(ec);
    }
    static const struct ecard_id eesoxscsi_cids[] = {
    { MANU_EESOX, PROD_EESOX_SCSI2 },
    { 0xffff, 0xffff },
    };
    static struct ecard_driver eesoxscsi_driver = {
    .probe		= eesoxscsi_probe,
    .remove		= eesoxscsi_remove,
    .id_table	= eesoxscsi_cids,
    .drv = {
    .name		= "eesoxscsi",
    },
    };
#[no_mangle]
unsafe extern "C" fn eesox_init() -> int __init {
    static int __init eesox_init(void)
    {
    return ecard_register_driver(&eesoxscsi_driver);
    }
#[no_mangle]
unsafe extern "C" fn eesox_exit() -> void __exit {
    static void __exit eesox_exit(void)
    {
    ecard_remove_driver(&eesoxscsi_driver);
    }
    module_init(eesox_init);
    module_exit(eesox_exit);
    MODULE_AUTHOR("Russell King");
    MODULE_DESCRIPTION("EESOX 'Fast' SCSI driver for Acorn machines");
    module_param_array(term, int, core::ptr::null_mut(), 0);
    MODULE_PARM_DESC(term, "SCSI bus termination");
    MODULE_LICENSE("GPL");
