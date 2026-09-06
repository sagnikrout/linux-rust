//! Automatically rewritten from C to Rust
//! Source: block/bsg.c
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
// bsg.c - block layer implementation of the sg v4 interface
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bsg_device {
    pub queue: *mut request_queue,
    pub device: device,
    pub cdev: cdev,
    pub max_queue: c_int,
    pub timeout: c_uint,
    pub reserved_size: c_uint,
    pub sg_io_fn: *mut bsg_sg_io_fn,
    pub uring_cmd_fn: *mut bsg_uring_cmd_fn,
}

    static inline struct bsg_device *to_bsg_device(struct inode *inode)
    {
    return container_of(inode.i_cdev, struct bsg_device, cdev);
    }
pub const BSG_DEFAULT_CMDS: c_int = 64;

    static DEFINE_IDA(bsg_minor_ida);
    static const struct class bsg_class;
    static int bsg_major;
#[no_mangle]
unsafe extern "C" fn bsg_timeout(bd: *mut bsg_device, hdr: *mut sg_io_v4) -> c_uint {
    static unsigned int bsg_timeout(struct bsg_device *bd, struct sg_io_v4 *hdr)
    {
    let mut timeout: c_uint = BLK_DEFAULT_SG_TIMEOUT;
    if (hdr.timeout)
    timeout = msecs_to_jiffies(hdr.timeout);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: bd->timeout) -> else {
    else if (bd.timeout)
    timeout = bd.timeout;
    return max_t(unsigned int, timeout, BLK_MIN_SG_TIMEOUT);
    }
    static int bsg_sg_io(struct bsg_device *bd, bool open_for_write,
    void __user *uarg)
    {
    struct sg_io_v4 hdr;
    int ret;
    if (copy_from_user(&hdr, uarg, sizeof(hdr)))
    return -EFAULT;
    if (hdr.guard != 'Q')
    return -EINVAL;
    ret = bd.sg_io_fn(bd.queue, &hdr, open_for_write,
    bsg_timeout(bd, &hdr));
    if (!ret && copy_to_user(uarg, &hdr, sizeof(hdr)))
    return -EFAULT;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bsg_open(inode: *mut inode, file: *mut file) -> c_int {
    static int bsg_open(struct inode *inode, struct file *file)
    {
    if (!blk_get_queue(to_bsg_device(inode).queue))
    return -ENXIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bsg_release(inode: *mut inode, file: *mut file) -> c_int {
    static int bsg_release(struct inode *inode, struct file *file)
    {
    blk_put_queue(to_bsg_device(inode).queue);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bsg_get_command_q(bd: *mut bsg_device, uarg: *mut int __user) -> c_int {
    static int bsg_get_command_q(struct bsg_device *bd, int __user *uarg)
    {
    return put_user(READ_ONCE(bd.max_queue), uarg);
    }
#[no_mangle]
unsafe extern "C" fn bsg_set_command_q(bd: *mut bsg_device, uarg: *mut int __user) -> c_int {
    static int bsg_set_command_q(struct bsg_device *bd, int __user *uarg)
    {
    int max_queue;
    if (get_user(max_queue, uarg))
    return -EFAULT;
    if (max_queue < 1)
    return -EINVAL;
    WRITE_ONCE(bd.max_queue, max_queue);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bsg_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    static long bsg_ioctl(struct file *file, unsigned int cmd, unsigned long arg)
    {
    struct bsg_device *bd = to_bsg_device(file_inode(file));
    struct request_queue *q = bd.queue;
    void __user *uarg = (void __user *) arg;
    int __user *intp = uarg;
    int val;
    switch (cmd) {
//
// Our own ioctls
//
    case SG_GET_COMMAND_Q:
    return bsg_get_command_q(bd, uarg);
    case SG_SET_COMMAND_Q:
    return bsg_set_command_q(bd, uarg);
//
// SCSI/sg ioctls
//
    case SG_GET_VERSION_NUM:
    return put_user(30527, intp);
    case SCSI_IOCTL_GET_IDLUN:
    return put_user(0, intp);
    case SCSI_IOCTL_GET_BUS_NUMBER:
    return put_user(0, intp);
    case SG_SET_TIMEOUT:
    if (get_user(val, intp))
    return -EFAULT;
    bd.timeout = clock_t_to_jiffies(val);
    return 0;
    case SG_GET_TIMEOUT:
    return jiffies_to_clock_t(bd.timeout);
    case SG_GET_RESERVED_SIZE:
    return put_user(min(bd.reserved_size, queue_max_bytes(q)),
    intp);
    case SG_SET_RESERVED_SIZE:
    if (get_user(val, intp))
    return -EFAULT;
    if (val < 0)
    return -EINVAL;
    bd.reserved_size =
    min_t(unsigned int, val, queue_max_bytes(q));
    return 0;
    case SG_EMULATED_HOST:
    return put_user(1, intp);
    case SG_IO:
    return bsg_sg_io(bd, file.f_mode & FMODE_WRITE, uarg);
    case SCSI_IOCTL_SEND_COMMAND:
    pr_warn_ratelimited("%s: calling unsupported SCSI_IOCTL_SEND_COMMAND\n",
    current.comm);
    return -EINVAL;
    default:
    return -ENOTTY;
    }
    }
#[no_mangle]
unsafe extern "C" fn bsg_check_uring_features(issue_flags: c_uint) -> c_int {
    static int bsg_check_uring_features(unsigned int issue_flags)
    {
// BSG passthrough requires big SQE/CQE support
    if ((issue_flags & (IO_URING_F_SQE128|IO_URING_F_CQE32)) !=
    (IO_URING_F_SQE128|IO_URING_F_CQE32))
    return -EOPNOTSUPP;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bsg_uring_cmd(ioucmd: *mut io_uring_cmd, issue_flags: c_uint) -> c_int {
    static int bsg_uring_cmd(struct io_uring_cmd *ioucmd, unsigned int issue_flags)
    {
    struct bsg_device *bd = to_bsg_device(file_inode(ioucmd.file));
    let mut open_for_write: bool = ioucmd.file.f_mode & FMODE_WRITE;
    struct request_queue *q = bd.queue;
    int ret;
    ret = bsg_check_uring_features(issue_flags);
    if (ret)
    return ret;
    if (!bd.uring_cmd_fn)
    return -EOPNOTSUPP;
    return bd.uring_cmd_fn(q, ioucmd, issue_flags, open_for_write);
    }
    static const struct file_operations bsg_fops = {
    .open		=	bsg_open,
    .release	=	bsg_release,
    .unlocked_ioctl	=	bsg_ioctl,
    .compat_ioctl	=	compat_ptr_ioctl,
    .uring_cmd	=	bsg_uring_cmd,
    .owner		=	THIS_MODULE,
    .llseek		=	default_llseek,
    };
#[no_mangle]
unsafe extern "C" fn bsg_device_release(dev: *mut device) {
    static void bsg_device_release(struct device *dev)
    {
    struct bsg_device *bd = container_of(dev, struct bsg_device, device);
    ida_free(&bsg_minor_ida, MINOR(bd.device.devt));
    kfree(bd);
    }
#[no_mangle]
pub unsafe extern "C" fn bsg_unregister_queue(bd: *mut bsg_device) {
    void bsg_unregister_queue(struct bsg_device *bd)
    {
    struct gendisk *disk = bd.queue.disk;
    if (disk && disk.queue_kobj.sd)
    sysfs_remove_link(&disk.queue_kobj, "bsg");
    cdev_device_del(&bd.cdev, &bd.device);
    put_device(&bd.device);
    }
    EXPORT_SYMBOL_GPL(bsg_unregister_queue);
    struct bsg_device *bsg_register_queue(struct request_queue *q,
    struct device *parent, const char *name, bsg_sg_io_fn *sg_io_fn,
    bsg_uring_cmd_fn *uring_cmd_fn)
    {
    struct bsg_device *bd;
    int ret;
    bd = kzalloc_obj(*bd);
    if (!bd)
    return ERR_PTR(-ENOMEM);
    bd.max_queue = BSG_DEFAULT_CMDS;
    bd.reserved_size = INT_MAX;
    bd.queue = q;
    bd.sg_io_fn = sg_io_fn;
    bd.uring_cmd_fn = uring_cmd_fn;
    ret = ida_alloc_max(&bsg_minor_ida, BSG_MAX_DEVS - 1, GFP_KERNEL);
    if (ret < 0) {
    if (ret == -ENOSPC)
    dev_err(parent, "bsg: too many bsg devices\n");
    kfree(bd);
    return ERR_PTR(ret);
    }
    bd.device.devt = MKDEV(bsg_major, ret);
    bd.device.class = &bsg_class;
    bd.device.parent = parent;
    bd.device.release = bsg_device_release;
    dev_set_name(&bd.device, "%s", name);
    device_initialize(&bd.device);
    cdev_init(&bd.cdev, &bsg_fops);
    bd.cdev.owner = THIS_MODULE;
    ret = cdev_device_add(&bd.cdev, &bd.device);
    if (ret)
    goto out_put_device;
    if (q.disk && q.disk.queue_kobj.sd) {
    ret = sysfs_create_link(&q.disk.queue_kobj, &bd.device.kobj,
    "bsg");
    if (ret)
    goto out_device_del;
    }
    return bd;
    out_device_del:
    cdev_device_del(&bd.cdev, &bd.device);
    out_put_device:
    put_device(&bd.device);
    return ERR_PTR(ret);
    }
    EXPORT_SYMBOL_GPL(bsg_register_queue);
    static char *bsg_devnode(const struct device *dev, umode_t *mode)
    {
    return kasprintf(GFP_KERNEL, "bsg/%s", dev_name(dev));
    }
    static const struct class bsg_class = {
    .name		= "bsg",
    .devnode	= bsg_devnode,
    };
#[no_mangle]
unsafe extern "C" fn bsg_init() -> int __init {
    static int __init bsg_init(void)
    {
    dev_t devid;
    int ret;
    ret = class_register(&bsg_class);
    if (ret)
    return ret;
    ret = alloc_chrdev_region(&devid, 0, BSG_MAX_DEVS, "bsg");
    if (ret)
    goto destroy_bsg_class;
    bsg_major = MAJOR(devid);
    printk(KERN_INFO BSG_DESCRIPTION " version " BSG_VERSION
    " loaded (major %d)\n", bsg_major);
    return 0;
    destroy_bsg_class:
    class_unregister(&bsg_class);
    return ret;
    }
    MODULE_AUTHOR("Jens Axboe");
    MODULE_DESCRIPTION(BSG_DESCRIPTION);
    MODULE_LICENSE("GPL");
    device_initcall(bsg_init);
