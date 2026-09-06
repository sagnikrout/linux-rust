//! Automatically rewritten from C to Rust
//! Source: drivers/media/cec/i2c/ch7322.c
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
// Driver for the Chrontel CH7322 CEC Controller
//
// Copyright 2020 Google LLC.
//
// Notes
//
// - This device powers on in Auto Mode which has limited functionality. This
// driver disables Auto Mode when it attaches.
//

pub const CH7322_WRITE: c_uint = 0x00;
pub const CH7322_WRITE_MSENT: c_uint = 0x80;
pub const CH7322_WRITE_BOK: c_uint = 0x40;
pub const CH7322_WRITE_NMASK: c_uint = 0x0f;
// Write buffer is 0x01-0x10
pub const CH7322_WRBUF: c_uint = 0x01;
pub const CH7322_WRBUF_LEN: c_uint = 0x10;
pub const CH7322_READ: c_uint = 0x40;
pub const CH7322_READ_NRDT: c_uint = 0x80;
pub const CH7322_READ_MSENT: c_uint = 0x20;
pub const CH7322_READ_NMASK: c_uint = 0x0f;
// Read buffer is 0x41-0x50
pub const CH7322_RDBUF: c_uint = 0x41;
pub const CH7322_RDBUF_LEN: c_uint = 0x10;
pub const CH7322_MODE: c_uint = 0x11;
pub const CH7322_MODE_AUTO: c_uint = 0x78;
pub const CH7322_MODE_SW: c_uint = 0xb5;
pub const CH7322_RESET: c_uint = 0x12;
pub const CH7322_RESET_RST: c_uint = 0x00;
pub const CH7322_POWER: c_uint = 0x13;
pub const CH7322_POWER_FPD: c_uint = 0x04;
pub const CH7322_CFG0: c_uint = 0x17;
pub const CH7322_CFG0_EOBEN: c_uint = 0x40;
pub const CH7322_CFG0_PEOB: c_uint = 0x20;
pub const CH7322_CFG0_CLRSPP: c_uint = 0x10;
pub const CH7322_CFG0_FLOW: c_uint = 0x08;
pub const CH7322_CFG1: c_uint = 0x1a;
pub const CH7322_CFG1_STDBYO: c_uint = 0x04;
pub const CH7322_CFG1_HPBP: c_uint = 0x02;
pub const CH7322_CFG1_PIO: c_uint = 0x01;
pub const CH7322_INTCTL: c_uint = 0x1b;
pub const CH7322_INTCTL_INTPB: c_uint = 0x80;
pub const CH7322_INTCTL_STDBY: c_uint = 0x40;
pub const CH7322_INTCTL_HPDFALL: c_uint = 0x20;
pub const CH7322_INTCTL_HPDRISE: c_uint = 0x10;
pub const CH7322_INTCTL_RXMSG: c_uint = 0x08;
pub const CH7322_INTCTL_TXMSG: c_uint = 0x04;
pub const CH7322_INTCTL_NEWPHA: c_uint = 0x02;
pub const CH7322_INTCTL_ERROR: c_uint = 0x01;
pub const CH7322_DVCLKFNH: c_uint = 0x1d;
pub const CH7322_DVCLKFNL: c_uint = 0x1e;
pub const CH7322_CTL: c_uint = 0x31;
pub const CH7322_CTL_FSTDBY: c_uint = 0x80;
pub const CH7322_CTL_PLSEN: c_uint = 0x40;
pub const CH7322_CTL_PLSPB: c_uint = 0x20;
pub const CH7322_CTL_SPADL: c_uint = 0x10;
pub const CH7322_CTL_HINIT: c_uint = 0x08;
pub const CH7322_CTL_WPHYA: c_uint = 0x04;
pub const CH7322_CTL_H1T: c_uint = 0x02;
pub const CH7322_CTL_S1T: c_uint = 0x01;
pub const CH7322_PAWH: c_uint = 0x32;
pub const CH7322_PAWL: c_uint = 0x33;
pub const CH7322_ADDLW: c_uint = 0x34;
pub const CH7322_ADDLW_MASK: c_uint = 0xf0;
pub const CH7322_ADDLR: c_uint = 0x3d;
pub const CH7322_ADDLR_HPD: c_uint = 0x80;
pub const CH7322_ADDLR_MASK: c_uint = 0x0f;
pub const CH7322_INTDATA: c_uint = 0x3e;
pub const CH7322_INTDATA_MODE: c_uint = 0x80;
pub const CH7322_INTDATA_STDBY: c_uint = 0x40;
pub const CH7322_INTDATA_HPDFALL: c_uint = 0x20;
pub const CH7322_INTDATA_HPDRISE: c_uint = 0x10;
pub const CH7322_INTDATA_RXMSG: c_uint = 0x08;
pub const CH7322_INTDATA_TXMSG: c_uint = 0x04;
pub const CH7322_INTDATA_NEWPHA: c_uint = 0x02;
pub const CH7322_INTDATA_ERROR: c_uint = 0x01;
pub const CH7322_EVENT: c_uint = 0x3f;
pub const CH7322_EVENT_TXERR: c_uint = 0x80;
pub const CH7322_EVENT_HRST: c_uint = 0x40;
pub const CH7322_EVENT_HFST: c_uint = 0x20;
pub const CH7322_EVENT_PHACHG: c_uint = 0x10;
pub const CH7322_EVENT_ACTST: c_uint = 0x08;
pub const CH7322_EVENT_PHARDY: c_uint = 0x04;
pub const CH7322_EVENT_BSOK: c_uint = 0x02;
pub const CH7322_EVENT_ERRADCF: c_uint = 0x01;
pub const CH7322_DID: c_uint = 0x51;
pub const CH7322_DID_CH7322: c_uint = 0x5b;
pub const CH7322_DID_CH7323: c_uint = 0x5f;
pub const CH7322_REVISIONID: c_uint = 0x52;
pub const CH7322_PARH: c_uint = 0x53;
pub const CH7322_PARL: c_uint = 0x54;
pub const CH7322_IOCFG2: c_uint = 0x75;
pub const CH7322_IOCFG_CIO: c_uint = 0x80;
pub const CH7322_IOCFG_IOCFGMASK: c_uint = 0x78;
pub const CH7322_IOCFG_AUDIO: c_uint = 0x04;
pub const CH7322_IOCFG_SPAMST: c_uint = 0x02;
pub const CH7322_IOCFG_SPAMSP: c_uint = 0x01;
pub const CH7322_CTL3: c_uint = 0x7b;
pub const CH7322_CTL3_SWENA: c_uint = 0x80;
pub const CH7322_CTL3_FC_INIT: c_uint = 0x40;
pub const CH7322_CTL3_SML_FL: c_uint = 0x20;
pub const CH7322_CTL3_SM_RDST: c_uint = 0x10;
pub const CH7322_CTL3_SPP_CIAH: c_uint = 0x08;
pub const CH7322_CTL3_SPP_CIAL: c_uint = 0x04;
pub const CH7322_CTL3_SPP_ACTH: c_uint = 0x02;
pub const CH7322_CTL3_SPP_ACTL: c_uint = 0x01;
// BOK status means NACK

// Device will retry automatically

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch7322 {
    pub i2c: *mut i2c_client,
    pub regmap: *mut regmap,
    pub cec: *mut cec_adapter,
    pub /: *mut *mut mutex mutex; / device access mutex,
    pub tx_flags: u8,
}

    static const struct regmap_config ch7322_regmap = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0x7f,
    .disable_locking = true,
    };
#[no_mangle]
unsafe extern "C" fn ch7322_send_message(ch7322: *mut ch7322, msg: *const cec_msg) -> c_int {
    static int ch7322_send_message(struct ch7322 *ch7322, const struct cec_msg *msg)
    {
    unsigned int val;
    let mut len: c_uint = msg.len;
    int ret;
    int i;
    WARN_ON(!mutex_is_locked(&ch7322.mutex));
    if (len > CH7322_WRBUF_LEN || len < 1)
    return -EINVAL;
    ret = regmap_read(ch7322.regmap, CH7322_WRITE, &val);
    if (ret)
    return ret;
// Buffer not ready
    if (!(val & CH7322_WRITE_MSENT))
    return -EBUSY;
    if (cec_msg_opcode(msg) == -1 &&
    cec_msg_initiator(msg) == cec_msg_destination(msg)) {
    ch7322.tx_flags = CH7322_TX_FLAG_NACK | CH7322_TX_FLAG_RETRY;
    } else if (cec_msg_is_broadcast(msg)) {
    ch7322.tx_flags = CH7322_TX_FLAG_NACK;
    } else {
    ch7322.tx_flags = CH7322_TX_FLAG_RETRY;
    }
    ret = regmap_write(ch7322.regmap, CH7322_WRITE, len - 1);
    if (ret)
    return ret;
    for (i = 0; i < len; i++) {
    ret = regmap_write(ch7322.regmap,
    CH7322_WRBUF + i, msg.msg[i]);
    if (ret)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ch7322_receive_message(ch7322: *mut ch7322, msg: *mut cec_msg) -> c_int {
    static int ch7322_receive_message(struct ch7322 *ch7322, struct cec_msg *msg)
    {
    unsigned int val;
    let mut ret: c_int = 0;
    int i;
    WARN_ON(!mutex_is_locked(&ch7322.mutex));
    ret = regmap_read(ch7322.regmap, CH7322_READ, &val);
    if (ret)
    return ret;
// Message not ready
    if (!(val & CH7322_READ_NRDT))
    return -EIO;
    msg.len = (val & CH7322_READ_NMASK) + 1;
// Read entire RDBUF to clear state
    for (i = 0; i < CH7322_RDBUF_LEN; i++) {
    ret = regmap_read(ch7322.regmap, CH7322_RDBUF + i, &val);
    if (ret)
    return ret;
    msg.msg[i] = (u8)val;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ch7322_tx_done(ch7322: *mut ch7322) {
    static void ch7322_tx_done(struct ch7322 *ch7322)
    {
    int ret;
    unsigned int val;
    u8 status, flags;
    mutex_lock(&ch7322.mutex);
    ret = regmap_read(ch7322.regmap, CH7322_WRITE, &val);
    flags = ch7322.tx_flags;
    mutex_unlock(&ch7322.mutex);
//
// The device returns a one-bit OK status which usually means ACK but
// actually means NACK when sending a logical address query or a
// broadcast.
//
    if (ret)
    status = CEC_TX_STATUS_ERROR;
#[no_mangle]
pub unsafe extern "C" fn if(CH7322_TX_FLAG_NACK): (val & CH7322_WRITE_BOK) && (flags &) -> else {
    else if ((val & CH7322_WRITE_BOK) && (flags & CH7322_TX_FLAG_NACK))
    status = CEC_TX_STATUS_NACK;
#[no_mangle]
pub unsafe extern "C" fn if(CH7322_WRITE_BOK: val &) -> else {
    else if (val & CH7322_WRITE_BOK)
    status = CEC_TX_STATUS_OK;
#[no_mangle]
pub unsafe extern "C" fn if(CH7322_TX_FLAG_NACK: flags &) -> else {
    else if (flags & CH7322_TX_FLAG_NACK)
    status = CEC_TX_STATUS_OK;
    else
    status = CEC_TX_STATUS_NACK;
    if (status == CEC_TX_STATUS_NACK && (flags & CH7322_TX_FLAG_RETRY))
    status |= CEC_TX_STATUS_MAX_RETRIES;
    cec_transmit_attempt_done(ch7322.cec, status);
    }
#[no_mangle]
unsafe extern "C" fn ch7322_rx_done(ch7322: *mut ch7322) {
    static void ch7322_rx_done(struct ch7322 *ch7322)
    {
    struct cec_msg msg;
    int ret;
    mutex_lock(&ch7322.mutex);
    ret = ch7322_receive_message(ch7322, &msg);
    mutex_unlock(&ch7322.mutex);
    if (ret)
    dev_err(&ch7322.i2c.dev, "cec receive error: %d\n", ret);
    else
    cec_received_msg(ch7322.cec, &msg);
    }
//
// This device can either monitor the DDC lines to obtain the physical address
// or it can allow the host to program it. This driver lets the device obtain
// it.
//
#[no_mangle]
unsafe extern "C" fn ch7322_phys_addr(ch7322: *mut ch7322) {
    static void ch7322_phys_addr(struct ch7322 *ch7322)
    {
    unsigned int pah, pal;
    let mut ret: c_int = 0;
    mutex_lock(&ch7322.mutex);
    ret |= regmap_read(ch7322.regmap, CH7322_PARH, &pah);
    ret |= regmap_read(ch7322.regmap, CH7322_PARL, &pal);
    mutex_unlock(&ch7322.mutex);
    if (ret)
    dev_err(&ch7322.i2c.dev, "phys addr error\n");
    else
    cec_s_phys_addr(ch7322.cec, pal | (pah << 8), false);
    }
#[no_mangle]
unsafe extern "C" fn ch7322_irq(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t ch7322_irq(int irq, void *dev)
    {
    struct ch7322 *ch7322 = dev;
    let mut data: c_uint = 0;
    mutex_lock(&ch7322.mutex);
    regmap_read(ch7322.regmap, CH7322_INTDATA, &data);
    regmap_write(ch7322.regmap, CH7322_INTDATA, data);
    mutex_unlock(&ch7322.mutex);
    if (data & CH7322_INTDATA_HPDFALL)
    cec_phys_addr_invalidate(ch7322.cec);
    if (data & CH7322_INTDATA_TXMSG)
    ch7322_tx_done(ch7322);
    if (data & CH7322_INTDATA_RXMSG)
    ch7322_rx_done(ch7322);
    if (data & CH7322_INTDATA_NEWPHA)
    ch7322_phys_addr(ch7322);
    if (data & CH7322_INTDATA_ERROR)
    dev_dbg(&ch7322.i2c.dev, "unknown error\n");
    return IRQ_HANDLED;
    }
// This device is always enabled
#[no_mangle]
unsafe extern "C" fn ch7322_cec_adap_enable(adap: *mut cec_adapter, enable: bool) -> c_int {
    static int ch7322_cec_adap_enable(struct cec_adapter *adap, bool enable)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ch7322_cec_adap_log_addr(adap: *mut cec_adapter, log_addr: u8) -> c_int {
    static int ch7322_cec_adap_log_addr(struct cec_adapter *adap, u8 log_addr)
    {
    struct ch7322 *ch7322 = cec_get_drvdata(adap);
    int ret;
    mutex_lock(&ch7322.mutex);
    ret = regmap_update_bits(ch7322.regmap, CH7322_ADDLW,
    CH7322_ADDLW_MASK, log_addr << 4);
    mutex_unlock(&ch7322.mutex);
    return ret;
    }
    static int ch7322_cec_adap_transmit(struct cec_adapter *adap, u8 attempts,
    u32 signal_free_time, struct cec_msg *msg)
    {
    struct ch7322 *ch7322 = cec_get_drvdata(adap);
    int ret;
    mutex_lock(&ch7322.mutex);
    ret = ch7322_send_message(ch7322, msg);
    mutex_unlock(&ch7322.mutex);
    return ret;
    }
    static const struct cec_adap_ops ch7322_cec_adap_ops = {
    .adap_enable = ch7322_cec_adap_enable,
    .adap_log_addr = ch7322_cec_adap_log_addr,
    .adap_transmit = ch7322_cec_adap_transmit,
    };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch7322_conn_match {
    pub dev_name: *const c_char,
    pub pci_name: *const c_char,
    pub port_name: *const c_char,
}

    static struct ch7322_conn_match google_endeavour[] = {
    { "i2c-PRP0001:00", "0000:00:02.0", "Port B" },
    { "i2c-PRP0001:01", "0000:00:02.0", "Port C" },
    { },
    };
    static const struct dmi_system_id ch7322_dmi_table[] = {
    {
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "Google"),
    DMI_MATCH(DMI_BOARD_NAME, "Endeavour"),
    },
    .driver_data = google_endeavour,
    },
    { },
    };
// Make a best-effort attempt to locate a matching HDMI port
    static int ch7322_get_port(struct i2c_client *client,
    struct device **dev,
    const char **port)
    {
    const struct dmi_system_id *system;
    const struct ch7322_conn_match *conn;
// dev = NULL;
// port = NULL;
    system = dmi_first_match(ch7322_dmi_table);
    if (!system)
    return 0;
    for (conn = system.driver_data; conn.dev_name; conn++) {
    if (!strcmp(dev_name(&client.dev), conn.dev_name)) {
    struct device *d;
    d = bus_find_device_by_name(&pci_bus_type, core::ptr::null_mut(),
    conn.pci_name);
    if (!d)
    return -EPROBE_DEFER;
    put_device(d);
// dev = d;
// port = conn->port_name;
    return 0;
    }
    }
    return 0;
    }

    static int ch7322_get_port(struct i2c_client *client,
    struct device **dev,
    const char **port)
    {
// dev = NULL;
// port = NULL;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn ch7322_probe(client: *mut i2c_client) -> c_int {
    static int ch7322_probe(struct i2c_client *client)
    {
    struct device *hdmi_dev;
    const char *port_name;
    struct ch7322 *ch7322;
    struct cec_notifier *notifier = core::ptr::null_mut();
    let mut caps: u32 = CEC_CAP_DEFAULTS;
    int ret;
    unsigned int val;
    ret = ch7322_get_port(client, &hdmi_dev, &port_name);
    if (ret)
    return ret;
    if (hdmi_dev)
    caps |= CEC_CAP_CONNECTOR_INFO;
    ch7322 = devm_kzalloc(&client.dev, sizeof(*ch7322), GFP_KERNEL);
    if (!ch7322)
    return -ENOMEM;
    ch7322.regmap = devm_regmap_init_i2c(client, &ch7322_regmap);
    if (IS_ERR(ch7322.regmap))
    return PTR_ERR(ch7322.regmap);
    ret = regmap_read(ch7322.regmap, CH7322_DID, &val);
    if (ret)
    return ret;
    if (val != CH7322_DID_CH7322)
    return -EOPNOTSUPP;
    mutex_init(&ch7322.mutex);
    ch7322.i2c = client;
    ch7322.tx_flags = 0;
    i2c_set_clientdata(client, ch7322);
// Disable auto mode
    ret = regmap_write(ch7322.regmap, CH7322_MODE, CH7322_MODE_SW);
    if (ret)
    goto err_mutex;
// Enable logical address register
    ret = regmap_update_bits(ch7322.regmap, CH7322_CTL,
    CH7322_CTL_SPADL, CH7322_CTL_SPADL);
    if (ret)
    goto err_mutex;
    ch7322.cec = cec_allocate_adapter(&ch7322_cec_adap_ops, ch7322,
    dev_name(&client.dev),
    caps, 1);
    if (IS_ERR(ch7322.cec)) {
    ret = PTR_ERR(ch7322.cec);
    goto err_mutex;
    }
    ch7322.cec.adap_controls_phys_addr = true;
    if (hdmi_dev) {
    notifier = cec_notifier_cec_adap_register(hdmi_dev,
    port_name,
    ch7322.cec);
    if (!notifier) {
    ret = -ENOMEM;
    goto err_cec;
    }
    }
// Configure, mask, and clear interrupt
    ret = regmap_write(ch7322.regmap, CH7322_CFG1, 0);
    if (ret)
    goto err_notifier;
    ret = regmap_write(ch7322.regmap, CH7322_INTCTL, CH7322_INTCTL_INTPB);
    if (ret)
    goto err_notifier;
    ret = regmap_write(ch7322.regmap, CH7322_INTDATA, 0xff);
    if (ret)
    goto err_notifier;
// If HPD is up read physical address
    ret = regmap_read(ch7322.regmap, CH7322_ADDLR, &val);
    if (ret)
    goto err_notifier;
    if (val & CH7322_ADDLR_HPD)
    ch7322_phys_addr(ch7322);
    ret = devm_request_threaded_irq(&client.dev, client.irq, core::ptr::null_mut(),
    ch7322_irq,
    IRQF_ONESHOT | IRQF_TRIGGER_RISING,
    client.name, ch7322);
    if (ret)
    goto err_notifier;
// Unmask interrupt
    mutex_lock(&ch7322.mutex);
    ret = regmap_write(ch7322.regmap, CH7322_INTCTL, 0xff);
    mutex_unlock(&ch7322.mutex);
    if (ret)
    goto err_notifier;
    ret = cec_register_adapter(ch7322.cec, &client.dev);
    if (ret)
    goto err_notifier;
    dev_info(&client.dev, "device registered\n");
    return 0;
    err_notifier:
    if (notifier)
    cec_notifier_cec_adap_unregister(notifier, ch7322.cec);
    err_cec:
    cec_delete_adapter(ch7322.cec);
    err_mutex:
    mutex_destroy(&ch7322.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ch7322_remove(client: *mut i2c_client) {
    static void ch7322_remove(struct i2c_client *client)
    {
    struct ch7322 *ch7322 = i2c_get_clientdata(client);
// Mask interrupt
    mutex_lock(&ch7322.mutex);
    regmap_write(ch7322.regmap, CH7322_INTCTL, CH7322_INTCTL_INTPB);
    mutex_unlock(&ch7322.mutex);
    cec_unregister_adapter(ch7322.cec);
    mutex_destroy(&ch7322.mutex);
    dev_info(&client.dev, "device unregistered\n");
    }
    static const struct of_device_id ch7322_of_match[] = {
    { .compatible = "chrontel,ch7322", },
    {},
    };
    MODULE_DEVICE_TABLE(of, ch7322_of_match);
    static struct i2c_driver ch7322_i2c_driver = {
    .driver = {
    .name = "ch7322",
    .of_match_table = ch7322_of_match,
    },
    .probe		= ch7322_probe,
    .remove		= ch7322_remove,
    };
    module_i2c_driver(ch7322_i2c_driver);
    MODULE_DESCRIPTION("Chrontel CH7322 CEC Controller Driver");
    MODULE_AUTHOR("Jeff Chase <jnchase@google.com>");
    MODULE_LICENSE("GPL");
