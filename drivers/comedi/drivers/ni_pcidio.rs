//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/ni_pcidio.c
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
// Comedi driver for National Instruments PCI-DIO-32HS
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 1999,2002 David A. Schleef <ds@schleef.org>
//
// Driver: ni_pcidio
// Description: National Instruments PCI-DIO32HS, PCI-6533
// Author: ds
// Status: works
// Devices: [National Instruments] PCI-DIO-32HS (ni_pcidio)
// [National Instruments] PXI-6533, PCI-6533 (pxi-6533)
// [National Instruments] PCI-6534 (pci-6534)
// Updated: Mon, 09 Jan 2012 14:27:23 +0000
//
// The DIO32HS board appears as one subdevice, with 32 channels. Each
// channel is individually I/O configurable. The channel order is 0=A0,
// 1=A1, 2=A2, ... 8=B0, 16=C0, 24=D0. The driver only supports simple
// digital I/O; no handshaking is supported.
//
// DMA mostly works for the PCI-DIO32HS, but only in timed input mode.
//
// The PCI-DIO-32HS/PCI-6533 has a configurable external trigger. Setting
// scan_begin_arg to 0 or CR_EDGE triggers on the leading edge. Setting
// scan_begin_arg to CR_INVERT or (CR_EDGE | CR_INVERT) triggers on the
// trailing edge.
//
// This driver could be easily modified to support AT-MIO32HS and AT-MIO96.
//
// The PCI-6534 requires a firmware upload after power-up to work, the
// firmware data and instructions for loading it with comedi_config
// it are contained in the comedi_nonfree_firmware tarball available from
// https://www.comedi.org
//
// Macro flag: #define USE_DMA

// defines for the PCI-DIO-32HS

pub const WINDOW_ADDRESS_STATUS_MASK: c_uint = 0x7c;

// #define SerialRose
// #define ReqRose
// #define Paused

pub const CLEAR_ALL: c_uint = 0xf8;

pub const TRANSFER_COUNT: c_int = 20;
pub const CHIP_ID_D: c_int = 24;
pub const CHIP_ID_I: c_int = 25;
pub const CHIP_ID_O: c_int = 26;
pub const CHIP_VERSION: c_int = 27;

pub const MASTER_CLOCK_ROUTING: c_int = 45;

pub const DATA_PATH: c_int = 64;

pub const PROTOCOL_REGISTER_1: c_int = 65;

pub const PROTOCOL_REGISTER_2: c_int = 66;

pub const PROTOCOL_REGISTER_3: c_int = 67;

pub const PROTOCOL_REGISTER_4: c_int = 70;

pub const PROTOCOL_REGISTER_5: c_int = 71;

pub const FIFO_Control: c_int = 72;

pub const PROTOCOL_REGISTER_6: c_int = 73;

pub const PROTOCOL_REGISTER_7: c_int = 74;

pub const INTERRUPT_CONTROL: c_int = 75;
// bits same as flags
pub const DMA_LINE_CONTROL_GROUP1: c_int = 76;
pub const DMA_LINE_CONTROL_GROUP2: c_int = 108;
// channel zero is none
#[no_mangle]
pub unsafe extern "C" fn primary_DMAChannel_bits(channel: c_uint) -> c_uint {
    static inline unsigned int primary_DMAChannel_bits(unsigned int channel)
    {
    return channel & 0x3;
    }
#[no_mangle]
pub unsafe extern "C" fn secondary_DMAChannel_bits(channel: c_uint) -> c_uint {
    static inline unsigned int secondary_DMAChannel_bits(unsigned int channel)
    {
    return (channel << 2) & 0xc;
    }
pub const TRANSFER_SIZE_CONTROL: c_int = 77;

pub const PROTOCOL_REGISTER_15: c_int = 79;

pub const PATTERN_DETECTION: c_int = 81;

pub const PROTOCOL_REGISTER_9: c_int = 82;

pub const PROTOCOL_REGISTER_10: c_int = 83;

pub const PROTOCOL_REGISTER_11: c_int = 84;

pub const PROTOCOL_REGISTER_12: c_int = 85;

pub const PROTOCOL_REGISTER_13: c_int = 86;

// Firmware files for PCI-6524

    MODULE_FIRMWARE(FW_PCI_6534_MAIN);
    MODULE_FIRMWARE(FW_PCI_6534_SCARAB_DI);
    MODULE_FIRMWARE(FW_PCI_6534_SCARAB_DO);
    enum pci_6534_firmware_registers {	/* 16 bit */
    Firmware_Control_Register = 0x100,
    Firmware_Status_Register = 0x104,
    Firmware_Data_Register = 0x108,
    Firmware_Mask_Register = 0x10c,
    Firmware_Debug_Register = 0x110,
    };
// main fpga registers (32 bit)
    enum pci_6534_fpga_registers {
    FPGA_Control1_Register = 0x200,
    FPGA_Control2_Register = 0x204,
    FPGA_Irq_Mask_Register = 0x208,
    FPGA_Status_Register = 0x20c,
    FPGA_Signature_Register = 0x210,
    FPGA_SCALS_Counter_Register = 0x280,	/*write-clear */
    FPGA_SCAMS_Counter_Register = 0x284,	/*write-clear */
    FPGA_SCBLS_Counter_Register = 0x288,	/*write-clear */
    FPGA_SCBMS_Counter_Register = 0x28c,	/*write-clear */
    FPGA_Temp_Control_Register = 0x2a0,
    FPGA_DAR_Register = 0x2a8,
    FPGA_ELC_Read_Register = 0x2b8,
    FPGA_ELC_Write_Register = 0x2bc,
    };
    enum FPGA_Control_Bits {
    FPGA_Enable_Bit = 0x8000,
    };

    | PRIMARY_TC | SECONDARY_TC)

    enum nidio_boardid {
    BOARD_PCIDIO_32HS,
    BOARD_PXI6533,
    BOARD_PCI6534,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nidio_board {
    pub name: *const c_char,
    pub uses_firmware:1: c_uint,
    pub dio_speed: c_uint,
}

    static const struct nidio_board nidio_boards[] = {
    [BOARD_PCIDIO_32HS] = {
    .name		= "pci-dio-32hs",
    .dio_speed	= 50,
    },
    [BOARD_PXI6533] = {
    .name		= "pxi-6533",
    .dio_speed	= 50,
    },
    [BOARD_PCI6534] = {
    .name		= "pci-6534",
    .uses_firmware	= 1,
    .dio_speed	= 50,
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nidio96_private {
    pub mite: *mut mite,
    pub boardtype: c_int,
    pub dio: c_int,
    pub OP_MODEBits: c_ushort,
    pub di_mite_chan: *mut mite_channel,
    pub di_mite_ring: *mut mite_ring,
    pub mite_channel_lock: spinlock_t,
}

#[no_mangle]
unsafe extern "C" fn ni_pcidio_request_di_mite_channel(dev: *mut comedi_device) -> c_int {
    static int ni_pcidio_request_di_mite_channel(struct comedi_device *dev)
    {
    struct nidio96_private *devpriv = dev.private;
    unsigned long flags;
    spin_lock_irqsave(&devpriv.mite_channel_lock, flags);
    BUG_ON(devpriv.di_mite_chan);
    devpriv.di_mite_chan =
    mite_request_channel_in_range(devpriv.mite,
    devpriv.di_mite_ring, 1, 2);
    if (!devpriv.di_mite_chan) {
    spin_unlock_irqrestore(&devpriv.mite_channel_lock, flags);
    dev_err(dev.class_dev, "failed to reserve mite dma channel\n");
    return -EBUSY;
    }
    devpriv.di_mite_chan.dir = COMEDI_INPUT;
    writeb(primary_DMAChannel_bits(devpriv.di_mite_chan.channel) |
    secondary_DMAChannel_bits(devpriv.di_mite_chan.channel),
    dev.mmio + DMA_LINE_CONTROL_GROUP1);
    spin_unlock_irqrestore(&devpriv.mite_channel_lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ni_pcidio_release_di_mite_channel(dev: *mut comedi_device) {
    static void ni_pcidio_release_di_mite_channel(struct comedi_device *dev)
    {
    struct nidio96_private *devpriv = dev.private;
    unsigned long flags;
    spin_lock_irqsave(&devpriv.mite_channel_lock, flags);
    if (devpriv.di_mite_chan) {
    mite_release_channel(devpriv.di_mite_chan);
    devpriv.di_mite_chan = core::ptr::null_mut();
    writeb(primary_DMAChannel_bits(0) |
    secondary_DMAChannel_bits(0),
    dev.mmio + DMA_LINE_CONTROL_GROUP1);
    }
    spin_unlock_irqrestore(&devpriv.mite_channel_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn setup_mite_dma(dev: *mut comedi_device, s: *mut comedi_subdevice) -> c_int {
    static int setup_mite_dma(struct comedi_device *dev, struct comedi_subdevice *s)
    {
    struct nidio96_private *devpriv = dev.private;
    int retval;
    unsigned long flags;
    retval = ni_pcidio_request_di_mite_channel(dev);
    if (retval)
    return retval;
// write alloc the entire buffer
    comedi_buf_write_alloc(s, s.async.prealloc_bufsz);
    spin_lock_irqsave(&devpriv.mite_channel_lock, flags);
    if (devpriv.di_mite_chan) {
    mite_prep_dma(devpriv.di_mite_chan, 32, 32);
    mite_dma_arm(devpriv.di_mite_chan);
    } else {
    retval = -EIO;
    }
    spin_unlock_irqrestore(&devpriv.mite_channel_lock, flags);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn ni_pcidio_poll(dev: *mut comedi_device, s: *mut comedi_subdevice) -> c_int {
    static int ni_pcidio_poll(struct comedi_device *dev, struct comedi_subdevice *s)
    {
    struct nidio96_private *devpriv = dev.private;
    unsigned long irq_flags;
    int count;
    spin_lock_irqsave(&dev.spinlock, irq_flags);
    spin_lock(&devpriv.mite_channel_lock);
    if (devpriv.di_mite_chan)
    mite_sync_dma(devpriv.di_mite_chan, s);
    spin_unlock(&devpriv.mite_channel_lock);
    count = comedi_buf_n_bytes_ready(s);
    spin_unlock_irqrestore(&dev.spinlock, irq_flags);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn nidio_interrupt(irq: c_int, d: *mut c_void) -> irqreturn_t {
    static irqreturn_t nidio_interrupt(int irq, void *d)
    {
    struct comedi_device *dev = d;
    struct nidio96_private *devpriv = dev.private;
    struct comedi_subdevice *s = dev.read_subdev;
    struct comedi_async *async = s.async;
    unsigned int auxdata;
    int flags;
    int status;
    let mut work: c_int = 0;
// interrupcions parasites
    if (!dev.attached) {
// assume it's from another card
    return IRQ_NONE;
    }
// Lock to avoid race with comedi_poll
    spin_lock(&dev.spinlock);
    status = readb(dev.mmio + INTERRUPT_AND_WINDOW_STATUS);
    flags = readb(dev.mmio + GROUP_1_FLAGS);
    spin_lock(&devpriv.mite_channel_lock);
    if (devpriv.di_mite_chan) {
    mite_ack_linkc(devpriv.di_mite_chan, s, false);
// XXX need to byteswap sync'ed dma
    }
    spin_unlock(&devpriv.mite_channel_lock);
    while (status & DATA_LEFT) {
    work++;
    if (work > 20) {
    dev_dbg(dev.class_dev, "too much work in interrupt\n");
    writeb(0x00,
    dev.mmio + MASTER_DMA_AND_INTERRUPT_CONTROL);
    break;
    }
    flags &= INT_EN;
    if (flags & TRANSFER_READY) {
    while (flags & TRANSFER_READY) {
    work++;
    if (work > 100) {
    dev_dbg(dev.class_dev,
    "too much work in interrupt\n");
    writeb(0x00, dev.mmio +
    MASTER_DMA_AND_INTERRUPT_CONTROL
    );
    goto out;
    }
    auxdata = readl(dev.mmio + GROUP_1_FIFO);
    comedi_buf_write_samples(s, &auxdata, 1);
    flags = readb(dev.mmio + GROUP_1_FLAGS);
    }
    }
    if (flags & COUNT_EXPIRED) {
    writeb(CLEAR_EXPIRED, dev.mmio + GROUP_1_SECOND_CLEAR);
    async.events |= COMEDI_CB_EOA;
    writeb(0x00, dev.mmio + OP_MODE);
    break;
    } else if (flags & WAITED) {
    writeb(CLEAR_WAITED, dev.mmio + GROUP_1_FIRST_CLEAR);
    async.events |= COMEDI_CB_ERROR;
    break;
    } else if (flags & PRIMARY_TC) {
    writeb(CLEAR_PRIMARY_TC,
    dev.mmio + GROUP_1_FIRST_CLEAR);
    async.events |= COMEDI_CB_EOA;
    } else if (flags & SECONDARY_TC) {
    writeb(CLEAR_SECONDARY_TC,
    dev.mmio + GROUP_1_FIRST_CLEAR);
    async.events |= COMEDI_CB_EOA;
    }
    flags = readb(dev.mmio + GROUP_1_FLAGS);
    status = readb(dev.mmio + INTERRUPT_AND_WINDOW_STATUS);
    }
    out:
    comedi_handle_events(dev, s);

    if (!tag)
    writeb(0x03, dev.mmio + MASTER_DMA_AND_INTERRUPT_CONTROL);

    spin_unlock(&dev.spinlock);
    return IRQ_HANDLED;
    }
    static int ni_pcidio_insn_config(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    int ret;
    if (data[0] == INSN_CONFIG_GET_CMD_TIMING_CONSTRAINTS) {
    const struct nidio_board *board = dev.board_ptr;
// we don't care about actual channels
    data[1] = board.dio_speed;
    data[2] = 0;
    return 0;
    }
    ret = comedi_dio_insn_config(dev, s, insn, data, 0);
    if (ret)
    return ret;
    writel(s.io_bits, dev.mmio + PORT_PIN_DIRECTIONS(0));
    return insn.n;
    }
    static int ni_pcidio_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    if (comedi_dio_update_state(s, data))
    writel(s.state, dev.mmio + PORT_IO(0));
    data[1] = readl(dev.mmio + PORT_IO(0));
    return insn.n;
    }
#[no_mangle]
unsafe extern "C" fn ni_pcidio_ns_to_timer(nanosec: *mut c_int, flags: c_uint) -> c_int {
    static int ni_pcidio_ns_to_timer(int *nanosec, unsigned int flags)
    {
    int divider, base;
    base = TIMER_BASE;
    switch (flags & CMDF_ROUND_MASK) {
    case CMDF_ROUND_NEAREST:
    default:
    divider = DIV_ROUND_CLOSEST(*nanosec, base);
    break;
    case CMDF_ROUND_DOWN:
    divider = (*nanosec) / base;
    break;
    case CMDF_ROUND_UP:
    divider = DIV_ROUND_UP(*nanosec, base);
    break;
    }
// nanosec = base * divider;
    return divider;
    }
    static int ni_pcidio_cmdtest(struct comedi_device *dev,
    struct comedi_subdevice *s, struct comedi_cmd *cmd)
    {
    let mut err: c_int = 0;
    unsigned int arg;
// Step 1 : check if triggers are trivially valid
    err |= comedi_check_trigger_src(&cmd.start_src, TRIG_NOW | TRIG_INT);
    err |= comedi_check_trigger_src(&cmd.scan_begin_src,
    TRIG_TIMER | TRIG_EXT);
    err |= comedi_check_trigger_src(&cmd.convert_src, TRIG_NOW);
    err |= comedi_check_trigger_src(&cmd.scan_end_src, TRIG_COUNT);
    err |= comedi_check_trigger_src(&cmd.stop_src, TRIG_COUNT | TRIG_NONE);
    if (err)
    return 1;
// Step 2a : make sure trigger sources are unique
    err |= comedi_check_trigger_is_unique(cmd.start_src);
    err |= comedi_check_trigger_is_unique(cmd.scan_begin_src);
    err |= comedi_check_trigger_is_unique(cmd.stop_src);
// Step 2b : and mutually compatible
    if (err)
    return 2;
// Step 3: check if arguments are trivially valid
    err |= comedi_check_trigger_arg_is(&cmd.start_arg, 0);

    if (cmd.scan_begin_src == TRIG_TIMER) {
    err |= comedi_check_trigger_arg_min(&cmd.scan_begin_arg,
    MAX_SPEED);
// no minimum speed
    } else {
// TRIG_EXT
// should be level/edge, hi/lo specification here
    if ((cmd.scan_begin_arg & ~(CR_EDGE | CR_INVERT)) != 0) {
    cmd.scan_begin_arg &= (CR_EDGE | CR_INVERT);
    err |= -EINVAL;
    }
    }
    err |= comedi_check_trigger_arg_is(&cmd.convert_arg, 0);
    err |= comedi_check_trigger_arg_is(&cmd.scan_end_arg,
    cmd.chanlist_len);
    if (cmd.stop_src == TRIG_COUNT)
    err |= comedi_check_trigger_arg_min(&cmd.stop_arg, 1);
    else	/* TRIG_NONE */
    err |= comedi_check_trigger_arg_is(&cmd.stop_arg, 0);
    if (err)
    return 3;
// step 4: fix up any arguments
    if (cmd.scan_begin_src == TRIG_TIMER) {
    arg = cmd.scan_begin_arg;
    ni_pcidio_ns_to_timer(&arg, cmd.flags);
    err |= comedi_check_trigger_arg_is(&cmd.scan_begin_arg, arg);
    }
    if (err)
    return 4;
    return 0;
    }
    static int ni_pcidio_inttrig(struct comedi_device *dev,
    struct comedi_subdevice *s,
    unsigned int trig_num)
    {
    struct nidio96_private *devpriv = dev.private;
    struct comedi_cmd *cmd = &s.async.cmd;
    if (trig_num != cmd.start_arg)
    return -EINVAL;
    writeb(devpriv.OP_MODEBits, dev.mmio + OP_MODE);
    s.async.inttrig = core::ptr::null_mut();
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn ni_pcidio_cmd(dev: *mut comedi_device, s: *mut comedi_subdevice) -> c_int {
    static int ni_pcidio_cmd(struct comedi_device *dev, struct comedi_subdevice *s)
    {
    struct nidio96_private *devpriv = dev.private;
    struct comedi_cmd *cmd = &s.async.cmd;
// XXX configure ports for input
    writel(0x0000, dev.mmio + PORT_PIN_DIRECTIONS(0));
    if (1) {
// enable fifos A B C D
    writeb(0x0f, dev.mmio + DATA_PATH);
// set transfer width a 32 bits
    writeb(TRANSFER_WIDTH(0) | TRANSFER_LENGTH(0),
    dev.mmio + TRANSFER_SIZE_CONTROL);
    } else {
    writeb(0x03, dev.mmio + DATA_PATH);
    writeb(TRANSFER_WIDTH(3) | TRANSFER_LENGTH(0),
    dev.mmio + TRANSFER_SIZE_CONTROL);
    }
// protocol configuration
    if (cmd.scan_begin_src == TRIG_TIMER) {
// page 4-5, "input with internal REQs"
    writeb(0, dev.mmio + OP_MODE);
    writeb(0x00, dev.mmio + CLOCK_REG);
    writeb(1, dev.mmio + SEQUENCE);
    writeb(0x04, dev.mmio + REQ_REG);
    writeb(4, dev.mmio + BLOCK_MODE);
    writeb(3, dev.mmio + LINE_POLARITIES);
    writeb(0xc0, dev.mmio + ACK_SER);
    writel(ni_pcidio_ns_to_timer(&cmd.scan_begin_arg,
    CMDF_ROUND_NEAREST),
    dev.mmio + START_DELAY);
    writeb(1, dev.mmio + REQ_DELAY);
    writeb(1, dev.mmio + REQ_NOT_DELAY);
    writeb(1, dev.mmio + ACK_DELAY);
    writeb(0x0b, dev.mmio + ACK_NOT_DELAY);
    writeb(0x01, dev.mmio + DATA_1_DELAY);
//
// manual, page 4-5:
// CLOCK_SPEED comment is incorrectly listed on DAQ_OPTIONS
//
    writew(0, dev.mmio + CLOCK_SPEED);
    writeb(0, dev.mmio + DAQ_OPTIONS);
    } else {
// TRIG_EXT
// page 4-5, "input with external REQs"
    writeb(0, dev.mmio + OP_MODE);
    writeb(0x00, dev.mmio + CLOCK_REG);
    writeb(0, dev.mmio + SEQUENCE);
    writeb(0x00, dev.mmio + REQ_REG);
    writeb(4, dev.mmio + BLOCK_MODE);
    if (!(cmd.scan_begin_arg & CR_INVERT))	/* Leading Edge */
    writeb(0, dev.mmio + LINE_POLARITIES);
    else					/* Trailing Edge */
    writeb(2, dev.mmio + LINE_POLARITIES);
    writeb(0x00, dev.mmio + ACK_SER);
    writel(1, dev.mmio + START_DELAY);
    writeb(1, dev.mmio + REQ_DELAY);
    writeb(1, dev.mmio + REQ_NOT_DELAY);
    writeb(1, dev.mmio + ACK_DELAY);
    writeb(0x0C, dev.mmio + ACK_NOT_DELAY);
    writeb(0x10, dev.mmio + DATA_1_DELAY);
    writew(0, dev.mmio + CLOCK_SPEED);
    writeb(0x60, dev.mmio + DAQ_OPTIONS);
    }
    if (cmd.stop_src == TRIG_COUNT) {
    writel(cmd.stop_arg,
    dev.mmio + TRANSFER_COUNT);
    } else {
// XXX
    }

    writeb(CLEAR_PRIMARY_TC | CLEAR_SECONDARY_TC,
    dev.mmio + GROUP_1_FIRST_CLEAR);
    {
    let mut retval: c_int = setup_mite_dma(dev, s);
    if (retval)
    return retval;
    }

    writeb(0x00, dev.mmio + DMA_LINE_CONTROL_GROUP1);

    writeb(0x00, dev.mmio + DMA_LINE_CONTROL_GROUP2);
// clear and enable interrupts
    writeb(0xff, dev.mmio + GROUP_1_FIRST_CLEAR);
// writeb(CLEAR_EXPIRED, dev->mmio+GROUP_1_SECOND_CLEAR);
    writeb(INT_EN, dev.mmio + INTERRUPT_CONTROL);
    writeb(0x03, dev.mmio + MASTER_DMA_AND_INTERRUPT_CONTROL);
    if (cmd.stop_src == TRIG_NONE) {
    devpriv.OP_MODEBits = DATA_LATCHING(0) | RUN_MODE(7);
    } else {		/* TRIG_TIMER */
    devpriv.OP_MODEBits = NUMBERED | RUN_MODE(7);
    }
    if (cmd.start_src == TRIG_NOW) {
// start
    writeb(devpriv.OP_MODEBits, dev.mmio + OP_MODE);
    s.async.inttrig = core::ptr::null_mut();
    } else {
// TRIG_INT
    s.async.inttrig = ni_pcidio_inttrig;
    }
    return 0;
    }
    static int ni_pcidio_cancel(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    writeb(0x00, dev.mmio + MASTER_DMA_AND_INTERRUPT_CONTROL);
    ni_pcidio_release_di_mite_channel(dev);
    return 0;
    }
    static int ni_pcidio_change(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    struct nidio96_private *devpriv = dev.private;
    int ret;
    ret = mite_buf_change(devpriv.di_mite_ring, s);
    if (ret < 0)
    return ret;
    return 0;
    }
    static int pci_6534_load_fpga(struct comedi_device *dev,
    const u8 *data, size_t data_len,
    unsigned long context)
    {
    let mut timeout: static int = 1000;
    let mut fpga_index: c_int = context;
    int i;
    size_t j;
    writew(0x80 | fpga_index, dev.mmio + Firmware_Control_Register);
    writew(0xc0 | fpga_index, dev.mmio + Firmware_Control_Register);
    for (i = 0;
    (readw(dev.mmio + Firmware_Status_Register) & 0x2) == 0 &&
    i < timeout; ++i) {
    udelay(1);
    }
    if (i == timeout) {
    dev_warn(dev.class_dev,
    "ni_pcidio: failed to load fpga %i, waiting for status 0x2\n",
    fpga_index);
    return -EIO;
    }
    writew(0x80 | fpga_index, dev.mmio + Firmware_Control_Register);
    for (i = 0;
    readw(dev.mmio + Firmware_Status_Register) != 0x3 &&
    i < timeout; ++i) {
    udelay(1);
    }
    if (i == timeout) {
    dev_warn(dev.class_dev,
    "ni_pcidio: failed to load fpga %i, waiting for status 0x3\n",
    fpga_index);
    return -EIO;
    }
    for (j = 0; j + 1 < data_len;) {
    let mut value: c_uint = data[j++];
    value |= data[j++] << 8;
    writew(value, dev.mmio + Firmware_Data_Register);
    for (i = 0;
    (readw(dev.mmio + Firmware_Status_Register) & 0x2) == 0
    && i < timeout; ++i) {
    udelay(1);
    }
    if (i == timeout) {
    dev_warn(dev.class_dev,
    "ni_pcidio: failed to load word into fpga %i\n",
    fpga_index);
    return -EIO;
    }
    if (need_resched())
    schedule();
    }
    writew(0x0, dev.mmio + Firmware_Control_Register);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci_6534_reset_fpga(dev: *mut comedi_device, fpga_index: c_int) -> c_int {
    static int pci_6534_reset_fpga(struct comedi_device *dev, int fpga_index)
    {
    return pci_6534_load_fpga(dev, core::ptr::null_mut(), 0, fpga_index);
    }
#[no_mangle]
unsafe extern "C" fn pci_6534_reset_fpgas(dev: *mut comedi_device) -> c_int {
    static int pci_6534_reset_fpgas(struct comedi_device *dev)
    {
    int ret;
    int i;
    writew(0x0, dev.mmio + Firmware_Control_Register);
    for (i = 0; i < 3; ++i) {
    ret = pci_6534_reset_fpga(dev, i);
    if (ret < 0)
    break;
    }
    writew(0x0, dev.mmio + Firmware_Mask_Register);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pci_6534_init_main_fpga(dev: *mut comedi_device) {
    static void pci_6534_init_main_fpga(struct comedi_device *dev)
    {
    writel(0, dev.mmio + FPGA_Control1_Register);
    writel(0, dev.mmio + FPGA_Control2_Register);
    writel(0, dev.mmio + FPGA_SCALS_Counter_Register);
    writel(0, dev.mmio + FPGA_SCAMS_Counter_Register);
    writel(0, dev.mmio + FPGA_SCBLS_Counter_Register);
    writel(0, dev.mmio + FPGA_SCBMS_Counter_Register);
    }
#[no_mangle]
unsafe extern "C" fn pci_6534_upload_firmware(dev: *mut comedi_device) -> c_int {
    static int pci_6534_upload_firmware(struct comedi_device *dev)
    {
    struct nidio96_private *devpriv = dev.private;
    static const char *const fw_file[3] = {
    FW_PCI_6534_SCARAB_DI,	/* loaded into scarab A for DI */
    FW_PCI_6534_SCARAB_DO,	/* loaded into scarab B for DO */
    FW_PCI_6534_MAIN,	/* loaded into main FPGA */
    };
    int ret;
    int n;
    ret = pci_6534_reset_fpgas(dev);
    if (ret < 0)
    return ret;
// load main FPGA first, then the two scarabs
    for (n = 2; n >= 0; n--) {
    ret = comedi_load_firmware(dev, &devpriv.mite.pcidev.dev,
    fw_file[n],
    pci_6534_load_fpga, n);
    if (ret == 0 && n == 2)
    pci_6534_init_main_fpga(dev);
    if (ret < 0)
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nidio_reset_board(dev: *mut comedi_device) {
    static void nidio_reset_board(struct comedi_device *dev)
    {
    writel(0, dev.mmio + PORT_IO(0));
    writel(0, dev.mmio + PORT_PIN_DIRECTIONS(0));
    writel(0, dev.mmio + PORT_PIN_MASK(0));
// disable interrupts on board
    writeb(0, dev.mmio + MASTER_DMA_AND_INTERRUPT_CONTROL);
    }
    static int nidio_auto_attach(struct comedi_device *dev,
    unsigned long context)
    {
    struct pci_dev *pcidev = comedi_to_pci_dev(dev);
    const struct nidio_board *board = core::ptr::null_mut();
    struct nidio96_private *devpriv;
    struct comedi_subdevice *s;
    int ret;
    unsigned int irq;
    if (context < ARRAY_SIZE(nidio_boards))
    board = &nidio_boards[context];
    if (!board)
    return -ENODEV;
    dev.board_ptr = board;
    dev.board_name = board.name;
    ret = comedi_pci_enable(dev);
    if (ret)
    return ret;
    devpriv = comedi_alloc_devpriv(dev, sizeof(*devpriv));
    if (!devpriv)
    return -ENOMEM;
    spin_lock_init(&devpriv.mite_channel_lock);
    devpriv.mite = mite_attach(dev, false);	/* use win0 */
    if (!devpriv.mite)
    return -ENOMEM;
    devpriv.di_mite_ring = mite_alloc_ring(devpriv.mite);
    if (!devpriv.di_mite_ring)
    return -ENOMEM;
    if (board.uses_firmware) {
    ret = pci_6534_upload_firmware(dev);
    if (ret < 0)
    return ret;
    }
    nidio_reset_board(dev);
    ret = comedi_alloc_subdevices(dev, 1);
    if (ret)
    return ret;
    dev_info(dev.class_dev, "%s rev=%d\n", dev.board_name,
    readb(dev.mmio + CHIP_VERSION));
    s = &dev.subdevices[0];
    dev.read_subdev = s;
    s.type = COMEDI_SUBD_DIO;
    s.subdev_flags =
    SDF_READABLE | SDF_WRITABLE | SDF_LSAMPL | SDF_PACKED |
    SDF_CMD_READ;
    s.n_chan = 32;
    s.range_table = &range_digital;
    s.maxdata = 1;
    s.insn_config = &ni_pcidio_insn_config;
    s.insn_bits = &ni_pcidio_insn_bits;
    s.do_cmd = &ni_pcidio_cmd;
    s.do_cmdtest = &ni_pcidio_cmdtest;
    s.cancel = &ni_pcidio_cancel;
    s.len_chanlist = 32;	/* XXX */
    s.buf_change = &ni_pcidio_change;
    s.async_dma_dir = DMA_BIDIRECTIONAL;
    s.poll = &ni_pcidio_poll;
    irq = pcidev.irq;
    if (irq) {
    ret = request_irq(irq, nidio_interrupt, IRQF_SHARED,
    dev.board_name, dev);
    if (ret == 0)
    dev.irq = irq;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nidio_detach(dev: *mut comedi_device) {
    static void nidio_detach(struct comedi_device *dev)
    {
    struct nidio96_private *devpriv = dev.private;
    if (dev.irq)
    free_irq(dev.irq, dev);
    if (devpriv) {
    if (devpriv.di_mite_ring) {
    mite_free_ring(devpriv.di_mite_ring);
    devpriv.di_mite_ring = core::ptr::null_mut();
    }
    mite_detach(devpriv.mite);
    }
    if (dev.mmio)
    iounmap(dev.mmio);
    comedi_pci_disable(dev);
    }
    static struct comedi_driver ni_pcidio_driver = {
    .driver_name	= "ni_pcidio",
    .module		= THIS_MODULE,
    .auto_attach	= nidio_auto_attach,
    .detach		= nidio_detach,
    };
    static int ni_pcidio_pci_probe(struct pci_dev *dev,
    const struct pci_device_id *id)
    {
    return comedi_pci_auto_config(dev, &ni_pcidio_driver, id.driver_data);
    }
    static const struct pci_device_id ni_pcidio_pci_table[] = {
    { PCI_VDEVICE(NI, 0x1150), .driver_data = BOARD_PCIDIO_32HS },
    { PCI_VDEVICE(NI, 0x12b0), .driver_data = BOARD_PCI6534 },
    { PCI_VDEVICE(NI, 0x1320), .driver_data = BOARD_PXI6533 },
    { }
    };
    MODULE_DEVICE_TABLE(pci, ni_pcidio_pci_table);
    static struct pci_driver ni_pcidio_pci_driver = {
    .name		= "ni_pcidio",
    .id_table	= ni_pcidio_pci_table,
    .probe		= ni_pcidio_pci_probe,
    .remove		= comedi_pci_auto_unconfig,
    };
    module_comedi_pci_driver(ni_pcidio_driver, ni_pcidio_pci_driver);
    MODULE_AUTHOR("Comedi https://www.comedi.org");
    MODULE_DESCRIPTION("Comedi low-level driver");
    MODULE_LICENSE("GPL");
