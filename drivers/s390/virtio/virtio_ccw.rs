//! Automatically rewritten from C to Rust
//! Source: drivers/s390/virtio/virtio_ccw.c
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
// ccw based virtio transport
//
// Copyright IBM Corp. 2012, 2014
//
// Author(s): Cornelia Huck <cornelia.huck@de.ibm.com>
//

//
// virtio related functions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vq_config_block {
    pub index: __u16,
    pub num: __u16,
    pub __packed: },
pub const VIRTIO_CCW_CONFIG_SIZE: c_uint = 0x100;
// same as PCI config space size, should be enough for all drivers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcdev_dma_area {
    pub indicators: c_ulong,
    pub indicators2: c_ulong,
    pub config_block: vq_config_block,
    pub status: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_ccw_device {
    pub vdev: virtio_device,
    pub config: [__u8; VIRTIO_CCW_CONFIG_SIZE],
    pub cdev: *mut ccw_device,
// we make cdev->dev.dma_parms point to this
    pub dma_parms: device_dma_parameters,
    pub curr_io: __u32,
    pub err: c_int,
    pub /: *mut *mut unsigned int revision; / Transport revision,
    pub wait_q: wait_queue_head_t,
    pub lock: spinlock_t,
    pub irq_lock: rwlock_t,
    pub /: *mut *mut mutex io_lock; / Serializes I/O requests,
    pub virtqueues: list_head,
    pub is_thinint: bool,
    pub going_away: bool,
    pub device_lost: bool,
    pub config_ready: c_uint,
    pub airq_info: *mut c_void,
    pub dma_area: *mut vcdev_dma_area,
    pub dma_area_addr: dma32_t,
}

    static inline unsigned long *indicators(struct virtio_ccw_device *vcdev)
    {
    return &vcdev.dma_area.indicators;
    }
    static inline unsigned long *indicators2(struct virtio_ccw_device *vcdev)
    {
    return &vcdev.dma_area.indicators2;
    }
// Spec stipulates a 64 bit address
#[no_mangle]
pub unsafe extern "C" fn indicators_dma(vcdev: *mut virtio_ccw_device) -> dma64_t {
    static inline dma64_t indicators_dma(struct virtio_ccw_device *vcdev)
    {
    let mut dma_area_addr: u64 = dma32_to_u32(vcdev.dma_area_addr);
    return dma64_add(u64_to_dma64(dma_area_addr),
    offsetof(struct vcdev_dma_area, indicators));
    }
// Spec stipulates a 64 bit address
#[no_mangle]
pub unsafe extern "C" fn indicators2_dma(vcdev: *mut virtio_ccw_device) -> dma64_t {
    static inline dma64_t indicators2_dma(struct virtio_ccw_device *vcdev)
    {
    let mut dma_area_addr: u64 = dma32_to_u32(vcdev.dma_area_addr);
    return dma64_add(u64_to_dma64(dma_area_addr),
    offsetof(struct vcdev_dma_area, indicators2));
    }
#[no_mangle]
pub unsafe extern "C" fn config_block_dma(vcdev: *mut virtio_ccw_device) -> dma32_t {
    static inline dma32_t config_block_dma(struct virtio_ccw_device *vcdev)
    {
    return dma32_add(vcdev.dma_area_addr,
    offsetof(struct vcdev_dma_area, config_block));
    }
#[no_mangle]
pub unsafe extern "C" fn status_dma(vcdev: *mut virtio_ccw_device) -> dma32_t {
    static inline dma32_t status_dma(struct virtio_ccw_device *vcdev)
    {
    return dma32_add(vcdev.dma_area_addr,
    offsetof(struct vcdev_dma_area, status));
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vq_info_block_legacy {
    pub queue: dma64_t,
    pub align: __u32,
    pub index: __u16,
    pub num: __u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vq_info_block {
    pub desc: dma64_t,
    pub res0: __u32,
    pub index: __u16,
    pub num: __u16,
    pub avail: dma64_t,
    pub used: dma64_t,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_feature_desc {
    pub features: __le32,
    pub index: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_thinint_area {
    pub summary_indicator: dma64_t,
    pub indicator: dma64_t,
    pub bit_nr: u64,
    pub isc: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rev_info {
    pub revision: __u16,
    pub length: __u16,
    pub data: [__u8; ],
}

// the highest virtio-ccw revision we support
pub const VIRTIO_CCW_REV_MAX: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_ccw_vq_info {
    pub vq: *mut virtqueue,
    pub info_block_addr: dma32_t,
    pub num: c_int,
    union {
    pub s: vq_info_block,
    pub l: vq_info_block_legacy,
    pub info_block: *mut },
    pub bit_nr: c_int,
    pub node: list_head,
    pub cookie: c_long,
}

pub const MAX_AIRQ_AREAS: c_int = 20;
    let mut virtio_ccw_use_airq: static int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct airq_info {
    pub lock: rwlock_t,
    pub summary_indicator_idx: u8,
    pub airq: airq_struct,
    pub aiv: *mut airq_iv,
}

    static struct airq_info *airq_areas[MAX_AIRQ_AREAS];
    static DEFINE_MUTEX(airq_areas_lock);
    static u8 *summary_indicators;
    static inline u8 *get_summary_indicator(struct airq_info *info)
    {
    return summary_indicators + info.summary_indicator_idx;
    }
#[no_mangle]
pub unsafe extern "C" fn get_summary_indicator_dma(info: *mut airq_info) -> dma64_t {
    static inline dma64_t get_summary_indicator_dma(struct airq_info *info)
    {
    return virt_to_dma64(get_summary_indicator(info));
    }
pub const CCW_CMD_SET_VQ: c_uint = 0x13;
pub const CCW_CMD_VDEV_RESET: c_uint = 0x33;
pub const CCW_CMD_SET_IND: c_uint = 0x43;
pub const CCW_CMD_SET_CONF_IND: c_uint = 0x53;
pub const CCW_CMD_READ_FEAT: c_uint = 0x12;
pub const CCW_CMD_WRITE_FEAT: c_uint = 0x11;
pub const CCW_CMD_READ_CONF: c_uint = 0x22;
pub const CCW_CMD_WRITE_CONF: c_uint = 0x21;
pub const CCW_CMD_WRITE_STATUS: c_uint = 0x31;
pub const CCW_CMD_READ_VQ_CONF: c_uint = 0x32;
pub const CCW_CMD_READ_STATUS: c_uint = 0x72;
pub const CCW_CMD_SET_IND_ADAPTER: c_uint = 0x73;
pub const CCW_CMD_SET_VIRTIO_REV: c_uint = 0x83;
pub const VIRTIO_CCW_DOING_SET_VQ: c_uint = 0x00010000;
pub const VIRTIO_CCW_DOING_RESET: c_uint = 0x00040000;
pub const VIRTIO_CCW_DOING_READ_FEAT: c_uint = 0x00080000;
pub const VIRTIO_CCW_DOING_WRITE_FEAT: c_uint = 0x00100000;
pub const VIRTIO_CCW_DOING_READ_CONFIG: c_uint = 0x00200000;
pub const VIRTIO_CCW_DOING_WRITE_CONFIG: c_uint = 0x00400000;
pub const VIRTIO_CCW_DOING_WRITE_STATUS: c_uint = 0x00800000;
pub const VIRTIO_CCW_DOING_SET_IND: c_uint = 0x01000000;
pub const VIRTIO_CCW_DOING_READ_VQ_CONF: c_uint = 0x02000000;
pub const VIRTIO_CCW_DOING_SET_CONF_IND: c_uint = 0x04000000;
pub const VIRTIO_CCW_DOING_SET_IND_ADAPTER: c_uint = 0x08000000;
pub const VIRTIO_CCW_DOING_SET_VIRTIO_REV: c_uint = 0x10000000;
pub const VIRTIO_CCW_DOING_READ_STATUS: c_uint = 0x20000000;
pub const VIRTIO_CCW_INTPARM_MASK: c_uint = 0xffff0000;
    static struct virtio_ccw_device *to_vc_device(struct virtio_device *vdev)
    {
    return container_of(vdev, struct virtio_ccw_device, vdev);
    }
#[no_mangle]
unsafe extern "C" fn drop_airq_indicator(vq: *mut virtqueue, info: *mut airq_info) {
    static void drop_airq_indicator(struct virtqueue *vq, struct airq_info *info)
    {
    unsigned long i, flags;
    write_lock_irqsave(&info.lock, flags);
    for (i = 0; i < airq_iv_end(info.aiv); i++) {
    if (vq == (void *)airq_iv_get_ptr(info.aiv, i)) {
    airq_iv_free_bit(info.aiv, i);
    airq_iv_set_ptr(info.aiv, i, 0);
    break;
    }
    }
    write_unlock_irqrestore(&info.lock, flags);
    }
    static void virtio_airq_handler(struct airq_struct *airq,
    struct tpi_info *tpi_info)
    {
    struct airq_info *info = container_of(airq, struct airq_info, airq);
    unsigned long ai;
    inc_irq_stat(IRQIO_VAI);
    read_lock(&info.lock);
// Walk through indicators field, summary indicator active.
    for (ai = 0;;) {
    ai = airq_iv_scan(info.aiv, ai, airq_iv_end(info.aiv));
    if (ai == -1UL)
    break;
    vring_interrupt(0, (void *)airq_iv_get_ptr(info.aiv, ai));
    }
// (get_summary_indicator(info)) = 0;
    smp_wmb();
// Walk through indicators field, summary indicator not active.
    for (ai = 0;;) {
    ai = airq_iv_scan(info.aiv, ai, airq_iv_end(info.aiv));
    if (ai == -1UL)
    break;
    vring_interrupt(0, (void *)airq_iv_get_ptr(info.aiv, ai));
    }
    read_unlock(&info.lock);
    }
    static struct airq_info *new_airq_info(int index)
    {
    struct airq_info *info;
    int rc;
    info = kzalloc_obj(*info);
    if (!info)
    return core::ptr::null_mut();
    rwlock_init(&info.lock);
    info.aiv = airq_iv_create(VIRTIO_IV_BITS, AIRQ_IV_ALLOC | AIRQ_IV_PTR
    | AIRQ_IV_CACHELINE, core::ptr::null_mut());
    if (!info.aiv) {
    kfree(info);
    return core::ptr::null_mut();
    }
    info.airq.handler = virtio_airq_handler;
    info.summary_indicator_idx = index;
    info.airq.lsi_ptr = get_summary_indicator(info);
    info.airq.isc = VIRTIO_AIRQ_ISC;
    rc = register_adapter_interrupt(&info.airq);
    if (rc) {
    airq_iv_release(info.aiv);
    kfree(info);
    return core::ptr::null_mut();
    }
    return info;
    }
    static unsigned long *get_airq_indicator(struct virtqueue *vqs[], int nvqs,
    u64 *first, void **airq_info)
    {
    int i, j, queue_idx, highest_queue_idx = -1;
    struct airq_info *info;
    unsigned long *indicator_addr = core::ptr::null_mut();
    unsigned long bit, flags;
// Array entries without an actual queue pointer must be ignored.
    for (i = 0; i < nvqs; i++) {
    if (vqs[i])
    highest_queue_idx++;
    }
    for (i = 0; i < MAX_AIRQ_AREAS && !indicator_addr; i++) {
    mutex_lock(&airq_areas_lock);
    if (!airq_areas[i])
    airq_areas[i] = new_airq_info(i);
    info = airq_areas[i];
    mutex_unlock(&airq_areas_lock);
    if (!info)
    return core::ptr::null_mut();
    write_lock_irqsave(&info.lock, flags);
    bit = airq_iv_alloc(info.aiv, highest_queue_idx + 1);
    if (bit == -1UL) {
// Not enough vacancies.
    write_unlock_irqrestore(&info.lock, flags);
    continue;
    }
// first = bit;
// airq_info = info;
    indicator_addr = info.aiv.vector;
    for (j = 0, queue_idx = 0; j < nvqs; j++) {
    if (!vqs[j])
    continue;
    airq_iv_set_ptr(info.aiv, bit + queue_idx++,
    (unsigned long)vqs[j]);
    }
    write_unlock_irqrestore(&info.lock, flags);
    }
    return indicator_addr;
    }
#[no_mangle]
unsafe extern "C" fn virtio_ccw_drop_indicators(vcdev: *mut virtio_ccw_device) {
    static void virtio_ccw_drop_indicators(struct virtio_ccw_device *vcdev)
    {
    struct virtio_ccw_vq_info *info;
    if (!vcdev.airq_info)
    return;
    list_for_each_entry(info, &vcdev.virtqueues, node)
    drop_airq_indicator(info.vq, vcdev.airq_info);
    }
#[no_mangle]
unsafe extern "C" fn doing_io(vcdev: *mut virtio_ccw_device, flag: __u32) -> c_int {
    static int doing_io(struct virtio_ccw_device *vcdev, __u32 flag)
    {
    unsigned long flags;
    __u32 ret;
    spin_lock_irqsave(get_ccwdev_lock(vcdev.cdev), flags);
    if (vcdev.err)
    ret = 0;
    else
    ret = vcdev.curr_io & flag;
    spin_unlock_irqrestore(get_ccwdev_lock(vcdev.cdev), flags);
    return ret;
    }
    static int ccw_io_helper(struct virtio_ccw_device *vcdev,
    struct ccw1 *ccw, __u32 intparm)
    {
    int ret;
    unsigned long flags;
    let mut flag: c_int = intparm & VIRTIO_CCW_INTPARM_MASK;
    mutex_lock(&vcdev.io_lock);
    do {
    spin_lock_irqsave(get_ccwdev_lock(vcdev.cdev), flags);
    ret = ccw_device_start(vcdev.cdev, ccw, intparm, 0, 0);
    if (!ret) {
    if (!vcdev.curr_io)
    vcdev.err = 0;
    vcdev.curr_io |= flag;
    }
    spin_unlock_irqrestore(get_ccwdev_lock(vcdev.cdev), flags);
    cpu_relax();
    } while (ret == -EBUSY);
    wait_event(vcdev.wait_q, doing_io(vcdev, flag) == 0);
    ret = ret ? ret : vcdev.err;
    mutex_unlock(&vcdev.io_lock);
    return ret;
    }
    static void virtio_ccw_drop_indicator(struct virtio_ccw_device *vcdev,
    struct ccw1 *ccw)
    {
    int ret;
    struct virtio_thinint_area *thinint_area = core::ptr::null_mut();
    struct airq_info *airq_info = vcdev.airq_info;
    dma64_t *indicatorp = core::ptr::null_mut();
    if (vcdev.is_thinint) {
    thinint_area = ccw_device_dma_zalloc(vcdev.cdev,
    sizeof(*thinint_area),
    &ccw.cda);
    if (!thinint_area)
    return;
    thinint_area.summary_indicator =
    get_summary_indicator_dma(airq_info);
    thinint_area.isc = VIRTIO_AIRQ_ISC;
    ccw.cmd_code = CCW_CMD_SET_IND_ADAPTER;
    ccw.count = sizeof(*thinint_area);
    } else {
// payload is the address of the indicators
    indicatorp = ccw_device_dma_zalloc(vcdev.cdev,
    sizeof(*indicatorp),
    &ccw.cda);
    if (!indicatorp)
    return;
// indicatorp = 0;
    ccw.cmd_code = CCW_CMD_SET_IND;
    ccw.count = sizeof(*indicatorp);
    }
// Deregister indicators from host.
// indicators(vcdev) = 0;
    ccw.flags = 0;
    ret = ccw_io_helper(vcdev, ccw,
    vcdev.is_thinint ?
    VIRTIO_CCW_DOING_SET_IND_ADAPTER :
    VIRTIO_CCW_DOING_SET_IND);
    if (ret && (ret != -ENODEV))
    dev_info(&vcdev.cdev.dev,
    "Failed to deregister indicators (%d)\n", ret);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: vcdev->is_thinint) -> else {
    else if (vcdev.is_thinint)
    virtio_ccw_drop_indicators(vcdev);
    ccw_device_dma_free(vcdev.cdev, indicatorp, sizeof(*indicatorp));
    ccw_device_dma_free(vcdev.cdev, thinint_area, sizeof(*thinint_area));
    }
#[no_mangle]
pub unsafe extern "C" fn virtio_ccw_do_kvm_notify(vq: *mut virtqueue, data: u32) -> bool {
    static inline bool virtio_ccw_do_kvm_notify(struct virtqueue *vq, u32 data)
    {
    struct virtio_ccw_vq_info *info = vq.priv;
    struct virtio_ccw_device *vcdev;
    struct subchannel_id schid;
    vcdev = to_vc_device(info.vq.vdev);
    ccw_device_get_schid(vcdev.cdev, &schid);
    BUILD_BUG_ON(sizeof(struct subchannel_id) != sizeof(unsigned int));
    info.cookie = kvm_hypercall3(KVM_S390_VIRTIO_CCW_NOTIFY,
// ((unsigned int *)&schid),
    data, info.cookie);
    if (info.cookie < 0)
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn virtio_ccw_kvm_notify(vq: *mut virtqueue) -> bool {
    static bool virtio_ccw_kvm_notify(struct virtqueue *vq)
    {
    return virtio_ccw_do_kvm_notify(vq, vq.index);
    }
#[no_mangle]
unsafe extern "C" fn virtio_ccw_kvm_notify_with_data(vq: *mut virtqueue) -> bool {
    static bool virtio_ccw_kvm_notify_with_data(struct virtqueue *vq)
    {
    return virtio_ccw_do_kvm_notify(vq, vring_notification_data(vq));
    }
    static int virtio_ccw_read_vq_conf(struct virtio_ccw_device *vcdev,
    struct ccw1 *ccw, int index)
    {
    int ret;
    vcdev.dma_area.config_block.index = index;
    ccw.cmd_code = CCW_CMD_READ_VQ_CONF;
    ccw.flags = 0;
    ccw.count = sizeof(struct vq_config_block);
    ccw.cda = config_block_dma(vcdev);
    ret = ccw_io_helper(vcdev, ccw, VIRTIO_CCW_DOING_READ_VQ_CONF);
    if (ret)
    return ret;
    return vcdev.dma_area.config_block.num ?: -ENOENT;
    }
#[no_mangle]
unsafe extern "C" fn virtio_ccw_del_vq(vq: *mut virtqueue, ccw: *mut ccw1) {
    static void virtio_ccw_del_vq(struct virtqueue *vq, struct ccw1 *ccw)
    {
    struct virtio_ccw_device *vcdev = to_vc_device(vq.vdev);
    struct virtio_ccw_vq_info *info = vq.priv;
    unsigned long flags;
    int ret;
    let mut index: c_uint = vq.index;
// Remove from our list.
    spin_lock_irqsave(&vcdev.lock, flags);
    list_del(&info.node);
    spin_unlock_irqrestore(&vcdev.lock, flags);
// Release from host.
    if (vcdev.revision == 0) {
    info.info_block.l.queue = 0;
    info.info_block.l.align = 0;
    info.info_block.l.index = index;
    info.info_block.l.num = 0;
    ccw.count = sizeof(info.info_block.l);
    } else {
    info.info_block.s.desc = 0;
    info.info_block.s.index = index;
    info.info_block.s.num = 0;
    info.info_block.s.avail = 0;
    info.info_block.s.used = 0;
    ccw.count = sizeof(info.info_block.s);
    }
    ccw.cmd_code = CCW_CMD_SET_VQ;
    ccw.flags = 0;
    ccw.cda = info.info_block_addr;
    ret = ccw_io_helper(vcdev, ccw,
    VIRTIO_CCW_DOING_SET_VQ | index);
//
// -ENODEV isn't considered an error: The device is gone anyway.
// This may happen on device detach.
//
    if (ret && (ret != -ENODEV))
    dev_warn(&vq.vdev.dev, "Error %d while deleting queue %d\n",
    ret, index);
    vring_del_virtqueue(vq);
    ccw_device_dma_free(vcdev.cdev, info.info_block,
    sizeof(*info.info_block));
    kfree(info);
    }
#[no_mangle]
unsafe extern "C" fn virtio_ccw_del_vqs(vdev: *mut virtio_device) {
    static void virtio_ccw_del_vqs(struct virtio_device *vdev)
    {
    struct virtqueue *vq, *n;
    struct ccw1 *ccw;
    struct virtio_ccw_device *vcdev = to_vc_device(vdev);
    ccw = ccw_device_dma_zalloc(vcdev.cdev, sizeof(*ccw), core::ptr::null_mut());
    if (!ccw)
    return;
    virtio_ccw_drop_indicator(vcdev, ccw);
    list_for_each_entry_safe(vq, n, &vdev.vqs, list)
    virtio_ccw_del_vq(vq, ccw);
    ccw_device_dma_free(vcdev.cdev, ccw, sizeof(*ccw));
    }
    static struct virtqueue *virtio_ccw_setup_vq(struct virtio_device *vdev,
    int i, vq_callback_t *callback,
    const char *name, bool ctx,
    struct ccw1 *ccw)
    {
    struct virtio_ccw_device *vcdev = to_vc_device(vdev);
    bool (*notify)(struct virtqueue *vq);
    int err;
    struct virtqueue *vq = core::ptr::null_mut();
    struct virtio_ccw_vq_info *info;
    u64 queue;
    unsigned long flags;
    bool may_reduce;
    if (__virtio_test_bit(vdev, VIRTIO_F_NOTIFICATION_DATA))
    notify = virtio_ccw_kvm_notify_with_data;
    else
    notify = virtio_ccw_kvm_notify;
// Allocate queue.
    info = kzalloc_obj(struct virtio_ccw_vq_info);
    if (!info) {
    dev_warn(&vcdev.cdev.dev, "no info\n");
    err = -ENOMEM;
    goto out_err;
    }
    info.info_block = ccw_device_dma_zalloc(vcdev.cdev,
    sizeof(*info.info_block),
    &info.info_block_addr);
    if (!info.info_block) {
    dev_warn(&vcdev.cdev.dev, "no info block\n");
    err = -ENOMEM;
    goto out_err;
    }
    info.num = virtio_ccw_read_vq_conf(vcdev, ccw, i);
    if (info.num < 0) {
    err = info.num;
    goto out_err;
    }
    may_reduce = vcdev.revision > 0;
    vq = vring_create_virtqueue(i, info.num, KVM_VIRTIO_CCW_RING_ALIGN,
    vdev, true, may_reduce, ctx,
    notify, callback, name);
    if (!vq) {
// For now, we fail if we can't get the requested size.
    dev_warn(&vcdev.cdev.dev, "no vq\n");
    err = -ENOMEM;
    goto out_err;
    }
    vq.num_max = info.num;
// it may have been reduced
    info.num = virtqueue_get_vring_size(vq);
// Register it with the host.
    queue = virtqueue_get_desc_addr(vq);
    if (vcdev.revision == 0) {
    info.info_block.l.queue = u64_to_dma64(queue);
    info.info_block.l.align = KVM_VIRTIO_CCW_RING_ALIGN;
    info.info_block.l.index = i;
    info.info_block.l.num = info.num;
    ccw.count = sizeof(info.info_block.l);
    } else {
    info.info_block.s.desc = u64_to_dma64(queue);
    info.info_block.s.index = i;
    info.info_block.s.num = info.num;
    info.info_block.s.avail = u64_to_dma64(virtqueue_get_avail_addr(vq));
    info.info_block.s.used = u64_to_dma64(virtqueue_get_used_addr(vq));
    ccw.count = sizeof(info.info_block.s);
    }
    ccw.cmd_code = CCW_CMD_SET_VQ;
    ccw.flags = 0;
    ccw.cda = info.info_block_addr;
    err = ccw_io_helper(vcdev, ccw, VIRTIO_CCW_DOING_SET_VQ | i);
    if (err) {
    dev_warn(&vcdev.cdev.dev, "SET_VQ failed\n");
    goto out_err;
    }
    info.vq = vq;
    vq.priv = info;
// Save it to our list.
    spin_lock_irqsave(&vcdev.lock, flags);
    list_add(&info.node, &vcdev.virtqueues);
    spin_unlock_irqrestore(&vcdev.lock, flags);
    return vq;
    out_err:
    if (vq)
    vring_del_virtqueue(vq);
    if (info) {
    ccw_device_dma_free(vcdev.cdev, info.info_block,
    sizeof(*info.info_block));
    }
    kfree(info);
    return ERR_PTR(err);
    }
    static int virtio_ccw_register_adapter_ind(struct virtio_ccw_device *vcdev,
    struct virtqueue *vqs[], int nvqs,
    struct ccw1 *ccw)
    {
    int ret;
    struct virtio_thinint_area *thinint_area = core::ptr::null_mut();
    unsigned long *indicator_addr;
    struct airq_info *info;
    thinint_area = ccw_device_dma_zalloc(vcdev.cdev,
    sizeof(*thinint_area),
    &ccw.cda);
    if (!thinint_area) {
    ret = -ENOMEM;
    goto out;
    }
// Try to get an indicator.
    indicator_addr = get_airq_indicator(vqs, nvqs,
    &thinint_area.bit_nr,
    &vcdev.airq_info);
    if (!indicator_addr) {
    ret = -ENOSPC;
    goto out;
    }
    thinint_area.indicator = virt_to_dma64(indicator_addr);
    info = vcdev.airq_info;
    thinint_area.summary_indicator = get_summary_indicator_dma(info);
    thinint_area.isc = VIRTIO_AIRQ_ISC;
    ccw.cmd_code = CCW_CMD_SET_IND_ADAPTER;
    ccw.flags = CCW_FLAG_SLI;
    ccw.count = sizeof(*thinint_area);
    ret = ccw_io_helper(vcdev, ccw, VIRTIO_CCW_DOING_SET_IND_ADAPTER);
    if (ret) {
    if (ret == -EOPNOTSUPP) {
//
// The host does not support adapter interrupts
// for virtio-ccw, stop trying.
//
    virtio_ccw_use_airq = 0;
    pr_info("Adapter interrupts unsupported on host\n");
    } else
    dev_warn(&vcdev.cdev.dev,
    "enabling adapter interrupts = %d\n", ret);
    virtio_ccw_drop_indicators(vcdev);
    }
    out:
    ccw_device_dma_free(vcdev.cdev, thinint_area, sizeof(*thinint_area));
    return ret;
    }
    static int virtio_ccw_find_vqs(struct virtio_device *vdev, unsigned nvqs,
    struct virtqueue *vqs[],
    struct virtqueue_info vqs_info[],
    struct irq_affinity *desc)
    {
    struct virtio_ccw_device *vcdev = to_vc_device(vdev);
    dma64_t *indicatorp = core::ptr::null_mut();
    int ret, i, queue_idx = 0;
    struct ccw1 *ccw;
    let mut indicatorp_dma: dma32_t = 0;
    ccw = ccw_device_dma_zalloc(vcdev.cdev, sizeof(*ccw), core::ptr::null_mut());
    if (!ccw)
    return -ENOMEM;
    for (i = 0; i < nvqs; ++i) {
    struct virtqueue_info *vqi = &vqs_info[i];
    if (!vqi.name) {
    vqs[i] = core::ptr::null_mut();
    continue;
    }
    vqs[i] = virtio_ccw_setup_vq(vdev, queue_idx++, vqi.callback,
    vqi.name, vqi.ctx, ccw);
    if (IS_ERR(vqs[i])) {
    ret = PTR_ERR(vqs[i]);
    vqs[i] = core::ptr::null_mut();
    goto out;
    }
    }
    ret = -ENOMEM;
//
// We need a data area under 2G to communicate. Our payload is
// the address of the indicators.
//
    indicatorp = ccw_device_dma_zalloc(vcdev.cdev,
    sizeof(*indicatorp),
    &indicatorp_dma);
    if (!indicatorp)
    goto out;
// indicatorp = indicators_dma(vcdev);
    if (vcdev.is_thinint) {
    ret = virtio_ccw_register_adapter_ind(vcdev, vqs, nvqs, ccw);
    if (ret)
// no error, just fall back to legacy interrupts
    vcdev.is_thinint = false;
    }
    ccw.cda = indicatorp_dma;
    if (!vcdev.is_thinint) {
// Register queue indicators with host.
// indicators(vcdev) = 0;
    ccw.cmd_code = CCW_CMD_SET_IND;
    ccw.flags = 0;
    ccw.count = sizeof(*indicatorp);
    ret = ccw_io_helper(vcdev, ccw, VIRTIO_CCW_DOING_SET_IND);
    if (ret)
    goto out;
    }
// Register indicators2 with host for config changes
// indicatorp = indicators2_dma(vcdev);
// indicators2(vcdev) = 0;
    ccw.cmd_code = CCW_CMD_SET_CONF_IND;
    ccw.flags = 0;
    ccw.count = sizeof(*indicatorp);
    ret = ccw_io_helper(vcdev, ccw, VIRTIO_CCW_DOING_SET_CONF_IND);
    if (ret)
    goto out;
    if (indicatorp)
    ccw_device_dma_free(vcdev.cdev, indicatorp,
    sizeof(*indicatorp));
    ccw_device_dma_free(vcdev.cdev, ccw, sizeof(*ccw));
    return 0;
    out:
    if (indicatorp)
    ccw_device_dma_free(vcdev.cdev, indicatorp,
    sizeof(*indicatorp));
    ccw_device_dma_free(vcdev.cdev, ccw, sizeof(*ccw));
    virtio_ccw_del_vqs(vdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn virtio_ccw_reset(vdev: *mut virtio_device) {
    static void virtio_ccw_reset(struct virtio_device *vdev)
    {
    struct virtio_ccw_device *vcdev = to_vc_device(vdev);
    struct ccw1 *ccw;
    ccw = ccw_device_dma_zalloc(vcdev.cdev, sizeof(*ccw), core::ptr::null_mut());
    if (!ccw)
    return;
// Zero status bits.
    vcdev.dma_area.status = 0;
// Send a reset ccw on device.
    ccw.cmd_code = CCW_CMD_VDEV_RESET;
    ccw.flags = 0;
    ccw.count = 0;
    ccw.cda = 0;
    ccw_io_helper(vcdev, ccw, VIRTIO_CCW_DOING_RESET);
    ccw_device_dma_free(vcdev.cdev, ccw, sizeof(*ccw));
    }
#[no_mangle]
unsafe extern "C" fn virtio_ccw_get_features(vdev: *mut virtio_device) -> u64 {
    static u64 virtio_ccw_get_features(struct virtio_device *vdev)
    {
    struct virtio_ccw_device *vcdev = to_vc_device(vdev);
    struct virtio_feature_desc *features;
    int ret;
    u64 rc;
    struct ccw1 *ccw;
    ccw = ccw_device_dma_zalloc(vcdev.cdev, sizeof(*ccw), core::ptr::null_mut());
    if (!ccw)
    return 0;
    features = ccw_device_dma_zalloc(vcdev.cdev, sizeof(*features),
    &ccw.cda);
    if (!features) {
    rc = 0;
    goto out_free;
    }
// Read the feature bits from the host.
    features.index = 0;
    ccw.cmd_code = CCW_CMD_READ_FEAT;
    ccw.flags = 0;
    ccw.count = sizeof(*features);
    ret = ccw_io_helper(vcdev, ccw, VIRTIO_CCW_DOING_READ_FEAT);
    if (ret) {
    rc = 0;
    goto out_free;
    }
    rc = le32_to_cpu(features.features);
    if (vcdev.revision == 0)
    goto out_free;
// Read second half of the feature bits from the host.
    features.index = 1;
    ccw.cmd_code = CCW_CMD_READ_FEAT;
    ccw.flags = 0;
    ccw.count = sizeof(*features);
    ret = ccw_io_helper(vcdev, ccw, VIRTIO_CCW_DOING_READ_FEAT);
    if (ret == 0)
    rc |= (u64)le32_to_cpu(features.features) << 32;
    out_free:
    ccw_device_dma_free(vcdev.cdev, features, sizeof(*features));
    ccw_device_dma_free(vcdev.cdev, ccw, sizeof(*ccw));
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn ccw_transport_features(vdev: *mut virtio_device) {
    static void ccw_transport_features(struct virtio_device *vdev)
    {
//
// Currently nothing to do here.
//
    }
#[no_mangle]
unsafe extern "C" fn virtio_ccw_finalize_features(vdev: *mut virtio_device) -> c_int {
    static int virtio_ccw_finalize_features(struct virtio_device *vdev)
    {
    struct virtio_ccw_device *vcdev = to_vc_device(vdev);
    struct virtio_feature_desc *features;
    struct ccw1 *ccw;
    int ret;
    if (vcdev.revision >= 1 &&
    !__virtio_test_bit(vdev, VIRTIO_F_VERSION_1)) {
    dev_err(&vdev.dev, "virtio: device uses revision 1 "
    "but does not have VIRTIO_F_VERSION_1\n");
    return -EINVAL;
    }
    ccw = ccw_device_dma_zalloc(vcdev.cdev, sizeof(*ccw), core::ptr::null_mut());
    if (!ccw)
    return -ENOMEM;
    features = ccw_device_dma_zalloc(vcdev.cdev, sizeof(*features),
    &ccw.cda);
    if (!features) {
    ret = -ENOMEM;
    goto out_free;
    }
// Give virtio_ring a chance to accept features.
    vring_transport_features(vdev);
// Give virtio_ccw a chance to accept features.
    ccw_transport_features(vdev);
    features.index = 0;
    features.features = cpu_to_le32((u32)vdev.features);
// Write the first half of the feature bits to the host.
    ccw.cmd_code = CCW_CMD_WRITE_FEAT;
    ccw.flags = 0;
    ccw.count = sizeof(*features);
    ret = ccw_io_helper(vcdev, ccw, VIRTIO_CCW_DOING_WRITE_FEAT);
    if (ret)
    goto out_free;
    if (vcdev.revision == 0)
    goto out_free;
    features.index = 1;
    features.features = cpu_to_le32(vdev.features >> 32);
// Write the second half of the feature bits to the host.
    ccw.cmd_code = CCW_CMD_WRITE_FEAT;
    ccw.flags = 0;
    ccw.count = sizeof(*features);
    ret = ccw_io_helper(vcdev, ccw, VIRTIO_CCW_DOING_WRITE_FEAT);
    out_free:
    ccw_device_dma_free(vcdev.cdev, features, sizeof(*features));
    ccw_device_dma_free(vcdev.cdev, ccw, sizeof(*ccw));
    return ret;
    }
    static void virtio_ccw_get_config(struct virtio_device *vdev,
    unsigned int offset, void *buf, unsigned len)
    {
    struct virtio_ccw_device *vcdev = to_vc_device(vdev);
    int ret;
    struct ccw1 *ccw;
    void *config_area;
    unsigned long flags;
    ccw = ccw_device_dma_zalloc(vcdev.cdev, sizeof(*ccw), core::ptr::null_mut());
    if (!ccw)
    return;
    config_area = ccw_device_dma_zalloc(vcdev.cdev,
    VIRTIO_CCW_CONFIG_SIZE,
    &ccw.cda);
    if (!config_area)
    goto out_free;
// Read the config area from the host.
    ccw.cmd_code = CCW_CMD_READ_CONF;
    ccw.flags = 0;
    ccw.count = offset + len;
    ret = ccw_io_helper(vcdev, ccw, VIRTIO_CCW_DOING_READ_CONFIG);
    if (ret)
    goto out_free;
    spin_lock_irqsave(&vcdev.lock, flags);
    memcpy(vcdev.config, config_area, offset + len);
    if (vcdev.config_ready < offset + len)
    vcdev.config_ready = offset + len;
    spin_unlock_irqrestore(&vcdev.lock, flags);
    if (buf)
    memcpy(buf, config_area + offset, len);
    out_free:
    ccw_device_dma_free(vcdev.cdev, config_area, VIRTIO_CCW_CONFIG_SIZE);
    ccw_device_dma_free(vcdev.cdev, ccw, sizeof(*ccw));
    }
    static void virtio_ccw_set_config(struct virtio_device *vdev,
    unsigned int offset, const void *buf,
    unsigned len)
    {
    struct virtio_ccw_device *vcdev = to_vc_device(vdev);
    struct ccw1 *ccw;
    void *config_area;
    unsigned long flags;
    ccw = ccw_device_dma_zalloc(vcdev.cdev, sizeof(*ccw), core::ptr::null_mut());
    if (!ccw)
    return;
    config_area = ccw_device_dma_zalloc(vcdev.cdev,
    VIRTIO_CCW_CONFIG_SIZE,
    &ccw.cda);
    if (!config_area)
    goto out_free;
// Make sure we don't overwrite fields.
    if (vcdev.config_ready < offset)
    virtio_ccw_get_config(vdev, 0, core::ptr::null_mut(), offset);
    spin_lock_irqsave(&vcdev.lock, flags);
    memcpy(&vcdev.config[offset], buf, len);
// Write the config area to the host.
    memcpy(config_area, vcdev.config, sizeof(vcdev.config));
    spin_unlock_irqrestore(&vcdev.lock, flags);
    ccw.cmd_code = CCW_CMD_WRITE_CONF;
    ccw.flags = 0;
    ccw.count = offset + len;
    ccw_io_helper(vcdev, ccw, VIRTIO_CCW_DOING_WRITE_CONFIG);
    out_free:
    ccw_device_dma_free(vcdev.cdev, config_area, VIRTIO_CCW_CONFIG_SIZE);
    ccw_device_dma_free(vcdev.cdev, ccw, sizeof(*ccw));
    }
#[no_mangle]
unsafe extern "C" fn virtio_ccw_get_status(vdev: *mut virtio_device) -> u8 {
    static u8 virtio_ccw_get_status(struct virtio_device *vdev)
    {
    struct virtio_ccw_device *vcdev = to_vc_device(vdev);
    let mut old_status: u8 = vcdev.dma_area.status;
    struct ccw1 *ccw;
    if (vcdev.revision < 2)
    return vcdev.dma_area.status;
    ccw = ccw_device_dma_zalloc(vcdev.cdev, sizeof(*ccw), core::ptr::null_mut());
    if (!ccw)
    return old_status;
    ccw.cmd_code = CCW_CMD_READ_STATUS;
    ccw.flags = 0;
    ccw.count = sizeof(vcdev.dma_area.status);
    ccw.cda = status_dma(vcdev);
    ccw_io_helper(vcdev, ccw, VIRTIO_CCW_DOING_READ_STATUS);
//
// If the channel program failed (should only happen if the device
// was hotunplugged, and then we clean up via the machine check
// handler anyway), vcdev->dma_area->status was not overwritten and we just
// return the old status, which is fine.
//
    ccw_device_dma_free(vcdev.cdev, ccw, sizeof(*ccw));
    return vcdev.dma_area.status;
    }
#[no_mangle]
unsafe extern "C" fn virtio_ccw_set_status(vdev: *mut virtio_device, status: u8) {
    static void virtio_ccw_set_status(struct virtio_device *vdev, u8 status)
    {
    struct virtio_ccw_device *vcdev = to_vc_device(vdev);
    let mut old_status: u8 = vcdev.dma_area.status;
    struct ccw1 *ccw;
    int ret;
    ccw = ccw_device_dma_zalloc(vcdev.cdev, sizeof(*ccw), core::ptr::null_mut());
    if (!ccw)
    return;
// Write the status to the host.
    vcdev.dma_area.status = status;
    ccw.cmd_code = CCW_CMD_WRITE_STATUS;
    ccw.flags = 0;
    ccw.count = sizeof(status);
// We use ssch for setting the status which is a serializing
// instruction that guarantees the memory writes have
// completed before ssch.
//
    ccw.cda = status_dma(vcdev);
    ret = ccw_io_helper(vcdev, ccw, VIRTIO_CCW_DOING_WRITE_STATUS);
// Write failed? We assume status is unchanged.
    if (ret)
    vcdev.dma_area.status = old_status;
    ccw_device_dma_free(vcdev.cdev, ccw, sizeof(*ccw));
    }
    static const char *virtio_ccw_bus_name(struct virtio_device *vdev)
    {
    struct virtio_ccw_device *vcdev = to_vc_device(vdev);
    return dev_name(&vcdev.cdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn virtio_ccw_synchronize_cbs(vdev: *mut virtio_device) {
    static void virtio_ccw_synchronize_cbs(struct virtio_device *vdev)
    {
    struct virtio_ccw_device *vcdev = to_vc_device(vdev);
    struct airq_info *info = vcdev.airq_info;
    if (info) {
//
// This device uses adapter interrupts: synchronize with
// vring_interrupt() called by virtio_airq_handler()
// via the indicator area lock.
//
    write_lock_irq(&info.lock);
    write_unlock_irq(&info.lock);
    } else {
// This device uses classic interrupts: synchronize
// with vring_interrupt() called by
// virtio_ccw_int_handler() via the per-device
// irq_lock
//
    write_lock_irq(&vcdev.irq_lock);
    write_unlock_irq(&vcdev.irq_lock);
    }
    }
    static const struct virtio_config_ops virtio_ccw_config_ops = {
    .get_features = virtio_ccw_get_features,
    .finalize_features = virtio_ccw_finalize_features,
    .get = virtio_ccw_get_config,
    .set = virtio_ccw_set_config,
    .get_status = virtio_ccw_get_status,
    .set_status = virtio_ccw_set_status,
    .reset = virtio_ccw_reset,
    .find_vqs = virtio_ccw_find_vqs,
    .del_vqs = virtio_ccw_del_vqs,
    .bus_name = virtio_ccw_bus_name,
    .synchronize_cbs = virtio_ccw_synchronize_cbs,
    };
//
// ccw bus driver related functions
//
#[no_mangle]
unsafe extern "C" fn virtio_ccw_release_dev(_d: *mut device) {
    static void virtio_ccw_release_dev(struct device *_d)
    {
    struct virtio_device *dev = dev_to_virtio(_d);
    struct virtio_ccw_device *vcdev = to_vc_device(dev);
    ccw_device_dma_free(vcdev.cdev, vcdev.dma_area,
    sizeof(*vcdev.dma_area));
    kfree(vcdev);
    }
#[no_mangle]
unsafe extern "C" fn irb_is_error(irb: *mut irb) -> c_int {
    static int irb_is_error(struct irb *irb)
    {
    if (scsw_cstat(&irb.scsw) != 0)
    return 1;
    if (scsw_dstat(&irb.scsw) & ~(DEV_STAT_CHN_END | DEV_STAT_DEV_END))
    return 1;
    if (scsw_cc(&irb.scsw) != 0)
    return 1;
    return 0;
    }
    static struct virtqueue *virtio_ccw_vq_by_ind(struct virtio_ccw_device *vcdev,
    int index)
    {
    struct virtio_ccw_vq_info *info;
    unsigned long flags;
    struct virtqueue *vq;
    vq = core::ptr::null_mut();
    spin_lock_irqsave(&vcdev.lock, flags);
    list_for_each_entry(info, &vcdev.virtqueues, node) {
    if (info.vq.index == index) {
    vq = info.vq;
    break;
    }
    }
    spin_unlock_irqrestore(&vcdev.lock, flags);
    return vq;
    }
    static void virtio_ccw_check_activity(struct virtio_ccw_device *vcdev,
    __u32 activity)
    {
    if (vcdev.curr_io & activity) {
    switch (activity) {
    case VIRTIO_CCW_DOING_READ_FEAT:
    case VIRTIO_CCW_DOING_WRITE_FEAT:
    case VIRTIO_CCW_DOING_READ_CONFIG:
    case VIRTIO_CCW_DOING_WRITE_CONFIG:
    case VIRTIO_CCW_DOING_WRITE_STATUS:
    case VIRTIO_CCW_DOING_READ_STATUS:
    case VIRTIO_CCW_DOING_SET_VQ:
    case VIRTIO_CCW_DOING_SET_IND:
    case VIRTIO_CCW_DOING_SET_CONF_IND:
    case VIRTIO_CCW_DOING_RESET:
    case VIRTIO_CCW_DOING_READ_VQ_CONF:
    case VIRTIO_CCW_DOING_SET_IND_ADAPTER:
    case VIRTIO_CCW_DOING_SET_VIRTIO_REV:
    vcdev.curr_io &= ~activity;
    wake_up(&vcdev.wait_q);
    break;
    default:
// don't know what to do...
    dev_warn(&vcdev.cdev.dev,
    "Suspicious activity '%08x'\n", activity);
    WARN_ON(1);
    break;
    }
    }
    }
    static void virtio_ccw_int_handler(struct ccw_device *cdev,
    unsigned long intparm,
    struct irb *irb)
    {
    let mut activity: __u32 = intparm & VIRTIO_CCW_INTPARM_MASK;
    struct virtio_ccw_device *vcdev = dev_get_drvdata(&cdev.dev);
    int i;
    struct virtqueue *vq;
    if (!vcdev)
    return;
    if (IS_ERR(irb)) {
    vcdev.err = PTR_ERR(irb);
    virtio_ccw_check_activity(vcdev, activity);
// Don't poke around indicators, something's wrong.
    return;
    }
// Check if it's a notification from the host.
    if ((intparm == 0) &&
    (scsw_stctl(&irb.scsw) ==
    (SCSW_STCTL_ALERT_STATUS | SCSW_STCTL_STATUS_PEND))) {
// OK
    }
    if (irb_is_error(irb)) {
// Command reject?
    if ((scsw_dstat(&irb.scsw) & DEV_STAT_UNIT_CHECK) &&
    (irb.ecw[0] & SNS0_CMD_REJECT))
    vcdev.err = -EOPNOTSUPP;
    else
// Map everything else to -EIO.
    vcdev.err = -EIO;
    }
    virtio_ccw_check_activity(vcdev, activity);

//
// Paired with virtio_ccw_synchronize_cbs() and interrupts are
// disabled here.
//
    read_lock(&vcdev.irq_lock);

    for_each_set_bit(i, indicators(vcdev),
    sizeof(*indicators(vcdev)) * BITS_PER_BYTE) {
// The bit clear must happen before the vring kick.
    clear_bit(i, indicators(vcdev));
    barrier();
    vq = virtio_ccw_vq_by_ind(vcdev, i);
    vring_interrupt(0, vq);
    }

    read_unlock(&vcdev.irq_lock);

    if (test_bit(0, indicators2(vcdev))) {
    virtio_config_changed(&vcdev.vdev);
    clear_bit(0, indicators2(vcdev));
    }
    }
//
// We usually want to autoonline all devices, but give the admin
// a way to exempt devices from this.
//

    (8*sizeof(long)))
    static unsigned long devs_no_auto[__MAX_SSID + 1][__DEV_WORDS];
    static char *no_auto = "";
    module_param(no_auto, charp, 0444);
    MODULE_PARM_DESC(no_auto, "list of ccw bus id ranges not to be auto-onlined");
#[no_mangle]
unsafe extern "C" fn virtio_ccw_check_autoonline(cdev: *mut ccw_device) -> c_int {
    static int virtio_ccw_check_autoonline(struct ccw_device *cdev)
    {
    struct ccw_dev_id id;
    ccw_device_get_id(cdev, &id);
    if (test_bit(id.devno, devs_no_auto[id.ssid]))
    return 0;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn virtio_ccw_auto_online(data: *mut c_void, cookie: async_cookie_t) {
    static void virtio_ccw_auto_online(void *data, async_cookie_t cookie)
    {
    struct ccw_device *cdev = data;
    int ret;
    ret = ccw_device_set_online(cdev);
    if (ret)
    dev_warn(&cdev.dev, "Failed to set online: %d\n", ret);
    }
#[no_mangle]
unsafe extern "C" fn virtio_ccw_probe(cdev: *mut ccw_device) -> c_int {
    static int virtio_ccw_probe(struct ccw_device *cdev)
    {
    cdev.handler = virtio_ccw_int_handler;
    if (virtio_ccw_check_autoonline(cdev))
    async_schedule(virtio_ccw_auto_online, cdev);
    return 0;
    }
    static struct virtio_ccw_device *virtio_grab_drvdata(struct ccw_device *cdev)
    {
    unsigned long flags;
    struct virtio_ccw_device *vcdev;
    spin_lock_irqsave(get_ccwdev_lock(cdev), flags);
    vcdev = dev_get_drvdata(&cdev.dev);
    if (!vcdev || vcdev.going_away) {
    spin_unlock_irqrestore(get_ccwdev_lock(cdev), flags);
    return core::ptr::null_mut();
    }
    vcdev.going_away = true;
    spin_unlock_irqrestore(get_ccwdev_lock(cdev), flags);
    return vcdev;
    }
#[no_mangle]
unsafe extern "C" fn virtio_ccw_remove(cdev: *mut ccw_device) {
    static void virtio_ccw_remove(struct ccw_device *cdev)
    {
    unsigned long flags;
    struct virtio_ccw_device *vcdev = virtio_grab_drvdata(cdev);
    if (vcdev && cdev.online) {
    if (vcdev.device_lost)
    virtio_break_device(&vcdev.vdev);
    unregister_virtio_device(&vcdev.vdev);
    spin_lock_irqsave(get_ccwdev_lock(cdev), flags);
    dev_set_drvdata(&cdev.dev, core::ptr::null_mut());
    spin_unlock_irqrestore(get_ccwdev_lock(cdev), flags);
    }
    cdev.handler = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn virtio_ccw_offline(cdev: *mut ccw_device) -> c_int {
    static int virtio_ccw_offline(struct ccw_device *cdev)
    {
    unsigned long flags;
    struct virtio_ccw_device *vcdev = virtio_grab_drvdata(cdev);
    if (!vcdev)
    return 0;
    if (vcdev.device_lost)
    virtio_break_device(&vcdev.vdev);
    unregister_virtio_device(&vcdev.vdev);
    spin_lock_irqsave(get_ccwdev_lock(cdev), flags);
    dev_set_drvdata(&cdev.dev, core::ptr::null_mut());
    cdev.dev.dma_parms = core::ptr::null_mut();
    spin_unlock_irqrestore(get_ccwdev_lock(cdev), flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtio_ccw_set_transport_rev(vcdev: *mut virtio_ccw_device) -> c_int {
    static int virtio_ccw_set_transport_rev(struct virtio_ccw_device *vcdev)
    {
    struct virtio_rev_info *rev;
    struct ccw1 *ccw;
    int ret;
    ccw = ccw_device_dma_zalloc(vcdev.cdev, sizeof(*ccw), core::ptr::null_mut());
    if (!ccw)
    return -ENOMEM;
    rev = ccw_device_dma_zalloc(vcdev.cdev, sizeof(*rev), &ccw.cda);
    if (!rev) {
    ccw_device_dma_free(vcdev.cdev, ccw, sizeof(*ccw));
    return -ENOMEM;
    }
// Set transport revision
    ccw.cmd_code = CCW_CMD_SET_VIRTIO_REV;
    ccw.flags = 0;
    ccw.count = sizeof(*rev);
    vcdev.revision = VIRTIO_CCW_REV_MAX;
    do {
    rev.revision = vcdev.revision;
// none of our supported revisions carry payload
    rev.length = 0;
    ret = ccw_io_helper(vcdev, ccw,
    VIRTIO_CCW_DOING_SET_VIRTIO_REV);
    if (ret == -EOPNOTSUPP) {
    if (vcdev.revision == 0)
//
// The host device does not support setting
// the revision: let's operate it in legacy
// mode.
//
    ret = 0;
    else
    vcdev.revision--;
    }
    } while (ret == -EOPNOTSUPP);
    ccw_device_dma_free(vcdev.cdev, ccw, sizeof(*ccw));
    ccw_device_dma_free(vcdev.cdev, rev, sizeof(*rev));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn virtio_ccw_online(cdev: *mut ccw_device) -> c_int {
    static int virtio_ccw_online(struct ccw_device *cdev)
    {
    int ret;
    struct virtio_ccw_device *vcdev;
    unsigned long flags;
    vcdev = kzalloc_obj(*vcdev);
    if (!vcdev) {
    dev_warn(&cdev.dev, "Could not get memory for virtio\n");
    ret = -ENOMEM;
    goto out_free;
    }
    vcdev.vdev.dev.parent = &cdev.dev;
    vcdev.cdev = cdev;
    cdev.dev.dma_parms = &vcdev.dma_parms;
    vcdev.dma_area = ccw_device_dma_zalloc(vcdev.cdev,
    sizeof(*vcdev.dma_area),
    &vcdev.dma_area_addr);
    if (!vcdev.dma_area) {
    ret = -ENOMEM;
    goto out_free;
    }
    vcdev.is_thinint = virtio_ccw_use_airq; /* at least try */
    vcdev.vdev.dev.release = virtio_ccw_release_dev;
    vcdev.vdev.config = &virtio_ccw_config_ops;
    init_waitqueue_head(&vcdev.wait_q);
    INIT_LIST_HEAD(&vcdev.virtqueues);
    spin_lock_init(&vcdev.lock);
    rwlock_init(&vcdev.irq_lock);
    mutex_init(&vcdev.io_lock);
    spin_lock_irqsave(get_ccwdev_lock(cdev), flags);
    dev_set_drvdata(&cdev.dev, vcdev);
    spin_unlock_irqrestore(get_ccwdev_lock(cdev), flags);
    vcdev.vdev.id.vendor = cdev.id.cu_type;
    vcdev.vdev.id.device = cdev.id.cu_model;
    ret = virtio_ccw_set_transport_rev(vcdev);
    if (ret)
    goto out_free;
    ret = register_virtio_device(&vcdev.vdev);
    if (ret) {
    dev_warn(&cdev.dev, "Failed to register virtio device: %d\n",
    ret);
    goto out_put;
    }
    return 0;
    out_put:
    spin_lock_irqsave(get_ccwdev_lock(cdev), flags);
    dev_set_drvdata(&cdev.dev, core::ptr::null_mut());
    spin_unlock_irqrestore(get_ccwdev_lock(cdev), flags);
    put_device(&vcdev.vdev.dev);
    return ret;
    out_free:
    if (vcdev) {
    ccw_device_dma_free(vcdev.cdev, vcdev.dma_area,
    sizeof(*vcdev.dma_area));
    }
    kfree(vcdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn virtio_ccw_cio_notify(cdev: *mut ccw_device, event: c_int) -> c_int {
    static int virtio_ccw_cio_notify(struct ccw_device *cdev, int event)
    {
    int rc;
    struct virtio_ccw_device *vcdev = dev_get_drvdata(&cdev.dev);
//
// Make sure vcdev is set
// i.e. set_offline/remove callback not already running
//
    if (!vcdev)
    return NOTIFY_DONE;
    switch (event) {
    case CIO_GONE:
    vcdev.device_lost = true;
    rc = NOTIFY_DONE;
    break;
    case CIO_OPER:
    rc = NOTIFY_OK;
    break;
    default:
    rc = NOTIFY_DONE;
    break;
    }
    return rc;
    }
    static struct ccw_device_id virtio_ids[] = {
    { CCW_DEVICE(0x3832, 0) },
    {},
    };
    static struct ccw_driver virtio_ccw_driver = {
    .driver = {
    .owner = THIS_MODULE,
    .name = "virtio_ccw",
    },
    .ids = virtio_ids,
    .probe = virtio_ccw_probe,
    .remove = virtio_ccw_remove,
    .set_offline = virtio_ccw_offline,
    .set_online = virtio_ccw_online,
    .notify = virtio_ccw_cio_notify,
    .int_class = IRQIO_VIR,
    };
    static int __init pure_hex(char **cp, unsigned int *val, int min_digit,
    int max_digit, int max_val)
    {
    int diff;
    diff = 0;
// val = 0;
    while (diff <= max_digit) {
    let mut value: c_int = hex_to_bin(**cp);
    if (value < 0)
    break;
// val = *val * 16 + value;
    (*cp)++;
    diff++;
    }
    if ((diff < min_digit) || (diff > max_digit) || (*val > max_val))
    return 1;
    return 0;
    }
    static int __init parse_busid(char *str, unsigned int *cssid,
    unsigned int *ssid, unsigned int *devno)
    {
    char *str_work;
    int rc, ret;
    rc = 1;
    if (*str == '\0')
    goto out;
    str_work = str;
    ret = pure_hex(&str_work, cssid, 1, 2, __MAX_CSSID);
    if (ret || (str_work[0] != '.'))
    goto out;
    str_work++;
    ret = pure_hex(&str_work, ssid, 1, 1, __MAX_SSID);
    if (ret || (str_work[0] != '.'))
    goto out;
    str_work++;
    ret = pure_hex(&str_work, devno, 4, 4, __MAX_SUBCHANNEL);
    if (ret || (str_work[0] != '\0'))
    goto out;
    rc = 0;
    out:
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn no_auto_parse() -> void __init {
    static void __init no_auto_parse(void)
    {
    unsigned int from_cssid, to_cssid, from_ssid, to_ssid, from, to;
    char *parm, *str;
    int rc;
    str = no_auto;
    while ((parm = strsep(&str, ","))) {
    rc = parse_busid(strsep(&parm, "-"), &from_cssid,
    &from_ssid, &from);
    if (rc)
    continue;
    if (parm != core::ptr::null_mut()) {
    rc = parse_busid(parm, &to_cssid,
    &to_ssid, &to);
    if ((from_ssid > to_ssid) ||
    ((from_ssid == to_ssid) && (from > to)))
    rc = -EINVAL;
    } else {
    to_cssid = from_cssid;
    to_ssid = from_ssid;
    to = from;
    }
    if (rc)
    continue;
    while ((from_ssid < to_ssid) ||
    ((from_ssid == to_ssid) && (from <= to))) {
    set_bit(from, devs_no_auto[from_ssid]);
    from++;
    if (from > __MAX_SUBCHANNEL) {
    from_ssid++;
    from = 0;
    }
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn virtio_ccw_init() -> int __init {
    static int __init virtio_ccw_init(void)
    {
    int rc;
// parse no_auto string before we do anything further
    no_auto_parse();
    summary_indicators = cio_dma_zalloc(MAX_AIRQ_AREAS);
    if (!summary_indicators)
    return -ENOMEM;
    rc = ccw_driver_register(&virtio_ccw_driver);
    if (rc)
    cio_dma_free(summary_indicators, MAX_AIRQ_AREAS);
    return rc;
    }
    device_initcall(virtio_ccw_init);
