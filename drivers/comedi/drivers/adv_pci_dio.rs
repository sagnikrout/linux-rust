//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/adv_pci_dio.c
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


// SPDX-License-Identifier: GPL-2.0
//
// comedi/drivers/adv_pci_dio.c
//
// Author: Michal Dobes <dobes@tesnet.cz>
//
// Hardware driver for Advantech PCI DIO cards.
//
// Driver: adv_pci_dio
// Description: Advantech Digital I/O Cards
// Devices: [Advantech] PCI-1730 (adv_pci_dio), PCI-1733,
// PCI-1734, PCI-1735U, PCI-1736UP, PCI-1739U, PCI-1750,
// PCI-1751, PCI-1752, PCI-1753, PCI-1753+PCI-1753E,
// PCI-1754, PCI-1756, PCI-1761, PCI-1762
// Author: Michal Dobes <dobes@tesnet.cz>
// Updated: Fri, 25 Aug 2017 07:23:06 +0300
// Status: untested
//
// Configuration Options: not applicable, uses PCI auto config
//

//
// Register offset definitions
//
// PCI-1730, PCI-1733, PCI-1736 interrupt control registers
pub const PCI173X_INT_EN_REG: c_uint = 0x0008	/* R/W: enable/disable */;
pub const PCI173X_INT_RF_REG: c_uint = 0x000c	/* R/W: falling/rising edge */;
pub const PCI173X_INT_FLAG_REG: c_uint = 0x0010	/* R: status */;
pub const PCI173X_INT_CLR_REG: c_uint = 0x0010	/* W: clear */;
pub const PCI173X_INT_IDI0: c_uint = 0x01  /* IDI0 edge occurred */;
pub const PCI173X_INT_IDI1: c_uint = 0x02  /* IDI1 edge occurred */;
pub const PCI173X_INT_DI0: c_uint = 0x04  /* DI0 edge occurred */;
pub const PCI173X_INT_DI1: c_uint = 0x08  /* DI1 edge occurred */;
// PCI-1739U, PCI-1750, PCI1751 interrupt control registers
pub const PCI1750_INT_REG: c_uint = 0x20	/* R/W: status/control */;
// PCI-1753, PCI-1753E interrupt control registers

// PCI-1754, PCI-1756 interrupt control registers

// PCI-1752, PCI-1756 special registers
pub const PCI1752_CFC_REG: c_uint = 0x12	/* R/W: channel freeze function */;
// PCI-1761 interrupt control registers
pub const PCI1761_INT_EN_REG: c_uint = 0x03	/* R/W: enable/disable interrupts */;
pub const PCI1761_INT_RF_REG: c_uint = 0x04	/* R/W: falling/rising edge */;
pub const PCI1761_INT_CLR_REG: c_uint = 0x05	/* R/W: clear interrupts */;
// PCI-1762 interrupt control registers
pub const PCI1762_INT_REG: c_uint = 0x06	/* R/W: status/control */;
// maximum number of subdevice descriptions in the boardinfo

    enum pci_dio_boardid {
    TYPE_PCI1730,
    TYPE_PCI1733,
    TYPE_PCI1734,
    TYPE_PCI1735,
    TYPE_PCI1736,
    TYPE_PCI1739,
    TYPE_PCI1750,
    TYPE_PCI1751,
    TYPE_PCI1752,
    TYPE_PCI1753,
    TYPE_PCI1753E,
    TYPE_PCI1754,
    TYPE_PCI1756,
    TYPE_PCI1761,
    TYPE_PCI1762
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct diosubd_data {
    pub /: *mut *mut int chans; / num of chans or 8255 devices,
    pub /: *mut *mut unsigned long addr; / PCI address offset,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dio_irq_subd_data {
    pub /: *mut *mut unsigned short int_en; / interrupt enable/status bit,
    pub /: *mut *mut unsigned long addr; / PCI address offset,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dio_boardtype {
    pub /: *const *const *const char name; / board name,
    pub nsubdevs: c_int,
    pub sdi: [diosubd_data; PCI_DIO_MAX_DI_SUBDEVS],
    pub sdo: [diosubd_data; PCI_DIO_MAX_DO_SUBDEVS],
    pub sdio: [diosubd_data; PCI_DIO_MAX_DIO_SUBDEVG],
    pub sdirq: [dio_irq_subd_data; PCI_DIO_MAX_IRQ_SUBDEVS],
    pub id_reg: c_ulong,
    pub timer_regbase: c_ulong,
    pub is_16bit:1: c_uint,
}

    static const struct dio_boardtype boardtypes[] = {
    [TYPE_PCI1730] = {
    .name		= "pci1730",
// DI, IDI, DO, IDO, ID, IRQ_DI0, IRQ_DI1, IRQ_IDI0, IRQ_IDI1
    .nsubdevs	= 9,
    .sdi[0]		= { 16, 0x02, },	/* DI 0-15 */
    .sdi[1]		= { 16, 0x00, },	/* ISO DI 0-15 */
    .sdo[0]		= { 16, 0x02, },	/* DO 0-15 */
    .sdo[1]		= { 16, 0x00, },	/* ISO DO 0-15 */
    .id_reg		= 0x04,
    .sdirq[0]	= { PCI173X_INT_DI0,  0x02, },	/* DI 0 */
    .sdirq[1]	= { PCI173X_INT_DI1,  0x02, },	/* DI 1 */
    .sdirq[2]	= { PCI173X_INT_IDI0, 0x00, },	/* ISO DI 0 */
    .sdirq[3]	= { PCI173X_INT_IDI1, 0x00, },	/* ISO DI 1 */
    },
    [TYPE_PCI1733] = {
    .name		= "pci1733",
    .nsubdevs	= 2,
    .sdi[1]		= { 32, 0x00, },	/* ISO DI 0-31 */
    .id_reg		= 0x04,
    },
    [TYPE_PCI1734] = {
    .name		= "pci1734",
    .nsubdevs	= 2,
    .sdo[1]		= { 32, 0x00, },	/* ISO DO 0-31 */
    .id_reg		= 0x04,
    },
    [TYPE_PCI1735] = {
    .name		= "pci1735",
    .nsubdevs	= 4,
    .sdi[0]		= { 32, 0x00, },	/* DI 0-31 */
    .sdo[0]		= { 32, 0x00, },	/* DO 0-31 */
    .id_reg		= 0x08,
    .timer_regbase	= 0x04,
    },
    [TYPE_PCI1736] = {
    .name		= "pci1736",
    .nsubdevs	= 3,
    .sdi[1]		= { 16, 0x00, },	/* ISO DI 0-15 */
    .sdo[1]		= { 16, 0x00, },	/* ISO DO 0-15 */
    .id_reg		= 0x04,
    },
    [TYPE_PCI1739] = {
    .name		= "pci1739",
    .nsubdevs	= 3,
    .sdio[0]	= { 2, 0x00, },		/* 8255 DIO */
    .id_reg		= 0x08,
    },
    [TYPE_PCI1750] = {
    .name		= "pci1750",
    .nsubdevs	= 2,
    .sdi[1]		= { 16, 0x00, },	/* ISO DI 0-15 */
    .sdo[1]		= { 16, 0x00, },	/* ISO DO 0-15 */
    },
    [TYPE_PCI1751] = {
    .name		= "pci1751",
    .nsubdevs	= 3,
    .sdio[0]	= { 2, 0x00, },		/* 8255 DIO */
    .timer_regbase	= 0x18,
    },
    [TYPE_PCI1752] = {
    .name		= "pci1752",
    .nsubdevs	= 3,
    .sdo[0]		= { 32, 0x00, },	/* DO 0-31 */
    .sdo[1]		= { 32, 0x04, },	/* DO 32-63 */
    .id_reg		= 0x10,
    .is_16bit	= 1,
    },
    [TYPE_PCI1753] = {
    .name		= "pci1753",
    .nsubdevs	= 4,
    .sdio[0]	= { 4, 0x00, },		/* 8255 DIO */
    },
    [TYPE_PCI1753E] = {
    .name		= "pci1753e",
    .nsubdevs	= 8,
    .sdio[0]	= { 4, 0x00, },		/* 8255 DIO */
    .sdio[1]	= { 4, 0x20, },		/* 8255 DIO */
    },
    [TYPE_PCI1754] = {
    .name		= "pci1754",
    .nsubdevs	= 3,
    .sdi[0]		= { 32, 0x00, },	/* DI 0-31 */
    .sdi[1]		= { 32, 0x04, },	/* DI 32-63 */
    .id_reg		= 0x10,
    .is_16bit	= 1,
    },
    [TYPE_PCI1756] = {
    .name		= "pci1756",
    .nsubdevs	= 3,
    .sdi[1]		= { 32, 0x00, },	/* DI 0-31 */
    .sdo[1]		= { 32, 0x04, },	/* DO 0-31 */
    .id_reg		= 0x10,
    .is_16bit	= 1,
    },
    [TYPE_PCI1761] = {
    .name		= "pci1761",
    .nsubdevs	= 3,
    .sdi[1]		= { 8, 0x01 },		/* ISO DI 0-7 */
    .sdo[1]		= { 8, 0x00 },		/* RELAY DO 0-7 */
    .id_reg		= 0x02,
    },
    [TYPE_PCI1762] = {
    .name		= "pci1762",
    .nsubdevs	= 3,
    .sdi[1]		= { 16, 0x02, },	/* ISO DI 0-15 */
    .sdo[1]		= { 16, 0x00, },	/* ISO DO 0-15 */
    .id_reg		= 0x04,
    .is_16bit	= 1,
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_dio_dev_private_data {
    pub boardtype: c_int,
    pub irq_subd: c_int,
    pub int_ctrl: c_ushort,
    pub int_rf: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_dio_sd_private_data {
    pub /: *mut *mut spinlock_t subd_slock; / spin-lock for cmd_running,
    pub port_offset: c_ulong,
    pub cmd_running: short int,
}

    static void process_irq(struct comedi_device *dev, unsigned int subdev,
    unsigned char irqflags)
    {
    struct comedi_subdevice *s = &dev.subdevices[subdev];
    struct pci_dio_sd_private_data *sd_priv = s.private;
    let mut reg: c_ulong = sd_priv.port_offset;
    struct comedi_async *async_p = s.async;
    if (async_p) {
    let mut val: c_ushort = inw(dev.iobase + reg);
    spin_lock(&sd_priv.subd_slock);
    if (sd_priv.cmd_running)
    comedi_buf_write_samples(s, &val, 1);
    spin_unlock(&sd_priv.subd_slock);
    comedi_handle_events(dev, s);
    }
    }
#[no_mangle]
unsafe extern "C" fn pci_dio_interrupt(irq: c_int, p_device: *mut c_void) -> irqreturn_t {
    static irqreturn_t pci_dio_interrupt(int irq, void *p_device)
    {
    struct comedi_device *dev = p_device;
    struct pci_dio_dev_private_data *dev_private = dev.private;
    const struct dio_boardtype *board = dev.board_ptr;
    unsigned long cpu_flags;
    unsigned char irqflags;
    int i;
    if (!dev.attached) {
// Ignore interrupt before device fully attached.
// Might not even have allocated subdevices yet!
    return IRQ_NONE;
    }
// Check if we are source of interrupt
    spin_lock_irqsave(&dev.spinlock, cpu_flags);
    irqflags = inb(dev.iobase + PCI173X_INT_FLAG_REG);
    if (!(irqflags & 0x0F)) {
    spin_unlock_irqrestore(&dev.spinlock, cpu_flags);
    return IRQ_NONE;
    }
// clear all current interrupt flags
    outb(irqflags, dev.iobase + PCI173X_INT_CLR_REG);
    spin_unlock_irqrestore(&dev.spinlock, cpu_flags);
// check irq subdevice triggers
    for (i = 0; i < PCI_DIO_MAX_IRQ_SUBDEVS; i++) {
    if (irqflags & board.sdirq[i].int_en)
    process_irq(dev, dev_private.irq_subd + i, irqflags);
    }
    return IRQ_HANDLED;
    }
    static int pci_dio_asy_cmdtest(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_cmd *cmd)
    {
    let mut err: c_int = 0;
// Step 1 : check if triggers are trivially valid
    err |= comedi_check_trigger_src(&cmd.start_src, TRIG_NOW);
    err |= comedi_check_trigger_src(&cmd.scan_begin_src, TRIG_EXT);
    err |= comedi_check_trigger_src(&cmd.convert_src, TRIG_FOLLOW);
    err |= comedi_check_trigger_src(&cmd.scan_end_src, TRIG_COUNT);
    err |= comedi_check_trigger_src(&cmd.stop_src, TRIG_NONE);
    if (err)
    return 1;
// Step 2a : make sure trigger sources are unique
// Step 2b : and mutually compatible
// Step 3: check if arguments are trivially valid
    err |= comedi_check_trigger_arg_is(&cmd.start_arg, 0);
//
// For scan_begin_arg, the trigger number must be 0 and the only
// allowed flags are CR_EDGE and CR_INVERT.  CR_EDGE is ignored,
// CR_INVERT sets the trigger to falling edge.
//
    if (cmd.scan_begin_arg & ~(CR_EDGE | CR_INVERT)) {
    cmd.scan_begin_arg &= (CR_EDGE | CR_INVERT);
    err |= -EINVAL;
    }
    err |= comedi_check_trigger_arg_is(&cmd.convert_arg, 0);
    err |= comedi_check_trigger_arg_is(&cmd.scan_end_arg,
    cmd.chanlist_len);
    err |= comedi_check_trigger_arg_is(&cmd.stop_arg, 0);
    if (err)
    return 3;
// Step 4: fix up any arguments
// Step 5: check channel list if it exists
    return 0;
    }
    static int pci_dio_asy_cmd(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    struct pci_dio_dev_private_data *dev_private = dev.private;
    struct pci_dio_sd_private_data *sd_priv = s.private;
    const struct dio_boardtype *board = dev.board_ptr;
    struct comedi_cmd *cmd = &s.async.cmd;
    unsigned long cpu_flags;
    unsigned short int_en;
    int_en = board.sdirq[s.index - dev_private.irq_subd].int_en;
    spin_lock_irqsave(&dev.spinlock, cpu_flags);
    if (cmd.scan_begin_arg & CR_INVERT)
    dev_private.int_rf |= int_en;	/* falling edge */
    else
    dev_private.int_rf &= ~int_en;	/* rising edge */
    outb(dev_private.int_rf, dev.iobase + PCI173X_INT_RF_REG);
    dev_private.int_ctrl |= int_en;	/* enable interrupt source */
    outb(dev_private.int_ctrl, dev.iobase + PCI173X_INT_EN_REG);
    spin_unlock_irqrestore(&dev.spinlock, cpu_flags);
    spin_lock_irqsave(&sd_priv.subd_slock, cpu_flags);
    sd_priv.cmd_running = 1;
    spin_unlock_irqrestore(&sd_priv.subd_slock, cpu_flags);
    return 0;
    }
    static int pci_dio_asy_cancel(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    struct pci_dio_dev_private_data *dev_private = dev.private;
    struct pci_dio_sd_private_data *sd_priv = s.private;
    const struct dio_boardtype *board = dev.board_ptr;
    unsigned long cpu_flags;
    unsigned short int_en;
    spin_lock_irqsave(&sd_priv.subd_slock, cpu_flags);
    sd_priv.cmd_running = 0;
    spin_unlock_irqrestore(&sd_priv.subd_slock, cpu_flags);
    int_en = board.sdirq[s.index - dev_private.irq_subd].int_en;
    spin_lock_irqsave(&dev.spinlock, cpu_flags);
    dev_private.int_ctrl &= ~int_en;
    outb(dev_private.int_ctrl, dev.iobase + PCI173X_INT_EN_REG);
    spin_unlock_irqrestore(&dev.spinlock, cpu_flags);
    return 0;
    }
// same as _insn_bits_di_ because the IRQ-pins are the DI-ports
    static int pci_dio_insn_bits_dirq_b(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct pci_dio_sd_private_data *sd_priv = s.private;
    let mut reg: c_ulong = (unsigned long)sd_priv.port_offset;
    let mut iobase: c_ulong = dev.iobase + reg;
    data[1] = inb(iobase);
    return insn.n;
    }
    static int pci_dio_insn_bits_di_b(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut reg: c_ulong = (unsigned long)s.private;
    let mut iobase: c_ulong = dev.iobase + reg;
    data[1] = inb(iobase);
    if (s.n_chan > 8)
    data[1] |= (inb(iobase + 1) << 8);
    if (s.n_chan > 16)
    data[1] |= (inb(iobase + 2) << 16);
    if (s.n_chan > 24)
    data[1] |= (inb(iobase + 3) << 24);
    return insn.n;
    }
    static int pci_dio_insn_bits_di_w(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut reg: c_ulong = (unsigned long)s.private;
    let mut iobase: c_ulong = dev.iobase + reg;
    data[1] = inw(iobase);
    if (s.n_chan > 16)
    data[1] |= (inw(iobase + 2) << 16);
    return insn.n;
    }
    static int pci_dio_insn_bits_do_b(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut reg: c_ulong = (unsigned long)s.private;
    let mut iobase: c_ulong = dev.iobase + reg;
    if (comedi_dio_update_state(s, data)) {
    outb(s.state & 0xff, iobase);
    if (s.n_chan > 8)
    outb((s.state >> 8) & 0xff, iobase + 1);
    if (s.n_chan > 16)
    outb((s.state >> 16) & 0xff, iobase + 2);
    if (s.n_chan > 24)
    outb((s.state >> 24) & 0xff, iobase + 3);
    }
    data[1] = s.state;
    return insn.n;
    }
    static int pci_dio_insn_bits_do_w(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut reg: c_ulong = (unsigned long)s.private;
    let mut iobase: c_ulong = dev.iobase + reg;
    if (comedi_dio_update_state(s, data)) {
    outw(s.state & 0xffff, iobase);
    if (s.n_chan > 16)
    outw((s.state >> 16) & 0xffff, iobase + 2);
    }
    data[1] = s.state;
    return insn.n;
    }
#[no_mangle]
unsafe extern "C" fn pci_dio_reset(dev: *mut comedi_device, cardtype: c_ulong) -> c_int {
    static int pci_dio_reset(struct comedi_device *dev, unsigned long cardtype)
    {
    struct pci_dio_dev_private_data *dev_private = dev.private;
// disable channel freeze function on the PCI-1752/1756 boards
    if (cardtype == TYPE_PCI1752 || cardtype == TYPE_PCI1756)
    outw(0, dev.iobase + PCI1752_CFC_REG);
// disable and clear interrupts
    switch (cardtype) {
    case TYPE_PCI1730:
    case TYPE_PCI1733:
    case TYPE_PCI1736:
    dev_private.int_ctrl = 0x00;
    outb(dev_private.int_ctrl, dev.iobase + PCI173X_INT_EN_REG);
// Reset all 4 Int Flags
    outb(0x0f, dev.iobase + PCI173X_INT_CLR_REG);
// Rising Edge => IRQ . On all 4 Pins
    dev_private.int_rf = 0x00;
    outb(dev_private.int_rf, dev.iobase + PCI173X_INT_RF_REG);
    break;
    case TYPE_PCI1739:
    case TYPE_PCI1750:
    case TYPE_PCI1751:
    outb(0x88, dev.iobase + PCI1750_INT_REG);
    break;
    case TYPE_PCI1753:
    case TYPE_PCI1753E:
    outb(0x88, dev.iobase + PCI1753_INT_REG(0));
    outb(0x80, dev.iobase + PCI1753_INT_REG(1));
    outb(0x80, dev.iobase + PCI1753_INT_REG(2));
    outb(0x80, dev.iobase + PCI1753_INT_REG(3));
    if (cardtype == TYPE_PCI1753E) {
    outb(0x88, dev.iobase + PCI1753E_INT_REG(0));
    outb(0x80, dev.iobase + PCI1753E_INT_REG(1));
    outb(0x80, dev.iobase + PCI1753E_INT_REG(2));
    outb(0x80, dev.iobase + PCI1753E_INT_REG(3));
    }
    break;
    case TYPE_PCI1754:
    case TYPE_PCI1756:
    outw(0x08, dev.iobase + PCI1754_INT_REG(0));
    outw(0x08, dev.iobase + PCI1754_INT_REG(1));
    if (cardtype == TYPE_PCI1754) {
    outw(0x08, dev.iobase + PCI1754_INT_REG(2));
    outw(0x08, dev.iobase + PCI1754_INT_REG(3));
    }
    break;
    case TYPE_PCI1761:
// disable interrupts
    outb(0, dev.iobase + PCI1761_INT_EN_REG);
// clear interrupts
    outb(0xff, dev.iobase + PCI1761_INT_CLR_REG);
// set rising edge trigger
    outb(0, dev.iobase + PCI1761_INT_RF_REG);
    break;
    case TYPE_PCI1762:
    outw(0x0101, dev.iobase + PCI1762_INT_REG);
    break;
    default:
    break;
    }
    return 0;
    }
    static int pci_dio_auto_attach(struct comedi_device *dev,
    unsigned long context)
    {
    struct pci_dev *pcidev = comedi_to_pci_dev(dev);
    const struct dio_boardtype *board = core::ptr::null_mut();
    struct comedi_subdevice *s;
    struct pci_dio_dev_private_data *dev_private;
    int ret, subdev, i, j;
    if (context < ARRAY_SIZE(boardtypes))
    board = &boardtypes[context];
    if (!board)
    return -ENODEV;
    dev.board_ptr = board;
    dev.board_name = board.name;
    dev_private = comedi_alloc_devpriv(dev, sizeof(*dev_private));
    if (!dev_private)
    return -ENOMEM;
    ret = comedi_pci_enable(dev);
    if (ret)
    return ret;
    if (context == TYPE_PCI1736)
    dev.iobase = pci_resource_start(pcidev, 0);
    else
    dev.iobase = pci_resource_start(pcidev, 2);
    dev_private.boardtype = context;
    pci_dio_reset(dev, context);
// request IRQ if device has irq subdevices
    if (board.sdirq[0].int_en && pcidev.irq) {
    ret = request_irq(pcidev.irq, pci_dio_interrupt, IRQF_SHARED,
    dev.board_name, dev);
    if (ret == 0)
    dev.irq = pcidev.irq;
    }
    ret = comedi_alloc_subdevices(dev, board.nsubdevs);
    if (ret)
    return ret;
    subdev = 0;
    for (i = 0; i < PCI_DIO_MAX_DI_SUBDEVS; i++) {
    const struct diosubd_data *d = &board.sdi[i];
    if (d.chans) {
    s = &dev.subdevices[subdev++];
    s.type		= COMEDI_SUBD_DI;
    s.subdev_flags	= SDF_READABLE;
    s.n_chan	= d.chans;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= board.is_16bit
    ? pci_dio_insn_bits_di_w
    : pci_dio_insn_bits_di_b;
    s.private	= (void *)d.addr;
    }
    }
    for (i = 0; i < PCI_DIO_MAX_DO_SUBDEVS; i++) {
    const struct diosubd_data *d = &board.sdo[i];
    if (d.chans) {
    s = &dev.subdevices[subdev++];
    s.type		= COMEDI_SUBD_DO;
    s.subdev_flags	= SDF_WRITABLE;
    s.n_chan	= d.chans;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= board.is_16bit
    ? pci_dio_insn_bits_do_w
    : pci_dio_insn_bits_do_b;
    s.private	= (void *)d.addr;
// reset all outputs to 0
    if (board.is_16bit) {
    outw(0, dev.iobase + d.addr);
    if (s.n_chan > 16)
    outw(0, dev.iobase + d.addr + 2);
    } else {
    outb(0, dev.iobase + d.addr);
    if (s.n_chan > 8)
    outb(0, dev.iobase + d.addr + 1);
    if (s.n_chan > 16)
    outb(0, dev.iobase + d.addr + 2);
    if (s.n_chan > 24)
    outb(0, dev.iobase + d.addr + 3);
    }
    }
    }
    for (i = 0; i < PCI_DIO_MAX_DIO_SUBDEVG; i++) {
    const struct diosubd_data *d = &board.sdio[i];
    for (j = 0; j < d.chans; j++) {
    s = &dev.subdevices[subdev++];
    ret = subdev_8255_io_init(dev, s,
    d.addr + j * I8255_SIZE);
    if (ret)
    return ret;
    }
    }
    if (board.id_reg) {
    s = &dev.subdevices[subdev++];
    s.type		= COMEDI_SUBD_DI;
    s.subdev_flags	= SDF_READABLE | SDF_INTERNAL;
    s.n_chan	= 4;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= board.is_16bit ? pci_dio_insn_bits_di_w
    : pci_dio_insn_bits_di_b;
    s.private	= (void *)board.id_reg;
    }
    if (board.timer_regbase) {
    s = &dev.subdevices[subdev++];
    dev.pacer =
    comedi_8254_io_alloc(dev.iobase + board.timer_regbase,
    0, I8254_IO8, 0);
    if (IS_ERR(dev.pacer))
    return PTR_ERR(dev.pacer);
    comedi_8254_subdevice_init(s, dev.pacer);
    }
    dev_private.irq_subd = subdev; /* first interrupt subdevice index */
    for (i = 0; i < PCI_DIO_MAX_IRQ_SUBDEVS; ++i) {
    struct pci_dio_sd_private_data *sd_priv = core::ptr::null_mut();
    const struct dio_irq_subd_data *d = &board.sdirq[i];
    if (d.int_en) {
    s = &dev.subdevices[subdev++];
    s.type		= COMEDI_SUBD_DI;
    s.subdev_flags	= SDF_READABLE;
    s.n_chan	= 1;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= pci_dio_insn_bits_dirq_b;
    sd_priv = comedi_alloc_spriv(s, sizeof(*sd_priv));
    if (!sd_priv)
    return -ENOMEM;
    spin_lock_init(&sd_priv.subd_slock);
    sd_priv.port_offset = d.addr;
    sd_priv.cmd_running = 0;
    if (dev.irq) {
    dev.read_subdev = s;
    s.type		= COMEDI_SUBD_DI;
    s.subdev_flags	= SDF_READABLE | SDF_CMD_READ;
    s.len_chanlist	= 1;
    s.do_cmdtest	= pci_dio_asy_cmdtest;
    s.do_cmd	= pci_dio_asy_cmd;
    s.cancel	= pci_dio_asy_cancel;
    }
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci_dio_detach(dev: *mut comedi_device) {
    static void pci_dio_detach(struct comedi_device *dev)
    {
    struct pci_dio_dev_private_data *dev_private = dev.private;
    let mut boardtype: c_int = dev_private.boardtype;
    if (dev.iobase)
    pci_dio_reset(dev, boardtype);
    comedi_pci_detach(dev);
    }
    static struct comedi_driver adv_pci_dio_driver = {
    .driver_name	= "adv_pci_dio",
    .module		= THIS_MODULE,
    .auto_attach	= pci_dio_auto_attach,
    .detach		= pci_dio_detach,
    };
    static unsigned long pci_dio_override_cardtype(struct pci_dev *pcidev,
    unsigned long cardtype)
    {
//
// Change cardtype from TYPE_PCI1753 to TYPE_PCI1753E if expansion
// board available.  Need to enable PCI device and request the main
// registers PCI BAR temporarily to perform the test.
//
    if (cardtype != TYPE_PCI1753)
    return cardtype;
    if (pci_enable_device(pcidev) < 0)
    return cardtype;
    if (pci_request_region(pcidev, 2, "adv_pci_dio") == 0) {
//
// This test is based on Advantech's "advdaq" driver source
// (which declares its module licence as "GPL" although the
// driver source does not include a "COPYING" file).
//
    let mut reg: c_ulong = pci_resource_start(pcidev, 2) + 53;
    outb(0x05, reg);
    if ((inb(reg) & 0x07) == 0x02) {
    outb(0x02, reg);
    if ((inb(reg) & 0x07) == 0x05)
    cardtype = TYPE_PCI1753E;
    }
    pci_release_region(pcidev, 2);
    }
    pci_disable_device(pcidev);
    return cardtype;
    }
    static int adv_pci_dio_pci_probe(struct pci_dev *dev,
    const struct pci_device_id *id)
    {
    unsigned long cardtype;
    cardtype = pci_dio_override_cardtype(dev, id.driver_data);
    return comedi_pci_auto_config(dev, &adv_pci_dio_driver, cardtype);
    }
    static const struct pci_device_id adv_pci_dio_pci_table[] = {
    { PCI_VDEVICE(ADVANTECH, 0x1730), .driver_data = TYPE_PCI1730 },
    { PCI_VDEVICE(ADVANTECH, 0x1733), .driver_data = TYPE_PCI1733 },
    { PCI_VDEVICE(ADVANTECH, 0x1734), .driver_data = TYPE_PCI1734 },
    { PCI_VDEVICE(ADVANTECH, 0x1735), .driver_data = TYPE_PCI1735 },
    { PCI_VDEVICE(ADVANTECH, 0x1736), .driver_data = TYPE_PCI1736 },
    { PCI_VDEVICE(ADVANTECH, 0x1739), .driver_data = TYPE_PCI1739 },
    { PCI_VDEVICE(ADVANTECH, 0x1750), .driver_data = TYPE_PCI1750 },
    { PCI_VDEVICE(ADVANTECH, 0x1751), .driver_data = TYPE_PCI1751 },
    { PCI_VDEVICE(ADVANTECH, 0x1752), .driver_data = TYPE_PCI1752 },
    { PCI_VDEVICE(ADVANTECH, 0x1753), .driver_data = TYPE_PCI1753 },
    { PCI_VDEVICE(ADVANTECH, 0x1754), .driver_data = TYPE_PCI1754 },
    { PCI_VDEVICE(ADVANTECH, 0x1756), .driver_data = TYPE_PCI1756 },
    { PCI_VDEVICE(ADVANTECH, 0x1761), .driver_data = TYPE_PCI1761 },
    { PCI_VDEVICE(ADVANTECH, 0x1762), .driver_data = TYPE_PCI1762 },
    { }
    };
    MODULE_DEVICE_TABLE(pci, adv_pci_dio_pci_table);
    static struct pci_driver adv_pci_dio_pci_driver = {
    .name		= "adv_pci_dio",
    .id_table	= adv_pci_dio_pci_table,
    .probe		= adv_pci_dio_pci_probe,
    .remove		= comedi_pci_auto_unconfig,
    };
    module_comedi_pci_driver(adv_pci_dio_driver, adv_pci_dio_pci_driver);
    MODULE_AUTHOR("Comedi https://www.comedi.org");
    MODULE_DESCRIPTION("Comedi driver for Advantech Digital I/O Cards");
    MODULE_LICENSE("GPL");
