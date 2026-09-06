//! Automatically rewritten from C to Rust
//! Source: drivers/dax/super.c
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
// Copyright(c) 2017 Intel Corporation. All rights reserved.
//

//
// struct dax_device - anchor object for dax services
// @inode: core vfs
// @cdev: optional character interface for "device dax"
// @private: dax driver private data
// @flags: state and boolean properties
// @ops: operations for this device
// @holder_data: holder of a dax_device: could be filesystem or mapped device
// @holder_ops: operations for the inner holder
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dax_device {
    pub inode: inode,
    pub cdev: cdev,
    pub private: *mut c_void,
    pub flags: c_ulong,
    pub ops: *const dax_operations,
    pub holder_data: *mut c_void,
    pub holder_ops: *const dax_holder_operations,
}

    static dev_t dax_devt;
    DEFINE_STATIC_SRCU(dax_srcu);
    static struct vfsmount *dax_mnt;
    static DEFINE_IDA(dax_minor_ida);
    static struct kmem_cache *dax_cache __read_mostly;
    static struct super_block *dax_superblock __read_mostly;
#[no_mangle]
pub unsafe extern "C" fn dax_read_lock() -> c_int {
    int dax_read_lock(void)
    {
    return srcu_read_lock(&dax_srcu);
    }
    EXPORT_SYMBOL_GPL(dax_read_lock);
#[no_mangle]
pub unsafe extern "C" fn dax_read_unlock(id: c_int) {
    void dax_read_unlock(int id)
    {
    srcu_read_unlock(&dax_srcu, id);
    }
    EXPORT_SYMBOL_GPL(dax_read_unlock);

    static DEFINE_XARRAY(dax_hosts);
#[no_mangle]
pub unsafe extern "C" fn dax_add_host(dax_dev: *mut dax_device, disk: *mut gendisk) -> c_int {
    int dax_add_host(struct dax_device *dax_dev, struct gendisk *disk)
    {
    return xa_insert(&dax_hosts, (unsigned long)disk, dax_dev, GFP_KERNEL);
    }
    EXPORT_SYMBOL_GPL(dax_add_host);
#[no_mangle]
pub unsafe extern "C" fn dax_remove_host(disk: *mut gendisk) {
    void dax_remove_host(struct gendisk *disk)
    {
    xa_erase(&dax_hosts, (unsigned long)disk);
    }
    EXPORT_SYMBOL_GPL(dax_remove_host);
//
// fs_dax_get_by_bdev() - temporary lookup mechanism for filesystem-dax
// @bdev: block device to find a dax_device for
// @start_off: returns the byte offset into the dax_device that @bdev starts
// @holder: filesystem or mapped device inside the dax_device
// @ops: operations for the inner holder
//
    struct dax_device *fs_dax_get_by_bdev(struct block_device *bdev, u64 *start_off,
    void *holder, const struct dax_holder_operations *ops)
    {
    struct dax_device *dax_dev;
    u64 part_size;
    int id;
    if (!blk_queue_dax(bdev.bd_disk.queue))
    return core::ptr::null_mut();
// start_off = get_start_sect(bdev) * SECTOR_SIZE;
    part_size = bdev_nr_sectors(bdev) * SECTOR_SIZE;
    if (*start_off % PAGE_SIZE || part_size % PAGE_SIZE) {
    pr_info("%pg: error: unaligned partition for dax\n", bdev);
    return core::ptr::null_mut();
    }
    id = dax_read_lock();
    dax_dev = xa_load(&dax_hosts, (unsigned long)bdev.bd_disk);
    if (!dax_dev || !dax_alive(dax_dev) || !igrab(&dax_dev.inode))
    dax_dev = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn if(_arg: holder) -> else {
    if (!cmpxchg(&dax_dev.holder_data, core::ptr::null_mut(), holder))
    dax_dev.holder_ops = ops;
    else
    dax_dev = core::ptr::null_mut();
    }
    dax_read_unlock(id);
    return dax_dev;
    }
    EXPORT_SYMBOL_GPL(fs_dax_get_by_bdev);

//
// fs_put_dax() - release holder ownership of a dax_device
// @dax_dev: dax device to release (may be NULL)
// @holder: the holder pointer previously passed to fs_dax_get() or
// fs_dax_get_by_bdev(); must match exactly, as it is used
// in a cmpxchg to atomically release ownership
//
// Must only be called by the current holder. Clears holder_ops before
// holder_data to avoid a race where a concurrent fs_dax_get() could have
// its newly installed holder_ops overwritten.
//
#[no_mangle]
pub unsafe extern "C" fn fs_put_dax(dax_dev: *mut dax_device, holder: *mut c_void) {
    void fs_put_dax(struct dax_device *dax_dev, void *holder)
    {
    if (dax_dev && holder) {
    void *prev;
//
// Clear holder_ops before releasing holder_data. A concurrent
// dax_holder_notify_failure() that sees NULL ops returns
// -EOPNOTSUPP cleanly. A concurrent fs_dax_get() that acquires
// holder_data after the cmpxchg below is guaranteed to observe
// holder_ops=NULL first (cmpxchg provides release ordering), so
// its subsequent store of new ops will not be overwritten.
//
    WRITE_ONCE(dax_dev.holder_ops, core::ptr::null_mut());
    prev = cmpxchg(&dax_dev.holder_data, holder, core::ptr::null_mut());
//
// prev == holder: normal release.
// prev == NULL:   already released by kill_dax() when the
// device was removed under a live holder;
// not a bug.
// prev != holder (non-NULL): fs_put_dax() called by something
// that is not the current holder; an API
// contract violation. A lock would be needed
// to guard against this, but we WARN_ON()
// instead since violating the contract is
// a bug.
//
    WARN_ON(prev && prev != holder);
    }
    put_dax(dax_dev);
    }
    EXPORT_SYMBOL_GPL(fs_put_dax);
//
// fs_dax_get() - get ownership of a devdax via holder/holder_ops
//
// fs-dax file systems call this function to prepare to use a devdax device for
// fsdax. This is like fs_dax_get_by_bdev(), but the caller already has struct
// dev_dax (and there is no bdev). The holder makes this exclusive.
//
// @dax_dev: dev to be prepared for fs-dax usage
// @holder: filesystem or mapped device inside the dax_device
// @hops: operations for the inner holder
//
// Returns: 0 on success, <0 on failure
//
    int fs_dax_get(struct dax_device *dax_dev, void *holder,
    const struct dax_holder_operations *hops)
    {
    struct dev_dax *dev_dax;
    struct dax_device_driver *dax_drv;
    int id;
    id = dax_read_lock();
    if (!dax_dev || !dax_alive(dax_dev) || !igrab(&dax_dev.inode)) {
    dax_read_unlock(id);
    return -ENODEV;
    }
    dax_read_unlock(id);
// Verify the device is bound to fsdev_dax driver
    dev_dax = dax_get_private(dax_dev);
    if (!dev_dax) {
    iput(&dax_dev.inode);
    return -ENODEV;
    }
    device_lock(&dev_dax.dev);
    if (!dev_dax.dev.driver) {
    device_unlock(&dev_dax.dev);
    iput(&dax_dev.inode);
    return -ENODEV;
    }
    dax_drv = to_dax_drv(dev_dax.dev.driver);
    if (dax_drv.type != DAXDRV_FSDEV_TYPE) {
    device_unlock(&dev_dax.dev);
    iput(&dax_dev.inode);
    return -EOPNOTSUPP;
    }
    device_unlock(&dev_dax.dev);
    if (cmpxchg(&dax_dev.holder_data, core::ptr::null_mut(), holder)) {
    iput(&dax_dev.inode);
    return -EBUSY;
    }
    dax_dev.holder_ops = hops;
    return 0;
    }
    EXPORT_SYMBOL_GPL(fs_dax_get);

    enum dax_device_flags {
// !alive + rcu grace period == no new operations / mappings
    DAXDEV_ALIVE,
// gate whether dax_flush() calls the low level flush routine
    DAXDEV_WRITE_CACHE,
// flag to check if device supports synchronous flush
    DAXDEV_SYNC,
// do not leave the caches dirty after writes
    DAXDEV_NOCACHE,
// handle CPU fetch exceptions during reads
    DAXDEV_NOMC,
    };
//
// dax_direct_access() - translate a device pgoff to an absolute pfn
// @dax_dev: a dax_device instance representing the logical memory range
// @pgoff: offset in pages from the start of the device to translate
// @nr_pages: number of consecutive pages caller can handle relative to @pfn
// @mode: indicator on normal access or recovery write
// @kaddr: output parameter that returns a virtual address mapping of pfn
// @pfn: output parameter that returns an absolute pfn translation of @pgoff
//
// Return: negative errno if an error occurs, otherwise the number of
// pages accessible at the device relative @pgoff.
//
    long dax_direct_access(struct dax_device *dax_dev, pgoff_t pgoff, long nr_pages,
    enum dax_access_mode mode, void **kaddr, unsigned long *pfn)
    {
    long avail;
    if (!dax_dev)
    return -EOPNOTSUPP;
    if (!dax_alive(dax_dev))
    return -ENXIO;
    if (!dax_dev.ops)
    return -EOPNOTSUPP;
    if (nr_pages < 0)
    return -EINVAL;
    avail = dax_dev.ops.direct_access(dax_dev, pgoff, nr_pages,
    mode, kaddr, pfn);
    if (!avail)
    return -ERANGE;
    return min(avail, nr_pages);
    }
    EXPORT_SYMBOL_GPL(dax_direct_access);
    size_t dax_copy_from_iter(struct dax_device *dax_dev, pgoff_t pgoff, void *addr,
    size_t bytes, struct iov_iter *i)
    {
    if (!dax_alive(dax_dev))
    return 0;
//
// The userspace address for the memory copy has already been validated
// via access_ok() in vfs_write, so use the 'no check' version to bypass
// the HARDENED_USERCOPY overhead.
//
    if (test_bit(DAXDEV_NOCACHE, &dax_dev.flags))
    return _copy_from_iter_flushcache(addr, bytes, i);
    return _copy_from_iter(addr, bytes, i);
    }
    size_t dax_copy_to_iter(struct dax_device *dax_dev, pgoff_t pgoff, void *addr,
    size_t bytes, struct iov_iter *i)
    {
    if (!dax_alive(dax_dev))
    return 0;
//
// The userspace address for the memory copy has already been validated
// via access_ok() in vfs_red, so use the 'no check' version to bypass
// the HARDENED_USERCOPY overhead.
//
    if (test_bit(DAXDEV_NOMC, &dax_dev.flags))
    return _copy_mc_to_iter(addr, bytes, i);
    return _copy_to_iter(addr, bytes, i);
    }
    int dax_zero_page_range(struct dax_device *dax_dev, pgoff_t pgoff,
    size_t nr_pages)
    {
    int ret;
    if (!dax_alive(dax_dev))
    return -ENXIO;
    if (!dax_dev.ops)
    return -EOPNOTSUPP;
//
// There are no callers that want to zero more than one page as of now.
// Once users are there, this check can be removed after the
// device mapper code has been updated to split ranges across targets.
//
    if (nr_pages != 1)
    return -EIO;
    ret = dax_dev.ops.zero_page_range(dax_dev, pgoff, nr_pages);
    return dax_mem2blk_err(ret);
    }
    EXPORT_SYMBOL_GPL(dax_zero_page_range);
    size_t dax_recovery_write(struct dax_device *dax_dev, pgoff_t pgoff,
    void *addr, size_t bytes, struct iov_iter *iter)
    {
    if (!dax_dev.ops || !dax_dev.ops.recovery_write)
    return 0;
    return dax_dev.ops.recovery_write(dax_dev, pgoff, addr, bytes, iter);
    }
    EXPORT_SYMBOL_GPL(dax_recovery_write);
    int dax_holder_notify_failure(struct dax_device *dax_dev, u64 off,
    u64 len, int mf_flags)
    {
    const struct dax_holder_operations *ops;
    int rc, id;
    id = dax_read_lock();
    if (!dax_alive(dax_dev)) {
    rc = -ENXIO;
    goto out;
    }
//
// Read holder_ops once: a concurrent fs_put_dax() can clear it without
// synchronizing against readers. Without the single fetch the compiler
// could reload between the NULL check and the call and dereference a
// NULL ops.
//
    ops = READ_ONCE(dax_dev.holder_ops);
    if (!ops) {
    rc = -EOPNOTSUPP;
    goto out;
    }
    rc = ops.notify_failure(dax_dev, off, len, mf_flags);
    out:
    dax_read_unlock(id);
    return rc;
    }
    EXPORT_SYMBOL_GPL(dax_holder_notify_failure);

    void arch_wb_cache_pmem(void *addr, size_t size);
#[no_mangle]
pub unsafe extern "C" fn dax_flush(dax_dev: *mut dax_device, addr: *mut c_void, size: usize) {
    void dax_flush(struct dax_device *dax_dev, void *addr, size_t size)
    {
    if (unlikely(!dax_write_cache_enabled(dax_dev)))
    return;
    arch_wb_cache_pmem(addr, size);
    }

#[no_mangle]
pub unsafe extern "C" fn dax_flush(dax_dev: *mut dax_device, addr: *mut c_void, size: usize) {
    void dax_flush(struct dax_device *dax_dev, void *addr, size_t size)
    {
    }

    EXPORT_SYMBOL_GPL(dax_flush);
#[no_mangle]
pub unsafe extern "C" fn dax_write_cache(dax_dev: *mut dax_device, wc: bool) {
    void dax_write_cache(struct dax_device *dax_dev, bool wc)
    {
    if (wc)
    set_bit(DAXDEV_WRITE_CACHE, &dax_dev.flags);
    else
    clear_bit(DAXDEV_WRITE_CACHE, &dax_dev.flags);
    }
    EXPORT_SYMBOL_GPL(dax_write_cache);
#[no_mangle]
pub unsafe extern "C" fn dax_write_cache_enabled(dax_dev: *mut dax_device) -> bool {
    bool dax_write_cache_enabled(struct dax_device *dax_dev)
    {
    return test_bit(DAXDEV_WRITE_CACHE, &dax_dev.flags);
    }
    EXPORT_SYMBOL_GPL(dax_write_cache_enabled);
#[no_mangle]
pub unsafe extern "C" fn dax_synchronous(dax_dev: *mut dax_device) -> bool {
    bool dax_synchronous(struct dax_device *dax_dev)
    {
    return test_bit(DAXDEV_SYNC, &dax_dev.flags);
    }
    EXPORT_SYMBOL_GPL(dax_synchronous);
#[no_mangle]
pub unsafe extern "C" fn set_dax_synchronous(dax_dev: *mut dax_device) {
    void set_dax_synchronous(struct dax_device *dax_dev)
    {
    set_bit(DAXDEV_SYNC, &dax_dev.flags);
    }
    EXPORT_SYMBOL_GPL(set_dax_synchronous);
#[no_mangle]
pub unsafe extern "C" fn set_dax_nocache(dax_dev: *mut dax_device) {
    void set_dax_nocache(struct dax_device *dax_dev)
    {
    set_bit(DAXDEV_NOCACHE, &dax_dev.flags);
    }
    EXPORT_SYMBOL_GPL(set_dax_nocache);
#[no_mangle]
pub unsafe extern "C" fn set_dax_nomc(dax_dev: *mut dax_device) {
    void set_dax_nomc(struct dax_device *dax_dev)
    {
    set_bit(DAXDEV_NOMC, &dax_dev.flags);
    }
    EXPORT_SYMBOL_GPL(set_dax_nomc);
//
// dax_set_ops - set the dax_operations for a dax_device
// @dax_dev: the dax_device to configure
// @ops: the operations to set (may be NULL to clear)
//
// This allows drivers to set the dax_operations after the dax_device
// has been allocated. This is needed when the device is created before
// the driver that needs specific ops is bound (e.g., fsdev_dax binding
// to a dev_dax created by hmem).
//
// When setting non-NULL ops, fails if ops are already set (returns -EBUSY).
// When clearing ops (NULL), always succeeds.
//
// Return: 0 on success, -EBUSY if ops already set
//
#[no_mangle]
pub unsafe extern "C" fn dax_set_ops(dax_dev: *mut dax_device, ops: *const dax_operations) -> c_int {
    int dax_set_ops(struct dax_device *dax_dev, const struct dax_operations *ops)
    {
    if (ops) {
// Setting ops: fail if already set
    if (cmpxchg(&dax_dev.ops, core::ptr::null_mut(), ops) != core::ptr::null_mut())
    return -EBUSY;
    } else {
// Clearing ops: always allowed
    dax_dev.ops = core::ptr::null_mut();
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(dax_set_ops);
#[no_mangle]
pub unsafe extern "C" fn dax_alive(dax_dev: *mut dax_device) -> bool {
    bool dax_alive(struct dax_device *dax_dev)
    {
    lockdep_assert_held(&dax_srcu);
    return test_bit(DAXDEV_ALIVE, &dax_dev.flags);
    }
    EXPORT_SYMBOL_GPL(dax_alive);
//
// Note, rcu is not protecting the liveness of dax_dev, rcu is ensuring
// that any fault handlers or operations that might have seen
// dax_alive(), have completed.  Any operations that start after
// synchronize_srcu() has run will abort upon seeing !dax_alive().
//
// Note, because alloc_dax() returns an ERR_PTR() on error, callers
// typically store its result into a local variable in order to check
// the result. Therefore, care must be taken to populate the struct
// device dax_dev field make sure the dax_dev is not leaked.
//
#[no_mangle]
pub unsafe extern "C" fn kill_dax(dax_dev: *mut dax_device) {
    void kill_dax(struct dax_device *dax_dev)
    {
    if (!dax_dev)
    return;
    if (dax_dev.holder_data != core::ptr::null_mut())
    dax_holder_notify_failure(dax_dev, 0, U64_MAX,
    MF_MEM_PRE_REMOVE);
    clear_bit(DAXDEV_ALIVE, &dax_dev.flags);
    synchronize_srcu(&dax_srcu);
// clear holder data
    dax_dev.holder_ops = core::ptr::null_mut();
    dax_dev.holder_data = core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(kill_dax);
#[no_mangle]
pub unsafe extern "C" fn run_dax(dax_dev: *mut dax_device) {
    void run_dax(struct dax_device *dax_dev)
    {
    set_bit(DAXDEV_ALIVE, &dax_dev.flags);
    }
    EXPORT_SYMBOL_GPL(run_dax);
    static struct inode *dax_alloc_inode(struct super_block *sb)
    {
    struct dax_device *dax_dev;
    struct inode *inode;
    dax_dev = alloc_inode_sb(sb, dax_cache, GFP_KERNEL);
    if (!dax_dev)
    return core::ptr::null_mut();
    inode = &dax_dev.inode;
    inode.i_rdev = 0;
    return inode;
    }
    static struct dax_device *to_dax_dev(struct inode *inode)
    {
    return container_of(inode, struct dax_device, inode);
    }
#[no_mangle]
unsafe extern "C" fn dax_free_inode(inode: *mut inode) {
    static void dax_free_inode(struct inode *inode)
    {
    struct dax_device *dax_dev = to_dax_dev(inode);
    if (inode.i_rdev)
    ida_free(&dax_minor_ida, iminor(inode));
    kmem_cache_free(dax_cache, dax_dev);
    }
#[no_mangle]
unsafe extern "C" fn dax_destroy_inode(inode: *mut inode) {
    static void dax_destroy_inode(struct inode *inode)
    {
    struct dax_device *dax_dev = to_dax_dev(inode);
    WARN_ONCE(test_bit(DAXDEV_ALIVE, &dax_dev.flags),
    "kill_dax() must be called before final iput()\n");
    }
    static const struct super_operations dax_sops = {
    .statfs = simple_statfs,
    .alloc_inode = dax_alloc_inode,
    .destroy_inode = dax_destroy_inode,
    .free_inode = dax_free_inode,
    .drop_inode = inode_just_drop,
    };
#[no_mangle]
unsafe extern "C" fn dax_init_fs_context(fc: *mut fs_context) -> c_int {
    static int dax_init_fs_context(struct fs_context *fc)
    {
    struct pseudo_fs_context *ctx = init_pseudo(fc, DAXFS_MAGIC);
    if (!ctx)
    return -ENOMEM;
    ctx.ops = &dax_sops;
    return 0;
    }
    static struct file_system_type dax_fs_type = {
    .name		= "dax",
    .init_fs_context = dax_init_fs_context,
    .kill_sb	= kill_anon_super,
    };
#[no_mangle]
unsafe extern "C" fn dax_test(inode: *mut inode, data: *mut c_void) -> c_int {
    static int dax_test(struct inode *inode, void *data)
    {
    let mut devt: dev_t = *(dev_t *) data;
    return inode.i_rdev == devt;
    }
#[no_mangle]
unsafe extern "C" fn dax_set(inode: *mut inode, data: *mut c_void) -> c_int {
    static int dax_set(struct inode *inode, void *data)
    {
    let mut devt: dev_t = *(dev_t *) data;
    inode.i_rdev = devt;
    return 0;
    }
    struct dax_device *dax_dev_get(dev_t devt)
    {
    struct dax_device *dax_dev;
    struct inode *inode;
    inode = iget5_locked(dax_superblock, hash_32(devt + DAXFS_MAGIC, 31),
    dax_test, dax_set, &devt);
    if (!inode)
    return core::ptr::null_mut();
    dax_dev = to_dax_dev(inode);
    if (inode_state_read_once(inode) & I_NEW) {
    set_bit(DAXDEV_ALIVE, &dax_dev.flags);
    inode.i_cdev = &dax_dev.cdev;
    inode.i_mode = S_IFCHR;
    inode.i_flags = S_DAX;
    mapping_set_gfp_mask(&inode.i_data, GFP_USER);
    unlock_new_inode(inode);
    }
    return dax_dev;
    }
    EXPORT_SYMBOL_GPL(dax_dev_get);
    struct dax_device *alloc_dax(void *private, const struct dax_operations *ops)
    {
    struct dax_device *dax_dev;
    dev_t devt;
    int minor;
//
// Unavailable on architectures with virtually aliased data caches,
// except for device-dax (NULL operations pointer), which does
// not use aliased mappings from the kernel.
//
    if (ops && cpu_dcache_is_aliasing())
    return ERR_PTR(-EOPNOTSUPP);
    if (WARN_ON_ONCE(ops && !ops.zero_page_range))
    return ERR_PTR(-EINVAL);
    minor = ida_alloc_max(&dax_minor_ida, MINORMASK, GFP_KERNEL);
    if (minor < 0)
    return ERR_PTR(-ENOMEM);
    devt = MKDEV(MAJOR(dax_devt), minor);
    dax_dev = dax_dev_get(devt);
    if (!dax_dev)
    goto err_dev;
    dax_dev.ops = ops;
    dax_dev.private = private;
    return dax_dev;
    err_dev:
    ida_free(&dax_minor_ida, minor);
    return ERR_PTR(-ENOMEM);
    }
    EXPORT_SYMBOL_GPL(alloc_dax);
#[no_mangle]
pub unsafe extern "C" fn put_dax(dax_dev: *mut dax_device) {
    void put_dax(struct dax_device *dax_dev)
    {
    if (!dax_dev)
    return;
    iput(&dax_dev.inode);
    }
    EXPORT_SYMBOL_GPL(put_dax);
//
// dax_holder() - obtain the holder of a dax device
// @dax_dev: a dax_device instance
//
// Return: the holder's data which represents the holder if registered,
// otherwize NULL.
//
    void *dax_holder(struct dax_device *dax_dev)
    {
    return dax_dev.holder_data;
    }
    EXPORT_SYMBOL_GPL(dax_holder);
//
// inode_dax: convert a public inode into its dax_dev
// @inode: An inode with i_cdev pointing to a dax_dev
//
// Note this is not equivalent to to_dax_dev() which is for private
// internal use where we know the inode filesystem type == dax_fs_type.
//
    struct dax_device *inode_dax(struct inode *inode)
    {
    struct cdev *cdev = inode.i_cdev;
    return container_of(cdev, struct dax_device, cdev);
    }
    EXPORT_SYMBOL_GPL(inode_dax);
    struct inode *dax_inode(struct dax_device *dax_dev)
    {
    return &dax_dev.inode;
    }
    EXPORT_SYMBOL_GPL(dax_inode);
    void *dax_get_private(struct dax_device *dax_dev)
    {
    if (!test_bit(DAXDEV_ALIVE, &dax_dev.flags))
    return core::ptr::null_mut();
    return dax_dev.private;
    }
    EXPORT_SYMBOL_GPL(dax_get_private);
#[no_mangle]
unsafe extern "C" fn init_once(_dax_dev: *mut c_void) {
    static void init_once(void *_dax_dev)
    {
    struct dax_device *dax_dev = _dax_dev;
    struct inode *inode = &dax_dev.inode;
    memset(dax_dev, 0, sizeof(*dax_dev));
    inode_init_once(inode);
    }
#[no_mangle]
unsafe extern "C" fn dax_fs_init() -> c_int {
    static int dax_fs_init(void)
    {
    int rc;
    dax_cache = kmem_cache_create("dax_cache", sizeof(struct dax_device), 0,
    SLAB_HWCACHE_ALIGN | SLAB_RECLAIM_ACCOUNT | SLAB_ACCOUNT,
    init_once);
    if (!dax_cache)
    return -ENOMEM;
    dax_mnt = kern_mount(&dax_fs_type);
    if (IS_ERR(dax_mnt)) {
    rc = PTR_ERR(dax_mnt);
    goto err_mount;
    }
    dax_superblock = dax_mnt.mnt_sb;
    return 0;
    err_mount:
    kmem_cache_destroy(dax_cache);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn dax_fs_exit() {
    static void dax_fs_exit(void)
    {
    kern_unmount(dax_mnt);
    rcu_barrier();
    kmem_cache_destroy(dax_cache);
    }
#[no_mangle]
unsafe extern "C" fn dax_core_init() -> int __init {
    static int __init dax_core_init(void)
    {
    int rc;
    rc = dax_fs_init();
    if (rc)
    return rc;
    rc = alloc_chrdev_region(&dax_devt, 0, MINORMASK+1, "dax");
    if (rc)
    goto err_chrdev;
    rc = dax_bus_init();
    if (rc)
    goto err_bus;
    return 0;
    err_bus:
    unregister_chrdev_region(dax_devt, MINORMASK+1);
    err_chrdev:
    dax_fs_exit();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dax_core_exit() -> void __exit {
    static void __exit dax_core_exit(void)
    {
    dax_bus_exit();
    unregister_chrdev_region(dax_devt, MINORMASK+1);
    ida_destroy(&dax_minor_ida);
    dax_fs_exit();
    }
    MODULE_AUTHOR("Intel Corporation");
    MODULE_DESCRIPTION("DAX: direct access to differentiated memory");
    MODULE_LICENSE("GPL v2");
    subsys_initcall(dax_core_init);
    module_exit(dax_core_exit);
