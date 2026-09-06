//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/sgiwd93.c
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
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// Copyright (C) 1996 David S. Miller (davem@davemloft.net)
// Copyright (C) 1999 Andrew R. Baker (andrewb@uab.edu)
// Copyright (C) 2001 Florian Lohoff (flo@rfc822.org)
// Copyright (C) 2003, 07 Ralf Baechle (ralf@linux-mips.org)
//
// (In all truth, Jed Schimmel wrote all this code.)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip22_hostdata {
    pub wh: WD33C93_hostdata,
    pub dma: dma_addr_t,
    pub cpu: *mut c_void,
    pub dev: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpc_chunk {
    pub desc: hpc_dma_desc,
    pub /: *mut *mut u32 _padding; / align to quadword boundary,
}

// space for hpc dma descriptors

#[no_mangle]
unsafe extern "C" fn sgiwd93_intr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t sgiwd93_intr(int irq, void *dev_id)
    {
    let mut host: *mut Scsi_Host = dev_id;
    unsigned long flags;
    spin_lock_irqsave(host.host_lock, flags);
    wd33c93_intr(host);
    spin_unlock_irqrestore(host.host_lock, flags);
    return IRQ_HANDLED;
    }
    static inline
#[no_mangle]
pub unsafe extern "C" fn fill_hpc_entries(hd: *mut ip22_hostdata, cmd: *mut scsi_cmnd, din: c_int) {
    void fill_hpc_entries(struct ip22_hostdata *hd, struct scsi_cmnd *cmd, int din)
    {
    struct scsi_pointer *scsi_pointer = WD33C93_scsi_pointer(cmd);
    let mut len: c_ulong = scsi_pointer.this_residual;
    void *addr = scsi_pointer.ptr;
    dma_addr_t physaddr;
    unsigned long count;
    struct hpc_chunk *hcp;
    physaddr = dma_map_single(hd.dev, addr, len, DMA_DIR(din));
    scsi_pointer.dma_handle = physaddr;
    hcp = hd.cpu;
    while (len) {
//
// even cntinfo could be up to 16383, without
// magic only 8192 works correctly
//
    count = len > 8192 ? 8192 : len;
    hcp.desc.pbuf = physaddr;
    hcp.desc.cntinfo = count;
    hcp++;
    len -= count;
    physaddr += count;
    }
//
// To make sure, if we trip an HPC bug, that we transfer every single
// byte, we tag on an extra zero length dma descriptor at the end of
// the chain.
//
    hcp.desc.pbuf = 0;
    hcp.desc.cntinfo = HPCDMA_EOX;
    dma_sync_single_for_device(hd.dev, hd.dma,
    (unsigned long)(hcp + 1) - (unsigned long)hd.cpu,
    DMA_TO_DEVICE);
    }
#[no_mangle]
unsafe extern "C" fn dma_setup(cmd: *mut scsi_cmnd, datainp: c_int) -> c_int {
    static int dma_setup(struct scsi_cmnd *cmd, int datainp)
    {
    struct scsi_pointer *scsi_pointer = WD33C93_scsi_pointer(cmd);
    struct ip22_hostdata *hdata = host_to_hostdata(cmd.device.host);
    struct hpc3_scsiregs *hregs =
    (struct hpc3_scsiregs *) cmd.device.host.base;
    pr_debug("dma_setup: datainp<%d> hcp<%p> ", datainp, hdata.cpu);
    hdata.wh.dma_dir = datainp;
//
// wd33c93 shouldn't pass us bogus dma_setups, but it does:-(  The
// other wd33c93 drivers deal with it the same way (which isn't that
// obvious).  IMHO a better fix would be, not to do these dma setups
// in the first place.
//
    if (scsi_pointer.ptr == core::ptr::null_mut() || scsi_pointer.this_residual == 0)
    return 1;
    fill_hpc_entries(hdata, cmd, datainp);
    pr_debug(" HPCGO\n");
// Start up the HPC.
    hregs.ndptr = hdata.dma;
    if (datainp)
    hregs.ctrl = HPC3_SCTRL_ACTIVE;
    else
    hregs.ctrl = HPC3_SCTRL_ACTIVE | HPC3_SCTRL_DIR;
    return 0;
    }
    static void dma_stop(struct Scsi_Host *instance, struct scsi_cmnd *SCpnt,
    int status)
    {
    struct scsi_pointer *scsi_pointer = WD33C93_scsi_pointer(SCpnt);
    struct ip22_hostdata *hdata = host_to_hostdata(instance);
    struct hpc3_scsiregs *hregs;
    if (!SCpnt)
    return;
    if (scsi_pointer.ptr == core::ptr::null_mut() || scsi_pointer.this_residual == 0)
    return;
    hregs = (struct hpc3_scsiregs *) SCpnt.device.host.base;
    pr_debug("dma_stop: status<%d> ", status);
// First stop the HPC and flush it's FIFO.
    if (hdata.wh.dma_dir) {
    hregs.ctrl |= HPC3_SCTRL_FLUSH;
    while (hregs.ctrl & HPC3_SCTRL_ACTIVE)
    barrier();
    }
    hregs.ctrl = 0;
    dma_unmap_single(hdata.dev, scsi_pointer.dma_handle,
    scsi_pointer.this_residual,
    DMA_DIR(hdata.wh.dma_dir));
    pr_debug("\n");
    }
#[no_mangle]
pub unsafe extern "C" fn sgiwd93_reset(base: c_ulong) {
    void sgiwd93_reset(unsigned long base)
    {
    struct hpc3_scsiregs *hregs = (struct hpc3_scsiregs *) base;
    hregs.ctrl = HPC3_SCTRL_CRESET;
    udelay(50);
    hregs.ctrl = 0;
    }
    EXPORT_SYMBOL_GPL(sgiwd93_reset);
#[no_mangle]
pub unsafe extern "C" fn init_hpc_chain(hdata: *mut ip22_hostdata) {
    static inline void init_hpc_chain(struct ip22_hostdata *hdata)
    {
    struct hpc_chunk *hcp = (struct hpc_chunk *)hdata.cpu;
    let mut dma: dma_addr_t = hdata.dma;
    unsigned long start, end;
    start = (unsigned long) hcp;
    end = start + HPC_DMA_SIZE;
    while (start < end) {
    hcp.desc.pnext = (u32) (dma + sizeof(struct hpc_chunk));
    hcp.desc.cntinfo = HPCDMA_EOX;
    hcp++;
    dma += sizeof(struct hpc_chunk);
    start += sizeof(struct hpc_chunk);
    }
    hcp--;
    hcp.desc.pnext = hdata.dma;
    }
//
// Kludge alert - the SCSI code calls the abort and reset method with int
// arguments not with pointers.  So this is going to blow up beautyfully
// on 64-bit systems with memory outside the compat address spaces.
//
    static const struct scsi_host_template sgiwd93_template = {
    .module			= THIS_MODULE,
    .proc_name		= "SGIWD93",
    .name			= "SGI WD93",
    .queuecommand		= wd33c93_queuecommand,
    .eh_abort_handler	= wd33c93_abort,
    .eh_host_reset_handler	= wd33c93_host_reset,
    .can_queue		= 16,
    .this_id		= 7,
    .sg_tablesize		= SG_ALL,
    .cmd_per_lun		= 8,
    .dma_boundary		= PAGE_SIZE - 1,
    .cmd_size		= sizeof(struct scsi_pointer),
    };
#[no_mangle]
unsafe extern "C" fn sgiwd93_probe(pdev: *mut platform_device) -> c_int {
    static int sgiwd93_probe(struct platform_device *pdev)
    {
    struct sgiwd93_platform_data *pd = pdev.dev.platform_data;
    unsigned char *wdregs = pd.wdregs;
    struct hpc3_scsiregs *hregs = pd.hregs;
    struct ip22_hostdata *hdata;
    struct Scsi_Host *host;
    wd33c93_regs regs;
    let mut unit: c_uint = pd.unit;
    let mut irq: c_uint = pd.irq;
    int err;
    host = scsi_host_alloc(&sgiwd93_template, sizeof(struct ip22_hostdata));
    if (!host) {
    err = -ENOMEM;
    goto out;
    }
    host.base = (unsigned long) hregs;
    host.irq = irq;
    hdata = host_to_hostdata(host);
    hdata.dev = &pdev.dev;
    hdata.cpu = dma_alloc_noncoherent(&pdev.dev, HPC_DMA_SIZE,
    &hdata.dma, DMA_TO_DEVICE, GFP_KERNEL);
    if (!hdata.cpu) {
    printk(KERN_WARNING "sgiwd93: Could not allocate memory for "
    "host %d buffer.\n", unit);
    err = -ENOMEM;
    goto out_put;
    }
    init_hpc_chain(hdata);
    regs.SASR = wdregs + 3;
    regs.SCMD = wdregs + 7;
    hdata.wh.no_sync = 0;
    hdata.wh.fast = 1;
    hdata.wh.dma_mode = CTRL_BURST;
    wd33c93_init(host, regs, dma_setup, dma_stop, WD33C93_FS_MHZ(20));
    err = request_irq(irq, sgiwd93_intr, 0, "SGI WD93", host);
    if (err) {
    printk(KERN_WARNING "sgiwd93: Could not register irq %d "
    "for host %d.\n", irq, unit);
    goto out_free;
    }
    platform_set_drvdata(pdev, host);
    err = scsi_add_host(host, core::ptr::null_mut());
    if (err)
    goto out_irq;
    scsi_scan_host(host);
    return 0;
    out_irq:
    free_irq(irq, host);
    out_free:
    dma_free_noncoherent(&pdev.dev, HPC_DMA_SIZE, hdata.cpu, hdata.dma,
    DMA_TO_DEVICE);
    out_put:
    scsi_host_put(host);
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn sgiwd93_remove(pdev: *mut platform_device) {
    static void sgiwd93_remove(struct platform_device *pdev)
    {
    struct Scsi_Host *host = platform_get_drvdata(pdev);
    struct ip22_hostdata *hdata = (struct ip22_hostdata *) host.hostdata;
    struct sgiwd93_platform_data *pd = pdev.dev.platform_data;
    scsi_remove_host(host);
    free_irq(pd.irq, host);
    dma_free_noncoherent(&pdev.dev, HPC_DMA_SIZE, hdata.cpu, hdata.dma,
    DMA_TO_DEVICE);
    scsi_host_put(host);
    }
    static struct platform_driver sgiwd93_driver = {
    .probe  = sgiwd93_probe,
    .remove = sgiwd93_remove,
    .driver = {
    .name   = "sgiwd93",
    }
    };
#[no_mangle]
unsafe extern "C" fn sgiwd93_module_init() -> int __init {
    static int __init sgiwd93_module_init(void)
    {
    return platform_driver_register(&sgiwd93_driver);
    }
#[no_mangle]
unsafe extern "C" fn sgiwd93_module_exit() -> void __exit {
    static void __exit sgiwd93_module_exit(void)
    {
    return platform_driver_unregister(&sgiwd93_driver);
    }
    module_init(sgiwd93_module_init);
    module_exit(sgiwd93_module_exit);
    MODULE_DESCRIPTION("SGI WD33C93 driver");
    MODULE_AUTHOR("Ralf Baechle <ralf@linux-mips.org>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:sgiwd93");
