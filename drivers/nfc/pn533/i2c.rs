//! Automatically rewritten from C to Rust
//! Source: drivers/nfc/pn533/i2c.c
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
// Driver for NXP PN533 NFC Chip - I2C transport layer
//
// Copyright (C) 2011 Instituto Nokia de Tecnologia
// Copyright (C) 2012-2013 Tieto Poland
// Copyright (C) 2016 HALE electronic
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pn533_i2c_phy {
    pub i2c_dev: *mut i2c_client,
    pub priv: *mut pn533,
    pub aborted: bool,
    pub /*: *mut int hard_fault;,
// < 0 if hardware error occurred (e.g. i2c err)
// and prevents normal operation.
//
}

#[no_mangle]
unsafe extern "C" fn pn533_i2c_send_ack(dev: *mut pn533, flags: gfp_t) -> c_int {
    static int pn533_i2c_send_ack(struct pn533 *dev, gfp_t flags)
    {
    struct pn533_i2c_phy *phy = dev.phy;
    struct i2c_client *client = phy.i2c_dev;
    static const u8 ack[6] = {0x00, 0x00, 0xff, 0x00, 0xff, 0x00};
// spec 6.2.1.3:  Preamble, SoPC (2), ACK Code (2), Postamble
    return i2c_master_send(client, ack, 6);
    }
    static int pn533_i2c_send_frame(struct pn533 *dev,
    struct sk_buff *out)
    {
    struct pn533_i2c_phy *phy = dev.phy;
    struct i2c_client *client = phy.i2c_dev;
    int rc;
    if (phy.hard_fault != 0)
    return phy.hard_fault;
    if (phy.priv == core::ptr::null_mut())
    phy.priv = dev;
    phy.aborted = false;
    print_hex_dump_debug("PN533_i2c TX: ", DUMP_PREFIX_NONE, 16, 1,
    out.data, out.len, false);
    rc = i2c_master_send(client, out.data, out.len);
    if (rc == -EREMOTEIO) { /* Retry, chip was in power down */
    usleep_range(6000, 10000);
    rc = i2c_master_send(client, out.data, out.len);
    }
    if (rc >= 0) {
    if (rc != out.len)
    rc = -EREMOTEIO;
    else
    rc = 0;
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn pn533_i2c_abort_cmd(dev: *mut pn533, flags: gfp_t) {
    static void pn533_i2c_abort_cmd(struct pn533 *dev, gfp_t flags)
    {
    struct pn533_i2c_phy *phy = dev.phy;
    phy.aborted = true;
// An ack will cancel the last issued command
    pn533_i2c_send_ack(dev, flags);
// schedule cmd_complete_work to finish current command execution
    pn533_recv_frame(phy.priv, core::ptr::null_mut(), -ENOENT);
    }
#[no_mangle]
unsafe extern "C" fn pn533_i2c_read(phy: *mut pn533_i2c_phy, skb: *mut sk_buff) -> c_int {
    static int pn533_i2c_read(struct pn533_i2c_phy *phy, struct sk_buff **skb)
    {
    struct i2c_client *client = phy.i2c_dev;
    int len = PN533_EXT_FRAME_HEADER_LEN +
    PN533_STD_FRAME_MAX_PAYLOAD_LEN +
    PN533_STD_FRAME_TAIL_LEN + 1;
    int r;
// skb = alloc_skb(len, GFP_KERNEL);
    if (*skb == core::ptr::null_mut())
    return -ENOMEM;
    r = i2c_master_recv(client, skb_put(*skb, len), len);
    if (r != len) {
    nfc_err(&client.dev, "cannot read. r=%d len=%d\n", r, len);
    kfree_skb(*skb);
    return -EREMOTEIO;
    }
    if (!((*skb).data[0] & 0x01)) {
    nfc_err(&client.dev, "READY flag not set");
    kfree_skb(*skb);
    return -EBUSY;
    }
// remove READY byte
    skb_pull(*skb, 1);
// trim to frame size
    skb_trim(*skb, phy.priv.ops.rx_frame_size((*skb).data));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pn533_i2c_irq_thread_fn(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t pn533_i2c_irq_thread_fn(int irq, void *data)
    {
    struct pn533_i2c_phy *phy = data;
    struct sk_buff *skb = core::ptr::null_mut();
    int r;
    if (!phy || irq != phy.i2c_dev.irq) {
    WARN_ON_ONCE(1);
    return IRQ_NONE;
    }
    if (phy.hard_fault != 0)
    return IRQ_HANDLED;
    r = pn533_i2c_read(phy, &skb);
    if (r == -EREMOTEIO) {
    phy.hard_fault = r;
    pn533_recv_frame(phy.priv, core::ptr::null_mut(), -EREMOTEIO);
    return IRQ_HANDLED;
    } else if ((r == -ENOMEM) || (r == -EBADMSG) || (r == -EBUSY)) {
    return IRQ_HANDLED;
    }
    if (!phy.aborted)
    pn533_recv_frame(phy.priv, skb, 0);
    return IRQ_HANDLED;
    }
    static const struct pn533_phy_ops i2c_phy_ops = {
    .send_frame = pn533_i2c_send_frame,
    .send_ack = pn533_i2c_send_ack,
    .abort_cmd = pn533_i2c_abort_cmd,
    };
#[no_mangle]
unsafe extern "C" fn pn533_i2c_probe(client: *mut i2c_client) -> c_int {
    static int pn533_i2c_probe(struct i2c_client *client)
    {
    struct pn533_i2c_phy *phy;
    struct pn533 *priv;
    let mut r: c_int = 0;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C)) {
    nfc_err(&client.dev, "Need I2C_FUNC_I2C\n");
    return -ENODEV;
    }
    phy = devm_kzalloc(&client.dev, sizeof(struct pn533_i2c_phy),
    GFP_KERNEL);
    if (!phy)
    return -ENOMEM;
    phy.i2c_dev = client;
    i2c_set_clientdata(client, phy);
    priv = pn53x_common_init(PN533_DEVICE_PN532,
    PN533_PROTO_REQ_ACK_RESP,
    phy, &i2c_phy_ops, core::ptr::null_mut(),
    &phy.i2c_dev.dev);
    if (IS_ERR(priv))
    return PTR_ERR(priv);
    phy.priv = priv;
    r = pn532_i2c_nfc_alloc(priv, PN533_NO_TYPE_B_PROTOCOLS, &client.dev);
    if (r)
    goto nfc_alloc_err;
    r = request_threaded_irq(client.irq, core::ptr::null_mut(), pn533_i2c_irq_thread_fn,
    IRQF_TRIGGER_FALLING |
    IRQF_SHARED | IRQF_ONESHOT,
    PN533_I2C_DRIVER_NAME, phy);
    if (r < 0) {
    nfc_err(&client.dev, "Unable to register IRQ handler\n");
    goto irq_rqst_err;
    }
    r = pn533_finalize_setup(priv);
    if (r)
    goto fn_setup_err;
    r = nfc_register_device(priv.nfc_dev);
    if (r)
    goto fn_setup_err;
    return r;
    fn_setup_err:
    free_irq(client.irq, phy);
    irq_rqst_err:
    nfc_free_device(priv.nfc_dev);
    nfc_alloc_err:
    pn53x_common_clean(phy.priv);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn pn533_i2c_remove(client: *mut i2c_client) {
    static void pn533_i2c_remove(struct i2c_client *client)
    {
    struct pn533_i2c_phy *phy = i2c_get_clientdata(client);
    free_irq(client.irq, phy);
    pn53x_unregister_nfc(phy.priv);
    pn53x_common_clean(phy.priv);
    }
    static const struct of_device_id of_pn533_i2c_match[] = {
    { .compatible = "nxp,pn532" },
//
// NOTE: The use of the compatibles with the trailing "...-i2c" is
// deprecated and will be removed.
//
    { .compatible = "nxp,pn533-i2c" },
    { .compatible = "nxp,pn532-i2c" },
    { }
    };
    MODULE_DEVICE_TABLE(of, of_pn533_i2c_match);
    static const struct i2c_device_id pn533_i2c_id_table[] = {
    { .name = PN533_I2C_DRIVER_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, pn533_i2c_id_table);
    static struct i2c_driver pn533_i2c_driver = {
    .driver = {
    .name = PN533_I2C_DRIVER_NAME,
    .of_match_table = of_match_ptr(of_pn533_i2c_match),
    },
    .probe = pn533_i2c_probe,
    .id_table = pn533_i2c_id_table,
    .remove = pn533_i2c_remove,
    };
    module_i2c_driver(pn533_i2c_driver);
    MODULE_AUTHOR("Michael Thalmeier <michael.thalmeier@hale.at>");
    MODULE_DESCRIPTION("PN533 I2C driver ver " VERSION);
    MODULE_VERSION(VERSION);
    MODULE_LICENSE("GPL");
