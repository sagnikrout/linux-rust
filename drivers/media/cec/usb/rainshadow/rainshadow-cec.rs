//! Automatically rewritten from C to Rust
//! Source: drivers/media/cec/usb/rainshadow/rainshadow-cec.c
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
// RainShadow Tech HDMI CEC driver
//
// Copyright 2016 Hans Verkuil <hverkuil@kernel.org>
//
// Notes:
//
// The higher level protocols are currently disabled. This can be added
// later, similar to how this is done for the Pulse Eight CEC driver.
//
// Documentation of the protocol is available here:
//
// http://rainshadowtech.com/doc/HDMICECtoUSBandRS232v2.0.pdf
//

    MODULE_AUTHOR("Hans Verkuil <hverkuil@kernel.org>");
    MODULE_DESCRIPTION("RainShadow Tech HDMI CEC driver");
    MODULE_LICENSE("GPL");
pub const DATA_SIZE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rain {
    pub dev: *mut device,
    pub serio: *mut serio,
    pub adap: *mut cec_adapter,
    pub cmd_done: completion,
    pub work: work_struct,
// Low-level ringbuffer, collecting incoming characters
    pub buf: [c_char; DATA_SIZE],
    pub buf_rd_idx: c_uint,
    pub buf_wr_idx: c_uint,
    pub buf_len: c_uint,
    pub buf_lock: spinlock_t,
// command buffer
    pub cmd: [c_char; DATA_SIZE],
    pub cmd_idx: c_uint,
    pub cmd_started: bool,
// reply to a command, only used to store the firmware version
    pub cmd_reply: [c_char; DATA_SIZE],
    pub write_lock: mutex,
}

#[no_mangle]
unsafe extern "C" fn rain_process_msg(rain: *mut rain) {
    static void rain_process_msg(struct rain *rain)
    {
    let mut msg: cec_msg = {};
    const char *cmd = rain.cmd + 3;
    let mut stat: c_int = -1;
    for (; *cmd; cmd++) {
    if (!isxdigit(*cmd))
    continue;
    if (isxdigit(cmd[0]) && isxdigit(cmd[1])) {
    if (msg.len == CEC_MAX_MSG_SIZE)
    break;
    if (hex2bin(msg.msg + msg.len, cmd, 1))
    continue;
    msg.len++;
    cmd++;
    continue;
    }
    if (!cmd[1])
    stat = hex_to_bin(cmd[0]);
    break;
    }
    if (rain.cmd[0] == 'R') {
    if (stat == 1 || stat == 2)
    cec_received_msg(rain.adap, &msg);
    return;
    }
    switch (stat) {
    case 1:
    cec_transmit_attempt_done(rain.adap, CEC_TX_STATUS_OK);
    break;
    case 2:
    cec_transmit_attempt_done(rain.adap, CEC_TX_STATUS_NACK);
    break;
    default:
    cec_transmit_attempt_done(rain.adap, CEC_TX_STATUS_LOW_DRIVE);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn rain_irq_work_handler(work: *mut work_struct) {
    static void rain_irq_work_handler(struct work_struct *work)
    {
    struct rain *rain =
    container_of(work, struct rain, work);
    while (true) {
    unsigned long flags;
    char data;
    spin_lock_irqsave(&rain.buf_lock, flags);
    if (!rain.buf_len) {
    spin_unlock_irqrestore(&rain.buf_lock, flags);
    break;
    }
    data = rain.buf[rain.buf_rd_idx];
    rain.buf_len--;
    rain.buf_rd_idx = (rain.buf_rd_idx + 1) & 0xff;
    spin_unlock_irqrestore(&rain.buf_lock, flags);
    if (!rain.cmd_started && data != '?')
    continue;
    switch (data) {
    case '\r':
    rain.cmd[rain.cmd_idx] = '\0';
    dev_dbg(rain.dev, "received: %s\n", rain.cmd);
    if (!memcmp(rain.cmd, "REC", 3) ||
    !memcmp(rain.cmd, "STA", 3)) {
    rain_process_msg(rain);
    } else {
    strscpy(rain.cmd_reply, rain.cmd,
    sizeof(rain.cmd_reply));
    complete(&rain.cmd_done);
    }
    rain.cmd_idx = 0;
    rain.cmd_started = false;
    break;
    case '\n':
    rain.cmd_idx = 0;
    rain.cmd_started = false;
    break;
    case '?':
    rain.cmd_idx = 0;
    rain.cmd_started = true;
    break;
    default:
    if (rain.cmd_idx >= DATA_SIZE - 1) {
    dev_dbg(rain.dev,
    "throwing away %d bytes of garbage\n", rain.cmd_idx);
    rain.cmd_idx = 0;
    }
    rain.cmd[rain.cmd_idx++] = data;
    break;
    }
    }
    }
    static irqreturn_t rain_interrupt(struct serio *serio, unsigned char data,
    unsigned int flags)
    {
    struct rain *rain = serio_get_drvdata(serio);
    spin_lock(&rain.buf_lock);
    if (rain.buf_len == DATA_SIZE) {
    spin_unlock(&rain.buf_lock);
    dev_warn_once(rain.dev, "buffer overflow\n");
    return IRQ_HANDLED;
    }
    rain.buf_len++;
    rain.buf[rain.buf_wr_idx] = data;
    rain.buf_wr_idx = (rain.buf_wr_idx + 1) & 0xff;
    spin_unlock(&rain.buf_lock);
    schedule_work(&rain.work);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rain_disconnect(serio: *mut serio) {
    static void rain_disconnect(struct serio *serio)
    {
    struct rain *rain = serio_get_drvdata(serio);
    cancel_work_sync(&rain.work);
    cec_unregister_adapter(rain.adap);
    dev_info(&serio.dev, "disconnected\n");
    serio_close(serio);
    serio_set_drvdata(serio, core::ptr::null_mut());
    kfree(rain);
    }
#[no_mangle]
unsafe extern "C" fn rain_send(rain: *mut rain, command: *const c_char) -> c_int {
    static int rain_send(struct rain *rain, const char *command)
    {
    let mut err: c_int = serio_write(rain.serio, '!');
    dev_dbg(rain.dev, "send: %s\n", command);
    while (!err && *command)
    err = serio_write(rain.serio, *command++);
    if (!err)
    err = serio_write(rain.serio, '~');
    return err;
    }
    static int rain_send_and_wait(struct rain *rain,
    const char *cmd, const char *reply)
    {
    int err;
    init_completion(&rain.cmd_done);
    mutex_lock(&rain.write_lock);
    err = rain_send(rain, cmd);
    if (err)
    goto err;
    if (!wait_for_completion_timeout(&rain.cmd_done, HZ)) {
    err = -ETIMEDOUT;
    goto err;
    }
    if (reply && strncmp(rain.cmd_reply, reply, strlen(reply))) {
    dev_dbg(rain.dev,
    "transmit of '%s': received '%s' instead of '%s'\n",
    cmd, rain.cmd_reply, reply);
    err = -EIO;
    }
    err:
    mutex_unlock(&rain.write_lock);
    return err;
    }
    static int rain_setup(struct rain *rain, struct serio *serio,
    struct cec_log_addrs *log_addrs, u16 *pa)
    {
    int err;
    err = rain_send_and_wait(rain, "R", "REV");
    if (err)
    return err;
    dev_info(rain.dev, "Firmware version %s\n", rain.cmd_reply + 4);
    err = rain_send_and_wait(rain, "Q 1", "QTY");
    if (err)
    return err;
    err = rain_send_and_wait(rain, "c0000", "CFG");
    if (err)
    return err;
    return rain_send_and_wait(rain, "A F 0000", "ADR");
    }
#[no_mangle]
unsafe extern "C" fn rain_cec_adap_enable(adap: *mut cec_adapter, enable: bool) -> c_int {
    static int rain_cec_adap_enable(struct cec_adapter *adap, bool enable)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rain_cec_adap_log_addr(adap: *mut cec_adapter, log_addr: u8) -> c_int {
    static int rain_cec_adap_log_addr(struct cec_adapter *adap, u8 log_addr)
    {
    struct rain *rain = cec_get_drvdata(adap);
    u8 cmd[16];
    if (log_addr == CEC_LOG_ADDR_INVALID)
    log_addr = CEC_LOG_ADDR_UNREGISTERED;
    snprintf(cmd, sizeof(cmd), "A %x", log_addr);
    return rain_send_and_wait(rain, cmd, "ADR");
    }
    static int rain_cec_adap_transmit(struct cec_adapter *adap, u8 attempts,
    u32 signal_free_time, struct cec_msg *msg)
    {
    struct rain *rain = cec_get_drvdata(adap);
    char cmd[2 * CEC_MAX_MSG_SIZE + 16];
    unsigned int i;
    int err;
    if (msg.len == 1) {
    snprintf(cmd, sizeof(cmd), "x%x", cec_msg_destination(msg));
    } else {
    char hex[3];
    snprintf(cmd, sizeof(cmd), "x%x %02x ",
    cec_msg_destination(msg), msg.msg[1]);
    for (i = 2; i < msg.len; i++) {
    snprintf(hex, sizeof(hex), "%02x", msg.msg[i]);
    strlcat(cmd, hex, sizeof(cmd));
    }
    }
    mutex_lock(&rain.write_lock);
    err = rain_send(rain, cmd);
    mutex_unlock(&rain.write_lock);
    return err;
    }
    static const struct cec_adap_ops rain_cec_adap_ops = {
    .adap_enable = rain_cec_adap_enable,
    .adap_log_addr = rain_cec_adap_log_addr,
    .adap_transmit = rain_cec_adap_transmit,
    };
#[no_mangle]
unsafe extern "C" fn rain_connect(serio: *mut serio, drv: *mut serio_driver) -> c_int {
    static int rain_connect(struct serio *serio, struct serio_driver *drv)
    {
    let mut caps: u32 = CEC_CAP_DEFAULTS | CEC_CAP_PHYS_ADDR | CEC_CAP_MONITOR_ALL;
    struct rain *rain;
    let mut err: c_int = -ENOMEM;
    let mut log_addrs: cec_log_addrs = {};
    let mut pa: u16 = CEC_PHYS_ADDR_INVALID;
    rain = kzalloc_obj(*rain);
    if (!rain)
    return -ENOMEM;
    rain.serio = serio;
    rain.adap = cec_allocate_adapter(&rain_cec_adap_ops, rain,
    dev_name(&serio.dev), caps, 1);
    err = PTR_ERR_OR_ZERO(rain.adap);
    if (err < 0)
    goto free_device;
    rain.dev = &serio.dev;
    serio_set_drvdata(serio, rain);
    INIT_WORK(&rain.work, rain_irq_work_handler);
    mutex_init(&rain.write_lock);
    spin_lock_init(&rain.buf_lock);
    err = serio_open(serio, drv);
    if (err)
    goto delete_adap;
    err = rain_setup(rain, serio, &log_addrs, &pa);
    if (err)
    goto close_serio;
    err = cec_register_adapter(rain.adap, &serio.dev);
    if (err < 0)
    goto close_serio;
    rain.dev = &rain.adap.devnode.dev;
    return 0;
    close_serio:
    serio_close(serio);
    delete_adap:
    cec_delete_adapter(rain.adap);
    serio_set_drvdata(serio, core::ptr::null_mut());
    free_device:
    kfree(rain);
    return err;
    }
    static const struct serio_device_id rain_serio_ids[] = {
    {
    .type	= SERIO_RS232,
    .proto	= SERIO_RAINSHADOW_CEC,
    .id	= SERIO_ANY,
    .extra	= SERIO_ANY,
    },
    { 0 }
    };
    MODULE_DEVICE_TABLE(serio, rain_serio_ids);
    static struct serio_driver rain_drv = {
    .driver		= {
    .name	= "rainshadow-cec",
    },
    .description	= "RainShadow Tech HDMI CEC driver",
    .id_table	= rain_serio_ids,
    .interrupt	= rain_interrupt,
    .connect	= rain_connect,
    .disconnect	= rain_disconnect,
    };
    module_serio_driver(rain_drv);
