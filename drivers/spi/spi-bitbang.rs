//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-bitbang.c
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
// Polling/bitbanging SPI host controller controller driver utilities
//

pub const SPI_BITBANG_CS_DELAY: c_int = 100;
// ----------------------------------------------------------------------
//
// FIRST PART (OPTIONAL):  word-at-a-time spi_transfer support.
// Use this for GPIO or shift-register level hardware APIs.
//
// spi_bitbang_cs is in spi_device->controller_state, which is unavailable
// to glue code.  These bitbang setup() and cleanup() routines are always
// used, though maybe they're called from controller-aware code.
//
// chipselect() and friends may use spi_device->controller_data and
// controller registers as appropriate.
//
// NOTE:  SPI controller pins can often be used as GPIO pins instead,
// which means you could use a bitbang driver either to get hardware
// working quickly, or testing for differences that aren't speed related.
//
    typedef unsigned int (*spi_bb_txrx_bufs_fn)(struct spi_device *, spi_bb_txrx_word_fn,
    unsigned int, struct spi_transfer *,
    unsigned int);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_bitbang_cs {
    pub /: *mut *mut unsigned int nsecs; / (clock cycle time) / 2,
    pub txrx_word: spi_bb_txrx_word_fn,
    pub txrx_bufs: spi_bb_txrx_bufs_fn,
}

    static unsigned int bitbang_txrx_8(struct spi_device *spi,
    spi_bb_txrx_word_fn txrx_word,
    unsigned int ns,
    struct spi_transfer	*t,
    unsigned int flags)
    {
    struct spi_bitbang	*bitbang;
    let mut bits: c_uint = t.bits_per_word;
    let mut count: c_uint = t.len;
    const u8		*tx = t.tx_buf;
    u8			*rx = t.rx_buf;
    bitbang = spi_controller_get_devdata(spi.controller);
    while (likely(count > 0)) {
    let mut word: u8 = 0;
    if (tx)
    word = *tx++;
    else
    word = spi.mode & SPI_MOSI_IDLE_HIGH ? 0xFF : 0;
    word = txrx_word(spi, ns, word, bits, flags);
    if (rx)
// rx++ = word;
    count -= 1;
    }
    if (bitbang.set_mosi_idle)
    bitbang.set_mosi_idle(spi);
    return t.len - count;
    }
    static unsigned int bitbang_txrx_16(struct spi_device *spi,
    spi_bb_txrx_word_fn txrx_word,
    unsigned int ns,
    struct spi_transfer	*t,
    unsigned int flags)
    {
    struct spi_bitbang	*bitbang;
    let mut bits: c_uint = t.bits_per_word;
    let mut count: c_uint = t.len;
    const u16		*tx = t.tx_buf;
    u16			*rx = t.rx_buf;
    bitbang = spi_controller_get_devdata(spi.controller);
    while (likely(count > 1)) {
    let mut word: u16 = 0;
    if (tx)
    word = *tx++;
    else
    word = spi.mode & SPI_MOSI_IDLE_HIGH ? 0xFFFF : 0;
    word = txrx_word(spi, ns, word, bits, flags);
    if (rx)
// rx++ = word;
    count -= 2;
    }
    if (bitbang.set_mosi_idle)
    bitbang.set_mosi_idle(spi);
    return t.len - count;
    }
    static unsigned int bitbang_txrx_32(struct spi_device *spi,
    spi_bb_txrx_word_fn txrx_word,
    unsigned int ns,
    struct spi_transfer	*t,
    unsigned int flags)
    {
    struct spi_bitbang	*bitbang;
    let mut bits: c_uint = t.bits_per_word;
    let mut count: c_uint = t.len;
    const u32		*tx = t.tx_buf;
    u32			*rx = t.rx_buf;
    bitbang = spi_controller_get_devdata(spi.controller);
    while (likely(count > 3)) {
    let mut word: u32 = 0;
    if (tx)
    word = *tx++;
    else
    word = spi.mode & SPI_MOSI_IDLE_HIGH ? 0xFFFFFFFF : 0;
    word = txrx_word(spi, ns, word, bits, flags);
    if (rx)
// rx++ = word;
    count -= 4;
    }
    if (bitbang.set_mosi_idle)
    bitbang.set_mosi_idle(spi);
    return t.len - count;
    }
#[no_mangle]
pub unsafe extern "C" fn spi_bitbang_setup_transfer(spi: *mut spi_device, t: *mut spi_transfer) -> c_int {
    int spi_bitbang_setup_transfer(struct spi_device *spi, struct spi_transfer *t)
    {
    struct spi_bitbang_cs	*cs = spi.controller_state;
    u8			bits_per_word;
    u32			hz;
    if (t) {
    bits_per_word = t.bits_per_word;
    hz = t.speed_hz;
    } else {
    bits_per_word = 0;
    hz = 0;
    }
// spi_transfer level calls that work per-word
    if (!bits_per_word)
    bits_per_word = spi.bits_per_word;
    if (bits_per_word <= 8)
    cs.txrx_bufs = bitbang_txrx_8;
#[no_mangle]
pub unsafe extern "C" fn if(16: bits_per_word <=) -> else {
    else if (bits_per_word <= 16)
    cs.txrx_bufs = bitbang_txrx_16;
#[no_mangle]
pub unsafe extern "C" fn if(32: bits_per_word <=) -> else {
    else if (bits_per_word <= 32)
    cs.txrx_bufs = bitbang_txrx_32;
    else
    return -EINVAL;
// nsecs = (clock period)/2
    if (!hz)
    hz = spi.max_speed_hz;
    if (hz) {
    cs.nsecs = (NSEC_PER_SEC / 2) / hz;
    if (cs.nsecs > (MAX_UDELAY_MS * NSEC_PER_MSEC))
    return -EINVAL;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(spi_bitbang_setup_transfer);
//
// spi_bitbang_setup - default setup for per-word I/O loops
//
#[no_mangle]
pub unsafe extern "C" fn spi_bitbang_setup(spi: *mut spi_device) -> c_int {
    int spi_bitbang_setup(struct spi_device *spi)
    {
    struct spi_bitbang_cs	*cs = spi.controller_state;
    struct spi_bitbang	*bitbang;
    let mut initial_setup: bool = false;
    int			retval;
    bitbang = spi_controller_get_devdata(spi.controller);
    if (!cs) {
    cs = kzalloc_obj(*cs);
    if (!cs)
    return -ENOMEM;
    spi.controller_state = cs;
    initial_setup = true;
    }
// per-word shift register access, in hardware or bitbanging
    cs.txrx_word = bitbang.txrx_word[spi.mode & (SPI_CPOL|SPI_CPHA)];
    if (!cs.txrx_word) {
    retval = -EINVAL;
    goto err_free;
    }
    if (bitbang.setup_transfer) {
    retval = bitbang.setup_transfer(spi, core::ptr::null_mut());
    if (retval < 0)
    goto err_free;
    }
    if (bitbang.set_mosi_idle)
    bitbang.set_mosi_idle(spi);
    dev_dbg(&spi.dev, "%s, %u nsec/bit\n", __func__, 2 * cs.nsecs);
    return 0;
    err_free:
    if (initial_setup)
    kfree(cs);
    return retval;
    }
    EXPORT_SYMBOL_GPL(spi_bitbang_setup);
//
// spi_bitbang_cleanup - default cleanup for per-word I/O loops
//
#[no_mangle]
pub unsafe extern "C" fn spi_bitbang_cleanup(spi: *mut spi_device) {
    void spi_bitbang_cleanup(struct spi_device *spi)
    {
    kfree(spi.controller_state);
    }
    EXPORT_SYMBOL_GPL(spi_bitbang_cleanup);
#[no_mangle]
unsafe extern "C" fn spi_bitbang_bufs(spi: *mut spi_device, t: *mut spi_transfer) -> c_int {
    static int spi_bitbang_bufs(struct spi_device *spi, struct spi_transfer *t)
    {
    struct spi_bitbang_cs	*cs = spi.controller_state;
    let mut nsecs: c_uint = cs.nsecs;
    struct spi_bitbang	*bitbang;
    bitbang = spi_controller_get_devdata(spi.controller);
    if (bitbang.set_line_direction) {
    int err;
    err = bitbang.set_line_direction(spi, !!(t.tx_buf));
    if (err < 0)
    return err;
    }
    if (spi.mode & SPI_3WIRE) {
    unsigned int flags;
    flags = t.tx_buf ? SPI_CONTROLLER_NO_RX : SPI_CONTROLLER_NO_TX;
    return cs.txrx_bufs(spi, cs.txrx_word, nsecs, t, flags);
    }
    return cs.txrx_bufs(spi, cs.txrx_word, nsecs, t, 0);
    }
// ----------------------------------------------------------------------
//
// SECOND PART ... simple transfer queue runner.
//
// This costs a task context per controller, running the queue by
// performing each transfer in sequence.  Smarter hardware can queue
// several DMA transfers at once, and process several controller queues
// in parallel; this driver doesn't match such hardware very well.
//
// Drivers can provide word-at-a-time i/o primitives, or provide
// transfer-at-a-time ones to leverage dma or fifo hardware.
//
#[no_mangle]
unsafe extern "C" fn spi_bitbang_prepare_hardware(spi: *mut spi_controller) -> c_int {
    static int spi_bitbang_prepare_hardware(struct spi_controller *spi)
    {
    struct spi_bitbang	*bitbang;
    bitbang = spi_controller_get_devdata(spi);
    mutex_lock(&bitbang.lock);
    bitbang.busy = 1;
    mutex_unlock(&bitbang.lock);
    return 0;
    }
    static int spi_bitbang_transfer_one(struct spi_controller *ctlr,
    struct spi_device *spi,
    struct spi_transfer *transfer)
    {
    struct spi_bitbang *bitbang = spi_controller_get_devdata(ctlr);
    let mut status: c_int = 0;
    if (bitbang.setup_transfer) {
    status = bitbang.setup_transfer(spi, transfer);
    if (status < 0)
    goto out;
    }
    if (transfer.len)
    status = bitbang.txrx_bufs(spi, transfer);
    if (status == transfer.len)
    status = 0;
#[no_mangle]
pub unsafe extern "C" fn if(0: status >=) -> else {
    else if (status >= 0)
    status = -EREMOTEIO;
    out:
    spi_finalize_current_transfer(ctlr);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn spi_bitbang_unprepare_hardware(spi: *mut spi_controller) -> c_int {
    static int spi_bitbang_unprepare_hardware(struct spi_controller *spi)
    {
    struct spi_bitbang	*bitbang;
    bitbang = spi_controller_get_devdata(spi);
    mutex_lock(&bitbang.lock);
    bitbang.busy = 0;
    mutex_unlock(&bitbang.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spi_bitbang_set_cs(spi: *mut spi_device, enable: bool) {
    static void spi_bitbang_set_cs(struct spi_device *spi, bool enable)
    {
    struct spi_bitbang *bitbang = spi_controller_get_devdata(spi.controller);
// SPI core provides CS high / low, but bitbang driver
// expects CS active
// spi device driver takes care of handling SPI_CS_HIGH
//
    enable = (!!(spi.mode & SPI_CS_HIGH) == enable);
    ndelay(SPI_BITBANG_CS_DELAY);
    bitbang.chipselect(spi, enable ? BITBANG_CS_ACTIVE :
    BITBANG_CS_INACTIVE);
    ndelay(SPI_BITBANG_CS_DELAY);
    }
// ----------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn spi_bitbang_init(bitbang: *mut spi_bitbang) -> c_int {
    int spi_bitbang_init(struct spi_bitbang *bitbang)
    {
    struct spi_controller *ctlr = bitbang.ctlr;
    bool custom_cs;
    if (!ctlr)
    return -EINVAL;
//
// We only need the chipselect callback if we are actually using it.
// If we just use GPIO descriptors, it is surplus. If the
// SPI_CONTROLLER_GPIO_SS flag is set, we always need to call the
// driver-specific chipselect routine.
//
    custom_cs = (!ctlr.use_gpio_descriptors ||
    (ctlr.flags & SPI_CONTROLLER_GPIO_SS));
    if (custom_cs && !bitbang.chipselect)
    return -EINVAL;
    mutex_init(&bitbang.lock);
    if (!ctlr.mode_bits)
    ctlr.mode_bits = SPI_CPOL | SPI_CPHA | bitbang.flags;
    if (ctlr.transfer || ctlr.transfer_one_message)
    return -EINVAL;
    ctlr.prepare_transfer_hardware = spi_bitbang_prepare_hardware;
    ctlr.unprepare_transfer_hardware = spi_bitbang_unprepare_hardware;
    ctlr.transfer_one = spi_bitbang_transfer_one;
//
// When using GPIO descriptors, the ->set_cs() callback doesn't even
// get called unless SPI_CONTROLLER_GPIO_SS is set.
//
    if (custom_cs)
    ctlr.set_cs = spi_bitbang_set_cs;
    if (!bitbang.txrx_bufs) {
    bitbang.use_dma = 0;
    bitbang.txrx_bufs = spi_bitbang_bufs;
    if (!ctlr.setup) {
    if (!bitbang.setup_transfer)
    bitbang.setup_transfer =
    spi_bitbang_setup_transfer;
    ctlr.setup = spi_bitbang_setup;
    ctlr.cleanup = spi_bitbang_cleanup;
    }
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(spi_bitbang_init);
//
// spi_bitbang_start - start up a polled/bitbanging SPI host controller driver
// @bitbang: driver handle
//
// Caller should have zero-initialized all parts of the structure, and then
// provided callbacks for chip selection and I/O loops.  If the host controller has
// a transfer method, its final step should call spi_bitbang_transfer(); or,
// that's the default if the transfer routine is not initialized.  It should
// also set up the bus number and number of chipselects.
//
// For i/o loops, provide callbacks either per-word (for bitbanging, or for
// hardware that basically exposes a shift register) or per-spi_transfer
// (which takes better advantage of hardware like fifos or DMA engines).
//
// Drivers using per-word I/O loops should use (or call) spi_bitbang_setup(),
// spi_bitbang_cleanup() and spi_bitbang_setup_transfer() to handle those SPI
// host controller methods.  Those methods are the defaults if the bitbang->txrx_bufs
// routine isn't initialized.
//
// This routine registers the spi_controller, which will process requests in a
// dedicated task, keeping IRQs unblocked most of the time.  To stop
// processing those requests, call spi_bitbang_stop().
//
#[no_mangle]
pub unsafe extern "C" fn spi_bitbang_start(bitbang: *mut spi_bitbang) -> c_int {
    int spi_bitbang_start(struct spi_bitbang *bitbang)
    {
    struct spi_controller *ctlr = bitbang.ctlr;
    int ret;
    ret = spi_bitbang_init(bitbang);
    if (ret)
    return ret;
// driver may get busy before register() returns, especially
// if someone registered boardinfo for devices
//
    return spi_register_controller(ctlr);
    }
    EXPORT_SYMBOL_GPL(spi_bitbang_start);
//
// spi_bitbang_stop - stops the task providing spi communication
//
#[no_mangle]
pub unsafe extern "C" fn spi_bitbang_stop(bitbang: *mut spi_bitbang) {
    void spi_bitbang_stop(struct spi_bitbang *bitbang)
    {
    spi_unregister_controller(bitbang.ctlr);
    }
    EXPORT_SYMBOL_GPL(spi_bitbang_stop);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Utilities for Bitbanging SPI host controllers");
