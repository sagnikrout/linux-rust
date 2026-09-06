//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/qnap-mcu.c
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
// Core driver for the microcontroller unit in QNAP NAS devices that is
// connected via a dedicated UART port.
//
// Copyright (C) 2024 Heiko Stuebner <heiko@sntech.de>
//

// The longest command found so far is 5 bytes long
pub const QNAP_MCU_MAX_CMD_SIZE: c_int = 5;
pub const QNAP_MCU_MAX_DATA_SIZE: c_int = 36;
pub const QNAP_MCU_ERROR_SIZE: c_int = 2;
pub const QNAP_MCU_CHECKSUM_SIZE: c_int = 1;

    (QNAP_MCU_MAX_DATA_SIZE + QNAP_MCU_CHECKSUM_SIZE)

    (QNAP_MCU_MAX_CMD_SIZE + QNAP_MCU_CHECKSUM_SIZE)
pub const QNAP_MCU_ACK_LEN: c_int = 2;
pub const QNAP_MCU_VERSION_LEN: c_int = 4;
pub const QNAP_MCU_TIMEOUT_MS: c_int = 500;
//
// struct qnap_mcu_reply - Reply to a command
//
// @data:	Buffer to store reply payload in
// @length:	Expected reply length, including the checksum
// @received:	Received number of bytes, so far
// @done:	Triggered when the entire reply has been received
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qnap_mcu_reply {
    pub data: *mut u8,
    pub length: usize,
    pub received: usize,
    pub done: completion,
}

//
// struct qnap_mcu - QNAP NAS embedded controller
//
// @serdev:	Pointer to underlying serdev
// @bus_lock:	Lock to serialize access to the device
// @reply:	Reply data structure
// @variant:	Device variant specific information
// @version:	MCU firmware version
// @rx:		Receive buffer the reply is assembled in
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qnap_mcu {
    pub serdev: *mut serdev_device,
    pub bus_lock: mutex,
    pub reply: qnap_mcu_reply,
    pub variant: *const qnap_mcu_variant,
    pub version: [u8; QNAP_MCU_VERSION_LEN],
    pub rx: [u8; QNAP_MCU_RX_BUFFER_SIZE],
}

//
// The QNAP-MCU uses a basic XOR checksum.
// It is always the last byte and XORs the whole previous message.
//
#[no_mangle]
unsafe extern "C" fn qnap_mcu_csum(buf: *const u8, size: usize) -> u8 {
    static u8 qnap_mcu_csum(const u8 *buf, size_t size)
    {
    let mut csum: u8 = 0;
    while (size--)
    csum ^= *buf++;
    return csum;
    }
#[no_mangle]
unsafe extern "C" fn qnap_mcu_verify_checksum(buf: *const u8, size: usize) -> bool {
    static bool qnap_mcu_verify_checksum(const u8 *buf, size_t size)
    {
    let mut crc: u8 = qnap_mcu_csum(buf, size - QNAP_MCU_CHECKSUM_SIZE);
    let mut crc: return = = buf[size - QNAP_MCU_CHECKSUM_SIZE];
    }
#[no_mangle]
unsafe extern "C" fn qnap_mcu_write(mcu: *mut qnap_mcu, data: *const u8, data_size: u8) -> c_int {
    static int qnap_mcu_write(struct qnap_mcu *mcu, const u8 *data, u8 data_size)
    {
    unsigned char tx[QNAP_MCU_TX_BUFFER_SIZE];
    let mut length: usize = data_size + QNAP_MCU_CHECKSUM_SIZE;
    if (length > sizeof(tx)) {
    dev_err(&mcu.serdev.dev, "data too big for transmit buffer");
    return -EINVAL;
    }
    memcpy(tx, data, data_size);
    tx[data_size] = qnap_mcu_csum(data, data_size);
    serdev_device_write_flush(mcu.serdev);
    return serdev_device_write(mcu.serdev, tx, length, HZ);
    }
#[no_mangle]
unsafe extern "C" fn qnap_mcu_is_error_msg(size: usize) -> bool {
    static bool qnap_mcu_is_error_msg(size_t size)
    {
    return (size == QNAP_MCU_ERROR_SIZE + QNAP_MCU_CHECKSUM_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn qnap_mcu_reply_is_generic_error(buf: *mut c_uchar, size: usize) -> bool {
    static bool qnap_mcu_reply_is_generic_error(unsigned char *buf, size_t size)
    {
    if (!qnap_mcu_is_error_msg(size))
    return false;
    if (buf[0] == '@' && buf[1] == '9')
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn qnap_mcu_reply_is_checksum_error(buf: *mut c_uchar, size: usize) -> bool {
    static bool qnap_mcu_reply_is_checksum_error(unsigned char *buf, size_t size)
    {
    if (!qnap_mcu_is_error_msg(size))
    return false;
    if (buf[0] == '@' && buf[1] == '8')
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn qnap_mcu_reply_is_any_error(mcu: *mut qnap_mcu, buf: *mut c_uchar, size: usize) -> bool {
    static bool qnap_mcu_reply_is_any_error(struct qnap_mcu *mcu, unsigned char *buf, size_t size)
    {
    if (qnap_mcu_reply_is_generic_error(buf, size)) {
    dev_err(&mcu.serdev.dev, "Controller sent generic error response\n");
    return true;
    }
    if (qnap_mcu_reply_is_checksum_error(buf, size)) {
    dev_err(&mcu.serdev.dev, "Controller received invalid checksum for the command\n");
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn qnap_mcu_receive_buf(serdev: *mut serdev_device, buf: *const u8, size: usize) -> usize {
    static size_t qnap_mcu_receive_buf(struct serdev_device *serdev, const u8 *buf, size_t size)
    {
    struct device *dev = &serdev.dev;
    struct qnap_mcu *mcu = dev_get_drvdata(dev);
    struct qnap_mcu_reply *reply = &mcu.reply;
    const u8 *src = buf;
    const u8 *end = buf + size;
    if (!reply.length) {
    dev_warn(dev, "Received %zu bytes, we were not waiting for\n", size);
    return size;
    }
    while (src < end) {
    reply.data[reply.received] = *src++;
    reply.received++;
    if (reply.received == reply.length) {
// We don't expect any characters from the device now
    reply.length = 0;
    complete(&reply.done);
//
// We report the consumed number of bytes. If there
// are still bytes remaining (though there shouldn't)
// the serdev layer will re-execute this handler with
// the remainder of the Rx bytes.
//
    return src - buf;
    }
    }
//
// We received everything the uart had to offer for now.
// This could mean that either the uart will send more in a 2nd
// receive run, or that the MCU cut the reply short because it
// sent an error code instead of the expected reply.
//
// So check if the received data has the correct size for an error
// reply and if it matches, is an actual error code.
//
    if (qnap_mcu_is_error_msg(reply.received) &&
    qnap_mcu_verify_checksum(reply.data, reply.received) &&
    qnap_mcu_reply_is_any_error(mcu, reply.data, reply.received)) {
// The reply was an error code, we're done
    reply.length = 0;
    complete(&reply.done);
    }
//
// The only way to get out of the above loop and end up here
// is through consuming all of the supplied data, so here we
// report that we processed it all.
//
    return size;
    }
    static const struct serdev_device_ops qnap_mcu_serdev_device_ops = {
    .receive_buf  = qnap_mcu_receive_buf,
    .write_wakeup = serdev_device_write_wakeup,
    };
    int qnap_mcu_exec(struct qnap_mcu *mcu,
    const u8 *cmd_data, size_t cmd_data_size,
    u8 *reply_data, size_t reply_data_size)
    {
    let mut length: usize = reply_data_size + QNAP_MCU_CHECKSUM_SIZE;
    struct qnap_mcu_reply *reply = &mcu.reply;
    let mut ret: c_int = 0;
    if (length > sizeof(mcu.rx)) {
    dev_err(&mcu.serdev.dev, "expected data too big for receive buffer");
    return -EINVAL;
    }
    guard(mutex)(&mcu.bus_lock);
    reply.data = mcu.rx;
    reply.length = length;
    reply.received = 0;
    reinit_completion(&reply.done);
    ret = qnap_mcu_write(mcu, cmd_data, cmd_data_size);
    if (ret < 0)
    return ret;
    serdev_device_wait_until_sent(mcu.serdev, msecs_to_jiffies(QNAP_MCU_TIMEOUT_MS));
    if (!wait_for_completion_timeout(&reply.done, msecs_to_jiffies(QNAP_MCU_TIMEOUT_MS))) {
    dev_err(&mcu.serdev.dev, "Command timeout\n");
    return -ETIMEDOUT;
    }
    if (!qnap_mcu_verify_checksum(mcu.rx, reply.received)) {
    dev_err(&mcu.serdev.dev, "Invalid Checksum received from controller\n");
    return -EPROTO;
    }
    if (qnap_mcu_reply_is_any_error(mcu, mcu.rx, reply.received))
    return -EPROTO;
    memcpy(reply_data, mcu.rx, reply_data_size);
    return 0;
    }
    EXPORT_SYMBOL_GPL(qnap_mcu_exec);
    int qnap_mcu_exec_with_ack(struct qnap_mcu *mcu,
    const u8 *cmd_data, size_t cmd_data_size)
    {
    u8 ack[QNAP_MCU_ACK_LEN];
    int ret;
    ret = qnap_mcu_exec(mcu, cmd_data, cmd_data_size, ack, sizeof(ack));
    if (ret)
    return ret;
// Should return @0
    if (ack[0] != '@' || ack[1] != '0') {
    dev_err(&mcu.serdev.dev, "Did not receive ack\n");
    return -EIO;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(qnap_mcu_exec_with_ack);
#[no_mangle]
unsafe extern "C" fn qnap_mcu_get_version(mcu: *mut qnap_mcu) -> c_int {
    static int qnap_mcu_get_version(struct qnap_mcu *mcu)
    {
    const u8 cmd[] = { '%', 'V' };
    u8 rx[14];
    int ret;
// Reply is the 2 command-bytes + 4 bytes describing the version
    ret = qnap_mcu_exec(mcu, cmd, sizeof(cmd), rx, QNAP_MCU_VERSION_LEN + 2);
    if (ret)
    return ret;
    memcpy(mcu.version, &rx[2], QNAP_MCU_VERSION_LEN);
    return 0;
    }
//
// The MCU controls power to the peripherals but not the CPU.
//
// So using the PMIC to power off the system keeps the MCU and hard-drives
// running. This also then prevents the system from turning back on until
// the MCU is turned off by unplugging the power cable.
// Turning off the MCU alone on the other hand turns off the hard drives,
// LEDs, etc while the main SoC stays running - including its network ports.
//
#[no_mangle]
unsafe extern "C" fn qnap_mcu_power_off(data: *mut sys_off_data) -> c_int {
    static int qnap_mcu_power_off(struct sys_off_data *data)
    {
    const u8 cmd[] = { '@', 'C', '0' };
    struct qnap_mcu *mcu = data.cb_data;
    int ret;
    ret = qnap_mcu_exec_with_ack(mcu, cmd, sizeof(cmd));
    if (ret) {
    dev_err(&mcu.serdev.dev, "MCU poweroff failed %d\n", ret);
    return NOTIFY_STOP;
    }
    return NOTIFY_DONE;
    }
    static const struct qnap_mcu_variant qnap_ts133_mcu = {
    .baud_rate = 115200,
    .num_drives = 1,
    .fan_pwm_min = 51,  /* Specified in original model.conf */
    .fan_pwm_max = 255,
    .usb_led = false,
    };
    static const struct qnap_mcu_variant qnap_ts233_mcu = {
    .baud_rate = 115200,
    .num_drives = 2,
    .fan_pwm_min = 51,  /* Specified in original model.conf */
    .fan_pwm_max = 255,
    .usb_led = true,
    };
    static const struct qnap_mcu_variant qnap_ts433_mcu = {
    .baud_rate = 115200,
    .num_drives = 4,
    .fan_pwm_min = 51,  /* Specified in original model.conf */
    .fan_pwm_max = 255,
    .usb_led = true,
    };
    static struct mfd_cell qnap_mcu_cells[] = {
    { .name = "qnap-mcu-eeprom", },
    { .name = "qnap-mcu-input", },
    { .name = "qnap-mcu-leds", },
    { .name = "qnap-mcu-hwmon", }
    };
#[no_mangle]
unsafe extern "C" fn qnap_mcu_probe(serdev: *mut serdev_device) -> c_int {
    static int qnap_mcu_probe(struct serdev_device *serdev)
    {
    struct device *dev = &serdev.dev;
    struct qnap_mcu *mcu;
    int ret;
    mcu = devm_kzalloc(dev, sizeof(*mcu), GFP_KERNEL);
    if (!mcu)
    return -ENOMEM;
    mcu.serdev = serdev;
    dev_set_drvdata(dev, mcu);
    mcu.variant = of_device_get_match_data(dev);
    if (!mcu.variant)
    return -ENODEV;
    mutex_init(&mcu.bus_lock);
    init_completion(&mcu.reply.done);
    serdev_device_set_client_ops(serdev, &qnap_mcu_serdev_device_ops);
    ret = devm_serdev_device_open(dev, serdev);
    if (ret)
    return ret;
    serdev_device_set_baudrate(serdev, mcu.variant.baud_rate);
    serdev_device_set_flow_control(serdev, false);
    ret = serdev_device_set_parity(serdev, SERDEV_PARITY_NONE);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to set parity\n");
    ret = qnap_mcu_get_version(mcu);
    if (ret)
    return ret;
    ret = devm_register_sys_off_handler(dev,
    SYS_OFF_MODE_POWER_OFF_PREPARE,
    SYS_OFF_PRIO_DEFAULT,
    &qnap_mcu_power_off, mcu);
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed to register poweroff handler\n");
    for (int i = 0; i < ARRAY_SIZE(qnap_mcu_cells); i++) {
    qnap_mcu_cells[i].platform_data = mcu.variant;
    qnap_mcu_cells[i].pdata_size = sizeof(*mcu.variant);
    }
    ret = devm_mfd_add_devices(dev, PLATFORM_DEVID_AUTO, qnap_mcu_cells,
    ARRAY_SIZE(qnap_mcu_cells), core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (ret)
    return dev_err_probe(dev, ret, "Failed to add child devices\n");
    return 0;
    }
    static const struct of_device_id qnap_mcu_dt_ids[] = {
    { .compatible = "qnap,ts133-mcu", .data = &qnap_ts133_mcu },
    { .compatible = "qnap,ts233-mcu", .data = &qnap_ts233_mcu },
    { .compatible = "qnap,ts433-mcu", .data = &qnap_ts433_mcu },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, qnap_mcu_dt_ids);
    static struct serdev_device_driver qnap_mcu_drv = {
    .probe = qnap_mcu_probe,
    .driver = {
    .name = "qnap-mcu",
    .of_match_table = qnap_mcu_dt_ids,
    },
    };
    module_serdev_device_driver(qnap_mcu_drv);
    MODULE_AUTHOR("Heiko Stuebner <heiko@sntech.de>");
    MODULE_DESCRIPTION("QNAP MCU core driver");
    MODULE_LICENSE("GPL");
