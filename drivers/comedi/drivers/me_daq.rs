//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/me_daq.c
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
// comedi/drivers/me_daq.c
// Hardware driver for Meilhaus data acquisition cards:
// ME-2000i, ME-2600i, ME-3000vm1
//
// Copyright (C) 2002 Michael Hillmann <hillmann@syscongroup.de>
//
// Driver: me_daq
// Description: Meilhaus PCI data acquisition cards
// Devices: [Meilhaus] ME-2600i (me-2600i), ME-2000i (me-2000i)
// Author: Michael Hillmann <hillmann@syscongroup.de>
// Status: experimental
//
// Configuration options: not applicable, uses PCI auto config
//
// Supports:
// Analog Input, Analog Output, Digital I/O
//

pub const XILINX_DOWNLOAD_RESET: c_uint = 0x42	/* Xilinx registers */;
//
// PCI BAR2 Memory map (dev->mmio)
//
pub const ME_CTRL1_REG: c_uint = 0x00	/* R (ai start) | W */;

pub const ME_CTRL2_REG: c_uint = 0x02	/* R (dac update) | W */;

pub const ME_STATUS_REG: c_uint = 0x04	/* R | W (clears interrupts) */;

pub const ME_DIO_PORT_A_REG: c_uint = 0x06	/* R | W */;
pub const ME_DIO_PORT_B_REG: c_uint = 0x08	/* R | W */;

pub const ME_AI_FIFO_REG: c_uint = 0x10	/* R (fifo) | W (chanlist) */;

pub const ME_DAC_CTRL_REG: c_uint = 0x12	/* R (updates) | W */;

    ME_DAC_CTRL_GAIN(x))

    static const struct comedi_lrange me_ai_range = {
    8, {
    BIP_RANGE(10),
    BIP_RANGE(5),
    BIP_RANGE(2.5),
    BIP_RANGE(1.25),
    UNI_RANGE(10),
    UNI_RANGE(5),
    UNI_RANGE(2.5),
    UNI_RANGE(1.25)
    }
    };
    static const struct comedi_lrange me_ao_range = {
    3, {
    BIP_RANGE(10),
    BIP_RANGE(5),
    UNI_RANGE(10)
    }
    };
    enum me_boardid {
    BOARD_ME2600,
    BOARD_ME2000,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct me_board {
    pub name: *const c_char,
    pub needs_firmware: c_int,
    pub has_ao: c_int,
}

    static const struct me_board me_boards[] = {
    [BOARD_ME2600] = {
    .name		= "me-2600i",
    .needs_firmware	= 1,
    .has_ao		= 1,
    },
    [BOARD_ME2000] = {
    .name		= "me-2000i",
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct me_private_data {
    pub /: *mut *mut *mut void __iomem plx_regbase; / PLX configuration base address,
    pub /: *mut *mut unsigned short ctrl1; / Mirror of CONTROL_1 register,
    pub /: *mut *mut unsigned short ctrl2; / Mirror of CONTROL_2 register,
    pub /: *mut *mut unsigned short dac_ctrl; / Mirror of the DAC_CONTROL register,
}

#[no_mangle]
pub unsafe extern "C" fn sleep(sec: c_uint) {
    static inline void sleep(unsigned int sec)
    {
    schedule_timeout_interruptible(sec * HZ);
    }
    static int me_dio_insn_config(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct me_private_data *devpriv = dev.private;
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    unsigned int mask;
    int ret;
    if (chan < 16)
    mask = 0x0000ffff;
    else
    mask = 0xffff0000;
    ret = comedi_dio_insn_config(dev, s, insn, data, mask);
    if (ret)
    return ret;
    if (s.io_bits & 0x0000ffff)
    devpriv.ctrl2 |= ME_CTRL2_PORT_A_ENA;
    else
    devpriv.ctrl2 &= ~ME_CTRL2_PORT_A_ENA;
    if (s.io_bits & 0xffff0000)
    devpriv.ctrl2 |= ME_CTRL2_PORT_B_ENA;
    else
    devpriv.ctrl2 &= ~ME_CTRL2_PORT_B_ENA;
    writew(devpriv.ctrl2, dev.mmio + ME_CTRL2_REG);
    return insn.n;
    }
    static int me_dio_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    void __iomem *mmio_porta = dev.mmio + ME_DIO_PORT_A_REG;
    void __iomem *mmio_portb = dev.mmio + ME_DIO_PORT_B_REG;
    unsigned int mask;
    unsigned int val;
    mask = comedi_dio_update_state(s, data);
    if (mask) {
    if (mask & 0x0000ffff)
    writew((s.state & 0xffff), mmio_porta);
    if (mask & 0xffff0000)
    writew(((s.state >> 16) & 0xffff), mmio_portb);
    }
    if (s.io_bits & 0x0000ffff)
    val = s.state & 0xffff;
    else
    val = readw(mmio_porta);
    if (s.io_bits & 0xffff0000)
    val |= (s.state & 0xffff0000);
    else
    val |= (readw(mmio_portb) << 16);
    data[1] = val;
    return insn.n;
    }
    static int me_ai_eoc(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned long context)
    {
    unsigned int status;
    status = readw(dev.mmio + ME_STATUS_REG);
    if ((status & ME_STATUS_ADFIFO_EMPTY) == 0)
    return 0;
    return -EBUSY;
    }
    static int me_ai_insn_read(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct me_private_data *devpriv = dev.private;
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    let mut range: c_uint = CR_RANGE(insn.chanspec);
    let mut aref: c_uint = CR_AREF(insn.chanspec);
    unsigned int val;
    let mut ret: c_int = 0;
    int i;
//
// For differential operation, there are only 8 input channels
// and only bipolar ranges are available.
//
    if (aref & AREF_DIFF) {
    if (chan > 7 || comedi_range_is_unipolar(s, range))
    return -EINVAL;
    }
// clear chanlist and ad fifo
    devpriv.ctrl2 &= ~(ME_CTRL2_ADFIFO_ENA | ME_CTRL2_CHANLIST_ENA);
    writew(devpriv.ctrl2, dev.mmio + ME_CTRL2_REG);
    writew(0x00, dev.mmio + ME_STATUS_REG);	/* clear interrupts */
// enable the chanlist and ADC fifo
    devpriv.ctrl2 |= (ME_CTRL2_ADFIFO_ENA | ME_CTRL2_CHANLIST_ENA);
    writew(devpriv.ctrl2, dev.mmio + ME_CTRL2_REG);
// write to channel list fifo
    val = ME_AI_FIFO_CHANLIST_CHAN(chan) | ME_AI_FIFO_CHANLIST_GAIN(range);
    if (comedi_range_is_unipolar(s, range))
    val |= ME_AI_FIFO_CHANLIST_UNIPOLAR;
    if (aref & AREF_DIFF)
    val |= ME_AI_FIFO_CHANLIST_DIFF;
    writew(val, dev.mmio + ME_AI_FIFO_REG);
// set ADC mode to software trigger
    devpriv.ctrl1 |= ME_CTRL1_ADC_MODE_SOFT_TRIG;
    writew(devpriv.ctrl1, dev.mmio + ME_CTRL1_REG);
    for (i = 0; i < insn.n; i++) {
// start ai conversion
    readw(dev.mmio + ME_CTRL1_REG);
// wait for ADC fifo not empty flag
    ret = comedi_timeout(dev, s, insn, me_ai_eoc, 0);
    if (ret)
    break;
// get value from ADC fifo
    val = readw(dev.mmio + ME_AI_FIFO_REG) & s.maxdata;
// munge 2's complement value to offset binary
    data[i] = comedi_offset_munge(s, val);
    }
// stop any running conversion
    devpriv.ctrl1 &= ~ME_CTRL1_ADC_MODE_MASK;
    writew(devpriv.ctrl1, dev.mmio + ME_CTRL1_REG);
    return ret ? ret : insn.n;
    }
    static int me_ao_insn_write(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct me_private_data *devpriv = dev.private;
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    let mut range: c_uint = CR_RANGE(insn.chanspec);
    let mut val: c_uint = s.readback[chan];
    int i;
// Enable all DAC
    devpriv.ctrl2 |= ME_CTRL2_DAC_ENA;
    writew(devpriv.ctrl2, dev.mmio + ME_CTRL2_REG);
// and set DAC to "buffered" mode
    devpriv.ctrl2 |= ME_CTRL2_BUFFERED_DAC;
    writew(devpriv.ctrl2, dev.mmio + ME_CTRL2_REG);
// Set dac-control register
    devpriv.dac_ctrl &= ~ME_DAC_CTRL_MASK(chan);
    if (range == 0)
    devpriv.dac_ctrl |= ME_DAC_CTRL_GAIN(chan);
    if (comedi_range_is_bipolar(s, range))
    devpriv.dac_ctrl |= ME_DAC_CTRL_BIPOLAR(chan);
    writew(devpriv.dac_ctrl, dev.mmio + ME_DAC_CTRL_REG);
// Update dac-control register
    readw(dev.mmio + ME_DAC_CTRL_REG);
// Set data register
    for (i = 0; i < insn.n; i++) {
    val = data[i];
    writew(val, dev.mmio + ME_AO_DATA_REG(chan));
    }
    s.readback[chan] = val;
// Update dac with data registers
    readw(dev.mmio + ME_CTRL2_REG);
    return insn.n;
    }
    static int me2600_xilinx_download(struct comedi_device *dev,
    const u8 *data, size_t size,
    unsigned long context)
    {
    struct me_private_data *devpriv = dev.private;
    unsigned int value;
    unsigned int file_length;
    unsigned int i;
//
// Format of the firmware
// Build longs from the byte-wise coded header
// Byte 1-3:   length of the array
// Byte 4-7:   version
// Byte 8-11:  date
// Byte 12-15: reserved
//
    if (size >= 4) {
    file_length = (((unsigned int)data[0] & 0xff) << 24) +
    (((unsigned int)data[1] & 0xff) << 16) +
    (((unsigned int)data[2] & 0xff) << 8) +
    ((unsigned int)data[3] & 0xff);
    }
    if (size < 16 || file_length > size - 16) {
    dev_err(dev.class_dev, "Firmware length inconsistency\n");
    return -EINVAL;
    }
// disable irq's on PLX
    writel(0x00, devpriv.plx_regbase + PLX9052_INTCSR);
// First, make a dummy read to reset xilinx
    value = readw(dev.mmio + XILINX_DOWNLOAD_RESET);
// Wait until reset is over
    sleep(1);
// Write a dummy value to Xilinx
    writeb(0x00, dev.mmio + 0x0);
    sleep(1);
//
// Loop for writing firmware byte by byte to xilinx
// Firmware data start at offset 16
//
    for (i = 0; i < file_length; i++)
    writeb((data[16 + i] & 0xff), dev.mmio + 0x0);
// Write 5 dummy values to xilinx
    for (i = 0; i < 5; i++)
    writeb(0x00, dev.mmio + 0x0);
// Test if there was an error during download -> INTB was thrown
    value = readl(devpriv.plx_regbase + PLX9052_INTCSR);
    if (value & PLX9052_INTCSR_LI2STAT) {
// Disable interrupt
    writel(0x00, devpriv.plx_regbase + PLX9052_INTCSR);
    dev_err(dev.class_dev, "Xilinx download failed\n");
    return -EIO;
    }
// Wait until the Xilinx is ready for real work
    sleep(1);
// Enable PLX-Interrupts
    writel(PLX9052_INTCSR_LI1ENAB |
    PLX9052_INTCSR_LI1POL |
    PLX9052_INTCSR_PCIENAB,
    devpriv.plx_regbase + PLX9052_INTCSR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn me_reset(dev: *mut comedi_device) -> c_int {
    static int me_reset(struct comedi_device *dev)
    {
    struct me_private_data *devpriv = dev.private;
// Reset board
    writew(0x00, dev.mmio + ME_CTRL1_REG);
    writew(0x00, dev.mmio + ME_CTRL2_REG);
    writew(0x00, dev.mmio + ME_STATUS_REG);	/* clear interrupts */
    writew(0x00, dev.mmio + ME_DAC_CTRL_REG);
// Save values in the board context
    devpriv.dac_ctrl = 0;
    devpriv.ctrl1 = 0;
    devpriv.ctrl2 = 0;
    return 0;
    }
    static int me_auto_attach(struct comedi_device *dev,
    unsigned long context)
    {
    struct pci_dev *pcidev = comedi_to_pci_dev(dev);
    const struct me_board *board = core::ptr::null_mut();
    struct me_private_data *devpriv;
    struct comedi_subdevice *s;
    int ret;
    if (context < ARRAY_SIZE(me_boards))
    board = &me_boards[context];
    if (!board)
    return -ENODEV;
    dev.board_ptr = board;
    dev.board_name = board.name;
    devpriv = comedi_alloc_devpriv(dev, sizeof(*devpriv));
    if (!devpriv)
    return -ENOMEM;
    ret = comedi_pci_enable(dev);
    if (ret)
    return ret;
    devpriv.plx_regbase = pci_ioremap_bar(pcidev, 0);
    if (!devpriv.plx_regbase)
    return -ENOMEM;
    dev.mmio = pci_ioremap_bar(pcidev, 2);
    if (!dev.mmio)
    return -ENOMEM;
// Download firmware and reset card
    if (board.needs_firmware) {
    ret = comedi_load_firmware(dev, &comedi_to_pci_dev(dev).dev,
    ME2600_FIRMWARE,
    me2600_xilinx_download, 0);
    if (ret < 0)
    return ret;
    }
    me_reset(dev);
    ret = comedi_alloc_subdevices(dev, 3);
    if (ret)
    return ret;
    s = &dev.subdevices[0];
    s.type		= COMEDI_SUBD_AI;
    s.subdev_flags	= SDF_READABLE | SDF_COMMON | SDF_DIFF;
    s.n_chan	= 16;
    s.maxdata	= 0x0fff;
    s.len_chanlist	= 16;
    s.range_table	= &me_ai_range;
    s.insn_read	= me_ai_insn_read;
    s = &dev.subdevices[1];
    if (board.has_ao) {
    s.type		= COMEDI_SUBD_AO;
    s.subdev_flags	= SDF_WRITABLE | SDF_COMMON;
    s.n_chan	= 4;
    s.maxdata	= 0x0fff;
    s.len_chanlist	= 4;
    s.range_table	= &me_ao_range;
    s.insn_write	= me_ao_insn_write;
    ret = comedi_alloc_subdev_readback(s);
    if (ret)
    return ret;
    } else {
    s.type = COMEDI_SUBD_UNUSED;
    }
    s = &dev.subdevices[2];
    s.type		= COMEDI_SUBD_DIO;
    s.subdev_flags	= SDF_READABLE | SDF_WRITABLE;
    s.n_chan	= 32;
    s.maxdata	= 1;
    s.len_chanlist	= 32;
    s.range_table	= &range_digital;
    s.insn_bits	= me_dio_insn_bits;
    s.insn_config	= me_dio_insn_config;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn me_detach(dev: *mut comedi_device) {
    static void me_detach(struct comedi_device *dev)
    {
    struct me_private_data *devpriv = dev.private;
    if (devpriv) {
    if (dev.mmio)
    me_reset(dev);
    if (devpriv.plx_regbase)
    iounmap(devpriv.plx_regbase);
    }
    comedi_pci_detach(dev);
    }
    static struct comedi_driver me_daq_driver = {
    .driver_name	= "me_daq",
    .module		= THIS_MODULE,
    .auto_attach	= me_auto_attach,
    .detach		= me_detach,
    };
    static int me_daq_pci_probe(struct pci_dev *dev,
    const struct pci_device_id *id)
    {
    return comedi_pci_auto_config(dev, &me_daq_driver, id.driver_data);
    }
    static const struct pci_device_id me_daq_pci_table[] = {
    { PCI_VDEVICE(MEILHAUS, 0x2600), .driver_data = BOARD_ME2600 },
    { PCI_VDEVICE(MEILHAUS, 0x2000), .driver_data = BOARD_ME2000 },
    { }
    };
    MODULE_DEVICE_TABLE(pci, me_daq_pci_table);
    static struct pci_driver me_daq_pci_driver = {
    .name		= "me_daq",
    .id_table	= me_daq_pci_table,
    .probe		= me_daq_pci_probe,
    .remove		= comedi_pci_auto_unconfig,
    };
    module_comedi_pci_driver(me_daq_driver, me_daq_pci_driver);
    MODULE_AUTHOR("Comedi https://www.comedi.org");
    MODULE_DESCRIPTION("Comedi low-level driver");
    MODULE_LICENSE("GPL");
    MODULE_FIRMWARE(ME2600_FIRMWARE);
