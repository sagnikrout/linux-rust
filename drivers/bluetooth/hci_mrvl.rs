//! Automatically rewritten from C to Rust
//! Source: drivers/bluetooth/hci_mrvl.c
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
// Bluetooth HCI UART driver for marvell devices
//
// Copyright (C) 2016  Marvell International Ltd.
// Copyright (C) 2016  Intel Corporation
//

pub const HCI_FW_REQ_PKT: c_uint = 0xA5;
pub const HCI_CHIP_VER_PKT: c_uint = 0xAA;
pub const MRVL_ACK: c_uint = 0x5A;
pub const MRVL_NAK: c_uint = 0xBF;
pub const MRVL_RAW_DATA: c_uint = 0x1F;
pub const MRVL_SET_BAUDRATE: c_uint = 0xFC09;
    enum {
    STATE_CHIP_VER_PENDING,
    STATE_FW_REQ_PENDING,
    STATE_FW_LOADED,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrvl_data {
    pub rx_skb: *mut sk_buff,
    pub txq: sk_buff_head,
    pub rawq: sk_buff_head,
    pub flags: c_ulong,
    pub tx_len: c_uint,
    pub rev: u8 id,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrvl_serdev {
    pub hu: hci_uart,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_mrvl_pkt {
    pub lhs: __le16,
    pub rhs: __le16,
    pub __packed: },
pub const HCI_MRVL_PKT_SIZE: c_int = 4;
#[no_mangle]
unsafe extern "C" fn mrvl_open(hu: *mut hci_uart) -> c_int {
    static int mrvl_open(struct hci_uart *hu)
    {
    pub mrvl: *mut mrvl_data,
    pub ret: c_int,
    pub hu): BT_DBG("hu %p",,
    if (!hci_uart_has_flow_control(hu))
    pub -EOPNOTSUPP: return,
    pub kzalloc_obj(*mrvl): *mut mrvl =,
    if (!mrvl)
    pub -ENOMEM: return,
    pub &mrvl->flags): set_bit(STATE_CHIP_VER_PENDING,,
    pub mrvl: hu->priv =,
    if (hu.serdev) {
    pub serdev_device_open(hu->serdev): ret =,
    if (ret)
    pub err: goto,
    }
    pub 0: return,
    err:
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn mrvl_close(hu: *mut hci_uart) -> c_int {
    static int mrvl_close(struct hci_uart *hu)
    {
    pub hu->priv: *mut *mut mrvl_data mrvl =,
    pub hu): BT_DBG("hu %p",,
    if (hu.serdev)
    pub NULL: hu->priv =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn mrvl_flush(hu: *mut hci_uart) -> c_int {
    static int mrvl_flush(struct hci_uart *hu)
    {
    pub hu->priv: *mut *mut mrvl_data mrvl =,
    pub hu): BT_DBG("hu %p",,
    pub 0: return,
    }
    static struct sk_buff *mrvl_dequeue(struct hci_uart *hu)
    {
    pub hu->priv: *mut *mut mrvl_data mrvl =,
    pub skb: *mut sk_buff,
    pub skb_dequeue(&mrvl->txq): skb =,
    if (!skb) {
// Any raw data ?
    pub skb_dequeue(&mrvl->rawq): skb =,
    } else {
// Prepend skb with frame type
    pub 1): memcpy(skb_push(skb, 1), &bt_cb(skb)->pkt_type,,
    }
    pub skb: return,
    }
#[no_mangle]
unsafe extern "C" fn mrvl_enqueue(hu: *mut hci_uart, skb: *mut sk_buff) -> c_int {
    static int mrvl_enqueue(struct hci_uart *hu, struct sk_buff *skb)
    {
    pub hu->priv: *mut *mut mrvl_data mrvl =,
    pub skb): skb_queue_tail(&mrvl->txq,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn mrvl_send_ack(hu: *mut hci_uart, type: c_uchar) {
    static void mrvl_send_ack(struct hci_uart *hu, unsigned char type)
    {
    pub hu->priv: *mut *mut mrvl_data mrvl =,
    pub skb: *mut sk_buff,
// No H4 payload, only 1 byte header
    pub GFP_ATOMIC): skb = bt_skb_alloc(0,,
    if (!skb) {
    pub packet"): bt_dev_err(hu->hdev, "Unable to alloc ack/nak,
    }
    pub type: hci_skb_pkt_type(skb) =,
    pub skb): skb_queue_tail(&mrvl->txq,,
    }
#[no_mangle]
unsafe extern "C" fn mrvl_recv_fw_req(hdev: *mut hci_dev, skb: *mut sk_buff) -> c_int {
    static int mrvl_recv_fw_req(struct hci_dev *hdev, struct sk_buff *skb)
    {
    pub )skb->data: *mut *mut hci_mrvl_pkt pkt = (void,
    pub hci_get_drvdata(hdev): *mut *mut hci_uart hu =,
    pub hu->priv: *mut *mut mrvl_data mrvl =,
    pub 0: int ret =,
    if ((pkt.lhs ^ pkt.rhs) != 0xffff) {
    pub header"): bt_dev_err(hdev, "Corrupted mrvl,
    pub MRVL_NAK): mrvl_send_ack(hu,,
    pub -EINVAL: ret =,
    pub done: goto,
    }
    pub MRVL_ACK): mrvl_send_ack(hu,,
    if (!test_bit(STATE_FW_REQ_PENDING, &mrvl.flags)) {
    pub request"): bt_dev_err(hdev, "Received unexpected firmware,
    pub -EINVAL: ret =,
    pub done: goto,
    }
    pub le16_to_cpu(pkt->lhs): mrvl->tx_len =,
    pub &mrvl->flags): clear_bit(STATE_FW_REQ_PENDING,,
    pub STATE_FW_REQ_PENDING): wake_up_bit(&mrvl->flags,,
    done:
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn mrvl_recv_chip_ver(hdev: *mut hci_dev, skb: *mut sk_buff) -> c_int {
    static int mrvl_recv_chip_ver(struct hci_dev *hdev, struct sk_buff *skb)
    {
    pub )skb->data: *mut *mut hci_mrvl_pkt pkt = (void,
    pub hci_get_drvdata(hdev): *mut *mut hci_uart hu =,
    pub hu->priv: *mut *mut mrvl_data mrvl =,
    pub le16_to_cpu(pkt->lhs): u16 version =,
    pub 0: int ret =,
    if ((pkt.lhs ^ pkt.rhs) != 0xffff) {
    pub header"): bt_dev_err(hdev, "Corrupted mrvl,
    pub MRVL_NAK): mrvl_send_ack(hu,,
    pub -EINVAL: ret =,
    pub done: goto,
    }
    pub MRVL_ACK): mrvl_send_ack(hu,,
    if (!test_bit(STATE_CHIP_VER_PENDING, &mrvl.flags)) {
    pub version"): bt_dev_err(hdev, "Received unexpected chip,
    pub done: goto,
    }
    pub version: mrvl->id =,
    pub 8: mrvl->rev = version >>,
    pub mrvl->rev): bt_dev_info(hdev, "Controller id = %x, rev = %x", mrvl->id,,
    pub &mrvl->flags): clear_bit(STATE_CHIP_VER_PENDING,,
    pub STATE_CHIP_VER_PENDING): wake_up_bit(&mrvl->flags,,
    done:
    pub ret: return,
    }

    .type = HCI_CHIP_VER_PKT, \
    .hlen = HCI_MRVL_PKT_SIZE, \
    .loff = 0, \
    .lsize = 0, \
    .maxlen = HCI_MRVL_PKT_SIZE

    .type = HCI_FW_REQ_PKT, \
    .hlen = HCI_MRVL_PKT_SIZE, \
    .loff = 0, \
    .lsize = 0, \
    .maxlen = HCI_MRVL_PKT_SIZE
    static const struct h4_recv_pkt mrvl_recv_pkts[] = {
    { H4_RECV_ACL,       .recv = hci_recv_frame     },
    { H4_RECV_SCO,       .recv = hci_recv_frame     },
    { H4_RECV_EVENT,     .recv = hci_recv_frame     },
    { HCI_RECV_FW_REQ,   .recv = mrvl_recv_fw_req   },
    { HCI_RECV_CHIP_VER, .recv = mrvl_recv_chip_ver },
}

#[no_mangle]
unsafe extern "C" fn mrvl_recv(hu: *mut hci_uart, data: *const c_void, count: c_int) -> c_int {
    static int mrvl_recv(struct hci_uart *hu, const void *data, int count)
    {
    struct mrvl_data *mrvl = hu.priv;
    if (!test_bit(HCI_UART_REGISTERED, &hu.flags))
    return -EUNATCH;
// We might receive some noise when there is no firmware loaded. Therefore,
// we drop data if the firmware is not loaded yet and if there is no fw load
// request pending.
//
    if (!test_bit(STATE_FW_REQ_PENDING, &mrvl.flags) &&
    !test_bit(STATE_FW_LOADED, &mrvl.flags))
    return count;
    mrvl.rx_skb = h4_recv_buf(hu, mrvl.rx_skb, data, count,
    mrvl_recv_pkts,
    ARRAY_SIZE(mrvl_recv_pkts));
    if (IS_ERR(mrvl.rx_skb)) {
    let mut err: c_int = PTR_ERR(mrvl.rx_skb);
    bt_dev_err(hu.hdev, "Frame reassembly failed (%d)", err);
    mrvl.rx_skb = core::ptr::null_mut();
    return err;
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn mrvl_load_firmware(hdev: *mut hci_dev, name: *const c_char) -> c_int {
    static int mrvl_load_firmware(struct hci_dev *hdev, const char *name)
    {
    struct hci_uart *hu = hci_get_drvdata(hdev);
    struct mrvl_data *mrvl = hu.priv;
    const struct firmware *fw = core::ptr::null_mut();
    const u8 *fw_ptr, *fw_max;
    int err;
    err = request_firmware(&fw, name, &hdev.dev);
    if (err < 0) {
    bt_dev_err(hdev, "Failed to load firmware file %s", name);
    return err;
    }
    fw_ptr = fw.data;
    fw_max = fw.data + fw.size;
    bt_dev_info(hdev, "Loading %s", name);
    set_bit(STATE_FW_REQ_PENDING, &mrvl.flags);
    while (fw_ptr <= fw_max) {
    struct sk_buff *skb;
// Controller drives the firmware load by sending firmware
// request packets containing the expected fragment size.
//
    err = wait_on_bit_timeout(&mrvl.flags, STATE_FW_REQ_PENDING,
    TASK_INTERRUPTIBLE,
    msecs_to_jiffies(2000));
    if (err == -EINTR) {
    bt_dev_err(hdev, "Firmware load interrupted");
    break;
    } else if (err) {
    bt_dev_err(hdev, "Firmware request timeout");
    err = -ETIMEDOUT;
    break;
    }
    bt_dev_dbg(hdev, "Firmware request, expecting %d bytes",
    mrvl.tx_len);
    if (fw_ptr == fw_max) {
// Controller requests a null size once firmware is
// fully loaded. If controller expects more data, there
// is an issue.
//
    if (!mrvl.tx_len) {
    bt_dev_info(hdev, "Firmware loading complete");
    } else {
    bt_dev_err(hdev, "Firmware loading failure");
    err = -EINVAL;
    }
    break;
    }
    if (fw_ptr + mrvl.tx_len > fw_max) {
    mrvl.tx_len = fw_max - fw_ptr;
    bt_dev_dbg(hdev, "Adjusting tx_len to %d",
    mrvl.tx_len);
    }
    skb = bt_skb_alloc(mrvl.tx_len, GFP_KERNEL);
    if (!skb) {
    bt_dev_err(hdev, "Failed to alloc mem for FW packet");
    err = -ENOMEM;
    break;
    }
    bt_cb(skb).pkt_type = MRVL_RAW_DATA;
    skb_put_data(skb, fw_ptr, mrvl.tx_len);
    fw_ptr += mrvl.tx_len;
    set_bit(STATE_FW_REQ_PENDING, &mrvl.flags);
    skb_queue_tail(&mrvl.rawq, skb);
    hci_uart_tx_wakeup(hu);
    }
    release_firmware(fw);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mrvl_setup(hu: *mut hci_uart) -> c_int {
    static int mrvl_setup(struct hci_uart *hu)
    {
    int err;
    struct mrvl_data *mrvl = hu.priv;
    hci_uart_set_flow_control(hu, true);
    err = mrvl_load_firmware(hu.hdev, "mrvl/helper_uart_3000000.bin");
    if (err) {
    bt_dev_err(hu.hdev, "Unable to download firmware helper");
    return -EINVAL;
    }
// Let the final ack go out before switching the baudrate
    hci_uart_wait_until_sent(hu);
    if (hu.serdev)
    serdev_device_set_baudrate(hu.serdev, hu.oper_speed);
    else
    hci_uart_set_baudrate(hu, hu.oper_speed);
    hci_uart_set_flow_control(hu, false);
    err = mrvl_load_firmware(hu.hdev, "mrvl/uart8897_bt.bin");
    if (err)
    return err;
    set_bit(STATE_FW_LOADED, &mrvl.flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mrvl_set_baudrate(hu: *mut hci_uart, speed: c_uint) -> c_int {
    static int mrvl_set_baudrate(struct hci_uart *hu, unsigned int speed)
    {
    int err;
    struct mrvl_data *mrvl = hu.priv;
    let mut speed_le: __le32 = cpu_to_le32(speed);
// The firmware might be loaded by the Wifi driver over SDIO. We wait
// up to 10s for the CTS to go up. Afterward, we know that the firmware
// is ready.
//
    err = serdev_device_wait_for_cts(hu.serdev, true, 10000);
    if (err) {
    bt_dev_err(hu.hdev, "Wait for CTS failed with %d\n", err);
    return err;
    }
    set_bit(STATE_FW_LOADED, &mrvl.flags);
    err = __hci_cmd_sync_status(hu.hdev, MRVL_SET_BAUDRATE,
    sizeof(speed_le), &speed_le,
    HCI_INIT_TIMEOUT);
    if (err) {
    bt_dev_err(hu.hdev, "send command failed: %d", err);
    return err;
    }
    serdev_device_set_baudrate(hu.serdev, speed);
// We forcefully have to send a command to the bluetooth module so that
// the driver detects it after a baudrate change. This is foreseen by
// hci_serdev by setting HCI_UART_VND_DETECT which then causes a dummy
// local version read.
//
    set_bit(HCI_UART_VND_DETECT, &hu.hdev_flags);
    return 0;
    }
    static const struct hci_uart_proto mrvl_proto_8897 = {
    .id		= HCI_UART_MRVL,
    .name		= "Marvell",
    .init_speed	= 115200,
    .oper_speed	= 3000000,
    .open		= mrvl_open,
    .close		= mrvl_close,
    .flush		= mrvl_flush,
    .setup		= mrvl_setup,
    .recv		= mrvl_recv,
    .enqueue	= mrvl_enqueue,
    .dequeue	= mrvl_dequeue,
    };
    static const struct hci_uart_proto mrvl_proto_8997 = {
    .id		= HCI_UART_MRVL,
    .name		= "Marvell 8997",
    .init_speed	= 115200,
    .oper_speed	= 3000000,
    .open		= mrvl_open,
    .close		= mrvl_close,
    .flush		= mrvl_flush,
    .set_baudrate   = mrvl_set_baudrate,
    .recv		= mrvl_recv,
    .enqueue	= mrvl_enqueue,
    .dequeue	= mrvl_dequeue,
    };
#[no_mangle]
unsafe extern "C" fn mrvl_serdev_probe(serdev: *mut serdev_device) -> c_int {
    static int mrvl_serdev_probe(struct serdev_device *serdev)
    {
    struct mrvl_serdev *mrvldev;
    const struct hci_uart_proto *mrvl_proto = device_get_match_data(&serdev.dev);
    mrvldev = devm_kzalloc(&serdev.dev, sizeof(*mrvldev), GFP_KERNEL);
    if (!mrvldev)
    return -ENOMEM;
    mrvldev.hu.oper_speed = mrvl_proto.oper_speed;
    if (mrvl_proto.set_baudrate)
    of_property_read_u32(serdev.dev.of_node, "max-speed", &mrvldev.hu.oper_speed);
    mrvldev.hu.serdev = serdev;
    serdev_device_set_drvdata(serdev, mrvldev);
    return hci_uart_register_device(&mrvldev.hu, mrvl_proto);
    }
#[no_mangle]
unsafe extern "C" fn mrvl_serdev_remove(serdev: *mut serdev_device) {
    static void mrvl_serdev_remove(struct serdev_device *serdev)
    {
    struct mrvl_serdev *mrvldev = serdev_device_get_drvdata(serdev);
    hci_uart_unregister_device(&mrvldev.hu);
    }
    static const struct of_device_id __maybe_unused mrvl_bluetooth_of_match[] = {
    { .compatible = "mrvl,88w8897", .data = &mrvl_proto_8897},
    { .compatible = "mrvl,88w8997", .data = &mrvl_proto_8997},
    { },
    };
    MODULE_DEVICE_TABLE(of, mrvl_bluetooth_of_match);
    static struct serdev_device_driver mrvl_serdev_driver = {
    .probe = mrvl_serdev_probe,
    .remove = mrvl_serdev_remove,
    .driver = {
    .name = "hci_uart_mrvl",
    .of_match_table = of_match_ptr(mrvl_bluetooth_of_match),
    },
    };
#[no_mangle]
pub unsafe extern "C" fn mrvl_init() -> int __init {
    int __init mrvl_init(void)
    {
    serdev_device_driver_register(&mrvl_serdev_driver);
    return hci_uart_register_proto(&mrvl_proto_8897);
    }
#[no_mangle]
pub unsafe extern "C" fn mrvl_deinit() -> int __exit {
    int __exit mrvl_deinit(void)
    {
    serdev_device_driver_unregister(&mrvl_serdev_driver);
    return hci_uart_unregister_proto(&mrvl_proto_8897);
    }
