//! Automatically rewritten from C to Rust
//! Source: drivers/input/gameport/gameport.c
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
// Generic gameport layer
//
// Copyright (c) 1999-2002 Vojtech Pavlik
// Copyright (c) 2005 Dmitry Torokhov
//

    MODULE_AUTHOR("Vojtech Pavlik <vojtech@ucw.cz>");
    MODULE_DESCRIPTION("Generic gameport layer");
    MODULE_LICENSE("GPL");
    let mut use_ktime: static bool = true;
    module_param(use_ktime, bool, 0400);
    MODULE_PARM_DESC(use_ktime, "Use ktime for measuring I/O speed");
//
// gameport_mutex protects entire gameport subsystem and is taken
// every time gameport port or driver registrered or unregistered.
//
    static DEFINE_MUTEX(gameport_mutex);
    static LIST_HEAD(gameport_list);
    static const struct bus_type gameport_bus;
    static void gameport_add_port(struct gameport *gameport);
    static void gameport_attach_driver(struct gameport_driver *drv);
    static void gameport_reconnect_port(struct gameport *gameport);
    static void gameport_disconnect_port(struct gameport *gameport);

#[no_mangle]
unsafe extern "C" fn get_time_pit() -> c_uint {
    static unsigned int get_time_pit(void)
    {
    unsigned long flags;
    unsigned int count;
    raw_spin_lock_irqsave(&i8253_lock, flags);
    outb_p(0x00, 0x43);
    count = inb_p(0x40);
    count |= inb_p(0x40) << 8;
    raw_spin_unlock_irqrestore(&i8253_lock, flags);
    return count;
    }

//
// gameport_measure_speed() measures the gameport i/o speed.
//
#[no_mangle]
unsafe extern "C" fn gameport_measure_speed(gameport: *mut gameport) -> c_int {
    static int gameport_measure_speed(struct gameport *gameport)
    {
    unsigned int i, t, tx;
    u64 t1, t2, t3;
    unsigned long flags;
    if (gameport_open(gameport, core::ptr::null_mut(), GAMEPORT_MODE_RAW))
    return 0;
    tx = ~0;
    for (i = 0; i < 50; i++) {
    local_irq_save(flags);
    t1 = ktime_get_ns();
    for (t = 0; t < 50; t++)
    gameport_read(gameport);
    t2 = ktime_get_ns();
    t3 = ktime_get_ns();
    local_irq_restore(flags);
    udelay(i * 10);
    t = (t2 - t1) - (t3 - t2);
    if (t < tx)
    tx = t;
    }
    gameport_close(gameport);
    t = 1000000 * 50;
    if (tx)
    t /= tx;
    return t;
    }
#[no_mangle]
unsafe extern "C" fn old_gameport_measure_speed(gameport: *mut gameport) -> c_int {
    static int old_gameport_measure_speed(struct gameport *gameport)
    {

    unsigned int i, t, t1, t2, t3, tx;
    unsigned long flags;
    if (gameport_open(gameport, core::ptr::null_mut(), GAMEPORT_MODE_RAW))
    return 0;
    tx = 1 << 30;
    for(i = 0; i < 50; i++) {
    local_irq_save(flags);
    GET_TIME(t1);
    for (t = 0; t < 50; t++) gameport_read(gameport);
    GET_TIME(t2);
    GET_TIME(t3);
    local_irq_restore(flags);
    udelay(i * 10);
    if ((t = DELTA(t2,t1) - DELTA(t3,t2)) < tx) tx = t;
    }
    gameport_close(gameport);
    return 59659 / (tx < 1 ? 1 : tx);

    unsigned int i, t;
    unsigned long tx, t1, t2, flags;
    if (gameport_open(gameport, core::ptr::null_mut(), GAMEPORT_MODE_RAW))
    return 0;
    tx = 1 << 30;
    for(i = 0; i < 50; i++) {
    local_irq_save(flags);
    t1 = rdtsc();
    for (t = 0; t < 50; t++) gameport_read(gameport);
    t2 = rdtsc();
    local_irq_restore(flags);
    udelay(i * 10);
    if (t2 - t1 < tx) tx = t2 - t1;
    }
    gameport_close(gameport);
    return (this_cpu_read(cpu_info.loops_per_jiffy) *
    (unsigned long)HZ / (1000 / 50)) / (tx < 1 ? 1 : tx);

    unsigned int j, t = 0;
    if (gameport_open(gameport, core::ptr::null_mut(), GAMEPORT_MODE_RAW))
    return 0;
    j = jiffies; while (j == jiffies);
    j = jiffies; while (j == jiffies) { t++; gameport_read(gameport); }
    gameport_close(gameport);
    return t * HZ / 1000;

    }
#[no_mangle]
pub unsafe extern "C" fn gameport_start_polling(gameport: *mut gameport) {
    void gameport_start_polling(struct gameport *gameport)
    {
    spin_lock(&gameport.timer_lock);
    if (!gameport.poll_cnt++) {
    BUG_ON(!gameport.poll_handler);
    BUG_ON(!gameport.poll_interval);
    mod_timer(&gameport.poll_timer, jiffies + msecs_to_jiffies(gameport.poll_interval));
    }
    spin_unlock(&gameport.timer_lock);
    }
    EXPORT_SYMBOL(gameport_start_polling);
#[no_mangle]
pub unsafe extern "C" fn gameport_stop_polling(gameport: *mut gameport) {
    void gameport_stop_polling(struct gameport *gameport)
    {
    spin_lock(&gameport.timer_lock);
    if (!--gameport.poll_cnt)
    timer_delete(&gameport.poll_timer);
    spin_unlock(&gameport.timer_lock);
    }
    EXPORT_SYMBOL(gameport_stop_polling);
#[no_mangle]
unsafe extern "C" fn gameport_run_poll_handler(t: *mut timer_list) {
    static void gameport_run_poll_handler(struct timer_list *t)
    {
    struct gameport *gameport = timer_container_of(gameport, t,
    poll_timer);
    gameport.poll_handler(gameport);
    if (gameport.poll_cnt)
    mod_timer(&gameport.poll_timer, jiffies + msecs_to_jiffies(gameport.poll_interval));
    }
//
// Basic gameport -> driver core mappings
//
#[no_mangle]
unsafe extern "C" fn gameport_bind_driver(gameport: *mut gameport, drv: *mut gameport_driver) -> c_int {
    static int gameport_bind_driver(struct gameport *gameport, struct gameport_driver *drv)
    {
    int error;
    gameport.dev.driver = &drv.driver;
    if (drv.connect(gameport, drv)) {
    gameport.dev.driver = core::ptr::null_mut();
    return -ENODEV;
    }
    error = device_bind_driver(&gameport.dev);
    if (error) {
    dev_warn(&gameport.dev,
    "device_bind_driver() failed for %s (%s) and %s, error: %d\n",
    gameport.phys, gameport.name,
    drv.description, error);
    drv.disconnect(gameport);
    gameport.dev.driver = core::ptr::null_mut();
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gameport_find_driver(gameport: *mut gameport) {
    static void gameport_find_driver(struct gameport *gameport)
    {
    int error;
    error = device_attach(&gameport.dev);
    if (error < 0)
    dev_warn(&gameport.dev,
    "device_attach() failed for %s (%s), error: %d\n",
    gameport.phys, gameport.name, error);
    }
//
// Gameport event processing.
//
    enum gameport_event_type {
    GAMEPORT_REGISTER_PORT,
    GAMEPORT_ATTACH_DRIVER,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gameport_event {
    pub type: enum gameport_event_type,
    pub object: *mut c_void,
    pub owner: *mut module,
    pub node: list_head,
}

    static DEFINE_SPINLOCK(gameport_event_lock);	/* protects gameport_event_list */
    static LIST_HEAD(gameport_event_list);
    static struct gameport_event *gameport_get_event(void)
    {
    struct gameport_event *event = core::ptr::null_mut();
    unsigned long flags;
    spin_lock_irqsave(&gameport_event_lock, flags);
    if (!list_empty(&gameport_event_list)) {
    event = list_first_entry(&gameport_event_list,
    struct gameport_event, node);
    list_del_init(&event.node);
    }
    spin_unlock_irqrestore(&gameport_event_lock, flags);
    return event;
    }
#[no_mangle]
unsafe extern "C" fn gameport_free_event(event: *mut gameport_event) {
    static void gameport_free_event(struct gameport_event *event)
    {
    module_put(event.owner);
    kfree(event);
    }
#[no_mangle]
unsafe extern "C" fn gameport_remove_duplicate_events(event: *mut gameport_event) {
    static void gameport_remove_duplicate_events(struct gameport_event *event)
    {
    struct gameport_event *e, *next;
    unsigned long flags;
    spin_lock_irqsave(&gameport_event_lock, flags);
    list_for_each_entry_safe(e, next, &gameport_event_list, node) {
    if (event.object == e.object) {
//
// If this event is of different type we should not
// look further - we only suppress duplicate events
// that were sent back-to-back.
//
    if (event.type != e.type)
    break;
    list_del_init(&e.node);
    gameport_free_event(e);
    }
    }
    spin_unlock_irqrestore(&gameport_event_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn gameport_handle_events(work: *mut work_struct) {
    static void gameport_handle_events(struct work_struct *work)
    {
    struct gameport_event *event;
    mutex_lock(&gameport_mutex);
//
// Note that we handle only one event here to give swsusp
// a chance to freeze kgameportd thread. Gameport events
// should be pretty rare so we are not concerned about
// taking performance hit.
//
    if ((event = gameport_get_event())) {
    switch (event.type) {
    case GAMEPORT_REGISTER_PORT:
    gameport_add_port(event.object);
    break;
    case GAMEPORT_ATTACH_DRIVER:
    gameport_attach_driver(event.object);
    break;
    }
    gameport_remove_duplicate_events(event);
    gameport_free_event(event);
    }
    mutex_unlock(&gameport_mutex);
    }
    static DECLARE_WORK(gameport_event_work, gameport_handle_events);
    static int gameport_queue_event(void *object, struct module *owner,
    enum gameport_event_type event_type)
    {
    unsigned long flags;
    struct gameport_event *event;
    let mut retval: c_int = 0;
    spin_lock_irqsave(&gameport_event_lock, flags);
//
// Scan event list for the other events for the same gameport port,
// starting with the most recent one. If event is the same we
// do not need add new one. If event is of different type we
// need to add this event and should not look further because
// we need to preserve sequence of distinct events.
//
    list_for_each_entry_reverse(event, &gameport_event_list, node) {
    if (event.object == object) {
    if (event.type == event_type)
    goto out;
    break;
    }
    }
    event = kmalloc_obj(*event, GFP_ATOMIC);
    if (!event) {
    pr_err("Not enough memory to queue event %d\n", event_type);
    retval = -ENOMEM;
    goto out;
    }
    if (!try_module_get(owner)) {
    pr_warn("Can't get module reference, dropping event %d\n",
    event_type);
    kfree(event);
    retval = -EINVAL;
    goto out;
    }
    event.type = event_type;
    event.object = object;
    event.owner = owner;
    list_add_tail(&event.node, &gameport_event_list);
    queue_work(system_long_wq, &gameport_event_work);
    out:
    spin_unlock_irqrestore(&gameport_event_lock, flags);
    return retval;
    }
//
// Remove all events that have been submitted for a given object,
// be it a gameport port or a driver.
//
#[no_mangle]
unsafe extern "C" fn gameport_remove_pending_events(object: *mut c_void) {
    static void gameport_remove_pending_events(void *object)
    {
    struct gameport_event *event, *next;
    unsigned long flags;
    spin_lock_irqsave(&gameport_event_lock, flags);
    list_for_each_entry_safe(event, next, &gameport_event_list, node) {
    if (event.object == object) {
    list_del_init(&event.node);
    gameport_free_event(event);
    }
    }
    spin_unlock_irqrestore(&gameport_event_lock, flags);
    }
//
// Destroy child gameport port (if any) that has not been fully registered yet.
//
// Note that we rely on the fact that port can have only one child and therefore
// only one child registration request can be pending. Additionally, children
// are registered by driver's connect() handler so there can't be a grandchild
// pending registration together with a child.
//
    static struct gameport *gameport_get_pending_child(struct gameport *parent)
    {
    struct gameport_event *event;
    struct gameport *gameport, *child = core::ptr::null_mut();
    unsigned long flags;
    spin_lock_irqsave(&gameport_event_lock, flags);
    list_for_each_entry(event, &gameport_event_list, node) {
    if (event.type == GAMEPORT_REGISTER_PORT) {
    gameport = event.object;
    if (gameport.parent == parent) {
    child = gameport;
    break;
    }
    }
    }
    spin_unlock_irqrestore(&gameport_event_lock, flags);
    return child;
    }
//
// Gameport port operations
//
#[no_mangle]
unsafe extern "C" fn gameport_description_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize {
    static ssize_t gameport_description_show(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct gameport *gameport = to_gameport_port(dev);
    return sprintf(buf, "%s\n", gameport.name);
    }
    static DEVICE_ATTR(description, S_IRUGO, gameport_description_show, core::ptr::null_mut());
#[no_mangle]
unsafe extern "C" fn drvctl_store(dev: *mut device, attr: *mut device_attribute, buf: *const c_char, count: usize) -> isize {
    static ssize_t drvctl_store(struct device *dev, struct device_attribute *attr, const char *buf, size_t count)
    {
    struct gameport *gameport = to_gameport_port(dev);
    struct device_driver *drv;
    int error;
    error = mutex_lock_interruptible(&gameport_mutex);
    if (error)
    return error;
    if (!strncmp(buf, "none", count)) {
    gameport_disconnect_port(gameport);
    } else if (!strncmp(buf, "reconnect", count)) {
    gameport_reconnect_port(gameport);
    } else if (!strncmp(buf, "rescan", count)) {
    gameport_disconnect_port(gameport);
    gameport_find_driver(gameport);
    } else if ((drv = driver_find(buf, &gameport_bus)) != core::ptr::null_mut()) {
    gameport_disconnect_port(gameport);
    error = gameport_bind_driver(gameport, to_gameport_driver(drv));
    } else {
    error = -EINVAL;
    }
    mutex_unlock(&gameport_mutex);
    return error ? error : count;
    }
    static DEVICE_ATTR_WO(drvctl);
    static struct attribute *gameport_device_attrs[] = {
    &dev_attr_description.attr,
    &dev_attr_drvctl.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(gameport_device);
#[no_mangle]
unsafe extern "C" fn gameport_release_port(dev: *mut device) {
    static void gameport_release_port(struct device *dev)
    {
    struct gameport *gameport = to_gameport_port(dev);
    kfree(gameport);
    module_put(THIS_MODULE);
    }
#[no_mangle]
pub unsafe extern "C" fn gameport_set_phys(gameport: *mut gameport, fmt: *const c_char, ...) {
    void gameport_set_phys(struct gameport *gameport, const char *fmt, ...)
    {
    va_list args;
    va_start(args, fmt);
    vsnprintf(gameport.phys, sizeof(gameport.phys), fmt, args);
    va_end(args);
    }
    EXPORT_SYMBOL(gameport_set_phys);
#[no_mangle]
unsafe extern "C" fn gameport_default_trigger(gameport: *mut gameport) {
    static void gameport_default_trigger(struct gameport *gameport)
    {

    outb(0xff, gameport.io);

    }
#[no_mangle]
unsafe extern "C" fn gameport_default_read(gameport: *mut gameport) -> c_uchar {
    static unsigned char gameport_default_read(struct gameport *gameport)
    {

    return inb(gameport.io);

    return 0xff;

    }
#[no_mangle]
unsafe extern "C" fn gameport_setup_default_handlers(gameport: *mut gameport) {
    static void gameport_setup_default_handlers(struct gameport *gameport)
    {
    if ((!gameport.trigger || !gameport.read) &&
    !IS_ENABLED(CONFIG_HAS_IOPORT))
    dev_err(&gameport.dev,
    "I/O port access is required for %s (%s) but is not available\n",
    gameport.phys, gameport.name);
    if (!gameport.trigger)
    gameport.trigger = gameport_default_trigger;
    if (!gameport.read)
    gameport.read = gameport_default_read;
    }
//
// Prepare gameport port for registration.
//
#[no_mangle]
unsafe extern "C" fn gameport_init_port(gameport: *mut gameport) {
    static void gameport_init_port(struct gameport *gameport)
    {
    let mut gameport_no: static atomic_t = ATOMIC_INIT(-1);
    __module_get(THIS_MODULE);
    mutex_init(&gameport.drv_mutex);
    device_initialize(&gameport.dev);
    dev_set_name(&gameport.dev, "gameport%lu",
    (unsigned long)atomic_inc_return(&gameport_no));
    gameport.dev.bus = &gameport_bus;
    gameport.dev.release = gameport_release_port;
    if (gameport.parent)
    gameport.dev.parent = &gameport.parent.dev;
    gameport_setup_default_handlers(gameport);
    INIT_LIST_HEAD(&gameport.node);
    spin_lock_init(&gameport.timer_lock);
    timer_setup(&gameport.poll_timer, gameport_run_poll_handler, 0);
    }
//
// Complete gameport port registration.
// Driver core will attempt to find appropriate driver for the port.
//
#[no_mangle]
unsafe extern "C" fn gameport_add_port(gameport: *mut gameport) {
    static void gameport_add_port(struct gameport *gameport)
    {
    int error;
    if (gameport.parent)
    gameport.parent.child = gameport;
    gameport.speed = use_ktime ?
    gameport_measure_speed(gameport) :
    old_gameport_measure_speed(gameport);
    list_add_tail(&gameport.node, &gameport_list);
    if (gameport.io)
    dev_info(&gameport.dev, "%s is %s, io %#x, speed %dkHz\n",
    gameport.name, gameport.phys, gameport.io, gameport.speed);
    else
    dev_info(&gameport.dev, "%s is %s, speed %dkHz\n",
    gameport.name, gameport.phys, gameport.speed);
    error = device_add(&gameport.dev);
    if (error)
    dev_err(&gameport.dev,
    "device_add() failed for %s (%s), error: %d\n",
    gameport.phys, gameport.name, error);
    }
//
// gameport_destroy_port() completes deregistration process and removes
// port from the system
//
#[no_mangle]
unsafe extern "C" fn gameport_destroy_port(gameport: *mut gameport) {
    static void gameport_destroy_port(struct gameport *gameport)
    {
    struct gameport *child;
    child = gameport_get_pending_child(gameport);
    if (child) {
    gameport_remove_pending_events(child);
    put_device(&child.dev);
    }
    if (gameport.parent) {
    gameport.parent.child = core::ptr::null_mut();
    gameport.parent = core::ptr::null_mut();
    }
    if (device_is_registered(&gameport.dev))
    device_del(&gameport.dev);
    list_del_init(&gameport.node);
    gameport_remove_pending_events(gameport);
    put_device(&gameport.dev);
    }
//
// Reconnect gameport port and all its children (re-initialize attached devices)
//
#[no_mangle]
unsafe extern "C" fn gameport_reconnect_port(gameport: *mut gameport) {
    static void gameport_reconnect_port(struct gameport *gameport)
    {
    do {
    if (!gameport.drv || !gameport.drv.reconnect || gameport.drv.reconnect(gameport)) {
    gameport_disconnect_port(gameport);
    gameport_find_driver(gameport);
// Ok, old children are now gone, we are done
    break;
    }
    gameport = gameport.child;
    } while (gameport);
    }
//
// gameport_disconnect_port() unbinds a port from its driver. As a side effect
// all child ports are unbound and destroyed.
//
#[no_mangle]
unsafe extern "C" fn gameport_disconnect_port(gameport: *mut gameport) {
    static void gameport_disconnect_port(struct gameport *gameport)
    {
    struct gameport *s, *parent;
    if (gameport.child) {
//
// Children ports should be disconnected and destroyed
// first, staring with the leaf one, since we don't want
// to do recursion
//
    for (s = gameport; s.child; s = s.child)
// empty */;
    do {
    parent = s.parent;
    device_release_driver(&s.dev);
    gameport_destroy_port(s);
    } while ((s = parent) != gameport);
    }
//
// Ok, no children left, now disconnect this port
//
    device_release_driver(&gameport.dev);
    }
//
// Submits register request to kgameportd for subsequent execution.
// Note that port registration is always asynchronous.
//
#[no_mangle]
pub unsafe extern "C" fn __gameport_register_port(gameport: *mut gameport, owner: *mut module) {
    void __gameport_register_port(struct gameport *gameport, struct module *owner)
    {
    gameport_init_port(gameport);
    gameport_queue_event(gameport, owner, GAMEPORT_REGISTER_PORT);
    }
    EXPORT_SYMBOL(__gameport_register_port);
//
// Synchronously unregisters gameport port.
//
#[no_mangle]
pub unsafe extern "C" fn gameport_unregister_port(gameport: *mut gameport) {
    void gameport_unregister_port(struct gameport *gameport)
    {
    mutex_lock(&gameport_mutex);
    gameport_disconnect_port(gameport);
    gameport_destroy_port(gameport);
    mutex_unlock(&gameport_mutex);
    }
    EXPORT_SYMBOL(gameport_unregister_port);
//
// Gameport driver operations
//
#[no_mangle]
unsafe extern "C" fn description_show(drv: *mut device_driver, buf: *mut c_char) -> isize {
    static ssize_t description_show(struct device_driver *drv, char *buf)
    {
    struct gameport_driver *driver = to_gameport_driver(drv);
    return sprintf(buf, "%s\n", driver.description ? driver.description : "(none)");
    }
    static DRIVER_ATTR_RO(description);
    static struct attribute *gameport_driver_attrs[] = {
    &driver_attr_description.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(gameport_driver);
#[no_mangle]
unsafe extern "C" fn gameport_driver_probe(dev: *mut device) -> c_int {
    static int gameport_driver_probe(struct device *dev)
    {
    struct gameport *gameport = to_gameport_port(dev);
    struct gameport_driver *drv = to_gameport_driver(dev.driver);
    drv.connect(gameport, drv);
    return gameport.drv ? 0 : -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn gameport_driver_remove(dev: *mut device) {
    static void gameport_driver_remove(struct device *dev)
    {
    struct gameport *gameport = to_gameport_port(dev);
    struct gameport_driver *drv = to_gameport_driver(dev.driver);
    drv.disconnect(gameport);
    }
#[no_mangle]
unsafe extern "C" fn gameport_attach_driver(drv: *mut gameport_driver) {
    static void gameport_attach_driver(struct gameport_driver *drv)
    {
    int error;
    error = driver_attach(&drv.driver);
    if (error)
    pr_err("driver_attach() failed for %s, error: %d\n",
    drv.driver.name, error);
    }
    int __gameport_register_driver(struct gameport_driver *drv, struct module *owner,
    const char *mod_name)
    {
    int error;
    drv.driver.bus = &gameport_bus;
    drv.driver.owner = owner;
    drv.driver.mod_name = mod_name;
//
// Temporarily disable automatic binding because probing
// takes long time and we are better off doing it in kgameportd
//
    drv.ignore = true;
    error = driver_register(&drv.driver);
    if (error) {
    pr_err("driver_register() failed for %s, error: %d\n",
    drv.driver.name, error);
    return error;
    }
//
// Reset ignore flag and let kgameportd bind the driver to free ports
//
    drv.ignore = false;
    error = gameport_queue_event(drv, core::ptr::null_mut(), GAMEPORT_ATTACH_DRIVER);
    if (error) {
    driver_unregister(&drv.driver);
    return error;
    }
    return 0;
    }
    EXPORT_SYMBOL(__gameport_register_driver);
#[no_mangle]
pub unsafe extern "C" fn gameport_unregister_driver(drv: *mut gameport_driver) {
    void gameport_unregister_driver(struct gameport_driver *drv)
    {
    struct gameport *gameport;
    mutex_lock(&gameport_mutex);
    drv.ignore = true;	/* so gameport_find_driver ignores it */
    gameport_remove_pending_events(drv);
    start_over:
    list_for_each_entry(gameport, &gameport_list, node) {
    if (gameport.drv == drv) {
    gameport_disconnect_port(gameport);
    gameport_find_driver(gameport);
// we could've deleted some ports, restart
    goto start_over;
    }
    }
    driver_unregister(&drv.driver);
    mutex_unlock(&gameport_mutex);
    }
    EXPORT_SYMBOL(gameport_unregister_driver);
#[no_mangle]
unsafe extern "C" fn gameport_bus_match(dev: *mut device, drv: *const device_driver) -> c_int {
    static int gameport_bus_match(struct device *dev, const struct device_driver *drv)
    {
    const struct gameport_driver *gameport_drv = to_gameport_driver(drv);
    return !gameport_drv.ignore;
    }
    static const struct bus_type gameport_bus = {
    .name		= "gameport",
    .dev_groups	= gameport_device_groups,
    .drv_groups	= gameport_driver_groups,
    .match		= gameport_bus_match,
    .probe		= gameport_driver_probe,
    .remove		= gameport_driver_remove,
    };
#[no_mangle]
unsafe extern "C" fn gameport_set_drv(gameport: *mut gameport, drv: *mut gameport_driver) {
    static void gameport_set_drv(struct gameport *gameport, struct gameport_driver *drv)
    {
    mutex_lock(&gameport.drv_mutex);
    gameport.drv = drv;
    mutex_unlock(&gameport.drv_mutex);
    }
#[no_mangle]
pub unsafe extern "C" fn gameport_open(gameport: *mut gameport, drv: *mut gameport_driver, mode: c_int) -> c_int {
    int gameport_open(struct gameport *gameport, struct gameport_driver *drv, int mode)
    {
    if (gameport.open) {
    if (gameport.open(gameport, mode)) {
    return -1;
    }
    } else {
    if (mode != GAMEPORT_MODE_RAW)
    return -1;
    }
    gameport_set_drv(gameport, drv);
    return 0;
    }
    EXPORT_SYMBOL(gameport_open);
#[no_mangle]
pub unsafe extern "C" fn gameport_close(gameport: *mut gameport) {
    void gameport_close(struct gameport *gameport)
    {
    timer_delete_sync(&gameport.poll_timer);
    gameport.poll_handler = core::ptr::null_mut();
    gameport.poll_interval = 0;
    gameport_set_drv(gameport, core::ptr::null_mut());
    if (gameport.close)
    gameport.close(gameport);
    }
    EXPORT_SYMBOL(gameport_close);
#[no_mangle]
unsafe extern "C" fn gameport_init() -> int __init {
    static int __init gameport_init(void)
    {
    int error;
    error = bus_register(&gameport_bus);
    if (error) {
    pr_err("failed to register gameport bus, error: %d\n", error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gameport_exit() -> void __exit {
    static void __exit gameport_exit(void)
    {
    bus_unregister(&gameport_bus);
//
// There should not be any outstanding events but work may
// still be scheduled so simply cancel it.
//
    cancel_work_sync(&gameport_event_work);
    }
    subsys_initcall(gameport_init);
    module_exit(gameport_exit);
