//! Automatically rewritten from C to Rust
//! Source: drivers/vdpa/vdpa_sim/vdpa_sim_blk.c
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
// VDPA simulator for block device.
//
// Copyright (c) 2020, NVIDIA CORPORATION. All rights reserved.
// Copyright (c) 2021, Red Hat Inc. All rights reserved.
//

    (1ULL << VIRTIO_BLK_F_FLUSH)    | \
    (1ULL << VIRTIO_BLK_F_SIZE_MAX) | \
    (1ULL << VIRTIO_BLK_F_SEG_MAX)  | \
    (1ULL << VIRTIO_BLK_F_BLK_SIZE) | \
    (1ULL << VIRTIO_BLK_F_TOPOLOGY) | \
    (1ULL << VIRTIO_BLK_F_MQ)       | \
    (1ULL << VIRTIO_BLK_F_DISCARD)  | \
    (1ULL << VIRTIO_BLK_F_WRITE_ZEROES))
pub const VDPASIM_BLK_CAPACITY: c_uint = 0x40000;
pub const VDPASIM_BLK_SIZE_MAX: c_uint = 0x1000;
pub const VDPASIM_BLK_SEG_MAX: c_int = 32;

// 1 virtqueue, 1 address space, 1 virtqueue group
pub const VDPASIM_BLK_VQ_NUM: c_int = 1;
pub const VDPASIM_BLK_AS_NUM: c_int = 1;
pub const VDPASIM_BLK_GROUP_NUM: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdpasim_blk {
    pub vdpasim: vdpasim,
    pub buffer: *mut c_void,
    pub shared_backend: bool,
}

    static struct vdpasim_blk *sim_to_blk(struct vdpasim *vdpasim)
    {
    return container_of(vdpasim, struct vdpasim_blk, vdpasim);
    }
    static char vdpasim_blk_id[VIRTIO_BLK_ID_BYTES] = "vdpa_blk_sim";
    static bool shared_backend;
    module_param(shared_backend, bool, 0444);
    MODULE_PARM_DESC(shared_backend, "Enable the shared backend between virtio-blk devices");
    static void *shared_buffer;
// mutex to synchronize shared_buffer access
    static DEFINE_MUTEX(shared_buffer_mutex);
#[no_mangle]
unsafe extern "C" fn vdpasim_blk_buffer_lock(blk: *mut vdpasim_blk) {
    static void vdpasim_blk_buffer_lock(struct vdpasim_blk *blk)
    {
    if (blk.shared_backend)
    mutex_lock(&shared_buffer_mutex);
    }
#[no_mangle]
unsafe extern "C" fn vdpasim_blk_buffer_unlock(blk: *mut vdpasim_blk) {
    static void vdpasim_blk_buffer_unlock(struct vdpasim_blk *blk)
    {
    if (blk.shared_backend)
    mutex_unlock(&shared_buffer_mutex);
    }
    static bool vdpasim_blk_check_range(struct vdpasim *vdpasim, u64 start_sector,
    u64 num_sectors, u64 max_sectors)
    {
    if (start_sector > VDPASIM_BLK_CAPACITY) {
    dev_dbg(&vdpasim.vdpa.dev,
    "starting sector exceeds the capacity - start: 0x%llx capacity: 0x%x\n",
    start_sector, VDPASIM_BLK_CAPACITY);
    }
    if (num_sectors > max_sectors) {
    dev_dbg(&vdpasim.vdpa.dev,
    "number of sectors exceeds the max allowed in a request - num: 0x%llx max: 0x%llx\n",
    num_sectors, max_sectors);
    return false;
    }
    if (num_sectors > VDPASIM_BLK_CAPACITY - start_sector) {
    dev_dbg(&vdpasim.vdpa.dev,
    "request exceeds the capacity - start: 0x%llx num: 0x%llx capacity: 0x%x\n",
    start_sector, num_sectors, VDPASIM_BLK_CAPACITY);
    return false;
    }
    return true;
    }
// Returns 'true' if the request is handled (with or without an I/O error)
// and the status is correctly written in the last byte of the 'in iov',
// 'false' otherwise.
//
    static bool vdpasim_blk_handle_req(struct vdpasim *vdpasim,
    struct vdpasim_virtqueue *vq)
    {
    struct vdpasim_blk *blk = sim_to_blk(vdpasim);
    let mut pushed: usize = 0, to_pull, to_push;
    struct virtio_blk_outhdr hdr;
    let mut handled: bool = false;
    ssize_t bytes;
    loff_t offset;
    u64 sector;
    u8 status;
    u32 type;
    int ret;
    ret = vringh_getdesc_iotlb(&vq.vring, &vq.out_iov, &vq.in_iov,
    &vq.head, GFP_ATOMIC);
    if (ret != 1)
    return false;
    if (vq.out_iov.used < 1 || vq.in_iov.used < 1) {
    dev_dbg(&vdpasim.vdpa.dev, "missing headers - out_iov: %u in_iov %u\n",
    vq.out_iov.used, vq.in_iov.used);
    goto err;
    }
    if (vq.in_iov.iov[vq.in_iov.used - 1].iov_len < 1) {
    dev_dbg(&vdpasim.vdpa.dev, "request in header too short\n");
    goto err;
    }
// The last byte is the status and we checked if the last iov has
// enough room for it.
//
    to_push = vringh_kiov_length(&vq.in_iov) - 1;
    to_pull = vringh_kiov_length(&vq.out_iov);
    bytes = vringh_iov_pull_iotlb(&vq.vring, &vq.out_iov, &hdr,
    sizeof(hdr));
    if (bytes != sizeof(hdr)) {
    dev_dbg(&vdpasim.vdpa.dev, "request out header too short\n");
    goto err;
    }
    to_pull -= bytes;
    type = vdpasim32_to_cpu(vdpasim, hdr.type);
    sector = vdpasim64_to_cpu(vdpasim, hdr.sector);
    offset = sector << SECTOR_SHIFT;
    status = VIRTIO_BLK_S_OK;
    if (type != VIRTIO_BLK_T_IN && type != VIRTIO_BLK_T_OUT &&
    sector != 0) {
    dev_dbg(&vdpasim.vdpa.dev,
    "sector must be 0 for %u request - sector: 0x%llx\n",
    type, sector);
    status = VIRTIO_BLK_S_IOERR;
    goto err_status;
    }
    switch (type) {
    case VIRTIO_BLK_T_IN:
    if (!vdpasim_blk_check_range(vdpasim, sector,
    to_push >> SECTOR_SHIFT,
    VDPASIM_BLK_SIZE_MAX * VDPASIM_BLK_SEG_MAX)) {
    status = VIRTIO_BLK_S_IOERR;
    break;
    }
    vdpasim_blk_buffer_lock(blk);
    bytes = vringh_iov_push_iotlb(&vq.vring, &vq.in_iov,
    blk.buffer + offset, to_push);
    vdpasim_blk_buffer_unlock(blk);
    if (bytes < 0) {
    dev_dbg(&vdpasim.vdpa.dev,
    "vringh_iov_push_iotlb() error: %zd offset: 0x%llx len: 0x%zx\n",
    bytes, offset, to_push);
    status = VIRTIO_BLK_S_IOERR;
    break;
    }
    pushed += bytes;
    break;
    case VIRTIO_BLK_T_OUT:
    if (!vdpasim_blk_check_range(vdpasim, sector,
    to_pull >> SECTOR_SHIFT,
    VDPASIM_BLK_SIZE_MAX * VDPASIM_BLK_SEG_MAX)) {
    status = VIRTIO_BLK_S_IOERR;
    break;
    }
    vdpasim_blk_buffer_lock(blk);
    bytes = vringh_iov_pull_iotlb(&vq.vring, &vq.out_iov,
    blk.buffer + offset, to_pull);
    vdpasim_blk_buffer_unlock(blk);
    if (bytes < 0) {
    dev_dbg(&vdpasim.vdpa.dev,
    "vringh_iov_pull_iotlb() error: %zd offset: 0x%llx len: 0x%zx\n",
    bytes, offset, to_pull);
    status = VIRTIO_BLK_S_IOERR;
    break;
    }
    break;
    case VIRTIO_BLK_T_GET_ID:
    bytes = vringh_iov_push_iotlb(&vq.vring, &vq.in_iov,
    vdpasim_blk_id,
    VIRTIO_BLK_ID_BYTES);
    if (bytes < 0) {
    dev_dbg(&vdpasim.vdpa.dev,
    "vringh_iov_push_iotlb() error: %zd\n", bytes);
    status = VIRTIO_BLK_S_IOERR;
    break;
    }
    pushed += bytes;
    break;
    case VIRTIO_BLK_T_FLUSH:
// nothing to do
    break;
    case VIRTIO_BLK_T_DISCARD:
    case VIRTIO_BLK_T_WRITE_ZEROES: {
    struct virtio_blk_discard_write_zeroes range;
    u32 num_sectors, flags;
    if (to_pull != sizeof(range)) {
    dev_dbg(&vdpasim.vdpa.dev,
    "discard/write_zeroes header len: 0x%zx [expected: 0x%zx]\n",
    to_pull, sizeof(range));
    status = VIRTIO_BLK_S_IOERR;
    break;
    }
    bytes = vringh_iov_pull_iotlb(&vq.vring, &vq.out_iov, &range,
    to_pull);
    if (bytes < 0) {
    dev_dbg(&vdpasim.vdpa.dev,
    "vringh_iov_pull_iotlb() error: %zd offset: 0x%llx len: 0x%zx\n",
    bytes, offset, to_pull);
    status = VIRTIO_BLK_S_IOERR;
    break;
    }
    sector = le64_to_cpu(range.sector);
    offset = sector << SECTOR_SHIFT;
    num_sectors = le32_to_cpu(range.num_sectors);
    flags = le32_to_cpu(range.flags);
    if (type == VIRTIO_BLK_T_DISCARD && flags != 0) {
    dev_dbg(&vdpasim.vdpa.dev,
    "discard unexpected flags set - flags: 0x%x\n",
    flags);
    status = VIRTIO_BLK_S_UNSUPP;
    break;
    }
    if (type == VIRTIO_BLK_T_WRITE_ZEROES &&
    flags & ~VIRTIO_BLK_WRITE_ZEROES_FLAG_UNMAP) {
    dev_dbg(&vdpasim.vdpa.dev,
    "write_zeroes unexpected flags set - flags: 0x%x\n",
    flags);
    status = VIRTIO_BLK_S_UNSUPP;
    break;
    }
    if (!vdpasim_blk_check_range(vdpasim, sector, num_sectors,
    VDPASIM_BLK_DWZ_MAX_SECTORS)) {
    status = VIRTIO_BLK_S_IOERR;
    break;
    }
    if (type == VIRTIO_BLK_T_WRITE_ZEROES) {
    vdpasim_blk_buffer_lock(blk);
    memset(blk.buffer + offset, 0,
    num_sectors << SECTOR_SHIFT);
    vdpasim_blk_buffer_unlock(blk);
    }
    break;
    }
    default:
    dev_dbg(&vdpasim.vdpa.dev,
    "Unsupported request type %d\n", type);
    status = VIRTIO_BLK_S_IOERR;
    break;
    }
    err_status:
// If some operations fail, we need to skip the remaining bytes
// to put the status in the last byte
//
    if (to_push - pushed > 0)
    vringh_kiov_advance(&vq.in_iov, to_push - pushed);
// Last byte is the status
    bytes = vringh_iov_push_iotlb(&vq.vring, &vq.in_iov, &status, 1);
    if (bytes != 1)
    goto err;
    pushed += bytes;
// Make sure data is wrote before advancing index
    smp_wmb();
    handled = true;
    err:
    vringh_complete_iotlb(&vq.vring, vq.head, pushed);
    return handled;
    }
#[no_mangle]
unsafe extern "C" fn vdpasim_blk_work(vdpasim: *mut vdpasim) {
    static void vdpasim_blk_work(struct vdpasim *vdpasim)
    {
    let mut reschedule: bool = false;
    int i;
    mutex_lock(&vdpasim.mutex);
    if (!(vdpasim.status & VIRTIO_CONFIG_S_DRIVER_OK))
    goto out;
    if (!vdpasim.running)
    goto out;
    for (i = 0; i < VDPASIM_BLK_VQ_NUM; i++) {
    struct vdpasim_virtqueue *vq = &vdpasim.vqs[i];
    let mut reqs: c_int = 0;
    if (!vq.ready)
    continue;
    while (vdpasim_blk_handle_req(vdpasim, vq)) {
// Make sure used is visible before rasing the interrupt.
    smp_wmb();
    local_bh_disable();
    if (vringh_need_notify_iotlb(&vq.vring) > 0)
    vringh_notify(&vq.vring);
    local_bh_enable();
    if (++reqs > 4) {
    reschedule = true;
    break;
    }
    }
    }
    out:
    mutex_unlock(&vdpasim.mutex);
    if (reschedule)
    vdpasim_schedule_work(vdpasim);
    }
#[no_mangle]
unsafe extern "C" fn vdpasim_blk_get_config(vdpasim: *mut vdpasim, config: *mut c_void) {
    static void vdpasim_blk_get_config(struct vdpasim *vdpasim, void *config)
    {
    struct virtio_blk_config *blk_config = config;
    memset(config, 0, sizeof(struct virtio_blk_config));
    blk_config.capacity = cpu_to_vdpasim64(vdpasim, VDPASIM_BLK_CAPACITY);
    blk_config.size_max = cpu_to_vdpasim32(vdpasim, VDPASIM_BLK_SIZE_MAX);
    blk_config.seg_max = cpu_to_vdpasim32(vdpasim, VDPASIM_BLK_SEG_MAX);
    blk_config.num_queues = cpu_to_vdpasim16(vdpasim, VDPASIM_BLK_VQ_NUM);
    blk_config.min_io_size = cpu_to_vdpasim16(vdpasim, 1);
    blk_config.opt_io_size = cpu_to_vdpasim32(vdpasim, 1);
    blk_config.blk_size = cpu_to_vdpasim32(vdpasim, SECTOR_SIZE);
// VIRTIO_BLK_F_DISCARD
    blk_config.discard_sector_alignment =
    cpu_to_vdpasim32(vdpasim, SECTOR_SIZE);
    blk_config.max_discard_sectors =
    cpu_to_vdpasim32(vdpasim, VDPASIM_BLK_DWZ_MAX_SECTORS);
    blk_config.max_discard_seg = cpu_to_vdpasim32(vdpasim, 1);
// VIRTIO_BLK_F_WRITE_ZEROES
    blk_config.max_write_zeroes_sectors =
    cpu_to_vdpasim32(vdpasim, VDPASIM_BLK_DWZ_MAX_SECTORS);
    blk_config.max_write_zeroes_seg = cpu_to_vdpasim32(vdpasim, 1);
    }
#[no_mangle]
unsafe extern "C" fn vdpasim_blk_free(vdpasim: *mut vdpasim) {
    static void vdpasim_blk_free(struct vdpasim *vdpasim)
    {
    struct vdpasim_blk *blk = sim_to_blk(vdpasim);
    if (!blk.shared_backend)
    kvfree(blk.buffer);
    }
    static struct device *vdpasim_blk_mgmtdev;
    static int vdpasim_blk_dev_add(struct vdpa_mgmt_dev *mdev, const char *name,
    const struct vdpa_dev_set_config *config)
    {
    let mut dev_attr: vdpasim_dev_attr = {};
    struct vdpasim_blk *blk;
    struct vdpasim *simdev;
    int ret;
    dev_attr.mgmt_dev = mdev;
    dev_attr.name = name;
    dev_attr.id = VIRTIO_ID_BLOCK;
    dev_attr.supported_features = VDPASIM_BLK_FEATURES;
    dev_attr.nvqs = VDPASIM_BLK_VQ_NUM;
    dev_attr.ngroups = VDPASIM_BLK_GROUP_NUM;
    dev_attr.nas = VDPASIM_BLK_AS_NUM;
    dev_attr.alloc_size = sizeof(struct vdpasim_blk);
    dev_attr.config_size = sizeof(struct virtio_blk_config);
    dev_attr.get_config = vdpasim_blk_get_config;
    dev_attr.work_fn = vdpasim_blk_work;
    dev_attr.free = vdpasim_blk_free;
    simdev = vdpasim_create(&dev_attr, config);
    if (IS_ERR(simdev))
    return PTR_ERR(simdev);
    blk = sim_to_blk(simdev);
    blk.shared_backend = shared_backend;
    if (blk.shared_backend) {
    blk.buffer = shared_buffer;
    } else {
    blk.buffer = kvzalloc(VDPASIM_BLK_CAPACITY << SECTOR_SHIFT,
    GFP_KERNEL);
    if (!blk.buffer) {
    ret = -ENOMEM;
    goto put_dev;
    }
    }
    ret = _vdpa_register_device(&simdev.vdpa, VDPASIM_BLK_VQ_NUM);
    if (ret)
    goto put_dev;
    return 0;
    put_dev:
    put_device(&simdev.vdpa.dev);
    return ret;
    }
    static void vdpasim_blk_dev_del(struct vdpa_mgmt_dev *mdev,
    struct vdpa_device *dev)
    {
    struct vdpasim *simdev = container_of(dev, struct vdpasim, vdpa);
    _vdpa_unregister_device(&simdev.vdpa);
    }
    static const struct vdpa_mgmtdev_ops vdpasim_blk_mgmtdev_ops = {
    .dev_add = vdpasim_blk_dev_add,
    .dev_del = vdpasim_blk_dev_del
    };
    static struct virtio_device_id id_table[] = {
    { VIRTIO_ID_BLOCK, VIRTIO_DEV_ANY_ID },
    { 0 },
    };
    static struct vdpa_mgmt_dev mgmt_dev = {
    .id_table = id_table,
    .ops = &vdpasim_blk_mgmtdev_ops,
    };
#[no_mangle]
unsafe extern "C" fn vdpasim_blk_init() -> int __init {
    static int __init vdpasim_blk_init(void)
    {
    int ret;
    vdpasim_blk_mgmtdev = root_device_register("vdpasim_blk");
    if (IS_ERR(vdpasim_blk_mgmtdev))
    return PTR_ERR(vdpasim_blk_mgmtdev);
    mgmt_dev.device = vdpasim_blk_mgmtdev;
    ret = vdpa_mgmtdev_register(&mgmt_dev);
    if (ret)
    goto parent_err;
    if (shared_backend) {
    shared_buffer = kvzalloc(VDPASIM_BLK_CAPACITY << SECTOR_SHIFT,
    GFP_KERNEL);
    if (!shared_buffer) {
    ret = -ENOMEM;
    goto mgmt_dev_err;
    }
    }
    return 0;
    mgmt_dev_err:
    vdpa_mgmtdev_unregister(&mgmt_dev);
    parent_err:
    root_device_unregister(vdpasim_blk_mgmtdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn vdpasim_blk_exit() -> void __exit {
    static void __exit vdpasim_blk_exit(void)
    {
    kvfree(shared_buffer);
    vdpa_mgmtdev_unregister(&mgmt_dev);
    root_device_unregister(vdpasim_blk_mgmtdev);
    }
    module_init(vdpasim_blk_init)
    module_exit(vdpasim_blk_exit)
    MODULE_VERSION(DRV_VERSION);
    MODULE_LICENSE(DRV_LICENSE);
    MODULE_AUTHOR(DRV_AUTHOR);
    MODULE_DESCRIPTION(DRV_DESC);
