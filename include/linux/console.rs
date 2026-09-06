//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/console.h
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


//
// linux/include/linux/console.h
//
// Copyright (C) 1993        Hamish Macdonald
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//
// Changed:
// 10-Mar-94: Arno Griffioen: Conversion for vt100 emulator port from PC LINUX
//
pub const _LINUX_CONSOLE_H_: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum con_scroll {
    SM_UP,
    SM_DOWN,
}

//
// struct consw - callbacks for consoles
//
// @owner:      the module to get references of when this console is used
// @con_startup: set up the console and return its name (like VGA, EGA, ...)
// @con_init:   initialize the console on @vc. @init is true for the very first
// call on this @vc.
// @con_deinit: deinitialize the console from @vc.
// @con_clear:  erase @count characters at [@x, @y] on @vc. @count >= 1.
// @con_putc:   emit one character with attributes @ca to [@x, @y] on @vc.
// (optional -- @con_putcs would be called instead)
// @con_putcs:  emit @count characters with attributes @s to [@x, @y] on @vc.
// @con_cursor: enable/disable cursor depending on @enable
// @con_scroll: move lines from @top to @bottom in direction @dir by @lines.
// Return true if no generic handling should be done.
// Invoked by csi_M and printing to the console.
// @con_switch: notifier about the console switch; it is supposed to return
// true if a redraw is needed.
// @con_blank:  blank/unblank the console. The target mode is passed in @blank.
// @mode_switch is set if changing from/to text/graphics. The hook
// is supposed to return true if a redraw is needed.
// @con_font_set: set console @vc font to @font with height @vpitch. @flags can
// be %KD_FONT_FLAG_DONT_RECALC. (optional)
// @con_font_get: fetch the current font on @vc of height @vpitch into @font.
// (optional)
// @con_font_default: set default font on @vc. @name can be %NULL or font name
// to search for. @font can be filled back. (optional)
// @con_resize:	resize the @vc console to @width x @height. @from_user is true
// when this change comes from the user space.
// @con_set_palette: sets the palette of the console @vc to @table (optional)
// @con_scrolldelta: the contents of the console should be scrolled by @lines.
// Invoked by user. (optional)
// @con_set_origin: set origin (see &vc_data::vc_origin) of the @vc. If not
// provided or returns false, the origin is set to
// @vc->vc_screenbuf. (optional)
// @con_save_screen: save screen content into @vc->vc_screenbuf. Called e.g.
// upon entering graphics. (optional)
// @con_build_attr: build attributes based on @color, @intensity and other
// parameters. The result is used for both normal and erase
// characters. (optional)
// @con_invert_region: invert a region of length @count on @vc starting at @p.
// (optional)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct consw {
    pub owner: *mut module,
    pub (*con_startup)(void): *const c_char,
    pub init): *mut *mut *mut void (con_init)(struct vc_data vc, bool,
    pub vc): *mut *mut void (con_deinit)(struct vc_data,
    pub count): unsigned int x, unsigned int,
    pub x): c_uint,
    pub xpos): c_uint,
    pub enable): *mut *mut *mut void (con_cursor)(struct vc_data vc, bool,
    pub lines): c_uint,
    pub vc): *mut *mut bool (con_switch)(struct vc_data,
    pub mode_switch): bool,
    pub flags): unsigned int vpitch, unsigned int,
    pub vpitch): c_uint,
    pub name): *const *const console_font font, char,
    pub from_user): unsigned int height, bool,
    pub table): *const c_uchar,
    pub lines): *mut *mut *mut void (con_scrolldelta)(struct vc_data vc, int,
    pub vc): *mut *mut bool (con_set_origin)(struct vc_data,
    pub vc): *mut *mut void (con_save_screen)(struct vc_data,
    pub italic): bool blink, bool underline, bool reverse, bool,
    pub count): *mut *mut *mut *mut void (con_invert_region)(struct vc_data vc, u16 p, int,
}

extern "C" {
    pub fn vgacon_register_screen(si: *mut screen_info);
}

extern "C" {
    pub fn con_is_bound(csw: *const consw) -> c_int;
}
extern "C" {
    pub fn do_unregister_con_driver(csw: *const consw) -> c_int;
}
extern "C" {
    pub fn do_take_over_console(sw: *const consw, first: c_int, last: c_int, deflt: c_int) -> c_int;
}
extern "C" {
    pub fn give_up_console(sw: *const consw);
}

extern "C" {
    pub fn con_debug_enter(vc: *mut vc_data);
}
extern "C" {
    pub fn con_debug_leave();
}

//
// The interface for a console, or any other device that wants to capture
// console messages (printer driver?)
//
// enum cons_flags - General console flags
// @CON_PRINTBUFFER:	Used by newly registered consoles to avoid duplicate
// output of messages that were already shown by boot
// consoles or read by userspace via syslog() syscall.
// @CON_CONSDEV:	Indicates that the console driver is backing
// /dev/console.
// @CON_ENABLED:	Indicates if a console is allowed to print records. If
// false, the console also will not advance to later
// records.
// @CON_BOOT:		Marks the console driver as early console driver which
// is used during boot before the real driver becomes
// available. It will be automatically unregistered
// when the real console driver is registered unless
// "keep_bootcon" parameter is used.
// @CON_ANYTIME:	A misnomed historical flag which tells the core code
// that the legacy @console::write callback can be invoked
// on a CPU which is marked OFFLINE. That is misleading as
// it suggests that there is no contextual limit for
// invoking the callback. The original motivation was
// readiness of the per-CPU areas.
// @CON_BRL:		Indicates a braille device which is exempt from
// receiving the printk spam for obvious reasons.
// @CON_EXTENDED:	The console supports the extended output format of
// /dev/kmesg which requires a larger output buffer.
// @CON_SUSPENDED:	Indicates if a console is suspended. If true, the
// printing callbacks must not be called.
// @CON_NBCON:		Console can operate outside of the legacy style console_lock
// constraints.
// @CON_NBCON_ATOMIC_UNSAFE: The write_atomic() callback is not safe and is
// therefore only used by nbcon_atomic_flush_unsafe().
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cons_flags {
    CON_PRINTBUFFER		= BIT(0),
    CON_CONSDEV		= BIT(1),
    CON_ENABLED		= BIT(2),
    CON_BOOT		= BIT(3),
    CON_ANYTIME		= BIT(4),
    CON_BRL			= BIT(5),
    CON_EXTENDED		= BIT(6),
    CON_SUSPENDED		= BIT(7),
    CON_NBCON		= BIT(8),
    CON_NBCON_ATOMIC_UNSAFE	= BIT(9),
}

//
// struct nbcon_state - console state for nbcon consoles
// @atom:	Compound of the state fields for atomic operations
//
// @req_prio:		The priority of a handover request
// @prio:		The priority of the current owner
// @unsafe:		Console is busy in a non takeover region
// @unsafe_takeover:	A hostile takeover in an unsafe state happened in the
// past. The console cannot be safe until re-initialized.
// @cpu:		The CPU on which the owner runs
//
// To be used for reading and preparing of the value stored in the nbcon
// state variable @console::nbcon_state.
//
// The @prio and @req_prio fields are particularly important to allow
// spin-waiting to timeout and give up without the risk of a waiter being
// assigned the lock after giving up.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nbcon_state {
    pub atom: c_uint,
    pub 2: unsigned int prio :,
    pub 2: unsigned int req_prio :,
    pub 1: unsigned int unsafe :,
    pub 1: unsigned int unsafe_takeover :,
    pub 24: unsigned int cpu :,
}

//
// The nbcon_state struct is used to easily create and interpret values that
// are stored in the @console::nbcon_state variable. Ensure this struct stays
// within the size boundaries of the atomic variable's underlying type in
// order to avoid any accidental truncation.
//
// enum nbcon_prio - console owner priority for nbcon consoles
// @NBCON_PRIO_NONE:		Unused
// @NBCON_PRIO_NORMAL:		Normal (non-emergency) usage
// @NBCON_PRIO_EMERGENCY:	Emergency output (WARN/OOPS...)
// @NBCON_PRIO_PANIC:		Panic output
// @NBCON_PRIO_MAX:		The number of priority levels
//
// A higher priority context can takeover the console when it is
// in the safe state. The final attempt to flush consoles in panic()
// can be allowed to do so even in an unsafe state (Hope and pray).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nbcon_prio {
    NBCON_PRIO_NONE = 0,
    NBCON_PRIO_NORMAL,
    NBCON_PRIO_EMERGENCY,
    NBCON_PRIO_PANIC,
    NBCON_PRIO_MAX,
}

//
// struct nbcon_context - Context for console acquire/release
// @console:			The associated console
// @spinwait_max_us:		Limit for spin-wait acquire
// @prio:			Priority of the context
// @allow_unsafe_takeover:	Allow performing takeover even if unsafe. Can
// be used only with NBCON_PRIO_PANIC @prio. It
// might cause a system freeze when the console
// is used later.
// @backlog:			Ringbuffer has pending records
// @pbufs:			Pointer to the text buffer for this context
// @seq:			The sequence number to print for this context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nbcon_context {
// members set by caller
    pub console: *mut console,
    pub spinwait_max_us: c_uint,
    pub prio: nbcon_prio,
    pub 1: unsigned int allow_unsafe_takeover :,
// members set by emit
    pub 1: unsigned int backlog :,
// members set by acquire
    pub pbufs: *mut printk_buffers,
    pub seq: u64,
}

//
// struct nbcon_write_context - Context handed to the nbcon write callbacks
// @ctxt:		The core console context
// @outbuf:		Pointer to the text buffer for output
// @len:		Length to write
// @unsafe_takeover:	If a hostile takeover in an unsafe state has occurred
// @cpu:		CPU on which the message was generated
// @pid:		PID of the task that generated the message
// @comm:		Name of the task that generated the message
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nbcon_write_context {
    pub ctxt: nbcon_context __private,
    pub outbuf: *mut c_char,
    pub len: c_uint,
    pub unsafe_takeover: bool,

    pub cpu: c_int,
    pub pid: pid_t,
    pub comm: [c_char; TASK_COMM_LEN],
}

//
// struct console - The console descriptor structure
// @name:		The name of the console driver
// @write:		Legacy write callback to output messages (Optional)
// @read:		Read callback for console input (Optional)
// @device:		The underlying TTY device driver (Optional)
// @unblank:		Callback to unblank the console (Optional)
// @setup:		Callback for initializing the console (Optional)
// @exit:		Callback for teardown of the console (Optional)
// @match:		Callback for matching a console (Optional)
// @flags:		Console flags. See enum cons_flags
// @index:		Console index, e.g. port number
// @cflag:		TTY control mode flags
// @ispeed:		TTY input speed
// @ospeed:		TTY output speed
// @seq:		Sequence number of the next ringbuffer record to print
// @dropped:		Number of unreported dropped ringbuffer records
// @data:		Driver private data
// @node:		hlist node for the console list
//
// @nbcon_state:	State for nbcon consoles
// @nbcon_seq:		Sequence number of the next record for nbcon to print
// @nbcon_device_ctxt:	Context available for non-printing operations
// @nbcon_prev_seq:	Seq num the previous nbcon owner was assigned to print
// @pbufs:		Pointer to nbcon private buffer
// @kthread:		Printer kthread for this console
// @rcuwait:		RCU-safe wait object for @kthread waking
// @irq_work:		Defer @kthread waking to IRQ work context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct console {
    pub name: [c_char; 16],
    pub count): *const *const *const *const void (write)(struct console co, char s, unsigned int,
    pub count): *mut *mut *mut *mut int (read)(struct console co, char s, unsigned int,
    pub index): *mut *mut *mut *mut tty_driver (device)(console co, int,
    pub (*unblank)(void): *mut c_void,
    pub options): *mut *mut *mut int (setup)(struct console co, char,
    pub co): *mut *mut int (exit)(struct console,
    pub options): *mut *mut *mut *mut int (match)(struct console co, char name, int idx, char,
    pub flags: c_short,
    pub index: c_short,
    pub cflag: c_int,
    pub ispeed: c_uint,
    pub ospeed: c_uint,
    pub seq: u64,
    pub dropped: c_ulong,
    pub data: *mut c_void,
    pub node: hlist_node,
// nbcon console specific members
//
// @write_atomic:
//
// NBCON callback to write out text in any context. (Optional)
//
// This callback is called with the console already acquired. However,
// a higher priority context is allowed to take it over by default.
//
// The callback must call nbcon_enter_unsafe() and nbcon_exit_unsafe()
// around any code where the takeover is not safe, for example, when
// manipulating the serial port registers.
//
// nbcon_enter_unsafe() will fail if the context has lost the console
// ownership in the meantime. In this case, the callback is no longer
// allowed to go forward. It must back out immediately and carefully.
// The buffer content is also no longer trusted since it no longer
// belongs to the context.
//
// The callback should allow the takeover whenever it is safe. It
// increases the chance to see messages when the system is in trouble.
// If the driver must reacquire ownership in order to finalize or
// revert hardware changes, nbcon_reacquire_nobuf() can be used.
// However, on reacquire the buffer content is no longer available. A
// reacquire cannot be used to resume printing.
//
// The callback can be called from any context (including NMI).
// Therefore it must avoid usage of any locking and instead rely
// on the console ownership for synchronization.
//
    pub wctxt): *mut *mut *mut void (write_atomic)(struct console con, struct nbcon_write_context,
//
// @write_thread:
//
// NBCON callback to write out text in task context.
//
// This callback must be called only in task context with both
// device_lock() and the nbcon console acquired with
// NBCON_PRIO_NORMAL.
//
// The same rules for console ownership verification and unsafe
// sections handling applies as with write_atomic().
//
// The console ownership handling is necessary for synchronization
// against write_atomic() which is synchronized only via the context.
//
// The device_lock() provides the primary serialization for operations
// on the device. It might be as relaxed (mutex)[*] or as tight
// (disabled preemption and interrupts) as needed. It allows
// the kthread to operate in the least restrictive mode[**].
//
// [*] Standalone nbcon_context_try_acquire() is not safe with
// the preemption enabled, see nbcon_owner_matches(). But it
// can be safe when always called in the preemptive context
// under the device_lock().
//
// [**] The device_lock() makes sure that nbcon_context_try_acquire()
// would never need to spin which is important especially with
// PREEMPT_RT.
//
    pub wctxt): *mut *mut *mut void (write_thread)(struct console con, struct nbcon_write_context,
//
// @device_lock:
//
// NBCON callback to begin synchronization with driver code.
//
// Console drivers typically must deal with access to the hardware
// via user input/output (such as an interactive login shell) and
// output of kernel messages via printk() calls. This callback is
// called by the printk-subsystem whenever it needs to synchronize
// with hardware access by the driver. It should be implemented to
// use whatever synchronization mechanism the driver is using for
// itself (for example, the port lock for uart serial consoles).
//
// The callback is always called from task context. It may use any
// synchronization method required by the driver.
//
// IMPORTANT: The callback MUST disable migration. The console driver
// may be using a synchronization mechanism that already takes
// care of this (such as spinlocks). Otherwise this function must
// explicitly call migrate_disable().
//
// The flags argument is provided as a convenience to the driver. It
// will be passed again to device_unlock(). It can be ignored if the
// driver does not need it.
//
    pub flags): *mut *mut *mut void (device_lock)(struct console con, unsigned long,
//
// @device_unlock:
//
// NBCON callback to finish synchronization with driver code.
//
// It is the counterpart to device_lock().
//
// This callback is always called from task context. It must
// appropriately re-enable migration (depending on how device_lock()
// disabled migration).
//
// The flags argument is the value of the same variable that was
// passed to device_lock().
//
    pub flags): *mut *mut *mut void (device_unlock)(struct console con, unsigned long,
    pub nbcon_state: atomic_t __private,
    pub nbcon_seq: atomic_long_t __private,
    pub nbcon_device_ctxt: nbcon_context __private,
    pub nbcon_prev_seq: atomic_long_t __private,
    pub pbufs: *mut printk_buffers,
    pub kthread: *mut task_struct,
    pub rcuwait: rcuwait,
    pub irq_work: irq_work,
}

extern "C" {
    pub fn lockdep_assert_console_list_lock_held();
}

extern "C" {
    pub fn console_srcu_read_lock_is_held() -> bool;
}

extern "C" {
    pub fn console_srcu_read_lock() -> c_int;
}
extern "C" {
    pub fn console_srcu_read_unlock(cookie: c_int);
}
extern "C" {
    pub fn console_list_lock();
}
extern "C" {
    pub fn console_list_unlock();
}
//
// console_srcu_read_flags - Locklessly read flags of a possibly registered
// console
// @con:	struct console pointer of console to read flags from
//
// Locklessly reading @con->flags provides a consistent read value because
// there is at most one CPU modifying @con->flags and that CPU is using only
// read-modify-write operations to do so.
//
// Requires console_srcu_read_lock to be held, which implies that @con might
// be a registered console. The purpose of holding console_srcu_read_lock is
// to guarantee that the console state is valid (CON_SUSPENDED/CON_ENABLED)
// and that no exit/cleanup routines will run if the console is currently
// undergoing unregistration.
//
// If the caller is holding the console_list_lock or it is _certain_ that
// @con is not and will not become registered, the caller may read
// @con->flags directly instead.
//
// Context: Any context.
// Return: The current value of the @con->flags field.
//
// The READ_ONCE() matches the WRITE_ONCE() when @flags are modified
// for registered consoles with console_srcu_write_flags().
//
extern "C" {
    pub fn data_race(_arg: READ_ONCE(con->flags)) -> return;
}
//
// console_srcu_write_flags - Write flags for a registered console
// @con:	struct console pointer of console to write flags to
// @flags:	new flags value to write
//
// Only use this function to write flags for registered consoles. It
// requires holding the console_list_lock.
//
// Context: Any context.
//
// This matches the READ_ONCE() in console_srcu_read_flags().
// Variant of console_is_registered() when the console_list_lock is held.
//
// console_is_registered - Check if the console is registered
// @con:	struct console pointer of console to check
//
// Context: Process context. May sleep while acquiring console list lock.
// Return: true if the console is in the console list, otherwise false.
//
// If false is returned for a console that was previously registered, it
// can be assumed that the console's unregistration is fully completed,
// including the exit() callback after console list removal.
//
// for_each_console_srcu() - Iterator over registered consoles
// @con:	struct console pointer used as loop cursor
//
// Although SRCU guarantees the console list will be consistent, the
// struct console fields may be updated by other CPUs while iterating.
//
// Requires console_srcu_read_lock to be held. Can be invoked from
// any context.
//

//
// for_each_console() - Iterator over registered consoles
// @con:	struct console pointer used as loop cursor
//
// The console list and the &console.flags are immutable while iterating.
//
// Requires console_list_lock to be held.
//

extern "C" {
    pub fn nbcon_cpu_emergency_enter();
}
extern "C" {
    pub fn nbcon_cpu_emergency_exit();
}
extern "C" {
    pub fn nbcon_can_proceed(wctxt: *mut nbcon_write_context) -> bool;
}
extern "C" {
    pub fn nbcon_enter_unsafe(wctxt: *mut nbcon_write_context) -> bool;
}
extern "C" {
    pub fn nbcon_exit_unsafe(wctxt: *mut nbcon_write_context) -> bool;
}
extern "C" {
    pub fn nbcon_reacquire_nobuf(wctxt: *mut nbcon_write_context);
}
extern "C" {
    pub fn nbcon_allow_unsafe_takeover() -> bool;
}
extern "C" {
    pub fn nbcon_kdb_release(wctxt: *mut nbcon_write_context);
}
//
// Check if the given console is currently capable and allowed to print
// records. Note that this function does not consider the current context,
// which can also play a role in deciding if @con can be used to print
// records.
//
// The write_atomic() callback is optional.
//
// An unsafe write_atomic() callback is only usable
// when unsafe takeovers are allowed.
//
// For the !use_atomic case, @printk_kthreads_running is not
// checked because the write_thread() callback is also used
// via the legacy loop when the printer threads are not
// available.
//
// Console drivers may assume that per-cpu resources have been
// allocated. So unless they're explicitly marked as being able to
// cope (CON_ANYTIME) don't call them until this CPU is officially up.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum con_flush_mode {
    CONSOLE_FLUSH_PENDING,
    CONSOLE_REPLAY_ALL,
}

extern "C" {
    pub fn add_preferred_console(name: *const c_char, idx: c_short, options: *mut c_char) -> c_int;
}
extern "C" {
    pub fn console_force_preferred_locked(con: *mut console);
}
extern "C" {
    pub fn register_console(: *mut console);
}
extern "C" {
    pub fn unregister_console(: *mut console) -> c_int;
}
extern "C" {
    pub fn console_lock();
}
extern "C" {
    pub fn console_trylock() -> c_int;
}
extern "C" {
    pub fn console_unlock();
}
extern "C" {
    pub fn console_unblank();
}
extern "C" {
    pub fn console_flush_on_panic(mode: con_flush_mode);
}
extern "C" {
    pub fn console_suspend(: *mut console);
}
extern "C" {
    pub fn console_resume(: *mut console);
}
extern "C" {
    pub fn is_console_locked() -> c_int;
}
extern "C" {
    pub fn braille_unregister_console(: *mut console) -> c_int;
}

extern "C" {
    pub fn console_sysfs_notify();
}

// Suspend and resume console messages over PM events
extern "C" {
    pub fn console_suspend_all();
}
extern "C" {
    pub fn console_resume_all();
}
extern "C" {
    pub fn vcs_make_sysfs(index: c_int);
}
extern "C" {
    pub fn vcs_remove_sysfs(index: c_int);
}
// Some debug stub to catch some of the obvious races in the VT code

//
// Increment ignore_console_lock_warning if you need to quiet
// WARN_CONSOLE_UNLOCKED() for debugging purposes.
//
extern "C" {
    pub fn console_init();
}
// For deferred console takeover
extern "C" {
    pub fn dummycon_register_output_notifier(nb: *mut notifier_block);
}
extern "C" {
    pub fn dummycon_unregister_output_notifier(nb: *mut notifier_block);
}
