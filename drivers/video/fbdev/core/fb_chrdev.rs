//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/core/fb_chrdev.c
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
// We hold a reference to the fb_info in file->private_data,
// but if the current registered fb has changed, we don't
// actually want to use it.
//
// So look up the fb_info using the inode minor number,
// and just verify it against the reference we have.
//
    static struct fb_info *file_fb_info(struct file *file)
    {
    struct inode *inode = file_inode(file);
    let mut fbidx: c_int = iminor(inode);
    struct fb_info *info = registered_fb[fbidx];
    if (info != file.private_data)
    info = core::ptr::null_mut();
    return info;
    }
#[no_mangle]
unsafe extern "C" fn fb_read(file: *mut file, buf: *mut char __user, count: usize, ppos: *mut loff_t) -> isize {
    static ssize_t fb_read(struct file *file, char __user *buf, size_t count, loff_t *ppos)
    {
    struct fb_info *info = file_fb_info(file);
    if (!info)
    return -ENODEV;
    if (fb_WARN_ON_ONCE(info, !info.fbops.fb_read))
    return -EINVAL;
    if (info.state != FBINFO_STATE_RUNNING)
    return -EPERM;
    return info.fbops.fb_read(info, buf, count, ppos);
    }
#[no_mangle]
unsafe extern "C" fn fb_write(file: *mut file, buf: *const char __user, count: usize, ppos: *mut loff_t) -> isize {
    static ssize_t fb_write(struct file *file, const char __user *buf, size_t count, loff_t *ppos)
    {
    struct fb_info *info = file_fb_info(file);
    if (!info)
    return -ENODEV;
    if (fb_WARN_ON_ONCE(info, !info.fbops.fb_write))
    return -EINVAL;
    if (info.state != FBINFO_STATE_RUNNING)
    return -EPERM;
    return info.fbops.fb_write(info, buf, count, ppos);
    }
    static long do_fb_ioctl(struct fb_info *info, unsigned int cmd,
    unsigned long arg)
    {
    const struct fb_ops *fb;
    struct fb_var_screeninfo var;
    struct fb_fix_screeninfo fix;
    struct fb_cmap cmap_from;
    struct fb_cmap_user cmap;
    void __user *argp = (void __user *)arg;
    let mut ret: c_long = 0;
    switch (cmd) {
    case FBIOGET_VSCREENINFO:
    lock_fb_info(info);
    var = info.var;
    unlock_fb_info(info);
    ret = copy_to_user(argp, &var, sizeof(var)) ? -EFAULT : 0;
    break;
    case FBIOPUT_VSCREENINFO:
    if (copy_from_user(&var, argp, sizeof(var)))
    return -EFAULT;
// only for kernel-internal use
    var.activate &= ~FB_ACTIVATE_KD_TEXT;
    console_lock();
    lock_fb_info(info);
    ret = fb_set_var_from_user(info, &var);
    unlock_fb_info(info);
    console_unlock();
    if (!ret && copy_to_user(argp, &var, sizeof(var)))
    ret = -EFAULT;
    break;
    case FBIOGET_FSCREENINFO:
    lock_fb_info(info);
    memcpy(&fix, &info.fix, sizeof(fix));
    if (info.flags & FBINFO_HIDE_SMEM_START)
    fix.smem_start = 0;
    unlock_fb_info(info);
    ret = copy_to_user(argp, &fix, sizeof(fix)) ? -EFAULT : 0;
    break;
    case FBIOPUTCMAP:
    if (copy_from_user(&cmap, argp, sizeof(cmap)))
    return -EFAULT;
    ret = fb_set_user_cmap(&cmap, info);
    break;
    case FBIOGETCMAP:
    if (copy_from_user(&cmap, argp, sizeof(cmap)))
    return -EFAULT;
    lock_fb_info(info);
    cmap_from = info.cmap;
    unlock_fb_info(info);
    ret = fb_cmap_to_user(&cmap_from, &cmap);
    break;
    case FBIOPAN_DISPLAY:
    if (copy_from_user(&var, argp, sizeof(var)))
    return -EFAULT;
    console_lock();
    lock_fb_info(info);
    ret = fb_pan_display(info, &var);
    unlock_fb_info(info);
    console_unlock();
    if (ret == 0 && copy_to_user(argp, &var, sizeof(var)))
    return -EFAULT;
    break;
    case FBIO_CURSOR:
    ret = -EINVAL;
    break;
    case FBIOGET_CON2FBMAP:
    ret = fbcon_get_con2fb_map_ioctl(argp);
    break;
    case FBIOPUT_CON2FBMAP:
    ret = fbcon_set_con2fb_map_ioctl(argp);
    break;
    case FBIOBLANK:
    if (arg > FB_BLANK_POWERDOWN)
    return -EINVAL;
    console_lock();
    lock_fb_info(info);
    ret = fb_blank_from_user(info, arg);
    unlock_fb_info(info);
    console_unlock();
    break;
    default:
    lock_fb_info(info);
    fb = info.fbops;
    if (fb.fb_ioctl)
    ret = fb.fb_ioctl(info, cmd, arg);
    else
    ret = -ENOTTY;
    unlock_fb_info(info);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn fb_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    static long fb_ioctl(struct file *file, unsigned int cmd, unsigned long arg)
    {
    struct fb_info *info = file_fb_info(file);
    if (!info)
    return -ENODEV;
    return do_fb_ioctl(info, cmd, arg);
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_fix_screeninfo32 {
    pub id: [c_char; 16],
    pub smem_start: compat_caddr_t,
    pub smem_len: u32,
    pub type: u32,
    pub type_aux: u32,
    pub visual: u32,
    pub xpanstep: u16,
    pub ypanstep: u16,
    pub ywrapstep: u16,
    pub line_length: u32,
    pub mmio_start: compat_caddr_t,
    pub mmio_len: u32,
    pub accel: u32,
    pub reserved: [u16; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_cmap32 {
    pub start: u32,
    pub len: u32,
    pub red: compat_caddr_t,
    pub green: compat_caddr_t,
    pub blue: compat_caddr_t,
    pub transp: compat_caddr_t,
}

    static int fb_getput_cmap(struct fb_info *info, unsigned int cmd,
    unsigned long arg)
    {
    struct fb_cmap32 cmap32;
    struct fb_cmap cmap_from;
    struct fb_cmap_user cmap;
    if (copy_from_user(&cmap32, compat_ptr(arg), sizeof(cmap32)))
    return -EFAULT;
    cmap = (struct fb_cmap_user) {
    .start	= cmap32.start,
    .len	= cmap32.len,
    .red	= compat_ptr(cmap32.red),
    .green	= compat_ptr(cmap32.green),
    .blue	= compat_ptr(cmap32.blue),
    .transp	= compat_ptr(cmap32.transp),
    };
    if (cmd == FBIOPUTCMAP)
    return fb_set_user_cmap(&cmap, info);
    lock_fb_info(info);
    cmap_from = info.cmap;
    unlock_fb_info(info);
    return fb_cmap_to_user(&cmap_from, &cmap);
    }
    static int do_fscreeninfo_to_user(struct fb_fix_screeninfo *fix,
    struct fb_fix_screeninfo32 __user *fix32)
    {
    __u32 data;
    int err;
    err = copy_to_user(&fix32.id, &fix.id, sizeof(fix32.id));
    data = (__u32) (unsigned long) fix.smem_start;
    err |= put_user(data, &fix32.smem_start);
    err |= put_user(fix.smem_len, &fix32.smem_len);
    err |= put_user(fix.type, &fix32.type);
    err |= put_user(fix.type_aux, &fix32.type_aux);
    err |= put_user(fix.visual, &fix32.visual);
    err |= put_user(fix.xpanstep, &fix32.xpanstep);
    err |= put_user(fix.ypanstep, &fix32.ypanstep);
    err |= put_user(fix.ywrapstep, &fix32.ywrapstep);
    err |= put_user(fix.line_length, &fix32.line_length);
    data = (__u32) (unsigned long) fix.mmio_start;
    err |= put_user(data, &fix32.mmio_start);
    err |= put_user(fix.mmio_len, &fix32.mmio_len);
    err |= put_user(fix.accel, &fix32.accel);
    err |= copy_to_user(fix32.reserved, fix.reserved,
    sizeof(fix.reserved));
    if (err)
    return -EFAULT;
    return 0;
    }
    static int fb_get_fscreeninfo(struct fb_info *info, unsigned int cmd,
    unsigned long arg)
    {
    struct fb_fix_screeninfo fix;
    lock_fb_info(info);
    fix = info.fix;
    if (info.flags & FBINFO_HIDE_SMEM_START)
    fix.smem_start = 0;
    unlock_fb_info(info);
    return do_fscreeninfo_to_user(&fix, compat_ptr(arg));
    }
    static long fb_compat_ioctl(struct file *file, unsigned int cmd,
    unsigned long arg)
    {
    struct fb_info *info = file_fb_info(file);
    const struct fb_ops *fb;
    let mut ret: c_long = -ENOIOCTLCMD;
    if (!info)
    return -ENODEV;
    fb = info.fbops;
    switch (cmd) {
    case FBIOGET_VSCREENINFO:
    case FBIOPUT_VSCREENINFO:
    case FBIOPAN_DISPLAY:
    case FBIOGET_CON2FBMAP:
    case FBIOPUT_CON2FBMAP:
    arg = (unsigned long) compat_ptr(arg);
    fallthrough;
    case FBIOBLANK:
    ret = do_fb_ioctl(info, cmd, arg);
    break;
    case FBIOGET_FSCREENINFO:
    ret = fb_get_fscreeninfo(info, cmd, arg);
    break;
    case FBIOGETCMAP:
    case FBIOPUTCMAP:
    ret = fb_getput_cmap(info, cmd, arg);
    break;
    default:
    if (fb.fb_compat_ioctl)
    ret = fb.fb_compat_ioctl(info, cmd, arg);
    break;
    }
    return ret;
    }

#[no_mangle]
unsafe extern "C" fn fb_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    static int fb_mmap(struct file *file, struct vm_area_struct *vma)
    {
    struct fb_info *info = file_fb_info(file);
    int res;
    if (!info)
    return -ENODEV;
    if (fb_WARN_ON_ONCE(info, !info.fbops.fb_mmap))
    return -ENODEV;
    mutex_lock(&info.mm_lock);
    res = info.fbops.fb_mmap(info, vma);
    mutex_unlock(&info.mm_lock);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn fb_open(inode: *mut inode, file: *mut file) -> c_int {
    static int fb_open(struct inode *inode, struct file *file)
    __acquires(&info.lock)
    __releases(&info.lock)
    {
    let mut fbidx: c_int = iminor(inode);
    struct fb_info *info;
    let mut res: c_int = 0;
    info = get_fb_info(fbidx);
    if (!info) {
    request_module("fb%d", fbidx);
    info = get_fb_info(fbidx);
    if (!info)
    return -ENODEV;
    }
    if (IS_ERR(info))
    return PTR_ERR(info);
    lock_fb_info(info);
    if (!try_module_get(info.fbops.owner)) {
    res = -ENODEV;
    goto out;
    }
    file.private_data = info;
    if (info.fbops.fb_open) {
    res = info.fbops.fb_open(info, 1);
    if (res)
    module_put(info.fbops.owner);
    }

    if (info.fbdefio)
    fb_deferred_io_open(info, inode, file);

    out:
    unlock_fb_info(info);
    if (res)
    put_fb_info(info);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn fb_release(inode: *mut inode, file: *mut file) -> c_int {
    static int fb_release(struct inode *inode, struct file *file)
    __acquires(&info.lock)
    __releases(&info.lock)
    {
    let mut info: *mut fb_info  const = file.private_data;
    lock_fb_info(info);

    if (info.fbdefio)
    fb_deferred_io_release(info);

    if (info.fbops.fb_release)
    info.fbops.fb_release(info, 1);
    module_put(info.fbops.owner);
    unlock_fb_info(info);
    put_fb_info(info);
    return 0;
    }

    static unsigned long get_fb_unmapped_area(struct file *filp,
    unsigned long addr, unsigned long len,
    unsigned long pgoff, unsigned long flags)
    {
    let mut info: *mut fb_info  const = filp.private_data;
    let mut fb_size: c_ulong = PAGE_ALIGN(info.fix.smem_len);
    if (pgoff > fb_size || len > fb_size - pgoff)
    return -EINVAL;
    return (unsigned long)info.screen_base + pgoff;
    }

    static const struct file_operations fb_fops = {
    .owner = THIS_MODULE,
    .read = fb_read,
    .write = fb_write,
    .unlocked_ioctl = fb_ioctl,

    .compat_ioctl = fb_compat_ioctl,

    .mmap = fb_mmap,
    .open = fb_open,
    .release = fb_release,

    (defined(CONFIG_FB_PROVIDE_GET_FB_UNMAPPED_AREA) && \
    !defined(CONFIG_MMU))
    .get_unmapped_area = get_fb_unmapped_area,

    .fsync = fb_deferred_io_fsync,

    .llseek = default_llseek,
    };
#[no_mangle]
pub unsafe extern "C" fn fb_register_chrdev() -> c_int {
    int fb_register_chrdev(void)
    {
    int ret;
    ret = register_chrdev(FB_MAJOR, "fb", &fb_fops);
    if (ret) {
    pr_err("Unable to get major %d for fb devs\n", FB_MAJOR);
    return ret;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn fb_unregister_chrdev() {
    void fb_unregister_chrdev(void)
    {
    unregister_chrdev(FB_MAJOR, "fb");
    }
