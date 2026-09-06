//! Automatically rewritten from C to Rust
//! Source: drivers/xen/xenbus/xenbus_probe_frontend.c
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

    pr_debug("(%s:%d) " fmt "\n",			\
    __func__, __LINE__, ##__VA_ARGS__)

// device/<type>/<id> => <type>-<id>
#[no_mangle]
unsafe extern "C" fn frontend_bus_id(bus_id[XEN_BUS_ID_SIZE]: c_char, nodename: *const c_char) -> c_int {
    static int frontend_bus_id(char bus_id[XEN_BUS_ID_SIZE], const char *nodename)
    {
    nodename = strchr(nodename, '/');
    if (!nodename || strlen(nodename + 1) >= XEN_BUS_ID_SIZE) {
    pr_warn("bad frontend %s\n", nodename);
    return -EINVAL;
    }
    strscpy(bus_id, nodename + 1, XEN_BUS_ID_SIZE);
    if (!strchr(bus_id, '/')) {
    pr_warn("bus_id %s no slash\n", bus_id);
    return -EINVAL;
    }
// strchr(bus_id, '/') = '-';
    return 0;
    }
// device/<typename>/<name>
    static int xenbus_probe_frontend(struct xen_bus_type *bus, const char *type,
    const char *name)
    {
    char *nodename;
    int err;
// ignore console/0
    if (!strncmp(type, "console", 7) && !strncmp(name, "0", 1)) {
    DPRINTK("Ignoring buggy device entry console/0");
    return 0;
    }
    nodename = kasprintf(GFP_KERNEL, "%s/%s/%s", bus.root, type, name);
    if (!nodename)
    return -ENOMEM;
    DPRINTK("%s", nodename);
    err = xenbus_probe_node(bus, type, nodename);
    kfree(nodename);
    return err;
    }
    static int xenbus_uevent_frontend(const struct device *_dev,
    struct kobj_uevent_env *env)
    {
    const struct xenbus_device *dev = to_xenbus_device(_dev);
    if (add_uevent_var(env, "MODALIAS=xen:%s", dev.devicetype))
    return -ENOMEM;
    return 0;
    }
    static void backend_changed(struct xenbus_watch *watch,
    const char *path, const char *token)
    {
    xenbus_otherend_changed(watch, path, token, 1);
    }
#[no_mangle]
unsafe extern "C" fn xenbus_frontend_delayed_restore(w: *mut work_struct) {
    static void xenbus_frontend_delayed_restore(struct work_struct *w)
    {
    struct xenbus_device *xdev = container_of(w, struct xenbus_device, work);
    xenbus_dev_restore(&xdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn xenbus_frontend_dev_restore(dev: *mut device) -> c_int {
    static int xenbus_frontend_dev_restore(struct device *dev)
    {
//
// If xenstored is running in this domain, we cannot access the backend
// state at the moment, so we need to defer xenbus_dev_resume
//
    if (xen_store_domain_type == XS_LOCAL) {
    struct xenbus_device *xdev = to_xenbus_device(dev);
    schedule_work(&xdev.work);
    return 0;
    }
    return xenbus_dev_restore(dev);
    }
#[no_mangle]
unsafe extern "C" fn xenbus_frontend_dev_probe(dev: *mut device) -> c_int {
    static int xenbus_frontend_dev_probe(struct device *dev)
    {
    if (xen_store_domain_type == XS_LOCAL) {
    struct xenbus_device *xdev = to_xenbus_device(dev);
    INIT_WORK(&xdev.work, xenbus_frontend_delayed_restore);
    }
    return xenbus_dev_probe(dev);
    }
#[no_mangle]
unsafe extern "C" fn xenbus_frontend_dev_shutdown(_dev: *mut device) {
    static void xenbus_frontend_dev_shutdown(struct device *_dev)
    {
    struct xenbus_device *dev = to_xenbus_device(_dev);
    let mut timeout: c_ulong = 5*HZ;
    DPRINTK("%s", dev.nodename);
    get_device(&dev.dev);
    if (dev.state != XenbusStateConnected) {
    pr_info("%s: %s: %s != Connected, skipping\n",
    __func__, dev.nodename, xenbus_strstate(dev.state));
    goto out;
    }
    xenbus_switch_state(dev, XenbusStateClosing);
    timeout = wait_for_completion_timeout(&dev.down, timeout);
    if (!timeout)
    pr_info("%s: %s timeout closing device\n",
    __func__, dev.nodename);
    out:
    put_device(&dev.dev);
    }
    static const struct dev_pm_ops xenbus_pm_ops = {
    .freeze		= xenbus_dev_freeze,
    .thaw		= xenbus_dev_thaw,
    .restore	= xenbus_frontend_dev_restore,
    };
    static struct xen_bus_type xenbus_frontend = {
    .root = "device",
    .levels = 2,		/* device/type/<id> */
    .get_bus_id = frontend_bus_id,
    .probe = xenbus_probe_frontend,
    .otherend_changed = backend_changed,
    .bus = {
    .name		= "xen",
    .match		= xenbus_match,
    .uevent		= xenbus_uevent_frontend,
    .probe		= xenbus_frontend_dev_probe,
    .remove		= xenbus_dev_remove,
    .shutdown	= xenbus_frontend_dev_shutdown,
    .dev_groups	= xenbus_dev_groups,
    .pm		= &xenbus_pm_ops,
    },
    };
    static void frontend_changed(struct xenbus_watch *watch,
    const char *path, const char *token)
    {
    DPRINTK("");
    xenbus_dev_changed(path, &xenbus_frontend);
    }
// We watch for devices appearing and vanishing.
    static struct xenbus_watch fe_watch = {
    .node = "device",
    .callback = frontend_changed,
    };
#[no_mangle]
unsafe extern "C" fn read_backend_details(xendev: *mut xenbus_device) -> c_int {
    static int read_backend_details(struct xenbus_device *xendev)
    {
    return xenbus_read_otherend_details(xendev, "backend-id", "backend");
    }
#[no_mangle]
unsafe extern "C" fn is_device_connecting(dev: *mut device, data: *mut c_void, ignore_nonessential: bool) -> c_int {
    static int is_device_connecting(struct device *dev, void *data, bool ignore_nonessential)
    {
    struct xenbus_device *xendev = to_xenbus_device(dev);
    struct device_driver *drv = data;
    struct xenbus_driver *xendrv;
//
// A device with no driver will never connect. We care only about
// devices which should currently be in the process of connecting.
//
    if (!dev.driver)
    return 0;
// Is this search limited to a particular driver?
    if (drv && (dev.driver != drv))
    return 0;
    xendrv = to_xenbus_driver(dev.driver);
    if (ignore_nonessential && xendrv.not_essential)
    return 0;
    return (xendev.state < XenbusStateConnected ||
    (xendev.state == XenbusStateConnected &&
    xendrv.is_ready && !xendrv.is_ready(xendev)));
    }
#[no_mangle]
unsafe extern "C" fn essential_device_connecting(dev: *mut device, data: *mut c_void) -> c_int {
    static int essential_device_connecting(struct device *dev, void *data)
    {
    return is_device_connecting(dev, data, true /* ignore PV[KBB+FB] */);
    }
#[no_mangle]
unsafe extern "C" fn non_essential_device_connecting(dev: *mut device, data: *mut c_void) -> c_int {
    static int non_essential_device_connecting(struct device *dev, void *data)
    {
    return is_device_connecting(dev, data, false);
    }
#[no_mangle]
unsafe extern "C" fn exists_essential_connecting_device(drv: *mut device_driver) -> c_int {
    static int exists_essential_connecting_device(struct device_driver *drv)
    {
    return bus_for_each_dev(&xenbus_frontend.bus, core::ptr::null_mut(), drv,
    essential_device_connecting);
    }
#[no_mangle]
unsafe extern "C" fn exists_non_essential_connecting_device(drv: *mut device_driver) -> c_int {
    static int exists_non_essential_connecting_device(struct device_driver *drv)
    {
    return bus_for_each_dev(&xenbus_frontend.bus, core::ptr::null_mut(), drv,
    non_essential_device_connecting);
    }
#[no_mangle]
unsafe extern "C" fn print_device_status(dev: *mut device, data: *mut c_void) -> c_int {
    static int print_device_status(struct device *dev, void *data)
    {
    struct xenbus_device *xendev = to_xenbus_device(dev);
    struct device_driver *drv = data;
// Is this operation limited to a particular driver?
    if (drv && (dev.driver != drv))
    return 0;
    if (!dev.driver) {
// Information only: is this too noisy?
    pr_info("Device with no driver: %s\n", xendev.nodename);
    } else if (xendev.state < XenbusStateConnected) {
    let mut rstate: enum xenbus_state = XenbusStateUnknown;
    if (xendev.otherend)
    rstate = xenbus_read_driver_state(xendev, xendev.otherend);
    pr_warn("Timeout connecting to device: %s (local state %d, remote state %d)\n",
    xendev.nodename, xendev.state, rstate);
    }
    return 0;
    }
// We only wait for device setup after most initcalls have run.
    static int ready_to_wait_for_devices;
    static bool wait_loop(unsigned long start, unsigned int max_delay,
    unsigned int *seconds_waited)
    {
    if (time_after(jiffies, start + (*seconds_waited+5)*HZ)) {
    if (!*seconds_waited)
    pr_warn("Waiting for devices to initialise: ");
// seconds_waited += 5;
    pr_cont("%us...", max_delay - *seconds_waited);
    if (*seconds_waited == max_delay) {
    pr_cont("\n");
    return true;
    }
    }
    schedule_timeout_interruptible(HZ/10);
    return false;
    }
//
// On a 5-minute timeout, wait for all devices currently configured.  We need
// to do this to guarantee that the filesystems and / or network devices
// needed for boot are available, before we can allow the boot to proceed.
//
// This needs to be on a late_initcall, to happen after the frontend device
// drivers have been initialised, but before the root fs is mounted.
//
// A possible improvement here would be to have the tools add a per-device
// flag to the store entry, indicating whether it is needed at boot time.
// This would allow people who knew what they were doing to accelerate their
// boot slightly, but of course needs tools or manual intervention to set up
// those flags correctly.
//
#[no_mangle]
unsafe extern "C" fn wait_for_devices(xendrv: *mut xenbus_driver) {
    static void wait_for_devices(struct xenbus_driver *xendrv)
    {
    let mut start: c_ulong = jiffies;
    struct device_driver *drv = xendrv ? &xendrv.driver : core::ptr::null_mut();
    let mut seconds_waited: c_uint = 0;
    if (!ready_to_wait_for_devices || !xen_domain())
    return;
    while (exists_non_essential_connecting_device(drv))
    if (wait_loop(start, 30, &seconds_waited))
    break;
// Skips PVKB and PVFB check.
    while (exists_essential_connecting_device(drv))
    if (wait_loop(start, 270, &seconds_waited))
    break;
    if (seconds_waited)
    printk("\n");
    bus_for_each_dev(&xenbus_frontend.bus, core::ptr::null_mut(), drv,
    print_device_status);
    }
    int __xenbus_register_frontend(struct xenbus_driver *drv, struct module *owner,
    const char *mod_name)
    {
    int ret;
    drv.read_otherend_details = read_backend_details;
    ret = xenbus_register_driver_common(drv, &xenbus_frontend,
    owner, mod_name);
    if (ret)
    return ret;
// If this driver is loaded as a module wait for devices to attach.
    wait_for_devices(drv);
    return 0;
    }
    EXPORT_SYMBOL_GPL(__xenbus_register_frontend);
    static DECLARE_WAIT_QUEUE_HEAD(backend_state_wq);
    static int backend_state;
    static void xenbus_reset_backend_state_changed(struct xenbus_watch *w,
    const char *path, const char *token)
    {
    if (xenbus_scanf(XBT_NIL, path, "", "%i",
    &backend_state) != 1)
    backend_state = XenbusStateUnknown;
    printk(KERN_DEBUG "XENBUS: backend %s %s\n",
    path, xenbus_strstate(backend_state));
    wake_up(&backend_state_wq);
    }
#[no_mangle]
unsafe extern "C" fn xenbus_reset_wait_for_backend(be: *mut c_char, expected: c_int) {
    static void xenbus_reset_wait_for_backend(char *be, int expected)
    {
    long timeout;
    timeout = wait_event_interruptible_timeout(backend_state_wq,
    backend_state == expected, 5 * HZ);
    if (timeout <= 0)
    pr_info("backend %s timed out\n", be);
    }
//
// Reset frontend if it is in Connected or Closed state.
// Wait for backend to catch up.
// State Connected happens during kdump, Closed after kexec.
//
#[no_mangle]
unsafe extern "C" fn xenbus_reset_frontend(fe: *mut c_char, be: *mut c_char, be_state: c_int) {
    static void xenbus_reset_frontend(char *fe, char *be, int be_state)
    {
    struct xenbus_watch be_watch;
    printk(KERN_DEBUG "XENBUS: backend %s %s\n",
    be, xenbus_strstate(be_state));
    memset(&be_watch, 0, sizeof(be_watch));
    be_watch.node = kasprintf(GFP_NOIO | __GFP_HIGH, "%s/state", be);
    if (!be_watch.node)
    return;
    be_watch.callback = xenbus_reset_backend_state_changed;
    backend_state = XenbusStateUnknown;
    pr_info("triggering reconnect on %s\n", be);
    register_xenbus_watch(&be_watch);
// fall through to forward backend to state XenbusStateInitialising
    switch (be_state) {
    case XenbusStateConnected:
    xenbus_printf(XBT_NIL, fe, "state", "%d", XenbusStateClosing);
    xenbus_reset_wait_for_backend(be, XenbusStateClosing);
    fallthrough;
    case XenbusStateClosing:
    xenbus_printf(XBT_NIL, fe, "state", "%d", XenbusStateClosed);
    xenbus_reset_wait_for_backend(be, XenbusStateClosed);
    fallthrough;
    case XenbusStateClosed:
    xenbus_printf(XBT_NIL, fe, "state", "%d", XenbusStateInitialising);
    xenbus_reset_wait_for_backend(be, XenbusStateInitWait);
    }
    unregister_xenbus_watch(&be_watch);
    pr_info("reconnect done on %s\n", be);
    kfree(be_watch.node);
    }
#[no_mangle]
unsafe extern "C" fn xenbus_check_frontend(class: *mut c_char, dev: *mut c_char) {
    static void xenbus_check_frontend(char *class, char *dev)
    {
    int be_state, fe_state, err;
    char *backend, *frontend;
    frontend = kasprintf(GFP_NOIO | __GFP_HIGH, "device/%s/%s", class, dev);
    if (!frontend)
    return;
    err = xenbus_scanf(XBT_NIL, frontend, "state", "%i", &fe_state);
    if (err != 1)
    goto out;
    switch (fe_state) {
    case XenbusStateConnected:
    case XenbusStateClosed:
    printk(KERN_DEBUG "XENBUS: frontend %s %s\n",
    frontend, xenbus_strstate(fe_state));
    backend = xenbus_read(XBT_NIL, frontend, "backend", core::ptr::null_mut());
    if (IS_ERR_OR_NULL(backend))
    goto out;
    err = xenbus_scanf(XBT_NIL, backend, "state", "%i", &be_state);
    if (err == 1)
    xenbus_reset_frontend(frontend, backend, be_state);
    kfree(backend);
    break;
    default:
    break;
    }
    out:
    kfree(frontend);
    }
#[no_mangle]
unsafe extern "C" fn xenbus_reset_state() {
    static void xenbus_reset_state(void)
    {
    char **devclass, **dev;
    int devclass_n, dev_n;
    int i, j;
    devclass = xenbus_directory(XBT_NIL, "device", "", &devclass_n);
    if (IS_ERR(devclass))
    return;
    for (i = 0; i < devclass_n; i++) {
    dev = xenbus_directory(XBT_NIL, "device", devclass[i], &dev_n);
    if (IS_ERR(dev))
    continue;
    for (j = 0; j < dev_n; j++)
    xenbus_check_frontend(devclass[i], dev[j]);
    kfree(dev);
    }
    kfree(devclass);
    }
    static int frontend_probe_and_watch(struct notifier_block *notifier,
    unsigned long event,
    void *data)
    {
// reset devices in Connected or Closed state
    if (xen_hvm_domain())
    xenbus_reset_state();
// Enumerate devices in xenstore and watch for changes.
    xenbus_probe_devices(&xenbus_frontend);
    register_xenbus_watch(&fe_watch);
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn xenbus_probe_frontend_init() -> int __init {
    static int __init xenbus_probe_frontend_init(void)
    {
    static struct notifier_block xenstore_notifier = {
    .notifier_call = frontend_probe_and_watch
    };
    int err;
    DPRINTK("");
// Register ourselves with the kernel bus subsystem
    err = bus_register(&xenbus_frontend.bus);
    if (err)
    return err;
    register_xenstore_notifier(&xenstore_notifier);
    return 0;
    }
    subsys_initcall(xenbus_probe_frontend_init);

#[no_mangle]
unsafe extern "C" fn boot_wait_for_devices() -> int __init {
    static int __init boot_wait_for_devices(void)
    {
    if (!xen_has_pv_devices())
    return -ENODEV;
    ready_to_wait_for_devices = 1;
    wait_for_devices(core::ptr::null_mut());
    return 0;
    }
    late_initcall(boot_wait_for_devices);

    MODULE_DESCRIPTION("Xen PV-device frontend support");
    MODULE_LICENSE("GPL");
