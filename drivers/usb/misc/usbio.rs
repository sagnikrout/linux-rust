//! Automatically rewritten from C to Rust
//! Source: drivers/usb/misc/usbio.c
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
// Intel USBIO Bridge driver
//
// Copyright (c) 2025 Intel Corporation.
// Copyright (c) 2025 Red Hat, Inc.
//

//
// USBIO Bridge Protocol Definitions
//
// USBIO Control Commands
pub const USBIO_CTRLCMD_PROTVER: c_int = 0;
pub const USBIO_CTRLCMD_FWVER: c_int = 1;
pub const USBIO_CTRLCMD_HS: c_int = 2;
pub const USBIO_CTRLCMD_ENUMGPIO: c_int = 16;
pub const USBIO_CTRLCMD_ENUMI2C: c_int = 17;
// USBIO Packet Flags

pub const USBIO_CTRLXFER_TIMEOUT: c_int = 0;
pub const USBIO_BULKXFER_TIMEOUT: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbio_protver {
    pub ver: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbio_fwver {
    pub major: u8,
    pub minor: u8,
    pub patch: __le16,
    pub build: __le16,
    pub __packed: },
//
// USBIO Bridge Device Definitions
//
// struct usbio_device - the usb device exposing IOs
//
// @dev: the device in the usb interface
// @udev: the detected usb device
// @intf: the usb interface
// @quirks: quirks
// @ctrl_mutex: protects ctrl_buf
// @ctrl_pipe: the control transfer pipe
// @ctrlbuf_len: the size of the control transfer pipe
// @ctrlbuf: the buffer used for control transfers
// @bulk_mutex: protects tx_buf, rx_buf and split bulk-transfers getting interrupted
// @tx_pipe: the bulk out pipe
// @txbuf_len: the size of the bulk out pipe
// @txbuf: the buffer used for bulk out transfers
// @rx_pipe: the bulk in pipe
// @rxbuf_len: the size of the bulk in pipe
// @rxdat_len: the data length at rx buffer
// @rxbuf: the buffer used for bulk in transfers
// @urb: the urb to read bulk pipe
// @done: completion object as request is done
// @cli_list: device's client list
// @nr_gpio_banks: Number of GPIO banks
// @gpios: GPIO bank descriptors
// @nr_gpio_banks: Number of I2C busses
// @gpios: I2C bank descriptors
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbio_device {
    pub dev: *mut device,
    pub udev: *mut usb_device,
    pub intf: *mut usb_interface,
    pub quirks: c_ulong,
    pub ctrl_mutex: mutex,
    pub ctrl_pipe: c_uint,
    pub ctrlbuf_len: u16,
    pub ctrlbuf: *mut c_void,
    pub bulk_mutex: mutex,
    pub tx_pipe: c_uint,
    pub txbuf_len: u16,
    pub txbuf: *mut c_void,
    pub rx_pipe: c_uint,
    pub rxbuf_len: u16,
    pub rxdat_len: u16,
    pub rxbuf: *mut c_void,
    pub urb: *mut urb,
    pub done: completion,
    pub cli_list: list_head,
    pub nr_gpio_banks: c_uint,
    pub gpios: [usbio_gpio_bank_desc; USBIO_MAX_GPIOBANKS],
    pub nr_i2c_buses: c_uint,
    pub i2cs: [usbio_i2c_bus_desc; USBIO_MAX_I2CBUSES],
}

//
// struct usbio_client - represents a usbio client
//
// @auxdev: auxiliary device object
// @mutex: protects @bridge
// @bridge: usbio bridge who service the client
// @link: usbio bridge clients list member
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbio_client {
    pub auxdev: auxiliary_device,
    pub mutex: mutex,
    pub bridge: *mut usbio_device,
    pub link: list_head,
}

    static int usbio_ctrl_msg(struct usbio_device *usbio, u8 type, u8 cmd,
    const void *obuf, u16 obuf_len, void *ibuf, u16 ibuf_len)
    {
    let mut request: u8 = USB_TYPE_VENDOR | USB_RECIP_DEVICE;
    struct usbio_ctrl_packet *cpkt;
    unsigned int pipe;
    u16 cpkt_len;
    int ret;
    lockdep_assert_held(&usbio.ctrl_mutex);
    if ((obuf_len > (usbio.ctrlbuf_len - sizeof(*cpkt))) ||
    (ibuf_len > (usbio.ctrlbuf_len - sizeof(*cpkt))))
    return -EMSGSIZE;
// Prepare Control Packet Header
    cpkt = usbio.ctrlbuf;
    cpkt.header.type = type;
    cpkt.header.cmd = cmd;
    if (type == USBIO_PKTTYPE_CTRL || ibuf_len)
    cpkt.header.flags = USBIO_PKTFLAGS_REQRESP;
    else
    cpkt.header.flags = USBIO_PKTFLAG_CMP;
    cpkt.len = obuf_len;
// Copy the data
    memcpy(cpkt.data, obuf, obuf_len);
    pipe = usb_sndctrlpipe(usbio.udev, usbio.ctrl_pipe);
    cpkt_len = sizeof(*cpkt) + obuf_len;
    ret = usb_control_msg(usbio.udev, pipe, 0, request | USB_DIR_OUT, 0, 0,
    cpkt, cpkt_len, USBIO_CTRLXFER_TIMEOUT);
    dev_dbg(usbio.dev, "control out %d hdr %*phN data %*phN\n", ret,
    (int)sizeof(*cpkt), cpkt, (int)cpkt.len, cpkt.data);
    if (ret != cpkt_len) {
    dev_err(usbio.dev, "USB control out failed: %d\n", ret);
    return (ret < 0) ? ret : -EPROTO;
    }
    if (!(cpkt.header.flags & USBIO_PKTFLAG_ACK))
    return 0;
    pipe = usb_rcvctrlpipe(usbio.udev, usbio.ctrl_pipe);
    cpkt_len = sizeof(*cpkt) + ibuf_len;
    ret = usb_control_msg(usbio.udev, pipe, 0, request | USB_DIR_IN, 0, 0,
    cpkt, cpkt_len, USBIO_CTRLXFER_TIMEOUT);
    dev_dbg(usbio.dev, "control in %d hdr %*phN data %*phN\n", ret,
    (int)sizeof(*cpkt), cpkt, (int)cpkt.len, cpkt.data);
    if (ret < sizeof(*cpkt)) {
    dev_err(usbio.dev, "USB control in failed: %d\n", ret);
    return (ret < 0) ? ret : -EPROTO;
    }
    if (cpkt.header.type != type || cpkt.header.cmd != cmd ||
    !(cpkt.header.flags & USBIO_PKTFLAG_RSP)) {
    dev_err(usbio.dev, "Unexpected reply type: %u, cmd: %u, flags: %u\n",
    cpkt.header.type, cpkt.header.cmd, cpkt.header.flags);
    return -EPROTO;
    }
    if (cpkt.header.flags & USBIO_PKTFLAG_ERR)
    return -EREMOTEIO;
    if (ibuf_len < cpkt.len)
    return -ENOSPC;
    memcpy(ibuf, cpkt.data, cpkt.len);
    return cpkt.len;
    }
    int usbio_control_msg(struct auxiliary_device *adev, u8 type, u8 cmd,
    const void *obuf, u16 obuf_len, void *ibuf, u16 ibuf_len)
    {
    struct usbio_client *client = adev_to_client(adev);
    struct usbio_device *usbio;
    int ret;
    guard(mutex)(&client.mutex);
    usbio = client.bridge;
    if (!usbio)
    return -ENODEV; /* Disconnected */
    ret = usb_autopm_get_interface(usbio.intf);
    if (ret)
    return ret;
    mutex_lock(&usbio.ctrl_mutex);
    ret = usbio_ctrl_msg(client.bridge, type, cmd, obuf, obuf_len, ibuf, ibuf_len);
    mutex_unlock(&usbio.ctrl_mutex);
    usb_autopm_put_interface(usbio.intf);
    return ret;
    }
    EXPORT_SYMBOL_NS_GPL(usbio_control_msg, "USBIO");
#[no_mangle]
unsafe extern "C" fn usbio_bulk_recv(urb: *mut urb) {
    static void usbio_bulk_recv(struct urb *urb)
    {
    struct usbio_bulk_packet *bpkt = urb.transfer_buffer;
    struct usbio_device *usbio = urb.context;
    if (!urb.status) {
    if (bpkt.header.flags & USBIO_PKTFLAG_RSP) {
    usbio.rxdat_len = urb.actual_length;
    complete(&usbio.done);
    }
    } else if (urb.status != -ENOENT) {
    dev_err(usbio.dev, "Bulk in error %d\n", urb.status);
    }
    usb_submit_urb(usbio.urb, GFP_ATOMIC);
    }
    int usbio_bulk_msg(struct auxiliary_device *adev, u8 type, u8 cmd, bool last,
    const void *obuf, u16 obuf_len, void *ibuf, u16 ibuf_len)
    {
    struct usbio_client *client = adev_to_client(adev);
    struct usbio_device *usbio = client.bridge;
    struct usbio_bulk_packet *bpkt;
    int ret, act = 0;
    u16 bpkt_len;
    lockdep_assert_held(&client.mutex);
    lockdep_assert_held(&usbio.bulk_mutex);
    if ((obuf_len > (usbio.txbuf_len - sizeof(*bpkt))) ||
    (ibuf_len > (usbio.rxbuf_len - sizeof(*bpkt))))
    return -EMSGSIZE;
    if (ibuf_len)
    reinit_completion(&usbio.done);
// If no data to send, skip to read
    if (!obuf_len)
    goto read;
// Prepare Bulk Packet Header
    bpkt = usbio.txbuf;
    bpkt.header.type = type;
    bpkt.header.cmd = cmd;
    if (!last)
    bpkt.header.flags = 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: ibuf_len) -> else {
    else if (ibuf_len)
    bpkt.header.flags = USBIO_PKTFLAGS_REQRESP;
    else
    bpkt.header.flags = USBIO_PKTFLAG_CMP;
    bpkt.len = cpu_to_le16(obuf_len);
// Copy the data
    memcpy(bpkt.data, obuf, obuf_len);
    bpkt_len = sizeof(*bpkt) + obuf_len;
    ret = usb_bulk_msg(usbio.udev, usbio.tx_pipe, bpkt, bpkt_len, &act,
    USBIO_BULKXFER_TIMEOUT);
    dev_dbg(usbio.dev, "bulk out %d hdr %*phN data %*phN\n", act,
    (int)sizeof(*bpkt), bpkt, obuf_len, bpkt.data);
    if (ret || act != bpkt_len) {
    dev_err(usbio.dev, "Bulk out failed: %d\n", ret);
    return ret ?: -EPROTO;
    }
    if (!(bpkt.header.flags & USBIO_PKTFLAG_ACK))
    return obuf_len;
    read:
    ret = wait_for_completion_timeout(&usbio.done, USBIO_BULKXFER_TIMEOUT);
    if (ret <= 0) {
    dev_err(usbio.dev, "Bulk in wait failed: %d\n", ret);
    return ret ?: -ETIMEDOUT;
    }
    act = usbio.rxdat_len;
    bpkt = usbio.rxbuf;
    bpkt_len = le16_to_cpu(bpkt.len);
    dev_dbg(usbio.dev, "bulk in %d hdr %*phN data %*phN\n", act,
    (int)sizeof(*bpkt), bpkt, bpkt_len, bpkt.data);
//
// Unsupported bulk commands get only an usbio_packet_header with
// the error flag set as reply. Return -EPIPE for this case.
//
    if (act == sizeof(struct usbio_packet_header) &&
    (bpkt.header.flags & USBIO_PKTFLAG_ERR))
    return -EPIPE;
    if (act < sizeof(*bpkt)) {
    dev_err(usbio.dev, "Bulk in short read: %d\n", act);
    return -EPROTO;
    }
    if (bpkt.header.type != type || bpkt.header.cmd != cmd ||
    !(bpkt.header.flags & USBIO_PKTFLAG_RSP)) {
    dev_err(usbio.dev,
    "Unexpected bulk in type 0x%02x cmd 0x%02x flags 0x%02x\n",
    bpkt.header.type, bpkt.header.cmd, bpkt.header.flags);
    return -EPROTO;
    }
    if (bpkt.header.flags & USBIO_PKTFLAG_ERR)
    return -EREMOTEIO;
    if (ibuf_len < bpkt_len)
    return -ENOSPC;
// The device must not claim more payload than it actually sent.
    if (bpkt_len > act - sizeof(*bpkt))
    return -EPROTO;
    memcpy(ibuf, bpkt.data, bpkt_len);
    return bpkt_len;
    }
    EXPORT_SYMBOL_NS_GPL(usbio_bulk_msg, "USBIO");
#[no_mangle]
pub unsafe extern "C" fn usbio_acquire(adev: *mut auxiliary_device) -> c_int {
    int usbio_acquire(struct auxiliary_device *adev)
    {
    struct usbio_client *client = adev_to_client(adev);
    struct usbio_device *usbio;
    int ret;
    mutex_lock(&client.mutex);
    usbio = client.bridge;
    if (!usbio) {
    ret = -ENODEV; /* Disconnected */
    goto err_unlock;
    }
    ret = usb_autopm_get_interface(usbio.intf);
    if (ret)
    goto err_unlock;
    mutex_lock(&usbio.bulk_mutex);
// Leave client locked until release to avoid abba deadlock issues
    return 0;
    err_unlock:
    mutex_unlock(&client.mutex);
    return ret;
    }
    EXPORT_SYMBOL_NS_GPL(usbio_acquire, "USBIO");
#[no_mangle]
pub unsafe extern "C" fn usbio_release(adev: *mut auxiliary_device) {
    void usbio_release(struct auxiliary_device *adev)
    {
    struct usbio_client *client = adev_to_client(adev);
    struct usbio_device *usbio = client.bridge;
    lockdep_assert_held(&client.mutex);
    mutex_unlock(&usbio.bulk_mutex);
    usb_autopm_put_interface(usbio.intf);
    mutex_unlock(&client.mutex);
    }
    EXPORT_SYMBOL_NS_GPL(usbio_release, "USBIO");
#[no_mangle]
pub unsafe extern "C" fn usbio_get_txrxbuf_len(adev: *mut auxiliary_device, txbuf_len: *mut u16, rxbuf_len: *mut u16) {
    void usbio_get_txrxbuf_len(struct auxiliary_device *adev, u16 *txbuf_len, u16 *rxbuf_len)
    {
    struct usbio_client *client = adev_to_client(adev);
    struct usbio_device *usbio;
    guard(mutex)(&client.mutex);
    usbio = client.bridge;
    if (!usbio)
    return; /* Disconnected */
// txbuf_len = usbio->txbuf_len;
// rxbuf_len = usbio->rxbuf_len;
    }
    EXPORT_SYMBOL_NS_GPL(usbio_get_txrxbuf_len, "USBIO");
#[no_mangle]
pub unsafe extern "C" fn usbio_get_quirks(adev: *mut auxiliary_device) -> c_ulong {
    unsigned long usbio_get_quirks(struct auxiliary_device *adev)
    {
    struct usbio_client *client = adev_to_client(adev);
    struct usbio_device *usbio;
    guard(mutex)(&client.mutex);
    usbio = client.bridge;
    if (!usbio)
    return 0; /* Disconnected */
    return usbio.quirks;
    }
    EXPORT_SYMBOL_NS_GPL(usbio_get_quirks, "USBIO");
#[no_mangle]
unsafe extern "C" fn usbio_auxdev_release(dev: *mut device) {
    static void usbio_auxdev_release(struct device *dev)
    {
    struct auxiliary_device *adev = to_auxiliary_dev(dev);
    struct usbio_client *client = adev_to_client(adev);
    mutex_destroy(&client.mutex);
    kfree(client);
    }
#[no_mangle]
unsafe extern "C" fn usbio_add_client(usbio: *mut usbio_device, name: *mut c_char, id: u8, data: *mut c_void) -> c_int {
    static int usbio_add_client(struct usbio_device *usbio, char *name, u8 id, void *data)
    {
    struct usbio_client *client;
    struct auxiliary_device *adev;
    int ret;
    client = kzalloc_obj(*client);
    if (!client)
    return -ENOMEM;
    mutex_init(&client.mutex);
    client.bridge = usbio;
    adev = &client.auxdev;
    adev.name = name;
    adev.id = id;
    adev.dev.parent = usbio.dev;
    adev.dev.platform_data = data;
    adev.dev.release = usbio_auxdev_release;
    ret = auxiliary_device_init(adev);
    if (ret) {
    usbio_auxdev_release(&adev.dev);
    return ret;
    }
    ret = auxiliary_device_add(adev);
    if (ret) {
    auxiliary_device_uninit(adev);
    return ret;
    }
    list_add_tail(&client.link, &usbio.cli_list);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usbio_enum_gpios(usbio: *mut usbio_device) -> c_int {
    static int usbio_enum_gpios(struct usbio_device *usbio)
    {
    struct usbio_gpio_bank_desc *gpio = usbio.gpios;
    dev_dbg(usbio.dev, "GPIO Banks: %d\n", usbio.nr_gpio_banks);
    for (unsigned int i = 0; i < usbio.nr_gpio_banks; i++)
    dev_dbg(usbio.dev, "\tBank%d[%d] map: %#08x\n",
    gpio[i].id, gpio[i].pins, gpio[i].bmap);
    usbio_add_client(usbio, USBIO_GPIO_CLIENT, 0, gpio);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usbio_enum_i2cs(usbio: *mut usbio_device) -> c_int {
    static int usbio_enum_i2cs(struct usbio_device *usbio)
    {
    struct usbio_i2c_bus_desc *i2c = usbio.i2cs;
    dev_dbg(usbio.dev, "I2C Busses: %d\n", usbio.nr_i2c_buses);
    for (unsigned int i = 0; i < usbio.nr_i2c_buses; i++) {
    dev_dbg(usbio.dev, "\tBus%d caps: %#02x\n", i2c[i].id, i2c[i].caps);
    usbio_add_client(usbio, USBIO_I2C_CLIENT, i, &i2c[i]);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usbio_suspend(intf: *mut usb_interface, msg: pm_message_t) -> c_int {
    static int usbio_suspend(struct usb_interface *intf, pm_message_t msg)
    {
    struct usbio_device *usbio = usb_get_intfdata(intf);
    usb_kill_urb(usbio.urb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usbio_resume(intf: *mut usb_interface) -> c_int {
    static int usbio_resume(struct usb_interface *intf)
    {
    struct usbio_device *usbio = usb_get_intfdata(intf);
    return usb_submit_urb(usbio.urb, GFP_KERNEL);
    }
#[no_mangle]
unsafe extern "C" fn usbio_disconnect(intf: *mut usb_interface) {
    static void usbio_disconnect(struct usb_interface *intf)
    {
    struct usbio_device *usbio = usb_get_intfdata(intf);
    struct usbio_client *client, *next;
// Wakeup any clients waiting for a reply
    usbio.rxdat_len = 0;
    complete(&usbio.done);
// Let clients know the bridge is gone
    list_for_each_entry(client, &usbio.cli_list, link) {
    mutex_lock(&client.mutex);
    client.bridge = core::ptr::null_mut();
    mutex_unlock(&client.mutex);
    }
// From here on clients will no longer touch struct usbio_device
    usb_kill_urb(usbio.urb);
    usb_free_urb(usbio.urb);
    list_for_each_entry_safe_reverse(client, next, &usbio.cli_list, link) {
    auxiliary_device_delete(&client.auxdev);
    auxiliary_device_uninit(&client.auxdev);
    }
    }
#[no_mangle]
unsafe extern "C" fn usbio_probe(intf: *mut usb_interface, id: *const usb_device_id) -> c_int {
    static int usbio_probe(struct usb_interface *intf, const struct usb_device_id *id)
    {
    struct usb_device *udev = interface_to_usbdev(intf);
    struct usb_endpoint_descriptor *ep_in, *ep_out;
    struct device *dev = &intf.dev;
    struct usbio_protver protver;
    struct usbio_device *usbio;
    struct usbio_fwver fwver;
    int ret;
    usbio = devm_kzalloc(dev, sizeof(*usbio), GFP_KERNEL);
    if (!usbio)
    return -ENOMEM;
    ret = devm_mutex_init(dev, &usbio.ctrl_mutex);
    if (ret)
    return ret;
    ret = devm_mutex_init(dev, &usbio.bulk_mutex);
    if (ret)
    return ret;
    usbio.dev = dev;
    usbio.udev = udev;
    usbio.intf = intf;
    usbio.quirks = id ? id.driver_info : 0;
    init_completion(&usbio.done);
    INIT_LIST_HEAD(&usbio.cli_list);
    usb_set_intfdata(intf, usbio);
    usbio.ctrl_pipe = usb_endpoint_num(&udev.ep0.desc);
    usbio.ctrlbuf_len = usb_maxpacket(udev, usbio.ctrl_pipe);
    usbio.ctrlbuf = devm_kzalloc(dev, usbio.ctrlbuf_len, GFP_KERNEL);
    if (!usbio.ctrlbuf)
    return -ENOMEM;
// Find the first bulk-in and bulk-out endpoints
    ret = usb_find_common_endpoints(intf.cur_altsetting, &ep_in, &ep_out,
    core::ptr::null_mut(), core::ptr::null_mut());
    if (ret) {
    dev_err(dev, "Cannot find bulk endpoints: %d\n", ret);
    return ret;
    }
    usbio.tx_pipe = usb_sndbulkpipe(udev, usb_endpoint_num(ep_out));
    if (usbio.quirks & USBIO_QUIRK_BULK_MAXP_63)
    usbio.txbuf_len = 63;
    else
    usbio.txbuf_len = usb_endpoint_maxp(ep_out);
    usbio.txbuf = devm_kzalloc(dev, usbio.txbuf_len, GFP_KERNEL);
    if (!usbio.txbuf)
    return -ENOMEM;
    usbio.rx_pipe = usb_rcvbulkpipe(udev, usb_endpoint_num(ep_in));
    if (usbio.quirks & USBIO_QUIRK_BULK_MAXP_63)
    usbio.rxbuf_len = 63;
    else
    usbio.rxbuf_len = usb_endpoint_maxp(ep_in);
    usbio.rxbuf = devm_kzalloc(dev, usbio.rxbuf_len, GFP_KERNEL);
    if (!usbio.rxbuf)
    return -ENOMEM;
    usbio.urb = usb_alloc_urb(0, GFP_KERNEL);
    if (!usbio.urb)
    return -ENOMEM;
    usb_fill_bulk_urb(usbio.urb, udev, usbio.rx_pipe, usbio.rxbuf,
    usbio.rxbuf_len, usbio_bulk_recv, usbio);
    ret = usb_submit_urb(usbio.urb, GFP_KERNEL);
    if (ret) {
    dev_err_probe(dev, ret, "Submitting usb urb\n");
    goto err_free_urb;
    }
    mutex_lock(&usbio.ctrl_mutex);
    ret = usbio_ctrl_msg(usbio, USBIO_PKTTYPE_CTRL, USBIO_CTRLCMD_HS, core::ptr::null_mut(), 0, core::ptr::null_mut(), 0);
    if (ret < 0)
    goto err_unlock;
    ret = usbio_ctrl_msg(usbio, USBIO_PKTTYPE_CTRL, USBIO_CTRLCMD_PROTVER, core::ptr::null_mut(), 0,
    &protver, sizeof(protver));
    if (ret < 0)
    goto err_unlock;
    ret = usbio_ctrl_msg(usbio, USBIO_PKTTYPE_CTRL, USBIO_CTRLCMD_FWVER, core::ptr::null_mut(), 0,
    &fwver, sizeof(fwver));
    if (ret < 0)
    goto err_unlock;
    ret = usbio_ctrl_msg(usbio, USBIO_PKTTYPE_CTRL, USBIO_CTRLCMD_ENUMGPIO, core::ptr::null_mut(), 0,
    usbio.gpios, sizeof(usbio.gpios));
    if (ret < 0 || ret % sizeof(struct usbio_gpio_bank_desc)) {
    ret = (ret < 0) ? ret : -EPROTO;
    goto err_unlock;
    }
    usbio.nr_gpio_banks = ret / sizeof(struct usbio_gpio_bank_desc);
    ret = usbio_ctrl_msg(usbio, USBIO_PKTTYPE_CTRL, USBIO_CTRLCMD_ENUMI2C, core::ptr::null_mut(), 0,
    usbio.i2cs, sizeof(usbio.i2cs));
    if (ret < 0 || ret % sizeof(struct usbio_i2c_bus_desc)) {
    ret = (ret < 0) ? ret : -EPROTO;
    goto err_unlock;
    }
    usbio.nr_i2c_buses = ret / sizeof(struct usbio_i2c_bus_desc);
    mutex_unlock(&usbio.ctrl_mutex);
    dev_dbg(dev, "ProtVer(BCD): %02x FwVer: %d.%d.%d.%d\n",
    protver.ver, fwver.major, fwver.minor,
    le16_to_cpu(fwver.patch), le16_to_cpu(fwver.build));
    usbio_enum_gpios(usbio);
    usbio_enum_i2cs(usbio);
    return 0;
    err_unlock:
    mutex_unlock(&usbio.ctrl_mutex);
    usb_kill_urb(usbio.urb);
    err_free_urb:
    usb_free_urb(usbio.urb);
    return ret;
    }
    static const struct usb_device_id usbio_table[] = {
    { USB_DEVICE(0x2ac1, 0x20c1),	/* Lattice NX40 */
    .driver_info = USBIO_QUIRK_I2C_MAX_RW_LEN_52 },
    { USB_DEVICE(0x2ac1, 0x20c9),	/* Lattice NX33 */
    .driver_info = USBIO_QUIRK_I2C_NO_INIT_ACK | USBIO_QUIRK_I2C_MAX_RW_LEN_52 |
    USBIO_QUIRK_I2C_ALLOW_400KHZ },
    { USB_DEVICE(0x2ac1, 0x20cb) },	/* Lattice NX33U */
    { USB_DEVICE(0x06cb, 0x0701),	/* Synaptics Sabre */
    .driver_info = USBIO_QUIRK_BULK_MAXP_63 | USBIO_QUIRK_I2C_USE_CHUNK_LEN },
    { }
    };
    MODULE_DEVICE_TABLE(usb, usbio_table);
    static struct usb_driver usbio_driver = {
    .name = "usbio-bridge",
    .probe = usbio_probe,
    .disconnect = usbio_disconnect,
    .suspend = usbio_suspend,
    .resume = usbio_resume,
    .id_table = usbio_table,
    .supports_autosuspend = 1,
    };
    module_usb_driver(usbio_driver);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbio_match_ids_walk_data {
    pub adev: *mut acpi_device,
    pub hids: *const acpi_device_id,
    pub id: c_uint,
}

#[no_mangle]
unsafe extern "C" fn usbio_match_device_ids(adev: *mut acpi_device, data: *mut c_void) -> c_int {
    static int usbio_match_device_ids(struct acpi_device *adev, void *data)
    {
    struct usbio_match_ids_walk_data *wd = data;
    let mut id: c_uint = 0;
    char *uid;
    if (acpi_match_device_ids(adev, wd.hids))
    return 0;
    uid = acpi_device_uid(adev);
    if (uid) {
    for (int i = 0; i < strlen(uid); i++) {
    if (!kstrtouint(&uid[i], 10, &id))
    break;
    }
    }
    if (!uid || wd.id == id) {
    wd.adev = adev;
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn usbio_acpi_bind(adev: *mut auxiliary_device, hids: *const acpi_device_id) {
    void usbio_acpi_bind(struct auxiliary_device *adev, const struct acpi_device_id *hids)
    {
    struct device *dev = &adev.dev;
    struct acpi_device *parent;
    struct usbio_match_ids_walk_data wd = {
    .adev = core::ptr::null_mut(),
    .hids = hids,
    .id = adev.id,
    };
    parent = ACPI_COMPANION(dev.parent);
    if (!parent)
    return;
    acpi_dev_for_each_child(parent, usbio_match_device_ids, &wd);
    if (wd.adev)
    ACPI_COMPANION_SET(dev, wd.adev);
    }
    EXPORT_SYMBOL_NS_GPL(usbio_acpi_bind, "USBIO");
    MODULE_DESCRIPTION("Intel USBIO Bridge driver");
    MODULE_AUTHOR("Israel Cepeda <israel.a.cepeda.lopez@intel.com>");
    MODULE_AUTHOR("Hans de Goede <hansg@kernel.org>");
    MODULE_LICENSE("GPL");
