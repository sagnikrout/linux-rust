//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/pluto2/pluto2.c
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
// pluto2.c - Satelco Easywatch Mobile Terrestrial Receiver [DVB-T]
//
// Copyright (C) 2005 Andreas Oberritter <obi@linuxtv.org>
//
// based on pluto2.c 1.10 - http://instinct-wp8.no-ip.org/pluto
// by Dany Salman <salmandany@yahoo.fr>
// Copyright (c) 2004 TDF
//

    DVB_DEFINE_MOD_OPT_ADAPTER_NR(adapter_nr);

pub const REG_PCAR: c_uint = 0x0020		/* PC address register */;
pub const REG_TSCR: c_uint = 0x0024		/* TS ctrl & status */;
pub const REG_MISC: c_uint = 0x0028		/* miscellaneous */;
pub const REG_MMAC: c_uint = 0x002c		/* MSB MAC address */;
pub const REG_IMAC: c_uint = 0x0030		/* ISB MAC address */;
pub const REG_LMAC: c_uint = 0x0034		/* LSB MAC address */;
pub const REG_SPID: c_uint = 0x0038		/* SPI data */;
pub const REG_SLCS: c_uint = 0x003c		/* serial links ctrl/status */;

pub const I2C_ADDR_TDA10046: c_uint = 0x10;
pub const I2C_ADDR_TUA6034: c_uint = 0xc2;
pub const NHWFILTERS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pluto {
// pci
    pub pdev: *mut pci_dev,
    pub io_mem: *mut u8 __iomem,
// dvb
    pub hw_frontend: dmx_frontend,
    pub mem_frontend: dmx_frontend,
    pub dmxdev: dmxdev,
    pub dvb_adapter: dvb_adapter,
    pub demux: dvb_demux,
    pub fe: *mut dvb_frontend,
    pub dvbnet: dvb_net,
    pub full_ts_users: c_uint,
    pub users: c_uint,
// i2c
    pub i2c_bit: i2c_algo_bit_data,
    pub i2c_adap: i2c_adapter,
    pub i2cbug: c_uint,
// irq
    pub overflow: c_uint,
    pub dead: c_uint,
// dma
    pub dma_addr: dma_addr_t,
    pub dma_buf: [u8; TS_DMA_BYTES],
    pub dummy: [u8; 4096],
}

    static inline struct pluto *feed_to_pluto(struct dvb_demux_feed *feed)
    {
    return container_of(feed.demux, struct pluto, demux);
    }
    static inline struct pluto *frontend_to_pluto(struct dvb_frontend *fe)
    {
    return container_of(fe.dvb, struct pluto, dvb_adapter);
    }
#[no_mangle]
pub unsafe extern "C" fn pluto_readreg(pluto: *mut pluto, reg: u32) -> u32 {
    static inline u32 pluto_readreg(struct pluto *pluto, u32 reg)
    {
    return readl(&pluto.io_mem[reg]);
    }
#[no_mangle]
pub unsafe extern "C" fn pluto_writereg(pluto: *mut pluto, reg: u32, val: u32) {
    static inline void pluto_writereg(struct pluto *pluto, u32 reg, u32 val)
    {
    writel(val, &pluto.io_mem[reg]);
    }
#[no_mangle]
pub unsafe extern "C" fn pluto_rw(pluto: *mut pluto, reg: u32, mask: u32, bits: u32) {
    static inline void pluto_rw(struct pluto *pluto, u32 reg, u32 mask, u32 bits)
    {
    let mut val: u32 = readl(&pluto.io_mem[reg]);
    val &= ~mask;
    val |= bits;
    writel(val, &pluto.io_mem[reg]);
    }
#[no_mangle]
unsafe extern "C" fn pluto_write_tscr(pluto: *mut pluto, val: u32) {
    static void pluto_write_tscr(struct pluto *pluto, u32 val)
    {
// set the number of packets
    val &= ~TSCR_ADEF;
    val |= TS_DMA_PACKETS / 2;
    pluto_writereg(pluto, REG_TSCR, val);
    }
#[no_mangle]
unsafe extern "C" fn pluto_setsda(data: *mut c_void, state: c_int) {
    static void pluto_setsda(void *data, int state)
    {
    struct pluto *pluto = data;
    if (state)
    pluto_rw(pluto, REG_SLCS, SLCS_SDA, SLCS_SDA);
    else
    pluto_rw(pluto, REG_SLCS, SLCS_SDA, 0);
    }
#[no_mangle]
unsafe extern "C" fn pluto_setscl(data: *mut c_void, state: c_int) {
    static void pluto_setscl(void *data, int state)
    {
    struct pluto *pluto = data;
    if (state)
    pluto_rw(pluto, REG_SLCS, SLCS_SCL, SLCS_SCL);
    else
    pluto_rw(pluto, REG_SLCS, SLCS_SCL, 0);
// try to detect i2c_inb() to workaround hardware bug:
// reset SDA to high after SCL has been set to low
    if ((state) && (pluto.i2cbug == 0)) {
    pluto.i2cbug = 1;
    } else {
    if ((!state) && (pluto.i2cbug == 1))
    pluto_setsda(pluto, 1);
    pluto.i2cbug = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn pluto_getsda(data: *mut c_void) -> c_int {
    static int pluto_getsda(void *data)
    {
    struct pluto *pluto = data;
    return pluto_readreg(pluto, REG_SLCS) & SLCS_SDA;
    }
#[no_mangle]
unsafe extern "C" fn pluto_getscl(data: *mut c_void) -> c_int {
    static int pluto_getscl(void *data)
    {
    struct pluto *pluto = data;
    return pluto_readreg(pluto, REG_SLCS) & SLCS_SCL;
    }
#[no_mangle]
unsafe extern "C" fn pluto_reset_frontend(pluto: *mut pluto, reenable: c_int) {
    static void pluto_reset_frontend(struct pluto *pluto, int reenable)
    {
    let mut val: u32 = pluto_readreg(pluto, REG_MISC);
    if (val & MISC_FRST) {
    val &= ~MISC_FRST;
    pluto_writereg(pluto, REG_MISC, val);
    }
    if (reenable) {
    val |= MISC_FRST;
    pluto_writereg(pluto, REG_MISC, val);
    }
    }
#[no_mangle]
unsafe extern "C" fn pluto_reset_ts(pluto: *mut pluto, reenable: c_int) {
    static void pluto_reset_ts(struct pluto *pluto, int reenable)
    {
    let mut val: u32 = pluto_readreg(pluto, REG_TSCR);
    if (val & TSCR_RSTN) {
    val &= ~TSCR_RSTN;
    pluto_write_tscr(pluto, val);
    }
    if (reenable) {
    val |= TSCR_RSTN;
    pluto_write_tscr(pluto, val);
    }
    }
#[no_mangle]
unsafe extern "C" fn pluto_set_dma_addr(pluto: *mut pluto) {
    static void pluto_set_dma_addr(struct pluto *pluto)
    {
    pluto_writereg(pluto, REG_PCAR, pluto.dma_addr);
    }
#[no_mangle]
unsafe extern "C" fn pluto_dma_map(pluto: *mut pluto) -> c_int {
    static int pluto_dma_map(struct pluto *pluto)
    {
    pluto.dma_addr = dma_map_single(&pluto.pdev.dev, pluto.dma_buf,
    TS_DMA_BYTES, DMA_FROM_DEVICE);
    return dma_mapping_error(&pluto.pdev.dev, pluto.dma_addr);
    }
#[no_mangle]
unsafe extern "C" fn pluto_dma_unmap(pluto: *mut pluto) {
    static void pluto_dma_unmap(struct pluto *pluto)
    {
    dma_unmap_single(&pluto.pdev.dev, pluto.dma_addr, TS_DMA_BYTES,
    DMA_FROM_DEVICE);
    }
#[no_mangle]
unsafe extern "C" fn pluto_start_feed(f: *mut dvb_demux_feed) -> c_int {
    static int pluto_start_feed(struct dvb_demux_feed *f)
    {
    struct pluto *pluto = feed_to_pluto(f);
// enable PID filtering
    if (pluto.users++ == 0)
    pluto_rw(pluto, REG_PIDn(0), PID0_AFIL | PID0_NOFIL, 0);
    if ((f.pid < 0x2000) && (f.index < NHWFILTERS))
    pluto_rw(pluto, REG_PIDn(f.index), PIDn_ENP | PIDn_PID, PIDn_ENP | f.pid);
#[no_mangle]
pub unsafe extern "C" fn if(0: pluto->full_ts_users++ ==) -> else {
    else if (pluto.full_ts_users++ == 0)
    pluto_rw(pluto, REG_PIDn(0), PID0_NOFIL, PID0_NOFIL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pluto_stop_feed(f: *mut dvb_demux_feed) -> c_int {
    static int pluto_stop_feed(struct dvb_demux_feed *f)
    {
    struct pluto *pluto = feed_to_pluto(f);
// disable PID filtering
    if (--pluto.users == 0)
    pluto_rw(pluto, REG_PIDn(0), PID0_AFIL, PID0_AFIL);
    if ((f.pid < 0x2000) && (f.index < NHWFILTERS))
    pluto_rw(pluto, REG_PIDn(f.index), PIDn_ENP | PIDn_PID, 0x1fff);
#[no_mangle]
pub unsafe extern "C" fn if(0: --pluto->full_ts_users ==) -> else {
    else if (--pluto.full_ts_users == 0)
    pluto_rw(pluto, REG_PIDn(0), PID0_NOFIL, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pluto_dma_end(pluto: *mut pluto, nbpackets: c_uint) {
    static void pluto_dma_end(struct pluto *pluto, unsigned int nbpackets)
    {
// synchronize the DMA transfer with the CPU
// first so that we see updated contents.
    dma_sync_single_for_cpu(&pluto.pdev.dev, pluto.dma_addr,
    TS_DMA_BYTES, DMA_FROM_DEVICE);
// Workaround for broken hardware:
// [1] On startup NBPACKETS seems to contain an uninitialized value,
// but no packets have been transferred.
// [2] Sometimes (actually very often) NBPACKETS stays at zero
// although one packet has been transferred.
// [3] Sometimes (actually rarely), the card gets into an erroneous
// mode where it continuously generates interrupts, claiming it
// has received nbpackets>TS_DMA_PACKETS packets, but no packet
// has been transferred. Only a reset seems to solve this
//
    if ((nbpackets == 0) || (nbpackets > TS_DMA_PACKETS)) {
    let mut i: c_uint = 0;
    while (pluto.dma_buf[i] == 0x47)
    i += 188;
    nbpackets = i / 188;
    if (i == 0) {
    pluto_reset_ts(pluto, 1);
    dev_printk(KERN_DEBUG, &pluto.pdev.dev, "resetting TS because of invalid packet counter\n");
    }
    }
    dvb_dmx_swfilter_packets(&pluto.demux, pluto.dma_buf, nbpackets);
// clear the dma buffer. this is needed to be able to identify
// new valid ts packets above
    memset(pluto.dma_buf, 0, nbpackets * 188);
// reset the dma address
    pluto_set_dma_addr(pluto);
// sync the buffer and give it back to the card
    dma_sync_single_for_device(&pluto.pdev.dev, pluto.dma_addr,
    TS_DMA_BYTES, DMA_FROM_DEVICE);
    }
#[no_mangle]
unsafe extern "C" fn pluto_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t pluto_irq(int irq, void *dev_id)
    {
    struct pluto *pluto = dev_id;
    u32 tscr;
// check whether an interrupt occurred on this device
    tscr = pluto_readreg(pluto, REG_TSCR);
    if (!(tscr & (TSCR_DE | TSCR_OVR)))
    return IRQ_NONE;
    if (tscr == 0xffffffff) {
    if (pluto.dead == 0)
    dev_err(&pluto.pdev.dev, "card has hung or been ejected.\n");
// It's dead Jim
    pluto.dead = 1;
    return IRQ_HANDLED;
    }
// dma end interrupt
    if (tscr & TSCR_DE) {
    pluto_dma_end(pluto, (tscr & TSCR_NBPACKETS) >> 24);
// overflow interrupt
    if (tscr & TSCR_OVR)
    pluto.overflow++;
    if (pluto.overflow) {
    dev_err(&pluto.pdev.dev, "overflow irq (%d)\n",
    pluto.overflow);
    pluto_reset_ts(pluto, 1);
    pluto.overflow = 0;
    }
    } else if (tscr & TSCR_OVR) {
    pluto.overflow++;
    }
// ACK the interrupt
    pluto_write_tscr(pluto, tscr | TSCR_IACK);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pluto_enable_irqs(pluto: *mut pluto) {
    static void pluto_enable_irqs(struct pluto *pluto)
    {
    let mut val: u32 = pluto_readreg(pluto, REG_TSCR);
// disable AFUL and LOCK interrupts
    val |= (TSCR_MSKA | TSCR_MSKL);
// enable DMA and OVERFLOW interrupts
    val &= ~(TSCR_DEM | TSCR_MSKO);
// clear pending interrupts
    val |= TSCR_IACK;
    pluto_write_tscr(pluto, val);
    }
#[no_mangle]
unsafe extern "C" fn pluto_disable_irqs(pluto: *mut pluto) {
    static void pluto_disable_irqs(struct pluto *pluto)
    {
    let mut val: u32 = pluto_readreg(pluto, REG_TSCR);
// disable all interrupts
    val |= (TSCR_DEM | TSCR_MSKO | TSCR_MSKA | TSCR_MSKL);
// clear pending interrupts
    val |= TSCR_IACK;
    pluto_write_tscr(pluto, val);
    }
#[no_mangle]
unsafe extern "C" fn pluto_hw_init(pluto: *mut pluto) -> c_int {
    static int pluto_hw_init(struct pluto *pluto)
    {
    pluto_reset_frontend(pluto, 1);
// set automatic LED control by FPGA
    pluto_rw(pluto, REG_MISC, MISC_ALED, MISC_ALED);
// set data endianness

    pluto_rw(pluto, REG_PIDn(0), PID0_END, PID0_END);

    pluto_rw(pluto, REG_PIDn(0), PID0_END, 0);

// map DMA and set address
    pluto_dma_map(pluto);
    pluto_set_dma_addr(pluto);
// enable interrupts
    pluto_enable_irqs(pluto);
// reset TS logic
    pluto_reset_ts(pluto, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pluto_hw_exit(pluto: *mut pluto) {
    static void pluto_hw_exit(struct pluto *pluto)
    {
// disable interrupts
    pluto_disable_irqs(pluto);
    pluto_reset_ts(pluto, 0);
// LED: disable automatic control, enable yellow, disable green
    pluto_rw(pluto, REG_MISC, MISC_ALED | MISC_LED1 | MISC_LED0, MISC_LED1);
// unmap DMA
    pluto_dma_unmap(pluto);
    pluto_reset_frontend(pluto, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn divide(numerator: u32, denominator: u32) -> u32 {
    static inline u32 divide(u32 numerator, u32 denominator)
    {
    if (denominator == 0)
    return ~0;
    return DIV_ROUND_CLOSEST(numerator, denominator);
    }
// LG Innotek TDTE-E001P (Infineon TUA6034)
#[no_mangle]
unsafe extern "C" fn lg_tdtpe001p_tuner_set_params(fe: *mut dvb_frontend) -> c_int {
    static int lg_tdtpe001p_tuner_set_params(struct dvb_frontend *fe)
    {
    struct dtv_frontend_properties *p = &fe.dtv_property_cache;
    struct pluto *pluto = frontend_to_pluto(fe);
    struct i2c_msg msg;
    int ret;
    u8 buf[4];
    u32 div;
// Fref = 166.667 Hz
// Fref * 3 = 500.000 Hz
// IF = 36166667
// IF / Fref = 217
// div = divide(p->frequency + 36166667, 166667);
    div = divide(p.frequency * 3, 500000) + 217;
    buf[0] = (div >> 8) & 0x7f;
    buf[1] = (div >> 0) & 0xff;
    if (p.frequency < 611000000)
    buf[2] = 0xb4;
#[no_mangle]
pub unsafe extern "C" fn if(811000000: p->frequency <) -> else {
    else if (p.frequency < 811000000)
    buf[2] = 0xbc;
    else
    buf[2] = 0xf4;
// VHF: 174-230 MHz
// center: 350 MHz
// UHF: 470-862 MHz
    if (p.frequency < 350000000)
    buf[3] = 0x02;
    else
    buf[3] = 0x04;
    if (p.bandwidth_hz == 8000000)
    buf[3] |= 0x08;
    msg.addr = I2C_ADDR_TUA6034 >> 1;
    msg.flags = 0;
    msg.buf = buf;
    msg.len = sizeof(buf);
    if (fe.ops.i2c_gate_ctrl)
    fe.ops.i2c_gate_ctrl(fe, 1);
    ret = i2c_transfer(&pluto.i2c_adap, &msg, 1);
    if (ret < 0)
    return ret;
#[no_mangle]
pub unsafe extern "C" fn if(0: ret ==) -> else {
    else if (ret == 0)
    return -EREMOTEIO;
    return 0;
    }
    static int pluto2_request_firmware(struct dvb_frontend *fe,
    const struct firmware **fw, char *name)
    {
    struct pluto *pluto = frontend_to_pluto(fe);
    return request_firmware(fw, name, &pluto.pdev.dev);
    }
    static struct tda1004x_config pluto2_fe_config = {
    .demod_address = I2C_ADDR_TDA10046 >> 1,
    .invert = 1,
    .invert_oclk = 0,
    .xtal_freq = TDA10046_XTAL_16M,
    .agc_config = TDA10046_AGC_DEFAULT,
    .if_freq = TDA10046_FREQ_3617,
    .request_firmware = pluto2_request_firmware,
    };
#[no_mangle]
unsafe extern "C" fn frontend_init(pluto: *mut pluto) -> c_int {
    static int frontend_init(struct pluto *pluto)
    {
    int ret;
    pluto.fe = tda10046_attach(&pluto2_fe_config, &pluto.i2c_adap);
    if (!pluto.fe) {
    dev_err(&pluto.pdev.dev, "could not attach frontend\n");
    return -ENODEV;
    }
    pluto.fe.ops.tuner_ops.set_params = lg_tdtpe001p_tuner_set_params;
    ret = dvb_register_frontend(&pluto.dvb_adapter, pluto.fe);
    if (ret < 0) {
    if (pluto.fe.ops.release)
    pluto.fe.ops.release(pluto.fe);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pluto_read_rev(pluto: *mut pluto) {
    static void pluto_read_rev(struct pluto *pluto)
    {
    let mut val: u32 = pluto_readreg(pluto, REG_MISC) & MISC_DVR;
    dev_info(&pluto.pdev.dev, "board revision %d.%d\n",
    (val >> 12) & 0x0f, (val >> 4) & 0xff);
    }
#[no_mangle]
unsafe extern "C" fn pluto_read_mac(pluto: *mut pluto, mac: *mut u8) {
    static void pluto_read_mac(struct pluto *pluto, u8 *mac)
    {
    let mut val: u32 = pluto_readreg(pluto, REG_MMAC);
    mac[0] = (val >> 8) & 0xff;
    mac[1] = (val >> 0) & 0xff;
    val = pluto_readreg(pluto, REG_IMAC);
    mac[2] = (val >> 8) & 0xff;
    mac[3] = (val >> 0) & 0xff;
    val = pluto_readreg(pluto, REG_LMAC);
    mac[4] = (val >> 8) & 0xff;
    mac[5] = (val >> 0) & 0xff;
    dev_info(&pluto.pdev.dev, "MAC %pM\n", mac);
    }
#[no_mangle]
unsafe extern "C" fn pluto_read_serial(pluto: *mut pluto) -> c_int {
    static int pluto_read_serial(struct pluto *pluto)
    {
    struct pci_dev *pdev = pluto.pdev;
    unsigned int i, j;
    u8 __iomem *cis;
    cis = pci_iomap(pdev, 1, 0);
    if (!cis)
    return -EIO;
    dev_info(&pdev.dev, "S/N ");
    for (i = 0xe0; i < 0x100; i += 4) {
    let mut val: u32 = readl(&cis[i]);
    for (j = 0; j < 32; j += 8) {
    if ((val & 0xff) == 0xff)
    goto out;
    printk(KERN_CONT "%c", val & 0xff);
    val >>= 8;
    }
    }
    out:
    printk(KERN_CONT "\n");
    pci_iounmap(pdev, cis);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pluto2_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int pluto2_probe(struct pci_dev *pdev, const struct pci_device_id *ent)
    {
    struct pluto *pluto;
    struct dvb_adapter *dvb_adapter;
    struct dvb_demux *dvbdemux;
    struct dmx_demux *dmx;
    let mut ret: c_int = -ENOMEM;
    pluto = kzalloc_obj(struct pluto);
    if (!pluto)
    goto out;
    pluto.pdev = pdev;
    ret = pci_enable_device(pdev);
    if (ret < 0)
    goto err_kfree;
// enable interrupts
    pci_write_config_dword(pdev, 0x6c, 0x8000);
    ret = dma_set_mask(&pdev.dev, DMA_BIT_MASK(32));
    if (ret < 0)
    goto err_pci_disable_device;
    pci_set_master(pdev);
    ret = pci_request_regions(pdev, DRIVER_NAME);
    if (ret < 0)
    goto err_pci_disable_device;
    pluto.io_mem = pci_iomap(pdev, 0, 0x40);
    if (!pluto.io_mem) {
    ret = -EIO;
    goto err_pci_release_regions;
    }
    pci_set_drvdata(pdev, pluto);
    ret = request_irq(pdev.irq, pluto_irq, IRQF_SHARED, DRIVER_NAME, pluto);
    if (ret < 0)
    goto err_pci_iounmap;
    ret = pluto_hw_init(pluto);
    if (ret < 0)
    goto err_free_irq;
// i2c
    i2c_set_adapdata(&pluto.i2c_adap, pluto);
    strscpy(pluto.i2c_adap.name, DRIVER_NAME, sizeof(pluto.i2c_adap.name));
    pluto.i2c_adap.owner = THIS_MODULE;
    pluto.i2c_adap.dev.parent = &pdev.dev;
    pluto.i2c_adap.algo_data = &pluto.i2c_bit;
    pluto.i2c_bit.data = pluto;
    pluto.i2c_bit.setsda = pluto_setsda;
    pluto.i2c_bit.setscl = pluto_setscl;
    pluto.i2c_bit.getsda = pluto_getsda;
    pluto.i2c_bit.getscl = pluto_getscl;
    pluto.i2c_bit.udelay = 10;
    pluto.i2c_bit.timeout = 10;
// Raise SCL and SDA
    pluto_setsda(pluto, 1);
    pluto_setscl(pluto, 1);
    ret = i2c_bit_add_bus(&pluto.i2c_adap);
    if (ret < 0)
    goto err_pluto_hw_exit;
// dvb
    ret = dvb_register_adapter(&pluto.dvb_adapter, DRIVER_NAME,
    THIS_MODULE, &pdev.dev, adapter_nr);
    if (ret < 0)
    goto err_i2c_del_adapter;
    dvb_adapter = &pluto.dvb_adapter;
    pluto_read_rev(pluto);
    pluto_read_serial(pluto);
    pluto_read_mac(pluto, dvb_adapter.proposed_mac);
    dvbdemux = &pluto.demux;
    dvbdemux.filternum = 256;
    dvbdemux.feednum = 256;
    dvbdemux.start_feed = pluto_start_feed;
    dvbdemux.stop_feed = pluto_stop_feed;
    dvbdemux.dmx.capabilities = (DMX_TS_FILTERING |
    DMX_SECTION_FILTERING | DMX_MEMORY_BASED_FILTERING);
    ret = dvb_dmx_init(dvbdemux);
    if (ret < 0)
    goto err_dvb_unregister_adapter;
    dmx = &dvbdemux.dmx;
    pluto.hw_frontend.source = DMX_FRONTEND_0;
    pluto.mem_frontend.source = DMX_MEMORY_FE;
    pluto.dmxdev.filternum = NHWFILTERS;
    pluto.dmxdev.demux = dmx;
    ret = dvb_dmxdev_init(&pluto.dmxdev, dvb_adapter);
    if (ret < 0)
    goto err_dvb_dmx_release;
    ret = dmx.add_frontend(dmx, &pluto.hw_frontend);
    if (ret < 0)
    goto err_dvb_dmxdev_release;
    ret = dmx.add_frontend(dmx, &pluto.mem_frontend);
    if (ret < 0)
    goto err_remove_hw_frontend;
    ret = dmx.connect_frontend(dmx, &pluto.hw_frontend);
    if (ret < 0)
    goto err_remove_mem_frontend;
    ret = frontend_init(pluto);
    if (ret < 0)
    goto err_disconnect_frontend;
    dvb_net_init(dvb_adapter, &pluto.dvbnet, dmx);
    out:
    return ret;
    err_disconnect_frontend:
    dmx.disconnect_frontend(dmx);
    err_remove_mem_frontend:
    dmx.remove_frontend(dmx, &pluto.mem_frontend);
    err_remove_hw_frontend:
    dmx.remove_frontend(dmx, &pluto.hw_frontend);
    err_dvb_dmxdev_release:
    dvb_dmxdev_release(&pluto.dmxdev);
    err_dvb_dmx_release:
    dvb_dmx_release(dvbdemux);
    err_dvb_unregister_adapter:
    dvb_unregister_adapter(dvb_adapter);
    err_i2c_del_adapter:
    i2c_del_adapter(&pluto.i2c_adap);
    err_pluto_hw_exit:
    pluto_hw_exit(pluto);
    err_free_irq:
    free_irq(pdev.irq, pluto);
    err_pci_iounmap:
    pci_iounmap(pdev, pluto.io_mem);
    err_pci_release_regions:
    pci_release_regions(pdev);
    err_pci_disable_device:
    pci_disable_device(pdev);
    err_kfree:
    kfree(pluto);
    goto out;
    }
#[no_mangle]
unsafe extern "C" fn pluto2_remove(pdev: *mut pci_dev) {
    static void pluto2_remove(struct pci_dev *pdev)
    {
    struct pluto *pluto = pci_get_drvdata(pdev);
    struct dvb_adapter *dvb_adapter = &pluto.dvb_adapter;
    struct dvb_demux *dvbdemux = &pluto.demux;
    struct dmx_demux *dmx = &dvbdemux.dmx;
    dmx.close(dmx);
    dvb_net_release(&pluto.dvbnet);
    if (pluto.fe)
    dvb_unregister_frontend(pluto.fe);
    dmx.disconnect_frontend(dmx);
    dmx.remove_frontend(dmx, &pluto.mem_frontend);
    dmx.remove_frontend(dmx, &pluto.hw_frontend);
    dvb_dmxdev_release(&pluto.dmxdev);
    dvb_dmx_release(dvbdemux);
    dvb_unregister_adapter(dvb_adapter);
    i2c_del_adapter(&pluto.i2c_adap);
    pluto_hw_exit(pluto);
    free_irq(pdev.irq, pluto);
    pci_iounmap(pdev, pluto.io_mem);
    pci_release_regions(pdev);
    pci_disable_device(pdev);
    kfree(pluto);
    }

pub const PCI_VENDOR_ID_SCM: c_uint = 0x0432;

pub const PCI_DEVICE_ID_PLUTO2: c_uint = 0x0001;

    static const struct pci_device_id pluto2_id_table[] = {
    {
    PCI_VDEVICE(SCM, PCI_DEVICE_ID_PLUTO2),
    }, {
// empty
    },
    };
    MODULE_DEVICE_TABLE(pci, pluto2_id_table);
    static struct pci_driver pluto2_driver = {
    .name = DRIVER_NAME,
    .id_table = pluto2_id_table,
    .probe = pluto2_probe,
    .remove = pluto2_remove,
    };
    module_pci_driver(pluto2_driver);
    MODULE_AUTHOR("Andreas Oberritter <obi@linuxtv.org>");
    MODULE_DESCRIPTION("Pluto2 driver");
    MODULE_LICENSE("GPL");
