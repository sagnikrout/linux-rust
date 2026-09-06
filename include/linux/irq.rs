//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/irq.h
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
// Please do not include this file in generic code.  There is currently
// no requirement for any architecture to implement anything held
// within this file.
//
// Thanks. --rmk
//

//
// IRQ line status.
//
// Bits 0-7 are the same as the IRQF_* bits in linux/interrupt.h
//
// Note that the first 6 definitions are shadowed by C preprocessor definitions
// in include/dt-bindings/interrupt-controller/irq.h.  This is not an issue, as
// the actual values must be the same, due to being part of the stable DT ABI.
//
// IRQ_TYPE_NONE		- default, unspecified type
// IRQ_TYPE_EDGE_RISING		- rising edge triggered
// IRQ_TYPE_EDGE_FALLING	- falling edge triggered
// IRQ_TYPE_EDGE_BOTH		- rising and falling edge triggered
// IRQ_TYPE_LEVEL_HIGH		- high level triggered
// IRQ_TYPE_LEVEL_LOW		- low level triggered
// IRQ_TYPE_LEVEL_MASK		- Mask to filter out the level bits
// IRQ_TYPE_SENSE_MASK		- Mask for all the above bits
// IRQ_TYPE_DEFAULT		- For use by some PICs to ask irq_set_type
// to setup the HW to a sane default (used
// by irqdomain map() callbacks to synchronize
// the HW state and SW flags for a newly
// allocated descriptor).
//
// IRQ_TYPE_PROBE		- Special flag for probing in progress
//
// Bits which can be modified via irq_set/clear/modify_status_flags()
// IRQ_LEVEL			- Interrupt is level type. Will be also
// updated in the code when the above trigger
// bits are modified via irq_set_irq_type()
// IRQ_PER_CPU			- Mark an interrupt PER_CPU. Will protect
// it from affinity setting
// IRQ_NOPROBE			- Interrupt cannot be probed by autoprobing
// IRQ_NOREQUEST		- Interrupt cannot be requested via
// request_irq()
// IRQ_NOTHREAD			- Interrupt cannot be threaded
// IRQ_NOAUTOEN			- Interrupt is not automatically enabled in
// request/setup_irq()
// IRQ_NO_BALANCING		- Interrupt cannot be balanced (affinity set)
// IRQ_NESTED_THREAD		- Interrupt nests into another thread
// IRQ_PER_CPU_DEVID		- Dev_id is a per-cpu variable
// IRQ_IS_POLLED		- Always polled by another interrupt. Exclude
// it from the spurious interrupt detection
// mechanism and from core side polling.
// IRQ_DISABLE_UNLAZY		- Disable lazy irq disable
// IRQ_HIDDEN			- Don't show up in /proc/interrupts
// IRQ_NO_DEBUG			- Exclude from note_interrupt() debugging
//

//
// Return value for chip->irq_set_affinity()
//
// IRQ_SET_MASK_OK	- OK, core updates irq_common_data.affinity
// IRQ_SET_MASK_NOCOPY	- OK, chip did update irq_common_data.affinity
// IRQ_SET_MASK_OK_DONE	- Same as IRQ_SET_MASK_OK for core. Special code to
// support stacked irqchips, which indicates skipping
// all descendant irqchips.
//
// struct irq_common_data - per irq data shared by all irqchips
// @state_use_accessors: status information for irq chip functions.
// Use accessor functions to deal with it
// @node:		node index useful for balancing
// @handler_data:	per-IRQ data for the irq_chip methods
// @affinity:		IRQ affinity on SMP. If this is an IPI
// related irq, then this is the mask of the
// CPUs to which an IPI can be sent.
// @effective_affinity:	The effective IRQ affinity on SMP as some irq
// chips do not allow multi CPU destinations.
// A subset of @affinity.
// @msi_desc:		MSI descriptor
// @ipi_offset:		Offset of first IPI target cpu in @affinity. Optional.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_common_data {
    pub state_use_accessors: unsigned int __private,

    pub node: c_uint,

    pub handler_data: *mut c_void,
    pub msi_desc: *mut msi_desc,

    pub affinity: cpumask_var_t,

    pub effective_affinity: cpumask_var_t,

    pub ipi_offset: c_uint,

}

//
// struct irq_data - per irq chip data passed down to chip functions
// @mask:		precomputed bitmask for accessing the chip registers
// @irq:		interrupt number
// @hwirq:		hardware interrupt number, local to the interrupt domain
// @common:		point to data shared by all irqchips
// @chip:		low level interrupt hardware access
// @domain:		Interrupt translation domain; responsible for mapping
// between hwirq number and linux irq number.
// @parent_data:	pointer to parent struct irq_data to support hierarchy
// irq_domain
// @chip_data:		platform-specific per-chip private data for the chip
// methods, to allow shared chip implementations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_data {
    pub mask: u32,
    pub irq: c_uint,
    pub hwirq: irq_hw_number_t,
    pub common: *mut irq_common_data,
    pub chip: *mut irq_chip,
    pub domain: *mut irq_domain,

    pub parent_data: *mut irq_data,

    pub chip_data: *mut c_void,
}

//
// Bit masks for irq_common_data.state_use_accessors
//
// IRQD_TRIGGER_MASK		- Mask for the trigger type bits
// IRQD_SETAFFINITY_PENDING	- Affinity setting is pending
// IRQD_ACTIVATED		- Interrupt has already been activated
// IRQD_NO_BALANCING		- Balancing disabled for this IRQ
// IRQD_PER_CPU			- Interrupt is per cpu
// IRQD_AFFINITY_SET		- Interrupt affinity was set
// IRQD_LEVEL			- Interrupt is level triggered
// IRQD_WAKEUP_STATE		- Interrupt is configured for wakeup
// from suspend
// IRQD_IRQ_DISABLED		- Disabled state of the interrupt
// IRQD_IRQ_MASKED		- Masked state of the interrupt
// IRQD_IRQ_INPROGRESS		- In progress state of the interrupt
// IRQD_WAKEUP_ARMED		- Wakeup mode armed
// IRQD_FORWARDED_TO_VCPU	- The interrupt is forwarded to a VCPU
// IRQD_AFFINITY_MANAGED	- Affinity is auto-managed by the kernel
// IRQD_IRQ_STARTED		- Startup state of the interrupt
// IRQD_MANAGED_SHUTDOWN	- Interrupt was shutdown due to empty affinity
// mask. Applies only to affinity managed irqs.
// IRQD_SINGLE_TARGET		- IRQ allows only a single affinity target
// IRQD_DEFAULT_TRIGGER_SET	- Expected trigger already been set
// IRQD_CAN_RESERVE		- Can use reservation mode
// IRQD_HANDLE_ENFORCE_IRQCTX	- Enforce that handle_irq_*() is only invoked
// from actual interrupt context.
// IRQD_AFFINITY_ON_ACTIVATE	- Affinity is set on activation. Don't call
// irq_chip::irq_set_affinity() when deactivated.
// IRQD_IRQ_ENABLED_ON_SUSPEND	- Interrupt is enabled on suspend by irq pm if
// irqchip have flag IRQCHIP_ENABLE_WAKEUP_ON_SUSPEND set.
// IRQD_RESEND_WHEN_IN_PROGRESS	- Interrupt may fire when already in progress in which
// case it must be resent at the next available opportunity.
//

//
// Must only be called inside irq_chip.irq_set_type() functions or
// from the DT/ACPI setup code.
//
// Must only be called of irqchip.irq_set_affinity() or low level
// hierarchy domain allocation functions.
//

//
// struct irq_chip - hardware interrupt chip descriptor
//
// @name:		name for /proc/interrupts
// @irq_startup:	start up the interrupt (defaults to ->enable if NULL)
// @irq_shutdown:	shut down the interrupt (defaults to ->disable if NULL)
// @irq_enable:		enable the interrupt (defaults to chip->unmask if NULL)
// @irq_disable:	disable the interrupt
// @irq_ack:		start of a new interrupt
// @irq_mask:		mask an interrupt source
// @irq_mask_ack:	ack and mask an interrupt source
// @irq_unmask:		unmask an interrupt source
// @irq_eoi:		end of interrupt
// @irq_set_affinity:	Set the CPU affinity on SMP machines. If the force
// argument is true, it tells the driver to
// unconditionally apply the affinity setting. Sanity
// checks against the supplied affinity mask are not
// required. This is used for CPU hotplug where the
// target CPU is not yet set in the cpu_online_mask.
// @irq_pre_redirect:	Optional function to be invoked before redirecting
// an interrupt via irq_work. Called only on CONFIG_SMP.
// @irq_retrigger:	resend an IRQ to the CPU
// @irq_set_type:	set the flow type (IRQ_TYPE_LEVEL/etc.) of an IRQ
// @irq_set_wake:	enable/disable power-management wake-on of an IRQ
// @irq_bus_lock:	function to lock access to slow bus (i2c) chips
// @irq_bus_sync_unlock:function to sync and unlock slow bus (i2c) chips
// @irq_cpu_online:	configure an interrupt source for a secondary CPU
// @irq_cpu_offline:	un-configure an interrupt source for a secondary CPU
// @irq_suspend:	function called from core code on suspend once per
// chip, when one or more interrupts are installed
// @irq_resume:		function called from core code on resume once per chip,
// when one ore more interrupts are installed
// @irq_pm_shutdown:	function called from core code on shutdown once per chip
// @irq_calc_mask:	Optional function to set irq_data.mask for special cases
// @irq_print_chip:	optional to print special chip info in show_interrupts
// @irq_request_resources:	optional to request resources before calling
// any other callback related to this irq
// @irq_release_resources:	optional to release resources acquired with
// irq_request_resources
// @irq_compose_msi_msg:	optional to compose message content for MSI
// @irq_write_msi_msg:	optional to write message content for MSI
// @irq_get_irqchip_state:	return the internal state of an interrupt
// @irq_set_irqchip_state:	set the internal state of a interrupt
// @irq_set_vcpu_affinity:	optional to target a vCPU in a virtual machine
// @ipi_send_single:	send a single IPI to destination cpus
// @ipi_send_mask:	send an IPI to destination cpus in cpumask
// @irq_nmi_setup:	function called from core code before enabling an NMI
// @irq_nmi_teardown:	function called from core code after disabling an NMI
// @irq_force_complete_move:	optional function to force complete pending irq move
// @flags:		chip specific flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_chip {
    pub name: *const c_char,
    pub data): *mut *mut unsigned int (irq_startup)(struct irq_data,
    pub data): *mut *mut void (irq_shutdown)(struct irq_data,
    pub data): *mut *mut void (irq_enable)(struct irq_data,
    pub data): *mut *mut void (irq_disable)(struct irq_data,
    pub data): *mut *mut void (irq_ack)(struct irq_data,
    pub data): *mut *mut void (irq_mask)(struct irq_data,
    pub data): *mut *mut void (irq_mask_ack)(struct irq_data,
    pub data): *mut *mut void (irq_unmask)(struct irq_data,
    pub data): *mut *mut void (irq_eoi)(struct irq_data,
    pub force): *const *const *const *const int (irq_set_affinity)(struct irq_data data, struct cpumask dest, bool,
    pub data): *mut *mut void (irq_pre_redirect)(struct irq_data,
    pub data): *mut *mut int (irq_retrigger)(struct irq_data,
    pub flow_type): *mut *mut *mut int (irq_set_type)(struct irq_data data, unsigned int,
    pub on): *mut *mut *mut int (irq_set_wake)(struct irq_data data, unsigned int,
    pub data): *mut *mut void (irq_bus_lock)(struct irq_data,
    pub data): *mut *mut void (irq_bus_sync_unlock)(struct irq_data,

    pub data): *mut *mut void (irq_cpu_online)(struct irq_data,
    pub data): *mut *mut void (irq_cpu_offline)(struct irq_data,

    pub data): *mut *mut void (irq_suspend)(struct irq_data,
    pub data): *mut *mut void (irq_resume)(struct irq_data,
    pub data): *mut *mut void (irq_pm_shutdown)(struct irq_data,
    pub data): *mut *mut void (irq_calc_mask)(struct irq_data,
    pub p): *mut *mut *mut void (irq_print_chip)(struct irq_data data, struct seq_file,
    pub data): *mut *mut int (irq_request_resources)(struct irq_data,
    pub data): *mut *mut void (irq_release_resources)(struct irq_data,
    pub msg): *mut *mut *mut void (irq_compose_msi_msg)(struct irq_data data, struct msi_msg,
    pub msg): *mut *mut *mut void (irq_write_msi_msg)(struct irq_data data, struct msi_msg,
    pub state): *mut *mut *mut int (irq_get_irqchip_state)(struct irq_data data, enum irqchip_irq_state which, bool,
    pub state): *mut *mut *mut int (irq_set_irqchip_state)(struct irq_data data, enum irqchip_irq_state which, bool,
    pub vcpu_info): *mut *mut *mut int (irq_set_vcpu_affinity)(struct irq_data data, void,
    pub cpu): *mut *mut *mut void (ipi_send_single)(struct irq_data data, unsigned int,
    pub dest): *const *const *const void (ipi_send_mask)(struct irq_data data, struct cpumask,
    pub data): *mut *mut int (irq_nmi_setup)(struct irq_data,
    pub data): *mut *mut void (irq_nmi_teardown)(struct irq_data,
    pub data): *mut *mut void (irq_force_complete_move)(struct irq_data,
    pub flags: c_ulong,
}

//
// irq_chip specific flags
//
// IRQCHIP_SET_TYPE_MASKED:           Mask before calling chip.irq_set_type()
// IRQCHIP_EOI_IF_HANDLED:            Only issue irq_eoi() when irq was handled
// IRQCHIP_MASK_ON_SUSPEND:           Mask non wake irqs in the suspend path
// IRQCHIP_ONOFFLINE_ENABLED:         Only call irq_on/off_line callbacks
// when irq enabled
// IRQCHIP_SKIP_SET_WAKE:             Skip chip.irq_set_wake(), for this irq chip
// IRQCHIP_ONESHOT_SAFE:              One shot does not require mask/unmask
// IRQCHIP_EOI_THREADED:              Chip requires eoi() on unmask in threaded mode
// IRQCHIP_SUPPORTS_LEVEL_MSI:        Chip can provide two doorbells for Level MSIs
// IRQCHIP_SUPPORTS_NMI:              Chip can deliver NMIs, only for root irqchips
// IRQCHIP_ENABLE_WAKEUP_ON_SUSPEND:  Invokes __enable_irq()/__disable_irq() for wake irqs
// in the suspend path if they are in disabled state
// IRQCHIP_AFFINITY_PRE_STARTUP:      Default affinity update before startup
// IRQCHIP_IMMUTABLE:		      Don't ever change anything in this chip
// IRQCHIP_MOVE_DEFERRED:	      Move the interrupt in actual interrupt context
//

//
// Pick up the arch-dependent methods:
//

extern "C" {
    pub fn irq_cpu_online();
}
extern "C" {
    pub fn irq_cpu_offline();
}

extern "C" {
    pub fn irq_set_vcpu_affinity(irq: c_uint, vcpu_info: *mut c_void) -> c_int;
}

extern "C" {
    pub fn irq_migrate_all_off_this_cpu();
}
extern "C" {
    pub fn irq_affinity_online_cpu(cpu: c_uint) -> c_int;
}

extern "C" {
    pub fn irq_can_move_in_process_context(data: *mut irq_data) -> bool;
}
extern "C" {
    pub fn __irq_move_irq(data: *mut irq_data);
}
extern "C" {
    pub fn irq_move_masked_irq(data: *mut irq_data);
}

extern "C" {
    pub fn irq_set_parent(irq: c_int, parent_irq: c_int) -> c_int;
}

//
// Built-in IRQ handlers for various IRQ types,
// callable via desc->handle_irq()
//
extern "C" {
    pub fn handle_level_irq(desc: *mut irq_desc);
}
extern "C" {
    pub fn handle_fasteoi_irq(desc: *mut irq_desc);
}
extern "C" {
    pub fn handle_edge_irq(desc: *mut irq_desc);
}
extern "C" {
    pub fn handle_edge_eoi_irq(desc: *mut irq_desc);
}
extern "C" {
    pub fn handle_simple_irq(desc: *mut irq_desc);
}
extern "C" {
    pub fn handle_untracked_irq(desc: *mut irq_desc);
}
extern "C" {
    pub fn handle_percpu_irq(desc: *mut irq_desc);
}
extern "C" {
    pub fn handle_percpu_devid_irq(desc: *mut irq_desc);
}
extern "C" {
    pub fn handle_bad_irq(desc: *mut irq_desc);
}
extern "C" {
    pub fn handle_nested_irq(irq: c_uint);
}
extern "C" {
    pub fn handle_fasteoi_nmi(desc: *mut irq_desc);
}
extern "C" {
    pub fn irq_chip_compose_msi_msg(data: *mut irq_data, msg: *mut msi_msg) -> c_int;
}
extern "C" {
    pub fn irq_chip_pm_get(data: *mut irq_data) -> c_int;
}
extern "C" {
    pub fn irq_chip_pm_put(data: *mut irq_data);
}

extern "C" {
    pub fn handle_fasteoi_ack_irq(desc: *mut irq_desc);
}
extern "C" {
    pub fn handle_fasteoi_mask_irq(desc: *mut irq_desc);
}
extern "C" {
    pub fn irq_chip_shutdown_parent(data: *mut irq_data);
}
extern "C" {
    pub fn irq_chip_startup_parent(data: *mut irq_data) -> c_uint;
}
extern "C" {
    pub fn irq_chip_enable_parent(data: *mut irq_data);
}
extern "C" {
    pub fn irq_chip_disable_parent(data: *mut irq_data);
}
extern "C" {
    pub fn irq_chip_ack_parent(data: *mut irq_data);
}
extern "C" {
    pub fn irq_chip_retrigger_hierarchy(data: *mut irq_data) -> c_int;
}
extern "C" {
    pub fn irq_chip_mask_parent(data: *mut irq_data);
}
extern "C" {
    pub fn irq_chip_mask_ack_parent(data: *mut irq_data);
}
extern "C" {
    pub fn irq_chip_unmask_parent(data: *mut irq_data);
}
extern "C" {
    pub fn irq_chip_eoi_parent(data: *mut irq_data);
}
extern "C" {
    pub fn irq_chip_set_wake_parent(data: *mut irq_data, on: c_uint) -> c_int;
}
extern "C" {
    pub fn irq_chip_set_type_parent(data: *mut irq_data, type: c_uint) -> c_int;
}
extern "C" {
    pub fn irq_chip_request_resources_parent(data: *mut irq_data) -> c_int;
}
extern "C" {
    pub fn irq_chip_release_resources_parent(data: *mut irq_data);
}

extern "C" {
    pub fn irq_chip_pre_redirect_parent(data: *mut irq_data);
}

extern "C" {
    pub fn irq_chip_redirect_set_affinity(data: *mut irq_data, dest: *const cpumask, force: bool) -> c_int;
}

// Disable or mask interrupts during a kernel kexec
extern "C" {
    pub fn machine_kexec_mask_interrupts();
}
// Handling of unhandled and spurious interrupts:
extern "C" {
    pub fn note_interrupt(desc: *mut irq_desc, action_ret: irqreturn_t);
}
// Enable/disable irq debugging output:
extern "C" {
    pub fn noirqdebug_setup(str: *mut c_char) -> c_int;
}
// Checks whether the interrupt can be requested by request_irq():
extern "C" {
    pub fn can_request_irq(irq: c_uint, irqflags: c_ulong) -> bool;
}
// Dummy irq-chip implementations:
extern "C" {
    pub fn irq_set_percpu_devid(irq: c_uint) -> c_int;
}
//
// Set a highlevel chained flow handler for a given IRQ.
// (a chained handler is automatically enabled and set to
// IRQ_NOREQUEST, IRQ_NOPROBE, and IRQ_NOTHREAD)
//
// Set a highlevel chained flow handler and its data for a given IRQ.
// (a chained handler is automatically enabled and set to
// IRQ_NOREQUEST, IRQ_NOPROBE, and IRQ_NOTHREAD)
//
extern "C" {
    pub fn irq_modify_status(irq: c_uint, clr: c_ulong, set: c_ulong);
}
// Set/get chip/data for an IRQ:
extern "C" {
    pub fn irq_set_chip(irq: c_uint, chip: *const irq_chip) -> c_int;
}
extern "C" {
    pub fn irq_set_handler_data(irq: c_uint, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn irq_set_chip_data(irq: c_uint, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn irq_set_irq_type(irq: c_uint, type: c_uint) -> c_int;
}
extern "C" {
    pub fn irq_set_msi_desc(irq: c_uint, entry: *mut msi_desc) -> c_int;
}

extern "C" {
    pub fn irq_common_data_get_node(_arg: d->common) -> return;
}

extern "C" {
    pub fn cpumask_of(_arg: 0) -> return;
}

extern "C" {
    pub fn irq_data_get_affinity_mask(_arg: d) -> return;
}

extern "C" {
    pub fn arch_dynirq_lower_bound(from: c_uint) -> c_uint;
}
// use macros to avoid needing export.h for THIS_MODULE

extern "C" {
    pub fn irq_free_descs(irq: c_uint, cnt: c_uint);
}
//
// struct irq_chip_regs - register offsets for struct irq_gci
// @enable:	Enable register offset to reg_base
// @disable:	Disable register offset to reg_base
// @mask:	Mask register offset to reg_base
// @ack:	Ack register offset to reg_base
// @eoi:	Eoi register offset to reg_base
// @type:	Type configuration register offset to reg_base
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_chip_regs {
    pub enable: c_ulong,
    pub disable: c_ulong,
    pub mask: c_ulong,
    pub ack: c_ulong,
    pub eoi: c_ulong,
    pub type: c_ulong,
}

//
// struct irq_chip_type - Generic interrupt chip instance for a flow type
// @chip:		The real interrupt chip which provides the callbacks
// @regs:		Register offsets for this chip
// @handler:		Flow handler associated with this chip
// @type:		Chip can handle these flow types
// @mask_cache_priv:	Cached mask register private to the chip type
// @mask_cache:		Pointer to cached mask register
//
// A irq_generic_chip can have several instances of irq_chip_type when
// it requires different functions and register offsets for different
// flow types.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_chip_type {
    pub chip: irq_chip,
    pub regs: irq_chip_regs,
    pub handler: irq_flow_handler_t,
    pub type: u32,
    pub mask_cache_priv: u32,
    pub mask_cache: *mut u32,
}

//
// struct irq_chip_generic - Generic irq chip data structure
// @lock:		Lock to protect register and cache data access
// @reg_base:		Register base address (virtual)
// @reg_readl:		Alternate I/O accessor (defaults to readl if NULL)
// @reg_writel:		Alternate I/O accessor (defaults to writel if NULL)
// @suspend:		Function called from core code on suspend once per
// chip; can be useful instead of irq_chip::suspend to
// handle chip details even when no interrupts are in use
// @resume:		Function called from core code on resume once per chip;
// can be useful instead of irq_chip::suspend to handle
// chip details even when no interrupts are in use
// @irq_base:		Interrupt base nr for this chip
// @irq_cnt:		Number of interrupts handled by this chip
// @mask_cache:		Cached mask register shared between all chip types
// @wake_enabled:	Interrupt can wakeup from suspend
// @wake_active:	Interrupt is marked as an wakeup from suspend source
// @num_ct:		Number of available irq_chip_type instances (usually 1)
// @private:		Private data for non generic chip callbacks
// @installed:		bitfield to denote installed interrupts
// @unused:		bitfield to denote unused interrupts
// @domain:		irq domain pointer
// @list:		List head for keeping track of instances
// @chip_types:		Array of interrupt irq_chip_types
//
// Note, that irq_chip_generic can have multiple irq_chip_type
// implementations which can be associated to a particular irq line of
// an irq_chip_generic instance. That allows to share and protect
// state in an irq_chip_generic instance when we need to implement
// different flow mechanisms (level/edge) for it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_chip_generic {
    pub lock: raw_spinlock_t,
    pub reg_base: *mut void __iomem,
    pub addr): *mut *mut u32 (reg_readl)(void __iomem,
    pub addr): *mut *mut void (reg_writel)(u32 val, void __iomem,
    pub gc): *mut *mut void (suspend)(struct irq_chip_generic,
    pub gc): *mut *mut void (resume)(struct irq_chip_generic,
    pub irq_base: c_uint,
    pub irq_cnt: c_uint,
    pub mask_cache: u32,
    pub wake_enabled: u32,
    pub wake_active: u32,
    pub num_ct: c_uint,
    pub private: *mut c_void,
    pub installed: c_ulong,
    pub unused: c_ulong,
    pub domain: *mut irq_domain,
    pub list: list_head,
    pub chip_types: [irq_chip_type; ],
}

//
// enum irq_gc_flags - Initialization flags for generic irq chips
// @IRQ_GC_INIT_MASK_CACHE:	Initialize the mask_cache by reading mask reg
// @IRQ_GC_INIT_NESTED_LOCK:	Set the lock class of the irqs to nested for
// irq chips which need to call irq_set_wake() on
// the parent irq. Usually GPIO implementations
// @IRQ_GC_MASK_CACHE_PER_TYPE:	Mask cache is chip type private
// @IRQ_GC_NO_MASK:		Do not calculate irq_data->mask
// @IRQ_GC_BE_IO:		Use big-endian register accesses (default: LE)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irq_gc_flags {
    IRQ_GC_INIT_MASK_CACHE		= 1 << 0,
    IRQ_GC_INIT_NESTED_LOCK		= 1 << 1,
    IRQ_GC_MASK_CACHE_PER_TYPE	= 1 << 2,
    IRQ_GC_NO_MASK			= 1 << 3,
    IRQ_GC_BE_IO			= 1 << 4,
}

//
// struct irq_domain_chip_generic - Generic irq chip data structure for irq domains
// @irqs_per_chip:	Number of interrupts per chip
// @num_chips:		Number of chips
// @irq_flags_to_set:	IRQ* flags to set on irq setup
// @irq_flags_to_clear:	IRQ* flags to clear on irq setup
// @gc_flags:		Generic chip specific setup flags
// @exit:		Function called on each chip when they are destroyed.
// @gc:			Array of pointers to generic interrupt chips
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_domain_chip_generic {
    pub irqs_per_chip: c_uint,
    pub num_chips: c_uint,
    pub irq_flags_to_clear: c_uint,
    pub irq_flags_to_set: c_uint,
    pub gc_flags: irq_gc_flags,
    pub gc): *mut *mut void (exit)(struct irq_chip_generic,
    pub gc: [*mut irq_chip_generic; ],
}

//
// struct irq_domain_chip_generic_info - Generic chip information structure
// @name:		Name of the generic interrupt chip
// @handler:		Interrupt handler used by the generic interrupt chip
// @irqs_per_chip:	Number of interrupts each chip handles (max 32)
// @num_ct:		Number of irq_chip_type instances associated with each
// chip
// @irq_flags_to_clear:	IRQ_* bits to clear in the mapping function
// @irq_flags_to_set:	IRQ_* bits to set in the mapping function
// @gc_flags:		Generic chip specific setup flags
// @init:		Function called on each chip when they are created.
// Allow to do some additional chip initialisation.
// @exit:		Function called on each chip when they are destroyed.
// Allow to do some chip cleanup operation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_domain_chip_generic_info {
    pub name: *const c_char,
    pub handler: irq_flow_handler_t,
    pub irqs_per_chip: c_uint,
    pub num_ct: c_uint,
    pub irq_flags_to_clear: c_uint,
    pub irq_flags_to_set: c_uint,
    pub gc_flags: irq_gc_flags,
    pub gc): *mut *mut int (init)(struct irq_chip_generic,
    pub gc): *mut *mut void (exit)(struct irq_chip_generic,
}

// Generic chip callback functions
extern "C" {
    pub fn irq_gc_noop(d: *mut irq_data);
}
extern "C" {
    pub fn irq_gc_mask_disable_reg(d: *mut irq_data);
}
extern "C" {
    pub fn irq_gc_mask_set_bit(d: *mut irq_data);
}
extern "C" {
    pub fn irq_gc_mask_clr_bit(d: *mut irq_data);
}
extern "C" {
    pub fn irq_gc_unmask_enable_reg(d: *mut irq_data);
}
extern "C" {
    pub fn irq_gc_ack_set_bit(d: *mut irq_data);
}
extern "C" {
    pub fn irq_gc_ack_clr_bit(d: *mut irq_data);
}
extern "C" {
    pub fn irq_gc_mask_disable_and_ack_set(d: *mut irq_data);
}
extern "C" {
    pub fn irq_gc_eoi(d: *mut irq_data);
}
extern "C" {
    pub fn irq_gc_set_wake(d: *mut irq_data, on: c_uint) -> c_int;
}
// Setup functions for irq_chip_generic
extern "C" {
    pub fn irq_unmap_generic_chip(d: *mut irq_domain, virq: c_uint);
}
extern "C" {
    pub fn irq_setup_alt_chip(d: *mut irq_data, type: c_uint) -> c_int;
}

extern "C" {
    pub fn irq_domain_remove_generic_chips(d: *mut irq_domain);
}

extern "C" {
    pub fn container_of(_arg: d->chip, irq_chip_type: struct, _arg: chip) -> return;
}

extern "C" {
    pub fn readl(reg_offset: gc->reg_base +) -> return;
}
extern "C" {
    pub fn irq_matrix_online(m: *mut irq_matrix);
}
extern "C" {
    pub fn irq_matrix_offline(m: *mut irq_matrix);
}
extern "C" {
    pub fn irq_matrix_assign_system(m: *mut irq_matrix, bit: c_uint, replace: bool);
}
extern "C" {
    pub fn irq_matrix_reserve_managed(m: *mut irq_matrix, msk: *const cpumask) -> c_int;
}
extern "C" {
    pub fn irq_matrix_remove_managed(m: *mut irq_matrix, msk: *const cpumask);
}
extern "C" {
    pub fn irq_matrix_reserve(m: *mut irq_matrix);
}
extern "C" {
    pub fn irq_matrix_remove_reserved(m: *mut irq_matrix);
}
extern "C" {
    pub fn irq_matrix_assign(m: *mut irq_matrix, bit: c_uint);
}
extern "C" {
    pub fn irq_matrix_available(m: *mut irq_matrix, cpudown: bool) -> c_uint;
}
extern "C" {
    pub fn irq_matrix_allocated(m: *mut irq_matrix) -> c_uint;
}
extern "C" {
    pub fn irq_matrix_reserved(m: *mut irq_matrix) -> c_uint;
}
extern "C" {
    pub fn irq_matrix_debug_show(sf: *mut seq_file, m: *mut irq_matrix, ind: c_int);
}
// Contrary to Linux irqs, for hardware irqs the irq number 0 is valid

extern "C" {
    pub fn ipi_get_hwirq(irq: c_uint, cpu: c_uint) -> irq_hw_number_t;
}
extern "C" {
    pub fn __ipi_send_single(desc: *mut irq_desc, cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn __ipi_send_mask(desc: *mut irq_desc, dest: *const cpumask) -> c_int;
}
extern "C" {
    pub fn ipi_send_single(virq: c_uint, cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn ipi_send_mask(virq: c_uint, dest: *const cpumask) -> c_int;
}
extern "C" {
    pub fn ipi_mux_process();
}
extern "C" {
    pub fn ipi_mux_create(nr_ipi: c_uint, cpu): *mut *mut void (mux_send)(unsigned int) -> c_int;
}

//
// Registers a generic IRQ handling function as the top-level IRQ handler in
// the system, which is generally the first C code called from an assembly
// architecture-specific interrupt handler.
//
// Returns 0 on success, or -EBUSY if an IRQ handler has already been
// registered.
//
extern "C" {
    pub fn set_handle_irq(): *mut *mut void (handle_irq)(struct pt_regs) -> int __init;
}
//
// Allows interrupt handlers to find the irqchip that's been registered as the
// top-level IRQ handler.
//
extern "C" {
    pub fn generic_handle_arch_irq(regs: *mut pt_regs) -> asmlinkage void;
}

