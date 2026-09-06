//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-mpc52xx.c
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
// MPC52xx SPI bus driver.
//
// Copyright (C) 2008 Secret Lab Technologies Ltd.
//
// This is the driver for the MPC5200's dedicated SPI controller.
//
// Note: this driver does not support the MPC5200 PSC in SPI mode.  For
// that driver see drivers/spi/mpc52xx_psc_spi.c
//

    MODULE_AUTHOR("Grant Likely <grant.likely@secretlab.ca>");
    MODULE_DESCRIPTION("MPC52xx SPI (non-PSC) Driver");
    MODULE_LICENSE("GPL");
// Register offsets
pub const SPI_CTRL1: c_uint = 0x00;

pub const SPI_CTRL2: c_uint = 0x01;
pub const SPI_BRR: c_uint = 0x04;
pub const SPI_STATUS: c_uint = 0x05;

pub const SPI_DATA: c_uint = 0x09;
pub const SPI_PORTDATA: c_uint = 0x0d;
pub const SPI_DATADIR: c_uint = 0x10;
// FSM state return values

// do.  If something interesting happens
// then an IRQ will be received

// not expected

// Driver internal data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc52xx_spi {
    pub host: *mut spi_controller,
    pub regs: *mut void __iomem,
    pub /: *mut *mut int irq0; / MODF irq,
    pub /: *mut *mut int irq1; / SPIF irq,
    pub ipb_freq: c_uint,
// Statistics; not used now, but will be reintroduced for debugfs
    pub msg_count: c_int,
    pub wcol_count: c_int,
    pub wcol_ticks: c_int,
    pub wcol_tx_timestamp: u32,
    pub modf_count: c_int,
    pub byte_count: c_int,
    pub /: *mut *mut list_head queue; / queue of pending messages,
    pub lock: spinlock_t,
    pub work: work_struct,
// Details of current transfer (length, and buffer pointers)
    pub /: *mut *mut *mut spi_message message; / current message,
    pub /: *mut *mut *mut spi_transfer transfer; / current transfer,
    pub data): *mut *mut *mut int (state)(int irq, struct mpc52xx_spi ms, u8 status, u8,
    pub len: c_int,
    pub timestamp: c_int,
    pub rx_buf: *mut u8,
    pub tx_buf: *const u8,
    pub cs_change: c_int,
    pub gpio_cs_count: c_int,
    pub gpio_cs: *mut gpio_desc,
}

//
// CS control function
//
#[no_mangle]
unsafe extern "C" fn mpc52xx_spi_chipsel(ms: *mut mpc52xx_spi, value: c_int) {
    static void mpc52xx_spi_chipsel(struct mpc52xx_spi *ms, int value)
    {
    int cs;
    if (ms.gpio_cs_count > 0) {
    cs = spi_get_chipselect(ms.message.spi, 0);
    gpiod_set_value(ms.gpio_cs[cs], value);
    } else {
    out_8(ms.regs + SPI_PORTDATA, value ? 0 : 0x08);
    }
    }
//
// Start a new transfer.  This is called both by the idle state
// for the first transfer in a message, and by the wait state when the
// previous transfer in a message is complete.
//
#[no_mangle]
unsafe extern "C" fn mpc52xx_spi_start_transfer(ms: *mut mpc52xx_spi) {
    static void mpc52xx_spi_start_transfer(struct mpc52xx_spi *ms)
    {
    ms.rx_buf = ms.transfer.rx_buf;
    ms.tx_buf = ms.transfer.tx_buf;
    ms.len = ms.transfer.len;
// Activate the chip select
    if (ms.cs_change)
    mpc52xx_spi_chipsel(ms, 1);
    ms.cs_change = ms.transfer.cs_change;
// Write out the first byte
    ms.wcol_tx_timestamp = mftb();
    if (ms.tx_buf)
    out_8(ms.regs + SPI_DATA, *ms.tx_buf++);
    else
    out_8(ms.regs + SPI_DATA, 0);
    }
// Forward declaration of state handlers
    static int mpc52xx_spi_fsmstate_transfer(int irq, struct mpc52xx_spi *ms,
    u8 status, u8 data);
    static int mpc52xx_spi_fsmstate_wait(int irq, struct mpc52xx_spi *ms,
    u8 status, u8 data);
//
// IDLE state
//
// No transfers are in progress; if another transfer is pending then retrieve
// it and kick it off.  Otherwise, stop processing the state machine
//
    static int
    mpc52xx_spi_fsmstate_idle(int irq, struct mpc52xx_spi *ms, u8 status, u8 data)
    {
    struct spi_device *spi;
    int spr, sppr;
    u8 ctrl1;
    if (status && irq)
    dev_err(&ms.host.dev, "spurious irq, status=0x%.2x\n",
    status);
// Check if there is another transfer waiting.
    if (list_empty(&ms.queue))
    return FSM_STOP;
// get the head of the queue
    ms.message = list_first_entry(&ms.queue, struct spi_message, queue);
    list_del_init(&ms.message.queue);
// Setup the controller parameters
    ctrl1 = SPI_CTRL1_SPIE | SPI_CTRL1_SPE | SPI_CTRL1_MSTR;
    spi = ms.message.spi;
    if (spi.mode & SPI_CPHA)
    ctrl1 |= SPI_CTRL1_CPHA;
    if (spi.mode & SPI_CPOL)
    ctrl1 |= SPI_CTRL1_CPOL;
    if (spi.mode & SPI_LSB_FIRST)
    ctrl1 |= SPI_CTRL1_LSBFE;
    out_8(ms.regs + SPI_CTRL1, ctrl1);
// Setup the controller speed
// minimum divider is '2'.  Also, add '1' to force rounding the
// divider up.
    sppr = ((ms.ipb_freq / ms.message.spi.max_speed_hz) + 1) >> 1;
    spr = 0;
    if (sppr < 1)
    sppr = 1;
    while (((sppr - 1) & ~0x7) != 0) {
    sppr = (sppr + 1) >> 1; /* add '1' to force rounding up */
    spr++;
    }
    sppr--;		/* sppr quantity in register is offset by 1 */
    if (spr > 7) {
// Don't overrun limits of SPI baudrate register
    spr = 7;
    sppr = 7;
    }
    out_8(ms.regs + SPI_BRR, sppr << 4 | spr); /* Set speed */
    ms.cs_change = 1;
    ms.transfer = container_of(ms.message.transfers.next,
    struct spi_transfer, transfer_list);
    mpc52xx_spi_start_transfer(ms);
    ms.state = mpc52xx_spi_fsmstate_transfer;
    return FSM_CONTINUE;
    }
//
// TRANSFER state
//
// In the middle of a transfer.  If the SPI core has completed processing
// a byte, then read out the received data and write out the next byte
// (unless this transfer is finished; in which case go on to the wait
// state)
//
    static int mpc52xx_spi_fsmstate_transfer(int irq, struct mpc52xx_spi *ms,
    u8 status, u8 data)
    {
    if (!status)
    return ms.irq0 ? FSM_STOP : FSM_POLL;
    if (status & SPI_STATUS_WCOL) {
// The SPI controller is stoopid.  At slower speeds, it may
// raise the SPIF flag before the state machine is actually
// finished, which causes a collision (internal to the state
// machine only).  The manual recommends inserting a delay
// between receiving the interrupt and sending the next byte,
// but it can also be worked around simply by retrying the
// transfer which is what we do here.
    ms.wcol_count++;
    ms.wcol_ticks += mftb() - ms.wcol_tx_timestamp;
    ms.wcol_tx_timestamp = mftb();
    data = 0;
    if (ms.tx_buf)
    data = *(ms.tx_buf - 1);
    out_8(ms.regs + SPI_DATA, data); /* try again */
    return FSM_CONTINUE;
    } else if (status & SPI_STATUS_MODF) {
    ms.modf_count++;
    dev_err(&ms.host.dev, "mode fault\n");
    mpc52xx_spi_chipsel(ms, 0);
    ms.message.status = -EIO;
    if (ms.message.complete)
    ms.message.complete(ms.message.context);
    ms.state = mpc52xx_spi_fsmstate_idle;
    return FSM_CONTINUE;
    }
// Read data out of the spi device
    ms.byte_count++;
    if (ms.rx_buf)
// ms->rx_buf++ = data;
// Is the transfer complete?
    ms.len--;
    if (ms.len == 0) {
    ms.timestamp = mftb();
    if (ms.transfer.delay.unit == SPI_DELAY_UNIT_USECS)
    ms.timestamp += ms.transfer.delay.value *
    tb_ticks_per_usec;
    ms.state = mpc52xx_spi_fsmstate_wait;
    return FSM_CONTINUE;
    }
// Write out the next byte
    ms.wcol_tx_timestamp = mftb();
    if (ms.tx_buf)
    out_8(ms.regs + SPI_DATA, *ms.tx_buf++);
    else
    out_8(ms.regs + SPI_DATA, 0);
    return FSM_CONTINUE;
    }
//
// WAIT state
//
// A transfer has completed; need to wait for the delay period to complete
// before starting the next transfer
//
    static int
    mpc52xx_spi_fsmstate_wait(int irq, struct mpc52xx_spi *ms, u8 status, u8 data)
    {
    if (status && irq)
    dev_err(&ms.host.dev, "spurious irq, status=0x%.2x\n",
    status);
    if (((int)mftb()) - ms.timestamp < 0)
    return FSM_POLL;
    ms.message.actual_length += ms.transfer.len;
// Check if there is another transfer in this message.  If there
// aren't then deactivate CS, notify sender, and drop back to idle
// to start the next message.
    if (ms.transfer.transfer_list.next == &ms.message.transfers) {
    ms.msg_count++;
    mpc52xx_spi_chipsel(ms, 0);
    ms.message.status = 0;
    if (ms.message.complete)
    ms.message.complete(ms.message.context);
    ms.state = mpc52xx_spi_fsmstate_idle;
    return FSM_CONTINUE;
    }
// There is another transfer; kick it off
    if (ms.cs_change)
    mpc52xx_spi_chipsel(ms, 0);
    ms.transfer = container_of(ms.transfer.transfer_list.next,
    struct spi_transfer, transfer_list);
    mpc52xx_spi_start_transfer(ms);
    ms.state = mpc52xx_spi_fsmstate_transfer;
    return FSM_CONTINUE;
    }
//
// mpc52xx_spi_fsm_process - Finite State Machine iteration function
// @irq: irq number that triggered the FSM or 0 for polling
// @ms: pointer to mpc52xx_spi driver data
//
#[no_mangle]
unsafe extern "C" fn mpc52xx_spi_fsm_process(irq: c_int, ms: *mut mpc52xx_spi) {
    static void mpc52xx_spi_fsm_process(int irq, struct mpc52xx_spi *ms)
    {
    let mut rc: c_int = FSM_CONTINUE;
    u8 status, data;
    while (rc == FSM_CONTINUE) {
// Interrupt cleared by read of STATUS followed by
// read of DATA registers
    status = in_8(ms.regs + SPI_STATUS);
    data = in_8(ms.regs + SPI_DATA);
    rc = ms.state(irq, ms, status, data);
    }
    if (rc == FSM_POLL)
    schedule_work(&ms.work);
    }
//
// mpc52xx_spi_irq - IRQ handler
//
#[no_mangle]
unsafe extern "C" fn mpc52xx_spi_irq(irq: c_int, _ms: *mut c_void) -> irqreturn_t {
    static irqreturn_t mpc52xx_spi_irq(int irq, void *_ms)
    {
    struct mpc52xx_spi *ms = _ms;
    spin_lock(&ms.lock);
    mpc52xx_spi_fsm_process(irq, ms);
    spin_unlock(&ms.lock);
    return IRQ_HANDLED;
    }
//
// mpc52xx_spi_wq - Workqueue function for polling the state machine
//
#[no_mangle]
unsafe extern "C" fn mpc52xx_spi_wq(work: *mut work_struct) {
    static void mpc52xx_spi_wq(struct work_struct *work)
    {
    struct mpc52xx_spi *ms = container_of(work, struct mpc52xx_spi, work);
    unsigned long flags;
    spin_lock_irqsave(&ms.lock, flags);
    mpc52xx_spi_fsm_process(0, ms);
    spin_unlock_irqrestore(&ms.lock, flags);
    }
//
// spi_controller ops
//
#[no_mangle]
unsafe extern "C" fn mpc52xx_spi_transfer(spi: *mut spi_device, m: *mut spi_message) -> c_int {
    static int mpc52xx_spi_transfer(struct spi_device *spi, struct spi_message *m)
    {
    struct mpc52xx_spi *ms = spi_controller_get_devdata(spi.controller);
    unsigned long flags;
    m.actual_length = 0;
    m.status = -EINPROGRESS;
    spin_lock_irqsave(&ms.lock, flags);
    list_add_tail(&m.queue, &ms.queue);
    spin_unlock_irqrestore(&ms.lock, flags);
    schedule_work(&ms.work);
    return 0;
    }
//
// OF Platform Bus Binding
//
#[no_mangle]
unsafe extern "C" fn mpc52xx_spi_probe(op: *mut platform_device) -> c_int {
    static int mpc52xx_spi_probe(struct platform_device *op)
    {
    struct spi_controller *host;
    struct mpc52xx_spi *ms;
    struct gpio_desc *gpio_cs;
    void __iomem *regs;
    u8 ctrl1;
    int rc, i = 0;
    int irq0;
    int irq1;
    irq0 = platform_get_irq_optional(op, 0);
    if (irq0 == -EPROBE_DEFER)
    return irq0;
    irq0 = max(irq0, 0);
    irq1 = platform_get_irq_optional(op, 1);
    if (irq1 == -EPROBE_DEFER)
    return irq1;
    irq1 = max(irq1, 0);
// MMIO registers
    dev_dbg(&op.dev, "probing mpc5200 SPI device\n");
    regs = of_iomap(op.dev.of_node, 0);
    if (!regs)
    return -ENODEV;
// initialize the device
    ctrl1 = SPI_CTRL1_SPIE | SPI_CTRL1_SPE | SPI_CTRL1_MSTR;
    out_8(regs + SPI_CTRL1, ctrl1);
    out_8(regs + SPI_CTRL2, 0x0);
    out_8(regs + SPI_DATADIR, 0xe);	/* Set output pins */
    out_8(regs + SPI_PORTDATA, 0x8);	/* Deassert /SS signal */
// Clear the status register and re-read it to check for a MODF
// failure.  This driver cannot currently handle multiple hosts
// on the SPI bus.  This fault will also occur if the SPI signals
// are not connected to any pins (port_config setting)
    in_8(regs + SPI_STATUS);
    out_8(regs + SPI_CTRL1, ctrl1);
    in_8(regs + SPI_DATA);
    if (in_8(regs + SPI_STATUS) & SPI_STATUS_MODF) {
    dev_err(&op.dev, "mode fault; is port_config correct?\n");
    rc = -EIO;
    goto err_init;
    }
    dev_dbg(&op.dev, "allocating spi_controller struct\n");
    host = spi_alloc_host(&op.dev, sizeof(*ms));
    if (!host) {
    rc = -ENOMEM;
    goto err_alloc;
    }
    host.transfer = mpc52xx_spi_transfer;
    host.mode_bits = SPI_CPOL | SPI_CPHA | SPI_LSB_FIRST;
    host.bits_per_word_mask = SPI_BPW_MASK(8);
    platform_set_drvdata(op, host);
    ms = spi_controller_get_devdata(host);
    ms.host = host;
    ms.regs = regs;
    ms.irq0 = irq0;
    ms.irq1 = irq1;
    ms.state = mpc52xx_spi_fsmstate_idle;
    ms.ipb_freq = mpc5xxx_get_bus_frequency(&op.dev);
    ms.gpio_cs_count = gpiod_count(&op.dev, core::ptr::null_mut());
    if (ms.gpio_cs_count > 0) {
    host.num_chipselect = ms.gpio_cs_count;
    ms.gpio_cs = kmalloc_objs(*ms.gpio_cs, ms.gpio_cs_count);
    if (!ms.gpio_cs) {
    rc = -ENOMEM;
    goto err_alloc_gpio;
    }
    for (i = 0; i < ms.gpio_cs_count; i++) {
    gpio_cs = gpiod_get_index(&op.dev,
    core::ptr::null_mut(), i, GPIOD_OUT_LOW);
    rc = PTR_ERR_OR_ZERO(gpio_cs);
    if (rc) {
    dev_err(&op.dev,
    "failed to get spi cs gpio #%d: %d\n",
    i, rc);
    goto err_gpio;
    }
    ms.gpio_cs[i] = gpio_cs;
    }
    }
    spin_lock_init(&ms.lock);
    INIT_LIST_HEAD(&ms.queue);
    INIT_WORK(&ms.work, mpc52xx_spi_wq);
// Decide if interrupts can be used
    if (ms.irq0 && ms.irq1) {
    rc = request_irq(ms.irq0, mpc52xx_spi_irq, 0,
    "mpc5200-spi-modf", ms);
    if (rc == 0) {
    rc = request_irq(ms.irq1, mpc52xx_spi_irq, 0,
    "mpc5200-spi-spif", ms);
    if (rc)
    free_irq(ms.irq0, ms);
    }
    if (rc)
    ms.irq0 = ms.irq1 = 0;
    } else {
// operate in polled mode
    ms.irq0 = ms.irq1 = 0;
    }
    if (!ms.irq0)
    dev_info(&op.dev, "using polled mode\n");
    dev_dbg(&op.dev, "registering spi_controller struct\n");
    rc = spi_register_controller(host);
    if (rc)
    goto err_register;
    dev_info(&ms.host.dev, "registered MPC5200 SPI bus\n");
    return rc;
    err_register:
    dev_err(&ms.host.dev, "initialization failed\n");
    if (ms.irq0) {
    free_irq(ms.irq0, ms);
    free_irq(ms.irq1, ms);
    }
    cancel_work_sync(&ms.work);
    err_gpio:
    while (i-- > 0)
    gpiod_put(ms.gpio_cs[i]);
    kfree(ms.gpio_cs);
    err_alloc_gpio:
    spi_controller_put(host);
    err_alloc:
    err_init:
    iounmap(regs);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn mpc52xx_spi_remove(op: *mut platform_device) {
    static void mpc52xx_spi_remove(struct platform_device *op)
    {
    struct spi_controller *host = platform_get_drvdata(op);
    struct mpc52xx_spi *ms = spi_controller_get_devdata(host);
    int i;
    spi_unregister_controller(host);
    if (ms.irq0) {
    free_irq(ms.irq0, ms);
    free_irq(ms.irq1, ms);
    }
    cancel_work_sync(&ms.work);
    for (i = 0; i < ms.gpio_cs_count; i++)
    gpiod_put(ms.gpio_cs[i]);
    kfree(ms.gpio_cs);
    iounmap(ms.regs);
    spi_controller_put(host);
    }
    static const struct of_device_id mpc52xx_spi_match[] = {
    { .compatible = "fsl,mpc5200-spi", },
    {}
    };
    MODULE_DEVICE_TABLE(of, mpc52xx_spi_match);
    static struct platform_driver mpc52xx_spi_of_driver = {
    .driver = {
    .name = "mpc52xx-spi",
    .of_match_table = mpc52xx_spi_match,
    },
    .probe = mpc52xx_spi_probe,
    .remove = mpc52xx_spi_remove,
    };
    module_platform_driver(mpc52xx_spi_of_driver);
