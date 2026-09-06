//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/arm/powertec.c
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
// linux/drivers/acorn/scsi/powertec.c
//
// Copyright (C) 1997-2005 Russell King
//

pub const POWERTEC_FAS216_OFFSET: c_uint = 0x3000;
pub const POWERTEC_FAS216_SHIFT: c_int = 6;
pub const POWERTEC_INTR_STATUS: c_uint = 0x2000;
pub const POWERTEC_INTR_BIT: c_uint = 0x80;
pub const POWERTEC_RESET_CONTROL: c_uint = 0x1018;
pub const POWERTEC_RESET_BIT: c_int = 1;
pub const POWERTEC_TERM_CONTROL: c_uint = 0x2018;
pub const POWERTEC_TERM_ENABLE: c_int = 1;
pub const POWERTEC_INTR_CONTROL: c_uint = 0x101c;
pub const POWERTEC_INTR_ENABLE: c_int = 1;
pub const POWERTEC_INTR_DISABLE: c_int = 0;

//
// Use term=0,1,0,0,0 to turn terminators on/off.
// One entry per slot.
//
    static int term[MAX_ECARDS] = { 1, 1, 1, 1, 1, 1, 1, 1 };
pub const NR_SG: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct powertec_info {
    pub info: FAS216_Info,
    pub ec: *mut expansion_card,
    pub base: *mut void __iomem,
    pub term_ctl: c_uint,
    pub sg: [scatterlist; NR_SG],
}

// Prototype: void powertecscsi_irqenable(ec, irqnr)
// Purpose  : Enable interrupts on Powertec SCSI card
// Params   : ec    - expansion card structure
// : irqnr - interrupt number
//
    static void
    powertecscsi_irqenable(struct expansion_card *ec, int irqnr)
    {
    struct powertec_info *info = ec.irq_data;
    writeb(POWERTEC_INTR_ENABLE, info.base + POWERTEC_INTR_CONTROL);
    }
// Prototype: void powertecscsi_irqdisable(ec, irqnr)
// Purpose  : Disable interrupts on Powertec SCSI card
// Params   : ec    - expansion card structure
// : irqnr - interrupt number
//
    static void
    powertecscsi_irqdisable(struct expansion_card *ec, int irqnr)
    {
    struct powertec_info *info = ec.irq_data;
    writeb(POWERTEC_INTR_DISABLE, info.base + POWERTEC_INTR_CONTROL);
    }
    static const expansioncard_ops_t powertecscsi_ops = {
    .irqenable	= powertecscsi_irqenable,
    .irqdisable	= powertecscsi_irqdisable,
    };
// Prototype: void powertecscsi_terminator_ctl(host, on_off)
// Purpose  : Turn the Powertec SCSI terminators on or off
// Params   : host   - card to turn on/off
// : on_off - !0 to turn on, 0 to turn off
//
    static void
    powertecscsi_terminator_ctl(struct Scsi_Host *host, int on_off)
    {
    struct powertec_info *info = (struct powertec_info *)host.hostdata;
    info.term_ctl = on_off ? POWERTEC_TERM_ENABLE : 0;
    writeb(info.term_ctl, info.base + POWERTEC_TERM_CONTROL);
    }
// Prototype: void powertecscsi_intr(irq, *dev_id, *regs)
// Purpose  : handle interrupts from Powertec SCSI card
// Params   : irq    - interrupt number
// dev_id - user-defined (Scsi_Host structure)
//
#[no_mangle]
unsafe extern "C" fn powertecscsi_intr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t powertecscsi_intr(int irq, void *dev_id)
    {
    struct powertec_info *info = dev_id;
    return fas216_intr(&info.info);
    }
// Prototype: fasdmatype_t powertecscsi_dma_setup(host, SCpnt, direction, min_type)
// Purpose  : initialises DMA/PIO
// Params   : host      - host
// SCpnt     - command
// direction - DMA on to/off of card
// min_type  - minimum DMA support that we must have for this transfer
// Returns  : type of transfer to be performed
//
    static fasdmatype_t
    powertecscsi_dma_setup(struct Scsi_Host *host, struct scsi_pointer *SCp,
    fasdmadir_t direction, fasdmatype_t min_type)
    {
    struct powertec_info *info = (struct powertec_info *)host.hostdata;
    struct device *dev = scsi_get_device(host);
    let mut dmach: c_int = info.info.scsi.dma;
    if (info.info.ifcfg.capabilities & FASCAP_DMA &&
    min_type == fasdma_real_all) {
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
// If we're not doing DMA,
// we'll do slow PIO
//
    return fasdma_pio;
    }
// Prototype: int powertecscsi_dma_stop(host, SCpnt)
// Purpose  : stops DMA/PIO
// Params   : host  - host
// SCpnt - command
//
    static void
    powertecscsi_dma_stop(struct Scsi_Host *host, struct scsi_pointer *SCp)
    {
    struct powertec_info *info = (struct powertec_info *)host.hostdata;
    if (info.info.scsi.dma != NO_DMA)
    disable_dma(info.info.scsi.dma);
    }
// Prototype: const char *powertecscsi_info(struct Scsi_Host * host)
// Purpose  : returns a descriptive string about this interface,
// Params   : host - driver host structure to return info for.
// Returns  : pointer to a static buffer containing null terminated string.
//
    static const char *powertecscsi_info(struct Scsi_Host *host)
    {
    struct powertec_info *info = (struct powertec_info *)host.hostdata;
    static char string[150];
    sprintf(string, "%s (%s) in slot %d v%s terminators o%s",
    host.hostt.name, info.info.scsi.type, info.ec.slot_no,
    VERSION, info.term_ctl ? "n" : "ff");
    return string;
    }
// Prototype: int powertecscsi_set_proc_info(struct Scsi_Host *host, char *buffer, int length)
// Purpose  : Set a driver specific function
// Params   : host   - host to setup
// : buffer - buffer containing string describing operation
// : length - length of string
// Returns  : -EINVAL, or 0
//
    static int
    powertecscsi_set_proc_info(struct Scsi_Host *host, char *buffer, int length)
    {
    let mut ret: c_int = length;
    if (length >= 12 && strncmp(buffer, "POWERTECSCSI", 12) == 0) {
    buffer += 12;
    length -= 12;
    if (length >= 5 && strncmp(buffer, "term=", 5) == 0) {
    if (buffer[5] == '1')
    powertecscsi_terminator_ctl(host, 1);
#[no_mangle]
pub unsafe extern "C" fn if('0': buffer[5] ==) -> else {
    else if (buffer[5] == '0')
    powertecscsi_terminator_ctl(host, 0);
    else
    ret = -EINVAL;
    } else
    ret = -EINVAL;
    } else
    ret = -EINVAL;
    return ret;
    }
// Prototype: int powertecscsi_proc_info(char *buffer, char **start, off_t offset,
// int length, int host_no, int inout)
// Purpose  : Return information about the driver to a user process accessing
// the /proc filesystem.
// Params   : buffer  - a buffer to write information to
// start   - a pointer into this buffer set by this routine to the start
// of the required information.
// offset  - offset into information that we have read up to.
// length  - length of buffer
// inout   - 0 for reading, 1 for writing.
// Returns  : length of data written to buffer.
//
#[no_mangle]
unsafe extern "C" fn powertecscsi_show_info(m: *mut seq_file, host: *mut Scsi_Host) -> c_int {
    static int powertecscsi_show_info(struct seq_file *m, struct Scsi_Host *host)
    {
    struct powertec_info *info;
    info = (struct powertec_info *)host.hostdata;
    seq_printf(m, "PowerTec SCSI driver v%s\n", VERSION);
    fas216_print_host(&info.info, m);
    seq_printf(m, "Term    : o%s\n",
    info.term_ctl ? "n" : "ff");
    fas216_print_stats(&info.info, m);
    fas216_print_devices(&info.info, m);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn powertecscsi_show_term(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize {
    static ssize_t powertecscsi_show_term(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct expansion_card *ec = ECARD_DEV(dev);
    struct Scsi_Host *host = ecard_get_drvdata(ec);
    struct powertec_info *info = (struct powertec_info *)host.hostdata;
    return sprintf(buf, "%d\n", info.term_ctl ? 1 : 0);
    }
    static ssize_t
    powertecscsi_store_term(struct device *dev, struct device_attribute *attr, const char *buf, size_t len)
    {
    struct expansion_card *ec = ECARD_DEV(dev);
    struct Scsi_Host *host = ecard_get_drvdata(ec);
    if (len > 1)
    powertecscsi_terminator_ctl(host, buf[0] != '0');
    return len;
    }
    static DEVICE_ATTR(bus_term, S_IRUGO | S_IWUSR,
    powertecscsi_show_term, powertecscsi_store_term);
    static const struct scsi_host_template powertecscsi_template = {
    .module				= THIS_MODULE,
    .show_info			= powertecscsi_show_info,
    .write_info			= powertecscsi_set_proc_info,
    .name				= "PowerTec SCSI",
    .info				= powertecscsi_info,
    .queuecommand			= fas216_queue_command,
    .eh_host_reset_handler		= fas216_eh_host_reset,
    .eh_bus_reset_handler		= fas216_eh_bus_reset,
    .eh_device_reset_handler	= fas216_eh_device_reset,
    .eh_abort_handler		= fas216_eh_abort,
    .cmd_size			= sizeof(struct fas216_cmd_priv),
    .can_queue			= 8,
    .this_id			= 7,
    .sg_tablesize			= SG_MAX_SEGMENTS,
    .dma_boundary			= IOMD_DMA_BOUNDARY,
    .cmd_per_lun			= 2,
    .proc_name			= "powertec",
    };
    static int powertecscsi_probe(struct expansion_card *ec,
    const struct ecard_id *id)
    {
    struct Scsi_Host *host;
    struct powertec_info *info;
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
    host = scsi_host_alloc(&powertecscsi_template,
    sizeof (struct powertec_info));
    if (!host) {
    ret = -ENOMEM;
    goto out_region;
    }
    ecard_set_drvdata(ec, host);
    info = (struct powertec_info *)host.hostdata;
    info.base = base;
    powertecscsi_terminator_ctl(host, term[ec.slot_no]);
    info.ec = ec;
    info.info.scsi.io_base		= base + POWERTEC_FAS216_OFFSET;
    info.info.scsi.io_shift	= POWERTEC_FAS216_SHIFT;
    info.info.scsi.irq		= ec.irq;
    info.info.scsi.dma		= ec.dma;
    info.info.ifcfg.clockrate	= 40; /* MHz */
    info.info.ifcfg.select_timeout	= 255;
    info.info.ifcfg.asyncperiod	= 200; /* ns */
    info.info.ifcfg.sync_max_depth	= 7;
    info.info.ifcfg.cntl3		= CNTL3_BS8 | CNTL3_FASTSCSI | CNTL3_FASTCLK;
    info.info.ifcfg.disconnect_ok	= 1;
    info.info.ifcfg.wide_max_size	= 0;
    info.info.ifcfg.capabilities	= 0;
    info.info.dma.setup		= powertecscsi_dma_setup;
    info.info.dma.pseudo		= core::ptr::null_mut();
    info.info.dma.stop		= powertecscsi_dma_stop;
    ec.irqaddr	= base + POWERTEC_INTR_STATUS;
    ec.irqmask	= POWERTEC_INTR_BIT;
    ecard_setirq(ec, &powertecscsi_ops, info);
    device_create_file(&ec.dev, &dev_attr_bus_term);
    ret = fas216_init(host);
    if (ret)
    goto out_free;
    ret = request_irq(ec.irq, powertecscsi_intr,
    0, "powertec", info);
    if (ret) {
    printk("scsi%d: IRQ%d not free: %d\n",
    host.host_no, ec.irq, ret);
    goto out_release;
    }
    if (info.info.scsi.dma != NO_DMA) {
    if (request_dma(info.info.scsi.dma, "powertec")) {
    printk("scsi%d: DMA%d not free, using PIO\n",
    host.host_no, info.info.scsi.dma);
    info.info.scsi.dma = NO_DMA;
    } else {
    set_dma_speed(info.info.scsi.dma, 180);
    info.info.ifcfg.capabilities |= FASCAP_DMA;
    }
    }
    ret = fas216_add(host, &ec.dev);
    if (ret == 0)
    goto out;
    if (info.info.scsi.dma != NO_DMA)
    free_dma(info.info.scsi.dma);
    free_irq(ec.irq, info);
    out_release:
    fas216_release(host);
    out_free:
    device_remove_file(&ec.dev, &dev_attr_bus_term);
    scsi_host_put(host);
    out_region:
    ecard_release_resources(ec);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn powertecscsi_remove(ec: *mut expansion_card) {
    static void powertecscsi_remove(struct expansion_card *ec)
    {
    struct Scsi_Host *host = ecard_get_drvdata(ec);
    struct powertec_info *info = (struct powertec_info *)host.hostdata;
    ecard_set_drvdata(ec, core::ptr::null_mut());
    fas216_remove(host);
    device_remove_file(&ec.dev, &dev_attr_bus_term);
    if (info.info.scsi.dma != NO_DMA)
    free_dma(info.info.scsi.dma);
    free_irq(ec.irq, info);
    fas216_release(host);
    scsi_host_put(host);
    ecard_release_resources(ec);
    }
    static const struct ecard_id powertecscsi_cids[] = {
    { MANU_ALSYSTEMS, PROD_ALSYS_SCSIATAPI },
    { 0xffff, 0xffff },
    };
    static struct ecard_driver powertecscsi_driver = {
    .probe		= powertecscsi_probe,
    .remove		= powertecscsi_remove,
    .id_table	= powertecscsi_cids,
    .drv = {
    .name		= "powertecscsi",
    },
    };
#[no_mangle]
unsafe extern "C" fn powertecscsi_init() -> int __init {
    static int __init powertecscsi_init(void)
    {
    return ecard_register_driver(&powertecscsi_driver);
    }
#[no_mangle]
unsafe extern "C" fn powertecscsi_exit() -> void __exit {
    static void __exit powertecscsi_exit(void)
    {
    ecard_remove_driver(&powertecscsi_driver);
    }
    module_init(powertecscsi_init);
    module_exit(powertecscsi_exit);
    MODULE_AUTHOR("Russell King");
    MODULE_DESCRIPTION("Powertec SCSI driver");
    module_param_array(term, int, core::ptr::null_mut(), 0);
    MODULE_PARM_DESC(term, "SCSI bus termination");
    MODULE_LICENSE("GPL");
