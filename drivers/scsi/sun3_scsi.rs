//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/sun3_scsi.c
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
// Sun3 SCSI stuff by Erik Verbruggen (erik@bigmama.xtdnet.nl)
//
// Sun3 DMA routines added by Sam Creasey (sammy@sammy.net)
//
// VME support added by Sam Creasey
//
// TODO: modify this driver to support multiple Sun3 SCSI VME boards
//
// Adapted from mac_scsinew.c:
//
// Generic Macintosh NCR5380 driver
//
// Copyright 1998, Michael Schmitz <mschmitz@lbl.gov>
//
// derived in part from:
//
// Generic Generic NCR5380 driver
//
// Copyright 1995, Russell King
//

// minimum number of bytes to do dma on
pub const DMA_MIN_SIZE: c_int = 129;
// Definitions for the core NCR5380 driver.

// dma regs start at regbase + 8, directly after the NCR regs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun3_dma_regs {
    pub /: *mut *mut unsigned short dma_addr_hi; / vme only,
    pub /: *mut *mut unsigned short dma_addr_lo; / vme only,
    pub /: *mut *mut unsigned short dma_count_hi; / vme only,
    pub /: *mut *mut unsigned short dma_count_lo; / vme only,
    pub /: *mut *mut unsigned short udc_data; / udc dma data reg (obio only),
    pub /: *mut *mut unsigned short udc_addr; / uda dma addr reg (obio only),
    pub reg,: *mut *mut unsigned short fifo_data; / fifo data,
// holds extra byte on odd dma reads
//
    pub fifo_count: c_ushort,
    pub /: *mut *mut unsigned short csr; / control/status reg,
    pub /: *mut *mut unsigned short bpack_hi; / vme only,
    pub /: *mut *mut unsigned short bpack_lo; / vme only,
    pub /: *mut *mut unsigned short ivect; / vme only,
    pub /: *mut *mut unsigned short fifo_count_hi; / vme only,
}

// ucd chip specific regs - live in dvma space
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun3_udc_regs {
    pub /: *mut *mut unsigned short rsel; / select regs to load,
    pub /: *mut *mut unsigned short addr_hi; / high word of addr,
    pub /: *mut *mut unsigned short addr_lo; / low word,
    pub /: *mut *mut unsigned short count; / words to be xfer'd,
    pub /: *mut *mut unsigned short mode_hi; / high word of channel mode,
    pub /: *mut *mut unsigned short mode_lo; / low word of channel mode,
}

// addresses of the udc registers
pub const UDC_MODE: c_uint = 0x38;
pub const UDC_CSR: c_uint = 0x2e /* command/status */;
pub const UDC_CHN_HI: c_uint = 0x26 /* chain high word */;
pub const UDC_CHN_LO: c_uint = 0x22 /* chain lo word */;
pub const UDC_CURA_HI: c_uint = 0x1a /* cur reg A high */;
pub const UDC_CURA_LO: c_uint = 0x0a /* cur reg A low */;
pub const UDC_CURB_HI: c_uint = 0x12 /* cur reg B high */;
pub const UDC_CURB_LO: c_uint = 0x02 /* cur reg B low */;
pub const UDC_MODE_HI: c_uint = 0x56 /* mode reg high */;
pub const UDC_MODE_LO: c_uint = 0x52 /* mode reg low */;
pub const UDC_COUNT: c_uint = 0x32 /* words to xfer */;
// some udc commands
pub const UDC_RESET: c_int = 0;
pub const UDC_CHN_START: c_uint = 0xa0 /* start chain */;
pub const UDC_INT_ENABLE: c_uint = 0x32 /* channel 1 int on */;
// udc mode words
pub const UDC_MODE_HIWORD: c_uint = 0x40;
pub const UDC_MODE_LSEND: c_uint = 0xc2;
pub const UDC_MODE_LRECV: c_uint = 0xd2;
// udc reg selections
pub const UDC_RSEL_SEND: c_uint = 0x282;
pub const UDC_RSEL_RECV: c_uint = 0x182;
// bits in csr reg
pub const CSR_DMA_ACTIVE: c_uint = 0x8000;
pub const CSR_DMA_CONFLICT: c_uint = 0x4000;
pub const CSR_DMA_BUSERR: c_uint = 0x2000;
pub const CSR_FIFO_EMPTY: c_uint = 0x400 /* fifo flushed? */;
pub const CSR_SDB_INT: c_uint = 0x200 /* sbc interrupt pending */;
pub const CSR_DMA_INT: c_uint = 0x100 /* dma interrupt pending */;
pub const CSR_LEFT: c_uint = 0xc0;
pub const CSR_LEFT_3: c_uint = 0xc0;
pub const CSR_LEFT_2: c_uint = 0x80;
pub const CSR_LEFT_1: c_uint = 0x40;
pub const CSR_PACK_ENABLE: c_uint = 0x20;
pub const CSR_DMA_ENABLE: c_uint = 0x10;
pub const CSR_SEND: c_uint = 0x8 /* 1 = send  0 = recv */;
pub const CSR_FIFO: c_uint = 0x2 /* reset fifo */;
pub const CSR_INTR: c_uint = 0x4 /* interrupt enable */;
pub const CSR_SCSI: c_uint = 0x1;
pub const VME_DATA24: c_uint = 0x3d00;
    extern int sun3_map_test(unsigned long, char *);
    let mut setup_can_queue: static int = -1;
    module_param(setup_can_queue, int, 0);
    let mut setup_cmd_per_lun: static int = -1;
    module_param(setup_cmd_per_lun, int, 0);
    let mut setup_sg_tablesize: static int = -1;
    module_param(setup_sg_tablesize, int, 0);
    let mut setup_hostid: static int = -1;
    module_param(setup_hostid, int, 0);
// ms to wait after hitting dma regs
pub const SUN3_DMA_DELAY: c_int = 10;
// dvma buffer to allocate -- 32k should hopefully be more than sufficient
pub const SUN3_DVMA_BUFSIZE: c_uint = 0xe000;
    static struct scsi_cmnd *sun3_dma_setup_done;
    static volatile struct sun3_dma_regs *dregs;
    static struct sun3_udc_regs *udc_regs;
    static unsigned char *sun3_dma_orig_addr;
    static unsigned long sun3_dma_orig_count;
    static int sun3_dma_active;
    static unsigned long last_residual;

// dma controller register access functions
#[no_mangle]
pub unsafe extern "C" fn sun3_udc_read(reg: c_uchar) -> c_ushort {
    static inline unsigned short sun3_udc_read(unsigned char reg)
    {
    unsigned short ret;
    dregs.udc_addr = UDC_CSR;
    udelay(SUN3_DMA_DELAY);
    ret = dregs.udc_data;
    udelay(SUN3_DMA_DELAY);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn sun3_udc_write(val: c_ushort, reg: c_uchar) {
    static inline void sun3_udc_write(unsigned short val, unsigned char reg)
    {
    dregs.udc_addr = reg;
    udelay(SUN3_DMA_DELAY);
    dregs.udc_data = val;
    udelay(SUN3_DMA_DELAY);
    }

// safe bits for the CSR
pub const CSR_GOOD: c_uint = 0x060f;
#[no_mangle]
unsafe extern "C" fn scsi_sun3_intr(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t scsi_sun3_intr(int irq, void *dev)
    {
    struct Scsi_Host *instance = dev;
    let mut csr: c_ushort = dregs.csr;
    let mut handled: c_int = 0;

    dregs.csr &= ~CSR_DMA_ENABLE;

    if(csr & ~CSR_GOOD) {
    if (csr & CSR_DMA_BUSERR)
    shost_printk(KERN_ERR, instance, "bus error in DMA\n");
    if (csr & CSR_DMA_CONFLICT)
    shost_printk(KERN_ERR, instance, "DMA conflict\n");
    handled = 1;
    }
    if(csr & (CSR_SDB_INT | CSR_DMA_INT)) {
    NCR5380_intr(irq, dev);
    handled = 1;
    }
    return IRQ_RETVAL(handled);
    }
// sun3scsi_dma_setup() -- initialize the dma controller for a read/write
    static int sun3scsi_dma_setup(struct NCR5380_hostdata *hostdata,
    unsigned char *data, int count, int write_flag)
    {
    void *addr;
    if(sun3_dma_orig_addr != core::ptr::null_mut())
    dvma_unmap(sun3_dma_orig_addr);

    addr = (void *)dvma_map_vme((unsigned long) data, count);

    addr = (void *)dvma_map((unsigned long) data, count);

    sun3_dma_orig_addr = addr;
    sun3_dma_orig_count = count;

    dregs.fifo_count = 0;
    sun3_udc_write(UDC_RESET, UDC_CSR);
// reset fifo
    dregs.csr &= ~CSR_FIFO;
    dregs.csr |= CSR_FIFO;

// set direction
    if(write_flag)
    dregs.csr |= CSR_SEND;
    else
    dregs.csr &= ~CSR_SEND;

    dregs.csr |= CSR_PACK_ENABLE;
    dregs.dma_addr_hi = ((unsigned long)addr >> 16);
    dregs.dma_addr_lo = ((unsigned long)addr & 0xffff);
    dregs.dma_count_hi = 0;
    dregs.dma_count_lo = 0;
    dregs.fifo_count_hi = 0;
    dregs.fifo_count = 0;

// byte count for fifo
    dregs.fifo_count = count;
    sun3_udc_write(UDC_RESET, UDC_CSR);
// reset fifo
    dregs.csr &= ~CSR_FIFO;
    dregs.csr |= CSR_FIFO;
    if(dregs.fifo_count != count) {
    shost_printk(KERN_ERR, hostdata.host,
    "FIFO mismatch %04x not %04x\n",
    dregs.fifo_count, (unsigned int) count);
    NCR5380_dprint(NDEBUG_DMA, hostdata.host);
    }
// setup udc
    udc_regs.addr_hi = (((unsigned long)(addr) & 0xff0000) >> 8);
    udc_regs.addr_lo = ((unsigned long)(addr) & 0xffff);
    udc_regs.count = count/2; /* count in words */
    udc_regs.mode_hi = UDC_MODE_HIWORD;
    if(write_flag) {
    if(count & 1)
    udc_regs.count++;
    udc_regs.mode_lo = UDC_MODE_LSEND;
    udc_regs.rsel = UDC_RSEL_SEND;
    } else {
    udc_regs.mode_lo = UDC_MODE_LRECV;
    udc_regs.rsel = UDC_RSEL_RECV;
    }
// announce location of regs block
    sun3_udc_write(((dvma_vtob(udc_regs) & 0xff0000) >> 8),
    UDC_CHN_HI);
    sun3_udc_write((dvma_vtob(udc_regs) & 0xffff), UDC_CHN_LO);
// set dma master on
    sun3_udc_write(0xd, UDC_MODE);
// interrupt enable
    sun3_udc_write(UDC_INT_ENABLE, UDC_CSR);

    return count;
    }
    static int sun3scsi_dma_count(struct NCR5380_hostdata *hostdata,
    unsigned char *data, int count)
    {
    return count;
    }
    static inline int sun3scsi_dma_recv_setup(struct NCR5380_hostdata *hostdata,
    unsigned char *data, int count)
    {
    return sun3scsi_dma_setup(hostdata, data, count, 0);
    }
    static inline int sun3scsi_dma_send_setup(struct NCR5380_hostdata *hostdata,
    unsigned char *data, int count)
    {
    return sun3scsi_dma_setup(hostdata, data, count, 1);
    }
#[no_mangle]
unsafe extern "C" fn sun3scsi_dma_residual(hostdata: *mut NCR5380_hostdata) -> c_int {
    static int sun3scsi_dma_residual(struct NCR5380_hostdata *hostdata)
    {
    return last_residual;
    }
    static int sun3scsi_dma_xfer_len(struct NCR5380_hostdata *hostdata,
    struct scsi_cmnd *cmd)
    {
    let mut wanted_len: c_int = NCR5380_to_ncmd(cmd).this_residual;
    if (wanted_len < DMA_MIN_SIZE || blk_rq_is_passthrough(scsi_cmd_to_rq(cmd)))
    return 0;
    return wanted_len;
    }
#[no_mangle]
pub unsafe extern "C" fn sun3scsi_dma_start(count: c_ulong, data: *mut c_uchar) -> c_int {
    static inline int sun3scsi_dma_start(unsigned long count, unsigned char *data)
    {

    unsigned short csr;
    csr = dregs.csr;
    dregs.dma_count_hi = (sun3_dma_orig_count >> 16);
    dregs.dma_count_lo = (sun3_dma_orig_count & 0xffff);
    dregs.fifo_count_hi = (sun3_dma_orig_count >> 16);
    dregs.fifo_count = (sun3_dma_orig_count & 0xffff);
// if(!(csr & CSR_DMA_ENABLE))
// dregs->csr |= CSR_DMA_ENABLE;
//

    sun3_udc_write(UDC_CHN_START, UDC_CSR);

    return 0;
    }
// clean up after our dma is done
#[no_mangle]
unsafe extern "C" fn sun3scsi_dma_finish(data_dir: enum dma_data_direction) -> c_int {
    static int sun3scsi_dma_finish(enum dma_data_direction data_dir)
    {
    let mut write_flag: bool = data_dir == DMA_TO_DEVICE;
    unsigned short __maybe_unused count;
    unsigned short fifo;
    let mut ret: c_int = 0;
    sun3_dma_active = 0;

    dregs.csr &= ~CSR_DMA_ENABLE;
    fifo = dregs.fifo_count;
    if (write_flag) {
    if ((fifo > 0) && (fifo < sun3_dma_orig_count))
    fifo++;
    }
    last_residual = fifo;
// empty bytes from the fifo which didn't make it
    if ((!write_flag) && (dregs.csr & CSR_LEFT)) {
    unsigned char *vaddr;
    vaddr = (unsigned char *)dvma_vmetov(sun3_dma_orig_addr);
    vaddr += (sun3_dma_orig_count - fifo);
    vaddr--;
    switch (dregs.csr & CSR_LEFT) {
    case CSR_LEFT_3:
// vaddr = (dregs->bpack_lo & 0xff00) >> 8;
    vaddr--;
    fallthrough;
    case CSR_LEFT_2:
// vaddr = (dregs->bpack_hi & 0x00ff);
    vaddr--;
    fallthrough;
    case CSR_LEFT_1:
// vaddr = (dregs->bpack_hi & 0xff00) >> 8;
    break;
    }
    }

// check to empty the fifo on a read
    if(!write_flag) {
    int tmo = 20000; /* .2 sec */
    while(1) {
    if(dregs.csr & CSR_FIFO_EMPTY)
    break;
    if(--tmo <= 0) {
    printk("sun3scsi: fifo failed to empty!\n");
    return 1;
    }
    udelay(10);
    }
    }
    dregs.udc_addr = 0x32;
    udelay(SUN3_DMA_DELAY);
    count = 2 * dregs.udc_data;
    udelay(SUN3_DMA_DELAY);
    fifo = dregs.fifo_count;
    last_residual = fifo;
// empty bytes from the fifo which didn't make it
    if((!write_flag) && (count - fifo) == 2) {
    unsigned short data;
    unsigned char *vaddr;
    data = dregs.fifo_data;
    vaddr = (unsigned char *)dvma_btov(sun3_dma_orig_addr);
    vaddr += (sun3_dma_orig_count - fifo);
    vaddr[-2] = (data & 0xff00) >> 8;
    vaddr[-1] = (data & 0xff);
    }

    dvma_unmap(sun3_dma_orig_addr);
    sun3_dma_orig_addr = core::ptr::null_mut();

    dregs.dma_addr_hi = 0;
    dregs.dma_addr_lo = 0;
    dregs.dma_count_hi = 0;
    dregs.dma_count_lo = 0;
    dregs.fifo_count = 0;
    dregs.fifo_count_hi = 0;
    dregs.csr &= ~CSR_SEND;
// dregs->csr |= CSR_DMA_ENABLE;

    sun3_udc_write(UDC_RESET, UDC_CSR);
    dregs.fifo_count = 0;
    dregs.csr &= ~CSR_SEND;
// reset fifo
    dregs.csr &= ~CSR_FIFO;
    dregs.csr |= CSR_FIFO;

    sun3_dma_setup_done = core::ptr::null_mut();
    return ret;
    }

    static struct scsi_host_template sun3_scsi_template = {
    .module			= THIS_MODULE,
    .proc_name		= DRV_MODULE_NAME,
    .name			= SUN3_SCSI_NAME,
    .info			= sun3scsi_info,
    .queuecommand		= sun3scsi_queue_command,
    .eh_abort_handler	= sun3scsi_abort,
    .eh_host_reset_handler	= sun3scsi_host_reset,
    .can_queue		= 16,
    .this_id		= 7,
    .sg_tablesize		= 1,
    .cmd_per_lun		= 2,
    .dma_boundary		= PAGE_SIZE - 1,
    .cmd_size		= sizeof(struct NCR5380_cmd),
    };
#[no_mangle]
unsafe extern "C" fn sun3_scsi_probe(pdev: *mut platform_device) -> int __init {
    static int __init sun3_scsi_probe(struct platform_device *pdev)
    {
    struct Scsi_Host *instance;
    struct NCR5380_hostdata *hostdata;
    int error;
    struct resource *irq, *mem;
    void __iomem *ioaddr;
    let mut host_flags: c_int = 0;

    int i;

    if (setup_can_queue > 0)
    sun3_scsi_template.can_queue = setup_can_queue;
    if (setup_cmd_per_lun > 0)
    sun3_scsi_template.cmd_per_lun = setup_cmd_per_lun;
    if (setup_sg_tablesize > 0)
    sun3_scsi_template.sg_tablesize = setup_sg_tablesize;
    if (setup_hostid >= 0)
    sun3_scsi_template.this_id = setup_hostid & 7;

    ioaddr = core::ptr::null_mut();
    for (i = 0; i < 2; i++) {
    unsigned char x;
    irq = platform_get_resource(pdev, IORESOURCE_IRQ, i);
    mem = platform_get_resource(pdev, IORESOURCE_MEM, i);
    if (!irq || !mem)
    break;
    ioaddr = sun3_ioremap(mem.start, resource_size(mem),
    SUN3_PAGE_TYPE_VME16);
    dregs = (struct sun3_dma_regs *)(ioaddr + 8);
    if (sun3_map_test((unsigned long)dregs, &x)) {
    unsigned short oldcsr;
    oldcsr = dregs.csr;
    dregs.csr = 0;
    udelay(SUN3_DMA_DELAY);
    if (dregs.csr == 0x1400)
    break;
    dregs.csr = oldcsr;
    }
    iounmap(ioaddr);
    ioaddr = core::ptr::null_mut();
    }
    if (!ioaddr)
    return -ENODEV;

    irq = platform_get_resource(pdev, IORESOURCE_IRQ, 0);
    mem = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!irq || !mem)
    return -ENODEV;
    ioaddr = ioremap(mem.start, resource_size(mem));
    dregs = (struct sun3_dma_regs *)(ioaddr + 8);
    udc_regs = dvma_malloc(sizeof(struct sun3_udc_regs));
    if (!udc_regs) {
    pr_err(PFX "couldn't allocate DVMA memory!\n");
    iounmap(ioaddr);
    return -ENOMEM;
    }

    instance = scsi_host_alloc(&sun3_scsi_template,
    sizeof(struct NCR5380_hostdata));
    if (!instance) {
    error = -ENOMEM;
    goto fail_alloc;
    }
    instance.irq = irq.start;
    hostdata = shost_priv(instance);
    hostdata.base = mem.start;
    hostdata.io = ioaddr;
    error = NCR5380_init(instance, host_flags);
    if (error)
    goto fail_init;
    error = request_irq(instance.irq, scsi_sun3_intr, 0,
    "NCR5380", instance);
    if (error) {
    pr_err(PFX "scsi%d: IRQ %d not free, bailing out\n",
    instance.host_no, instance.irq);
    goto fail_irq;
    }
    dregs.csr = 0;
    udelay(SUN3_DMA_DELAY);
    dregs.csr = CSR_SCSI | CSR_FIFO | CSR_INTR;
    udelay(SUN3_DMA_DELAY);
    dregs.fifo_count = 0;

    dregs.fifo_count_hi = 0;
    dregs.dma_addr_hi = 0;
    dregs.dma_addr_lo = 0;
    dregs.dma_count_hi = 0;
    dregs.dma_count_lo = 0;
    dregs.ivect = VME_DATA24 | (instance.irq & 0xff);

    NCR5380_maybe_reset_bus(instance);
    error = scsi_add_host(instance, core::ptr::null_mut());
    if (error)
    goto fail_host;
    platform_set_drvdata(pdev, instance);
    scsi_scan_host(instance);
    return 0;
    fail_host:
    free_irq(instance.irq, instance);
    fail_irq:
    NCR5380_exit(instance);
    fail_init:
    scsi_host_put(instance);
    fail_alloc:
    if (udc_regs)
    dvma_free(udc_regs);
    iounmap(ioaddr);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn sun3_scsi_remove(pdev: *mut platform_device) -> void __exit {
    static void __exit sun3_scsi_remove(struct platform_device *pdev)
    {
    struct Scsi_Host *instance = platform_get_drvdata(pdev);
    struct NCR5380_hostdata *hostdata = shost_priv(instance);
    void __iomem *ioaddr = hostdata.io;
    scsi_remove_host(instance);
    free_irq(instance.irq, instance);
    NCR5380_exit(instance);
    scsi_host_put(instance);
    if (udc_regs)
    dvma_free(udc_regs);
    iounmap(ioaddr);
    }
//
// sun3_scsi_remove() lives in .exit.text. For drivers registered via
// module_platform_driver_probe() this is ok because they cannot get unbound at
// runtime. So mark the driver struct with __refdata to prevent modpost
// triggering a section mismatch warning.
//
    static struct platform_driver sun3_scsi_driver __refdata = {
    .remove = __exit_p(sun3_scsi_remove),
    .driver = {
    .name	= DRV_MODULE_NAME,
    },
    };
    module_platform_driver_probe(sun3_scsi_driver, sun3_scsi_probe);
    MODULE_ALIAS("platform:" DRV_MODULE_NAME);
    MODULE_DESCRIPTION("Sun3 NCR5380 SCSI controller driver");
    MODULE_LICENSE("GPL");
