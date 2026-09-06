//! Automatically rewritten from C to Rust
//! Source: arch/um/drivers/hostaudio_kern.c
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
// Copyright (C) 2002 Steve Schmidtke
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hostaudio_state {
    pub fd: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hostmixer_state {
    pub fd: c_int,
}

//
// Changed either at boot time or module load time.  At boot, this is
// single-threaded; at module load, multiple modules would each have
// their own copy of these variables.
//
    static char *dsp = HOSTAUDIO_DEV_DSP;
    static char *mixer = HOSTAUDIO_DEV_MIXER;

    "    This is used to specify the host dsp device to the hostaudio driver.\n" \
    "    The default is \"" HOSTAUDIO_DEV_DSP "\".\n\n"

    "    This is used to specify the host mixer device to the hostaudio driver.\n"\
    "    The default is \"" HOSTAUDIO_DEV_MIXER "\".\n\n"
    module_param(dsp, charp, 0644);
    MODULE_PARM_DESC(dsp, DSP_HELP);
    module_param(mixer, charp, 0644);
    MODULE_PARM_DESC(mixer, MIXER_HELP);

#[no_mangle]
unsafe extern "C" fn set_dsp(name: *mut c_char, add: *mut c_int) -> c_int {
    static int set_dsp(char *name, int *add)
    {
// add = 0;
    dsp = name;
    return 0;
    }
    __uml_setup("dsp=", set_dsp, "dsp=<dsp device>\n" DSP_HELP);
#[no_mangle]
unsafe extern "C" fn set_mixer(name: *mut c_char, add: *mut c_int) -> c_int {
    static int set_mixer(char *name, int *add)
    {
// add = 0;
    mixer = name;
    return 0;
    }
    __uml_setup("mixer=", set_mixer, "mixer=<mixer device>\n" MIXER_HELP);

    static DEFINE_MUTEX(hostaudio_mutex);
// /dev/dsp file operations
    static ssize_t hostaudio_read(struct file *file, char __user *buffer,
    size_t count, loff_t *ppos)
    {
    struct hostaudio_state *state = file.private_data;
    void *kbuf;
    int err;

    printk(KERN_DEBUG "hostaudio: read called, count = %d\n", count);

    kbuf = kmalloc(count, GFP_KERNEL);
    if (kbuf == core::ptr::null_mut())
    return -ENOMEM;
    err = os_read_file(state.fd, kbuf, count);
    if (err < 0)
    goto out;
    if (copy_to_user(buffer, kbuf, err))
    err = -EFAULT;
    out:
    kfree(kbuf);
    return err;
    }
    static ssize_t hostaudio_write(struct file *file, const char __user *buffer,
    size_t count, loff_t *ppos)
    {
    struct hostaudio_state *state = file.private_data;
    void *kbuf;
    int err;

    printk(KERN_DEBUG "hostaudio: write called, count = %d\n", count);

    kbuf = memdup_user(buffer, count);
    if (IS_ERR(kbuf))
    return PTR_ERR(kbuf);
    err = os_write_file(state.fd, kbuf, count);
    if (err < 0)
    goto out;
// ppos += err;
    out:
    kfree(kbuf);
    return err;
    }
    static __poll_t hostaudio_poll(struct file *file,
    struct poll_table_struct *wait)
    {

    printk(KERN_DEBUG "hostaudio: poll called (unimplemented)\n");

    return 0;
    }
    static long hostaudio_ioctl(struct file *file,
    unsigned int cmd, unsigned long arg)
    {
    struct hostaudio_state *state = file.private_data;
    let mut data: c_ulong = 0;
    int err;

    printk(KERN_DEBUG "hostaudio: ioctl called, cmd = %u\n", cmd);

    switch(cmd){
    case SNDCTL_DSP_SPEED:
    case SNDCTL_DSP_STEREO:
    case SNDCTL_DSP_GETBLKSIZE:
    case SNDCTL_DSP_CHANNELS:
    case SNDCTL_DSP_SUBDIVIDE:
    case SNDCTL_DSP_SETFRAGMENT:
    if (get_user(data, (int __user *) arg))
    return -EFAULT;
    break;
    default:
    break;
    }
    err = os_ioctl_generic(state.fd, cmd, (unsigned long) &data);
    switch(cmd){
    case SNDCTL_DSP_SPEED:
    case SNDCTL_DSP_STEREO:
    case SNDCTL_DSP_GETBLKSIZE:
    case SNDCTL_DSP_CHANNELS:
    case SNDCTL_DSP_SUBDIVIDE:
    case SNDCTL_DSP_SETFRAGMENT:
    if (put_user(data, (int __user *) arg))
    return -EFAULT;
    break;
    default:
    break;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn hostaudio_open(inode: *mut inode, file: *mut file) -> c_int {
    static int hostaudio_open(struct inode *inode, struct file *file)
    {
    struct hostaudio_state *state;
    let mut r: c_int = 0, w = 0;
    int ret;

    kernel_param_lock(THIS_MODULE);
    printk(KERN_DEBUG "hostaudio: open called (host: %s)\n", dsp);
    kernel_param_unlock(THIS_MODULE);

    state = kmalloc_obj(struct hostaudio_state);
    if (state == core::ptr::null_mut())
    return -ENOMEM;
    if (file.f_mode & FMODE_READ)
    r = 1;
    if (file.f_mode & FMODE_WRITE)
    w = 1;
    kernel_param_lock(THIS_MODULE);
    mutex_lock(&hostaudio_mutex);
    ret = os_open_file(dsp, of_set_rw(OPENFLAGS(), r, w), 0);
    mutex_unlock(&hostaudio_mutex);
    kernel_param_unlock(THIS_MODULE);
    if (ret < 0) {
    kfree(state);
    return ret;
    }
    state.fd = ret;
    file.private_data = state;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hostaudio_release(inode: *mut inode, file: *mut file) -> c_int {
    static int hostaudio_release(struct inode *inode, struct file *file)
    {
    struct hostaudio_state *state = file.private_data;

    printk(KERN_DEBUG "hostaudio: release called\n");

    os_close_file(state.fd);
    kfree(state);
    return 0;
    }
// /dev/mixer file operations
    static long hostmixer_ioctl_mixdev(struct file *file,
    unsigned int cmd, unsigned long arg)
    {
    struct hostmixer_state *state = file.private_data;

    printk(KERN_DEBUG "hostmixer: ioctl called\n");

    return os_ioctl_generic(state.fd, cmd, arg);
    }
#[no_mangle]
unsafe extern "C" fn hostmixer_open_mixdev(inode: *mut inode, file: *mut file) -> c_int {
    static int hostmixer_open_mixdev(struct inode *inode, struct file *file)
    {
    struct hostmixer_state *state;
    let mut r: c_int = 0, w = 0;
    int ret;

    printk(KERN_DEBUG "hostmixer: open called (host: %s)\n", mixer);

    state = kmalloc_obj(struct hostmixer_state);
    if (state == core::ptr::null_mut())
    return -ENOMEM;
    if (file.f_mode & FMODE_READ)
    r = 1;
    if (file.f_mode & FMODE_WRITE)
    w = 1;
    kernel_param_lock(THIS_MODULE);
    mutex_lock(&hostaudio_mutex);
    ret = os_open_file(mixer, of_set_rw(OPENFLAGS(), r, w), 0);
    mutex_unlock(&hostaudio_mutex);
    kernel_param_unlock(THIS_MODULE);
    if (ret < 0) {
    kernel_param_lock(THIS_MODULE);
    printk(KERN_ERR "hostaudio_open_mixdev failed to open '%s', "
    "err = %d\n", dsp, -ret);
    kernel_param_unlock(THIS_MODULE);
    kfree(state);
    return ret;
    }
    file.private_data = state;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hostmixer_release(inode: *mut inode, file: *mut file) -> c_int {
    static int hostmixer_release(struct inode *inode, struct file *file)
    {
    struct hostmixer_state *state = file.private_data;

    printk(KERN_DEBUG "hostmixer: release called\n");

    os_close_file(state.fd);
    kfree(state);
    return 0;
    }
// kernel module operations
    static const struct file_operations hostaudio_fops = {
    .owner          = THIS_MODULE,
    .read           = hostaudio_read,
    .write          = hostaudio_write,
    .poll           = hostaudio_poll,
    .unlocked_ioctl	= hostaudio_ioctl,
    .compat_ioctl	= compat_ptr_ioctl,
    .mmap           = core::ptr::null_mut(),
    .open           = hostaudio_open,
    .release        = hostaudio_release,
    };
    static const struct file_operations hostmixer_fops = {
    .owner          = THIS_MODULE,
    .unlocked_ioctl	= hostmixer_ioctl_mixdev,
    .open           = hostmixer_open_mixdev,
    .release        = hostmixer_release,
    };
    static struct {
    int dev_audio;
    int dev_mixer;
    } module_data;
    MODULE_AUTHOR("Steve Schmidtke");
    MODULE_DESCRIPTION("UML Audio Relay");
    MODULE_LICENSE("GPL");
#[no_mangle]
unsafe extern "C" fn hostaudio_init_module() -> int __init {
    static int __init hostaudio_init_module(void)
    {
    kernel_param_lock(THIS_MODULE);
    printk(KERN_INFO "UML Audio Relay (host dsp = %s, host mixer = %s)\n",
    dsp, mixer);
    kernel_param_unlock(THIS_MODULE);
    module_data.dev_audio = register_sound_dsp(&hostaudio_fops, -1);
    if (module_data.dev_audio < 0) {
    printk(KERN_ERR "hostaudio: couldn't register DSP device!\n");
    return -ENODEV;
    }
    module_data.dev_mixer = register_sound_mixer(&hostmixer_fops, -1);
    if (module_data.dev_mixer < 0) {
    printk(KERN_ERR "hostmixer: couldn't register mixer "
    "device!\n");
    unregister_sound_dsp(module_data.dev_audio);
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hostaudio_cleanup_module() -> void __exit {
    static void __exit hostaudio_cleanup_module (void)
    {
    unregister_sound_mixer(module_data.dev_mixer);
    unregister_sound_dsp(module_data.dev_audio);
    }
    module_init(hostaudio_init_module);
    module_exit(hostaudio_cleanup_module);
