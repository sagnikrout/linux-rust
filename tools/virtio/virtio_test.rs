//! Automatically rewritten from C to Rust
//! Source: tools/virtio/virtio_test.c
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
// Macro flag: #define _GNU_SOURCE

// Unused
    void *__kmalloc_fake, *__kfree_ignore_start, *__kfree_ignore_end;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vq_info {
    pub kick: c_int,
    pub call: c_int,
    pub num: c_int,
    pub idx: c_int,
    pub ring: *mut c_void,
// copy used for control
    pub vring: vring,
    pub vq: *mut virtqueue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdev_info {
    pub vdev: virtio_device,
    pub control: c_int,
    pub fds: [pollfd; 1],
    pub vqs: [vq_info; 1],
    pub nvqs: c_int,
    pub buf: *mut c_void,
    pub buf_size: usize,
    pub mem: *mut vhost_memory,
}

    static const struct vhost_vring_file no_backend = { .fd = -1 },
    backend = { .fd = 1 };
    let mut null_state: static struct vhost_vring_state = {};
#[no_mangle]
pub unsafe extern "C" fn vq_notify(vq: *mut virtqueue) -> bool {
    bool vq_notify(struct virtqueue *vq)
    {
    struct vq_info *info = vq.priv;
    let mut v: c_ulonglong = 1;
    int r;
    r = write(info.kick, &v, sizeof v);
    assert(r == sizeof v);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn vq_callback(vq: *mut virtqueue) {
    void vq_callback(struct virtqueue *vq)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn vhost_vq_setup(dev: *mut vdev_info, info: *mut vq_info) {
    void vhost_vq_setup(struct vdev_info *dev, struct vq_info *info)
    {
    let mut state: vhost_vring_state = { .index = info.idx };
    let mut file: vhost_vring_file = { .index = info.idx };
    let mut features: c_ulonglong = dev.vdev.features;
    struct vhost_vring_addr addr = {
    .index = info.idx,
    .desc_user_addr = (uint64_t)(unsigned long)info.vring.desc,
    .avail_user_addr = (uint64_t)(unsigned long)info.vring.avail,
    .used_user_addr = (uint64_t)(unsigned long)info.vring.used,
    };
    int r;
    r = ioctl(dev.control, VHOST_SET_FEATURES, &features);
    assert(r >= 0);
    state.num = info.vring.num;
    r = ioctl(dev.control, VHOST_SET_VRING_NUM, &state);
    assert(r >= 0);
    state.num = 0;
    r = ioctl(dev.control, VHOST_SET_VRING_BASE, &state);
    assert(r >= 0);
    r = ioctl(dev.control, VHOST_SET_VRING_ADDR, &addr);
    assert(r >= 0);
    file.fd = info.kick;
    r = ioctl(dev.control, VHOST_SET_VRING_KICK, &file);
    assert(r >= 0);
    file.fd = info.call;
    r = ioctl(dev.control, VHOST_SET_VRING_CALL, &file);
    assert(r >= 0);
    }
#[no_mangle]
unsafe extern "C" fn vq_reset(info: *mut vq_info, num: c_int, vdev: *mut virtio_device) {
    static void vq_reset(struct vq_info *info, int num, struct virtio_device *vdev)
    {
    if (info.vq)
    vring_del_virtqueue(info.vq);
    memset(info.ring, 0, vring_size(num, 4096));
    vring_init(&info.vring, num, info.ring, 4096);
    info.vq = vring_new_virtqueue(info.idx, num, 4096, vdev, true, false,
    info.ring, vq_notify, vq_callback, "test");
    assert(info.vq);
    info.vq.priv = info;
    }
#[no_mangle]
unsafe extern "C" fn vq_info_add(dev: *mut vdev_info, num: c_int) {
    static void vq_info_add(struct vdev_info *dev, int num)
    {
    struct vq_info *info = &dev.vqs[dev.nvqs];
    int r;
    info.idx = dev.nvqs;
    info.kick = eventfd(0, EFD_NONBLOCK);
    info.call = eventfd(0, EFD_NONBLOCK);
    r = posix_memalign(&info.ring, 4096, vring_size(num, 4096));
    assert(r >= 0);
    vq_reset(info, num, &dev.vdev);
    vhost_vq_setup(dev, info);
    dev.fds[info.idx].fd = info.call;
    dev.fds[info.idx].events = POLLIN;
    dev.nvqs++;
    }
#[no_mangle]
unsafe extern "C" fn vdev_info_init(dev: *mut *mut vdev_info, features: c_ulonglong) {
    static void vdev_info_init(struct vdev_info* dev, unsigned long long features)
    {
    int r;
    memset(dev, 0, sizeof *dev);
    dev.vdev.features = features;
    INIT_LIST_HEAD(&dev.vdev.vqs);
    spin_lock_init(&dev.vdev.vqs_list_lock);
    dev.buf_size = 1024;
    dev.buf = malloc(dev.buf_size);
    assert(dev.buf);
    dev.control = open("/dev/vhost-test", O_RDWR);
    assert(dev.control >= 0);
    r = ioctl(dev.control, VHOST_SET_OWNER, core::ptr::null_mut());
    assert(r >= 0);
    dev.mem = malloc(offsetof(struct vhost_memory, regions) +
    sizeof dev.mem.regions[0]);
    assert(dev.mem);
    memset(dev.mem, 0, offsetof(struct vhost_memory, regions) +
    sizeof dev.mem.regions[0]);
    dev.mem.nregions = 1;
    dev.mem.regions[0].guest_phys_addr = (long)dev.buf;
    dev.mem.regions[0].userspace_addr = (long)dev.buf;
    dev.mem.regions[0].memory_size = dev.buf_size;
    r = ioctl(dev.control, VHOST_SET_MEM_TABLE, dev.mem);
    assert(r >= 0);
    }
// TODO: this is pretty bad: we get a cache line bounce
// for the wait queue on poll and another one on read,
// plus the read which is there just to clear the
// current state.
#[no_mangle]
unsafe extern "C" fn wait_for_interrupt(dev: *mut vdev_info) {
    static void wait_for_interrupt(struct vdev_info *dev)
    {
    int i;
    unsigned long long val;
    poll(dev.fds, dev.nvqs, -1);
    for (i = 0; i < dev.nvqs; ++i)
    if (dev.fds[i].revents & POLLIN) {
    read(dev.fds[i].fd, &val, sizeof val);
    }
    }
    static void run_test(struct vdev_info *dev, struct vq_info *vq,
    bool delayed, int batch, int reset_n, int bufs)
    {
    struct scatterlist sl;
    let mut started: c_long = 0, completed = 0, next_reset = reset_n;
    long completed_before, started_before;
    int r, test = 1;
    unsigned int len;
    let mut spurious: c_longlong = 0;
    let mut random_batch: bool = batch == RANDOM_BATCH;
    r = ioctl(dev.control, VHOST_TEST_RUN, &test);
    assert(r >= 0);
    if (!reset_n) {
    next_reset = INT_MAX;
    }
    for (;;) {
    virtqueue_disable_cb(vq.vq);
    completed_before = completed;
    started_before = started;
    do {
    let mut reset: bool = completed > next_reset;
    if (random_batch)
    batch = (random() % vq.vring.num) + 1;
    while (started < bufs &&
    (started - completed) < batch) {
    sg_init_one(&sl, dev.buf, dev.buf_size);
    r = virtqueue_add_outbuf(vq.vq, &sl, 1,
    dev.buf + started,
    GFP_ATOMIC);
    if (unlikely(r != 0)) {
    if (r == -ENOSPC &&
    started > started_before)
    r = 0;
    else
    r = -1;
    break;
    }
    ++started;
    if (unlikely(!virtqueue_kick(vq.vq))) {
    r = -1;
    break;
    }
    }
    if (started >= bufs)
    r = -1;
    if (reset) {
    r = ioctl(dev.control, VHOST_TEST_SET_BACKEND,
    &no_backend);
    assert(!r);
    }
// Flush out completed bufs if any
    while (virtqueue_get_buf(vq.vq, &len)) {
    ++completed;
    r = 0;
    }
    if (reset) {
    let mut s: vhost_vring_state = { .index = 0 };
    vq_reset(vq, vq.vring.num, &dev.vdev);
    r = ioctl(dev.control, VHOST_GET_VRING_BASE,
    &s);
    assert(!r);
    s.num = 0;
    r = ioctl(dev.control, VHOST_SET_VRING_BASE,
    &null_state);
    assert(!r);
    r = ioctl(dev.control, VHOST_TEST_SET_BACKEND,
    &backend);
    assert(!r);
    started = completed;
    while (completed > next_reset)
    next_reset += completed;
    }
    } while (r == 0);
    if (completed == completed_before && started == started_before)
    ++spurious;
    assert(completed <= bufs);
    assert(started <= bufs);
    if (completed == bufs)
    break;
    if (delayed) {
    if (virtqueue_enable_cb_delayed(vq.vq))
    wait_for_interrupt(dev);
    } else {
    if (virtqueue_enable_cb(vq.vq))
    wait_for_interrupt(dev);
    }
    }
    test = 0;
    r = ioctl(dev.control, VHOST_TEST_RUN, &test);
    assert(r >= 0);
    fprintf(stderr,
    "spurious wakeups: 0x%llx started=0x%lx completed=0x%lx\n",
    spurious, started, completed);
    }
    const char optstring[] = "h";
    const struct option longopts[] = {
    {
    .name = "help",
    .val = 'h',
    },
    {
    .name = "event-idx",
    .val = 'E',
    },
    {
    .name = "no-event-idx",
    .val = 'e',
    },
    {
    .name = "indirect",
    .val = 'I',
    },
    {
    .name = "no-indirect",
    .val = 'i',
    },
    {
    .name = "virtio-1",
    .val = '1',
    },
    {
    .name = "no-virtio-1",
    .val = '0',
    },
    {
    .name = "delayed-interrupt",
    .val = 'D',
    },
    {
    .name = "no-delayed-interrupt",
    .val = 'd',
    },
    {
    .name = "batch",
    .val = 'b',
    .has_arg = required_argument,
    },
    {
    .name = "reset",
    .val = 'r',
    .has_arg = optional_argument,
    },
    {
    }
    };
#[no_mangle]
unsafe extern "C" fn help(status: c_int) {
    static void help(int status)
    {
    fprintf(stderr, "Usage: virtio_test [--help]"
    " [--no-indirect]"
    " [--no-event-idx]"
    " [--no-virtio-1]"
    " [--delayed-interrupt]"
    " [--batch=random/N]"
    " [--reset=N]"
    "\n");
    exit(status);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    struct vdev_info dev;
    unsigned long long features = (1ULL << VIRTIO_RING_F_INDIRECT_DESC) |
    (1ULL << VIRTIO_RING_F_EVENT_IDX) | (1ULL << VIRTIO_F_VERSION_1);
    let mut batch: c_long = 1, reset = 0;
    int o;
    let mut delayed: bool = false;
    for (;;) {
    o = getopt_long(argc, argv, optstring, longopts, core::ptr::null_mut());
    switch (o) {
    case -1:
    goto done;
    case '?':
    help(2);
    case 'e':
    features &= ~(1ULL << VIRTIO_RING_F_EVENT_IDX);
    break;
    case 'h':
    help(0);
    case 'i':
    features &= ~(1ULL << VIRTIO_RING_F_INDIRECT_DESC);
    break;
    case '0':
    features &= ~(1ULL << VIRTIO_F_VERSION_1);
    break;
    case 'D':
    delayed = true;
    break;
    case 'b':
    if (0 == strcmp(optarg, "random")) {
    batch = RANDOM_BATCH;
    } else {
    batch = strtol(optarg, core::ptr::null_mut(), 10);
    assert(batch > 0);
    assert(batch < (long)INT_MAX + 1);
    }
    break;
    case 'r':
    if (!optarg) {
    reset = 1;
    } else {
    reset = strtol(optarg, core::ptr::null_mut(), 10);
    assert(reset > 0);
    assert(reset < (long)INT_MAX + 1);
    }
    break;
    default:
    assert(0);
    break;
    }
    }
    done:
    vdev_info_init(&dev, features);
    vq_info_add(&dev, 256);
    run_test(&dev, &dev.vqs[0], delayed, batch, reset, 0x100000);
    return 0;
    }
