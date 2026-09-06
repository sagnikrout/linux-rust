//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/dm1105/dm1105.c
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
// dm1105.c - driver for DVB cards based on SDMC DM1105 PCI chip
//
// Copyright (C) 2008 Igor M. Liplianin <liplianin@me.by>
//

pub const DM1105_BOARD_UNKNOWN: c_int = 0;
pub const DM1105_BOARD_DVBWORLD_2002: c_int = 1;
pub const DM1105_BOARD_DVBWORLD_2004: c_int = 2;
pub const DM1105_BOARD_AXESS_DM05: c_int = 3;
pub const DM1105_BOARD_UNBRANDED_I2C_ON_GPIO: c_int = 4;
// -----------------------------------------------
//
// PCI ID's
//

pub const PCI_VENDOR_ID_TRIGEM: c_uint = 0x109f;

pub const PCI_VENDOR_ID_AXESS: c_uint = 0x195d;

pub const PCI_DEVICE_ID_DM1105: c_uint = 0x036f;

pub const PCI_DEVICE_ID_DW2002: c_uint = 0x2002;

pub const PCI_DEVICE_ID_DW2004: c_uint = 0x2004;

pub const PCI_DEVICE_ID_DM05: c_uint = 0x1105;

// -----------------------------------------------
// sdmc dm1105 registers
// TS Control
pub const DM1105_TSCTR: c_uint = 0x00;
pub const DM1105_DTALENTH: c_uint = 0x04;
// GPIO Interface
pub const DM1105_GPIOVAL: c_uint = 0x08;
pub const DM1105_GPIOCTR: c_uint = 0x0c;
// PID serial number
pub const DM1105_PIDN: c_uint = 0x10;
// Odd-even secret key select
pub const DM1105_CWSEL: c_uint = 0x14;
// Host Command Interface
pub const DM1105_HOST_CTR: c_uint = 0x18;
pub const DM1105_HOST_AD: c_uint = 0x1c;
// PCI Interface
pub const DM1105_CR: c_uint = 0x30;
pub const DM1105_RST: c_uint = 0x34;
pub const DM1105_STADR: c_uint = 0x38;
pub const DM1105_RLEN: c_uint = 0x3c;
pub const DM1105_WRP: c_uint = 0x40;
pub const DM1105_INTCNT: c_uint = 0x44;
pub const DM1105_INTMAK: c_uint = 0x48;
pub const DM1105_INTSTS: c_uint = 0x4c;
// CW Value
pub const DM1105_ODD: c_uint = 0x50;
pub const DM1105_EVEN: c_uint = 0x58;
// PID Value
pub const DM1105_PID: c_uint = 0x60;
// IR Control
pub const DM1105_IRCTR: c_uint = 0x64;
pub const DM1105_IRMODE: c_uint = 0x68;
pub const DM1105_SYSTEMCODE: c_uint = 0x6c;
pub const DM1105_IRCODE: c_uint = 0x70;
// Unknown Values
pub const DM1105_ENCRYPT: c_uint = 0x74;
pub const DM1105_VER: c_uint = 0x7c;
// I2C Interface
pub const DM1105_I2CCTR: c_uint = 0x80;
pub const DM1105_I2CSTS: c_uint = 0x81;
pub const DM1105_I2CDAT: c_uint = 0x82;
pub const DM1105_I2C_RA: c_uint = 0x83;
// -----------------------------------------------
// Interrupt Mask Bits
pub const INTMAK_TSIRQM: c_uint = 0x01;
pub const INTMAK_HIRQM: c_uint = 0x04;
pub const INTMAK_IRM: c_uint = 0x08;

    INTMAK_HIRQM | \
    INTMAK_IRM)
pub const INTMAK_NONEMASK: c_uint = 0x00;
// Interrupt Status Bits
pub const INTSTS_TSIRQ: c_uint = 0x01;
pub const INTSTS_HIRQ: c_uint = 0x04;
pub const INTSTS_IR: c_uint = 0x08;
// IR Control Bits
pub const DM1105_IR_EN: c_uint = 0x01;
pub const DM1105_SYS_CHK: c_uint = 0x02;
pub const DM1105_REP_FLG: c_uint = 0x08;
// EEPROM addr
pub const IIC_24C01_addr: c_uint = 0xa0;
// Max board count
pub const DM1105_MAX: c_uint = 0x04;

pub const DM1105_DMA_PACKETS: c_int = 47;

//

pub const GPIO_ALL: c_uint = 0x03ffff;
// GPIO's for LNB power control

// GPIO's for LNB power control for Axess DM05

// GPIO's for LNB power control for unbranded with I2C on GPIO

pub const UNBR_LNB_OFF: c_int = 0;

    static unsigned int card[]  = {[0 ... 3] = UNSET };
    module_param_array(card,  int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(card, "card type");
    static int ir_debug;
    module_param(ir_debug, int, 0644);
    MODULE_PARM_DESC(ir_debug, "enable debugging information for IR decoding");
    static unsigned int dm1105_devcount;
    DVB_DEFINE_MOD_OPT_ADAPTER_NR(adapter_nr);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm1105_board {
    pub name: *mut c_char,
    struct	{
    pub v18: u32 mask, off, v13,,
    pub lnb: },
    pub gpio_sda: u32 gpio_scl,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm1105_subid {
    pub subvendor: u16,
    pub subdevice: u16,
    pub card: u32,
}

    static const struct dm1105_board dm1105_boards[] = {
    [DM1105_BOARD_UNKNOWN] = {
    .name		= "UNKNOWN/GENERIC",
    .lnb = {
    .mask = DM1105_LNB_MASK,
    .off = DM1105_LNB_OFF,
    .v13 = DM1105_LNB_13V,
    .v18 = DM1105_LNB_18V,
    },
    },
    [DM1105_BOARD_DVBWORLD_2002] = {
    .name		= "DVBWorld PCI 2002",
    .lnb = {
    .mask = DM1105_LNB_MASK,
    .off = DM1105_LNB_OFF,
    .v13 = DM1105_LNB_13V,
    .v18 = DM1105_LNB_18V,
    },
    },
    [DM1105_BOARD_DVBWORLD_2004] = {
    .name		= "DVBWorld PCI 2004",
    .lnb = {
    .mask = DM1105_LNB_MASK,
    .off = DM1105_LNB_OFF,
    .v13 = DM1105_LNB_13V,
    .v18 = DM1105_LNB_18V,
    },
    },
    [DM1105_BOARD_AXESS_DM05] = {
    .name		= "Axess/EasyTv DM05",
    .lnb = {
    .mask = DM05_LNB_MASK,
    .off = DM05_LNB_OFF,
    .v13 = DM05_LNB_13V,
    .v18 = DM05_LNB_18V,
    },
    },
    [DM1105_BOARD_UNBRANDED_I2C_ON_GPIO] = {
    .name		= "Unbranded DM1105 with i2c on GPIOs",
    .lnb = {
    .mask = UNBR_LNB_MASK,
    .off = UNBR_LNB_OFF,
    .v13 = UNBR_LNB_13V,
    .v18 = UNBR_LNB_18V,
    },
    .gpio_scl	= GPIO14,
    .gpio_sda	= GPIO13,
    },
    };
    static const struct dm1105_subid dm1105_subids[] = {
    {
    .subvendor = 0x0000,
    .subdevice = 0x2002,
    .card      = DM1105_BOARD_DVBWORLD_2002,
    }, {
    .subvendor = 0x0001,
    .subdevice = 0x2002,
    .card      = DM1105_BOARD_DVBWORLD_2002,
    }, {
    .subvendor = 0x0000,
    .subdevice = 0x2004,
    .card      = DM1105_BOARD_DVBWORLD_2004,
    }, {
    .subvendor = 0x0001,
    .subdevice = 0x2004,
    .card      = DM1105_BOARD_DVBWORLD_2004,
    }, {
    .subvendor = 0x195d,
    .subdevice = 0x1105,
    .card      = DM1105_BOARD_AXESS_DM05,
    },
    };
#[no_mangle]
unsafe extern "C" fn dm1105_card_list(pci: *mut pci_dev) {
    static void dm1105_card_list(struct pci_dev *pci)
    {
    int i;
    if (0 == pci.subsystem_vendor &&
    0 == pci.subsystem_device) {
    printk(KERN_ERR
    "dm1105: Your board has no valid PCI Subsystem ID\n"
    "dm1105: and thus can't be autodetected\n"
    "dm1105: Please pass card=<n> insmod option to\n"
    "dm1105: workaround that.  Redirect complaints to\n"
    "dm1105: the vendor of the TV card.  Best regards,\n"
    "dm1105: -- tux\n");
    } else {
    printk(KERN_ERR
    "dm1105: Your board isn't known (yet) to the driver.\n"
    "dm1105: You can try to pick one of the existing\n"
    "dm1105: card configs via card=<n> insmod option.\n"
    "dm1105: Updating to the latest version might help\n"
    "dm1105: as well.\n");
    }
    printk(KERN_ERR "Here is a list of valid choices for the card=<n> insmod option:\n");
    for (i = 0; i < ARRAY_SIZE(dm1105_boards); i++)
    printk(KERN_ERR "dm1105:    card=%d . %s\n",
    i, dm1105_boards[i].name);
    }
// infrared remote control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct infrared {
    pub dev: *mut rc_dev,
    pub input_phys: [c_char; 32],
    pub work: work_struct,
    pub ir_command: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm1105_dev {
// pci
    pub pdev: *mut pci_dev,
    pub io_mem: *mut u8 __iomem,
// ir
    pub ir: infrared,
// dvb
    pub hw_frontend: dmx_frontend,
    pub mem_frontend: dmx_frontend,
    pub dmxdev: dmxdev,
    pub dvb_adapter: dvb_adapter,
    pub demux: dvb_demux,
    pub fe: *mut dvb_frontend,
    pub dvbnet: dvb_net,
    pub full_ts_users: c_uint,
    pub boardnr: c_uint,
    pub nr: c_int,
// i2c
    pub i2c_adap: i2c_adapter,
    pub i2c_bb_adap: i2c_adapter,
    pub i2c_bit: i2c_algo_bit_data,
// irq
    pub work: work_struct,
    pub wq: *mut workqueue_struct,
    pub wqn: [c_char; 16],
// dma
    pub dma_addr: dma_addr_t,
    pub ts_buf: *mut c_uchar,
    pub wrp: u32,
    pub nextwrp: u32,
    pub buffer_size: u32,
    pub PacketErrorCount: c_uint,
    pub dmarst: c_uint,
    pub lock: spinlock_t,
}

    outl((inl(dm_io_mem(reg)) & ~(mask)) |\
    ((value) & (mask)), (dm_io_mem(reg)))

// The chip has 18 GPIOs. In HOST mode GPIO's used as 15 bit address lines,
    so we can use only 3 GPIO's from GPIO15 to GPIO17.
    Here I don't check whether HOST is enebled as it is not implemented yet.
//
#[no_mangle]
unsafe extern "C" fn dm1105_gpio_set(dev: *mut dm1105_dev, mask: u32) {
    static void dm1105_gpio_set(struct dm1105_dev *dev, u32 mask)
    {
    if (mask & 0xfffc0000)
    printk(KERN_ERR "%s: Only 18 GPIO's are allowed\n", __func__);
    if (mask & 0x0003ffff)
    dm_setl(DM1105_GPIOVAL, mask & 0x0003ffff);
    }
#[no_mangle]
unsafe extern "C" fn dm1105_gpio_clear(dev: *mut dm1105_dev, mask: u32) {
    static void dm1105_gpio_clear(struct dm1105_dev *dev, u32 mask)
    {
    if (mask & 0xfffc0000)
    printk(KERN_ERR "%s: Only 18 GPIO's are allowed\n", __func__);
    if (mask & 0x0003ffff)
    dm_clearl(DM1105_GPIOVAL, mask & 0x0003ffff);
    }
#[no_mangle]
unsafe extern "C" fn dm1105_gpio_andor(dev: *mut dm1105_dev, mask: u32, val: u32) {
    static void dm1105_gpio_andor(struct dm1105_dev *dev, u32 mask, u32 val)
    {
    if (mask & 0xfffc0000)
    printk(KERN_ERR "%s: Only 18 GPIO's are allowed\n", __func__);
    if (mask & 0x0003ffff)
    dm_andorl(DM1105_GPIOVAL, mask & 0x0003ffff, val);
    }
#[no_mangle]
unsafe extern "C" fn dm1105_gpio_get(dev: *mut dm1105_dev, mask: u32) -> u32 {
    static u32 dm1105_gpio_get(struct dm1105_dev *dev, u32 mask)
    {
    if (mask & 0xfffc0000)
    printk(KERN_ERR "%s: Only 18 GPIO's are allowed\n", __func__);
    if (mask & 0x0003ffff)
    return dm_readl(DM1105_GPIOVAL) & mask & 0x0003ffff;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dm1105_gpio_enable(dev: *mut dm1105_dev, mask: u32, asoutput: c_int) {
    static void dm1105_gpio_enable(struct dm1105_dev *dev, u32 mask, int asoutput)
    {
    if (mask & 0xfffc0000)
    printk(KERN_ERR "%s: Only 18 GPIO's are allowed\n", __func__);
    if ((mask & 0x0003ffff) && asoutput)
    dm_clearl(DM1105_GPIOCTR, mask & 0x0003ffff);
#[no_mangle]
pub unsafe extern "C" fn if(!asoutput: (mask & 0x0003ffff) &&) -> else {
    else if ((mask & 0x0003ffff) && !asoutput)
    dm_setl(DM1105_GPIOCTR, mask & 0x0003ffff);
    }
#[no_mangle]
unsafe extern "C" fn dm1105_setline(dev: *mut dm1105_dev, line: u32, state: c_int) {
    static void dm1105_setline(struct dm1105_dev *dev, u32 line, int state)
    {
    if (state)
    dm1105_gpio_enable(dev, line, 0);
    else {
    dm1105_gpio_enable(dev, line, 1);
    dm1105_gpio_clear(dev, line);
    }
    }
#[no_mangle]
unsafe extern "C" fn dm1105_setsda(data: *mut c_void, state: c_int) {
    static void dm1105_setsda(void *data, int state)
    {
    struct dm1105_dev *dev = data;
    dm1105_setline(dev, dm1105_boards[dev.boardnr].gpio_sda, state);
    }
#[no_mangle]
unsafe extern "C" fn dm1105_setscl(data: *mut c_void, state: c_int) {
    static void dm1105_setscl(void *data, int state)
    {
    struct dm1105_dev *dev = data;
    dm1105_setline(dev, dm1105_boards[dev.boardnr].gpio_scl, state);
    }
#[no_mangle]
unsafe extern "C" fn dm1105_getsda(data: *mut c_void) -> c_int {
    static int dm1105_getsda(void *data)
    {
    struct dm1105_dev *dev = data;
#[no_mangle]
pub unsafe extern "C" fn dm1105_gpio_get(_arg: dev, _arg: dm1105_boards[dev->boardnr].gpio_sda) -> return {
    return dm1105_gpio_get(dev, dm1105_boards[dev.boardnr].gpio_sda)
    ? 1 : 0;
    }
#[no_mangle]
unsafe extern "C" fn dm1105_getscl(data: *mut c_void) -> c_int {
    static int dm1105_getscl(void *data)
    {
    struct dm1105_dev *dev = data;
#[no_mangle]
pub unsafe extern "C" fn dm1105_gpio_get(_arg: dev, _arg: dm1105_boards[dev->boardnr].gpio_scl) -> return {
    return dm1105_gpio_get(dev, dm1105_boards[dev.boardnr].gpio_scl)
    ? 1 : 0;
    }
    static int dm1105_i2c_xfer(struct i2c_adapter *i2c_adap,
    struct i2c_msg *msgs, int num)
    {
    struct dm1105_dev *dev ;
    int addr, rc, i, j, k, len, byte, data;
    u8 status;
    dev = i2c_adap.algo_data;
    for (i = 0; i < num; i++) {
    dm_writeb(DM1105_I2CCTR, 0x00);
    if (msgs[i].flags & I2C_M_RD) {
// read bytes
    addr  = msgs[i].addr << 1;
    addr |= 1;
    dm_writeb(DM1105_I2CDAT, addr);
    for (byte = 0; byte < msgs[i].len; byte++)
    dm_writeb(DM1105_I2CDAT + byte + 1, 0);
    dm_writeb(DM1105_I2CCTR, 0x81 + msgs[i].len);
    for (j = 0; j < 55; j++) {
    mdelay(10);
    status = dm_readb(DM1105_I2CSTS);
    if ((status & 0xc0) == 0x40)
    break;
    }
    if (j >= 55)
    return -1;
    for (byte = 0; byte < msgs[i].len; byte++) {
    rc = dm_readb(DM1105_I2CDAT + byte + 1);
    if (rc < 0)
    goto err;
    msgs[i].buf[byte] = rc;
    }
    } else if ((msgs[i].buf[0] == 0xf7) && (msgs[i].addr == 0x55)) {
// prepared for cx24116 firmware
// Write in small blocks
    len = msgs[i].len - 1;
    k = 1;
    do {
    dm_writeb(DM1105_I2CDAT, msgs[i].addr << 1);
    dm_writeb(DM1105_I2CDAT + 1, 0xf7);
    for (byte = 0; byte < (len > 48 ? 48 : len); byte++) {
    data = msgs[i].buf[k + byte];
    dm_writeb(DM1105_I2CDAT + byte + 2, data);
    }
    dm_writeb(DM1105_I2CCTR, 0x82 + (len > 48 ? 48 : len));
    for (j = 0; j < 25; j++) {
    mdelay(10);
    status = dm_readb(DM1105_I2CSTS);
    if ((status & 0xc0) == 0x40)
    break;
    }
    if (j >= 25)
    return -1;
    k += 48;
    len -= 48;
    } while (len > 0);
    } else {
// write bytes
    dm_writeb(DM1105_I2CDAT, msgs[i].addr << 1);
    for (byte = 0; byte < msgs[i].len; byte++) {
    data = msgs[i].buf[byte];
    dm_writeb(DM1105_I2CDAT + byte + 1, data);
    }
    dm_writeb(DM1105_I2CCTR, 0x81 + msgs[i].len);
    for (j = 0; j < 25; j++) {
    mdelay(10);
    status = dm_readb(DM1105_I2CSTS);
    if ((status & 0xc0) == 0x40)
    break;
    }
    if (j >= 25)
    return -1;
    }
    }
    return num;
    err:
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn functionality(adap: *mut i2c_adapter) -> u32 {
    static u32 functionality(struct i2c_adapter *adap)
    {
    return I2C_FUNC_I2C;
    }
    static const struct i2c_algorithm dm1105_algo = {
    .master_xfer   = dm1105_i2c_xfer,
    .functionality = functionality,
    };
    static inline struct dm1105_dev *feed_to_dm1105_dev(struct dvb_demux_feed *feed)
    {
    return container_of(feed.demux, struct dm1105_dev, demux);
    }
    static inline struct dm1105_dev *frontend_to_dm1105_dev(struct dvb_frontend *fe)
    {
    return container_of(fe.dvb, struct dm1105_dev, dvb_adapter);
    }
    static int dm1105_set_voltage(struct dvb_frontend *fe,
    enum fe_sec_voltage voltage)
    {
    struct dm1105_dev *dev = frontend_to_dm1105_dev(fe);
    dm1105_gpio_enable(dev, dm1105_boards[dev.boardnr].lnb.mask, 1);
    if (voltage == SEC_VOLTAGE_18)
    dm1105_gpio_andor(dev,
    dm1105_boards[dev.boardnr].lnb.mask,
    dm1105_boards[dev.boardnr].lnb.v18);
#[no_mangle]
pub unsafe extern "C" fn if(SEC_VOLTAGE_13: voltage ==) -> else {
    else if (voltage == SEC_VOLTAGE_13)
    dm1105_gpio_andor(dev,
    dm1105_boards[dev.boardnr].lnb.mask,
    dm1105_boards[dev.boardnr].lnb.v13);
    else
    dm1105_gpio_andor(dev,
    dm1105_boards[dev.boardnr].lnb.mask,
    dm1105_boards[dev.boardnr].lnb.off);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dm1105_set_dma_addr(dev: *mut dm1105_dev) {
    static void dm1105_set_dma_addr(struct dm1105_dev *dev)
    {
    dm_writel(DM1105_STADR, ( u32)cpu_to_le32(dev.dma_addr));
    }
#[no_mangle]
unsafe extern "C" fn dm1105_dma_map(dev: *mut dm1105_dev) -> c_int {
    static int dm1105_dma_map(struct dm1105_dev *dev)
    {
    dev.ts_buf = dma_alloc_coherent(&dev.pdev.dev,
    6 * DM1105_DMA_BYTES, &dev.dma_addr,
    GFP_KERNEL);
    return !dev.ts_buf;
    }
#[no_mangle]
unsafe extern "C" fn dm1105_dma_unmap(dev: *mut dm1105_dev) {
    static void dm1105_dma_unmap(struct dm1105_dev *dev)
    {
    dma_free_coherent(&dev.pdev.dev, 6 * DM1105_DMA_BYTES, dev.ts_buf,
    dev.dma_addr);
    }
#[no_mangle]
unsafe extern "C" fn dm1105_enable_irqs(dev: *mut dm1105_dev) {
    static void dm1105_enable_irqs(struct dm1105_dev *dev)
    {
    dm_writeb(DM1105_INTMAK, INTMAK_ALLMASK);
    dm_writeb(DM1105_CR, 1);
    }
#[no_mangle]
unsafe extern "C" fn dm1105_disable_irqs(dev: *mut dm1105_dev) {
    static void dm1105_disable_irqs(struct dm1105_dev *dev)
    {
    dm_writeb(DM1105_INTMAK, INTMAK_IRM);
    dm_writeb(DM1105_CR, 0);
    }
#[no_mangle]
unsafe extern "C" fn dm1105_start_feed(f: *mut dvb_demux_feed) -> c_int {
    static int dm1105_start_feed(struct dvb_demux_feed *f)
    {
    struct dm1105_dev *dev = feed_to_dm1105_dev(f);
    if (dev.full_ts_users++ == 0)
    dm1105_enable_irqs(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dm1105_stop_feed(f: *mut dvb_demux_feed) -> c_int {
    static int dm1105_stop_feed(struct dvb_demux_feed *f)
    {
    struct dm1105_dev *dev = feed_to_dm1105_dev(f);
    if (--dev.full_ts_users == 0)
    dm1105_disable_irqs(dev);
    return 0;
    }
// ir work handler
#[no_mangle]
unsafe extern "C" fn dm1105_emit_key(work: *mut work_struct) {
    static void dm1105_emit_key(struct work_struct *work)
    {
    struct infrared *ir = container_of(work, struct infrared, work);
    let mut ircom: u32 = ir.ir_command;
    u8 data;
    if (ir_debug)
    printk(KERN_INFO "%s: received byte 0x%04x\n", __func__, ircom);
    data = (ircom >> 8) & 0x7f;
// FIXME: UNKNOWN because we don't generate a full NEC scancode (yet?)
    rc_keydown(ir.dev, RC_PROTO_UNKNOWN, data, 0);
    }
// work handler
#[no_mangle]
unsafe extern "C" fn dm1105_dmx_buffer(work: *mut work_struct) {
    static void dm1105_dmx_buffer(struct work_struct *work)
    {
    struct dm1105_dev *dev = container_of(work, struct dm1105_dev, work);
    unsigned int nbpackets;
    let mut oldwrp: u32 = dev.wrp;
    let mut nextwrp: u32 = dev.nextwrp;
    if (!((dev.ts_buf[oldwrp] == 0x47) &&
    (dev.ts_buf[oldwrp + 188] == 0x47) &&
    (dev.ts_buf[oldwrp + 188 * 2] == 0x47))) {
    dev.PacketErrorCount++;
// bad packet found
    if ((dev.PacketErrorCount >= 2) &&
    (dev.dmarst == 0)) {
    dm_writeb(DM1105_RST, 1);
    dev.wrp = 0;
    dev.PacketErrorCount = 0;
    dev.dmarst = 0;
    return;
    }
    }
    if (nextwrp < oldwrp) {
    memcpy(dev.ts_buf + dev.buffer_size, dev.ts_buf, nextwrp);
    nbpackets = ((dev.buffer_size - oldwrp) + nextwrp) / 188;
    } else
    nbpackets = (nextwrp - oldwrp) / 188;
    dev.wrp = nextwrp;
    dvb_dmx_swfilter_packets(&dev.demux, &dev.ts_buf[oldwrp], nbpackets);
    }
#[no_mangle]
unsafe extern "C" fn dm1105_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t dm1105_irq(int irq, void *dev_id)
    {
    struct dm1105_dev *dev = dev_id;
// Read-Write INSTS Ack's Interrupt for DM1105 chip 16.03.2008
    let mut intsts: c_uint = dm_readb(DM1105_INTSTS);
    dm_writeb(DM1105_INTSTS, intsts);
    switch (intsts) {
    case INTSTS_TSIRQ:
    case (INTSTS_TSIRQ | INTSTS_IR):
    dev.nextwrp = dm_readl(DM1105_WRP) - dm_readl(DM1105_STADR);
    queue_work(dev.wq, &dev.work);
    break;
    case INTSTS_IR:
    dev.ir.ir_command = dm_readl(DM1105_IRCODE);
    schedule_work(&dev.ir.work);
    break;
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn dm1105_ir_init(dm1105: *mut dm1105_dev) -> c_int {
    static int dm1105_ir_init(struct dm1105_dev *dm1105)
    {
    struct rc_dev *dev;
    let mut err: c_int = -ENOMEM;
    dev = rc_allocate_device(RC_DRIVER_SCANCODE);
    if (!dev)
    return -ENOMEM;
    snprintf(dm1105.ir.input_phys, sizeof(dm1105.ir.input_phys),
    "pci-%s/ir0", pci_name(dm1105.pdev));
    dev.driver_name = MODULE_NAME;
    dev.map_name = RC_MAP_DM1105_NEC;
    dev.device_name = "DVB on-card IR receiver";
    dev.input_phys = dm1105.ir.input_phys;
    dev.input_id.bustype = BUS_PCI;
    dev.input_id.version = 1;
    if (dm1105.pdev.subsystem_vendor) {
    dev.input_id.vendor = dm1105.pdev.subsystem_vendor;
    dev.input_id.product = dm1105.pdev.subsystem_device;
    } else {
    dev.input_id.vendor = dm1105.pdev.vendor;
    dev.input_id.product = dm1105.pdev.device;
    }
    dev.dev.parent = &dm1105.pdev.dev;
    INIT_WORK(&dm1105.ir.work, dm1105_emit_key);
    err = rc_register_device(dev);
    if (err < 0) {
    rc_free_device(dev);
    return err;
    }
    dm1105.ir.dev = dev;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dm1105_ir_exit(dm1105: *mut dm1105_dev) {
    static void dm1105_ir_exit(struct dm1105_dev *dm1105)
    {
    rc_unregister_device(dm1105.ir.dev);
    rc_free_device(dm1105.ir.dev);
    }
#[no_mangle]
unsafe extern "C" fn dm1105_hw_init(dev: *mut dm1105_dev) -> c_int {
    static int dm1105_hw_init(struct dm1105_dev *dev)
    {
    int ret;
    dm1105_disable_irqs(dev);
    dm_writeb(DM1105_HOST_CTR, 0);
// DATALEN 188,
    dm_writeb(DM1105_DTALENTH, 188);
// TS_STRT TS_VALP MSBFIRST TS_MODE ALPAS TSPES
    dm_writew(DM1105_TSCTR, 0xc10a);
// map DMA and set address
    ret = dm1105_dma_map(dev);
    if (ret)
    return -ENOMEM;
    dm1105_set_dma_addr(dev);
// big buffer
    dm_writel(DM1105_RLEN, 5 * DM1105_DMA_BYTES);
    dm_writeb(DM1105_INTCNT, 47);
// IR NEC mode enable
    dm_writeb(DM1105_IRCTR, (DM1105_IR_EN | DM1105_SYS_CHK));
    dm_writeb(DM1105_IRMODE, 0);
    dm_writew(DM1105_SYSTEMCODE, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dm1105_hw_exit(dev: *mut dm1105_dev) {
    static void dm1105_hw_exit(struct dm1105_dev *dev)
    {
    dm1105_disable_irqs(dev);
// IR disable
    dm_writeb(DM1105_IRCTR, 0);
    dm_writeb(DM1105_INTMAK, INTMAK_NONEMASK);
    dm1105_dma_unmap(dev);
    }
    static const struct stv0299_config sharp_z0194a_config = {
    .demod_address = 0x68,
    .inittab = sharp_z0194a_inittab,
    .mclk = 88000000UL,
    .invert = 1,
    .skip_reinit = 0,
    .lock_output = STV0299_LOCKOUTPUT_1,
    .volt13_op0_op1 = STV0299_VOLT13_OP1,
    .min_delay_ms = 100,
    .set_symbol_rate = sharp_z0194a_set_symbol_rate,
    };
    static struct stv0288_config earda_config = {
    .demod_address = 0x68,
    .min_delay_ms = 100,
    };
    static struct si21xx_config serit_config = {
    .demod_address = 0x68,
    .min_delay_ms = 100,
    };
    static struct cx24116_config serit_sp2633_config = {
    .demod_address = 0x55,
    };
    static struct ds3000_config dvbworld_ds3000_config = {
    .demod_address = 0x68,
    };
    static struct ts2020_config dvbworld_ts2020_config  = {
    .tuner_address = 0x60,
    .clk_out_div = 1,
    };
#[no_mangle]
unsafe extern "C" fn frontend_init(dev: *mut dm1105_dev) -> c_int {
    static int frontend_init(struct dm1105_dev *dev)
    {
    int ret;
    switch (dev.boardnr) {
    case DM1105_BOARD_UNBRANDED_I2C_ON_GPIO:
    dm1105_gpio_enable(dev, GPIO15, 1);
    dm1105_gpio_clear(dev, GPIO15);
    msleep(100);
    dm1105_gpio_set(dev, GPIO15);
    msleep(200);
    dev.fe = dvb_attach(
    stv0299_attach, &sharp_z0194a_config,
    &dev.i2c_bb_adap);
    if (dev.fe) {
    dev.fe.ops.set_voltage = dm1105_set_voltage;
    dvb_attach(dvb_pll_attach, dev.fe, 0x60,
    &dev.i2c_bb_adap, DVB_PLL_OPERA1);
    break;
    }
    dev.fe = dvb_attach(
    stv0288_attach, &earda_config,
    &dev.i2c_bb_adap);
    if (dev.fe) {
    dev.fe.ops.set_voltage = dm1105_set_voltage;
    dvb_attach(stb6000_attach, dev.fe, 0x61,
    &dev.i2c_bb_adap);
    break;
    }
    dev.fe = dvb_attach(
    si21xx_attach, &serit_config,
    &dev.i2c_bb_adap);
    if (dev.fe)
    dev.fe.ops.set_voltage = dm1105_set_voltage;
    break;
    case DM1105_BOARD_DVBWORLD_2004:
    dev.fe = dvb_attach(
    cx24116_attach, &serit_sp2633_config,
    &dev.i2c_adap);
    if (dev.fe) {
    dev.fe.ops.set_voltage = dm1105_set_voltage;
    break;
    }
    dev.fe = dvb_attach(
    ds3000_attach, &dvbworld_ds3000_config,
    &dev.i2c_adap);
    if (dev.fe) {
    dvb_attach(ts2020_attach, dev.fe,
    &dvbworld_ts2020_config, &dev.i2c_adap);
    dev.fe.ops.set_voltage = dm1105_set_voltage;
    }
    break;
    case DM1105_BOARD_DVBWORLD_2002:
    case DM1105_BOARD_AXESS_DM05:
    default:
    dev.fe = dvb_attach(
    stv0299_attach, &sharp_z0194a_config,
    &dev.i2c_adap);
    if (dev.fe) {
    dev.fe.ops.set_voltage = dm1105_set_voltage;
    dvb_attach(dvb_pll_attach, dev.fe, 0x60,
    &dev.i2c_adap, DVB_PLL_OPERA1);
    break;
    }
    dev.fe = dvb_attach(
    stv0288_attach, &earda_config,
    &dev.i2c_adap);
    if (dev.fe) {
    dev.fe.ops.set_voltage = dm1105_set_voltage;
    dvb_attach(stb6000_attach, dev.fe, 0x61,
    &dev.i2c_adap);
    break;
    }
    dev.fe = dvb_attach(
    si21xx_attach, &serit_config,
    &dev.i2c_adap);
    if (dev.fe)
    dev.fe.ops.set_voltage = dm1105_set_voltage;
    }
    if (!dev.fe) {
    dev_err(&dev.pdev.dev, "could not attach frontend\n");
    return -ENODEV;
    }
    ret = dvb_register_frontend(&dev.dvb_adapter, dev.fe);
    if (ret < 0) {
    if (dev.fe.ops.release)
    dev.fe.ops.release(dev.fe);
    dev.fe = core::ptr::null_mut();
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dm1105_read_mac(dev: *mut dm1105_dev, mac: *mut u8) {
    static void dm1105_read_mac(struct dm1105_dev *dev, u8 *mac)
    {
    static u8 command[1] = { 0x28 };
    struct i2c_msg msg[] = {
    {
    .addr = IIC_24C01_addr >> 1,
    .flags = 0,
    .buf = command,
    .len = 1
    }, {
    .addr = IIC_24C01_addr >> 1,
    .flags = I2C_M_RD,
    .buf = mac,
    .len = 6
    },
    };
    dm1105_i2c_xfer(&dev.i2c_adap, msg , 2);
    dev_info(&dev.pdev.dev, "MAC %pM\n", mac);
    }
    static int dm1105_probe(struct pci_dev *pdev,
    const struct pci_device_id *ent)
    {
    struct dm1105_dev *dev;
    struct dvb_adapter *dvb_adapter;
    struct dvb_demux *dvbdemux;
    struct dmx_demux *dmx;
    let mut ret: c_int = -ENOMEM;
    int i;
    if (dm1105_devcount >= ARRAY_SIZE(card))
    return -ENODEV;
    dev = kzalloc_obj(struct dm1105_dev);
    if (!dev)
    return -ENOMEM;
// board config
    dev.nr = dm1105_devcount;
    dev.boardnr = UNSET;
    if (card[dev.nr] < ARRAY_SIZE(dm1105_boards))
    dev.boardnr = card[dev.nr];
    for (i = 0; UNSET == dev.boardnr &&
    i < ARRAY_SIZE(dm1105_subids); i++)
    if (pdev.subsystem_vendor ==
    dm1105_subids[i].subvendor &&
    pdev.subsystem_device ==
    dm1105_subids[i].subdevice)
    dev.boardnr = dm1105_subids[i].card;
    if (UNSET == dev.boardnr) {
    dev.boardnr = DM1105_BOARD_UNKNOWN;
    dm1105_card_list(pdev);
    }
    dm1105_devcount++;
    dev.pdev = pdev;
    dev.buffer_size = 5 * DM1105_DMA_BYTES;
    dev.PacketErrorCount = 0;
    dev.dmarst = 0;
    ret = pci_enable_device(pdev);
    if (ret < 0)
    goto err_kfree;
    ret = dma_set_mask(&pdev.dev, DMA_BIT_MASK(32));
    if (ret < 0)
    goto err_pci_disable_device;
    pci_set_master(pdev);
    ret = pci_request_regions(pdev, DRIVER_NAME);
    if (ret < 0)
    goto err_pci_disable_device;
    dev.io_mem = pci_iomap(pdev, 0, pci_resource_len(pdev, 0));
    if (!dev.io_mem) {
    ret = -EIO;
    goto err_pci_release_regions;
    }
    spin_lock_init(&dev.lock);
    pci_set_drvdata(pdev, dev);
    ret = dm1105_hw_init(dev);
    if (ret < 0)
    goto err_pci_iounmap;
// i2c
    i2c_set_adapdata(&dev.i2c_adap, dev);
    strscpy(dev.i2c_adap.name, DRIVER_NAME, sizeof(dev.i2c_adap.name));
    dev.i2c_adap.owner = THIS_MODULE;
    dev.i2c_adap.dev.parent = &pdev.dev;
    dev.i2c_adap.algo = &dm1105_algo;
    dev.i2c_adap.algo_data = dev;
    ret = i2c_add_adapter(&dev.i2c_adap);
    if (ret < 0)
    goto err_dm1105_hw_exit;
    i2c_set_adapdata(&dev.i2c_bb_adap, dev);
    strscpy(dev.i2c_bb_adap.name, DM1105_I2C_GPIO_NAME,
    sizeof(dev.i2c_bb_adap.name));
    dev.i2c_bb_adap.owner = THIS_MODULE;
    dev.i2c_bb_adap.dev.parent = &pdev.dev;
    dev.i2c_bb_adap.algo_data = &dev.i2c_bit;
    dev.i2c_bit.data = dev;
    dev.i2c_bit.setsda = dm1105_setsda;
    dev.i2c_bit.setscl = dm1105_setscl;
    dev.i2c_bit.getsda = dm1105_getsda;
    dev.i2c_bit.getscl = dm1105_getscl;
    dev.i2c_bit.udelay = 10;
    dev.i2c_bit.timeout = 10;
// Raise SCL and SDA
    dm1105_setsda(dev, 1);
    dm1105_setscl(dev, 1);
    ret = i2c_bit_add_bus(&dev.i2c_bb_adap);
    if (ret < 0)
    goto err_i2c_del_adapter;
// dvb
    ret = dvb_register_adapter(&dev.dvb_adapter, DRIVER_NAME,
    THIS_MODULE, &pdev.dev, adapter_nr);
    if (ret < 0)
    goto err_i2c_del_adapters;
    dvb_adapter = &dev.dvb_adapter;
    dm1105_read_mac(dev, dvb_adapter.proposed_mac);
    dvbdemux = &dev.demux;
    dvbdemux.filternum = 256;
    dvbdemux.feednum = 256;
    dvbdemux.start_feed = dm1105_start_feed;
    dvbdemux.stop_feed = dm1105_stop_feed;
    dvbdemux.dmx.capabilities = (DMX_TS_FILTERING |
    DMX_SECTION_FILTERING | DMX_MEMORY_BASED_FILTERING);
    ret = dvb_dmx_init(dvbdemux);
    if (ret < 0)
    goto err_dvb_unregister_adapter;
    dmx = &dvbdemux.dmx;
    dev.dmxdev.filternum = 256;
    dev.dmxdev.demux = dmx;
    dev.dmxdev.capabilities = 0;
    ret = dvb_dmxdev_init(&dev.dmxdev, dvb_adapter);
    if (ret < 0)
    goto err_dvb_dmx_release;
    dev.hw_frontend.source = DMX_FRONTEND_0;
    ret = dmx.add_frontend(dmx, &dev.hw_frontend);
    if (ret < 0)
    goto err_dvb_dmxdev_release;
    dev.mem_frontend.source = DMX_MEMORY_FE;
    ret = dmx.add_frontend(dmx, &dev.mem_frontend);
    if (ret < 0)
    goto err_remove_hw_frontend;
    ret = dmx.connect_frontend(dmx, &dev.hw_frontend);
    if (ret < 0)
    goto err_remove_mem_frontend;
    ret = dvb_net_init(dvb_adapter, &dev.dvbnet, dmx);
    if (ret < 0)
    goto err_disconnect_frontend;
    ret = frontend_init(dev);
    if (ret < 0)
    goto err_dvb_net;
    dm1105_ir_init(dev);
    INIT_WORK(&dev.work, dm1105_dmx_buffer);
    sprintf(dev.wqn, "%s/%d", dvb_adapter.name, dvb_adapter.num);
    dev.wq = create_singlethread_workqueue(dev.wqn);
    if (!dev.wq) {
    ret = -ENOMEM;
    goto err_dvb_net;
    }
    ret = request_irq(pdev.irq, dm1105_irq, IRQF_SHARED,
    DRIVER_NAME, dev);
    if (ret < 0)
    goto err_workqueue;
    return 0;
    err_workqueue:
    destroy_workqueue(dev.wq);
    err_dvb_net:
    dvb_net_release(&dev.dvbnet);
    err_disconnect_frontend:
    dmx.disconnect_frontend(dmx);
    err_remove_mem_frontend:
    dmx.remove_frontend(dmx, &dev.mem_frontend);
    err_remove_hw_frontend:
    dmx.remove_frontend(dmx, &dev.hw_frontend);
    err_dvb_dmxdev_release:
    dvb_dmxdev_release(&dev.dmxdev);
    err_dvb_dmx_release:
    dvb_dmx_release(dvbdemux);
    err_dvb_unregister_adapter:
    dvb_unregister_adapter(dvb_adapter);
    err_i2c_del_adapters:
    i2c_del_adapter(&dev.i2c_bb_adap);
    err_i2c_del_adapter:
    i2c_del_adapter(&dev.i2c_adap);
    err_dm1105_hw_exit:
    dm1105_hw_exit(dev);
    err_pci_iounmap:
    pci_iounmap(pdev, dev.io_mem);
    err_pci_release_regions:
    pci_release_regions(pdev);
    err_pci_disable_device:
    pci_disable_device(pdev);
    err_kfree:
    kfree(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dm1105_remove(pdev: *mut pci_dev) {
    static void dm1105_remove(struct pci_dev *pdev)
    {
    struct dm1105_dev *dev = pci_get_drvdata(pdev);
    struct dvb_adapter *dvb_adapter = &dev.dvb_adapter;
    struct dvb_demux *dvbdemux = &dev.demux;
    struct dmx_demux *dmx = &dvbdemux.dmx;
    cancel_work_sync(&dev.ir.work);
    dm1105_ir_exit(dev);
    dmx.close(dmx);
    dvb_net_release(&dev.dvbnet);
    if (dev.fe)
    dvb_unregister_frontend(dev.fe);
    dmx.disconnect_frontend(dmx);
    dmx.remove_frontend(dmx, &dev.mem_frontend);
    dmx.remove_frontend(dmx, &dev.hw_frontend);
    dvb_dmxdev_release(&dev.dmxdev);
    dvb_dmx_release(dvbdemux);
    dvb_unregister_adapter(dvb_adapter);
    i2c_del_adapter(&dev.i2c_adap);
    dm1105_hw_exit(dev);
    free_irq(pdev.irq, dev);
    destroy_workqueue(dev.wq);
    pci_iounmap(pdev, dev.io_mem);
    pci_release_regions(pdev);
    pci_disable_device(pdev);
    dm1105_devcount--;
    kfree(dev);
    }
    static const struct pci_device_id dm1105_id_table[] = {
    {
    PCI_VDEVICE(TRIGEM, PCI_DEVICE_ID_DM1105),
    }, {
    PCI_VDEVICE(AXESS, PCI_DEVICE_ID_DM05),
    }, {
// empty
    },
    };
    MODULE_DEVICE_TABLE(pci, dm1105_id_table);
    static struct pci_driver dm1105_driver = {
    .name = DRIVER_NAME,
    .id_table = dm1105_id_table,
    .probe = dm1105_probe,
    .remove = dm1105_remove,
    };
    module_pci_driver(dm1105_driver);
    MODULE_AUTHOR("Igor M. Liplianin <liplianin@me.by>");
    MODULE_DESCRIPTION("SDMC DM1105 DVB driver");
    MODULE_LICENSE("GPL");
