//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/edac.h
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
// Generic EDAC defs
//
// Author: Dave Jiang <djiang@mvista.com>
//
// 2006-2008 (c) MontaVista Software, Inc. This file is licensed under
// the terms of the GNU General Public License version 2. This program
// is licensed "as is" without any warranty of any kind, whether express
// or implied.
//

pub const EDAC_DEVICE_NAME_LEN: c_int = 31;

pub const EDAC_OPSTATE_POLL: c_int = 0;
pub const EDAC_OPSTATE_NMI: c_int = 1;
pub const EDAC_OPSTATE_INT: c_int = 2;
// Max length of a DIMM label
pub const EDAC_MC_LABEL_LEN: c_int = 31;
// Maximum size of the location string
pub const LOCATION_SIZE: c_int = 256;
// Defines the maximum number of labels that can be reported
pub const EDAC_MAX_LABELS: c_int = 8;
// String used to join two or more labels

//
// enum dev_type - describe the type of memory DRAM chips used at the stick
// @DEV_UNKNOWN:	Can't be determined, or MC doesn't support detect it
// @DEV_X1:		1 bit for data
// @DEV_X2:		2 bits for data
// @DEV_X4:		4 bits for data
// @DEV_X8:		8 bits for data
// @DEV_X16:		16 bits for data
// @DEV_X32:		32 bits for data
// @DEV_X64:		64 bits for data
//
// Typical values are x4 and x8.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dev_type {
    DEV_UNKNOWN = 0,
    DEV_X1,
    DEV_X2,
    DEV_X4,
    DEV_X8,
    DEV_X16,
    DEV_X32,		/* Do these parts exist? */
    DEV_X64			/* Do these parts exist? */
}

//
// enum hw_event_mc_err_type - type of the detected error
//
// @HW_EVENT_ERR_CORRECTED:	Corrected Error - Indicates that an ECC
// corrected error was detected
// @HW_EVENT_ERR_UNCORRECTED:	Uncorrected Error - Indicates an error that
// can't be corrected by ECC, but it is not
// fatal (maybe it is on an unused memory area,
// or the memory controller could recover from
// it for example, by re-trying the operation).
// @HW_EVENT_ERR_DEFERRED:	Deferred Error - Indicates an uncorrectable
// error whose handling is not urgent. This could
// be due to hardware data poisoning where the
// system can continue operation until the poisoned
// data is consumed. Preemptive measures may also
// be taken, e.g. offlining pages, etc.
// @HW_EVENT_ERR_FATAL:		Fatal Error - Uncorrected error that could not
// be recovered.
// @HW_EVENT_ERR_INFO:		Informational - The CPER spec defines a forth
// type of error: informational logs.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hw_event_mc_err_type {
    HW_EVENT_ERR_CORRECTED,
    HW_EVENT_ERR_UNCORRECTED,
    HW_EVENT_ERR_DEFERRED,
    HW_EVENT_ERR_FATAL,
    HW_EVENT_ERR_INFO,
}

//
// enum mem_type - memory types. For a more detailed reference, please see
// http://en.wikipedia.org/wiki/DRAM
//
// @MEM_EMPTY:		Empty csrow
// @MEM_RESERVED:	Reserved csrow type
// @MEM_UNKNOWN:	Unknown csrow type
// @MEM_FPM:		FPM - Fast Page Mode, used on systems up to 1995.
// @MEM_EDO:		EDO - Extended data out, used on systems up to 1998.
// @MEM_BEDO:		BEDO - Burst Extended data out, an EDO variant.
// @MEM_SDR:		SDR - Single data rate SDRAM
// http://en.wikipedia.org/wiki/Synchronous_dynamic_random-access_memory
// They use 3 pins for chip select: Pins 0 and 2 are
// for rank 0; pins 1 and 3 are for rank 1, if the memory
// is dual-rank.
// @MEM_RDR:		Registered SDR SDRAM
// @MEM_DDR:		Double data rate SDRAM
// http://en.wikipedia.org/wiki/DDR_SDRAM
// @MEM_RDDR:		Registered Double data rate SDRAM
// This is a variant of the DDR memories.
// A registered memory has a buffer inside it, hiding
// part of the memory details to the memory controller.
// @MEM_RMBS:		Rambus DRAM, used on a few Pentium III/IV controllers.
// @MEM_DDR2:		DDR2 RAM, as described at JEDEC JESD79-2F.
// Those memories are labeled as "PC2-" instead of "PC" to
// differentiate from DDR.
// @MEM_FB_DDR2:	Fully-Buffered DDR2, as described at JEDEC Std No. 205
// and JESD206.
// Those memories are accessed per DIMM slot, and not by
// a chip select signal.
// @MEM_RDDR2:		Registered DDR2 RAM
// This is a variant of the DDR2 memories.
// @MEM_XDR:		Rambus XDR
// It is an evolution of the original RAMBUS memories,
// created to compete with DDR2. Weren't used on any
// x86 arch, but cell_edac PPC memory controller uses it.
// @MEM_DDR3:		DDR3 RAM
// @MEM_RDDR3:		Registered DDR3 RAM
// This is a variant of the DDR3 memories.
// @MEM_LRDDR3:		Load-Reduced DDR3 memory.
// @MEM_LPDDR3:		Low-Power DDR3 memory.
// @MEM_DDR4:		Unbuffered DDR4 RAM
// @MEM_RDDR4:		Registered DDR4 RAM
// This is a variant of the DDR4 memories.
// @MEM_LRDDR4:		Load-Reduced DDR4 memory.
// @MEM_LPDDR4:		Low-Power DDR4 memory.
// @MEM_DDR5:		Unbuffered DDR5 RAM
// @MEM_RDDR5:		Registered DDR5 RAM
// @MEM_LRDDR5:		Load-Reduced DDR5 memory.
// @MEM_LPDDR5:		Low-Power DDR5 memory.
// @MEM_NVDIMM:		Non-volatile RAM
// @MEM_WIO2:		Wide I/O 2.
// @MEM_HBM2:		High bandwidth Memory Gen 2.
// @MEM_HBM3:		High bandwidth Memory Gen 3.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mem_type {
    MEM_EMPTY = 0,
    MEM_RESERVED,
    MEM_UNKNOWN,
    MEM_FPM,
    MEM_EDO,
    MEM_BEDO,
    MEM_SDR,
    MEM_RDR,
    MEM_DDR,
    MEM_RDDR,
    MEM_RMBS,
    MEM_DDR2,
    MEM_FB_DDR2,
    MEM_RDDR2,
    MEM_XDR,
    MEM_DDR3,
    MEM_RDDR3,
    MEM_LRDDR3,
    MEM_LPDDR3,
    MEM_DDR4,
    MEM_RDDR4,
    MEM_LRDDR4,
    MEM_LPDDR4,
    MEM_DDR5,
    MEM_RDDR5,
    MEM_LRDDR5,
    MEM_LPDDR5,
    MEM_NVDIMM,
    MEM_WIO2,
    MEM_HBM2,
    MEM_HBM3,
}

//
// enum edac_type - Error Detection and Correction capabilities and mode
// @EDAC_UNKNOWN:	Unknown if ECC is available
// @EDAC_NONE:		Doesn't support ECC
// @EDAC_RESERVED:	Reserved ECC type
// @EDAC_PARITY:	Detects parity errors
// @EDAC_EC:		Error Checking - no correction
// @EDAC_SECDED:	Single bit error correction, Double detection
// @EDAC_S2ECD2ED:	Chipkill x2 devices - do these exist?
// @EDAC_S4ECD4ED:	Chipkill x4 devices
// @EDAC_S8ECD8ED:	Chipkill x8 devices
// @EDAC_S16ECD16ED:	Chipkill x16 devices
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum edac_type {
    EDAC_UNKNOWN =	0,
    EDAC_NONE,
    EDAC_RESERVED,
    EDAC_PARITY,
    EDAC_EC,
    EDAC_SECDED,
    EDAC_S2ECD2ED,
    EDAC_S4ECD4ED,
    EDAC_S8ECD8ED,
    EDAC_S16ECD16ED,
}

//
// enum scrub_type - scrubbing capabilities
// @SCRUB_UNKNOWN:		Unknown if scrubber is available
// @SCRUB_NONE:			No scrubber
// @SCRUB_SW_PROG:		SW progressive (sequential) scrubbing
// @SCRUB_SW_SRC:		Software scrub only errors
// @SCRUB_SW_PROG_SRC:		Progressive software scrub from an error
// @SCRUB_SW_TUNABLE:		Software scrub frequency is tunable
// @SCRUB_HW_PROG:		HW progressive (sequential) scrubbing
// @SCRUB_HW_SRC:		Hardware scrub only errors
// @SCRUB_HW_PROG_SRC:		Progressive hardware scrub from an error
// @SCRUB_HW_TUNABLE:		Hardware scrub frequency is tunable
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scrub_type {
    SCRUB_UNKNOWN =	0,
    SCRUB_NONE,
    SCRUB_SW_PROG,
    SCRUB_SW_SRC,
    SCRUB_SW_PROG_SRC,
    SCRUB_SW_TUNABLE,
    SCRUB_HW_PROG,
    SCRUB_HW_SRC,
    SCRUB_HW_PROG_SRC,
    SCRUB_HW_TUNABLE
}

// FIXME - should have notify capabilities: NMI, LOG, PROC, etc
// EDAC internal operation states
pub const OP_ALLOC: c_uint = 0x100;
pub const OP_RUNNING_POLL: c_uint = 0x201;
pub const OP_RUNNING_INTERRUPT: c_uint = 0x202;
pub const OP_RUNNING_POLL_INTR: c_uint = 0x203;
pub const OP_OFFLINE: c_uint = 0x300;
//
// enum edac_mc_layer_type - memory controller hierarchy layer
//
// @EDAC_MC_LAYER_BRANCH:	memory layer is named "branch"
// @EDAC_MC_LAYER_CHANNEL:	memory layer is named "channel"
// @EDAC_MC_LAYER_SLOT:		memory layer is named "slot"
// @EDAC_MC_LAYER_CHIP_SELECT:	memory layer is named "chip select"
// @EDAC_MC_LAYER_ALL_MEM:	memory layout is unknown. All memory is mapped
// as a single memory area. This is used when
// retrieving errors from a firmware driven driver.
//
// This enum is used by the drivers to tell edac_mc_sysfs what name should
// be used when describing a memory stick location.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum edac_mc_layer_type {
    EDAC_MC_LAYER_BRANCH,
    EDAC_MC_LAYER_CHANNEL,
    EDAC_MC_LAYER_SLOT,
    EDAC_MC_LAYER_CHIP_SELECT,
    EDAC_MC_LAYER_ALL_MEM,
}

//
// struct edac_mc_layer - describes the memory controller hierarchy
// @type:		layer type
// @size:		number of components per layer. For example,
// if the channel layer has two channels, size = 2
// @is_virt_csrow:	This layer is part of the "csrow" when old API
// compatibility mode is enabled. Otherwise, it is
// a channel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edac_mc_layer {
    pub type: edac_mc_layer_type,
    pub size: unsigned,
    pub is_virt_csrow: bool,
}

//
// Maximum number of layers used by the memory controller to uniquely
// identify a single memory stick.
// NOTE: Changing this constant requires not only to change the constant
// below, but also to change the existing code at the core, as there are
// some code there that are optimized for 3 layers.
//
pub const EDAC_MAX_LAYERS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dimm_info {
    pub dev: device,
    pub /: *mut *mut char label[EDAC_MC_LABEL_LEN + 1]; / DIMM label on motherboard,
// Memory location data
    pub location: [c_uint; EDAC_MAX_LAYERS],
    pub /: *mut *mut *mut mem_ctl_info mci; / the parent,
    pub /: *mut *mut unsigned int idx; / index within the parent dimm array,
    pub /: *mut *mut u32 grain; / granularity of reported error in bytes,
    pub /: *mut *mut dev_type dtype; / memory device type,
    pub /: *mut *mut mem_type mtype; / memory dimm type,
    pub /: *mut *mut edac_type edac_mode; / EDAC mode for this dimm,
    pub /: *mut *mut u32 nr_pages; / number of pages on this dimm,
    pub /: *mut *mut unsigned int csrow, cschannel; / Points to the old API data,
    pub /: *mut *mut u16 smbios_handle; / Handle for SMBIOS type 17,
    pub ce_count: u32,
    pub ue_count: u32,
}

//
// struct rank_info - contains the information for one DIMM rank
//
// @chan_idx:	channel number where the rank is (typically, 0 or 1)
// @ce_count:	number of correctable errors for this rank
// @csrow:	A pointer to the chip select row structure (the parent
// structure). The location of the rank is given by
// the (csrow->csrow_idx, chan_idx) vector.
// @dimm:	A pointer to the DIMM structure, where the DIMM label
// information is stored.
//
// FIXME: Currently, the EDAC core model will assume one DIMM per rank.
// This is a bad assumption, but it makes this patch easier. Later
// patches in this series will fix this issue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rank_info {
    pub chan_idx: c_int,
    pub csrow: *mut csrow_info,
    pub dimm: *mut dimm_info,
    pub /: *mut *mut u32 ce_count; / Correctable Errors for this csrow,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csrow_info {
    pub dev: device,
// Used only by edac_mc_find_csrow_by_page()
    pub /: *mut *mut unsigned long first_page; / first page number in csrow,
    pub /: *mut *mut unsigned long last_page; / last page number in csrow,
    pub -: *mut *mut unsigned long page_mask; / used for interleaving,
// 0UL for non intlv
    pub /: *mut *mut int csrow_idx; / the chip-select row,
    pub /: *mut *mut u32 ue_count; / Uncorrectable Errors for this csrow,
    pub /: *mut *mut u32 ce_count; / Correctable Errors for this csrow,
    pub /: *mut *mut *mut mem_ctl_info mci; / the parent,
// channel information for this csrow
    pub nr_channels: u32,
    pub channels: *mut rank_info,
}

//
// struct errcount_attribute - used to store the several error counts
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct errcount_attribute_data {
    pub n_layers: c_int,
    pub pos: [c_int; EDAC_MAX_LAYERS],
    pub layer2: int layer0, layer1,,
}

//
// struct edac_raw_error_desc - Raw error report structure
// @grain:			minimum granularity for an error report, in bytes
// @error_count:		number of errors of the same type
// @type:			severity of the error (CE/UE/Fatal)
// @top_layer:			top layer of the error (layer[0])
// @mid_layer:			middle layer of the error (layer[1])
// @low_layer:			low layer of the error (layer[2])
// @page_frame_number:		page where the error happened
// @offset_in_page:		page offset
// @syndrome:			syndrome of the error (or 0 if unknown or if
// the syndrome is not applicable)
// @msg:			error message
// @location:			location of the error
// @label:			label of the affected DIMM(s)
// @other_detail:		other driver-specific detail about the error
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edac_raw_error_desc {
    pub location: [c_char; LOCATION_SIZE],
    pub EDAC_MAX_LABELS]: *mut *mut char label[(EDAC_MC_LABEL_LEN + 1 + sizeof(OTHER_LABEL)),
    pub grain: c_long,
    pub error_count: u16,
    pub type: hw_event_mc_err_type,
    pub top_layer: c_int,
    pub mid_layer: c_int,
    pub low_layer: c_int,
    pub page_frame_number: c_ulong,
    pub offset_in_page: c_ulong,
    pub syndrome: c_ulong,
    pub msg: *const c_char,
    pub other_detail: *const c_char,
}

// MEMORY controller information structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mem_ctl_info {
    pub dev: device,
    pub bus: *const bus_type,
    pub /: *mut *mut list_head link; / for global list of mem_ctl_info structs,
    pub /: *mut *mut *mut module owner; / Module owner of this control struct,
    pub /: *mut *mut unsigned long mtype_cap; / memory types supported by mc,
    pub /: *mut *mut unsigned long edac_ctl_cap; / Mem controller EDAC capabilities,
    pub is: *mut *mut unsigned long edac_cap; / configuration capabilities - this,
// closely related to edac_ctl_cap.  The
// difference is that the controller may be
// capable of s4ecd4ed which would be listed
// in edac_ctl_cap, but if channels aren't
// capable of s4ecd4ed then the edac_cap would
// not have that capability.
//
    pub /: *mut *mut unsigned long scrub_cap; / chipset scrub capabilities,
    pub /: *mut *mut scrub_type scrub_mode; / current scrub mode,
// Translates sdram memory scrub rate given in bytes/sec to the
//
    pub bw): *mut *mut *mut int (set_sdram_scrub_rate) (struct mem_ctl_info  mci, u32,
// Get the current sdram memory scrub rate from the internal
//
    pub mci): *mut *mut *mut int (get_sdram_scrub_rate) (struct mem_ctl_info,
// pointer to edac checking routine
    pub mci): *mut *mut *mut void (edac_check) (struct mem_ctl_info,
//
// Remaps memory pages: controller pages to physical pages.
// For most MC's, this will be NULL.
//
// FIXME - why not send the phys page to begin with?
    pub page): c_ulong,
    pub mc_idx: c_int,
    pub csrows: *mut csrow_info,
    pub num_cschannel: unsigned int nr_csrows,,
    pub csbased: bool,
//
// DIMM info. Will eventually remove the entire csrows_info some day
//
    pub tot_dimms: c_uint,
    pub dimms: *mut dimm_info,
//
// FIXME - what about controllers on other busses? - IDs must be
// unique.  dev pointer should be sufficiently unique, but
// BUS:SLOT.FUNC numbers may not be unique.
//
    pub pdev: *mut device,
    pub mod_name: *const c_char,
    pub ctl_name: *const c_char,
    pub dev_name: *const c_char,
    pub pvt_info: *mut c_void,
    pub /: *mut *mut unsigned long start_time; / mci load start time (in jiffies),
//
// drivers shouldn't access those fields directly, as the core
// already handles that.
//
    pub ue_noinfo_count: u32 ce_noinfo_count,,
    pub ce_mc: u32 ue_mc,,
    pub complete: completion,
// Additional top controller level attributes, but specified
// by the low level driver.
//
// Set by the low level driver to provide attributes at the
// controller level.
// An array of structures, NULL terminated
//
// If attributes are desired, then set to array of attributes
// If no attributes are desired, leave NULL
//
    pub mc_driver_sysfs_attributes: *const mcidev_sysfs_attribute,
// work struct for this MC
    pub work: delayed_work,
//
// Used to report an error - by being at the global struct
// makes the memory allocated by the EDAC core
//
    pub error_desc: edac_raw_error_desc,
// the internal state of this controller instance
    pub op_state: c_int,
    pub debugfs: *mut dentry,
//
// Memory Controller hierarchy
//
// There are basically two types of memory controller: the ones that
// sees memory sticks ("dimms"), and the ones that sees memory ranks.
// All old memory controllers enumerate memories per rank, but most
// of the recent drivers enumerate memories per DIMM, instead.
// When the memory controller is per rank, csbased is true.
//
    pub n_layers: c_uint,
    pub __counted_by(n_layers): edac_mc_layer layers[],
}

//
// edac_get_dimm - Get DIMM info from a memory controller given by
// [layer0,layer1,layer2] position
//
// @mci:	MC descriptor struct mem_ctl_info
// @layer0:	layer0 position
// @layer1:	layer1 position. Unused if n_layers < 2
// @layer2:	layer2 position. Unused if n_layers < 3
//
// For 1 layer, this function returns "dimms[layer0]";
//
// For 2 layers, this function is similar to allocating a two-dimensional
// array and returning "dimms[layer0][layer1]";
//
// For 3 layers, this function is similar to allocating a tri-dimensional
// array and returning "dimms[layer0][layer1][layer2]";
//
pub const EDAC_FEAT_NAME_LEN: c_int = 128;
// RAS feature type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum edac_dev_feat {
    RAS_FEAT_SCRUB,
    RAS_FEAT_ECS,
    RAS_FEAT_MEM_REPAIR,
    RAS_FEAT_MAX
}

//
// struct edac_scrub_ops - scrub device operations (all elements optional)
// @read_addr: read base address of scrubbing range.
// @read_size: read offset of scrubbing range.
// @write_addr: set base address of the scrubbing range.
// @write_size: set offset of the scrubbing range.
// @get_enabled_bg: check if currently performing background scrub.
// @set_enabled_bg: start or stop a bg-scrub.
// @get_min_cycle: get minimum supported scrub cycle duration in seconds.
// @get_max_cycle: get maximum supported scrub cycle duration in seconds.
// @get_cycle_duration: get current scrub cycle duration in seconds.
// @set_cycle_duration: set current scrub cycle duration in seconds.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edac_scrub_ops {
    pub base): *mut *mut *mut *mut int (read_addr)(struct device dev, void drv_data, u64,
    pub size): *mut *mut *mut *mut int (read_size)(struct device dev, void drv_data, u64,
    pub base): *mut *mut *mut *mut int (write_addr)(struct device dev, void drv_data, u64,
    pub size): *mut *mut *mut *mut int (write_size)(struct device dev, void drv_data, u64,
    pub enable): *mut *mut *mut *mut int (get_enabled_bg)(struct device dev, void drv_data, bool,
    pub enable): *mut *mut *mut *mut int (set_enabled_bg)(struct device dev, void drv_data, bool,
    pub min): *mut *mut *mut *mut int (get_min_cycle)(struct device dev, void drv_data, u32,
    pub max): *mut *mut *mut *mut int (get_max_cycle)(struct device dev, void drv_data, u32,
    pub cycle): *mut *mut *mut *mut int (get_cycle_duration)(struct device dev, void drv_data, u32,
    pub cycle): *mut *mut *mut *mut int (set_cycle_duration)(struct device dev, void drv_data, u32,
}

//
// struct edac_ecs_ops - ECS device operations (all elements optional)
// @get_log_entry_type: read the log entry type value.
// @set_log_entry_type: set the log entry type value.
// @get_mode: read the mode value.
// @set_mode: set the mode value.
// @reset: reset the ECS counter.
// @get_threshold: read the threshold count per gigabits of memory cells.
// @set_threshold: set the threshold count per gigabits of memory cells.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edac_ecs_ops {
    pub val): *mut *mut *mut *mut int (get_log_entry_type)(struct device dev, void drv_data, int fru_id, u32,
    pub val): *mut *mut *mut *mut int (set_log_entry_type)(struct device dev, void drv_data, int fru_id, u32,
    pub val): *mut *mut *mut *mut int (get_mode)(struct device dev, void drv_data, int fru_id, u32,
    pub val): *mut *mut *mut *mut int (set_mode)(struct device dev, void drv_data, int fru_id, u32,
    pub val): *mut *mut *mut *mut int (reset)(struct device dev, void drv_data, int fru_id, u32,
    pub threshold): *mut *mut *mut *mut int (get_threshold)(struct device dev, void drv_data, int fru_id, u32,
    pub threshold): *mut *mut *mut *mut int (set_threshold)(struct device dev, void drv_data, int fru_id, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct edac_ecs_ex_info {
    pub num_media_frus: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum edac_mem_repair_type {
    EDAC_REPAIR_PPR,
    EDAC_REPAIR_CACHELINE_SPARING,
    EDAC_REPAIR_ROW_SPARING,
    EDAC_REPAIR_BANK_SPARING,
    EDAC_REPAIR_RANK_SPARING,
    EDAC_REPAIR_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum edac_mem_repair_cmd {
    EDAC_DO_MEM_REPAIR = 1,
}

//
// struct edac_mem_repair_ops - memory repair operations
// (all elements are optional except do_repair, set_hpa/set_dpa)
// @get_repair_type: get the memory repair type, listed in
// enum edac_mem_repair_function.
// @get_persist_mode: get the current persist mode.
// false - Soft repair type (temporary repair).
// true - Hard memory repair type (permanent repair).
// @set_persist_mode: set the persist mode of the memory repair instance.
// @get_repair_safe_when_in_use: get whether memory media is accessible and
// data is retained during repair operation.
// @get_hpa: get current host physical address (HPA) of memory to repair.
// @set_hpa: set host physical address (HPA) of memory to repair.
// @get_min_hpa: get the minimum supported host physical address (HPA).
// @get_max_hpa: get the maximum supported host physical address (HPA).
// @get_dpa: get current device physical address (DPA) of memory to repair.
// @set_dpa: set device physical address (DPA) of memory to repair.
// In some states of system configuration (e.g. before address decoders
// have been configured), memory devices (e.g. CXL) may not have an active
// mapping in the host physical address map. As such, the memory
// to repair must be identified by a device specific physical addressing
// scheme using a device physical address(DPA). The DPA and other control
// attributes to use for the repair operations will be presented in related
// error records.
// @get_min_dpa: get the minimum supported device physical address (DPA).
// @get_max_dpa: get the maximum supported device physical address (DPA).
// @get_nibble_mask: get current nibble mask of memory to repair.
// @set_nibble_mask: set nibble mask of memory to repair.
// @get_bank_group: get current bank group of memory to repair.
// @set_bank_group: set bank group of memory to repair.
// @get_bank: get current bank of memory to repair.
// @set_bank: set bank of memory to repair.
// @get_rank: get current rank of memory to repair.
// @set_rank: set rank of memory to repair.
// @get_row: get current row of memory to repair.
// @set_row: set row of memory to repair.
// @get_column: get current column of memory to repair.
// @set_column: set column of memory to repair.
// @get_channel: get current channel of memory to repair.
// @set_channel: set channel of memory to repair.
// @get_sub_channel: get current subchannel of memory to repair.
// @set_sub_channel: set subchannel of memory to repair.
// @do_repair: Issue memory repair operation for the HPA/DPA and
// other control attributes set for the memory to repair.
//
// All elements are optional except do_repair and at least one of set_hpa/set_dpa.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edac_mem_repair_ops {
    pub type): *const *const *const *const int (get_repair_type)(struct device dev, void drv_data, char,
    pub persist): *mut *mut *mut *mut int (get_persist_mode)(struct device dev, void drv_data, bool,
    pub persist): *mut *mut *mut *mut int (set_persist_mode)(struct device dev, void drv_data, bool,
    pub safe): *mut *mut *mut *mut int (get_repair_safe_when_in_use)(struct device dev, void drv_data, bool,
    pub hpa): *mut *mut *mut *mut int (get_hpa)(struct device dev, void drv_data, u64,
    pub hpa): *mut *mut *mut *mut int (set_hpa)(struct device dev, void drv_data, u64,
    pub hpa): *mut *mut *mut *mut int (get_min_hpa)(struct device dev, void drv_data, u64,
    pub hpa): *mut *mut *mut *mut int (get_max_hpa)(struct device dev, void drv_data, u64,
    pub dpa): *mut *mut *mut *mut int (get_dpa)(struct device dev, void drv_data, u64,
    pub dpa): *mut *mut *mut *mut int (set_dpa)(struct device dev, void drv_data, u64,
    pub dpa): *mut *mut *mut *mut int (get_min_dpa)(struct device dev, void drv_data, u64,
    pub dpa): *mut *mut *mut *mut int (get_max_dpa)(struct device dev, void drv_data, u64,
    pub val): *mut *mut *mut *mut int (get_nibble_mask)(struct device dev, void drv_data, u32,
    pub val): *mut *mut *mut *mut int (set_nibble_mask)(struct device dev, void drv_data, u32,
    pub val): *mut *mut *mut *mut int (get_bank_group)(struct device dev, void drv_data, u32,
    pub val): *mut *mut *mut *mut int (set_bank_group)(struct device dev, void drv_data, u32,
    pub val): *mut *mut *mut *mut int (get_bank)(struct device dev, void drv_data, u32,
    pub val): *mut *mut *mut *mut int (set_bank)(struct device dev, void drv_data, u32,
    pub val): *mut *mut *mut *mut int (get_rank)(struct device dev, void drv_data, u32,
    pub val): *mut *mut *mut *mut int (set_rank)(struct device dev, void drv_data, u32,
    pub val): *mut *mut *mut *mut int (get_row)(struct device dev, void drv_data, u32,
    pub val): *mut *mut *mut *mut int (set_row)(struct device dev, void drv_data, u32,
    pub val): *mut *mut *mut *mut int (get_column)(struct device dev, void drv_data, u32,
    pub val): *mut *mut *mut *mut int (set_column)(struct device dev, void drv_data, u32,
    pub val): *mut *mut *mut *mut int (get_channel)(struct device dev, void drv_data, u32,
    pub val): *mut *mut *mut *mut int (set_channel)(struct device dev, void drv_data, u32,
    pub val): *mut *mut *mut *mut int (get_sub_channel)(struct device dev, void drv_data, u32,
    pub val): *mut *mut *mut *mut int (set_sub_channel)(struct device dev, void drv_data, u32,
    pub val): *mut *mut *mut *mut int (do_repair)(struct device dev, void drv_data, u32,
}

// EDAC device feature information structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edac_dev_data {
    pub scrub_ops: *const edac_scrub_ops,
    pub ecs_ops: *const edac_ecs_ops,
    pub mem_repair_ops: *const edac_mem_repair_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct edac_dev_feat_ctx {
    pub dev: device,
    pub private: *mut c_void,
    pub scrub: *mut edac_dev_data,
    pub ecs: edac_dev_data,
    pub mem_repair: *mut edac_dev_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct edac_dev_feature {
    pub ft_type: edac_dev_feat,
    pub instance: u8,
    pub scrub_ops: *const edac_scrub_ops,
    pub ecs_ops: *const edac_ecs_ops,
    pub mem_repair_ops: *const edac_mem_repair_ops,
}
