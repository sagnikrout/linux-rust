//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/sja1000/peak_pci.c
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
// Copyright (C) 2007, 2011 Wolfgang Grandegger <wg@grandegger.com>
//
// Derived from the PCAN project file driver/src/pcan_pci.c:
//
// Copyright (C) 2001-2025 PEAK System-Technik GmbH
// Author: Stéphane Grosjean <s.grosjean@peak-system.fr>
//

    MODULE_AUTHOR("Stéphane Grosjean <s.grosjean@peak-system.fr>");
    MODULE_DESCRIPTION("Socket-CAN driver for PEAK PCAN PCI family cards");
    MODULE_LICENSE("GPL v2");

// FPGA cards FW version registers
pub const PEAK_VER_REG1: c_uint = 0x40;
pub const PEAK_VER_REG2: c_uint = 0x44;
    struct peak_pciec_card;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct peak_pci_chan {
    pub /: *mut *mut *mut void __iomem cfg_base; / Common for all channels,
    pub /: *mut *mut *mut net_device prev_dev; / Chain of network devices,
    pub /: *mut *mut u16 icr_mask; / Interrupt mask for fast ack,
    pub /: *mut *mut *mut peak_pciec_card pciec_card; / only for PCIeC LEDs,
}

// Important PITA registers
pub const PITA_ICR: c_uint = 0x00	/* Interrupt control register */;
pub const PITA_GPIOICR: c_uint = 0x18	/* GPIO interface control register */;
pub const PITA_MISC: c_uint = 0x1C	/* Miscellaneous register */;
pub const PEAK_PCI_CFG_SIZE: c_uint = 0x1000	/* Size of the config PCI bar */;
pub const PEAK_PCI_CHAN_SIZE: c_uint = 0x0400	/* Size used by the channel */;
pub const PEAK_PCI_VENDOR_ID: c_uint = 0x001C	/* The PCI device and vendor IDs */;
pub const PEAK_PCI_DEVICE_ID: c_uint = 0x0001	/* for PCI/PCIe slot cards */;
pub const PEAK_PCIEC_DEVICE_ID: c_uint = 0x0002	/* for ExpressCard slot cards */;
pub const PEAK_PCIE_DEVICE_ID: c_uint = 0x0003	/* for nextgen PCIe slot cards */;
pub const PEAK_CPCI_DEVICE_ID: c_uint = 0x0004	/* for nextgen cPCI slot cards */;
pub const PEAK_MPCI_DEVICE_ID: c_uint = 0x0005	/* for nextgen miniPCI slot cards */;
pub const PEAK_PC_104P_DEVICE_ID: c_uint = 0x0006	/* PCAN-PC/104+ cards */;
pub const PEAK_PCI_104E_DEVICE_ID: c_uint = 0x0007	/* PCAN-PCI/104 Express cards */;
pub const PEAK_MPCIE_DEVICE_ID: c_uint = 0x0008	/* The miniPCIe slot cards */;
pub const PEAK_PCIE_OEM_ID: c_uint = 0x0009	/* PCAN-PCI Express OEM */;
pub const PEAK_PCIEC34_DEVICE_ID: c_uint = 0x000A	/* PCAN-PCI Express 34 (one channel) */;
pub const PEAK_PCI_CHAN_MAX: c_int = 4;
    static const u16 peak_pci_icr_masks[PEAK_PCI_CHAN_MAX] = {
    0x02, 0x01, 0x40, 0x80
    };
    static const struct pci_device_id peak_pci_tbl[] = {
    {
    PEAK_PCI_VENDOR_ID, PEAK_PCI_DEVICE_ID, PCI_ANY_ID, PCI_ANY_ID,
    .driver_data = (kernel_ulong_t)"PCAN-PCI",
    }, {
    PEAK_PCI_VENDOR_ID, PEAK_PCIE_DEVICE_ID, PCI_ANY_ID, PCI_ANY_ID,
    .driver_data = (kernel_ulong_t)"PCAN-PCI Express",
    }, {
    PEAK_PCI_VENDOR_ID, PEAK_MPCI_DEVICE_ID, PCI_ANY_ID, PCI_ANY_ID,
    .driver_data = (kernel_ulong_t)"PCAN-miniPCI",
    }, {
    PEAK_PCI_VENDOR_ID, PEAK_MPCIE_DEVICE_ID, PCI_ANY_ID, PCI_ANY_ID,
    .driver_data = (kernel_ulong_t)"PCAN-miniPCIe",
    }, {
    PEAK_PCI_VENDOR_ID, PEAK_PC_104P_DEVICE_ID, PCI_ANY_ID, PCI_ANY_ID,
    .driver_data = (kernel_ulong_t)"PCAN-PC/104-Plus Quad",
    }, {
    PEAK_PCI_VENDOR_ID, PEAK_PCI_104E_DEVICE_ID, PCI_ANY_ID, PCI_ANY_ID,
    .driver_data = (kernel_ulong_t)"PCAN-PCI/104-Express",
    }, {
    PEAK_PCI_VENDOR_ID, PEAK_CPCI_DEVICE_ID, PCI_ANY_ID, PCI_ANY_ID,
    .driver_data = (kernel_ulong_t)"PCAN-cPCI",
    }, {
    PEAK_PCI_VENDOR_ID, PEAK_PCIE_OEM_ID, PCI_ANY_ID, PCI_ANY_ID,
    .driver_data = (kernel_ulong_t)"PCAN-Chip PCIe",
    },

    {
    PEAK_PCI_VENDOR_ID, PEAK_PCIEC_DEVICE_ID, PCI_ANY_ID, PCI_ANY_ID,
    .driver_data = (kernel_ulong_t)"PCAN-ExpressCard",
    }, {
    PEAK_PCI_VENDOR_ID, PEAK_PCIEC34_DEVICE_ID, PCI_ANY_ID, PCI_ANY_ID,
    .driver_data = (kernel_ulong_t)"PCAN-ExpressCard 34",
    },

    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(pci, peak_pci_tbl);

// PCAN-ExpressCard needs I2C bit-banging configuration option.
// GPIOICR byte access offsets
pub const PITA_GPOUT: c_uint = 0x18	/* GPx output value */;
pub const PITA_GPIN: c_uint = 0x19	/* GPx input value */;
pub const PITA_GPOEN: c_uint = 0x1A	/* configure GPx as output pin */;
// I2C GP bits
pub const PITA_GPIN_SCL: c_uint = 0x01	/* Serial Clock Line */;
pub const PITA_GPIN_SDA: c_uint = 0x04	/* Serial DAta line */;

// PCA9553 LS0 fields values
    enum {
    PCA9553_LOW,
    PCA9553_HIGHZ,
    PCA9553_PWM0,
    PCA9553_PWM1
    };
// LEDs control

pub const PCA9553_LS0_INIT: c_uint = 0x40 /* initial value (!= from 0x00) */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct peak_pciec_chan {
    pub netdev: *mut net_device,
    pub prev_rx_bytes: c_ulong,
    pub prev_tx_bytes: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct peak_pciec_card {
    pub /: *mut *mut *mut void __iomem cfg_base; / Common for all channels,
    pub /: *mut *mut *mut void __iomem reg_base; / first channel base address,
    pub /: *mut *mut u8 led_cache; / leds state cache,
// PCIExpressCard i2c data
    pub i2c_bit: i2c_algo_bit_data,
    pub led_chip: i2c_adapter,
    pub /: *mut *mut delayed_work led_work; / led delayed work,
    pub chan_count: c_int,
    pub channel: [peak_pciec_chan; PEAK_PCI_CHAN_MAX],
}

// "normal" pci register write callback is overloaded for leds control
    static void peak_pci_write_reg(const struct sja1000_priv *priv,
    int port, u8 val);
#[no_mangle]
pub unsafe extern "C" fn pita_set_scl_highz(card: *mut peak_pciec_card) {
    static inline void pita_set_scl_highz(struct peak_pciec_card *card)
    {
    let mut gp_outen: u8 = readb(card.cfg_base + PITA_GPOEN) & ~PITA_GPIN_SCL;
    writeb(gp_outen, card.cfg_base + PITA_GPOEN);
    }
#[no_mangle]
pub unsafe extern "C" fn pita_set_sda_highz(card: *mut peak_pciec_card) {
    static inline void pita_set_sda_highz(struct peak_pciec_card *card)
    {
    let mut gp_outen: u8 = readb(card.cfg_base + PITA_GPOEN) & ~PITA_GPIN_SDA;
    writeb(gp_outen, card.cfg_base + PITA_GPOEN);
    }
#[no_mangle]
unsafe extern "C" fn peak_pciec_init_pita_gpio(card: *mut peak_pciec_card) {
    static void peak_pciec_init_pita_gpio(struct peak_pciec_card *card)
    {
// raise SCL & SDA GPIOs to high-Z
    pita_set_scl_highz(card);
    pita_set_sda_highz(card);
    }
#[no_mangle]
unsafe extern "C" fn pita_setsda(data: *mut c_void, state: c_int) {
    static void pita_setsda(void *data, int state)
    {
    struct peak_pciec_card *card = (struct peak_pciec_card *)data;
    u8 gp_out, gp_outen;
// set output sda always to 0
    gp_out = readb(card.cfg_base + PITA_GPOUT) & ~PITA_GPIN_SDA;
    writeb(gp_out, card.cfg_base + PITA_GPOUT);
// control output sda with GPOEN
    gp_outen = readb(card.cfg_base + PITA_GPOEN);
    if (state)
    gp_outen &= ~PITA_GPIN_SDA;
    else
    gp_outen |= PITA_GPIN_SDA;
    writeb(gp_outen, card.cfg_base + PITA_GPOEN);
    }
#[no_mangle]
unsafe extern "C" fn pita_setscl(data: *mut c_void, state: c_int) {
    static void pita_setscl(void *data, int state)
    {
    struct peak_pciec_card *card = (struct peak_pciec_card *)data;
    u8 gp_out, gp_outen;
// set output scl always to 0
    gp_out = readb(card.cfg_base + PITA_GPOUT) & ~PITA_GPIN_SCL;
    writeb(gp_out, card.cfg_base + PITA_GPOUT);
// control output scl with GPOEN
    gp_outen = readb(card.cfg_base + PITA_GPOEN);
    if (state)
    gp_outen &= ~PITA_GPIN_SCL;
    else
    gp_outen |= PITA_GPIN_SCL;
    writeb(gp_outen, card.cfg_base + PITA_GPOEN);
    }
#[no_mangle]
unsafe extern "C" fn pita_getsda(data: *mut c_void) -> c_int {
    static int pita_getsda(void *data)
    {
    struct peak_pciec_card *card = (struct peak_pciec_card *)data;
// set tristate
    pita_set_sda_highz(card);
    return (readb(card.cfg_base + PITA_GPIN) & PITA_GPIN_SDA) ? 1 : 0;
    }
#[no_mangle]
unsafe extern "C" fn pita_getscl(data: *mut c_void) -> c_int {
    static int pita_getscl(void *data)
    {
    struct peak_pciec_card *card = (struct peak_pciec_card *)data;
// set tristate
    pita_set_scl_highz(card);
    return (readb(card.cfg_base + PITA_GPIN) & PITA_GPIN_SCL) ? 1 : 0;
    }
// write commands to the LED chip though the I2C-bus of the PCAN-PCIeC
    static int peak_pciec_write_pca9553(struct peak_pciec_card *card,
    u8 offset, u8 data)
    {
    u8 buffer[2] = {
    offset,
    data
    };
    struct i2c_msg msg = {
    .addr = PCA9553_1_SLAVEADDR,
    .len = 2,
    .buf = buffer,
    };
    int ret;
// cache led mask
    if (offset == 5 && data == card.led_cache)
    return 0;
    ret = i2c_transfer(&card.led_chip, &msg, 1);
    if (ret < 0)
    return ret;
    if (offset == 5)
    card.led_cache = data;
    return 0;
    }
// delayed work callback used to control the LEDs
#[no_mangle]
unsafe extern "C" fn peak_pciec_led_work(work: *mut work_struct) {
    static void peak_pciec_led_work(struct work_struct *work)
    {
    struct peak_pciec_card *card =
    container_of(work, struct peak_pciec_card, led_work.work);
    struct net_device *netdev;
    let mut new_led: u8 = card.led_cache;
    int i, up_count = 0;
// first check what is to do
    for (i = 0; i < card.chan_count; i++) {
// default is: not configured
    new_led &= ~PCA9553_LED_MASK(i);
    new_led |= PCA9553_LED_ON(i);
    netdev = card.channel[i].netdev;
    if (!netdev || !(netdev.flags & IFF_UP))
    continue;
    up_count++;
// no activity (but configured)
    new_led &= ~PCA9553_LED_MASK(i);
    new_led |= PCA9553_LED_SLOW(i);
// if bytes counters changed, set fast blinking led
    if (netdev.stats.rx_bytes != card.channel[i].prev_rx_bytes) {
    card.channel[i].prev_rx_bytes = netdev.stats.rx_bytes;
    new_led &= ~PCA9553_LED_MASK(i);
    new_led |= PCA9553_LED_FAST(i);
    }
    if (netdev.stats.tx_bytes != card.channel[i].prev_tx_bytes) {
    card.channel[i].prev_tx_bytes = netdev.stats.tx_bytes;
    new_led &= ~PCA9553_LED_MASK(i);
    new_led |= PCA9553_LED_FAST(i);
    }
    }
// check if LS0 settings changed, only update i2c if so
    peak_pciec_write_pca9553(card, 5, new_led);
// restart timer (except if no more configured channels)
    if (up_count)
    schedule_delayed_work(&card.led_work, HZ);
    }
// set LEDs blinking state
#[no_mangle]
unsafe extern "C" fn peak_pciec_set_leds(card: *mut peak_pciec_card, led_mask: u8, s: u8) {
    static void peak_pciec_set_leds(struct peak_pciec_card *card, u8 led_mask, u8 s)
    {
    let mut new_led: u8 = card.led_cache;
    int i;
// first check what is to do
    for (i = 0; i < card.chan_count; i++)
    if (led_mask & PCA9553_LED(i)) {
    new_led &= ~PCA9553_LED_MASK(i);
    new_led |= PCA9553_LED_STATE(s, i);
    }
// check if LS0 settings changed, only update i2c if so
    peak_pciec_write_pca9553(card, 5, new_led);
    }
// start one second delayed work to control LEDs
#[no_mangle]
unsafe extern "C" fn peak_pciec_start_led_work(card: *mut peak_pciec_card) {
    static void peak_pciec_start_led_work(struct peak_pciec_card *card)
    {
    schedule_delayed_work(&card.led_work, HZ);
    }
// stop LEDs delayed work
#[no_mangle]
unsafe extern "C" fn peak_pciec_stop_led_work(card: *mut peak_pciec_card) {
    static void peak_pciec_stop_led_work(struct peak_pciec_card *card)
    {
    cancel_delayed_work_sync(&card.led_work);
    }
// initialize the PCA9553 4-bit I2C-bus LED chip
#[no_mangle]
unsafe extern "C" fn peak_pciec_init_leds(card: *mut peak_pciec_card) -> c_int {
    static int peak_pciec_init_leds(struct peak_pciec_card *card)
    {
    int err;
// prescaler for frequency 0: "SLOW" = 1 Hz = "44"
    err = peak_pciec_write_pca9553(card, 1, 44 / 1);
    if (err)
    return err;
// duty cycle 0: 50%
    err = peak_pciec_write_pca9553(card, 2, 0x80);
    if (err)
    return err;
// prescaler for frequency 1: "FAST" = 5 Hz
    err = peak_pciec_write_pca9553(card, 3, 44 / 5);
    if (err)
    return err;
// duty cycle 1: 50%
    err = peak_pciec_write_pca9553(card, 4, 0x80);
    if (err)
    return err;
// switch LEDs to initial state
    return peak_pciec_write_pca9553(card, 5, PCA9553_LS0_INIT);
    }
// restore LEDs state to off peak_pciec_leds_exit
#[no_mangle]
unsafe extern "C" fn peak_pciec_leds_exit(card: *mut peak_pciec_card) {
    static void peak_pciec_leds_exit(struct peak_pciec_card *card)
    {
// switch LEDs to off
    peak_pciec_write_pca9553(card, 5, PCA9553_LED_OFF_ALL);
    }
// normal write sja1000 register method overloaded to catch when controller
// is started or stopped, to control leds
//
    static void peak_pciec_write_reg(const struct sja1000_priv *priv,
    int port, u8 val)
    {
    struct peak_pci_chan *chan = priv.priv;
    struct peak_pciec_card *card = chan.pciec_card;
    let mut c: c_int = (priv.reg_base - card.reg_base) / PEAK_PCI_CHAN_SIZE;
// sja1000 register changes control the leds state
    if (port == SJA1000_MOD)
    switch (val) {
    case MOD_RM:
// Reset Mode: set led on
    peak_pciec_set_leds(card, PCA9553_LED(c), PCA9553_ON);
    break;
    case 0x00:
// Normal Mode: led slow blinking and start led timer
    peak_pciec_set_leds(card, PCA9553_LED(c), PCA9553_SLOW);
    peak_pciec_start_led_work(card);
    break;
    default:
    break;
    }
// call base function
    peak_pci_write_reg(priv, port, val);
    }
    static const struct i2c_algo_bit_data peak_pciec_i2c_bit_ops = {
    .setsda	= pita_setsda,
    .setscl	= pita_setscl,
    .getsda	= pita_getsda,
    .getscl	= pita_getscl,
    .udelay	= 10,
    .timeout = HZ,
    };
#[no_mangle]
unsafe extern "C" fn peak_pciec_probe(pdev: *mut pci_dev, dev: *mut net_device) -> c_int {
    static int peak_pciec_probe(struct pci_dev *pdev, struct net_device *dev)
    {
    struct sja1000_priv *priv = netdev_priv(dev);
    struct peak_pci_chan *chan = priv.priv;
    struct peak_pciec_card *card;
    int err;
// copy i2c object address from 1st channel
    if (chan.prev_dev) {
    struct sja1000_priv *prev_priv = netdev_priv(chan.prev_dev);
    struct peak_pci_chan *prev_chan = prev_priv.priv;
    card = prev_chan.pciec_card;
    if (!card)
    return -ENODEV;
// channel is the first one: do the init part
    } else {
// create the bit banging I2C adapter structure
    card = kzalloc_obj(*card);
    if (!card)
    return -ENOMEM;
    card.cfg_base = chan.cfg_base;
    card.reg_base = priv.reg_base;
    card.led_chip.owner = THIS_MODULE;
    card.led_chip.dev.parent = &pdev.dev;
    card.led_chip.algo_data = &card.i2c_bit;
    strscpy(card.led_chip.name, "peak_i2c",
    sizeof(card.led_chip.name));
    card.i2c_bit = peak_pciec_i2c_bit_ops;
    card.i2c_bit.udelay = 10;
    card.i2c_bit.timeout = HZ;
    card.i2c_bit.data = card;
    peak_pciec_init_pita_gpio(card);
    err = i2c_bit_add_bus(&card.led_chip);
    if (err) {
    dev_err(&pdev.dev, "i2c init failed\n");
    goto pciec_init_err_1;
    }
    err = peak_pciec_init_leds(card);
    if (err) {
    dev_err(&pdev.dev, "leds hardware init failed\n");
    goto pciec_init_err_2;
    }
    INIT_DELAYED_WORK(&card.led_work, peak_pciec_led_work);
// PCAN-ExpressCard needs its own callback for leds
    priv.write_reg = peak_pciec_write_reg;
    }
    chan.pciec_card = card;
    card.channel[card.chan_count++].netdev = dev;
    return 0;
    pciec_init_err_2:
    i2c_del_adapter(&card.led_chip);
    pciec_init_err_1:
    peak_pciec_init_pita_gpio(card);
    kfree(card);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn peak_pciec_remove(card: *mut peak_pciec_card) {
    static void peak_pciec_remove(struct peak_pciec_card *card)
    {
    peak_pciec_stop_led_work(card);
    peak_pciec_leds_exit(card);
    i2c_del_adapter(&card.led_chip);
    peak_pciec_init_pita_gpio(card);
    kfree(card);
    }

// Placebo functions when PCAN-ExpressCard support is not selected
#[no_mangle]
pub unsafe extern "C" fn peak_pciec_probe(pdev: *mut pci_dev, dev: *mut net_device) -> c_int {
    static inline int peak_pciec_probe(struct pci_dev *pdev, struct net_device *dev)
    {
    return -ENODEV;
    }
#[no_mangle]
pub unsafe extern "C" fn peak_pciec_remove(card: *mut peak_pciec_card) {
    static inline void peak_pciec_remove(struct peak_pciec_card *card)
    {
    }

#[no_mangle]
unsafe extern "C" fn peak_pci_read_reg(priv: *const sja1000_priv, port: c_int) -> u8 {
    static u8 peak_pci_read_reg(const struct sja1000_priv *priv, int port)
    {
    return readb(priv.reg_base + (port << 2));
    }
    static void peak_pci_write_reg(const struct sja1000_priv *priv,
    int port, u8 val)
    {
    writeb(val, priv.reg_base + (port << 2));
    }
#[no_mangle]
unsafe extern "C" fn peak_pci_post_irq(priv: *const sja1000_priv) {
    static void peak_pci_post_irq(const struct sja1000_priv *priv)
    {
    struct peak_pci_chan *chan = priv.priv;
    u16 icr;
// Select and clear in PITA stored interrupt
    icr = readw(chan.cfg_base + PITA_ICR);
    if (icr & chan.icr_mask)
    writew(chan.icr_mask, chan.cfg_base + PITA_ICR);
    }
#[no_mangle]
unsafe extern "C" fn peak_pci_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int peak_pci_probe(struct pci_dev *pdev, const struct pci_device_id *ent)
    {
    struct sja1000_priv *priv;
    struct peak_pci_chan *chan;
    struct net_device *dev, *prev_dev;
    void __iomem *cfg_base, *reg_base;
    u16 sub_sys_id, icr;
    int i, err, channels;
    char fw_str[14] = "";
    err = pci_enable_device(pdev);
    if (err)
    return err;
    err = pci_request_regions(pdev, DRV_NAME);
    if (err)
    goto failure_disable_pci;
    err = pci_read_config_word(pdev, 0x2e, &sub_sys_id);
    if (err)
    goto failure_release_regions;
    dev_dbg(&pdev.dev, "probing device %04x:%04x:%04x\n",
    pdev.vendor, pdev.device, sub_sys_id);
    err = pci_write_config_word(pdev, 0x44, 0);
    if (err)
    goto failure_release_regions;
    if (sub_sys_id >= 12)
    channels = 4;
#[no_mangle]
pub unsafe extern "C" fn if(10: sub_sys_id >=) -> else {
    else if (sub_sys_id >= 10)
    channels = 3;
#[no_mangle]
pub unsafe extern "C" fn if(4: sub_sys_id >=) -> else {
    else if (sub_sys_id >= 4)
    channels = 2;
    else
    channels = 1;
    cfg_base = pci_iomap(pdev, 0, PEAK_PCI_CFG_SIZE);
    if (!cfg_base) {
    dev_err(&pdev.dev, "failed to map PCI resource #0\n");
    err = -ENOMEM;
    goto failure_release_regions;
    }
    reg_base = pci_iomap(pdev, 1, PEAK_PCI_CHAN_SIZE * channels);
    if (!reg_base) {
    dev_err(&pdev.dev, "failed to map PCI resource #1\n");
    err = -ENOMEM;
    goto failure_unmap_cfg_base;
    }
// Set GPIO control register
    writew(0x0005, cfg_base + PITA_GPIOICR + 2);
// Enable all channels of this card
    writeb(0x00, cfg_base + PITA_GPIOICR);
// Toggle reset
    writeb(0x05, cfg_base + PITA_MISC + 3);
    usleep_range(5000, 6000);
// Leave parport mux mode
    writeb(0x04, cfg_base + PITA_MISC + 3);
// FPGA equipped card if not 0
    if (readl(cfg_base + PEAK_VER_REG1)) {
// FPGA card: display version of the running firmware
    let mut fw_ver: u32 = readl(cfg_base + PEAK_VER_REG2);
    snprintf(fw_str, sizeof(fw_str), " FW v%u.%u.%u",
    (fw_ver >> 12) & 0xf,
    (fw_ver >> 8) & 0xf,
    (fw_ver >> 4) & 0xf);
    }
// Display commercial name (and, eventually, FW version) of the card
    dev_info(&pdev.dev, "%ux CAN %s%s\n",
    channels, (const char *)ent.driver_data, fw_str);
    icr = readw(cfg_base + PITA_ICR + 2);
    for (i = 0; i < channels; i++) {
    dev = alloc_sja1000dev(sizeof(struct peak_pci_chan));
    if (!dev) {
    err = -ENOMEM;
    goto failure_remove_channels;
    }
    priv = netdev_priv(dev);
    chan = priv.priv;
    chan.cfg_base = cfg_base;
    priv.reg_base = reg_base + i * PEAK_PCI_CHAN_SIZE;
    priv.read_reg = peak_pci_read_reg;
    priv.write_reg = peak_pci_write_reg;
    priv.post_irq = peak_pci_post_irq;
    priv.can.clock.freq = PEAK_PCI_CAN_CLOCK;
    priv.ocr = PEAK_PCI_OCR;
    priv.cdr = PEAK_PCI_CDR;
// Neither a slave nor a single device distributes the clock
    if (channels == 1 || i > 0)
    priv.cdr |= CDR_CLK_OFF;
// Setup interrupt handling
    priv.irq_flags = IRQF_SHARED;
    dev.irq = pdev.irq;
    chan.icr_mask = peak_pci_icr_masks[i];
    icr |= chan.icr_mask;
    SET_NETDEV_DEV(dev, &pdev.dev);
    dev.dev_id = i;
// Create chain of SJA1000 devices
    chan.prev_dev = pci_get_drvdata(pdev);
    pci_set_drvdata(pdev, dev);
// PCAN-ExpressCard needs some additional i2c init.
// This must be done *before* register_sja1000dev() but
// *after* devices linkage
//
    if (pdev.device == PEAK_PCIEC_DEVICE_ID ||
    pdev.device == PEAK_PCIEC34_DEVICE_ID) {
    err = peak_pciec_probe(pdev, dev);
    if (err) {
    dev_err(&pdev.dev,
    "failed to probe device (err %d)\n",
    err);
    goto failure_free_dev;
    }
    }
    err = register_sja1000dev(dev);
    if (err) {
    dev_err(&pdev.dev, "failed to register device\n");
    goto failure_free_dev;
    }
    dev_info(&pdev.dev,
    "%s at reg_base=0x%p cfg_base=0x%p irq=%d\n",
    dev.name, priv.reg_base, chan.cfg_base, dev.irq);
    }
// Enable interrupts
    writew(icr, cfg_base + PITA_ICR + 2);
    return 0;
    failure_free_dev:
    pci_set_drvdata(pdev, chan.prev_dev);
    free_sja1000dev(dev);
    failure_remove_channels:
// Disable interrupts
    writew(0x0, cfg_base + PITA_ICR + 2);
    chan = core::ptr::null_mut();
    for (dev = pci_get_drvdata(pdev); dev; dev = prev_dev) {
    priv = netdev_priv(dev);
    chan = priv.priv;
    prev_dev = chan.prev_dev;
    unregister_sja1000dev(dev);
    free_sja1000dev(dev);
    }
// free any PCIeC resources too
    if (chan && chan.pciec_card)
    peak_pciec_remove(chan.pciec_card);
    pci_iounmap(pdev, reg_base);
    failure_unmap_cfg_base:
    pci_iounmap(pdev, cfg_base);
    failure_release_regions:
    pci_release_regions(pdev);
    failure_disable_pci:
    pci_disable_device(pdev);
// pci_xxx_config_word() return positive PCIBIOS_xxx error codes while
// the probe() function must return a negative errno in case of failure
// (err is unchanged if negative)
//
    return pcibios_err_to_errno(err);
    }
#[no_mangle]
unsafe extern "C" fn peak_pci_remove(pdev: *mut pci_dev) {
    static void peak_pci_remove(struct pci_dev *pdev)
    {
    struct net_device *dev = pci_get_drvdata(pdev); /* Last device */
    struct sja1000_priv *priv = netdev_priv(dev);
    struct peak_pci_chan *chan = priv.priv;
    void __iomem *cfg_base = chan.cfg_base;
    void __iomem *reg_base = priv.reg_base;
// Disable interrupts
    writew(0x0, cfg_base + PITA_ICR + 2);
// Loop over all registered devices
    while (1) {
    struct net_device *prev_dev = chan.prev_dev;
    dev_info(&pdev.dev, "removing device %s\n", dev.name);
// do that only for first channel
    if (!prev_dev && chan.pciec_card)
    peak_pciec_remove(chan.pciec_card);
    unregister_sja1000dev(dev);
    free_sja1000dev(dev);
    dev = prev_dev;
    if (!dev)
    break;
    priv = netdev_priv(dev);
    chan = priv.priv;
    }
    pci_iounmap(pdev, reg_base);
    pci_iounmap(pdev, cfg_base);
    pci_release_regions(pdev);
    pci_disable_device(pdev);
    }
    static struct pci_driver peak_pci_driver = {
    .name = DRV_NAME,
    .id_table = peak_pci_tbl,
    .probe = peak_pci_probe,
    .remove = peak_pci_remove,
    };
    module_pci_driver(peak_pci_driver);
