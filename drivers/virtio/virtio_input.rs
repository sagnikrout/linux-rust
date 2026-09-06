//! Automatically rewritten from C to Rust
//! Source: drivers/virtio/virtio_input.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_input {
    pub vdev: *mut virtio_device,
    pub idev: *mut input_dev,
    pub name: [c_char; 64],
    pub serial: [c_char; 64],
    pub phys: [c_char; 64],
    pub sts: *mut *mut virtqueue evt,,
    pub evts: [virtio_input_event; 64],
    pub lock: spinlock_t,
    pub ready: bool,
}

    static void virtinput_queue_evtbuf(struct virtio_input *vi,
    struct virtio_input_event *evtbuf)
    {
    struct scatterlist sg[1];
    sg_init_one(sg, evtbuf, sizeof(*evtbuf));
    virtqueue_add_inbuf_cache_clean(vi.evt, sg, 1, evtbuf, GFP_ATOMIC);
    }
#[no_mangle]
unsafe extern "C" fn virtinput_recv_events(vq: *mut virtqueue) {
    static void virtinput_recv_events(struct virtqueue *vq)
    {
    struct virtio_input *vi = vq.vdev.priv;
    struct virtio_input_event *event;
    unsigned long flags;
    unsigned int len;
    spin_lock_irqsave(&vi.lock, flags);
    if (vi.ready) {
    while ((event = virtqueue_get_buf(vi.evt, &len)) != core::ptr::null_mut()) {
    spin_unlock_irqrestore(&vi.lock, flags);
    input_event(vi.idev,
    le16_to_cpu(event.type),
    le16_to_cpu(event.code),
    le32_to_cpu(event.value));
    spin_lock_irqsave(&vi.lock, flags);
    virtinput_queue_evtbuf(vi, event);
    }
    virtqueue_kick(vq);
    }
    spin_unlock_irqrestore(&vi.lock, flags);
    }
//
// On error we are losing the status update, which isn't critical as
// this is typically used for stuff like keyboard leds.
//
    static int virtinput_send_status(struct virtio_input *vi,
    u16 type, u16 code, s32 value)
    {
    struct virtio_input_event *stsbuf;
    struct scatterlist sg[1];
    unsigned long flags;
    int rc;
//
// Since 29cc309d8bf1 (HID: hid-multitouch: forward MSC_TIMESTAMP),
// EV_MSC/MSC_TIMESTAMP is added to each before EV_SYN event.
// EV_MSC is configured as INPUT_PASS_TO_ALL.
// In case of touch device:
// BE pass EV_MSC/MSC_TIMESTAMP to FE on receiving event from evdev.
// FE pass EV_MSC/MSC_TIMESTAMP back to BE.
// BE writes EV_MSC/MSC_TIMESTAMP to evdev due to INPUT_PASS_TO_ALL.
// BE receives extra EV_MSC/MSC_TIMESTAMP and pass to FE.
// >>> Each new frame becomes larger and larger.
// Disable EV_MSC/MSC_TIMESTAMP forwarding for MT.
//
    if (vi.idev.mt && type == EV_MSC && code == MSC_TIMESTAMP)
    return 0;
    stsbuf = kzalloc_obj(*stsbuf, GFP_ATOMIC);
    if (!stsbuf)
    return -ENOMEM;
    stsbuf.type  = cpu_to_le16(type);
    stsbuf.code  = cpu_to_le16(code);
    stsbuf.value = cpu_to_le32(value);
    sg_init_one(sg, stsbuf, sizeof(*stsbuf));
    spin_lock_irqsave(&vi.lock, flags);
    if (vi.ready) {
    rc = virtqueue_add_outbuf(vi.sts, sg, 1, stsbuf, GFP_ATOMIC);
    virtqueue_kick(vi.sts);
    } else {
    rc = -ENODEV;
    }
    spin_unlock_irqrestore(&vi.lock, flags);
    if (rc != 0)
    kfree(stsbuf);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn virtinput_recv_status(vq: *mut virtqueue) {
    static void virtinput_recv_status(struct virtqueue *vq)
    {
    struct virtio_input *vi = vq.vdev.priv;
    struct virtio_input_event *stsbuf;
    unsigned long flags;
    unsigned int len;
    spin_lock_irqsave(&vi.lock, flags);
    while ((stsbuf = virtqueue_get_buf(vi.sts, &len)) != core::ptr::null_mut())
    kfree(stsbuf);
    spin_unlock_irqrestore(&vi.lock, flags);
    }
    static int virtinput_status(struct input_dev *idev, unsigned int type,
    unsigned int code, int value)
    {
    struct virtio_input *vi = input_get_drvdata(idev);
    return virtinput_send_status(vi, type, code, value);
    }
    static u8 virtinput_cfg_select(struct virtio_input *vi,
    u8 select, u8 subsel)
    {
    u8 size;
    virtio_cwrite_le(vi.vdev, struct virtio_input_config, select, &select);
    virtio_cwrite_le(vi.vdev, struct virtio_input_config, subsel, &subsel);
    virtio_cread_le(vi.vdev, struct virtio_input_config, size, &size);
    return size;
    }
    static void virtinput_cfg_bits(struct virtio_input *vi, int select, int subsel,
    unsigned long *bits, unsigned int bitcount)
    {
    unsigned int bit;
    u8 *virtio_bits;
    u8 bytes;
    bytes = virtinput_cfg_select(vi, select, subsel);
    if (!bytes)
    return;
    if (bitcount > bytes * 8)
    bitcount = bytes * 8;
//
// Bitmap in virtio config space is a simple stream of bytes,
// with the first byte carrying bits 0-7, second bits 8-15 and
// so on.
//
    virtio_bits = kzalloc(bytes, GFP_KERNEL);
    if (!virtio_bits)
    return;
    virtio_cread_bytes(vi.vdev, offsetof(struct virtio_input_config,
    u.bitmap),
    virtio_bits, bytes);
    for (bit = 0; bit < bitcount; bit++) {
    if (virtio_bits[bit / 8] & (1 << (bit % 8)))
    __set_bit(bit, bits);
    }
    kfree(virtio_bits);
    if (select == VIRTIO_INPUT_CFG_EV_BITS)
    __set_bit(subsel, vi.idev.evbit);
    }
#[no_mangle]
unsafe extern "C" fn virtinput_cfg_abs(vi: *mut virtio_input, abs: c_int) {
    static void virtinput_cfg_abs(struct virtio_input *vi, int abs)
    {
    u32 mi, ma, re, fu, fl;
    virtinput_cfg_select(vi, VIRTIO_INPUT_CFG_ABS_INFO, abs);
    virtio_cread_le(vi.vdev, struct virtio_input_config, u.abs.min, &mi);
    virtio_cread_le(vi.vdev, struct virtio_input_config, u.abs.max, &ma);
    virtio_cread_le(vi.vdev, struct virtio_input_config, u.abs.res, &re);
    virtio_cread_le(vi.vdev, struct virtio_input_config, u.abs.fuzz, &fu);
    virtio_cread_le(vi.vdev, struct virtio_input_config, u.abs.flat, &fl);
    input_set_abs_params(vi.idev, abs, mi, ma, fu, fl);
    input_abs_set_res(vi.idev, abs, re);
    }
#[no_mangle]
unsafe extern "C" fn virtinput_init_vqs(vi: *mut virtio_input) -> c_int {
    static int virtinput_init_vqs(struct virtio_input *vi)
    {
    struct virtqueue_info vqs_info[] = {
    { "events", virtinput_recv_events },
    { "status", virtinput_recv_status },
    };
    struct virtqueue *vqs[2];
    int err;
    err = virtio_find_vqs(vi.vdev, 2, vqs, vqs_info, core::ptr::null_mut());
    if (err)
    return err;
    vi.evt = vqs[0];
    vi.sts = vqs[1];
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtinput_fill_evt(vi: *mut virtio_input) {
    static void virtinput_fill_evt(struct virtio_input *vi)
    {
    unsigned long flags;
    int i, size;
    spin_lock_irqsave(&vi.lock, flags);
    size = virtqueue_get_vring_size(vi.evt);
    if (size > ARRAY_SIZE(vi.evts))
    size = ARRAY_SIZE(vi.evts);
    for (i = 0; i < size; i++)
    virtinput_queue_evtbuf(vi, &vi.evts[i]);
    virtqueue_kick(vi.evt);
    spin_unlock_irqrestore(&vi.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn virtinput_probe(vdev: *mut virtio_device) -> c_int {
    static int virtinput_probe(struct virtio_device *vdev)
    {
    struct virtio_input *vi;
    unsigned long flags;
    size_t size;
    int abs, err, nslots;
    if (!virtio_has_feature(vdev, VIRTIO_F_VERSION_1))
    return -ENODEV;
    vi = kzalloc_obj(*vi);
    if (!vi)
    return -ENOMEM;
    vdev.priv = vi;
    vi.vdev = vdev;
    spin_lock_init(&vi.lock);
    err = virtinput_init_vqs(vi);
    if (err)
    goto err_init_vq;
    vi.idev = input_allocate_device();
    if (!vi.idev) {
    err = -ENOMEM;
    goto err_input_alloc;
    }
    input_set_drvdata(vi.idev, vi);
    size = virtinput_cfg_select(vi, VIRTIO_INPUT_CFG_ID_NAME, 0);
    virtio_cread_bytes(vi.vdev, offsetof(struct virtio_input_config,
    u.string),
    vi.name, min(size, sizeof(vi.name)));
    size = virtinput_cfg_select(vi, VIRTIO_INPUT_CFG_ID_SERIAL, 0);
    virtio_cread_bytes(vi.vdev, offsetof(struct virtio_input_config,
    u.string),
    vi.serial, min(size, sizeof(vi.serial)));
    snprintf(vi.phys, sizeof(vi.phys),
    "virtio%d/input0", vdev.index);
    vi.idev.name = vi.name;
    vi.idev.phys = vi.phys;
    vi.idev.uniq = vi.serial;
    size = virtinput_cfg_select(vi, VIRTIO_INPUT_CFG_ID_DEVIDS, 0);
    if (size >= sizeof(struct virtio_input_devids)) {
    virtio_cread_le(vi.vdev, struct virtio_input_config,
    u.ids.bustype, &vi.idev.id.bustype);
    virtio_cread_le(vi.vdev, struct virtio_input_config,
    u.ids.vendor, &vi.idev.id.vendor);
    virtio_cread_le(vi.vdev, struct virtio_input_config,
    u.ids.product, &vi.idev.id.product);
    virtio_cread_le(vi.vdev, struct virtio_input_config,
    u.ids.version, &vi.idev.id.version);
    } else {
    vi.idev.id.bustype = BUS_VIRTUAL;
    }
    virtinput_cfg_bits(vi, VIRTIO_INPUT_CFG_PROP_BITS, 0,
    vi.idev.propbit, INPUT_PROP_CNT);
    size = virtinput_cfg_select(vi, VIRTIO_INPUT_CFG_EV_BITS, EV_REP);
    if (size)
    __set_bit(EV_REP, vi.idev.evbit);
    vi.idev.dev.parent = &vdev.dev;
    vi.idev.event = virtinput_status;
// device -> kernel
    virtinput_cfg_bits(vi, VIRTIO_INPUT_CFG_EV_BITS, EV_KEY,
    vi.idev.keybit, KEY_CNT);
    virtinput_cfg_bits(vi, VIRTIO_INPUT_CFG_EV_BITS, EV_REL,
    vi.idev.relbit, REL_CNT);
    virtinput_cfg_bits(vi, VIRTIO_INPUT_CFG_EV_BITS, EV_ABS,
    vi.idev.absbit, ABS_CNT);
    virtinput_cfg_bits(vi, VIRTIO_INPUT_CFG_EV_BITS, EV_MSC,
    vi.idev.mscbit, MSC_CNT);
    virtinput_cfg_bits(vi, VIRTIO_INPUT_CFG_EV_BITS, EV_SW,
    vi.idev.swbit,  SW_CNT);
// kernel -> device
    virtinput_cfg_bits(vi, VIRTIO_INPUT_CFG_EV_BITS, EV_LED,
    vi.idev.ledbit, LED_CNT);
    virtinput_cfg_bits(vi, VIRTIO_INPUT_CFG_EV_BITS, EV_SND,
    vi.idev.sndbit, SND_CNT);
    if (test_bit(EV_ABS, vi.idev.evbit)) {
    for (abs = 0; abs < ABS_CNT; abs++) {
    if (!test_bit(abs, vi.idev.absbit))
    continue;
    virtinput_cfg_abs(vi, abs);
    }
    if (test_bit(ABS_MT_SLOT, vi.idev.absbit)) {
    nslots = input_abs_get_max(vi.idev, ABS_MT_SLOT) + 1;
    err = input_mt_init_slots(vi.idev, nslots, 0);
    if (err)
    goto err_mt_init_slots;
    }
    }
    virtio_device_ready(vdev);
    vi.ready = true;
    err = input_register_device(vi.idev);
    if (err)
    goto err_input_register;
    virtinput_fill_evt(vi);
    return 0;
    err_input_register:
    spin_lock_irqsave(&vi.lock, flags);
    vi.ready = false;
    spin_unlock_irqrestore(&vi.lock, flags);
    err_mt_init_slots:
    input_free_device(vi.idev);
    err_input_alloc:
    vdev.config.del_vqs(vdev);
    err_init_vq:
    kfree(vi);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn virtinput_remove(vdev: *mut virtio_device) {
    static void virtinput_remove(struct virtio_device *vdev)
    {
    struct virtio_input *vi = vdev.priv;
    void *buf;
    unsigned long flags;
    spin_lock_irqsave(&vi.lock, flags);
    vi.ready = false;
    spin_unlock_irqrestore(&vi.lock, flags);
    input_unregister_device(vi.idev);
    virtio_reset_device(vdev);
    while ((buf = virtqueue_detach_unused_buf(vi.sts)) != core::ptr::null_mut())
    kfree(buf);
    vdev.config.del_vqs(vdev);
    kfree(vi);
    }

#[no_mangle]
unsafe extern "C" fn virtinput_freeze(vdev: *mut virtio_device) -> c_int {
    static int virtinput_freeze(struct virtio_device *vdev)
    {
    struct virtio_input *vi = vdev.priv;
    unsigned long flags;
    void *buf;
    spin_lock_irqsave(&vi.lock, flags);
    vi.ready = false;
    spin_unlock_irqrestore(&vi.lock, flags);
    virtio_reset_device(vdev);
    while ((buf = virtqueue_detach_unused_buf(vi.sts)) != core::ptr::null_mut())
    kfree(buf);
    vdev.config.del_vqs(vdev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtinput_restore(vdev: *mut virtio_device) -> c_int {
    static int virtinput_restore(struct virtio_device *vdev)
    {
    struct virtio_input *vi = vdev.priv;
    int err;
    err = virtinput_init_vqs(vi);
    if (err)
    return err;
    virtio_device_ready(vdev);
    vi.ready = true;
    virtinput_fill_evt(vi);
    return 0;
    }

    static unsigned int features[] = {
// none
    };
    static const struct virtio_device_id id_table[] = {
    { VIRTIO_ID_INPUT, VIRTIO_DEV_ANY_ID },
    { 0 },
    };
    static struct virtio_driver virtio_input_driver = {
    .driver.name         = KBUILD_MODNAME,
    .feature_table       = features,
    .feature_table_size  = ARRAY_SIZE(features),
    .id_table            = id_table,
    .probe               = virtinput_probe,
    .remove              = virtinput_remove,

    .freeze	             = virtinput_freeze,
    .restore             = virtinput_restore,

    };
    module_virtio_driver(virtio_input_driver);
    MODULE_DEVICE_TABLE(virtio, id_table);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Virtio input device driver");
    MODULE_AUTHOR("Gerd Hoffmann <kraxel@redhat.com>");
