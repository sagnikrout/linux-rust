//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/irqdomain.h
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
// irq_domain - IRQ Translation Domains
//
// See Documentation/core-api/irq/irq-domain.rst for the details.
//

pub const IRQ_DOMAIN_IRQ_SPEC_PARAMS: c_int = 16;
//
// struct irq_fwspec - generic IRQ specifier structure
//
// @fwnode:		Pointer to a firmware-specific descriptor
// @param_count:	Number of device-specific parameters
// @param:		Device-specific parameters
//
// This structure, directly modeled after of_phandle_args, is used to
// pass a device-specific description of an interrupt.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_fwspec {
    pub fwnode: *mut fwnode_handle,
    pub param_count: c_int,
    pub param: [u32; IRQ_DOMAIN_IRQ_SPEC_PARAMS],
}

//
// struct irq_fwspec_info - firmware provided IRQ information structure
//
// @flags:		Information validity flags
// @affinity:		Affinity mask for this interrupt
//
// This structure reports firmware-specific information about an
// interrupt. The only significant information is the affinity of a
// per-CPU interrupt, but this is designed to be extended as required.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_fwspec_info {
    pub flags: c_ulong,
    pub affinity: *const cpumask,
}

// Conversion function from of_phandle_args fields to fwspec
//
// struct irq_domain_ops - Methods for irq_domain objects
// @match:	Match an interrupt controller device node to a domain, returns
// 1 on a match
// @select:	Match an interrupt controller fw specification. It is more generic
// than @match as it receives a complete struct irq_fwspec. Therefore,
// @select is preferred if provided. Returns 1 on a match.
// @map:	Create or update a mapping between a virtual irq number and a hw
// irq number. This is called only once for a given mapping.
// @unmap:	Dispose of such a mapping
// @xlate:	Given a device tree node and interrupt specifier, decode
// the hardware irq number and linux irq type value.
// @alloc:	Allocate @nr_irqs interrupts starting from @virq.
// @free:	Free @nr_irqs interrupts starting from @virq.
// @activate:	Activate one interrupt in HW (@irqd). If @reserve is set, only
// reserve the vector. If unset, assign the vector (called from
// request_irq()).
// @deactivate:	Disarm one interrupt (@irqd).
// @translate:	Given @fwspec, decode the hardware irq number (@out_hwirq) and
// linux irq type value (@out_type). This is a generalised @xlate
// (over struct irq_fwspec) and is preferred if provided.
// @get_fwspec_info:
// Given @fwspec, report additional firmware-provided information in
// @info. Optional.
// @debug_show:	For domains to show specific data for an interrupt in debugfs.
//
// Functions below are provided by the driver and called whenever a new mapping
// is created or an old mapping is disposed. The driver can then proceed to
// whatever internal data structures management is required. It also needs
// to setup the irq_desc when returning from map().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_domain_ops {
    pub bus_token): irq_domain_bus_token,
    pub bus_token): irq_domain_bus_token,
    pub hw): *mut *mut *mut int (map)(struct irq_domain d, unsigned int virq, irq_hw_number_t,
    pub virq): *mut *mut *mut void (unmap)(struct irq_domain d, unsigned int,
    pub out_type): *mut *mut unsigned long out_hwirq, unsigned int,

// extended V2 interfaces to support hierarchy irq_domains
    pub arg): *mut unsigned int nr_irqs, void,
    pub nr_irqs): c_uint,
    pub reserve): *mut *mut *mut *mut int (activate)(struct irq_domain d, struct irq_data irqd, bool,
    pub irq_data): *mut *mut *mut void (deactivate)(struct irq_domain d, struct irq_data,
    pub out_type): *mut *mut unsigned long out_hwirq, unsigned int,
    pub info): *mut *mut *mut int (get_fwspec_info)(struct irq_fwspec fwspec, struct irq_fwspec_info,

    pub ind): *mut *mut irq_data irqd, int,

}

//
// struct irq_domain - Hardware interrupt number translation object
// @link:	Element in global irq_domain list.
// @name:	Name of interrupt domain
// @ops:	Pointer to irq_domain methods
// @host_data:	Private data pointer for use by owner.  Not touched by irq_domain
// core code.
// @flags:	Per irq_domain flags
// @mapcount:	The number of mapped interrupts
// @mutex:	Domain lock, hierarchical domains use root domain's lock
// @root:	Pointer to root domain, or containing structure if non-hierarchical
//
// Optional elements:
// @fwnode:	Pointer to firmware node associated with the irq_domain. Pretty easy
// to swap it for the of_node via the irq_domain_get_of_node accessor
// @bus_token:	@fwnode's device_node might be used for several irq domains. But
// in connection with @bus_token, the pair shall be unique in a
// system.
// @gc:		Pointer to a list of generic chips. There is a helper function for
// setting up one or more generic chips for interrupt controllers
// drivers using the generic chip library which uses this pointer.
// @dev:	Pointer to the device which instantiated the irqdomain
// With per device irq domains this is not necessarily the same
// as @pm_dev.
// @pm_dev:	Pointer to a device that can be utilized for power management
// purposes related to the irq domain.
// @parent:	Pointer to parent irq_domain to support hierarchy irq_domains
// @msi_parent_ops: Pointer to MSI parent domain methods for per device domain init
// @exit:	Function called when the domain is destroyed
//
// Revmap data, used internally by the irq domain code:
// @hwirq_max:		Top limit for the HW irq number. Especially to avoid
// conflicts/failures with reserved HW irqs. Can be ~0.
// @revmap_size:	Size of the linear map table @revmap
// @revmap_tree:	Radix map tree for hwirqs that don't fit in the linear map
// @revmap:		Linear table of irq_data pointers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_domain {
    pub link: list_head,
    pub name: *const c_char,
    pub ops: *const irq_domain_ops,
    pub host_data: *mut c_void,
    pub flags: c_uint,
    pub mapcount: c_uint,
    pub mutex: mutex,
    pub root: *mut irq_domain,
// Optional data
    pub fwnode: *mut fwnode_handle,
    pub bus_token: irq_domain_bus_token,
    pub gc: *mut irq_domain_chip_generic,
    pub dev: *mut device,
    pub pm_dev: *mut device,

    pub parent: *mut irq_domain,

    pub msi_parent_ops: *const msi_parent_ops,

    pub d): *mut *mut void (exit)(struct irq_domain,
// reverse map data. The linear map gets appended to the irq_domain
    pub hwirq_max: irq_hw_number_t,
    pub revmap_size: c_uint,
    pub revmap_tree: radix_tree_root,
    pub __counted_by(revmap_size): *mut *mut irq_data __rcu revmap[],
}

// Irq domain flags
// Irq domain is hierarchical
// Irq domain name was allocated internally
// Irq domain is an IPI domain with virq per cpu
// Irq domain is an IPI domain with single virq
// Irq domain implements MSIs
//
// Irq domain implements isolated MSI, see msi_device_has_isolated_msi()
//
// Irq domain doesn't translate anything
// Irq domain is a MSI parent domain
// Irq domain is a MSI device domain
// Irq domain must destroy generic chips when removed
// Address and data pair is mutable when irq_set_affinity()
// IRQ domain requires parent fwnode matching
//
// Flags starting from IRQ_DOMAIN_FLAG_NONCORE are reserved
// for implementation specific purposes and ignored by the
// core code.
//
extern "C" {
    pub fn to_of_node(_arg: d->fwnode) -> return;
}

extern "C" {
    pub fn __irq_domain_alloc_fwnode(_arg: IRQCHIP_FWNODE_NAMED, _arg: 0, _arg: name, _arg: NULL, _arg: NULL) -> return;
}
extern "C" {
    pub fn __irq_domain_alloc_fwnode(_arg: IRQCHIP_FWNODE_NAMED, _arg: 0, _arg: name, _arg: NULL, _arg: parent) -> return;
}
extern "C" {
    pub fn __irq_domain_alloc_fwnode(_arg: IRQCHIP_FWNODE_REAL, _arg: 0, _arg: NULL, _arg: pa, _arg: NULL) -> return;
}
extern "C" {
    pub fn __irq_domain_alloc_fwnode(_arg: IRQCHIP_FWNODE_REAL, _arg: 0, _arg: NULL, _arg: pa, _arg: parent) -> return;
}
extern "C" {
    pub fn irq_domain_free_fwnode(fwnode: *mut fwnode_handle);
}
//
// struct irq_domain_info - Domain information structure
// @fwnode:		firmware node for the interrupt controller
// @domain_flags:	Additional flags to add to the domain flags
// @size:		Size of linear map; 0 for radix mapping only
// @hwirq_max:		Maximum number of interrupts supported by controller
// @direct_max:		Maximum value of direct maps;
// Use ~0 for no limit; 0 for no direct mapping
// @hwirq_base:		The first hardware interrupt number (legacy domains only)
// @virq_base:		The first Linux interrupt number for legacy domains to
// immediately associate the interrupts after domain creation
// @bus_token:		Domain bus token
// @name_suffix:	Optional name suffix to avoid collisions when multiple
// domains are added using same fwnode
// @ops:		Domain operation callbacks
// @host_data:		Controller private data pointer
// @dev:		Device which creates the domain
// @dgc_info:		Geneneric chip information structure pointer used to
// create generic chips for the domain if not NULL.
// @init:		Function called when the domain is created.
// Allow to do some additional domain initialisation.
// @exit:		Function called when the domain is destroyed.
// Allow to do some additional cleanup operation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_domain_info {
    pub fwnode: *mut fwnode_handle,
    pub domain_flags: c_uint,
    pub size: c_uint,
    pub hwirq_max: irq_hw_number_t,
    pub direct_max: c_int,
    pub hwirq_base: c_uint,
    pub virq_base: c_uint,
    pub bus_token: irq_domain_bus_token,
    pub name_suffix: *const c_char,
    pub ops: *const irq_domain_ops,
    pub host_data: *mut c_void,
    pub dev: *mut device,

//
// @parent: Pointer to the parent irq domain used in a hierarchy domain
//
    pub parent: *mut irq_domain,

    pub dgc_info: *mut irq_domain_chip_generic_info,
    pub d): *mut *mut int (init)(struct irq_domain,
    pub d): *mut *mut void (exit)(struct irq_domain,
}

extern "C" {
    pub fn irq_set_default_domain(domain: *mut irq_domain);
}
extern "C" {
    pub fn irq_domain_update_bus_token(domain: *mut irq_domain, bus_token: irq_domain_bus_token);
}
extern "C" {
    pub fn irq_find_matching_fwspec(_arg: &fwspec, _arg: bus_token) -> return;
}
extern "C" {
    pub fn irq_find_matching_fwnode(_arg: of_fwnode_handle(node), _arg: bus_token) -> return;
}

extern "C" {
    pub fn irq_create_direct_mapping(domain: *mut irq_domain) -> c_uint;
}

//
// irq_domain_create_linear - Allocate and register a linear revmap irq_domain.
// @fwnode:	pointer to interrupt controller's FW node.
// @size:	Number of interrupts in the domain.
// @ops:	map/unmap domain callbacks
// @host_data:	Controller private data pointer
//
// Returns: Newly created irq_domain
//
extern "C" {
    pub fn irq_domain_remove(domain: *mut irq_domain);
}
extern "C" {
    pub fn irq_domain_associate(domain: *mut irq_domain, irq: c_uint, hwirq: irq_hw_number_t) -> c_int;
}
extern "C" {
    pub fn irq_create_fwspec_mapping(fwspec: *mut irq_fwspec) -> c_uint;
}
extern "C" {
    pub fn irq_dispose_mapping(virq: c_uint);
}
//
// irq_create_mapping - Map a hardware interrupt into linux irq space
// @domain:	domain owning this hardware interrupt or NULL for default domain
// @hwirq:	hardware irq number in that domain space
//
// Only one mapping per hardware interrupt is permitted.
//
// If the sense/trigger is to be specified, set_irq_type() should be called
// on the number returned from that call.
//
// Returns: Linux irq number or 0 on error
//
extern "C" {
    pub fn irq_create_mapping_affinity(_arg: domain, _arg: hwirq, _arg: NULL) -> return;
}
//
// irq_resolve_mapping - Find a linux irq from a hw irq number.
// @domain:	domain owning this hardware interrupt
// @hwirq:	hardware irq number in that domain space
//
// Returns: Interrupt descriptor
//
extern "C" {
    pub fn __irq_resolve_mapping(_arg: domain, _arg: hwirq, _arg: NULL) -> return;
}
//
// irq_find_mapping() - Find a linux irq from a hw irq number.
// @domain:	domain owning this hardware interrupt
// @hwirq:	hardware irq number in that domain space
//
// Returns: Linux irq number or 0 if not found
//
// stock xlate functions
// IPI functions
extern "C" {
    pub fn irq_reserve_ipi(domain: *mut irq_domain, dest: *const cpumask) -> c_int;
}
extern "C" {
    pub fn irq_destroy_ipi(irq: c_uint, dest: *const cpumask) -> c_int;
}
// V2 interfaces to support hierarchy IRQ domains.
extern "C" {
    pub fn irq_domain_reset_irq_data(irq_data: *mut irq_data);
}

//
// irq_domain_create_hierarchy - Add a irqdomain into the hierarchy
// @parent:	Parent irq domain to associate with the new domain
// @flags:	Irq domain flags associated to the domain
// @size:	Size of the domain. See below
// @fwnode:	Optional fwnode of the interrupt controller
// @ops:	Pointer to the interrupt domain callbacks
// @host_data:	Controller private data pointer
//
// If @size is 0 a tree domain is created, otherwise a linear domain.
//
// If successful the parent is associated to the new domain and the
// domain flags are set.
//
// Returns: A pointer to IRQ domain, or %NULL on failure.
//
extern "C" {
    pub fn irq_domain_free_irqs(virq: c_uint, nr_irqs: c_uint);
}
extern "C" {
    pub fn irq_domain_activate_irq(irq_data: *mut irq_data, early: bool) -> c_int;
}
extern "C" {
    pub fn irq_domain_deactivate_irq(irq_data: *mut irq_data);
}
//
// irq_domain_alloc_irqs - Allocate IRQs from domain
// @domain:	domain to allocate from
// @nr_irqs:	number of IRQs to allocate
// @node:	NUMA node id for memory allocation
// @arg:	domain specific argument
//
// See __irq_domain_alloc_irqs()' documentation.
//
extern "C" {
    pub fn __irq_domain_alloc_irqs(_arg: domain, _arg: -1, _arg: nr_irqs, _arg: node, _arg: arg, _arg: false, _arg: NULL) -> return;
}
extern "C" {
    pub fn irq_domain_free_irqs_top(domain: *mut irq_domain, virq: c_uint, nr_irqs: c_uint);
}
extern "C" {
    pub fn irq_domain_push_irq(domain: *mut irq_domain, virq: c_int, arg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn irq_domain_pop_irq(domain: *mut irq_domain, virq: c_int) -> c_int;
}
extern "C" {
    pub fn irq_domain_disconnect_hierarchy(domain: *mut irq_domain, virq: c_uint) -> c_int;
}
extern "C" {
    pub fn irq_populate_fwspec_info(fwspec: *mut irq_fwspec, info: *mut irq_fwspec_info) -> c_int;
}

extern "C" {
    pub fn msi_device_domain_alloc_wired(domain: *mut irq_domain, hwirq: c_uint, type: c_uint) -> c_int;
}
extern "C" {
    pub fn msi_device_domain_free_wired(domain: *mut irq_domain, virq: c_uint);
}

