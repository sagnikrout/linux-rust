//! Automatically rewritten from C to Rust
//! Source: kernel/time/clockevents.c
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
// This file contains functions which manage clock event devices.
//
// Copyright(C) 2005-2006, Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>
// Copyright(C) 2005-2007, Red Hat, Inc., Ingo Molnar
// Copyright(C) 2006-2007, Timesys Corp., Thomas Gleixner
//

// The registered clock event devices
    static LIST_HEAD(clockevent_devices);
    static LIST_HEAD(clockevents_released);
// Protection for the above
    static DEFINE_RAW_SPINLOCK(clockevents_lock);
// Protection for unbind operations
    static DEFINE_MUTEX(clockevents_mutex);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ce_unbind {
    pub ce: *mut clock_event_device,
    pub res: c_int,
}

    static u64 cev_delta2ns(unsigned long latch, struct clock_event_device *evt,
    bool ismax)
    {
    let mut clc: u64 = (u64) latch << evt.shift;
    u64 rnd;
    if (WARN_ON(!evt.mult))
    evt.mult = 1;
    rnd = (u64) evt.mult - 1;
//
// Upper bound sanity check. If the backwards conversion is
// not equal latch, we know that the above shift overflowed.
//
    if ((clc >> evt.shift) != (u64)latch)
    clc = ~0ULL;
//
// Scaled math oddities:
//
// For mult <= (1 << shift) we can safely add mult - 1 to
// prevent integer rounding loss. So the backwards conversion
// from nsec to device ticks will be correct.
//
// For mult > (1 << shift), i.e. device frequency is > 1GHz we
// need to be careful. Adding mult - 1 will result in a value
// which when converted back to device ticks can be larger
// than latch by up to (mult - 1) >> shift. For the min_delta
// calculation we still want to apply this in order to stay
// above the minimum device ticks limit. For the upper limit
// we would end up with a latch value larger than the upper
// limit of the device, so we omit the add to stay below the
// device upper boundary.
//
// Also omit the add if it would overflow the u64 boundary.
//
    if ((~0ULL - clc > rnd) &&
    (!ismax || evt.mult <= (1ULL << evt.shift)))
    clc += rnd;
    do_div(clc, evt.mult);
// Deltas less than 1usec are pointless noise
    return clc > 1000 ? clc : 1000;
    }
//
// clockevent_delta2ns - Convert a latch value (device ticks) to nanoseconds
// @latch:	value to convert
// @evt:	pointer to clock event device descriptor
//
// Math helper, returns latch value converted to nanoseconds (bound checked)
//
#[no_mangle]
pub unsafe extern "C" fn clockevent_delta2ns(latch: c_ulong, evt: *mut clock_event_device) -> u64 {
    u64 clockevent_delta2ns(unsigned long latch, struct clock_event_device *evt)
    {
    return cev_delta2ns(latch, evt, false);
    }
    EXPORT_SYMBOL_GPL(clockevent_delta2ns);
    static int __clockevents_switch_state(struct clock_event_device *dev,
    enum clock_event_state state)
    {
    if (dev.features & CLOCK_EVT_FEAT_DUMMY)
    return 0;
// On state transitions clear the forced flag unconditionally
    dev.next_event_forced = 0;
// Transition with new state-specific callbacks
    switch (state) {
    case CLOCK_EVT_STATE_DETACHED:
// The clockevent device is getting replaced. Shut it down.
    case CLOCK_EVT_STATE_SHUTDOWN:
    if (dev.set_state_shutdown)
    return dev.set_state_shutdown(dev);
    return 0;
    case CLOCK_EVT_STATE_PERIODIC:
// Core internal bug
    if (!(dev.features & CLOCK_EVT_FEAT_PERIODIC))
    return -ENOSYS;
    if (dev.set_state_periodic)
    return dev.set_state_periodic(dev);
    return 0;
    case CLOCK_EVT_STATE_ONESHOT:
// Core internal bug
    if (!(dev.features & CLOCK_EVT_FEAT_ONESHOT))
    return -ENOSYS;
    if (dev.set_state_oneshot)
    return dev.set_state_oneshot(dev);
    return 0;
    case CLOCK_EVT_STATE_ONESHOT_STOPPED:
// Core internal bug
    if (WARN_ONCE(!clockevent_state_oneshot(dev),
    "Current state: %d\n",
    clockevent_get_state(dev)))
    return -EINVAL;
    if (dev.set_state_oneshot_stopped)
    return dev.set_state_oneshot_stopped(dev);
    else
    return -ENOSYS;
    default:
    return -ENOSYS;
    }
    }
//
// clockevents_switch_state - set the operating state of a clock event device
// @dev:	device to modify
// @state:	new state
//
// Must be called with interrupts disabled !
//
    void clockevents_switch_state(struct clock_event_device *dev,
    enum clock_event_state state)
    {
    if (clockevent_get_state(dev) != state) {
    if (__clockevents_switch_state(dev, state))
    return;
    clockevent_set_state(dev, state);
//
// A nsec2cyc multiplicator of 0 is invalid and we'd crash
// on it, so fix it up and emit a warning:
//
    if (clockevent_state_oneshot(dev)) {
    if (WARN_ON(!dev.mult))
    dev.mult = 1;
    }
    }
    }
//
// clockevents_shutdown - shutdown the device and clear next_event
// @dev:	device to shutdown
//
#[no_mangle]
pub unsafe extern "C" fn clockevents_shutdown(dev: *mut clock_event_device) {
    void clockevents_shutdown(struct clock_event_device *dev)
    {
    clockevents_switch_state(dev, CLOCK_EVT_STATE_SHUTDOWN);
    dev.next_event = KTIME_MAX;
    dev.next_event_forced = 0;
    }
//
// clockevents_tick_resume -	Resume the tick device before using it again
// @dev:			device to resume
//
#[no_mangle]
pub unsafe extern "C" fn clockevents_tick_resume(dev: *mut clock_event_device) -> c_int {
    int clockevents_tick_resume(struct clock_event_device *dev)
    {
    let mut ret: c_int = 0;
    if (dev.tick_resume)
    ret = dev.tick_resume(dev);
    return ret;
    }

// Limit min_delta to a jiffy

//
// clockevents_increase_min_delta - raise minimum delta of a clock event device
// @dev:       device to increase the minimum delta
//
// Returns 0 on success, -ETIME when the minimum delta reached the limit.
//
#[no_mangle]
unsafe extern "C" fn clockevents_increase_min_delta(dev: *mut clock_event_device) -> c_int {
    static int clockevents_increase_min_delta(struct clock_event_device *dev)
    {
// Nothing to do if we already reached the limit
    if (dev.min_delta_ns >= MIN_DELTA_LIMIT) {
    printk_deferred(KERN_WARNING
    "CE: Reprogramming failure. Giving up\n");
    dev.next_event = KTIME_MAX;
    return -ETIME;
    }
    if (dev.min_delta_ns < 5000)
    dev.min_delta_ns = 5000;
    else
    dev.min_delta_ns += dev.min_delta_ns >> 1;
    if (dev.min_delta_ns > MIN_DELTA_LIMIT)
    dev.min_delta_ns = MIN_DELTA_LIMIT;
    printk_deferred(KERN_WARNING
    "CE: %s increased min_delta_ns to %llu nsec\n",
    dev.name ? dev.name : "?",
    (unsigned long long) dev.min_delta_ns);
    return 0;
    }
//
// clockevents_program_min_delta - Set clock event device to the minimum delay.
// @dev:	device to program
//
// Returns 0 on success, -ETIME when the retry loop failed.
//
#[no_mangle]
unsafe extern "C" fn clockevents_program_min_delta(dev: *mut clock_event_device) -> c_int {
    static int clockevents_program_min_delta(struct clock_event_device *dev)
    {
    unsigned long long clc;
    int64_t delta;
    int i;
    for (i = 0;;) {
    delta = dev.min_delta_ns;
    dev.next_event = ktime_add_ns(ktime_get(), delta);
    if (clockevent_state_shutdown(dev))
    return 0;
    dev.retries++;
    clc = ((unsigned long long) delta * dev.mult) >> dev.shift;
    if (dev.set_next_event((unsigned long) clc, dev) == 0)
    return 0;
    if (++i > 2) {
//
// We tried 3 times to program the device with the
// given min_delta_ns. Try to increase the minimum
// delta, if that fails as well get out of here.
//
    if (clockevents_increase_min_delta(dev))
    return -ETIME;
    i = 0;
    }
    }
    }

//
// clockevents_program_min_delta - Set clock event device to the minimum delay.
// @dev:	device to program
//
// Returns 0 on success, -ETIME when the retry loop failed.
//
#[no_mangle]
unsafe extern "C" fn clockevents_program_min_delta(dev: *mut clock_event_device) -> c_int {
    static int clockevents_program_min_delta(struct clock_event_device *dev)
    {
    unsigned long long clc;
    let mut delta: i64 = 0;
    int i;
    for (i = 0; i < 10; i++) {
    delta += dev.min_delta_ns;
    dev.next_event = ktime_add_ns(ktime_get(), delta);
    if (clockevent_state_shutdown(dev))
    return 0;
    dev.retries++;
    clc = ((unsigned long long) delta * dev.mult) >> dev.shift;
    if (dev.set_next_event((unsigned long) clc, dev) == 0)
    return 0;
    }
    return -ETIME;
    }

    static __always_inline void
    arch_inlined_clockevent_set_next_coupled(u64 cycles, struct clock_event_device *dev) { }

#[no_mangle]
pub unsafe extern "C" fn clockevent_set_next_coupled(dev: *mut clock_event_device, expires: ktime_t) -> bool {
    static inline bool clockevent_set_next_coupled(struct clock_event_device *dev, ktime_t expires)
    {
    u64 cycles;
    if (unlikely(!(dev.features & CLOCK_EVT_FEAT_CLOCKSOURCE_COUPLED)))
    return false;
    if (unlikely(!ktime_expiry_to_cycles(dev.cs_id, expires, &cycles)))
    return false;
    if (IS_ENABLED(CONFIG_GENERIC_CLOCKEVENTS_COUPLED_INLINE))
    arch_inlined_clockevent_set_next_coupled(cycles, dev);
    else
    dev.set_next_coupled(cycles, dev);
    return true;
    }

#[no_mangle]
pub unsafe extern "C" fn clockevent_set_next_coupled(dev: *mut clock_event_device, expires: ktime_t) -> bool {
    static inline bool clockevent_set_next_coupled(struct clock_event_device *dev, ktime_t expires)
    {
    return false;
    }

//
// clockevents_program_event - Reprogram the clock event device.
// @dev:	device to program
// @expires:	absolute expiry time (monotonic clock)
// @force:	program minimum delay if expires can not be set
//
// Returns 0 on success, -ETIME when the event is in the past.
//
#[no_mangle]
pub unsafe extern "C" fn clockevents_program_event(dev: *mut clock_event_device, expires: ktime_t, force: bool) -> c_int {
    int clockevents_program_event(struct clock_event_device *dev, ktime_t expires, bool force)
    {
    int64_t delta;
    u64 cycles;
    if (WARN_ON_ONCE(expires < 0))
    return -ETIME;
    dev.next_event = expires;
    if (clockevent_state_shutdown(dev))
    return 0;
// We must be in ONESHOT state here
    WARN_ONCE(!clockevent_state_oneshot(dev), "Current state: %d\n",
    clockevent_get_state(dev));
// ktime_t based reprogramming for the broadcast hrtimer device
    if (unlikely(dev.features & CLOCK_EVT_FEAT_HRTIMER))
    return dev.set_next_ktime(expires, dev);
    if (likely(clockevent_set_next_coupled(dev, expires)))
    return 0;
    delta = ktime_to_ns(ktime_sub(expires, ktime_get()));
// Required for tick_periodic() during early boot
    if (delta <= 0 && !force)
    return -ETIME;
    if (delta > (int64_t)dev.min_delta_ns) {
    delta = min(delta, (int64_t) dev.max_delta_ns);
    cycles = ((u64)delta * dev.mult) >> dev.shift;
    if (!dev.set_next_event((unsigned long) cycles, dev)) {
    dev.next_event_forced = 0;
    return 0;
    }
    }
    if (dev.next_event_forced)
    return 0;
    if (dev.set_next_event(dev.min_delta_ticks, dev)) {
    if (!force || clockevents_program_min_delta(dev))
    return -ETIME;
    }
    dev.next_event_forced = 1;
    return 0;
    }
//
// Called after a clockevent has been added which might
// have replaced a current regular or broadcast device. A
// released normal device might be a suitable replacement
// for the current broadcast device. Similarly a released
// broadcast device might be a suitable replacement for a
// normal device.
//
#[no_mangle]
unsafe extern "C" fn clockevents_notify_released() {
    static void clockevents_notify_released(void)
    {
    struct clock_event_device *dev;
//
// Keep iterating as long as tick_check_new_device()
// replaces a device.
//
    while (!list_empty(&clockevents_released)) {
    dev = list_entry(clockevents_released.next,
    struct clock_event_device, list);
    list_move(&dev.list, &clockevent_devices);
    tick_check_new_device(dev);
    }
    }
//
// Try to install a replacement clock event device
//
#[no_mangle]
unsafe extern "C" fn clockevents_replace(ced: *mut clock_event_device) -> c_int {
    static int clockevents_replace(struct clock_event_device *ced)
    {
    struct clock_event_device *dev, *newdev = core::ptr::null_mut();
    list_for_each_entry(dev, &clockevent_devices, list) {
    if (dev == ced || !clockevent_state_detached(dev))
    continue;
    if (!tick_check_replacement(newdev, dev))
    continue;
    if (!try_module_get(dev.owner))
    continue;
    if (newdev)
    module_put(newdev.owner);
    newdev = dev;
    }
    if (newdev) {
    tick_install_replacement(newdev);
    list_del_init(&ced.list);
    }
    return newdev ? 0 : -EBUSY;
    }
//
// Called with clockevents_mutex and clockevents_lock held
//
#[no_mangle]
unsafe extern "C" fn __clockevents_try_unbind(ced: *mut clock_event_device, cpu: c_int) -> c_int {
    static int __clockevents_try_unbind(struct clock_event_device *ced, int cpu)
    {
// Fast track. Device is unused
    if (clockevent_state_detached(ced)) {
    list_del_init(&ced.list);
    return 0;
    }
    let mut ced: return = = per_cpu(tick_cpu_device, cpu).evtdev ? -EAGAIN : -EBUSY;
    }
//
// SMP function call to unbind a device
//
#[no_mangle]
unsafe extern "C" fn __clockevents_unbind(arg: *mut c_void) {
    static void __clockevents_unbind(void *arg)
    {
    struct ce_unbind *cu = arg;
    int res;
    raw_spin_lock(&clockevents_lock);
    res = __clockevents_try_unbind(cu.ce, smp_processor_id());
    if (res == -EAGAIN)
    res = clockevents_replace(cu.ce);
    cu.res = res;
    raw_spin_unlock(&clockevents_lock);
    }
//
// Issues smp function call to unbind a per cpu device. Called with
// clockevents_mutex held.
//
#[no_mangle]
unsafe extern "C" fn clockevents_unbind(ced: *mut clock_event_device, cpu: c_int) -> c_int {
    static int clockevents_unbind(struct clock_event_device *ced, int cpu)
    {
    let mut cu: ce_unbind = { .ce = ced, .res = -ENODEV };
    smp_call_function_single(cpu, __clockevents_unbind, &cu, 1);
    return cu.res;
    }
//
// Unbind a clockevents device.
//
#[no_mangle]
pub unsafe extern "C" fn clockevents_unbind_device(ced: *mut clock_event_device, cpu: c_int) -> c_int {
    int clockevents_unbind_device(struct clock_event_device *ced, int cpu)
    {
    int ret;
    mutex_lock(&clockevents_mutex);
    ret = clockevents_unbind(ced, cpu);
    mutex_unlock(&clockevents_mutex);
    return ret;
    }
    EXPORT_SYMBOL_GPL(clockevents_unbind_device);
//
// clockevents_register_device - register a clock event device
// @dev:	device to register
//
#[no_mangle]
pub unsafe extern "C" fn clockevents_register_device(dev: *mut clock_event_device) {
    void clockevents_register_device(struct clock_event_device *dev)
    {
    unsigned long flags;
// Initialize state to DETACHED
    clockevent_set_state(dev, CLOCK_EVT_STATE_DETACHED);
    if (!dev.cpumask) {
    WARN_ON(num_possible_cpus() > 1);
    dev.cpumask = cpumask_of(smp_processor_id());
    }
    if (dev.cpumask == cpu_all_mask) {
    WARN(1, "%s cpumask == cpu_all_mask, using cpu_possible_mask instead\n",
    dev.name);
    dev.cpumask = cpu_possible_mask;
    }
    raw_spin_lock_irqsave(&clockevents_lock, flags);
    list_add(&dev.list, &clockevent_devices);
    tick_check_new_device(dev);
    clockevents_notify_released();
    raw_spin_unlock_irqrestore(&clockevents_lock, flags);
    }
    EXPORT_SYMBOL_GPL(clockevents_register_device);
#[no_mangle]
unsafe extern "C" fn clockevents_config(dev: *mut clock_event_device, freq: u32) {
    static void clockevents_config(struct clock_event_device *dev, u32 freq)
    {
    u64 sec;
    if (!(dev.features & CLOCK_EVT_FEAT_ONESHOT))
    return;
//
// Calculate the maximum number of seconds we can sleep. Limit
// to 10 minutes for hardware which can program more than
// 32bit ticks so we still get reasonable conversion values.
//
    sec = dev.max_delta_ticks;
    do_div(sec, freq);
    if (!sec)
    sec = 1;
#[no_mangle]
pub unsafe extern "C" fn if(UINT_MAX: sec > 600 && dev->max_delta_ticks >) -> else {
    else if (sec > 600 && dev.max_delta_ticks > UINT_MAX)
    sec = 600;
    clockevents_calc_mult_shift(dev, freq, sec);
    dev.min_delta_ns = cev_delta2ns(dev.min_delta_ticks, dev, false);
    dev.max_delta_ns = cev_delta2ns(dev.max_delta_ticks, dev, true);
    }
//
// clockevents_config_and_register - Configure and register a clock event device
// @dev:	device to register
// @freq:	The clock frequency
// @min_delta:	The minimum clock ticks to program in oneshot mode
// @max_delta:	The maximum clock ticks to program in oneshot mode
//
// min/max_delta can be 0 for devices which do not support oneshot mode.
//
    void clockevents_config_and_register(struct clock_event_device *dev,
    u32 freq, unsigned long min_delta,
    unsigned long max_delta)
    {
    dev.min_delta_ticks = min_delta;
    dev.max_delta_ticks = max_delta;
    clockevents_config(dev, freq);
    clockevents_register_device(dev);
    }
    EXPORT_SYMBOL_GPL(clockevents_config_and_register);
#[no_mangle]
pub unsafe extern "C" fn __clockevents_update_freq(dev: *mut clock_event_device, freq: u32) -> c_int {
    int __clockevents_update_freq(struct clock_event_device *dev, u32 freq)
    {
    clockevents_config(dev, freq);
    if (clockevent_state_oneshot(dev))
    return clockevents_program_event(dev, dev.next_event, false);
    if (clockevent_state_periodic(dev))
    return __clockevents_switch_state(dev, CLOCK_EVT_STATE_PERIODIC);
    return 0;
    }
//
// clockevents_update_freq - Update frequency and reprogram a clock event device.
// @dev:	device to modify
// @freq:	new device frequency
//
// Reconfigure and reprogram a clock event device in oneshot
// mode. Must be called on the cpu for which the device delivers per
// cpu timer events. If called for the broadcast device the core takes
// care of serialization.
//
// Returns 0 on success, -ETIME when the event is in the past.
//
#[no_mangle]
pub unsafe extern "C" fn clockevents_update_freq(dev: *mut clock_event_device, freq: u32) -> c_int {
    int clockevents_update_freq(struct clock_event_device *dev, u32 freq)
    {
    unsigned long flags;
    int ret;
    local_irq_save(flags);
    ret = tick_broadcast_update_freq(dev, freq);
    if (ret == -ENODEV)
    ret = __clockevents_update_freq(dev, freq);
    local_irq_restore(flags);
    return ret;
    }
//
// Noop handler when we shut down an event device
//
#[no_mangle]
pub unsafe extern "C" fn clockevents_handle_noop(dev: *mut clock_event_device) {
    void clockevents_handle_noop(struct clock_event_device *dev)
    {
    }
//
// clockevents_exchange_device - release and request clock devices
// @old:	device to release (can be NULL)
// @new:	device to request (can be NULL)
//
// Called from various tick functions with clockevents_lock held and
// interrupts disabled.
//
    void clockevents_exchange_device(struct clock_event_device *old,
    struct clock_event_device *new)
    {
//
// Caller releases a clock event device. We queue it into the
// released list and do a notify add later.
//
    if (old) {
    module_put(old.owner);
    clockevents_switch_state(old, CLOCK_EVT_STATE_DETACHED);
    list_move(&old.list, &clockevents_released);
    }
    if (new) {
    BUG_ON(!clockevent_state_detached(new));
    clockevents_shutdown(new);
    }
    }
//
// clockevents_suspend - suspend clock devices
//
#[no_mangle]
pub unsafe extern "C" fn clockevents_suspend() {
    void clockevents_suspend(void)
    {
    struct clock_event_device *dev;
    list_for_each_entry_reverse(dev, &clockevent_devices, list)
    if (dev.suspend && !clockevent_state_detached(dev))
    dev.suspend(dev);
    }
//
// clockevents_resume - resume clock devices
//
#[no_mangle]
pub unsafe extern "C" fn clockevents_resume() {
    void clockevents_resume(void)
    {
    struct clock_event_device *dev;
    list_for_each_entry(dev, &clockevent_devices, list)
    if (dev.resume && !clockevent_state_detached(dev))
    dev.resume(dev);
    }

//
// tick_offline_cpu - Shutdown all clock events related
// to this CPU and take it out of the
// broadcast mechanism.
// @cpu:	The outgoing CPU
//
// Called by the dying CPU during teardown.
//
#[no_mangle]
pub unsafe extern "C" fn tick_offline_cpu(cpu: c_uint) {
    void tick_offline_cpu(unsigned int cpu)
    {
    struct clock_event_device *dev, *tmp;
    raw_spin_lock(&clockevents_lock);
    tick_broadcast_offline(cpu);
    tick_shutdown();
//
// Unregister the clock event devices which were
// released above.
//
    list_for_each_entry_safe(dev, tmp, &clockevents_released, list)
    list_del(&dev.list);
//
// Now check whether the CPU has left unused per cpu devices
//
    list_for_each_entry_safe(dev, tmp, &clockevent_devices, list) {
    if (cpumask_test_cpu(cpu, dev.cpumask) &&
    cpumask_weight(dev.cpumask) == 1 &&
    !tick_is_broadcast_device(dev)) {
    BUG_ON(!clockevent_state_detached(dev));
    list_del(&dev.list);
    }
    }
    raw_spin_unlock(&clockevents_lock);
    }

    static const struct bus_type clockevents_subsys = {
    .name		= "clockevents",
    .dev_name       = "clockevent",
    };
    static DEFINE_PER_CPU(struct device, tick_percpu_dev);
    static struct tick_device *tick_get_tick_dev(struct device *dev);
    static ssize_t current_device_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct tick_device *td;
    let mut count: isize = 0;
    raw_spin_lock_irq(&clockevents_lock);
    td = tick_get_tick_dev(dev);
    if (td && td.evtdev)
    count = sysfs_emit(buf, "%s\n", td.evtdev.name);
    raw_spin_unlock_irq(&clockevents_lock);
    return count;
    }
    static DEVICE_ATTR_RO(current_device);
// We don't support the abomination of removable broadcast devices
    static ssize_t unbind_device_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    char name[CS_NAME_LEN];
    let mut ret: isize = sysfs_get_uname(buf, name, count);
    struct clock_event_device *ce = core::ptr::null_mut(), *iter;
    if (ret < 0)
    return ret;
    ret = -ENODEV;
    mutex_lock(&clockevents_mutex);
    raw_spin_lock_irq(&clockevents_lock);
    list_for_each_entry(iter, &clockevent_devices, list) {
    if (!strcmp(iter.name, name)) {
    ret = __clockevents_try_unbind(iter, dev.id);
    ce = iter;
    break;
    }
    }
    raw_spin_unlock_irq(&clockevents_lock);
//
// We hold clockevents_mutex, so ce can't go away
//
    if (ret == -EAGAIN)
    ret = clockevents_unbind(ce, dev.id);
    mutex_unlock(&clockevents_mutex);
    return ret ? ret : count;
    }
    static DEVICE_ATTR_WO(unbind_device);

    static struct device tick_bc_dev = {
    .init_name	= "broadcast",
    .id		= 0,
    .bus		= &clockevents_subsys,
    };
    static struct tick_device *tick_get_tick_dev(struct device *dev)
    {
    return dev == &tick_bc_dev ? tick_get_broadcast_device() :
    &per_cpu(tick_cpu_device, dev.id);
    }
#[no_mangle]
unsafe extern "C" fn tick_broadcast_init_sysfs() -> __init int {
    static __init int tick_broadcast_init_sysfs(void)
    {
    let mut err: c_int = device_register(&tick_bc_dev);
    if (!err)
    err = device_create_file(&tick_bc_dev, &dev_attr_current_device);
    return err;
    }

    static struct tick_device *tick_get_tick_dev(struct device *dev)
    {
    return &per_cpu(tick_cpu_device, dev.id);
    }
    static inline int tick_broadcast_init_sysfs(void) { return 0; }

#[no_mangle]
unsafe extern "C" fn tick_init_sysfs() -> int __init {
    static int __init tick_init_sysfs(void)
    {
    int cpu;
    for_each_possible_cpu(cpu) {
    struct device *dev = &per_cpu(tick_percpu_dev, cpu);
    int err;
    dev.id = cpu;
    dev.bus = &clockevents_subsys;
    err = device_register(dev);
    if (!err)
    err = device_create_file(dev, &dev_attr_current_device);
    if (!err)
    err = device_create_file(dev, &dev_attr_unbind_device);
    if (err)
    return err;
    }
    return tick_broadcast_init_sysfs();
    }
#[no_mangle]
unsafe extern "C" fn clockevents_init_sysfs() -> int __init {
    static int __init clockevents_init_sysfs(void)
    {
    let mut err: c_int = subsys_system_register(&clockevents_subsys, core::ptr::null_mut());
    if (!err)
    err = tick_init_sysfs();
    return err;
    }
    device_initcall(clockevents_init_sysfs);
