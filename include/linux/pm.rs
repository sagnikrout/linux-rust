//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pm.h
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
// pm.h - Power management interface
//
// Copyright (C) 2000 Andrew Henroid
//

//
// Callbacks for platform drivers to implement.
//
extern "C" {
    pub fn void(_arg: *mut pm_power_off)(void) -> extern;
}

extern "C" {
    pub fn pm_vt_switch_required(dev: *mut device, required: bool) -> c_int;
}
extern "C" {
    pub fn pm_vt_switch_unregister(dev: *mut device);
}

extern "C" {
    pub fn cxl_mem_active() -> bool;
}

//
// Device power management
//

//
// struct dev_pm_ops - device PM callbacks.
//
// @prepare: The principal role of this callback is to prevent new children of
// the device from being registered after it has returned (the driver's
// subsystem and generally the rest of the kernel is supposed to prevent
// new calls to the probe method from being made too once @prepare() has
// succeeded).  If @prepare() detects a situation it cannot handle (e.g.
// registration of a child already in progress), it may return -EAGAIN, so
// that the PM core can execute it once again (e.g. after a new child has
// been registered) to recover from the race condition.
// This method is executed for all kinds of suspend transitions and is
// followed by one of the suspend callbacks: @suspend(), @freeze(), or
// @poweroff().  If the transition is a suspend to memory or standby (that
// is, not related to hibernation), the return value of @prepare() may be
// used to indicate to the PM core to leave the device in runtime suspend
// if applicable.  Namely, if @prepare() returns a positive number, the PM
// core will understand that as a declaration that the device appears to be
// runtime-suspended and it may be left in that state during the entire
// transition and during the subsequent resume if all of its descendants
// are left in runtime suspend too.  If that happens, @complete() will be
// executed directly after @prepare() and it must ensure the proper
// functioning of the device after the system resume.
// The PM core executes subsystem-level @prepare() for all devices before
// starting to invoke suspend callbacks for any of them, so generally
// devices may be assumed to be functional or to respond to runtime resume
// requests while @prepare() is being executed.  However, device drivers
// may NOT assume anything about the availability of user space at that
// time and it is NOT valid to request firmware from within @prepare()
// (it's too late to do that).  It also is NOT valid to allocate
// substantial amounts of memory from @prepare() in the GFP_KERNEL mode.
// [To work around these limitations, drivers may register suspend and
// hibernation notifiers to be executed before the freezing of tasks.]
//
// @complete: Undo the changes made by @prepare().  This method is executed for
// all kinds of resume transitions, following one of the resume callbacks:
// @resume(), @thaw(), @restore().  Also called if the state transition
// fails before the driver's suspend callback: @suspend(), @freeze() or
// @poweroff(), can be executed (e.g. if the suspend callback fails for one
// of the other devices that the PM core has unsuccessfully attempted to
// suspend earlier).
// The PM core executes subsystem-level @complete() after it has executed
// the appropriate resume callbacks for all devices.  If the corresponding
// @prepare() at the beginning of the suspend transition returned a
// positive number and the device was left in runtime suspend (without
// executing any suspend and resume callbacks for it), @complete() will be
// the only callback executed for the device during resume.  In that case,
// @complete() must be prepared to do whatever is necessary to ensure the
// proper functioning of the device after the system resume.  To this end,
// @complete() can check the power.direct_complete flag of the device to
// learn whether (unset) or not (set) the previous suspend and resume
// callbacks have been executed for it.
//
// @suspend: Executed before putting the system into a sleep state in which the
// contents of main memory are preserved.  The exact action to perform
// depends on the device's subsystem (PM domain, device type, class or bus
// type), but generally the device must be quiescent after subsystem-level
// @suspend() has returned, so that it doesn't do any I/O or DMA.
// Subsystem-level @suspend() is executed for all devices after invoking
// subsystem-level @prepare() for all of them.
//
// @suspend_late: Continue operations started by @suspend().  For a number of
// devices @suspend_late() may point to the same callback routine as the
// runtime suspend callback.
//
// @resume: Executed after waking the system up from a sleep state in which the
// contents of main memory were preserved.  The exact action to perform
// depends on the device's subsystem, but generally the driver is expected
// to start working again, responding to hardware events and software
// requests (the device itself may be left in a low-power state, waiting
// for a runtime resume to occur).  The state of the device at the time its
// driver's @resume() callback is run depends on the platform and subsystem
// the device belongs to.  On most platforms, there are no restrictions on
// availability of resources like clocks during @resume().
// Subsystem-level @resume() is executed for all devices after invoking
// subsystem-level @resume_noirq() for all of them.
//
// @resume_early: Prepare to execute @resume().  For a number of devices
// @resume_early() may point to the same callback routine as the runtime
// resume callback.
//
// @freeze: Hibernation-specific, executed before creating a hibernation image.
// Analogous to @suspend(), but it should not enable the device to signal
// wakeup events or change its power state.  The majority of subsystems
// (with the notable exception of the PCI bus type) expect the driver-level
// @freeze() to save the device settings in memory to be used by @restore()
// during the subsequent resume from hibernation.
// Subsystem-level @freeze() is executed for all devices after invoking
// subsystem-level @prepare() for all of them.
//
// @freeze_late: Continue operations started by @freeze().  Analogous to
// @suspend_late(), but it should not enable the device to signal wakeup
// events or change its power state.
//
// @thaw: Hibernation-specific, executed after creating a hibernation image OR
// if the creation of an image has failed.  Also executed after a failing
// attempt to restore the contents of main memory from such an image.
// Undo the changes made by the preceding @freeze(), so the device can be
// operated in the same way as immediately before the call to @freeze().
// Subsystem-level @thaw() is executed for all devices after invoking
// subsystem-level @thaw_noirq() for all of them.  It also may be executed
// directly after @freeze() in case of a transition error.
//
// @thaw_early: Prepare to execute @thaw().  Undo the changes made by the
// preceding @freeze_late().
//
// @poweroff: Hibernation-specific, executed after saving a hibernation image.
// Analogous to @suspend(), but it need not save the device's settings in
// memory.
// Subsystem-level @poweroff() is executed for all devices after invoking
// subsystem-level @prepare() for all of them.
//
// @poweroff_late: Continue operations started by @poweroff().  Analogous to
// @suspend_late(), but it need not save the device's settings in memory.
//
// @restore: Hibernation-specific, executed after restoring the contents of main
// memory from a hibernation image, analogous to @resume().
//
// @restore_early: Prepare to execute @restore(), analogous to @resume_early().
//
// @suspend_noirq: Complete the actions started by @suspend().  Carry out any
// additional operations required for suspending the device that might be
// racing with its driver's interrupt handler, which is guaranteed not to
// run while @suspend_noirq() is being executed.
// It generally is expected that the device will be in a low-power state
// (appropriate for the target system sleep state) after subsystem-level
// @suspend_noirq() has returned successfully.  If the device can generate
// system wakeup signals and is enabled to wake up the system, it should be
// configured to do so at that time.  However, depending on the platform
// and device's subsystem, @suspend() or @suspend_late() may be allowed to
// put the device into the low-power state and configure it to generate
// wakeup signals, in which case it generally is not necessary to define
// @suspend_noirq().
//
// @resume_noirq: Prepare for the execution of @resume() by carrying out any
// operations required for resuming the device that might be racing with
// its driver's interrupt handler, which is guaranteed not to run while
// @resume_noirq() is being executed.
//
// @freeze_noirq: Complete the actions started by @freeze().  Carry out any
// additional operations required for freezing the device that might be
// racing with its driver's interrupt handler, which is guaranteed not to
// run while @freeze_noirq() is being executed.
// The power state of the device should not be changed by either @freeze(),
// or @freeze_late(), or @freeze_noirq() and it should not be configured to
// signal system wakeup by any of these callbacks.
//
// @thaw_noirq: Prepare for the execution of @thaw() by carrying out any
// operations required for thawing the device that might be racing with its
// driver's interrupt handler, which is guaranteed not to run while
// @thaw_noirq() is being executed.
//
// @poweroff_noirq: Complete the actions started by @poweroff().  Analogous to
// @suspend_noirq(), but it need not save the device's settings in memory.
//
// @restore_noirq: Prepare for the execution of @restore() by carrying out any
// operations required for thawing the device that might be racing with its
// driver's interrupt handler, which is guaranteed not to run while
// @restore_noirq() is being executed.  Analogous to @resume_noirq().
//
// @runtime_suspend: Prepare the device for a condition in which it won't be
// able to communicate with the CPU(s) and RAM due to power management.
// This need not mean that the device should be put into a low-power state.
// For example, if the device is behind a link which is about to be turned
// off, the device may remain at full power.  If the device does go to low
// power and is capable of generating runtime wakeup events, remote wakeup
// (i.e., a hardware mechanism allowing the device to request a change of
// its power state via an interrupt) should be enabled for it.
//
// @runtime_resume: Put the device into the fully active state in response to a
// wakeup event generated by hardware or at the request of software.  If
// necessary, put the device into the full-power state and restore its
// registers, so that it is fully operational.
//
// @runtime_idle: Device appears to be inactive and it might be put into a
// low-power state if all of the necessary conditions are satisfied.
// Check these conditions, and return 0 if it's appropriate to let the PM
// core queue a suspend request for the device.
//
// Several device power state transitions are externally visible, affecting
// the state of pending I/O queues and (for drivers that touch hardware)
// interrupts, wakeups, DMA, and other hardware state.  There may also be
// internal transitions to various low-power modes which are transparent
// to the rest of the driver stack (such as a driver that's ON gating off
// clocks which are not in active use).
//
// The externally visible transitions are handled with the help of callbacks
// included in this structure in such a way that, typically, two levels of
// callbacks are involved.  First, the PM core executes callbacks provided by PM
// domains, device types, classes and bus types.  They are the subsystem-level
// callbacks expected to execute callbacks provided by device drivers, although
// they may choose not to do that.  If the driver callbacks are executed, they
// have to collaborate with the subsystem-level callbacks to achieve the goals
// appropriate for the given system transition, given transition phase and the
// subsystem the device belongs to.
//
// All of the above callbacks, except for @complete(), return error codes.
// However, the error codes returned by @resume(), @thaw(), @restore(),
// @resume_noirq(), @thaw_noirq(), and @restore_noirq(), do not cause the PM
// core to abort the resume transition during which they are returned.  The
// error codes returned in those cases are only printed to the system logs for
// debugging purposes.  Still, it is recommended that drivers only return error
// codes from their resume methods in case of an unrecoverable failure (i.e.
// when the device being handled refuses to resume and becomes unusable) to
// allow the PM core to be modified in the future, so that it can avoid
// attempting to handle devices that failed to resume and their children.
//
// It is allowed to unregister devices while the above callbacks are being
// executed.  However, a callback routine MUST NOT try to unregister the device
// it was called for, although it may unregister children of that device (for
// example, if it detects that a child was unplugged while the system was
// asleep).
//
// There also are callbacks related to runtime power management of devices.
// Again, as a rule these callbacks are executed by the PM core for subsystems
// (PM domains, device types, classes and bus types) and the subsystem-level
// callbacks are expected to invoke the driver callbacks.  Moreover, the exact
// actions to be performed by a device driver's callbacks generally depend on
// the platform and subsystem the device belongs to.
//
// Refer to Documentation/power/runtime_pm.rst for more information about the
// role of the @runtime_suspend(), @runtime_resume() and @runtime_idle()
// callbacks in device runtime power management.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_pm_ops {
    pub dev): *mut *mut int (prepare)(struct device,
    pub dev): *mut *mut void (complete)(struct device,
    pub dev): *mut *mut int (suspend)(struct device,
    pub dev): *mut *mut int (resume)(struct device,
    pub dev): *mut *mut int (freeze)(struct device,
    pub dev): *mut *mut int (thaw)(struct device,
    pub dev): *mut *mut int (poweroff)(struct device,
    pub dev): *mut *mut int (restore)(struct device,
    pub dev): *mut *mut int (suspend_late)(struct device,
    pub dev): *mut *mut int (resume_early)(struct device,
    pub dev): *mut *mut int (freeze_late)(struct device,
    pub dev): *mut *mut int (thaw_early)(struct device,
    pub dev): *mut *mut int (poweroff_late)(struct device,
    pub dev): *mut *mut int (restore_early)(struct device,
    pub dev): *mut *mut int (suspend_noirq)(struct device,
    pub dev): *mut *mut int (resume_noirq)(struct device,
    pub dev): *mut *mut int (freeze_noirq)(struct device,
    pub dev): *mut *mut int (thaw_noirq)(struct device,
    pub dev): *mut *mut int (poweroff_noirq)(struct device,
    pub dev): *mut *mut int (restore_noirq)(struct device,
    pub dev): *mut *mut int (runtime_suspend)(struct device,
    pub dev): *mut *mut int (runtime_resume)(struct device,
    pub dev): *mut *mut int (runtime_idle)(struct device,
}

//
// Use this if you want to use the same suspend and resume callbacks for suspend
// to RAM and hibernation.
//
// If the underlying dev_pm_ops struct symbol has to be exported, use
// EXPORT_SIMPLE_DEV_PM_OPS() or EXPORT_GPL_SIMPLE_DEV_PM_OPS() instead.
//

// Deprecated. Use DEFINE_SIMPLE_DEV_PM_OPS() instead.

//
// Use this for defining a set of PM operations to be used in all situations
// (system suspend, hibernation or runtime PM).
// NOTE: In general, system suspend callbacks, .suspend() and .resume(), should
// be different from the corresponding runtime PM callbacks, .runtime_suspend(),
// and .runtime_resume(), because .runtime_suspend() always works on an already
// quiescent device, while .suspend() should assume that the device may be doing
// something when it is called (it should ensure that the device will be
// quiescent after it has returned).  Therefore it's better to point the "late"
// suspend and "early" resume callback pointers, .suspend_late() and
// .resume_early(), to the same routines as .runtime_suspend() and
// .runtime_resume(), respectively (and analogously for hibernation).
//
// Deprecated. You most likely don't want this macro. Use
// DEFINE_RUNTIME_DEV_PM_OPS() instead.
//

//
// Use this if you want to have the suspend and resume callbacks be called
// with IRQs disabled.
//

//
// PM_EVENT_ messages
//
// The following PM_EVENT_ messages are defined for the internal use of the PM
// core, in order to provide a mechanism allowing the high level suspend and
// hibernation code to convey the necessary information to the device PM core
// code:
//
// ON		No transition.
//
// FREEZE	System is going to hibernate, call ->prepare() and ->freeze()
// for all devices.
//
// SUSPEND	System is going to suspend, call ->prepare() and ->suspend()
// for all devices.
//
// HIBERNATE	Hibernation image has been saved, call ->prepare() and
// ->poweroff() for all devices.
//
// QUIESCE	Contents of main memory are going to be restored from a (loaded)
// hibernation image, call ->prepare() and ->freeze() for all
// devices.
//
// RESUME	System is resuming, call ->resume() and ->complete() for all
// devices.
//
// THAW		Hibernation image has been created, call ->thaw() and
// ->complete() for all devices.
//
// RESTORE	Contents of main memory have been restored from a hibernation
// image, call ->restore() and ->complete() for all devices.
//
// RECOVER	Creation of a hibernation image or restoration of the main
// memory contents from a hibernation image has failed, call
// ->thaw() and ->complete() for all devices.
// POWEROFF	System will poweroff, call ->poweroff() for all devices.
//
// The following PM_EVENT_ messages are defined for internal use by
// kernel subsystems.  They are never issued by the PM core.
//
// USER_SUSPEND		Manual selective suspend was issued by userspace.
//
// USER_RESUME		Manual selective resume was issued by userspace.
//
// REMOTE_WAKEUP	Remote-wakeup request was received from the device.
//
// AUTO_SUSPEND		Automatic (device idle) runtime suspend was
// initiated by the subsystem.
//
// AUTO_RESUME		Automatic (device needed) runtime resume was
// requested by a driver.
//

pub const PM_EVENT_ON: c_uint = 0x0000;
pub const PM_EVENT_FREEZE: c_uint = 0x0001;
pub const PM_EVENT_SUSPEND: c_uint = 0x0002;
pub const PM_EVENT_HIBERNATE: c_uint = 0x0004;
pub const PM_EVENT_QUIESCE: c_uint = 0x0008;
pub const PM_EVENT_RESUME: c_uint = 0x0010;
pub const PM_EVENT_THAW: c_uint = 0x0020;
pub const PM_EVENT_RESTORE: c_uint = 0x0040;
pub const PM_EVENT_RECOVER: c_uint = 0x0080;
pub const PM_EVENT_USER: c_uint = 0x0100;
pub const PM_EVENT_REMOTE: c_uint = 0x0200;
pub const PM_EVENT_AUTO: c_uint = 0x0400;
pub const PM_EVENT_POWEROFF: c_uint = 0x0800;

//
// Device run-time power management status.
//
// These status labels are used internally by the PM core to indicate the
// current status of a device with respect to the PM core operations.  They do
// not reflect the actual power state of the device or its status as seen by the
// driver.
//
// RPM_ACTIVE		Device is fully operational.  Indicates that the device
// bus type's ->runtime_resume() callback has completed
// successfully.
//
// RPM_SUSPENDED	Device bus type's ->runtime_suspend() callback has
// completed successfully.  The device is regarded as
// suspended.
//
// RPM_RESUMING		Device bus type's ->runtime_resume() callback is being
// executed.
//
// RPM_SUSPENDING	Device bus type's ->runtime_suspend() callback is being
// executed.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpm_status {
    RPM_INVALID = -1,
    RPM_ACTIVE = 0,
    RPM_RESUMING,
    RPM_SUSPENDED,
    RPM_SUSPENDING,
    RPM_BLOCKED,
}

//
// Device run-time power management request types.
//
// RPM_REQ_NONE		Do nothing.
//
// RPM_REQ_IDLE		Run the device bus type's ->runtime_idle() callback
//
// RPM_REQ_SUSPEND	Run the device bus type's ->runtime_suspend() callback
//
// RPM_REQ_AUTOSUSPEND	Same as RPM_REQ_SUSPEND, but not until the device has
// been inactive for as long as power.autosuspend_delay
//
// RPM_REQ_RESUME	Run the device bus type's ->runtime_resume() callback
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpm_request {
    RPM_REQ_NONE = 0,
    RPM_REQ_IDLE,
    RPM_REQ_SUSPEND,
    RPM_REQ_AUTOSUSPEND,
    RPM_REQ_RESUME,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm_subsys_data {
    pub lock: spinlock_t,
    pub refcount: c_uint,

    pub clock_op_might_sleep: c_uint,
    pub clock_mutex: mutex,
    pub clock_list: list_head,

    pub domain_data: *mut pm_domain_data,

}

//
// Driver flags to control system suspend/resume behavior.
//
// These flags can be set by device drivers at the probe time.  They need not be
// cleared by the drivers as the driver core will take care of that.
//
// NO_DIRECT_COMPLETE: Do not apply direct-complete optimization to the device.
// SMART_PREPARE: Take the driver ->prepare callback return value into account.
// SMART_SUSPEND: Avoid resuming the device from runtime suspend.
// MAY_SKIP_RESUME: Allow driver "noirq" and "early" callbacks to be skipped.
//
// See Documentation/driver-api/pm/devices.rst for details.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_pm_info {
    pub power_state: pm_message_t,
    pub can_wakeup:1: bool,
    pub async_suspend:1: bool,
    pub /: *mut *mut bool in_dpm_list:1; / Owned by the PM core,
    pub /: *mut *mut bool is_prepared:1; / Owned by the PM core,
    pub /: *mut *mut bool is_suspended:1; / Ditto,
    pub is_noirq_suspended:1: bool,
    pub is_late_suspended:1: bool,
    pub no_pm:1: bool,
    pub /: *mut *mut bool early_init:1; / Owned by the PM core,
    pub /: *mut *mut bool direct_complete:1; / Owned by the PM core,
    pub driver_flags: u32,
    pub lock: spinlock_t,

    pub entry: list_head,
    pub completion: completion,
    pub wakeup: *mut wakeup_source,
    pub /: *mut *mut bool work_in_progress; / Owned by the PM core,
    pub wakeup_path:1: bool,
    pub syscore:1: bool,
    pub /: *mut *mut bool no_pm_callbacks:1; / Owned by the PM core,
    pub /: *mut *mut bool smart_suspend:1; / Owned by the PM core,
    pub /: *mut *mut bool must_resume:1; / Owned by the PM core,
    pub /: *mut *mut bool may_skip_resume:1; / Set by subsystems,
    pub out_band_wakeup:1: bool,
    pub strict_midlayer:1: bool,

    pub should_wakeup:1: bool,

    pub suspend_timer: hrtimer,
    pub timer_expires: u64,
    pub work: work_struct,
    pub wait_queue: wait_queue_head_t,
    pub wakeirq: *mut wake_irq,
    pub usage_count: core::sync::atomic::AtomicI32,
    pub child_count: core::sync::atomic::AtomicI32,
    pub disable_depth:3: c_uint,
    pub idle_notification:1: bool,
    pub request_pending:1: bool,
    pub deferred_resume:1: bool,
    pub needs_force_resume:1: bool,
    pub runtime_auto:1: bool,
    pub ignore_children:1: bool,
    pub no_callbacks:1: bool,
    pub irq_safe:1: bool,
    pub use_autosuspend:1: bool,
    pub timer_autosuspends:1: bool,
    pub memalloc_noio:1: bool,
    pub links_count: c_uint,
    pub request: rpm_request,
    pub runtime_status: rpm_status,
    pub last_status: rpm_status,
    pub runtime_error: c_int,
    pub autosuspend_delay: c_int,
    pub last_busy: u64,
    pub active_time: u64,
    pub suspended_time: u64,
    pub accounting_timestamp: u64,

    pub /: *mut *mut *mut pm_subsys_data subsys_data; / Owned by the subsystem.,
    pub s32): *mut *mut *mut void (set_latency_tolerance)(struct device ,,
    pub qos: *mut dev_pm_qos,
    pub /: *mut *mut bool detach_power_off:1; / Owned by the driver core,
}

extern "C" {
    pub fn dev_pm_get_subsys_data(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn dev_pm_put_subsys_data(dev: *mut device);
}
//
// struct dev_pm_domain - power management domain representation.
//
// @ops: Power management operations associated with this domain.
// @start: Called when a user needs to start the device via the domain.
// @detach: Called when removing a device from the domain.
// @activate: Called before executing probe routines for bus types and drivers.
// @sync: Called after successful driver probe.
// @dismiss: Called after unsuccessful driver probe and after driver removal.
// @set_performance_state: Called to request a new performance state.
//
// Power domains provide callbacks that are executed during system suspend,
// hibernation, system resume and during runtime PM transitions instead of
// subsystem-level and driver-level callbacks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_pm_domain {
    pub ops: dev_pm_ops,
    pub dev): *mut *mut int (start)(struct device,
    pub power_off): *mut *mut *mut void (detach)(struct device dev, bool,
    pub dev): *mut *mut int (activate)(struct device,
    pub dev): *mut *mut void (sync)(struct device,
    pub dev): *mut *mut void (dismiss)(struct device,
    pub state): *mut *mut *mut int (set_performance_state)(struct device dev, unsigned int,
}

//
// The PM_EVENT_ messages are also used by drivers implementing the legacy
// suspend framework, based on the ->suspend() and ->resume() callbacks common
// for suspend and hibernation transitions, according to the rules below.
//
// Necessary, because several drivers use PM_EVENT_PRETHAW

//
// One transition is triggered by resume(), after a suspend() call; the
// message is implicit:
//
// ON		Driver starts working again, responding to hardware events
// and software requests.  The hardware may have gone through
// a power-off reset, or it may have maintained state from the
// previous suspend() which the driver will rely on while
// resuming.  On most platforms, there are no restrictions on
// availability of resources like clocks during resume().
//
// Other transitions are triggered by messages sent using suspend().  All
// these transitions quiesce the driver, so that I/O queues are inactive.
// That commonly entails turning off IRQs and DMA; there may be rules
// about how to quiesce that are specific to the bus or the device's type.
// (For example, network drivers mark the link state.)  Other details may
// differ according to the message:
//
// SUSPEND	Quiesce, enter a low power device state appropriate for
// the upcoming system state (such as PCI_D3hot), and enable
// wakeup events as appropriate.
//
// HIBERNATE	Enter a low power device state appropriate for the hibernation
// state (eg. ACPI S4) and enable wakeup events as appropriate.
//
// FREEZE	Quiesce operations so that a consistent image can be saved;
// but do NOT otherwise enter a low power device state, and do
// NOT emit system wakeup events.
//
// PRETHAW	Quiesce as if for FREEZE; additionally, prepare for restoring
// the system from a snapshot taken after an earlier FREEZE.
// Some drivers will need to reset their hardware state instead
// of preserving it, to ensure that it's never mistaken for the
// state which that earlier snapshot had set up.
//
// A minimally power-aware driver treats all messages as SUSPEND, fully
// reinitializes its device during resume() -- whether or not it was reset
// during the suspend/resume cycle -- and can't issue wakeup events.
//
// More power-aware drivers may also use low power states at runtime as
// well as during system sleep states like PM_SUSPEND_STANDBY.  They may
// be able to use wakeup events to exit from runtime low-power states,
// or from system low-power states such as standby or suspend-to-RAM.
//

extern "C" {
    pub fn device_pm_lock();
}
extern "C" {
    pub fn dpm_resume_start(state: pm_message_t);
}
extern "C" {
    pub fn dpm_resume_end(state: pm_message_t);
}
extern "C" {
    pub fn dpm_resume_noirq(state: pm_message_t);
}
extern "C" {
    pub fn dpm_resume_early(state: pm_message_t);
}
extern "C" {
    pub fn dpm_resume(state: pm_message_t);
}
extern "C" {
    pub fn dpm_complete(state: pm_message_t);
}
extern "C" {
    pub fn device_pm_unlock();
}
extern "C" {
    pub fn dpm_suspend_end(state: pm_message_t) -> c_int;
}
extern "C" {
    pub fn dpm_suspend_start(state: pm_message_t) -> c_int;
}
extern "C" {
    pub fn dpm_suspend_noirq(state: pm_message_t) -> c_int;
}
extern "C" {
    pub fn dpm_suspend_late(state: pm_message_t) -> c_int;
}
extern "C" {
    pub fn dpm_suspend(state: pm_message_t) -> c_int;
}
extern "C" {
    pub fn dpm_prepare(state: pm_message_t) -> c_int;
}
extern "C" {
    pub fn __suspend_report_result(function: *const c_char, dev: *mut device, fn: *mut c_void, ret: c_int);
}

extern "C" {
    pub fn device_pm_wait_for_dev(sub: *mut device, dev: *mut device) -> c_int;
}
extern "C" {
    pub fn dpm_for_each_dev(data: *mut c_void, : *mut *mut void (fn)(struct device, ): *mut c_void);
}
extern "C" {
    pub fn pm_generic_prepare(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_generic_suspend_late(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_generic_suspend_noirq(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_generic_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_generic_resume_early(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_generic_resume_noirq(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_generic_resume(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_generic_freeze_noirq(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_generic_freeze(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_generic_thaw_noirq(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_generic_thaw(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_generic_restore_noirq(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_generic_restore_early(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_generic_restore(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_generic_poweroff_noirq(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_generic_poweroff_late(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_generic_poweroff(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_generic_complete(dev: *mut device);
}
extern "C" {
    pub fn dev_pm_skip_resume(dev: *mut device) -> bool;
}
extern "C" {
    pub fn dev_pm_skip_suspend(dev: *mut device) -> bool;
}

// How to reorder dpm_list after device_move()
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpm_order {
    DPM_ORDER_NONE,
    DPM_ORDER_DEV_AFTER_PARENT,
    DPM_ORDER_PARENT_BEFORE_DEV,
    DPM_ORDER_DEV_LAST,
}
