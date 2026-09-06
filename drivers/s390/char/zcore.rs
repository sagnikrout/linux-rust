//! Automatically rewritten from C to Rust
//! Source: drivers/s390/char/zcore.c
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


// SPDX-License-Identifier: GPL-1.0+
//
// zcore module to export memory content and register sets for creating system
// dumps on SCSI/NVMe disks (zfcp/nvme dump).
//
// For more information please refer to Documentation/arch/s390/zfcpdump.rst
//
// Copyright IBM Corp. 2003, 2008
// Author(s): Michael Holzheu
//

    enum arch_id {
    ARCH_S390	= 0,
    ARCH_S390X	= 1,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipib_info {
    pub ipib: c_ulong,
    pub checksum: u32,
    pub __attribute__((packed)): },
    pub zcore_dbf: *mut static struct debug_info,
    pub hsa_available: static int,
    pub zcore_dir: *mut static struct dentry,
    pub zcore_ipl_block: *mut static struct ipl_parameter_block,
    pub os_info_flags: static unsigned long,
    pub DEFINE_MUTEX(hsa_buf_mutex): static,
    pub __aligned(PAGE_SIZE): static char hsa_buf[PAGE_SIZE],
//
// Copy memory from HSA to iterator (not reentrant):
//
// @iter:  Iterator where memory should be copied to
// @src:   Start address within HSA where data should be copied
// @count: Size of buffer, which should be copied
//
#[no_mangle]
pub unsafe extern "C" fn memcpy_hsa_iter(iter: *mut iov_iter, src: c_ulong, count: usize) -> usize {
    size_t memcpy_hsa_iter(struct iov_iter *iter, unsigned long src, size_t count)
    {
    pub 0: size_t bytes, copied, res =,
    pub offset: c_ulong,
    if (!hsa_available)
    pub 0: return,
    while (count) {
    if (sclp_sdias_copy(hsa_buf, src / PAGE_SIZE + 2, 1)) {
    pub failed\n"): TRACE("sclp_sdias_copy(),
    }
    pub PAGE_SIZE: offset = src %,
    pub count): bytes = min(PAGE_SIZE - offset,,
    pub iter): copied = copy_to_iter(hsa_buf + offset, bytes,,
    pub copied: count -=,
    pub copied: src +=,
    pub copied: res +=,
    if (copied < bytes)
    }
    pub res: return,
    }
//
// Copy memory from HSA to kernel memory (not reentrant):
//
// @dest:  Kernel or user buffer where memory should be copied to
// @src:   Start address within HSA where data should be copied
// @count: Size of buffer, which should be copied
//
#[no_mangle]
pub unsafe extern "C" fn memcpy_hsa_kernel(dst: *mut c_void, src: c_ulong, count: usize) -> c_int {
    static inline int memcpy_hsa_kernel(void *dst, unsigned long src, size_t count)
    {
    pub iter: iov_iter,
    pub kvec: kvec,
    pub dst: kvec.iov_base =,
    pub count: kvec.iov_len =,
    pub count): iov_iter_kvec(&iter, ITER_DEST, &kvec, 1,,
    if (memcpy_hsa_iter(&iter, src, count) < count)
    pub -EIO: return,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn init_cpu_info() -> int __init {
    static int __init init_cpu_info(void)
    {
    pub sa: *mut save_area,
// get info for boot cpu from lowcore, stored in the HSA
    pub save_area_boot_cpu(): sa =,
    if (!sa)
    pub -ENOMEM: return,
    if (memcpy_hsa_kernel(hsa_buf, __LC_FPREGS_SAVE_AREA, 512) < 0) {
    pub HSA\n"): TRACE("could not copy from,
    pub -EIO: return,
    }
    pub /: *mut *mut save_area_add_regs(sa, hsa_buf); / vx registers are saved in smp.c,
    pub 0: return,
    }
//
// Release the HSA
//
#[no_mangle]
unsafe extern "C" fn release_hsa() {
    static void release_hsa(void)
    {
    pub NULL): diag308(DIAG308_REL_HSA,,
    pub 0: hsa_available =,
    }
    static ssize_t zcore_reipl_write(struct file *filp, const char __user *buf,
    size_t count, loff_t *ppos)
    {
    if (zcore_ipl_block) {
    pub zcore_ipl_block): diag308(DIAG308_SET,,
    if (os_info_flags & OS_INFO_FLAG_REIPL_CLEAR)
    pub NULL): diag308(DIAG308_LOAD_CLEAR,,
// Use special diag308 subcode for CCW normal ipl
    if (zcore_ipl_block.pb0_hdr.pbt == IPL_PBT_CCW)
    pub NULL): diag308(DIAG308_LOAD_NORMAL_DUMP,,
    else
    pub NULL): diag308(DIAG308_LOAD_NORMAL,,
    }
    pub count: return,
    }
#[no_mangle]
unsafe extern "C" fn zcore_reipl_open(inode: *mut inode, filp: *mut file) -> c_int {
    static int zcore_reipl_open(struct inode *inode, struct file *filp)
    {
    pub filp): return stream_open(inode,,
    }
#[no_mangle]
unsafe extern "C" fn zcore_reipl_release(inode: *mut inode, filp: *mut file) -> c_int {
    static int zcore_reipl_release(struct inode *inode, struct file *filp)
    {
    pub 0: return,
    }
    static const struct file_operations zcore_reipl_fops = {
    .owner		= THIS_MODULE,
    .write		= zcore_reipl_write,
    .open		= zcore_reipl_open,
    .release	= zcore_reipl_release,
}

    static ssize_t zcore_hsa_read(struct file *filp, char __user *buf,
    size_t count, loff_t *ppos)
    {
    static char str[18];
    if (hsa_available)
    snprintf(str, sizeof(str), "%lx\n", sclp.hsa_size);
    else
    snprintf(str, sizeof(str), "0\n");
    return simple_read_from_buffer(buf, count, ppos, str, strlen(str));
    }
    static ssize_t zcore_hsa_write(struct file *filp, const char __user *buf,
    size_t count, loff_t *ppos)
    {
    char value;
    if (*ppos != 0)
    return -EPIPE;
    if (copy_from_user(&value, buf, 1))
    return -EFAULT;
    if (value != '0')
    return -EINVAL;
    release_hsa();
    return count;
    }
    static const struct file_operations zcore_hsa_fops = {
    .owner		= THIS_MODULE,
    .write		= zcore_hsa_write,
    .read		= zcore_hsa_read,
    .open		= nonseekable_open,
    };
#[no_mangle]
unsafe extern "C" fn check_sdias() -> int __init {
    static int __init check_sdias(void)
    {
    if (!sclp.hsa_size) {
    TRACE("Could not determine HSA size\n");
    return -ENODEV;
    }
    return 0;
    }
//
// Provide IPL parameter information block from either HSA or memory
// for future reipl
//
#[no_mangle]
unsafe extern "C" fn zcore_reipl_init() -> int __init {
    static int __init zcore_reipl_init(void)
    {
    struct os_info_entry *entry;
    struct ipib_info ipib_info;
    unsigned long os_info_addr;
    struct os_info *os_info;
    int rc;
    rc = memcpy_hsa_kernel(&ipib_info, __LC_DUMP_REIPL, sizeof(ipib_info));
    if (rc)
    return rc;
    if (ipib_info.ipib == 0)
    return 0;
    zcore_ipl_block = (void *) __get_free_page(GFP_KERNEL);
    if (!zcore_ipl_block)
    return -ENOMEM;
    if (ipib_info.ipib < sclp.hsa_size)
    rc = memcpy_hsa_kernel(zcore_ipl_block, ipib_info.ipib,
    PAGE_SIZE);
    else
    rc = memcpy_real(zcore_ipl_block, ipib_info.ipib, PAGE_SIZE);
    if (rc || ( u32)csum_partial(zcore_ipl_block, zcore_ipl_block.hdr.len, 0) !=
    ipib_info.checksum) {
    TRACE("Checksum does not match\n");
    free_page((unsigned long) zcore_ipl_block);
    zcore_ipl_block = core::ptr::null_mut();
    }
//
// Read the bit-flags field from os_info flags entry.
// Return zero even for os_info read or entry checksum errors in order
// to continue dump processing, considering that os_info could be
// corrupted on the panicked system.
//
    os_info = (void *)__get_free_page(GFP_KERNEL);
    if (!os_info)
    return -ENOMEM;
    rc = memcpy_hsa_kernel(&os_info_addr, __LC_OS_INFO, sizeof(os_info_addr));
    if (rc)
    goto out;
    if (os_info_addr < sclp.hsa_size)
    rc = memcpy_hsa_kernel(os_info, os_info_addr, PAGE_SIZE);
    else
    rc = memcpy_real(os_info, os_info_addr, PAGE_SIZE);
    if (rc || os_info_csum(os_info) != os_info.csum)
    goto out;
    entry = &os_info.entry[OS_INFO_FLAGS_ENTRY];
    if (entry.addr && entry.size) {
    if (entry.addr < sclp.hsa_size)
    rc = memcpy_hsa_kernel(&os_info_flags, entry.addr, sizeof(os_info_flags));
    else
    rc = memcpy_real(&os_info_flags, entry.addr, sizeof(os_info_flags));
    if (rc || ( u32)csum_partial(&os_info_flags, entry.size, 0) != entry.csum)
    os_info_flags = 0;
    }
    out:
    free_page((unsigned long)os_info);
    return 0;
    }
    static int zcore_reboot_and_on_panic_handler(struct notifier_block *self,
    unsigned long	   event,
    void		   *data)
    {
    if (hsa_available)
    release_hsa();
    return NOTIFY_OK;
    }
    static struct notifier_block zcore_reboot_notifier = {
    .notifier_call	= zcore_reboot_and_on_panic_handler,
// we need to be notified before reipl and kdump
    .priority	= INT_MAX,
    };
    static struct notifier_block zcore_on_panic_notifier = {
    .notifier_call	= zcore_reboot_and_on_panic_handler,
// we need to be notified before reipl and kdump
    .priority	= INT_MAX,
    };
#[no_mangle]
unsafe extern "C" fn zcore_init() -> int __init {
    static int __init zcore_init(void)
    {
    unsigned char arch;
    int rc;
    if (!is_ipl_type_dump())
    return -ENODATA;
    if (oldmem_data.start)
    return -ENODATA;
    zcore_dbf = debug_register("zcore", 4, 1, 4 * sizeof(long));
    debug_register_view(zcore_dbf, &debug_sprintf_view);
    debug_set_level(zcore_dbf, 6);
    if (ipl_info.type == IPL_TYPE_FCP_DUMP) {
    TRACE("type:   fcp\n");
    TRACE("devno:  %x\n", ipl_info.data.fcp.dev_id.devno);
    TRACE("wwpn:   %llx\n", (unsigned long long) ipl_info.data.fcp.wwpn);
    TRACE("lun:    %llx\n", (unsigned long long) ipl_info.data.fcp.lun);
    } else if (ipl_info.type == IPL_TYPE_NVME_DUMP) {
    TRACE("type:   nvme\n");
    TRACE("fid:    %x\n", ipl_info.data.nvme.fid);
    TRACE("nsid:   %x\n", ipl_info.data.nvme.nsid);
    } else if (ipl_info.type == IPL_TYPE_ECKD_DUMP) {
    TRACE("type:   eckd\n");
    TRACE("devno:  %x\n", ipl_info.data.eckd.dev_id.devno);
    TRACE("ssid:   %x\n", ipl_info.data.eckd.dev_id.ssid);
    }
    rc = sclp_sdias_init();
    if (rc)
    goto fail;
    rc = check_sdias();
    if (rc)
    goto fail;
    hsa_available = 1;
    rc = memcpy_hsa_kernel(&arch, __LC_AR_MODE_ID, 1);
    if (rc)
    goto fail;
    if (arch == ARCH_S390) {
    pr_alert("The 64-bit dump tool cannot be used for a "
    "32-bit system\n");
    rc = -EINVAL;
    goto fail;
    }
    pr_alert("The dump process started for a 64-bit operating system\n");
    rc = init_cpu_info();
    if (rc)
    goto fail;
    rc = zcore_reipl_init();
    if (rc)
    goto fail;
    zcore_dir = debugfs_create_dir("zcore" , core::ptr::null_mut());
    debugfs_create_file("reipl", 0400, zcore_dir, core::ptr::null_mut(), &zcore_reipl_fops);
    debugfs_create_file("hsa", 0600, zcore_dir, core::ptr::null_mut(), &zcore_hsa_fops);
    register_reboot_notifier(&zcore_reboot_notifier);
    atomic_notifier_chain_register(&panic_notifier_list, &zcore_on_panic_notifier);
    return 0;
    fail:
    diag308(DIAG308_REL_HSA, core::ptr::null_mut());
    return rc;
    }
    subsys_initcall(zcore_init);
