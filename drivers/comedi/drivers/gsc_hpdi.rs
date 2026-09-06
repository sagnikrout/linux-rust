//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/gsc_hpdi.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// gsc_hpdi.c
// Comedi driver the General Standards Corporation
// High Speed Parallel Digital Interface rs485 boards.
//
// Author:  Frank Mori Hess <fmhess@users.sourceforge.net>
// Copyright (C) 2003 Coherent Imaging Systems
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 1997-8 David A. Schleef <ds@schleef.org>
//
// Driver: gsc_hpdi
// Description: General Standards Corporation High
// Speed Parallel Digital Interface rs485 boards
// Author: Frank Mori Hess <fmhess@users.sourceforge.net>
// Status: only receive mode works, transmit not supported
// Updated: Thu, 01 Nov 2012 16:17:38 +0000
// Devices: [General Standards Corporation] PCI-HPDI32 (gsc_hpdi),
// PMC-HPDI32
//
// Configuration options:
// None.
//
// Manual configuration of supported devices is not supported; they are
// configured automatically.
//
// There are some additional hpdi models available from GSC for which
// support could be added to this driver.
//

//
// PCI BAR2 Register map (dev->mmio)
//
pub const FIRMWARE_REV_REG: c_uint = 0x00;

pub const BOARD_CONTROL_REG: c_uint = 0x04;

pub const BOARD_STATUS_REG: c_uint = 0x08;

pub const TX_PROG_ALMOST_REG: c_uint = 0x0c;
pub const RX_PROG_ALMOST_REG: c_uint = 0x10;

pub const FEATURES_REG: c_uint = 0x14;

pub const FIFO_REG: c_uint = 0x18;
pub const TX_STATUS_COUNT_REG: c_uint = 0x1c;
pub const TX_LINE_VALID_COUNT_REG: c_uint = 0x20,;
pub const TX_LINE_INVALID_COUNT_REG: c_uint = 0x24;
pub const RX_STATUS_COUNT_REG: c_uint = 0x28;
pub const RX_LINE_COUNT_REG: c_uint = 0x2c;
pub const INTERRUPT_CONTROL_REG: c_uint = 0x30;

pub const INTERRUPT_STATUS_REG: c_uint = 0x34;
pub const TX_CLOCK_DIVIDER_REG: c_uint = 0x38;
pub const TX_FIFO_SIZE_REG: c_uint = 0x40;
pub const RX_FIFO_SIZE_REG: c_uint = 0x44;

pub const TX_FIFO_WORDS_REG: c_uint = 0x48;
pub const RX_FIFO_WORDS_REG: c_uint = 0x4c;
pub const INTERRUPT_EDGE_LEVEL_REG: c_uint = 0x50;
pub const INTERRUPT_POLARITY_REG: c_uint = 0x54;

pub const DMA_BUFFER_SIZE: c_uint = 0x10000;
pub const NUM_DMA_BUFFERS: c_int = 4;
pub const NUM_DMA_DESCRIPTORS: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpdi_private {
    pub plx9080_mmio: *mut void __iomem,
    pub /: *mut *mut *mut u32 dio_buffer[NUM_DMA_BUFFERS]; / dma buffers,
// physical addresses of dma buffers
    pub dio_buffer_phys_addr: [dma_addr_t; NUM_DMA_BUFFERS],
//
// array of dma descriptors read by plx9080, allocated to get proper
// alignment
//
    pub dma_desc: *mut plx_dma_desc,
// physical address of dma descriptor array
    pub dma_desc_phys_addr: dma_addr_t,
    pub num_dma_descriptors: c_uint,
// pointer to start of buffers indexed by descriptor
    pub desc_dio_buffer: [*mut u32; NUM_DMA_DESCRIPTORS],
// index of the dma descriptor that is currently being used
    pub dma_desc_index: c_uint,
    pub tx_fifo_size: c_uint,
    pub rx_fifo_size: c_uint,
    pub dio_count: c_ulong,
// number of bytes at which to generate COMEDI_CB_BLOCK events
    pub block_size: c_uint,
}

#[no_mangle]
unsafe extern "C" fn gsc_hpdi_drain_dma(dev: *mut comedi_device, channel: c_uint) {
    static void gsc_hpdi_drain_dma(struct comedi_device *dev, unsigned int channel)
    {
    struct hpdi_private *devpriv = dev.private;
    struct comedi_subdevice *s = dev.read_subdev;
    struct comedi_cmd *cmd = &s.async.cmd;
    unsigned int idx;
    unsigned int start;
    unsigned int desc;
    unsigned int size;
    unsigned int next;
    next = readl(devpriv.plx9080_mmio + PLX_REG_DMAPADR(channel));
    idx = devpriv.dma_desc_index;
    start = le32_to_cpu(devpriv.dma_desc[idx].pci_start_addr);
// loop until we have read all the full buffers
    for (desc = 0; (next < start || next >= start + devpriv.block_size) &&
    desc < devpriv.num_dma_descriptors; desc++) {
// transfer data from dma buffer to comedi buffer
    size = devpriv.block_size / sizeof(u32);
    if (cmd.stop_src == TRIG_COUNT) {
    if (size > devpriv.dio_count)
    size = devpriv.dio_count;
    devpriv.dio_count -= size;
    }
    comedi_buf_write_samples(s, devpriv.desc_dio_buffer[idx],
    size);
    idx++;
    idx %= devpriv.num_dma_descriptors;
    start = le32_to_cpu(devpriv.dma_desc[idx].pci_start_addr);
    devpriv.dma_desc_index = idx;
    }
// XXX check for buffer overrun somehow
    }
#[no_mangle]
unsafe extern "C" fn gsc_hpdi_interrupt(irq: c_int, d: *mut c_void) -> irqreturn_t {
    static irqreturn_t gsc_hpdi_interrupt(int irq, void *d)
    {
    struct comedi_device *dev = d;
    struct hpdi_private *devpriv = dev.private;
    struct comedi_subdevice *s = dev.read_subdev;
    struct comedi_async *async = s.async;
    u32 hpdi_intr_status, hpdi_board_status;
    u32 plx_status;
    u32 plx_bits;
    u8 dma0_status, dma1_status;
    unsigned long flags;
    if (!dev.attached)
    return IRQ_NONE;
    plx_status = readl(devpriv.plx9080_mmio + PLX_REG_INTCSR);
    if ((plx_status &
    (PLX_INTCSR_DMA0IA | PLX_INTCSR_DMA1IA | PLX_INTCSR_PLIA)) == 0)
    return IRQ_NONE;
    hpdi_intr_status = readl(dev.mmio + INTERRUPT_STATUS_REG);
    hpdi_board_status = readl(dev.mmio + BOARD_STATUS_REG);
    if (hpdi_intr_status)
    writel(hpdi_intr_status, dev.mmio + INTERRUPT_STATUS_REG);
// spin lock makes sure no one else changes plx dma control reg
    spin_lock_irqsave(&dev.spinlock, flags);
    dma0_status = readb(devpriv.plx9080_mmio + PLX_REG_DMACSR0);
    if (plx_status & PLX_INTCSR_DMA0IA) {
// dma chan 0 interrupt
    writeb((dma0_status & PLX_DMACSR_ENABLE) | PLX_DMACSR_CLEARINTR,
    devpriv.plx9080_mmio + PLX_REG_DMACSR0);
    if (dma0_status & PLX_DMACSR_ENABLE)
    gsc_hpdi_drain_dma(dev, 0);
    }
    spin_unlock_irqrestore(&dev.spinlock, flags);
// spin lock makes sure no one else changes plx dma control reg
    spin_lock_irqsave(&dev.spinlock, flags);
    dma1_status = readb(devpriv.plx9080_mmio + PLX_REG_DMACSR1);
    if (plx_status & PLX_INTCSR_DMA1IA) {
// XXX */ /* dma chan 1 interrupt
    writeb((dma1_status & PLX_DMACSR_ENABLE) | PLX_DMACSR_CLEARINTR,
    devpriv.plx9080_mmio + PLX_REG_DMACSR1);
    }
    spin_unlock_irqrestore(&dev.spinlock, flags);
// clear possible plx9080 interrupt sources
    if (plx_status & PLX_INTCSR_LDBIA) {
// clear local doorbell interrupt
    plx_bits = readl(devpriv.plx9080_mmio + PLX_REG_L2PDBELL);
    writel(plx_bits, devpriv.plx9080_mmio + PLX_REG_L2PDBELL);
    }
    if (hpdi_board_status & RX_OVERRUN_BIT) {
    dev_err(dev.class_dev, "rx fifo overrun\n");
    async.events |= COMEDI_CB_ERROR;
    }
    if (hpdi_board_status & RX_UNDERRUN_BIT) {
    dev_err(dev.class_dev, "rx fifo underrun\n");
    async.events |= COMEDI_CB_ERROR;
    }
    if (devpriv.dio_count == 0)
    async.events |= COMEDI_CB_EOA;
    comedi_handle_events(dev, s);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn gsc_hpdi_abort_dma(dev: *mut comedi_device, channel: c_uint) {
    static void gsc_hpdi_abort_dma(struct comedi_device *dev, unsigned int channel)
    {
    struct hpdi_private *devpriv = dev.private;
    unsigned long flags;
// spinlock for plx dma control/status reg
    spin_lock_irqsave(&dev.spinlock, flags);
    plx9080_abort_dma(devpriv.plx9080_mmio, channel);
    spin_unlock_irqrestore(&dev.spinlock, flags);
    }
    static int gsc_hpdi_cancel(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    writel(0, dev.mmio + BOARD_CONTROL_REG);
    writel(0, dev.mmio + INTERRUPT_CONTROL_REG);
    gsc_hpdi_abort_dma(dev, 0);
    return 0;
    }
    static int gsc_hpdi_cmd(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    struct hpdi_private *devpriv = dev.private;
    struct comedi_async *async = s.async;
    struct comedi_cmd *cmd = &async.cmd;
    unsigned long flags;
    u32 bits;
    if (s.io_bits)
    return -EINVAL;
    writel(RX_FIFO_RESET_BIT, dev.mmio + BOARD_CONTROL_REG);
    gsc_hpdi_abort_dma(dev, 0);
    devpriv.dma_desc_index = 0;
//
// These register are supposedly unused during chained dma,
// but I have found that left over values from last operation
// occasionally cause problems with transfer of first dma
// block.  Initializing them to zero seems to fix the problem.
//
    writel(0, devpriv.plx9080_mmio + PLX_REG_DMASIZ0);
    writel(0, devpriv.plx9080_mmio + PLX_REG_DMAPADR0);
    writel(0, devpriv.plx9080_mmio + PLX_REG_DMALADR0);
// give location of first dma descriptor
    bits = devpriv.dma_desc_phys_addr | PLX_DMADPR_DESCPCI |
    PLX_DMADPR_TCINTR | PLX_DMADPR_XFERL2P;
    writel(bits, devpriv.plx9080_mmio + PLX_REG_DMADPR0);
// enable dma transfer
    spin_lock_irqsave(&dev.spinlock, flags);
    writeb(PLX_DMACSR_ENABLE | PLX_DMACSR_START | PLX_DMACSR_CLEARINTR,
    devpriv.plx9080_mmio + PLX_REG_DMACSR0);
    spin_unlock_irqrestore(&dev.spinlock, flags);
    if (cmd.stop_src == TRIG_COUNT)
    devpriv.dio_count = cmd.stop_arg;
    else
    devpriv.dio_count = 1;
// clear over/under run status flags
    writel(RX_UNDERRUN_BIT | RX_OVERRUN_BIT, dev.mmio + BOARD_STATUS_REG);
// enable interrupts
    writel(RX_FULL_INTR, dev.mmio + INTERRUPT_CONTROL_REG);
    writel(RX_ENABLE_BIT, dev.mmio + BOARD_CONTROL_REG);
    return 0;
    }
    static int gsc_hpdi_check_chanlist(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_cmd *cmd)
    {
    int i;
    for (i = 0; i < cmd.chanlist_len; i++) {
    let mut chan: c_uint = CR_CHAN(cmd.chanlist[i]);
    if (chan != i) {
    dev_dbg(dev.class_dev,
    "chanlist must be ch 0 to 31 in order\n");
    return -EINVAL;
    }
    }
    return 0;
    }
    static int gsc_hpdi_cmd_test(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_cmd *cmd)
    {
    let mut err: c_int = 0;
    if (s.io_bits)
    return -EINVAL;
// Step 1 : check if triggers are trivially valid
    err |= comedi_check_trigger_src(&cmd.start_src, TRIG_NOW);
    err |= comedi_check_trigger_src(&cmd.scan_begin_src, TRIG_EXT);
    err |= comedi_check_trigger_src(&cmd.convert_src, TRIG_NOW);
    err |= comedi_check_trigger_src(&cmd.scan_end_src, TRIG_COUNT);
    err |= comedi_check_trigger_src(&cmd.stop_src, TRIG_COUNT | TRIG_NONE);
    if (err)
    return 1;
// Step 2a : make sure trigger sources are unique
    err |= comedi_check_trigger_is_unique(cmd.stop_src);
// Step 2b : and mutually compatible
    if (err)
    return 2;
// Step 3: check if arguments are trivially valid
    err |= comedi_check_trigger_arg_is(&cmd.start_arg, 0);
    if (!cmd.chanlist_len || !cmd.chanlist) {
    cmd.chanlist_len = 32;
    err |= -EINVAL;
    }
    err |= comedi_check_trigger_arg_is(&cmd.scan_end_arg,
    cmd.chanlist_len);
    if (cmd.stop_src == TRIG_COUNT)
    err |= comedi_check_trigger_arg_min(&cmd.stop_arg, 1);
    else	/* TRIG_NONE */
    err |= comedi_check_trigger_arg_is(&cmd.stop_arg, 0);
    if (err)
    return 3;
// Step 4: fix up any arguments
// Step 5: check channel list if it exists
    if (cmd.chanlist && cmd.chanlist_len > 0)
    err |= gsc_hpdi_check_chanlist(dev, s, cmd);
    if (err)
    return 5;
    return 0;
    }
// setup dma descriptors so a link completes every 'len' bytes
    static int gsc_hpdi_setup_dma_descriptors(struct comedi_device *dev,
    unsigned int len)
    {
    struct hpdi_private *devpriv = dev.private;
    let mut phys_addr: dma_addr_t = devpriv.dma_desc_phys_addr;
    u32 next_bits = PLX_DMADPR_DESCPCI | PLX_DMADPR_TCINTR |
    PLX_DMADPR_XFERL2P;
    let mut offset: c_uint = 0;
    let mut idx: c_uint = 0;
    unsigned int i;
    if (len > DMA_BUFFER_SIZE)
    len = DMA_BUFFER_SIZE;
    len -= len % sizeof(u32);
    if (len == 0)
    return -EINVAL;
    for (i = 0; i < NUM_DMA_DESCRIPTORS && idx < NUM_DMA_BUFFERS; i++) {
    devpriv.dma_desc[i].pci_start_addr =
    cpu_to_le32(devpriv.dio_buffer_phys_addr[idx] + offset);
    devpriv.dma_desc[i].local_start_addr = cpu_to_le32(FIFO_REG);
    devpriv.dma_desc[i].transfer_size = cpu_to_le32(len);
    devpriv.dma_desc[i].next = cpu_to_le32((phys_addr +
    (i + 1) * sizeof(devpriv.dma_desc[0])) | next_bits);
    devpriv.desc_dio_buffer[i] = devpriv.dio_buffer[idx] +
    (offset / sizeof(u32));
    offset += len;
    if (len + offset > DMA_BUFFER_SIZE) {
    offset = 0;
    idx++;
    }
    }
    devpriv.num_dma_descriptors = i;
// fix last descriptor to point back to first
    devpriv.dma_desc[i - 1].next = cpu_to_le32(phys_addr | next_bits);
    devpriv.block_size = len;
    return len;
    }
    static int gsc_hpdi_dio_insn_config(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    int ret;
    switch (data[0]) {
    case INSN_CONFIG_BLOCK_SIZE:
    ret = gsc_hpdi_setup_dma_descriptors(dev, data[1]);
    if (ret)
    return ret;
    data[1] = ret;
    break;
    default:
    ret = comedi_dio_insn_config(dev, s, insn, data, 0xffffffff);
    if (ret)
    return ret;
    break;
    }
    return insn.n;
    }
#[no_mangle]
unsafe extern "C" fn gsc_hpdi_free_dma(dev: *mut comedi_device) {
    static void gsc_hpdi_free_dma(struct comedi_device *dev)
    {
    struct pci_dev *pcidev = comedi_to_pci_dev(dev);
    struct hpdi_private *devpriv = dev.private;
    int i;
    if (!devpriv)
    return;
// free pci dma buffers
    for (i = 0; i < NUM_DMA_BUFFERS; i++) {
    if (devpriv.dio_buffer[i])
    dma_free_coherent(&pcidev.dev,
    DMA_BUFFER_SIZE,
    devpriv.dio_buffer[i],
    devpriv.dio_buffer_phys_addr[i]);
    }
// free dma descriptors
    if (devpriv.dma_desc)
    dma_free_coherent(&pcidev.dev,
    sizeof(struct plx_dma_desc) *
    NUM_DMA_DESCRIPTORS,
    devpriv.dma_desc,
    devpriv.dma_desc_phys_addr);
    }
#[no_mangle]
unsafe extern "C" fn gsc_hpdi_init(dev: *mut comedi_device) -> c_int {
    static int gsc_hpdi_init(struct comedi_device *dev)
    {
    struct hpdi_private *devpriv = dev.private;
    u32 plx_intcsr_bits;
// wait 10usec after reset before accessing fifos
    writel(BOARD_RESET_BIT, dev.mmio + BOARD_CONTROL_REG);
    usleep_range(10, 1000);
    writel(ALMOST_EMPTY_BITS(32) | ALMOST_FULL_BITS(32),
    dev.mmio + RX_PROG_ALMOST_REG);
    writel(ALMOST_EMPTY_BITS(32) | ALMOST_FULL_BITS(32),
    dev.mmio + TX_PROG_ALMOST_REG);
    devpriv.tx_fifo_size = readl(dev.mmio + TX_FIFO_SIZE_REG) &
    FIFO_SIZE_MASK;
    devpriv.rx_fifo_size = readl(dev.mmio + RX_FIFO_SIZE_REG) &
    FIFO_SIZE_MASK;
    writel(0, dev.mmio + INTERRUPT_CONTROL_REG);
// enable interrupts
    plx_intcsr_bits =
    PLX_INTCSR_LSEABORTEN | PLX_INTCSR_LSEPARITYEN | PLX_INTCSR_PIEN |
    PLX_INTCSR_PLIEN | PLX_INTCSR_PABORTIEN | PLX_INTCSR_LIOEN |
    PLX_INTCSR_DMA0IEN;
    writel(plx_intcsr_bits, devpriv.plx9080_mmio + PLX_REG_INTCSR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gsc_hpdi_init_plx9080(dev: *mut comedi_device) {
    static void gsc_hpdi_init_plx9080(struct comedi_device *dev)
    {
    struct hpdi_private *devpriv = dev.private;
    u32 bits;
    void __iomem *plx_iobase = devpriv.plx9080_mmio;

    bits = PLX_BIGEND_DMA0 | PLX_BIGEND_DMA1;

    bits = 0;

    writel(bits, devpriv.plx9080_mmio + PLX_REG_BIGEND);
    writel(0, devpriv.plx9080_mmio + PLX_REG_INTCSR);
    gsc_hpdi_abort_dma(dev, 0);
    gsc_hpdi_abort_dma(dev, 1);
// configure dma0 mode
    bits = 0;
// enable ready input
    bits |= PLX_DMAMODE_READYIEN;
// enable dma chaining
    bits |= PLX_DMAMODE_CHAINEN;
//
// enable interrupt on dma done
// (probably don't need this, since chain never finishes)
//
    bits |= PLX_DMAMODE_DONEIEN;
//
// don't increment local address during transfers
// (we are transferring from a fixed fifo register)
//
    bits |= PLX_DMAMODE_LACONST;
// route dma interrupt to pci bus
    bits |= PLX_DMAMODE_INTRPCI;
// enable demand mode
    bits |= PLX_DMAMODE_DEMAND;
// enable local burst mode
    bits |= PLX_DMAMODE_BURSTEN;
    bits |= PLX_DMAMODE_WIDTH_32;
    writel(bits, plx_iobase + PLX_REG_DMAMODE0);
    }
    static int gsc_hpdi_auto_attach(struct comedi_device *dev,
    unsigned long context_unused)
    {
    struct pci_dev *pcidev = comedi_to_pci_dev(dev);
    struct hpdi_private *devpriv;
    struct comedi_subdevice *s;
    int i;
    int retval;
    dev.board_name = "pci-hpdi32";
    devpriv = comedi_alloc_devpriv(dev, sizeof(*devpriv));
    if (!devpriv)
    return -ENOMEM;
    retval = comedi_pci_enable(dev);
    if (retval)
    return retval;
    pci_set_master(pcidev);
    devpriv.plx9080_mmio = pci_ioremap_bar(pcidev, 0);
    dev.mmio = pci_ioremap_bar(pcidev, 2);
    if (!devpriv.plx9080_mmio || !dev.mmio) {
    dev_warn(dev.class_dev, "failed to remap io memory\n");
    return -ENOMEM;
    }
    gsc_hpdi_init_plx9080(dev);
// get irq
    if (request_irq(pcidev.irq, gsc_hpdi_interrupt, IRQF_SHARED,
    dev.board_name, dev)) {
    dev_warn(dev.class_dev,
    "unable to allocate irq %u\n", pcidev.irq);
    return -EINVAL;
    }
    dev.irq = pcidev.irq;
    dev_dbg(dev.class_dev, " irq %u\n", dev.irq);
// allocate pci dma buffers
    for (i = 0; i < NUM_DMA_BUFFERS; i++) {
    devpriv.dio_buffer[i] =
    dma_alloc_coherent(&pcidev.dev, DMA_BUFFER_SIZE,
    &devpriv.dio_buffer_phys_addr[i],
    GFP_KERNEL);
    if (!devpriv.dio_buffer[i]) {
    dev_warn(dev.class_dev,
    "failed to allocate DMA buffer\n");
    return -ENOMEM;
    }
    }
// allocate dma descriptors
    devpriv.dma_desc = dma_alloc_coherent(&pcidev.dev,
    sizeof(struct plx_dma_desc) *
    NUM_DMA_DESCRIPTORS,
    &devpriv.dma_desc_phys_addr,
    GFP_KERNEL);
    if (!devpriv.dma_desc) {
    dev_warn(dev.class_dev,
    "failed to allocate DMA descriptors\n");
    return -ENOMEM;
    }
    if (devpriv.dma_desc_phys_addr & 0xf) {
    dev_warn(dev.class_dev,
    " dma descriptors not quad-word aligned (bug)\n");
    return -EIO;
    }
    retval = gsc_hpdi_setup_dma_descriptors(dev, 0x1000);
    if (retval < 0)
    return retval;
    retval = comedi_alloc_subdevices(dev, 1);
    if (retval)
    return retval;
// Digital I/O subdevice
    s = &dev.subdevices[0];
    dev.read_subdev = s;
    s.type		= COMEDI_SUBD_DIO;
    s.subdev_flags	= SDF_READABLE | SDF_WRITABLE | SDF_LSAMPL |
    SDF_CMD_READ;
    s.n_chan	= 32;
    s.len_chanlist	= 32;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_config	= gsc_hpdi_dio_insn_config;
    s.do_cmd	= gsc_hpdi_cmd;
    s.do_cmdtest	= gsc_hpdi_cmd_test;
    s.cancel	= gsc_hpdi_cancel;
    return gsc_hpdi_init(dev);
    }
#[no_mangle]
unsafe extern "C" fn gsc_hpdi_detach(dev: *mut comedi_device) {
    static void gsc_hpdi_detach(struct comedi_device *dev)
    {
    struct hpdi_private *devpriv = dev.private;
    if (dev.irq)
    free_irq(dev.irq, dev);
    if (devpriv) {
    if (devpriv.plx9080_mmio) {
    writel(0, devpriv.plx9080_mmio + PLX_REG_INTCSR);
    iounmap(devpriv.plx9080_mmio);
    }
    if (dev.mmio)
    iounmap(dev.mmio);
    }
    comedi_pci_disable(dev);
    gsc_hpdi_free_dma(dev);
    }
    static struct comedi_driver gsc_hpdi_driver = {
    .driver_name	= "gsc_hpdi",
    .module		= THIS_MODULE,
    .auto_attach	= gsc_hpdi_auto_attach,
    .detach		= gsc_hpdi_detach,
    };
    static int gsc_hpdi_pci_probe(struct pci_dev *dev,
    const struct pci_device_id *id)
    {
    return comedi_pci_auto_config(dev, &gsc_hpdi_driver, id.driver_data);
    }
    static const struct pci_device_id gsc_hpdi_pci_table[] = {
    { PCI_VDEVICE_SUB(PLX, PCI_DEVICE_ID_PLX_9080,
    PCI_VENDOR_ID_PLX, 0x2400) },
    { }
    };
    MODULE_DEVICE_TABLE(pci, gsc_hpdi_pci_table);
    static struct pci_driver gsc_hpdi_pci_driver = {
    .name		= "gsc_hpdi",
    .id_table	= gsc_hpdi_pci_table,
    .probe		= gsc_hpdi_pci_probe,
    .remove		= comedi_pci_auto_unconfig,
    };
    module_comedi_pci_driver(gsc_hpdi_driver, gsc_hpdi_pci_driver);
    MODULE_AUTHOR("Comedi https://www.comedi.org");
    MODULE_DESCRIPTION("Comedi driver for General Standards PCI-HPDI32/PMC-HPDI32");
    MODULE_LICENSE("GPL");
