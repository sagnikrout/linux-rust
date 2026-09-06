//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/lenovo/yb9-kbdock.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Lenovo Yoga Book 9 keyboard-dock detection
//
// The Yoga Book 9 ships with a detachable Bluetooth keyboard that magnetically
// attaches to the bottom screen in one of two positions.  The EC tracks
// attachment state in a 2-bit field called BKBD and signals changes via WMI
// event 0xEB on the WM10 ACPI device (_UID "GMZN").
//
// BKBD values:
// 0 = keyboard detached
// 1 = keyboard docked on the top half of the bottom screen
// 2 = keyboard docked on the bottom half of the bottom screen
// 3 = reserved / not observed
//
// Two WMI interfaces are used (documented in embedded BMOF, WQDD, 20705 bytes):
//
// LENOVO_BTKBD_EVENT (event GUID, 806BD2A2-...)
// WmiDataId(1) uint32 Status — _WED(0xEB) returns EC.BKBD directly.
// The notify callback receives BKBD as an integer; no separate query needed.
//
// LENOVO_FEATURE_STATUS_DATA (block GUID, E7F300FA-...)
// WmiDataId(1) uint32 IDs   = 0x00060000 (feature selector)
// WmiDataId(2) uint32 Status = BKBD value
// Used on probe and resume to read initial state.
//
// The event driver (LENOVO_BTKBD_EVENT) fires a notifier chain on each WMI
// event.  The block driver (LENOVO_FEATURE_STATUS_DATA) owns the input_dev
// and registers a notifier_block to receive those events, eliminating the
// need for shared global state or a mutex.
//
// SW_TABLET_MODE=1 is reported when the keyboard is detached;
// SW_TABLET_MODE=0 when docked in either position (keyboard present).
// The raw BKBD value is exposed via the sysfs attribute "keyboard_position".
//
// Copyright (C) 2026 Dave Carey <carvsdriver@gmail.com>
//

// BKBD encoding
pub const BKBD_DETACHED: c_int = 0;
// LENOVO_FEATURE_STATUS_DATA feature selector
pub const YB9_FEATURE_STATUS_ID: c_uint = 0x00060000u;
//
// LENOVO_FEATURE_STATUS_DATA: 8-byte buffer {uint32 IDs, uint32 Status}.
// IDs is always 0x00060000; Status holds the BKBD value (0–3).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lenovo_feature_status {
    pub id: __le32,
    pub status: __le32,
    pub __packed: },
// ------------------------------------------------------------------
// Notifier chain — event driver fires it, block driver listens
// ------------------------------------------------------------------
    pub BLOCKING_NOTIFIER_HEAD(yb9_kbdock_chain_head): static,
#[no_mangle]
unsafe extern "C" fn devm_yb9_kbdock_unregister_notifier(data: *mut c_void) {
    static void devm_yb9_kbdock_unregister_notifier(void *data)
    {
    pub data: *mut *mut notifier_block nb =,
    pub nb): blocking_notifier_chain_unregister(&yb9_kbdock_chain_head,,
    }
    static int devm_yb9_kbdock_register_notifier(struct device *dev,
    struct notifier_block *nb)
    {
    pub ret: c_int,
    pub nb): ret = blocking_notifier_chain_register(&yb9_kbdock_chain_head,,
    if (ret < 0)
    pub ret: return,
    pub nb): return devm_add_action_or_reset(dev, devm_yb9_kbdock_unregister_notifier,,
    }
// ------------------------------------------------------------------
// Block WMI driver — LENOVO_FEATURE_STATUS_DATA
// (owns input_dev, sysfs, PM resume)
// ------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct yb9_kbdock_data {
    pub wdev: *mut wmi_device,
    pub input_dev: *mut input_dev,
    pub nb: notifier_block,
    pub /: *mut *mut spinlock_t lock; / protects input_report_switch + input_sync,
}

#[no_mangle]
unsafe extern "C" fn yb9_kbdock_query(d: *mut yb9_kbdock_data, bkbd: *mut u32) -> c_int {
    static int yb9_kbdock_query(struct yb9_kbdock_data *d, u32 *bkbd)
    {
    struct wmi_buffer out;
    int ret;
    ret = wmidev_query_block(d.wdev, 0, &out,
    sizeof(struct lenovo_feature_status));
    if (ret)
    return ret;
    struct lenovo_feature_status *fs __free(kfree) = out.data;
    if (le32_to_cpu(fs.id) != YB9_FEATURE_STATUS_ID)
    return -EIO;
// bkbd = le32_to_cpu(fs->status);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn yb9_kbdock_report(d: *mut yb9_kbdock_data, bkbd: u32) {
    static void yb9_kbdock_report(struct yb9_kbdock_data *d, u32 bkbd)
    {
    let mut tablet: c_int = (bkbd == BKBD_DETACHED) ? 1 : 0;
    spin_lock(&d.lock);
    input_report_switch(d.input_dev, SW_TABLET_MODE, tablet);
    input_sync(d.input_dev);
    spin_unlock(&d.lock);
    dev_dbg(&d.wdev.dev, "BKBD=%u SW_TABLET_MODE=%d\n", bkbd, tablet);
    }
#[no_mangle]
unsafe extern "C" fn yb9_kbdock_sync(d: *mut yb9_kbdock_data) -> c_int {
    static int yb9_kbdock_sync(struct yb9_kbdock_data *d)
    {
    u32 bkbd;
    int ret;
    ret = yb9_kbdock_query(d, &bkbd);
    if (ret)
    return ret;
    yb9_kbdock_report(d, bkbd);
    return 0;
    }
    static int yb9_kbdock_nb_call(struct notifier_block *nb,
    unsigned long bkbd, void *unused)
    {
    struct yb9_kbdock_data *d =
    container_of(nb, struct yb9_kbdock_data, nb);
    yb9_kbdock_report(d, bkbd);
    return NOTIFY_DONE;
    }
    static ssize_t keyboard_position_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct yb9_kbdock_data *d = dev_get_drvdata(dev);
    u32 bkbd;
    int ret;
    ret = yb9_kbdock_query(d, &bkbd);
    if (ret)
    return ret;
    return sysfs_emit(buf, "%u\n", bkbd);
    }
    static DEVICE_ATTR_RO(keyboard_position);
    static const struct attribute * const yb9_kbdock_attrs[] = {
    &dev_attr_keyboard_position.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(yb9_kbdock);
#[no_mangle]
unsafe extern "C" fn yb9_kbdock_resume(dev: *mut device) -> c_int {
    static int yb9_kbdock_resume(struct device *dev)
    {
    struct yb9_kbdock_data *d = dev_get_drvdata(dev);
    return yb9_kbdock_sync(d);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(yb9_kbdock_pm_ops, core::ptr::null_mut(), yb9_kbdock_resume);
#[no_mangle]
unsafe extern "C" fn yb9_kbdock_block_probe(wdev: *mut wmi_device, ctx: *const c_void) -> c_int {
    static int yb9_kbdock_block_probe(struct wmi_device *wdev, const void *ctx)
    {
    struct yb9_kbdock_data *d;
    struct input_dev *input_dev;
    int ret;
    d = devm_kzalloc(&wdev.dev, sizeof(*d), GFP_KERNEL);
    if (!d)
    return -ENOMEM;
    d.wdev = wdev;
    spin_lock_init(&d.lock);
    input_dev = devm_input_allocate_device(&wdev.dev);
    if (!input_dev)
    return -ENOMEM;
    input_dev.name		= "Lenovo Yoga Book 9 keyboard dock switch";
    input_dev.phys		= YB9_KBDOCK_QUERY_GUID "/input0";
    input_dev.id.bustype	= BUS_HOST;
    input_set_capability(input_dev, EV_SW, SW_TABLET_MODE);
    ret = input_register_device(input_dev);
    if (ret)
    return ret;
    d.input_dev		= input_dev;
    d.nb.notifier_call	= yb9_kbdock_nb_call;
    ret = devm_yb9_kbdock_register_notifier(&wdev.dev, &d.nb);
    if (ret)
    return ret;
    dev_set_drvdata(&wdev.dev, d);
    return yb9_kbdock_sync(d);
    }
    static const struct wmi_device_id yb9_kbdock_block_id_table[] = {
    { .guid_string = YB9_KBDOCK_QUERY_GUID },
    { }
    };
    static struct wmi_driver yb9_kbdock_block_driver = {
    .driver = {
    .name		= "lenovo-yb9-kbdock",
    .dev_groups	= yb9_kbdock_groups,
    .pm		= pm_sleep_ptr(&yb9_kbdock_pm_ops),
    },
    .id_table	= yb9_kbdock_block_id_table,
    .no_singleton	= true,
    .probe		= yb9_kbdock_block_probe,
    };
// ------------------------------------------------------------------
// Event WMI driver — LENOVO_BTKBD_EVENT
// (fires the notifier chain on each WMI event)
// ------------------------------------------------------------------
    static void yb9_kbdock_notify_new(struct wmi_device *wdev,
    const struct wmi_buffer *data)
    {
//
// _WED(0xEB) returns EC.BKBD directly as a 32-bit integer
// (LENOVO_BTKBD_EVENT WmiDataId(1) uint32 Status).
// Short-buffer guard is handled by .min_event_size below.
//
    let mut bkbd: u32 = le32_to_cpu(*(const __le32 *)data.data);
    blocking_notifier_call_chain(&yb9_kbdock_chain_head, bkbd, core::ptr::null_mut());
    }
    static const struct wmi_device_id yb9_kbdock_event_id_table[] = {
    { .guid_string = YB9_KBDOCK_EVENT_GUID },
    { }
    };
    MODULE_DEVICE_TABLE(wmi, yb9_kbdock_event_id_table);
    static struct wmi_driver yb9_kbdock_event_driver = {
    .driver = {
    .name = "lenovo-yb9-kbdock-event",
    },
    .id_table	= yb9_kbdock_event_id_table,
    .no_singleton	= true,
    .notify_new	= yb9_kbdock_notify_new,
    .min_event_size	= sizeof(__le32),
    };
// ------------------------------------------------------------------
// Module init / exit
// ------------------------------------------------------------------
    static const struct dmi_system_id yb9_kbdock_dmi_table[] __initconst = {
    {
// Lenovo Yoga Book 9 14IAH10
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR,   "LENOVO"),
    DMI_MATCH(DMI_PRODUCT_NAME, "83KJ"),
    },
    },
    { }
    };
#[no_mangle]
unsafe extern "C" fn yb9_kbdock_init() -> int __init {
    static int __init yb9_kbdock_init(void)
    {
    int ret;
    if (!dmi_check_system(yb9_kbdock_dmi_table))
    return -ENODEV;
    ret = wmi_driver_register(&yb9_kbdock_event_driver);
    if (ret)
    return ret;
    ret = wmi_driver_register(&yb9_kbdock_block_driver);
    if (ret) {
    wmi_driver_unregister(&yb9_kbdock_event_driver);
    return ret;
    }
    return 0;
    }
    module_init(yb9_kbdock_init);
#[no_mangle]
unsafe extern "C" fn yb9_kbdock_exit() -> void __exit {
    static void __exit yb9_kbdock_exit(void)
    {
    wmi_driver_unregister(&yb9_kbdock_block_driver);
    wmi_driver_unregister(&yb9_kbdock_event_driver);
    }
    module_exit(yb9_kbdock_exit);
    MODULE_AUTHOR("Dave Carey <carvsdriver@gmail.com>");
    MODULE_DESCRIPTION("Lenovo Yoga Book 9 keyboard dock detection");
    MODULE_LICENSE("GPL");
