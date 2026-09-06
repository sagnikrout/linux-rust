//! Automatically rewritten from C to Rust
//! Source: drivers/bluetooth/virtio_bt.c
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

pub const VIRTBT_RX_BUF_SIZE: c_int = 1000;
    enum {
    VIRTBT_VQ_TX,
    VIRTBT_VQ_RX,
    VIRTBT_NUM_VQS,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_bluetooth {
    pub vdev: *mut virtio_device,
    pub vqs: [*mut virtqueue; VIRTBT_NUM_VQS],
    pub rx: work_struct,
    pub hdev: *mut hci_dev,
}

#[no_mangle]
unsafe extern "C" fn virtbt_add_inbuf(vbt: *mut virtio_bluetooth) -> c_int {
    static int virtbt_add_inbuf(struct virtio_bluetooth *vbt)
    {
    struct virtqueue *vq = vbt.vqs[VIRTBT_VQ_RX];
    struct scatterlist sg[1];
    struct sk_buff *skb;
    int err;
    skb = alloc_skb(VIRTBT_RX_BUF_SIZE, GFP_KERNEL);
    if (!skb)
    return -ENOMEM;
    sg_init_one(sg, skb.data, VIRTBT_RX_BUF_SIZE);
    err = virtqueue_add_inbuf(vq, sg, 1, skb, GFP_KERNEL);
    if (err < 0) {
    kfree_skb(skb);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtbt_open(hdev: *mut hci_dev) -> c_int {
    static int virtbt_open(struct hci_dev *hdev)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtbt_open_vdev(vbt: *mut virtio_bluetooth) -> c_int {
    static int virtbt_open_vdev(struct virtio_bluetooth *vbt)
    {
    if (virtbt_add_inbuf(vbt) < 0)
    return -EIO;
    virtqueue_kick(vbt.vqs[VIRTBT_VQ_RX]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtbt_close(hdev: *mut hci_dev) -> c_int {
    static int virtbt_close(struct hci_dev *hdev)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtbt_close_vdev(vbt: *mut virtio_bluetooth) -> c_int {
    static int virtbt_close_vdev(struct virtio_bluetooth *vbt)
    {
    int i;
    cancel_work_sync(&vbt.rx);
    for (i = 0; i < ARRAY_SIZE(vbt.vqs); i++) {
    struct virtqueue *vq = vbt.vqs[i];
    struct sk_buff *skb;
    while ((skb = virtqueue_detach_unused_buf(vq)))
    kfree_skb(skb);
    cond_resched();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtbt_flush(hdev: *mut hci_dev) -> c_int {
    static int virtbt_flush(struct hci_dev *hdev)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtbt_send_frame(hdev: *mut hci_dev, skb: *mut sk_buff) -> c_int {
    static int virtbt_send_frame(struct hci_dev *hdev, struct sk_buff *skb)
    {
    struct virtio_bluetooth *vbt = hci_get_drvdata(hdev);
    struct scatterlist sg[1];
    int err;
    memcpy(skb_push(skb, 1), &hci_skb_pkt_type(skb), 1);
    sg_init_one(sg, skb.data, skb.len);
    err = virtqueue_add_outbuf(vbt.vqs[VIRTBT_VQ_TX], sg, 1, skb,
    GFP_KERNEL);
    if (err) {
    kfree_skb(skb);
    return err;
    }
    virtqueue_kick(vbt.vqs[VIRTBT_VQ_TX]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtbt_setup_zephyr(hdev: *mut hci_dev) -> c_int {
    static int virtbt_setup_zephyr(struct hci_dev *hdev)
    {
    struct sk_buff *skb;
// Read Build Information
    skb = __hci_cmd_sync(hdev, 0xfc08, 0, core::ptr::null_mut(), HCI_INIT_TIMEOUT);
    if (IS_ERR(skb))
    return PTR_ERR(skb);
// Bounded print: the backend controls skb->len.
    if (skb.len > 1) {
    let mut len: c_int = skb.len - 1;
    bt_dev_info(hdev, "%.*s", len, (char *)(skb.data + 1));
    hci_set_fw_info(hdev, "%.*s", len, skb.data + 1);
    }
    kfree_skb(skb);
    return 0;
    }
    static int virtbt_set_bdaddr_zephyr(struct hci_dev *hdev,
    const bdaddr_t *bdaddr)
    {
    struct sk_buff *skb;
// Write BD_ADDR
    skb = __hci_cmd_sync(hdev, 0xfc06, 6, bdaddr, HCI_INIT_TIMEOUT);
    if (IS_ERR(skb))
    return PTR_ERR(skb);
    kfree_skb(skb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtbt_setup_intel(hdev: *mut hci_dev) -> c_int {
    static int virtbt_setup_intel(struct hci_dev *hdev)
    {
    struct sk_buff *skb;
// Intel Read Version
    skb = __hci_cmd_sync(hdev, 0xfc05, 0, core::ptr::null_mut(), HCI_CMD_TIMEOUT);
    if (IS_ERR(skb))
    return PTR_ERR(skb);
    kfree_skb(skb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtbt_set_bdaddr_intel(hdev: *mut hci_dev, bdaddr: *const bdaddr_t) -> c_int {
    static int virtbt_set_bdaddr_intel(struct hci_dev *hdev, const bdaddr_t *bdaddr)
    {
    struct sk_buff *skb;
// Intel Write BD Address
    skb = __hci_cmd_sync(hdev, 0xfc31, 6, bdaddr, HCI_INIT_TIMEOUT);
    if (IS_ERR(skb))
    return PTR_ERR(skb);
    kfree_skb(skb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtbt_setup_realtek(hdev: *mut hci_dev) -> c_int {
    static int virtbt_setup_realtek(struct hci_dev *hdev)
    {
    struct sk_buff *skb;
// Read ROM Version
    skb = __hci_cmd_sync(hdev, 0xfc6d, 0, core::ptr::null_mut(), HCI_INIT_TIMEOUT);
    if (IS_ERR(skb))
    return PTR_ERR(skb);
    bt_dev_info(hdev, "ROM version %u", *((__u8 *) (skb.data + 1)));
    kfree_skb(skb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtbt_shutdown_generic(hdev: *mut hci_dev) -> c_int {
    static int virtbt_shutdown_generic(struct hci_dev *hdev)
    {
    struct sk_buff *skb;
// Reset
    skb = __hci_cmd_sync(hdev, HCI_OP_RESET, 0, core::ptr::null_mut(), HCI_INIT_TIMEOUT);
    if (IS_ERR(skb))
    return PTR_ERR(skb);
    kfree_skb(skb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtbt_rx_handle(vbt: *mut virtio_bluetooth, skb: *mut sk_buff) {
    static void virtbt_rx_handle(struct virtio_bluetooth *vbt, struct sk_buff *skb)
    {
    size_t min_hdr;
    __u8 pkt_type;
    pkt_type = *((__u8 *) skb.data);
    skb_pull(skb, 1);
    switch (pkt_type) {
    case HCI_EVENT_PKT:
    min_hdr = sizeof(struct hci_event_hdr);
    break;
    case HCI_ACLDATA_PKT:
    min_hdr = sizeof(struct hci_acl_hdr);
    break;
    case HCI_SCODATA_PKT:
    min_hdr = sizeof(struct hci_sco_hdr);
    break;
    case HCI_ISODATA_PKT:
    min_hdr = sizeof(struct hci_iso_hdr);
    break;
    default:
    kfree_skb(skb);
    return;
    }
    if (skb.len < min_hdr) {
    bt_dev_err_ratelimited(vbt.hdev,
    "rx pkt_type 0x%02x payload %u < hdr %zu\n",
    pkt_type, skb.len, min_hdr);
    kfree_skb(skb);
    return;
    }
    hci_skb_pkt_type(skb) = pkt_type;
    hci_recv_frame(vbt.hdev, skb);
    }
#[no_mangle]
unsafe extern "C" fn virtbt_rx_work(work: *mut work_struct) {
    static void virtbt_rx_work(struct work_struct *work)
    {
    struct virtio_bluetooth *vbt = container_of(work,
    struct virtio_bluetooth, rx);
    struct sk_buff *skb;
    unsigned int len;
    skb = virtqueue_get_buf(vbt.vqs[VIRTBT_VQ_RX], &len);
    if (!skb)
    return;
    if (!len || len > VIRTBT_RX_BUF_SIZE) {
    bt_dev_err_ratelimited(vbt.hdev,
    "rx reply len %u outside [1, %u]\n",
    len, VIRTBT_RX_BUF_SIZE);
    kfree_skb(skb);
    } else {
    skb_put(skb, len);
    virtbt_rx_handle(vbt, skb);
    }
    if (virtbt_add_inbuf(vbt) < 0)
    return;
    virtqueue_kick(vbt.vqs[VIRTBT_VQ_RX]);
    }
#[no_mangle]
unsafe extern "C" fn virtbt_tx_done(vq: *mut virtqueue) {
    static void virtbt_tx_done(struct virtqueue *vq)
    {
    struct sk_buff *skb;
    unsigned int len;
    while ((skb = virtqueue_get_buf(vq, &len)))
    kfree_skb(skb);
    }
#[no_mangle]
unsafe extern "C" fn virtbt_rx_done(vq: *mut virtqueue) {
    static void virtbt_rx_done(struct virtqueue *vq)
    {
    struct virtio_bluetooth *vbt = vq.vdev.priv;
    schedule_work(&vbt.rx);
    }
#[no_mangle]
unsafe extern "C" fn virtbt_probe(vdev: *mut virtio_device) -> c_int {
    static int virtbt_probe(struct virtio_device *vdev)
    {
    struct virtqueue_info vqs_info[VIRTBT_NUM_VQS] = {
    [VIRTBT_VQ_TX] = { "tx", virtbt_tx_done },
    [VIRTBT_VQ_RX] = { "rx", virtbt_rx_done },
    };
    struct virtio_bluetooth *vbt;
    struct hci_dev *hdev;
    int err;
    __u8 type;
    if (!virtio_has_feature(vdev, VIRTIO_F_VERSION_1))
    return -ENODEV;
    type = virtio_cread8(vdev, offsetof(struct virtio_bt_config, type));
    switch (type) {
    case VIRTIO_BT_CONFIG_TYPE_PRIMARY:
    break;
    default:
    return -EINVAL;
    }
    vbt = kzalloc_obj(*vbt);
    if (!vbt)
    return -ENOMEM;
    vdev.priv = vbt;
    vbt.vdev = vdev;
    INIT_WORK(&vbt.rx, virtbt_rx_work);
    err = virtio_find_vqs(vdev, VIRTBT_NUM_VQS, vbt.vqs, vqs_info, core::ptr::null_mut());
    if (err)
    return err;
    hdev = hci_alloc_dev();
    if (!hdev) {
    err = -ENOMEM;
    goto failed;
    }
    vbt.hdev = hdev;
    hdev.bus = HCI_VIRTIO;
    hci_set_drvdata(hdev, vbt);
    hdev.open  = virtbt_open;
    hdev.close = virtbt_close;
    hdev.flush = virtbt_flush;
    hdev.send  = virtbt_send_frame;
    if (virtio_has_feature(vdev, VIRTIO_BT_F_VND_HCI)) {
    __u16 vendor;
    if (virtio_has_feature(vdev, VIRTIO_BT_F_CONFIG_V2))
    virtio_cread(vdev, struct virtio_bt_config_v2,
    vendor, &vendor);
    else
    virtio_cread(vdev, struct virtio_bt_config,
    vendor, &vendor);
    switch (vendor) {
    case VIRTIO_BT_CONFIG_VENDOR_ZEPHYR:
    hdev.manufacturer = 1521;
    hdev.setup = virtbt_setup_zephyr;
    hdev.shutdown = virtbt_shutdown_generic;
    hdev.set_bdaddr = virtbt_set_bdaddr_zephyr;
    break;
    case VIRTIO_BT_CONFIG_VENDOR_INTEL:
    hdev.manufacturer = 2;
    hdev.setup = virtbt_setup_intel;
    hdev.shutdown = virtbt_shutdown_generic;
    hdev.set_bdaddr = virtbt_set_bdaddr_intel;
    hci_set_quirk(hdev, HCI_QUIRK_STRICT_DUPLICATE_FILTER);
    hci_set_quirk(hdev, HCI_QUIRK_SIMULTANEOUS_DISCOVERY);
    hci_set_quirk(hdev, HCI_QUIRK_WIDEBAND_SPEECH_SUPPORTED);
    break;
    case VIRTIO_BT_CONFIG_VENDOR_REALTEK:
    hdev.manufacturer = 93;
    hdev.setup = virtbt_setup_realtek;
    hdev.shutdown = virtbt_shutdown_generic;
    hci_set_quirk(hdev, HCI_QUIRK_SIMULTANEOUS_DISCOVERY);
    hci_set_quirk(hdev, HCI_QUIRK_WIDEBAND_SPEECH_SUPPORTED);
    break;
    }
    }
    if (virtio_has_feature(vdev, VIRTIO_BT_F_MSFT_EXT)) {
    __u16 msft_opcode;
    if (virtio_has_feature(vdev, VIRTIO_BT_F_CONFIG_V2))
    virtio_cread(vdev, struct virtio_bt_config_v2,
    msft_opcode, &msft_opcode);
    else
    virtio_cread(vdev, struct virtio_bt_config,
    msft_opcode, &msft_opcode);
    hci_set_msft_opcode(hdev, msft_opcode);
    }
    if (virtio_has_feature(vdev, VIRTIO_BT_F_AOSP_EXT))
    hci_set_aosp_capable(hdev);
    if (hci_register_dev(hdev) < 0) {
    hci_free_dev(hdev);
    err = -EBUSY;
    goto failed;
    }
    virtio_device_ready(vdev);
    err = virtbt_open_vdev(vbt);
    if (err)
    goto open_failed;
    return 0;
    open_failed:
    hci_free_dev(hdev);
    failed:
    vdev.config.del_vqs(vdev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn virtbt_remove(vdev: *mut virtio_device) {
    static void virtbt_remove(struct virtio_device *vdev)
    {
    struct virtio_bluetooth *vbt = vdev.priv;
    struct hci_dev *hdev = vbt.hdev;
    hci_unregister_dev(hdev);
    virtio_reset_device(vdev);
    virtbt_close_vdev(vbt);
    hci_free_dev(hdev);
    vbt.hdev = core::ptr::null_mut();
    vdev.config.del_vqs(vdev);
    kfree(vbt);
    }
    static struct virtio_device_id virtbt_table[] = {
    { VIRTIO_ID_BT, VIRTIO_DEV_ANY_ID },
    { 0 },
    };
    MODULE_DEVICE_TABLE(virtio, virtbt_table);
    static const unsigned int virtbt_features[] = {
    VIRTIO_BT_F_VND_HCI,
    VIRTIO_BT_F_MSFT_EXT,
    VIRTIO_BT_F_AOSP_EXT,
    VIRTIO_BT_F_CONFIG_V2,
    };
    static struct virtio_driver virtbt_driver = {
    .driver.name         = KBUILD_MODNAME,
    .feature_table       = virtbt_features,
    .feature_table_size  = ARRAY_SIZE(virtbt_features),
    .id_table            = virtbt_table,
    .probe               = virtbt_probe,
    .remove              = virtbt_remove,
    };
    module_virtio_driver(virtbt_driver);
    MODULE_AUTHOR("Marcel Holtmann <marcel@holtmann.org>");
    MODULE_DESCRIPTION("Generic Bluetooth VIRTIO driver ver " VERSION);
    MODULE_VERSION(VERSION);
    MODULE_LICENSE("GPL");
