//! Automatically rewritten from C to Rust
//! Source: drivers/input/serio/ps2-gpio.c
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
// GPIO based serio bus driver for bit banging the PS/2 protocol
//
// Author: Danilo Krummrich <danilokrummrich@dk-develop.de>
//

pub const PS2_MODE_RX: c_int = 0;
pub const PS2_MODE_TX: c_int = 1;
pub const PS2_START_BIT: c_int = 0;
pub const PS2_DATA_BIT0: c_int = 1;
pub const PS2_DATA_BIT1: c_int = 2;
pub const PS2_DATA_BIT2: c_int = 3;
pub const PS2_DATA_BIT3: c_int = 4;
pub const PS2_DATA_BIT4: c_int = 5;
pub const PS2_DATA_BIT5: c_int = 6;
pub const PS2_DATA_BIT6: c_int = 7;
pub const PS2_DATA_BIT7: c_int = 8;
pub const PS2_PARITY_BIT: c_int = 9;
pub const PS2_STOP_BIT: c_int = 10;
pub const PS2_ACK_BIT: c_int = 11;
pub const PS2_DEV_RET_ACK: c_uint = 0xfa;
pub const PS2_DEV_RET_NACK: c_uint = 0xfe;
pub const PS2_CMD_RESEND: c_uint = 0xfe;
//
// The PS2 protocol specifies a clock frequency between 10kHz and 16.7kHz,
// therefore the maximal interrupt interval should be 100us and the minimum
// interrupt interval should be ~60us. Let's allow +/- 20us for frequency
// deviations and interrupt latency.
//
// The data line must be sampled after ~30us to 50us after the falling edge,
// since the device updates the data line at the rising edge.
//
// ___            ______            ______            ______            ___
// \          /      \          /      \          /      \
// \        /        \        /        \        /        \
// \______/          \______/          \______/          \______
//
// |-----------------|                 |--------|
// 60us/100us                      30us/50us
//
pub const PS2_CLK_FREQ_MIN_HZ: c_int = 10000;
pub const PS2_CLK_FREQ_MAX_HZ: c_int = 16700;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps2_gpio_data {
    pub dev: *mut device,
    pub serio: *mut serio,
    pub mode: c_uchar,
    pub gpio_clk: *mut gpio_desc,
    pub gpio_data: *mut gpio_desc,
    pub write_enable: bool,
    pub irq: c_int,
    pub t_irq_now: ktime_t,
    pub t_irq_last: ktime_t,
    struct {
    pub cnt: c_uchar,
    pub byte: c_uchar,
    pub rx: },
    struct {
    pub cnt: c_uchar,
    pub byte: c_uchar,
    pub t_xfer_start: ktime_t,
    pub t_xfer_end: ktime_t,
    pub complete: completion,
    pub mutex: mutex,
    pub work: delayed_work,
    pub tx: },
}

#[no_mangle]
unsafe extern "C" fn ps2_gpio_open(serio: *mut serio) -> c_int {
    static int ps2_gpio_open(struct serio *serio)
    {
    struct ps2_gpio_data *drvdata = serio.port_data;
    drvdata.t_irq_last = 0;
    drvdata.tx.t_xfer_end = 0;
    enable_irq(drvdata.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ps2_gpio_close(serio: *mut serio) {
    static void ps2_gpio_close(struct serio *serio)
    {
    struct ps2_gpio_data *drvdata = serio.port_data;
    flush_delayed_work(&drvdata.tx.work);
    disable_irq(drvdata.irq);
    }
#[no_mangle]
unsafe extern "C" fn __ps2_gpio_write(serio: *mut serio, val: c_uchar) -> c_int {
    static int __ps2_gpio_write(struct serio *serio, unsigned char val)
    {
    struct ps2_gpio_data *drvdata = serio.port_data;
    disable_irq_nosync(drvdata.irq);
    gpiod_direction_output(drvdata.gpio_clk, 0);
    drvdata.mode = PS2_MODE_TX;
    drvdata.tx.byte = val;
    schedule_delayed_work(&drvdata.tx.work, usecs_to_jiffies(200));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ps2_gpio_write(serio: *mut serio, val: c_uchar) -> c_int {
    static int ps2_gpio_write(struct serio *serio, unsigned char val)
    {
    struct ps2_gpio_data *drvdata = serio.port_data;
    let mut ret: c_int = 0;
    if (in_task()) {
    guard(mutex)(&drvdata.tx.mutex);
    __ps2_gpio_write(serio, val);
    if (!wait_for_completion_timeout(&drvdata.tx.complete,
    msecs_to_jiffies(10000)))
    ret = SERIO_TIMEOUT;
    } else {
    __ps2_gpio_write(serio, val);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ps2_gpio_tx_work_fn(work: *mut work_struct) {
    static void ps2_gpio_tx_work_fn(struct work_struct *work)
    {
    struct delayed_work *dwork = to_delayed_work(work);
    struct ps2_gpio_data *drvdata = container_of(dwork,
    struct ps2_gpio_data,
    tx.work);
    drvdata.tx.t_xfer_start = ktime_get();
    enable_irq(drvdata.irq);
    gpiod_direction_output(drvdata.gpio_data, 0);
    gpiod_direction_input(drvdata.gpio_clk);
    }
#[no_mangle]
unsafe extern "C" fn ps2_gpio_irq_rx(drvdata: *mut ps2_gpio_data) -> irqreturn_t {
    static irqreturn_t ps2_gpio_irq_rx(struct ps2_gpio_data *drvdata)
    {
    unsigned char byte, cnt;
    int data;
    let mut rxflags: c_int = 0;
    s64 us_delta;
    byte = drvdata.rx.byte;
    cnt = drvdata.rx.cnt;
    drvdata.t_irq_now = ktime_get();
//
// We need to consider spurious interrupts happening right after
// a TX xfer finished.
//
    us_delta = ktime_us_delta(drvdata.t_irq_now, drvdata.tx.t_xfer_end);
    if (unlikely(us_delta < PS2_IRQ_MIN_INTERVAL_US))
    goto end;
    us_delta = ktime_us_delta(drvdata.t_irq_now, drvdata.t_irq_last);
    if (us_delta > PS2_IRQ_MAX_INTERVAL_US && cnt) {
    dev_err(drvdata.dev,
    "RX: timeout, probably we missed an interrupt\n");
    goto err;
    } else if (unlikely(us_delta < PS2_IRQ_MIN_INTERVAL_US)) {
// Ignore spurious IRQs.
    goto end;
    }
    drvdata.t_irq_last = drvdata.t_irq_now;
    data = gpiod_get_value(drvdata.gpio_data);
    if (unlikely(data < 0)) {
    dev_err(drvdata.dev, "RX: failed to get data gpio val: %d\n",
    data);
    goto err;
    }
    switch (cnt) {
    case PS2_START_BIT:
// start bit should be low
    if (unlikely(data)) {
    dev_err(drvdata.dev, "RX: start bit should be low\n");
    goto err;
    }
    break;
    case PS2_DATA_BIT0:
    case PS2_DATA_BIT1:
    case PS2_DATA_BIT2:
    case PS2_DATA_BIT3:
    case PS2_DATA_BIT4:
    case PS2_DATA_BIT5:
    case PS2_DATA_BIT6:
    case PS2_DATA_BIT7:
// processing data bits
    if (data)
    byte |= (data << (cnt - 1));
    break;
    case PS2_PARITY_BIT:
// check odd parity
    if (!((hweight8(byte) & 1) ^ data)) {
    rxflags |= SERIO_PARITY;
    dev_warn(drvdata.dev, "RX: parity error\n");
    if (!drvdata.write_enable)
    goto err;
    }
    break;
    case PS2_STOP_BIT:
// stop bit should be high
    if (unlikely(!data)) {
    dev_err(drvdata.dev, "RX: stop bit should be high\n");
    goto err;
    }
//
// Do not send spurious ACK's and NACK's when write fn is
// not provided.
//
    if (!drvdata.write_enable) {
    if (byte == PS2_DEV_RET_NACK)
    goto err;
#[no_mangle]
pub unsafe extern "C" fn if(PS2_DEV_RET_ACK: byte ==) -> else {
    else if (byte == PS2_DEV_RET_ACK)
    break;
    }
    serio_interrupt(drvdata.serio, byte, rxflags);
    dev_dbg(drvdata.dev, "RX: sending byte 0x%x\n", byte);
    cnt = byte = 0;
    goto end; /* success */
    default:
    dev_err(drvdata.dev, "RX: got out of sync with the device\n");
    goto err;
    }
    cnt++;
    goto end; /* success */
    err:
    cnt = byte = 0;
    __ps2_gpio_write(drvdata.serio, PS2_CMD_RESEND);
    end:
    drvdata.rx.cnt = cnt;
    drvdata.rx.byte = byte;
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ps2_gpio_irq_tx(drvdata: *mut ps2_gpio_data) -> irqreturn_t {
    static irqreturn_t ps2_gpio_irq_tx(struct ps2_gpio_data *drvdata)
    {
    unsigned char byte, cnt;
    int data;
    s64 us_delta;
    cnt = drvdata.tx.cnt;
    byte = drvdata.tx.byte;
    drvdata.t_irq_now = ktime_get();
//
// There might be pending IRQs since we disabled IRQs in
// __ps2_gpio_write().  We can expect at least one clock period until
// the device generates the first falling edge after releasing the
// clock line.
//
    us_delta = ktime_us_delta(drvdata.t_irq_now,
    drvdata.tx.t_xfer_start);
    if (unlikely(us_delta < PS2_CLK_MIN_INTERVAL_US))
    goto end;
    us_delta = ktime_us_delta(drvdata.t_irq_now, drvdata.t_irq_last);
    if (us_delta > PS2_IRQ_MAX_INTERVAL_US && cnt > 1) {
    dev_err(drvdata.dev,
    "TX: timeout, probably we missed an interrupt\n");
    goto err;
    } else if (unlikely(us_delta < PS2_IRQ_MIN_INTERVAL_US)) {
// Ignore spurious IRQs.
    goto end;
    }
    drvdata.t_irq_last = drvdata.t_irq_now;
    switch (cnt) {
    case PS2_START_BIT:
// should never happen
    dev_err(drvdata.dev,
    "TX: start bit should have been sent already\n");
    goto err;
    case PS2_DATA_BIT0:
    case PS2_DATA_BIT1:
    case PS2_DATA_BIT2:
    case PS2_DATA_BIT3:
    case PS2_DATA_BIT4:
    case PS2_DATA_BIT5:
    case PS2_DATA_BIT6:
    case PS2_DATA_BIT7:
    data = byte & BIT(cnt - 1);
    gpiod_set_value(drvdata.gpio_data, data);
    break;
    case PS2_PARITY_BIT:
// do odd parity
    data = !(hweight8(byte) & 1);
    gpiod_set_value(drvdata.gpio_data, data);
    break;
    case PS2_STOP_BIT:
// release data line to generate stop bit
    gpiod_direction_input(drvdata.gpio_data);
    break;
    case PS2_ACK_BIT:
    data = gpiod_get_value(drvdata.gpio_data);
    if (data) {
    dev_warn(drvdata.dev, "TX: received NACK, retry\n");
    goto err;
    }
    drvdata.tx.t_xfer_end = ktime_get();
    drvdata.mode = PS2_MODE_RX;
    complete(&drvdata.tx.complete);
    cnt = 1;
    goto end; /* success */
    default:
//
// Probably we missed the stop bit. Therefore we release data
// line and try again.
//
    gpiod_direction_input(drvdata.gpio_data);
    dev_err(drvdata.dev, "TX: got out of sync with the device\n");
    goto err;
    }
    cnt++;
    goto end; /* success */
    err:
    cnt = 1;
    gpiod_direction_input(drvdata.gpio_data);
    __ps2_gpio_write(drvdata.serio, drvdata.tx.byte);
    end:
    drvdata.tx.cnt = cnt;
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ps2_gpio_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ps2_gpio_irq(int irq, void *dev_id)
    {
    struct ps2_gpio_data *drvdata = dev_id;
    return drvdata.mode ? ps2_gpio_irq_tx(drvdata) :
    ps2_gpio_irq_rx(drvdata);
    }
    static int ps2_gpio_get_props(struct device *dev,
    struct ps2_gpio_data *drvdata)
    {
    enum gpiod_flags gflags;
// Enforce open drain, since this is required by the PS/2 bus.
    gflags = GPIOD_IN | GPIOD_FLAGS_BIT_OPEN_DRAIN;
    drvdata.gpio_data = devm_gpiod_get(dev, "data", gflags);
    if (IS_ERR(drvdata.gpio_data)) {
    dev_err(dev, "failed to request data gpio: %ld",
    PTR_ERR(drvdata.gpio_data));
    return PTR_ERR(drvdata.gpio_data);
    }
    drvdata.gpio_clk = devm_gpiod_get(dev, "clk", gflags);
    if (IS_ERR(drvdata.gpio_clk)) {
    dev_err(dev, "failed to request clock gpio: %ld",
    PTR_ERR(drvdata.gpio_clk));
    return PTR_ERR(drvdata.gpio_clk);
    }
    drvdata.write_enable = device_property_read_bool(dev,
    "write-enable");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ps2_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int ps2_gpio_probe(struct platform_device *pdev)
    {
    struct ps2_gpio_data *drvdata;
    struct serio *serio;
    struct device *dev = &pdev.dev;
    int error;
    drvdata = devm_kzalloc(dev, sizeof(*drvdata), GFP_KERNEL);
    serio = kzalloc_obj(*serio);
    if (!drvdata || !serio) {
    error = -ENOMEM;
    goto err_free_serio;
    }
    error = ps2_gpio_get_props(dev, drvdata);
    if (error)
    goto err_free_serio;
    if (gpiod_cansleep(drvdata.gpio_data) ||
    gpiod_cansleep(drvdata.gpio_clk)) {
    dev_err(dev, "GPIO data or clk are connected via slow bus\n");
    error = -EINVAL;
    goto err_free_serio;
    }
    drvdata.irq = platform_get_irq(pdev, 0);
    if (drvdata.irq < 0) {
    error = drvdata.irq;
    goto err_free_serio;
    }
    error = devm_request_irq(dev, drvdata.irq, ps2_gpio_irq,
    IRQF_NO_THREAD | IRQF_NO_AUTOEN, DRIVER_NAME,
    drvdata);
    if (error) {
    dev_err(dev, "failed to request irq %d: %d\n",
    drvdata.irq, error);
    goto err_free_serio;
    }
    serio.id.type = SERIO_8042;
    serio.open = ps2_gpio_open;
    serio.close = ps2_gpio_close;
//
// Write can be enabled in platform/dt data, but possibly it will not
// work because of the tough timings.
//
    serio.write = drvdata.write_enable ? ps2_gpio_write : core::ptr::null_mut();
    serio.port_data = drvdata;
    serio.dev.parent = dev;
    strscpy(serio.name, dev_name(dev), sizeof(serio.name));
    strscpy(serio.phys, dev_name(dev), sizeof(serio.phys));
    drvdata.serio = serio;
    drvdata.dev = dev;
    drvdata.mode = PS2_MODE_RX;
//
// Tx count always starts at 1, as the start bit is sent implicitly by
// host-to-device communication initialization.
//
    drvdata.tx.cnt = 1;
    INIT_DELAYED_WORK(&drvdata.tx.work, ps2_gpio_tx_work_fn);
    init_completion(&drvdata.tx.complete);
    mutex_init(&drvdata.tx.mutex);
    serio_register_port(serio);
    platform_set_drvdata(pdev, drvdata);
    return 0;	/* success */
    err_free_serio:
    kfree(serio);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn ps2_gpio_remove(pdev: *mut platform_device) {
    static void ps2_gpio_remove(struct platform_device *pdev)
    {
    struct ps2_gpio_data *drvdata = platform_get_drvdata(pdev);
    serio_unregister_port(drvdata.serio);
    }

    static const struct of_device_id ps2_gpio_match[] = {
    { .compatible = "ps2-gpio", },
    { },
    };
    MODULE_DEVICE_TABLE(of, ps2_gpio_match);

    static struct platform_driver ps2_gpio_driver = {
    .probe		= ps2_gpio_probe,
    .remove		= ps2_gpio_remove,
    .driver = {
    .name = DRIVER_NAME,
    .of_match_table = of_match_ptr(ps2_gpio_match),
    },
    };
    module_platform_driver(ps2_gpio_driver);
    MODULE_AUTHOR("Danilo Krummrich <danilokrummrich@dk-develop.de>");
    MODULE_DESCRIPTION("GPIO PS2 driver");
    MODULE_LICENSE("GPL v2");
