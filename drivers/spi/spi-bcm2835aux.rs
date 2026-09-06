//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-bcm2835aux.c
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
// Driver for Broadcom BCM2835 auxiliary SPI Controllers
//
// the driver does not rely on the native chipselects at all
// but only uses the gpio type chipselects
//
// Based on: spi-bcm2835.c
//
// Copyright (C) 2015 Martin Sperl
//

// define polling limits
    let mut polling_limit_us: static unsigned int = 30;
    module_param(polling_limit_us, uint, 0664);
    MODULE_PARM_DESC(polling_limit_us,
    "time in us to run a transfer in polling mode - if zero no polling is used\n");
//
// spi register defines
//
// note there is garbage in the "official" documentation,
// so some data is taken from the file:
// brcm_usrlib/dag/vmcsx/vcinclude/bcm2708_chip/aux_io.h
// inside of:
// http://www.broadcom.com/docs/support/videocore/Brcm_Android_ICS_Graphics_Stack.tar.gz
//
// SPI register offsets
pub const BCM2835_AUX_SPI_CNTL0: c_uint = 0x00;
pub const BCM2835_AUX_SPI_CNTL1: c_uint = 0x04;
pub const BCM2835_AUX_SPI_STAT: c_uint = 0x08;
pub const BCM2835_AUX_SPI_PEEK: c_uint = 0x0C;
pub const BCM2835_AUX_SPI_IO: c_uint = 0x20;
pub const BCM2835_AUX_SPI_TXHOLD: c_uint = 0x30;
// Bitfields in CNTL0
pub const BCM2835_AUX_SPI_CNTL0_SPEED: c_uint = 0xFFF00000;
pub const BCM2835_AUX_SPI_CNTL0_SPEED_MAX: c_uint = 0xFFF;
pub const BCM2835_AUX_SPI_CNTL0_SPEED_SHIFT: c_int = 20;
pub const BCM2835_AUX_SPI_CNTL0_CS: c_uint = 0x000E0000;
pub const BCM2835_AUX_SPI_CNTL0_POSTINPUT: c_uint = 0x00010000;
pub const BCM2835_AUX_SPI_CNTL0_VAR_CS: c_uint = 0x00008000;
pub const BCM2835_AUX_SPI_CNTL0_VAR_WIDTH: c_uint = 0x00004000;
pub const BCM2835_AUX_SPI_CNTL0_DOUTHOLD: c_uint = 0x00003000;
pub const BCM2835_AUX_SPI_CNTL0_ENABLE: c_uint = 0x00000800;
pub const BCM2835_AUX_SPI_CNTL0_IN_RISING: c_uint = 0x00000400;
pub const BCM2835_AUX_SPI_CNTL0_CLEARFIFO: c_uint = 0x00000200;
pub const BCM2835_AUX_SPI_CNTL0_OUT_RISING: c_uint = 0x00000100;
pub const BCM2835_AUX_SPI_CNTL0_CPOL: c_uint = 0x00000080;
pub const BCM2835_AUX_SPI_CNTL0_MSBF_OUT: c_uint = 0x00000040;
pub const BCM2835_AUX_SPI_CNTL0_SHIFTLEN: c_uint = 0x0000003F;
// Bitfields in CNTL1
pub const BCM2835_AUX_SPI_CNTL1_CSHIGH: c_uint = 0x00000700;
pub const BCM2835_AUX_SPI_CNTL1_TXEMPTY: c_uint = 0x00000080;
pub const BCM2835_AUX_SPI_CNTL1_IDLE: c_uint = 0x00000040;
pub const BCM2835_AUX_SPI_CNTL1_MSBF_IN: c_uint = 0x00000002;
pub const BCM2835_AUX_SPI_CNTL1_KEEP_IN: c_uint = 0x00000001;
// Bitfields in STAT
pub const BCM2835_AUX_SPI_STAT_TX_LVL: c_uint = 0xFF000000;
pub const BCM2835_AUX_SPI_STAT_RX_LVL: c_uint = 0x00FF0000;
pub const BCM2835_AUX_SPI_STAT_TX_FULL: c_uint = 0x00000400;
pub const BCM2835_AUX_SPI_STAT_TX_EMPTY: c_uint = 0x00000200;
pub const BCM2835_AUX_SPI_STAT_RX_FULL: c_uint = 0x00000100;
pub const BCM2835_AUX_SPI_STAT_RX_EMPTY: c_uint = 0x00000080;
pub const BCM2835_AUX_SPI_STAT_BUSY: c_uint = 0x00000040;
pub const BCM2835_AUX_SPI_STAT_BITCOUNT: c_uint = 0x0000003F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm2835aux_spi {
    pub regs: *mut void __iomem,
    pub clk: *mut clk,
    pub irq: c_int,
    pub cntl: [u32; 2],
    pub tx_buf: *const u8,
    pub rx_buf: *mut u8,
    pub tx_len: c_int,
    pub rx_len: c_int,
    pub pending: c_int,
    pub count_transfer_polling: u64,
    pub count_transfer_irq: u64,
    pub count_transfer_irq_after_poll: u64,
    pub debugfs_dir: *mut dentry,
}

    static void bcm2835aux_debugfs_create(struct bcm2835aux_spi *bs,
    const char *dname)
    {
    char name[64];
    struct dentry *dir;
// get full name
    snprintf(name, sizeof(name), "spi-bcm2835aux-%s", dname);
// the base directory
    dir = debugfs_create_dir(name, core::ptr::null_mut());
    bs.debugfs_dir = dir;
// the counters
    debugfs_create_u64("count_transfer_polling", 0444, dir,
    &bs.count_transfer_polling);
    debugfs_create_u64("count_transfer_irq", 0444, dir,
    &bs.count_transfer_irq);
    debugfs_create_u64("count_transfer_irq_after_poll", 0444, dir,
    &bs.count_transfer_irq_after_poll);
    }
#[no_mangle]
unsafe extern "C" fn bcm2835aux_debugfs_remove(bs: *mut bcm2835aux_spi) {
    static void bcm2835aux_debugfs_remove(struct bcm2835aux_spi *bs)
    {
    debugfs_remove_recursive(bs.debugfs_dir);
    bs.debugfs_dir = core::ptr::null_mut();
    }

    static void bcm2835aux_debugfs_create(struct bcm2835aux_spi *bs,
    const char *dname)
    {
    }
#[no_mangle]
unsafe extern "C" fn bcm2835aux_debugfs_remove(bs: *mut bcm2835aux_spi) {
    static void bcm2835aux_debugfs_remove(struct bcm2835aux_spi *bs)
    {
    }

#[no_mangle]
pub unsafe extern "C" fn bcm2835aux_rd(bs: *mut bcm2835aux_spi, reg: c_uint) -> u32 {
    static inline u32 bcm2835aux_rd(struct bcm2835aux_spi *bs, unsigned int reg)
    {
    return readl(bs.regs + reg);
    }
    static inline void bcm2835aux_wr(struct bcm2835aux_spi *bs, unsigned int reg,
    u32 val)
    {
    writel(val, bs.regs + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn bcm2835aux_rd_fifo(bs: *mut bcm2835aux_spi) {
    static inline void bcm2835aux_rd_fifo(struct bcm2835aux_spi *bs)
    {
    u32 data;
    let mut count: c_int = min(bs.rx_len, 3);
    data = bcm2835aux_rd(bs, BCM2835_AUX_SPI_IO);
    if (bs.rx_buf) {
    switch (count) {
    case 3:
// bs->rx_buf++ = (data >> 16) & 0xff;
    fallthrough;
    case 2:
// bs->rx_buf++ = (data >> 8) & 0xff;
    fallthrough;
    case 1:
// bs->rx_buf++ = (data >> 0) & 0xff;
// fallthrough - no default
    }
    }
    bs.rx_len -= count;
    bs.pending -= count;
    }
#[no_mangle]
pub unsafe extern "C" fn bcm2835aux_wr_fifo(bs: *mut bcm2835aux_spi) {
    static inline void bcm2835aux_wr_fifo(struct bcm2835aux_spi *bs)
    {
    u32 data;
    u8 byte;
    int count;
    int i;
// gather up to 3 bytes to write to the FIFO
    count = min(bs.tx_len, 3);
    data = 0;
    for (i = 0; i < count; i++) {
    byte = bs.tx_buf ? *bs.tx_buf++ : 0;
    data |= byte << (8 * (2 - i));
    }
// and set the variable bit-length
    data |= (count * 8) << 24;
// and decrement length
    bs.tx_len -= count;
    bs.pending += count;
// write to the correct TX-register
    if (bs.tx_len)
    bcm2835aux_wr(bs, BCM2835_AUX_SPI_TXHOLD, data);
    else
    bcm2835aux_wr(bs, BCM2835_AUX_SPI_IO, data);
    }
#[no_mangle]
unsafe extern "C" fn bcm2835aux_spi_reset_hw(bs: *mut bcm2835aux_spi) {
    static void bcm2835aux_spi_reset_hw(struct bcm2835aux_spi *bs)
    {
// disable spi clearing fifo and interrupts
    bcm2835aux_wr(bs, BCM2835_AUX_SPI_CNTL1, 0);
    bcm2835aux_wr(bs, BCM2835_AUX_SPI_CNTL0,
    BCM2835_AUX_SPI_CNTL0_CLEARFIFO);
    }
#[no_mangle]
unsafe extern "C" fn bcm2835aux_spi_transfer_helper(bs: *mut bcm2835aux_spi) {
    static void bcm2835aux_spi_transfer_helper(struct bcm2835aux_spi *bs)
    {
    let mut stat: u32 = bcm2835aux_rd(bs, BCM2835_AUX_SPI_STAT);
// check if we have data to read
    for (; bs.rx_len && (stat & BCM2835_AUX_SPI_STAT_RX_LVL);
    stat = bcm2835aux_rd(bs, BCM2835_AUX_SPI_STAT))
    bcm2835aux_rd_fifo(bs);
// check if we have data to write
    while (bs.tx_len &&
    (bs.pending < 12) &&
    (!(bcm2835aux_rd(bs, BCM2835_AUX_SPI_STAT) &
    BCM2835_AUX_SPI_STAT_TX_FULL))) {
    bcm2835aux_wr_fifo(bs);
    }
    }
#[no_mangle]
unsafe extern "C" fn bcm2835aux_spi_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t bcm2835aux_spi_interrupt(int irq, void *dev_id)
    {
    struct spi_controller *host = dev_id;
    struct bcm2835aux_spi *bs = spi_controller_get_devdata(host);
// IRQ may be shared, so return if our interrupts are disabled
    if (!(bcm2835aux_rd(bs, BCM2835_AUX_SPI_CNTL1) &
    (BCM2835_AUX_SPI_CNTL1_TXEMPTY | BCM2835_AUX_SPI_CNTL1_IDLE)))
    return IRQ_NONE;
// do common fifo handling
    bcm2835aux_spi_transfer_helper(bs);
    if (!bs.tx_len) {
// disable tx fifo empty interrupt
    bcm2835aux_wr(bs, BCM2835_AUX_SPI_CNTL1, bs.cntl[1] |
    BCM2835_AUX_SPI_CNTL1_IDLE);
    }
// and if rx_len is 0 then disable interrupts and wake up completion
    if (!bs.rx_len) {
    bcm2835aux_wr(bs, BCM2835_AUX_SPI_CNTL1, bs.cntl[1]);
    spi_finalize_current_transfer(host);
    }
    return IRQ_HANDLED;
    }
    static int __bcm2835aux_spi_transfer_one_irq(struct spi_controller *host,
    struct spi_device *spi,
    struct spi_transfer *tfr)
    {
    struct bcm2835aux_spi *bs = spi_controller_get_devdata(host);
// enable interrupts
    bcm2835aux_wr(bs, BCM2835_AUX_SPI_CNTL1, bs.cntl[1] |
    BCM2835_AUX_SPI_CNTL1_TXEMPTY |
    BCM2835_AUX_SPI_CNTL1_IDLE);
// and wait for finish...
    return 1;
    }
    static int bcm2835aux_spi_transfer_one_irq(struct spi_controller *host,
    struct spi_device *spi,
    struct spi_transfer *tfr)
    {
    struct bcm2835aux_spi *bs = spi_controller_get_devdata(host);
// update statistics
    bs.count_transfer_irq++;
// fill in registers and fifos before enabling interrupts
    bcm2835aux_wr(bs, BCM2835_AUX_SPI_CNTL1, bs.cntl[1]);
    bcm2835aux_wr(bs, BCM2835_AUX_SPI_CNTL0, bs.cntl[0]);
// fill in tx fifo with data before enabling interrupts
    while ((bs.tx_len) &&
    (bs.pending < 12) &&
    (!(bcm2835aux_rd(bs, BCM2835_AUX_SPI_STAT) &
    BCM2835_AUX_SPI_STAT_TX_FULL))) {
    bcm2835aux_wr_fifo(bs);
    }
// now run the interrupt mode
    return __bcm2835aux_spi_transfer_one_irq(host, spi, tfr);
    }
    static int bcm2835aux_spi_transfer_one_poll(struct spi_controller *host,
    struct spi_device *spi,
    struct spi_transfer *tfr)
    {
    struct bcm2835aux_spi *bs = spi_controller_get_devdata(host);
    unsigned long timeout;
// update statistics
    bs.count_transfer_polling++;
// configure spi
    bcm2835aux_wr(bs, BCM2835_AUX_SPI_CNTL1, bs.cntl[1]);
    bcm2835aux_wr(bs, BCM2835_AUX_SPI_CNTL0, bs.cntl[0]);
// set the timeout to at least 2 jiffies
    timeout = jiffies + 2 + HZ * polling_limit_us / 1000000;
// loop until finished the transfer
    while (bs.rx_len) {
// do common fifo handling
    bcm2835aux_spi_transfer_helper(bs);
// there is still data pending to read check the timeout
    if (bs.rx_len && time_after(jiffies, timeout)) {
    dev_dbg_ratelimited(&spi.dev,
    "timeout period reached: jiffies: %lu remaining tx/rx: %d/%d - falling back to interrupt mode\n",
    jiffies - timeout,
    bs.tx_len, bs.rx_len);
// forward to interrupt handler
    bs.count_transfer_irq_after_poll++;
    return __bcm2835aux_spi_transfer_one_irq(host,
    spi, tfr);
    }
    }
// and return without waiting for completion
    return 0;
    }
    static int bcm2835aux_spi_transfer_one(struct spi_controller *host,
    struct spi_device *spi,
    struct spi_transfer *tfr)
    {
    struct bcm2835aux_spi *bs = spi_controller_get_devdata(host);
    unsigned long spi_hz, clk_hz, speed;
    unsigned long hz_per_byte, byte_limit;
// calculate the registers to handle
//
// note that we use the variable data mode, which
// is not optimal for longer transfers as we waste registers
// resulting (potentially) in more interrupts when transferring
// more than 12 bytes
//
// set clock
    spi_hz = tfr.speed_hz;
    clk_hz = clk_get_rate(bs.clk);
    if (spi_hz >= clk_hz / 2) {
    speed = 0;
    } else if (spi_hz) {
    speed = DIV_ROUND_UP(clk_hz, 2 * spi_hz) - 1;
    if (speed >  BCM2835_AUX_SPI_CNTL0_SPEED_MAX)
    speed = BCM2835_AUX_SPI_CNTL0_SPEED_MAX;
    } else { /* the slowest we can go */
    speed = BCM2835_AUX_SPI_CNTL0_SPEED_MAX;
    }
// mask out old speed from previous spi_transfer
    bs.cntl[0] &= ~(BCM2835_AUX_SPI_CNTL0_SPEED);
// set the new speed
    bs.cntl[0] |= speed << BCM2835_AUX_SPI_CNTL0_SPEED_SHIFT;
    tfr.effective_speed_hz = clk_hz / (2 * (speed + 1));
// set transmit buffers and length
    bs.tx_buf = tfr.tx_buf;
    bs.rx_buf = tfr.rx_buf;
    bs.tx_len = tfr.len;
    bs.rx_len = tfr.len;
    bs.pending = 0;
// Calculate the estimated time in us the transfer runs.  Note that
// there are 2 idle clocks cycles after each chunk getting
// transferred - in our case the chunk size is 3 bytes, so we
// approximate this by 9 cycles/byte.  This is used to find the number
// of Hz per byte per polling limit.  E.g., we can transfer 1 byte in
// 30 µs per 300,000 Hz of bus clock.
//
    hz_per_byte = polling_limit_us ? (9 * 1000000) / polling_limit_us : 0;
    byte_limit = hz_per_byte ? tfr.effective_speed_hz / hz_per_byte : 1;
// run in polling mode for short transfers
    if (tfr.len < byte_limit)
    return bcm2835aux_spi_transfer_one_poll(host, spi, tfr);
// run in interrupt mode for all others
    return bcm2835aux_spi_transfer_one_irq(host, spi, tfr);
    }
    static int bcm2835aux_spi_prepare_message(struct spi_controller *host,
    struct spi_message *msg)
    {
    struct spi_device *spi = msg.spi;
    struct bcm2835aux_spi *bs = spi_controller_get_devdata(host);
    bs.cntl[0] = BCM2835_AUX_SPI_CNTL0_ENABLE |
    BCM2835_AUX_SPI_CNTL0_VAR_WIDTH |
    BCM2835_AUX_SPI_CNTL0_MSBF_OUT;
    bs.cntl[1] = BCM2835_AUX_SPI_CNTL1_MSBF_IN;
// handle all the modes
    if (spi.mode & SPI_CPOL) {
    bs.cntl[0] |= BCM2835_AUX_SPI_CNTL0_CPOL;
    bs.cntl[0] |= BCM2835_AUX_SPI_CNTL0_OUT_RISING;
    } else {
    bs.cntl[0] |= BCM2835_AUX_SPI_CNTL0_IN_RISING;
    }
    bcm2835aux_wr(bs, BCM2835_AUX_SPI_CNTL1, bs.cntl[1]);
    bcm2835aux_wr(bs, BCM2835_AUX_SPI_CNTL0, bs.cntl[0]);
    return 0;
    }
    static int bcm2835aux_spi_unprepare_message(struct spi_controller *host,
    struct spi_message *msg)
    {
    struct bcm2835aux_spi *bs = spi_controller_get_devdata(host);
    bcm2835aux_spi_reset_hw(bs);
    return 0;
    }
    static void bcm2835aux_spi_handle_err(struct spi_controller *host,
    struct spi_message *msg)
    {
    struct bcm2835aux_spi *bs = spi_controller_get_devdata(host);
    bcm2835aux_spi_reset_hw(bs);
    }
#[no_mangle]
unsafe extern "C" fn bcm2835aux_spi_setup(spi: *mut spi_device) -> c_int {
    static int bcm2835aux_spi_setup(struct spi_device *spi)
    {
// sanity check for native cs
    if (spi.mode & SPI_NO_CS)
    return 0;
    if (spi_get_csgpiod(spi, 0))
    return 0;
// for dt-backwards compatibility: only support native on CS0
// known things not supported with broken native CS:
// * multiple chip-selects: cs0-cs2 are all
// simultaniously asserted whenever there is a transfer
// this even includes SPI_NO_CS
// * SPI_CS_HIGH: cs are always asserted low
// * cs_change: cs is deasserted after each spi_transfer
// * cs_delay_usec: cs is always deasserted one SCK cycle
// after the last transfer
// probably more...
//
    dev_warn(&spi.dev,
    "Native CS is not supported - please configure cs-gpio in device-tree\n");
    if (spi_get_chipselect(spi, 0) == 0)
    return 0;
    dev_warn(&spi.dev, "Native CS is not working for cs > 0\n");
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn bcm2835aux_spi_probe(pdev: *mut platform_device) -> c_int {
    static int bcm2835aux_spi_probe(struct platform_device *pdev)
    {
    struct spi_controller *host;
    struct bcm2835aux_spi *bs;
    unsigned long clk_hz;
    int err;
    host = devm_spi_alloc_host(&pdev.dev, sizeof(*bs));
    if (!host)
    return -ENOMEM;
    platform_set_drvdata(pdev, host);
    host.mode_bits = (SPI_CPOL | SPI_CS_HIGH | SPI_NO_CS);
    host.bits_per_word_mask = SPI_BPW_MASK(8);
// even though the driver never officially supported native CS
// allow a single native CS for legacy DT support purposes when
// no cs-gpio is configured.
// Known limitations for native cs are:
// * multiple chip-selects: cs0-cs2 are all simultaniously asserted
// whenever there is a transfer -  this even includes SPI_NO_CS
// * SPI_CS_HIGH: is ignores - cs are always asserted low
// * cs_change: cs is deasserted after each spi_transfer
// * cs_delay_usec: cs is always deasserted one SCK cycle after
// a spi_transfer
//
    host.num_chipselect = 1;
    host.setup = bcm2835aux_spi_setup;
    host.transfer_one = bcm2835aux_spi_transfer_one;
    host.handle_err = bcm2835aux_spi_handle_err;
    host.prepare_message = bcm2835aux_spi_prepare_message;
    host.unprepare_message = bcm2835aux_spi_unprepare_message;
    host.use_gpio_descriptors = true;
    bs = spi_controller_get_devdata(host);
// the main area
    bs.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(bs.regs))
    return PTR_ERR(bs.regs);
    bs.clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(bs.clk)) {
    err = PTR_ERR(bs.clk);
    dev_err(&pdev.dev, "could not get clk: %d\n", err);
    return err;
    }
    bs.irq = platform_get_irq(pdev, 0);
    if (bs.irq < 0)
    return bs.irq;
// just checking if the clock returns a sane value
    clk_hz = clk_get_rate(bs.clk);
    if (!clk_hz) {
    dev_err(&pdev.dev, "clock returns 0 Hz\n");
    return -ENODEV;
    }
// reset SPI-HW block
    bcm2835aux_spi_reset_hw(bs);
    err = devm_request_irq(&pdev.dev, bs.irq,
    bcm2835aux_spi_interrupt,
    IRQF_SHARED,
    dev_name(&pdev.dev), host);
    if (err) {
    dev_err(&pdev.dev, "could not request IRQ: %d\n", err);
    return err;
    }
    err = spi_register_controller(host);
    if (err) {
    dev_err(&pdev.dev, "could not register SPI host: %d\n", err);
    return err;
    }
    bcm2835aux_debugfs_create(bs, dev_name(&pdev.dev));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm2835aux_spi_remove(pdev: *mut platform_device) {
    static void bcm2835aux_spi_remove(struct platform_device *pdev)
    {
    struct spi_controller *host = platform_get_drvdata(pdev);
    struct bcm2835aux_spi *bs = spi_controller_get_devdata(host);
    bcm2835aux_debugfs_remove(bs);
    spi_unregister_controller(host);
    bcm2835aux_spi_reset_hw(bs);
    }
    static const struct of_device_id bcm2835aux_spi_match[] = {
    { .compatible = "brcm,bcm2835-aux-spi", },
    {}
    };
    MODULE_DEVICE_TABLE(of, bcm2835aux_spi_match);
    static struct platform_driver bcm2835aux_spi_driver = {
    .driver		= {
    .name		= "spi-bcm2835aux",
    .of_match_table	= bcm2835aux_spi_match,
    },
    .probe		= bcm2835aux_spi_probe,
    .remove		= bcm2835aux_spi_remove,
    };
    module_platform_driver(bcm2835aux_spi_driver);
    MODULE_DESCRIPTION("SPI controller driver for Broadcom BCM2835 aux");
    MODULE_AUTHOR("Martin Sperl <kernel@martin.sperl.org>");
    MODULE_LICENSE("GPL");
