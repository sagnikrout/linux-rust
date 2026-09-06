//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/msi.h
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
// This header file contains MSI data structures and functions which are
// only relevant for:
// - Interrupt core code
// - PCI/MSI core code
// - MSI interrupt domain implementations
// - IOMMU, low level VFIO, NTB and other justified exceptions
// dealing with low level MSI details.
//
// Regular device drivers have no business with any of these functions and
// especially storing MSI descriptor pointers in random code is considered
// abuse.
//
// Device driver relevant functions are available in <linux/msi_api.h>
//

// Dummy shadow structures if an architecture does not define them

//
// struct msi_msg - Representation of a MSI message
// @address_lo:		Low 32 bits of msi message address
// @arch_addr_lo:	Architecture specific shadow of @address_lo
// @address_hi:		High 32 bits of msi message address
// (only used when device supports it)
// @arch_addr_hi:	Architecture specific shadow of @address_hi
// @data:		MSI message data (usually 16 bits)
// @arch_data:		Architecture specific shadow of @data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msi_msg {
    pub address_lo: u32,
    pub arch_addr_lo: arch_msi_msg_addr_lo_t,
}

// Helper functions
extern "C" {
    pub fn __get_cached_msi_msg(entry: *mut msi_desc, msg: *mut msi_msg);
}

extern "C" {
    pub fn get_cached_msi_msg(irq: c_uint, msg: *mut msi_msg);
}

//
// struct pci_msi_desc - PCI/MSI specific MSI descriptor data
//
// @msi_mask:	[PCI MSI]   MSI cached mask bits
// @msix_ctrl:	[PCI MSI-X] MSI-X cached per vector control bits
// @is_msix:	[PCI MSI/X] True if MSI-X
// @multiple:	[PCI MSI/X] log2 num of messages allocated
// @multi_cap:	[PCI MSI/X] log2 num of messages supported
// @can_mask:	[PCI MSI/X] Masking supported?
// @is_64:	[PCI MSI/X] Address size: 0=32bit 1=64bit
// @default_irq:[PCI MSI/X] The default pre-assigned non-MSI irq
// @msi_attrib:	[PCI MSI/X] Compound struct of MSI/X attributes
// @mask_pos:	[PCI MSI]   Mask register position
// @mask_base:	[PCI MSI-X] Mask register base address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_msi_desc {
    pub msi_mask: u32,
    pub msix_ctrl: u32,
}

//
// union msi_domain_cookie - Opaque MSI domain specific data
// @value:	u64 value store
// @ptr:	Pointer to domain specific data
// @iobase:	Domain specific IOmem pointer
//
// The content of this data is implementation defined and used by the MSI
// domain to store domain specific information which is requried for
// interrupt chip callbacks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union msi_domain_cookie {
    pub value: u64,
    pub ptr: *mut c_void,
    pub iobase: *mut void __iomem,
}

//
// struct msi_desc_data - Generic MSI descriptor data
// @dcookie:	Cookie for MSI domain specific data which is required
// for irq_chip callbacks
// @icookie:	Cookie for the MSI interrupt instance provided by
// the usage site to the allocation function
//
// The content of this data is implementation defined, e.g. PCI/IMS
// implementations define the meaning of the data. The MSI core ignores
// this data completely.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msi_desc_data {
    pub dcookie: msi_domain_cookie,
    pub icookie: msi_instance_cookie,
}

//
// struct msi_desc - Descriptor structure for MSI based interrupts
// @irq:	The base interrupt number
// @nvec_used:	The number of vectors used
// @dev:	Pointer to the device which uses this descriptor
// @msg:	The last set MSI message cached for reuse
// @affinity:	Optional pointer to a cpu affinity mask for this descriptor
// @iommu_msi_iova: Optional shifted IOVA from the IOMMU to override the msi_addr.
// Only used if iommu_msi_shift != 0
// @iommu_msi_shift: Indicates how many bits of the original address should be
// preserved when using iommu_msi_iova.
// @sysfs_attrs:	Pointer to sysfs device attribute
//
// @write_msi_msg:	Callback that may be called when the MSI message
// address or data changes
// @write_msi_msg_data:	Data parameter for the callback.
//
// @msi_index:	Index of the msi descriptor
// @pci:	PCI specific msi descriptor data
// @data:	Generic MSI descriptor data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msi_desc {
// Shared device/bus type independent data
    pub irq: c_uint,
    pub nvec_used: c_uint,
    pub dev: *mut device,
    pub msg: msi_msg,
    pub affinity: *mut irq_affinity_desc,

    pub 58: u64 iommu_msi_iova :,
    pub 6: u64 iommu_msi_shift :,

    pub sysfs_attrs: *mut device_attribute,

    pub data): *mut *mut *mut void (write_msi_msg)(struct msi_desc entry, void,
    pub write_msi_msg_data: *mut c_void,
    pub msi_index: u16,
    pub pci: pci_msi_desc,
    pub data: msi_desc_data,
}

//
// Filter values for the MSI descriptor iterators and accessor functions.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msi_desc_filter {
// All descriptors
    MSI_DESC_ALL,
// Descriptors which have no interrupt associated
    MSI_DESC_NOTASSOCIATED,
// Descriptors which have an interrupt associated
    MSI_DESC_ASSOCIATED,
}

//
// struct msi_dev_domain - The internals of MSI domain info per device
// @store:		Xarray for storing MSI descriptor pointers
// @domain:		Pointer to a per device interrupt domain
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msi_dev_domain {
    pub store: xarray,
    pub domain: *mut irq_domain,
}

extern "C" {
    pub fn msi_setup_device_data(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn __msi_lock_descs(dev: *mut device);
}
extern "C" {
    pub fn __msi_unlock_descs(dev: *mut device);
}
//
// msi_first_desc - Get the first MSI descriptor of the default irqdomain
// @dev:	Device to operate on
// @filter:	Descriptor state filter
//
// Must be called with the MSI descriptor mutex held, i.e. msi_lock_descs()
// must be invoked before the call.
//
// Return: Pointer to the first MSI descriptor matching the search
// criteria, NULL if none found.
//
extern "C" {
    pub fn msi_domain_first_desc(_arg: dev, _arg: MSI_DEFAULT_DOMAIN, _arg: filter) -> return;
}
//
// msi_domain_for_each_desc - Iterate the MSI descriptors in a specific domain
//
// @desc:	struct msi_desc pointer used as iterator
// @dev:	struct device pointer - device to iterate
// @domid:	The id of the interrupt domain which should be walked.
// @filter:	Filter for descriptor selection
//
// Notes:
// - The loop must be protected with a msi_lock_descs()/msi_unlock_descs()
// pair.
// - It is safe to remove a retrieved MSI descriptor in the loop.
//

//
// msi_for_each_desc - Iterate the MSI descriptors in the default irqdomain
//
// @desc:	struct msi_desc pointer used as iterator
// @dev:	struct device pointer - device to iterate
// @filter:	Filter for descriptor selection
//
// Notes:
// - The loop must be protected with a msi_lock_descs()/msi_unlock_descs()
// pair.
// - It is safe to remove a retrieved MSI descriptor in the loop.
//

//
// msi_msg_set_addr() - Set MSI address in an MSI message
//
// @desc:	MSI descriptor that may carry an IOVA base address for MSI via @iommu_msi_iova/shift
// @msg:	Target MSI message to set its address_hi and address_lo
// @msi_addr:	Physical address to set the MSI message
//
// Notes:
// - Override @msi_addr using the IOVA base address in the @desc if @iommu_msi_shift is set
// - Otherwise, simply set @msi_addr to @msg
//

//
// msi_insert_msi_desc - Allocate and initialize a MSI descriptor in the
// default irqdomain and insert it at @init_desc->msi_index
// @dev:	Pointer to the device for which the descriptor is allocated
// @init_desc:	Pointer to an MSI descriptor to initialize the new descriptor
//
// Return: 0 on success or an appropriate failure code.
//
extern "C" {
    pub fn msi_domain_insert_msi_desc(_arg: dev, _arg: MSI_DEFAULT_DOMAIN, _arg: init_desc) -> return;
}
//
// msi_free_msi_descs_range - Free a range of MSI descriptors of a device
// in the default irqdomain
//
// @dev:	Device for which to free the descriptors
// @first:	Index to start freeing from (inclusive)
// @last:	Last index to be freed (inclusive)
//
// msi_free_msi_descs - Free all MSI descriptors of a device in the default irqdomain
// @dev:	Device to free the descriptors
//
// The arch hooks to setup up msi irqs. Default functions are implemented
// as weak symbols so that they /can/ be overriden by architecture specific
// code if needed. These hooks can only be enabled by the architecture.
//
// If CONFIG_PCI_MSI_ARCH_FALLBACKS is not selected they are replaced by
// stubs with warnings.
//

extern "C" {
    pub fn arch_setup_msi_irq(dev: *mut pci_dev, desc: *mut msi_desc) -> c_int;
}
extern "C" {
    pub fn arch_teardown_msi_irq(irq: c_uint);
}
extern "C" {
    pub fn arch_setup_msi_irqs(dev: *mut pci_dev, nvec: c_int, type: c_int) -> c_int;
}
extern "C" {
    pub fn arch_teardown_msi_irqs(dev: *mut pci_dev);
}

//
// Xen uses non-default msi_domain_ops and hence needs a way to populate sysfs
// entries of MSI IRQs.
//

extern "C" {
    pub fn msi_device_populate_sysfs(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn msi_device_destroy_sysfs(dev: *mut device);
}

//
// The restore hook is still available even for fully irq domain based
// setups. Courtesy to XEN/X86.
//
extern "C" {
    pub fn arch_restore_msi_irqs(dev: *mut pci_dev) -> bool;
}

//
// struct msi_domain_ops - MSI interrupt domain callbacks
// @get_hwirq:		Retrieve the resulting hw irq number
// @msi_init:		Domain specific init function for MSI interrupts
// @msi_free:		Domain specific function to free a MSI interrupts
// @msi_prepare:	Prepare the allocation of the interrupts in the domain
// @msi_teardown:	Reverse the effects of @msi_prepare
// @prepare_desc:	Optional function to prepare the allocated MSI descriptor
// in the domain
// @set_desc:		Set the msi descriptor for an interrupt
// @domain_alloc_irqs:	Optional function to override the default allocation
// function.
// @domain_free_irqs:	Optional function to override the default free
// function.
// @msi_translate:	Optional translate callback to support the odd wire to
// MSI bridges, e.g. MBIGEN
//
// @get_hwirq, @msi_init and @msi_free are callbacks used by the underlying
// irqdomain.
//
// @msi_check, @msi_prepare, @msi_teardown, @prepare_desc and
// @set_desc are callbacks used by the msi_domain_alloc/free_irqs*()
// variants.
//
// @domain_alloc_irqs, @domain_free_irqs can be used to override the
// default allocation/free functions (__msi_domain_alloc/free_irqs). This
// is initially for a wrapper around XEN's separate MSI universe which can't
// be wrapped into the regular irq domains concepts by mere mortals.  This
// allows to universally use msi_domain_alloc/free_irqs without having to
// special case XEN all over the place.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msi_domain_ops {
    pub arg): *mut msi_alloc_info_t,
    pub arg): *mut msi_alloc_info_t,
    pub virq): c_uint,
    pub arg): *mut msi_alloc_info_t,
    pub arg): *mut msi_alloc_info_t,
    pub desc): *mut msi_desc,
    pub desc): *mut msi_desc,
    pub nvec): *mut *mut device dev, int,
    pub dev): *mut device,
    pub type): *mut *mut irq_hw_number_t hwirq, unsigned int,
}

//
// struct msi_domain_info - MSI interrupt domain data
// @flags:		Flags to decribe features and capabilities
// @bus_token:		The domain bus token
// @hwsize:		The hardware table size or the software index limit.
// If 0 then the size is considered unlimited and
// gets initialized to the maximum software index limit
// by the domain creation code.
// @ops:		The callback data structure
// @dev:		Device which creates the domain
// @chip:		Optional: associated interrupt chip
// @chip_data:		Optional: associated interrupt chip data
// @handler:		Optional: associated interrupt flow handler
// @handler_data:	Optional: associated interrupt flow handler data
// @handler_name:	Optional: associated interrupt flow handler name
// @alloc_data:		Optional: associated interrupt allocation data
// @data:		Optional: domain specific data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msi_domain_info {
    pub flags: u32,
    pub bus_token: irq_domain_bus_token,
    pub hwsize: c_uint,
    pub ops: *mut msi_domain_ops,
    pub dev: *mut device,
    pub chip: *mut irq_chip,
    pub chip_data: *mut c_void,
    pub handler: irq_flow_handler_t,
    pub handler_data: *mut c_void,
    pub handler_name: *const c_char,
    pub alloc_data: *mut msi_alloc_info_t,
    pub data: *mut c_void,
}

//
// struct msi_domain_template - Template for MSI device domains
// @name:	Storage for the resulting name. Filled in by the core.
// @chip:	Interrupt chip for this domain
// @ops:	MSI domain ops
// @info:	MSI domain info data
// @alloc_info:	MSI domain allocation data (architecture specific)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msi_domain_template {
    pub name: [c_char; 48],
    pub chip: irq_chip,
    pub ops: msi_domain_ops,
    pub info: msi_domain_info,
    pub alloc_info: msi_alloc_info_t,
}

//
// Flags for msi_domain_info
//
// Bit 0-15:	Generic MSI functionality which is not subject to restriction
// by parent domains
//
// Bit 16-31:	Functionality which depends on the underlying parent domain and
// can be masked out by msi_parent_ops::init_dev_msi_info() when
// a device MSI domain is initialized.
//
// Init non implemented ops callbacks with default MSI domain
// callbacks.
//
// Init non implemented chip callbacks with default MSI chip
// callbacks.
//
// Needs early activate, required for PCI
//
// Must reactivate when irq is started even when
// MSI_FLAG_ACTIVATE_EARLY has been set.
//
// Populate sysfs on alloc() and destroy it on free()
// Allocate simple MSI descriptors
// Free MSI descriptors
// Use dev->fwnode for MSI device domain creation
// Set parent->dev into domain->pm_dev on device domain creation
// Support for parent mask/unmask
// Support for parent startup/shutdown
// Mask for the generic functionality
// Mask for the domain specific functionality
// Support multiple PCI MSI interrupts
// Support PCI MSIX interrupts
// Is level-triggered capable, using two messages
// MSI-X entries must be contiguous
// PCI/MSI-X vectors can be dynamically allocated/freed post MSI-X enable
// PCI MSIs cannot be steered separately to CPU cores
// Inhibit usage of entry masking
//
// Flags for msi_parent_ops::chip_flags
//
// struct msi_parent_ops - MSI parent domain callbacks and configuration info
//
// @supported_flags:	Required: The supported MSI flags of the parent domain
// @required_flags:	Optional: The required MSI flags of the parent MSI domain
// @chip_flags:		Optional: Select MSI chip callbacks to update with defaults
// in msi_lib_init_dev_msi_info().
// @bus_select_token:	Optional: The bus token of the real parent domain for
// irq_domain::select()
// @bus_select_mask:	Optional: A mask of supported BUS_DOMAINs for
// irq_domain::select()
// @prefix:		Optional: Prefix for the domain and chip name
// @init_dev_msi_info:	Required: Callback for MSI parent domains to setup parent
// domain specific domain flags, domain ops and interrupt chip
// callbacks when a per device domain is created.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msi_parent_ops {
    pub supported_flags: u32,
    pub required_flags: u32,
    pub chip_flags: u32,
    pub bus_select_token: u32,
    pub bus_select_mask: u32,
    pub prefix: *const c_char,
    pub msi_child_info): *mut msi_domain_info,
}

extern "C" {
    pub fn msi_remove_device_irq_domain(dev: *mut device, domid: c_uint);
}
extern "C" {
    pub fn msi_domain_alloc_irqs_all_locked(dev: *mut device, domid: c_uint, nirqs: c_int) -> c_int;
}
extern "C" {
    pub fn msi_domain_free_irqs_all_locked(dev: *mut device, domid: c_uint);
}
extern "C" {
    pub fn msi_domain_free_irqs_all(dev: *mut device, domid: c_uint);
}
// Per device platform MSI
extern "C" {
    pub fn platform_device_msi_free_irqs_all(dev: *mut device);
}
extern "C" {
    pub fn msi_device_has_isolated_msi(dev: *mut device) -> bool;
}
extern "C" {
    pub fn msi_domain_alloc_irqs_range(_arg: dev, _arg: domid, _arg: 0, 1: nirqs -) -> return;
}

//
// Arguably if the platform does not enable MSI support then it has
// "isolated MSI", as an interrupt controller that cannot receive MSIs
// is inherently isolated by our definition. The default definition for
// arch_is_isolated_msi() is conservative and returns false anyhow.
//
extern "C" {
    pub fn arch_is_isolated_msi() -> return;
}

// PCI specific interfaces

extern "C" {
    pub fn pci_write_msi_msg(irq: c_uint, msg: *mut msi_msg);
}
extern "C" {
    pub fn __pci_read_msi_msg(entry: *mut msi_desc, msg: *mut msi_msg);
}
extern "C" {
    pub fn __pci_write_msi_msg(entry: *mut msi_desc, msg: *mut msi_msg);
}
extern "C" {
    pub fn pci_msi_mask_irq(data: *mut irq_data);
}
extern "C" {
    pub fn pci_msi_unmask_irq(data: *mut irq_data);
}
extern "C" {
    pub fn pci_msi_domain_get_msi_rid(domain: *mut irq_domain, pdev: *mut pci_dev) -> u32;
}

