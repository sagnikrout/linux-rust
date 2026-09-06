//! Automatically rewritten from C to Rust
//! Source: drivers/dma-buf/udmabuf.c
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

    let mut list_limit: static int = 1024;
    module_param(list_limit, int, 0644);
    MODULE_PARM_DESC(list_limit, "udmabuf_create_list.count limit. Default is 1024.");
    let mut size_limit_mb: static int = INT_MAX;
    module_param(size_limit_mb, int, 0644);
    MODULE_PARM_DESC(size_limit_mb, "Max size of a dmabuf, in megabytes. Default is INT_MAX.");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udmabuf {
    pub pagecount: pgoff_t,
    pub pages: *mut page,
//
// Unlike pages, pinned_folios is only used for unpin.
// So, nr_pinned is not the same to pagecount, the pinned_folios
// only set each folio which already pinned when udmabuf_create.
// Note that, since a folio may be pinned multiple times, each folio
// can be added to pinned_folios multiple times, depending on how many
// times the folio has been pinned when create.
//
    pub nr_pinned: pgoff_t,
    pub pinned_folios: *mut folio,
    pub sg: *mut sg_table,
    pub sg_dir: enum dma_data_direction,
    pub device: *mut miscdevice,
}

#[no_mangle]
unsafe extern "C" fn udmabuf_vm_fault(vmf: *mut vm_fault) -> vm_fault_t {
    static vm_fault_t udmabuf_vm_fault(struct vm_fault *vmf)
    {
    struct vm_area_struct *vma = vmf.vma;
    struct udmabuf *ubuf = vma.vm_private_data;
    let mut pgoff: pgoff_t = vmf.pgoff;
    unsigned long addr, pfn;
    vm_fault_t ret;
    if (pgoff >= ubuf.pagecount)
    return VM_FAULT_SIGBUS;
    pfn = page_to_pfn(ubuf.pages[pgoff]);
    ret = vmf_insert_pfn(vma, vmf.address, pfn);
    if (ret & VM_FAULT_ERROR)
    return ret;
// pre fault
    pgoff = vma.vm_pgoff;
    addr = vma.vm_start;
    for (; addr < vma.vm_end; pgoff++, addr += PAGE_SIZE) {
    if (addr == vmf.address)
    continue;
    if (WARN_ON(pgoff >= ubuf.pagecount))
    break;
    pfn = page_to_pfn(ubuf.pages[pgoff]);
//
// If the below vmf_insert_pfn() fails, we do not return an
// error here during this pre-fault step. However, an error
// will be returned if the failure occurs when the addr is
// truly accessed.
//
    if (vmf_insert_pfn(vma, addr, pfn) & VM_FAULT_ERROR)
    break;
    }
    return ret;
    }
    static const struct vm_operations_struct udmabuf_vm_ops = {
    .fault = udmabuf_vm_fault,
    };
#[no_mangle]
unsafe extern "C" fn mmap_udmabuf(buf: *mut dma_buf, vma: *mut vm_area_struct) -> c_int {
    static int mmap_udmabuf(struct dma_buf *buf, struct vm_area_struct *vma)
    {
    struct udmabuf *ubuf = buf.priv;
    if ((vma.vm_flags & (VM_SHARED | VM_MAYSHARE)) == 0)
    return -EINVAL;
    vma.vm_ops = &udmabuf_vm_ops;
    vma.vm_private_data = ubuf;
    vm_flags_set(vma, VM_PFNMAP | VM_DONTEXPAND | VM_DONTDUMP);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vmap_udmabuf(buf: *mut dma_buf, map: *mut iosys_map) -> c_int {
    static int vmap_udmabuf(struct dma_buf *buf, struct iosys_map *map)
    {
    struct udmabuf *ubuf = buf.priv;
    void *vaddr;
    dma_resv_assert_held(buf.resv);
    vaddr = vm_map_ram(ubuf.pages, ubuf.pagecount, -1);
    if (!vaddr)
    return -EINVAL;
    iosys_map_set_vaddr(map, vaddr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vunmap_udmabuf(buf: *mut dma_buf, map: *mut iosys_map) {
    static void vunmap_udmabuf(struct dma_buf *buf, struct iosys_map *map)
    {
    struct udmabuf *ubuf = buf.priv;
    dma_resv_assert_held(buf.resv);
    vm_unmap_ram(map.vaddr, ubuf.pagecount);
    }
    static struct sg_table *get_sg_table(struct device *dev, struct dma_buf *buf,
    enum dma_data_direction direction)
    {
    struct udmabuf *ubuf = buf.priv;
    struct sg_table *sg;
    int ret;
    sg = kzalloc_obj(*sg);
    if (!sg)
    return ERR_PTR(-ENOMEM);
    ret = sg_alloc_table_from_pages(sg, ubuf.pages, ubuf.pagecount, 0,
    ubuf.pagecount << PAGE_SHIFT,
    GFP_KERNEL);
    if (ret < 0)
    goto err_alloc;
    ret = dma_map_sgtable(dev, sg, direction, DMA_ATTR_SKIP_CPU_SYNC);
    if (ret < 0)
    goto err_map;
    return sg;
    err_map:
    sg_free_table(sg);
    err_alloc:
    kfree(sg);
    return ERR_PTR(ret);
    }
    static void put_sg_table(struct device *dev, struct sg_table *sg,
    enum dma_data_direction direction)
    {
    dma_unmap_sgtable(dev, sg, direction, DMA_ATTR_SKIP_CPU_SYNC);
    sg_free_table(sg);
    kfree(sg);
    }
    static struct sg_table *map_udmabuf(struct dma_buf_attachment *at,
    enum dma_data_direction direction)
    {
    return get_sg_table(at.dev, at.dmabuf, direction);
    }
    static void unmap_udmabuf(struct dma_buf_attachment *at,
    struct sg_table *sg,
    enum dma_data_direction direction)
    {
    return put_sg_table(at.dev, sg, direction);
    }
#[no_mangle]
unsafe extern "C" fn unpin_all_folios(ubuf: *mut udmabuf) {
    static void unpin_all_folios(struct udmabuf *ubuf)
    {
    pgoff_t i;
    for (i = 0; i < ubuf.nr_pinned; ++i)
    unpin_folio(ubuf.pinned_folios[i]);
    kvfree(ubuf.pinned_folios);
    }
#[no_mangle]
unsafe extern "C" fn init_udmabuf(ubuf: *mut udmabuf, pgcnt: pgoff_t) -> __always_inline int {
    static __always_inline int init_udmabuf(struct udmabuf *ubuf, pgoff_t pgcnt)
    {
    ubuf.pages = kvmalloc_objs(*ubuf.pages, pgcnt);
    if (!ubuf.pages)
    return -ENOMEM;
    ubuf.pinned_folios = kvmalloc_objs(*ubuf.pinned_folios, pgcnt);
    if (!ubuf.pinned_folios)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn deinit_udmabuf(ubuf: *mut udmabuf) -> __always_inline void {
    static __always_inline void deinit_udmabuf(struct udmabuf *ubuf)
    {
    unpin_all_folios(ubuf);
    kvfree(ubuf.pages);
    }
#[no_mangle]
unsafe extern "C" fn release_udmabuf(buf: *mut dma_buf) {
    static void release_udmabuf(struct dma_buf *buf)
    {
    struct udmabuf *ubuf = buf.priv;
    struct device *dev = ubuf.device.this_device;
    if (ubuf.sg)
    put_sg_table(dev, ubuf.sg, ubuf.sg_dir);
    deinit_udmabuf(ubuf);
    kfree(ubuf);
    }
    static int begin_cpu_udmabuf(struct dma_buf *buf,
    enum dma_data_direction direction)
    {
    struct udmabuf *ubuf = buf.priv;
    struct device *dev = ubuf.device.this_device;
    if (!ubuf.sg) {
    ubuf.sg = get_sg_table(dev, buf, direction);
    if (IS_ERR(ubuf.sg)) {
    int ret;
    ret = PTR_ERR(ubuf.sg);
    ubuf.sg = core::ptr::null_mut();
    return ret;
    } else {
    ubuf.sg_dir = direction;
    }
    }
    dma_sync_sgtable_for_cpu(dev, ubuf.sg, direction);
    return 0;
    }
    static int end_cpu_udmabuf(struct dma_buf *buf,
    enum dma_data_direction direction)
    {
    struct udmabuf *ubuf = buf.priv;
    struct device *dev = ubuf.device.this_device;
    if (!ubuf.sg)
    return -EINVAL;
    dma_sync_sgtable_for_device(dev, ubuf.sg, direction);
    return 0;
    }
    static const struct dma_buf_ops udmabuf_ops = {
    .map_dma_buf	   = map_udmabuf,
    .unmap_dma_buf	   = unmap_udmabuf,
    .release	   = release_udmabuf,
    .mmap		   = mmap_udmabuf,
    .vmap		   = vmap_udmabuf,
    .vunmap		   = vunmap_udmabuf,
    .begin_cpu_access  = begin_cpu_udmabuf,
    .end_cpu_access    = end_cpu_udmabuf,
    };

#[no_mangle]
unsafe extern "C" fn check_memfd_seals(memfd: *mut file) -> c_int {
    static int check_memfd_seals(struct file *memfd)
    {
    int seals;
    if (!shmem_file(memfd) && !is_file_hugepages(memfd))
    return -EBADFD;
    seals = memfd_fcntl(memfd, F_GET_SEALS, 0);
    if (seals == -EINVAL)
    return -EBADFD;
    if ((seals & SEALS_WANTED) != SEALS_WANTED ||
    (seals & SEALS_DENIED) != 0)
    return -EINVAL;
    return 0;
    }
    static struct dma_buf *export_udmabuf(struct udmabuf *ubuf,
    struct miscdevice *device)
    {
    DEFINE_DMA_BUF_EXPORT_INFO(exp_info);
    ubuf.device = device;
    exp_info.ops  = &udmabuf_ops;
    exp_info.size = ubuf.pagecount << PAGE_SHIFT;
    exp_info.priv = ubuf;
    exp_info.flags = O_RDWR;
    return dma_buf_export(&exp_info);
    }
    static long udmabuf_pin_folios(struct udmabuf *ubuf, struct file *memfd,
    loff_t start, loff_t size, struct folio **folios)
    {
    let mut nr_pinned: pgoff_t = ubuf.nr_pinned;
    let mut upgcnt: pgoff_t = ubuf.pagecount;
    u32 cur_folio, cur_pgcnt;
    pgoff_t pgoff, pgcnt;
    long nr_folios;
    loff_t end;
    pgcnt = size >> PAGE_SHIFT;
    end = start + (pgcnt << PAGE_SHIFT) - 1;
    nr_folios = memfd_pin_folios(memfd, start, end, folios, pgcnt, &pgoff);
    if (nr_folios <= 0)
    return nr_folios ? nr_folios : -EINVAL;
    cur_pgcnt = 0;
    for (cur_folio = 0; cur_folio < nr_folios; ++cur_folio) {
    let mut subpgoff: pgoff_t = pgoff;
    let mut fsize: usize = folio_size(folios[cur_folio]);
    ubuf.pinned_folios[nr_pinned++] = folios[cur_folio];
    for (; subpgoff < fsize; subpgoff += PAGE_SIZE) {
    ubuf.pages[upgcnt] = folio_page(folios[cur_folio],
    subpgoff >> PAGE_SHIFT);
    ++upgcnt;
    if (++cur_pgcnt >= pgcnt)
    goto end;
    }
//
// In a given range, only the first subpage of the first folio
// has an offset, that is returned by memfd_pin_folios().
// The first subpages of other folios (in the range) have an
// offset of 0.
//
    pgoff = 0;
    }
    end:
    ubuf.pagecount = upgcnt;
    ubuf.nr_pinned = nr_pinned;
    return 0;
    }
    static long udmabuf_create(struct miscdevice *device,
    struct udmabuf_create_list *head,
    struct udmabuf_create_item *list)
    {
    let mut max_nr_folios: c_ulong = 0;
    struct folio **folios = core::ptr::null_mut();
    let mut pgcnt: pgoff_t = 0, pglimit;
    struct udmabuf *ubuf;
    struct dma_buf *dmabuf;
    let mut ret: c_long = -EINVAL;
    u32 i, flags;
    ubuf = kzalloc_obj(*ubuf);
    if (!ubuf)
    return -ENOMEM;
    pglimit = ((u64)size_limit_mb * 1024 * 1024) >> PAGE_SHIFT;
    for (i = 0; i < head.count; i++) {
    pgoff_t subpgcnt;
    if (!PAGE_ALIGNED(list[i].offset))
    goto err_noinit;
    if (!PAGE_ALIGNED(list[i].size))
    goto err_noinit;
    subpgcnt = list[i].size >> PAGE_SHIFT;
    pgcnt += subpgcnt;
    if (pgcnt > pglimit)
    goto err_noinit;
    max_nr_folios = max_t(unsigned long, subpgcnt, max_nr_folios);
    }
    if (!pgcnt)
    goto err_noinit;
    ret = init_udmabuf(ubuf, pgcnt);
    if (ret)
    goto err;
    folios = kvmalloc_objs(*folios, max_nr_folios);
    if (!folios) {
    ret = -ENOMEM;
    goto err;
    }
    for (i = 0; i < head.count; i++) {
    struct file *memfd = fget(list[i].memfd);
    if (!memfd) {
    ret = -EBADFD;
    goto err;
    }
//
// Take the inode lock to protect against concurrent
// memfd_add_seals(), which takes this lock in write mode.
//
    inode_lock_shared(file_inode(memfd));
    ret = check_memfd_seals(memfd);
    if (ret)
    goto out_unlock;
    ret = udmabuf_pin_folios(ubuf, memfd, list[i].offset,
    list[i].size, folios);
    out_unlock:
    inode_unlock_shared(file_inode(memfd));
    fput(memfd);
    if (ret)
    goto err;
    }
    flags = head.flags & UDMABUF_FLAGS_CLOEXEC ? O_CLOEXEC : 0;
    dmabuf = export_udmabuf(ubuf, device);
    if (IS_ERR(dmabuf)) {
    ret = PTR_ERR(dmabuf);
    goto err;
    }
//
// Ownership of ubuf is held by the dmabuf from here.
// If the following dma_buf_fd() fails, dma_buf_put() cleans up both the
// dmabuf and the ubuf (through udmabuf_ops.release).
//
    ret = dma_buf_fd(dmabuf, flags);
    if (ret < 0)
    dma_buf_put(dmabuf);
    kvfree(folios);
    return ret;
    err:
    deinit_udmabuf(ubuf);
    err_noinit:
    kfree(ubuf);
    kvfree(folios);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn udmabuf_ioctl_create(filp: *mut file, arg: c_ulong) -> c_long {
    static long udmabuf_ioctl_create(struct file *filp, unsigned long arg)
    {
    struct udmabuf_create create;
    struct udmabuf_create_list head;
    struct udmabuf_create_item list;
    if (copy_from_user(&create, (void __user *)arg,
    sizeof(create)))
    return -EFAULT;
    head.flags  = create.flags;
    head.count  = 1;
    list.memfd  = create.memfd;
    list.offset = create.offset;
    list.size   = create.size;
    return udmabuf_create(filp.private_data, &head, &list);
    }
#[no_mangle]
unsafe extern "C" fn udmabuf_ioctl_create_list(filp: *mut file, arg: c_ulong) -> c_long {
    static long udmabuf_ioctl_create_list(struct file *filp, unsigned long arg)
    {
    struct udmabuf_create_list head;
    struct udmabuf_create_item *list;
    let mut ret: c_int = -EINVAL;
    u32 lsize;
    if (copy_from_user(&head, (void __user *)arg, sizeof(head)))
    return -EFAULT;
    if (head.count > list_limit)
    return -EINVAL;
    lsize = sizeof(struct udmabuf_create_item) * head.count;
    list = memdup_user((void __user *)(arg + sizeof(head)), lsize);
    if (IS_ERR(list))
    return PTR_ERR(list);
    ret = udmabuf_create(filp.private_data, &head, list);
    kfree(list);
    return ret;
    }
    static long udmabuf_ioctl(struct file *filp, unsigned int ioctl,
    unsigned long arg)
    {
    long ret;
    switch (ioctl) {
    case UDMABUF_CREATE:
    ret = udmabuf_ioctl_create(filp, arg);
    break;
    case UDMABUF_CREATE_LIST:
    ret = udmabuf_ioctl_create_list(filp, arg);
    break;
    default:
    ret = -ENOTTY;
    break;
    }
    return ret;
    }
    static const struct file_operations udmabuf_fops = {
    .owner		= THIS_MODULE,
    .unlocked_ioctl = udmabuf_ioctl,

    .compat_ioctl   = udmabuf_ioctl,

    };
    static struct miscdevice udmabuf_misc = {
    .minor          = MISC_DYNAMIC_MINOR,
    .name           = "udmabuf",
    .fops           = &udmabuf_fops,
    };
#[no_mangle]
unsafe extern "C" fn udmabuf_dev_init() -> int __init {
    static int __init udmabuf_dev_init(void)
    {
    int ret;
    ret = misc_register(&udmabuf_misc);
    if (ret < 0) {
    pr_err("Could not initialize udmabuf device\n");
    return ret;
    }
    ret = dma_coerce_mask_and_coherent(udmabuf_misc.this_device,
    DMA_BIT_MASK(64));
    if (ret < 0) {
    pr_err("Could not setup DMA mask for udmabuf device\n");
    misc_deregister(&udmabuf_misc);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn udmabuf_dev_exit() -> void __exit {
    static void __exit udmabuf_dev_exit(void)
    {
    misc_deregister(&udmabuf_misc);
    }
    module_init(udmabuf_dev_init)
    module_exit(udmabuf_dev_exit)
    MODULE_AUTHOR("Gerd Hoffmann <kraxel@redhat.com>");
