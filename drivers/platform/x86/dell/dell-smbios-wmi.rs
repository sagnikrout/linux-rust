//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/dell/dell-smbios-wmi.c
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
// WMI methods for use with dell-smbios
//
// Copyright (c) 2017 Dell Inc.
//

    static int wmi_supported;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct misc_bios_flags_structure {
    pub header: dmi_header,
    pub flags0: u16,
    pub __packed: },
pub const FLAG_HAS_ACPI_WMI: c_uint = 0x02;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_smbios_priv {
    pub /: *mut *mut mutex call_lock; / Protects the content of the SMBIOS buffer,
    pub buf: *mut dell_wmi_smbios_buffer,
    pub wdev: *mut wmi_device,
    pub child: *mut device,
    pub req_buf_size: u64,
    pub char_dev: miscdevice,
}

    static DECLARE_RWSEM(chardev_lock);	/* Protects chardev_priv */
    static struct wmi_smbios_priv *chardev_priv;
#[no_mangle]
unsafe extern "C" fn run_smbios_call(wdev: *mut wmi_device) -> c_int {
    static int run_smbios_call(struct wmi_device *wdev)
    {
    struct wmi_smbios_priv *priv = dev_get_drvdata(&wdev.dev);
    const struct wmi_buffer input = {
    .length = priv.req_buf_size - sizeof(u64),
    .data = &priv.buf.std,
    };
    struct wmi_buffer output;
    int ret;
    dev_dbg(&wdev.dev, "evaluating: %u/%u [%x,%x,%x,%x]\n",
    priv.buf.std.cmd_class, priv.buf.std.cmd_select,
    priv.buf.std.input[0], priv.buf.std.input[1],
    priv.buf.std.input[2], priv.buf.std.input[3]);
//
// The output buffer returned by the WMI method should have at least the size
// of the input buffer.
//
    ret = wmidev_invoke_method(wdev, 0, 1, &input, &output, input.length);
    if (ret < 0)
    return ret;
    memcpy(input.data, output.data, input.length);
    kfree(output.data);
    dev_dbg(&wdev.dev, "result: [%08x,%08x,%08x,%08x]\n",
    priv.buf.std.output[0], priv.buf.std.output[1],
    priv.buf.std.output[2], priv.buf.std.output[3]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dell_smbios_wmi_call(dev: *mut device, buffer: *mut calling_interface_buffer) -> c_int {
    static int dell_smbios_wmi_call(struct device *dev, struct calling_interface_buffer *buffer)
    {
    struct wmi_smbios_priv *priv = dev_get_drvdata(dev);
    size_t difference;
    size_t size;
    int ret;
    size = sizeof(struct calling_interface_buffer);
    difference = priv.req_buf_size - sizeof(u64) - size;
    guard(mutex)(&priv.call_lock);
    memset(&priv.buf.ext, 0, difference);
    memcpy(&priv.buf.std, buffer, size);
    ret = run_smbios_call(priv.wdev);
    memcpy(buffer, &priv.buf.std, size);
    return ret;
    }
    static ssize_t dell_smbios_wmi_read(struct file *filp, char __user *buffer, size_t length,
    loff_t *offset)
    {
    guard(rwsem_read)(&chardev_lock);
    if (!chardev_priv)
    return -ENODEV;
    return simple_read_from_buffer(buffer, length, offset, &chardev_priv.req_buf_size,
    sizeof(chardev_priv.req_buf_size));
    }
    static long dell_smbios_wmi_do_ioctl(struct wmi_smbios_priv *priv,
    struct dell_wmi_smbios_buffer __user *arg)
    {
    long ret;
    if (get_user(priv.buf.length, &arg.length))
    return -EFAULT;
    if (priv.buf.length < priv.req_buf_size)
    return -EINVAL;
// if it's too big, warn, driver will only use what is needed
    if (priv.buf.length > priv.req_buf_size)
    dev_err(&priv.wdev.dev, "Buffer %llu is bigger than required %llu\n",
    priv.buf.length, priv.req_buf_size);
    if (copy_from_user(priv.buf, arg, priv.req_buf_size))
    return -EFAULT;
    if (dell_smbios_call_filter(&priv.wdev.dev, &priv.buf.std)) {
    dev_err(&priv.wdev.dev, "Invalid call %d/%d:%8x\n",
    priv.buf.std.cmd_class,
    priv.buf.std.cmd_select,
    priv.buf.std.input[0]);
    return -EINVAL;
    }
    ret = run_smbios_call(priv.wdev);
    if (ret)
    return ret;
    if (copy_to_user(arg, priv.buf, priv.req_buf_size))
    return -EFAULT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dell_smbios_wmi_ioctl(filp: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    static long dell_smbios_wmi_ioctl(struct file *filp, unsigned int cmd, unsigned long arg)
    {
    struct dell_wmi_smbios_buffer __user *input = (struct dell_wmi_smbios_buffer __user *)arg;
    if (cmd != DELL_WMI_SMBIOS_CMD)
    return -ENOIOCTLCMD;
    guard(rwsem_read)(&chardev_lock);
    if (!chardev_priv)
    return -ENODEV;
    guard(mutex)(&chardev_priv.call_lock);
    return dell_smbios_wmi_do_ioctl(chardev_priv, input);
    }
    static const struct file_operations dell_smbios_wmi_fops = {
    .owner		= THIS_MODULE,
    .read		= dell_smbios_wmi_read,
    .unlocked_ioctl	= dell_smbios_wmi_ioctl,
    .compat_ioctl	= compat_ptr_ioctl,
    };
#[no_mangle]
unsafe extern "C" fn dell_smbios_wmi_unregister_chardev(data: *mut c_void) {
    static void dell_smbios_wmi_unregister_chardev(void *data)
    {
    struct miscdevice *char_dev = data;
    misc_deregister(char_dev);
    }
#[no_mangle]
unsafe extern "C" fn dell_smbios_wmi_clear_chardev(data: *mut c_void) {
    static void dell_smbios_wmi_clear_chardev(void *data)
    {
    guard(rwsem_write)(&chardev_lock);
    chardev_priv = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn dell_smbios_wmi_register_chardev(priv: *mut wmi_smbios_priv) -> c_int {
    static int dell_smbios_wmi_register_chardev(struct wmi_smbios_priv *priv)
    {
    int ret;
    scoped_guard(rwsem_write, &chardev_lock) {
// We can only have a single chardev at a time
    if (chardev_priv)
    return -EBUSY;
    chardev_priv = priv;
    }
    ret = devm_add_action_or_reset(&priv.wdev.dev, dell_smbios_wmi_clear_chardev, core::ptr::null_mut());
    if (ret < 0)
    return ret;
    priv.char_dev.minor = MISC_DYNAMIC_MINOR;
    priv.char_dev.name = "wmi/dell-smbios";
    priv.char_dev.fops = &dell_smbios_wmi_fops;
    priv.char_dev.mode = 0444;
    ret = misc_register(&priv.char_dev);
    if (ret < 0)
    return ret;
    return devm_add_action_or_reset(&priv.wdev.dev, dell_smbios_wmi_unregister_chardev,
    &priv.char_dev);
    }
#[no_mangle]
unsafe extern "C" fn dell_smbios_wmi_probe(wdev: *mut wmi_device, context: *const c_void) -> c_int {
    static int dell_smbios_wmi_probe(struct wmi_device *wdev, const void *context)
    {
    struct wmi_smbios_priv *priv;
    u32 buffer_size, hotfix;
    int count;
    int ret;
    ret = dell_wmi_get_descriptor_valid();
    if (ret)
    return ret;
    priv = devm_kzalloc(&wdev.dev, sizeof(struct wmi_smbios_priv),
    GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.wdev = wdev;
    dev_set_drvdata(&wdev.dev, priv);
// WMI buffer size will be either 4k or 32k depending on machine
    if (!dell_wmi_get_size(&buffer_size))
    return -EPROBE_DEFER;
    priv.req_buf_size = buffer_size;
// some SMBIOS calls fail unless BIOS contains hotfix
    if (!dell_wmi_get_hotfix(&hotfix))
    return -EPROBE_DEFER;
    if (!hotfix)
    dev_warn(&wdev.dev,
    "WMI SMBIOS userspace interface not supported(%u), try upgrading to a newer BIOS\n",
    hotfix);
// add in the length object we will use internally with ioctl
    priv.req_buf_size += sizeof(u64);
    count = get_order(priv.req_buf_size);
    priv.buf = (void *)devm_get_free_pages(&wdev.dev, GFP_KERNEL, count);
    if (!priv.buf)
    return -ENOMEM;
    ret = devm_mutex_init(&wdev.dev, &priv.call_lock);
    if (ret)
    return ret;
    ret = dell_smbios_wmi_register_chardev(priv);
    if (ret)
    return ret;
    return dell_smbios_register_device(&wdev.dev, 1, &dell_smbios_wmi_call);
    }
#[no_mangle]
unsafe extern "C" fn dell_smbios_wmi_remove(wdev: *mut wmi_device) {
    static void dell_smbios_wmi_remove(struct wmi_device *wdev)
    {
    dell_smbios_unregister_device(&wdev.dev);
    }
    static const struct wmi_device_id dell_smbios_wmi_id_table[] = {
    { .guid_string = DELL_WMI_SMBIOS_GUID },
    { },
    };
#[no_mangle]
unsafe extern "C" fn parse_b1_table(dm: *const dmi_header) {
    static void parse_b1_table(const struct dmi_header *dm)
    {
    struct misc_bios_flags_structure *flags =
    container_of(dm, struct misc_bios_flags_structure, header);
// 4 bytes header, 8 bytes flags
    if (dm.length < 12)
    return;
    if (dm.handle != 0xb100)
    return;
    if ((flags.flags0 & FLAG_HAS_ACPI_WMI))
    wmi_supported = 1;
    }
#[no_mangle]
unsafe extern "C" fn find_b1(dm: *const dmi_header, dummy: *mut c_void) {
    static void find_b1(const struct dmi_header *dm, void *dummy)
    {
    switch (dm.type) {
    case 0xb1: /* misc bios flags */
    parse_b1_table(dm);
    break;
    }
    }
    static struct wmi_driver dell_smbios_wmi_driver = {
    .driver = {
    .name = "dell-smbios",
    },
    .probe = dell_smbios_wmi_probe,
    .remove = dell_smbios_wmi_remove,
    .id_table = dell_smbios_wmi_id_table,
    .no_singleton = true,
    };
#[no_mangle]
pub unsafe extern "C" fn init_dell_smbios_wmi() -> c_int {
    int init_dell_smbios_wmi(void)
    {
    dmi_walk(find_b1, core::ptr::null_mut());
    if (!wmi_supported)
    return -ENODEV;
    return wmi_driver_register(&dell_smbios_wmi_driver);
    }
#[no_mangle]
pub unsafe extern "C" fn exit_dell_smbios_wmi() {
    void exit_dell_smbios_wmi(void)
    {
    if (wmi_supported)
    wmi_driver_unregister(&dell_smbios_wmi_driver);
    }
    MODULE_DEVICE_TABLE(wmi, dell_smbios_wmi_id_table);
