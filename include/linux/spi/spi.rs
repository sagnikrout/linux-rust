//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/spi/spi.h
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
// Copyright (C) 2005 David Brownell
//

// Max no. of CS supported per spi device
pub const SPI_DEVICE_CS_CNT_MAX: c_int = 4;
// Max no. of data lanes supported per spi device
pub const SPI_DEVICE_DATA_LANE_CNT_MAX: c_int = 8;
//
// INTERFACES between SPI controller-side drivers and SPI target protocol handlers,
// and SPI infrastructure.
//
// struct spi_statistics - statistics for spi transfers
// @syncp:         seqcount to protect members in this struct for per-cpu update
// on 32-bit systems
//
// @messages:      number of spi-messages handled
// @transfers:     number of spi_transfers handled
// @errors:        number of errors during spi_transfer
// @timedout:      number of timeouts during spi_transfer
//
// @spi_sync:      number of times spi_sync is used
// @spi_sync_immediate:
// number of times spi_sync is executed immediately
// in calling context without queuing and scheduling
// @spi_async:     number of times spi_async is used
//
// @bytes:         number of bytes transferred to/from device
// @bytes_tx:      number of bytes sent to device
// @bytes_rx:      number of bytes received from device
//
// @transfer_bytes_histo:
// transfer bytes histogram
//
// @transfers_split_maxsize:
// number of transfers that have been split because of
// maxsize limit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_statistics {
    pub syncp: u64_stats_sync,
    pub messages: u64_stats_t,
    pub transfers: u64_stats_t,
    pub errors: u64_stats_t,
    pub timedout: u64_stats_t,
    pub spi_sync: u64_stats_t,
    pub spi_sync_immediate: u64_stats_t,
    pub spi_async: u64_stats_t,
    pub bytes: u64_stats_t,
    pub bytes_rx: u64_stats_t,
    pub bytes_tx: u64_stats_t,
pub const SPI_STATISTICS_HISTO_SIZE: c_int = 17;
    pub transfer_bytes_histo: [u64_stats_t; SPI_STATISTICS_HISTO_SIZE],
    pub transfers_split_maxsize: u64_stats_t,
}

//
// struct spi_delay - SPI delay information
// @value: Value for the delay
// @unit: Unit for the delay
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_delay {
pub const SPI_DELAY_UNIT_USECS: c_int = 0;
pub const SPI_DELAY_UNIT_NSECS: c_int = 1;
pub const SPI_DELAY_UNIT_SCK: c_int = 2;
    pub value: u16,
    pub unit: u8,
}

extern "C" {
    pub fn spi_delay_to_ns(_delay: *mut spi_delay, xfer: *mut spi_transfer) -> c_int;
}
extern "C" {
    pub fn spi_delay_exec(_delay: *mut spi_delay, xfer: *mut spi_transfer) -> c_int;
}
//
// struct spi_device - Controller side proxy for an SPI target device
// @dev: Driver model representation of the device.
// @controller: SPI controller used with the device.
// @max_speed_hz: Maximum clock rate to be used with this chip
// (on this board); may be changed by the device's driver.
// The spi_transfer.speed_hz can override this for each transfer.
// @bits_per_word: Data transfers involve one or more words; word sizes
// like eight or 12 bits are common.  In-memory wordsizes are
// powers of two bytes (e.g. 20 bit samples use 32 bits).
// This may be changed by the device's driver, or left at the
// default (0) indicating protocol words are eight bit bytes.
// The spi_transfer.bits_per_word can override this for each transfer.
// @rt: Make the pump thread real time priority.
// @mode: The spi mode defines how data is clocked out and in.
// This may be changed by the device's driver.
// The "active low" default for chipselect mode can be overridden
// (by specifying SPI_CS_HIGH) as can the "MSB first" default for
// each word in a transfer (by specifying SPI_LSB_FIRST).
// @irq: Negative, or the number passed to request_irq() to receive
// interrupts from this device.
// @controller_state: Controller's runtime state
// @controller_data: Board-specific definitions for controller, such as
// FIFO initialization parameters; from board_info.controller_data
// @modalias: Name of the driver to use with this device, or an alias
// for that name.  This appears in the sysfs "modalias" attribute
// for driver coldplugging, and in uevents used for hotplugging
// @pcpu_statistics: statistics for the spi_device
// @word_delay: delay to be inserted between consecutive
// words of a transfer
// @cs_setup: delay to be introduced by the controller after CS is asserted
// @cs_hold: delay to be introduced by the controller before CS is deasserted
// @cs_inactive: delay to be introduced by the controller after CS is
// deasserted. If @cs_change_delay is used from @spi_transfer, then the
// two delays will be added up.
// @chip_select: Array of physical chipselect, spi->chipselect[i] gives
// the corresponding physical CS for logical CS i.
// @num_chipselect: Number of physical chipselects used.
// @cs_index_mask: Bit mask of the active chipselect(s) in the chipselect array
// @cs_gpiod: Array of GPIO descriptors of the corresponding chipselect lines
// (optional, NULL when not using a GPIO line)
// @tx_lane_map: Map of peripheral lanes (index) to controller lanes (value).
// @num_tx_lanes: Number of transmit lanes wired up.
// @rx_lane_map: Map of peripheral lanes (index) to controller lanes (value).
// @num_rx_lanes: Number of receive lanes wired up.
// @userspace_node: entry on the parent controller's userspace_clients list
// when this device was instantiated via the sysfs new_device interface
//
// A @spi_device is used to interchange data between an SPI target device
// (usually a discrete chip) and CPU memory.
//
// In @dev, the platform_data is used to hold information about this
// device that's meaningful to the device's protocol driver, but not
// to its controller.  One example might be an identifier for a chip
// variant with slightly different functionality; another might be
// information about how this particular board wires the chip's pins.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_device {
    pub dev: device,
    pub controller: *mut spi_controller,
    pub max_speed_hz: u32,
    pub bits_per_word: u8,
    pub rt: bool,

//
// TPM specification defines flow control over SPI. Client device
// can insert a wait state on MISO when address is transmitted by
// controller on MOSI. Detecting the wait state in software is only
// possible for full duplex controllers. For controllers that support
// only half-duplex, the wait state detection needs to be implemented
// in hardware. TPM devices would set this flag when hardware flow
// control is expected from SPI controller.
//

//
// All bits defined above should be covered by SPI_MODE_KERNEL_MASK.
// The SPI_MODE_KERNEL_MASK has the SPI_MODE_USER_MASK counterpart,
// which is defined in 'include/uapi/linux/spi/spi.h'.
// The bits defined here are from bit 31 downwards, while in
// SPI_MODE_USER_MASK are from 0 upwards.
// These bits must not overlap. A static assert check should make sure of that.
// If adding extra bits, make sure to decrease the bit index below as well.
//

    pub mode: u32,
    pub irq: c_int,
    pub controller_state: *mut c_void,
    pub controller_data: *mut c_void,
    pub modalias: [c_char; SPI_NAME_SIZE],
// The statistics
    pub pcpu_statistics: *mut spi_statistics __percpu,
    pub /: *mut *mut spi_delay word_delay; / Inter-word delay,
// CS delays
    pub cs_setup: spi_delay,
    pub cs_hold: spi_delay,
    pub cs_inactive: spi_delay,
    pub chip_select: [u8; SPI_DEVICE_CS_CNT_MAX],
    pub num_chipselect: u8,
//
// Bit mask of the chipselect(s) that the driver need to use from
// the chipselect array. When the controller is capable to handle
// multiple chip selects & memories are connected in parallel
// then more than one bit need to be set in cs_index_mask.
//
    pub SPI_DEVICE_CS_CNT_MAX: u32 cs_index_mask :,
    pub /: *mut *mut *mut gpio_desc cs_gpiod[SPI_DEVICE_CS_CNT_MAX]; / Chip select gpio desc,
// Multi-lane SPI controller support.
    pub tx_lane_map: [u8; SPI_DEVICE_DATA_LANE_CNT_MAX],
    pub num_tx_lanes: u8,
    pub rx_lane_map: [u8; SPI_DEVICE_DATA_LANE_CNT_MAX],
    pub num_rx_lanes: u8,

    pub userspace_node: list_head,

//
// Likely need more hooks for more protocol options affecting how
// the controller talks to each chip, like:
// - memory packing (12 bit samples into low bits, others zeroed)
// - priority
// - chipselect delays
// - ...
//
}

// Make sure that SPI_MODE_KERNEL_MASK & SPI_MODE_USER_MASK don't overlap

// Most drivers won't need to care about device refcounting
// ctldata is for the bus_controller driver's runtime state
// Device driver data
extern "C" {
    pub fn dev_get_drvdata(_arg: &spi->dev) -> return;
}
//
// struct spi_driver - Host side "protocol" driver
// @id_table: List of SPI devices supported by this driver
// @probe: Binds this driver to the SPI device.  Drivers can verify
// that the device is actually present, and may need to configure
// characteristics (such as bits_per_word) which weren't needed for
// the initial configuration done during system setup.
// @remove: Unbinds this driver from the SPI device
// @shutdown: Standard shutdown callback used during system state
// transitions such as powerdown/halt and kexec
// @driver: SPI device drivers should initialize the name and owner
// field of this structure.
//
// This represents the kind of device driver that uses SPI messages to
// interact with the hardware at the other end of a SPI link.  It's called
// a "protocol" driver because it works through messages rather than talking
// directly to SPI hardware (which is what the underlying SPI controller
// driver does to pass those messages).  These protocols are defined in the
// specification for the device(s) supported by the driver.
//
// As a rule, those device protocols represent the lowest level interface
// supported by a driver, and it will support upper level interfaces too.
// Examples of such upper levels include frameworks like MTD, networking,
// MMC, RTC, filesystem character device nodes, and hardware monitoring.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_driver {
    pub id_table: *const spi_device_id,
    pub spi): *mut *mut int (probe)(struct spi_device,
    pub spi): *mut *mut void (remove)(struct spi_device,
    pub spi): *mut *mut void (shutdown)(struct spi_device,
    pub driver: device_driver,
}

extern "C" {
    pub fn __spi_register_driver(owner: *mut module, sdrv: *mut spi_driver) -> c_int;
}
//
// spi_unregister_driver - reverse effect of spi_register_driver
// @sdrv: the driver to unregister
// Context: can sleep
//
// Use a define to avoid include chaining to get THIS_MODULE

//
// module_spi_driver() - Helper macro for registering a SPI driver
// @__spi_driver: spi_driver struct
//
// Helper macro for SPI drivers which do not do anything special in module
// init/exit. This eliminates a lot of boilerplate. Each module may only
// use this macro once, and calling it replaces module_init() and module_exit()
//

//
// struct spi_controller - interface to SPI host or target controller
// @dev: device interface to this driver
// @list: link with the global spi_controller list
// @bus_num: board-specific (and often SOC-specific) identifier for a
// given SPI controller.
// @num_chipselect: chipselects are used to distinguish individual
// SPI targets, and are numbered from zero to num_chipselects.
// each target has a chipselect signal, but it's common that not
// every chipselect is connected to a target.
// @num_data_lanes: Number of data lanes supported by this controller. Default is 1.
// @dma_alignment: SPI controller constraint on DMA buffers alignment.
// @mode_bits: flags understood by this controller driver
// @buswidth_override_bits: flags to override for this controller driver
// @bits_per_word_mask: A mask indicating which values of bits_per_word are
// supported by the driver. Bit n indicates that a bits_per_word n+1 is
// supported. If set, the SPI core will reject any transfer with an
// unsupported bits_per_word. If not set, this value is simply ignored,
// and it's up to the individual driver to perform any validation.
// @min_speed_hz: Lowest supported transfer speed
// @max_speed_hz: Highest supported transfer speed
// @flags: other constraints relevant to this driver
// @slave: indicates that this is an SPI slave controller
// @target: indicates that this is an SPI target controller
// @max_transfer_size: function that returns the max transfer size for
// a &spi_device; may be %NULL, so the default %SIZE_MAX will be used.
// @max_message_size: function that returns the max message size for
// a &spi_device; may be %NULL, so the default %SIZE_MAX will be used.
// @io_mutex: mutex for physical bus access
// @add_lock: mutex to avoid adding devices to the same chipselect
// @bus_lock_spinlock: spinlock for SPI bus locking
// @bus_lock_mutex: mutex for exclusion of multiple callers
// @bus_lock_flag: indicates that the SPI bus is locked for exclusive use
// @setup: updates the device mode and clocking records used by a
// device's SPI controller; protocol code may call this.  This
// must fail if an unrecognized or unsupported mode is requested.
// It's always safe to call this unless transfers are pending on
// the device whose settings are being modified.
// @set_cs_timing: optional hook for SPI devices to request SPI
// controller for configuring specific CS setup time, hold time and inactive
// delay in terms of clock counts
// @transfer: adds a message to the controller's transfer queue.
// @cleanup: frees controller-specific state
// @can_dma: determine whether this controller supports DMA
// @dma_map_dev: device which can be used for DMA mapping
// @cur_rx_dma_dev: device which is currently used for RX DMA mapping
// @cur_tx_dma_dev: device which is currently used for TX DMA mapping
// @queued: whether this controller is providing an internal message queue
// @kworker: pointer to thread struct for message pump
// @pump_messages: work struct for scheduling work to the message pump
// @queue_lock: spinlock to synchronise access to message queue
// @queue: message queue
// @cur_msg: the currently in-flight message
// @cur_msg_completion: a completion for the current in-flight message
// @cur_msg_incomplete: Flag used internally to opportunistically skip
// the @cur_msg_completion. This flag is used to check if the driver has
// already called spi_finalize_current_message().
// @cur_msg_need_completion: Flag used internally to opportunistically skip
// the @cur_msg_completion. This flag is used to signal the context that
// is running spi_finalize_current_message() that it needs to complete()
// @fallback: fallback to PIO if DMA transfer return failure with
// SPI_TRANS_FAIL_NO_START.
// @last_cs_mode_high: was (mode & SPI_CS_HIGH) true on the last call to set_cs.
// @last_cs: the last chip_select that is recorded by set_cs, -1 on non chip
// selected
// @last_cs_index_mask: bit mask the last chip selects that were used
// @xfer_completion: used by core transfer_one_message()
// @busy: message pump is busy
// @running: message pump is running
// @rt: whether this queue is set to run as a realtime task
// @auto_runtime_pm: the core should ensure a runtime PM reference is held
// while the hardware is prepared, using the parent
// device for the spidev
// @max_dma_len: Maximum length of a DMA transfer for the device.
// @prepare_transfer_hardware: a message will soon arrive from the queue
// so the subsystem requests the driver to prepare the transfer hardware
// by issuing this call
// @transfer_one_message: the subsystem calls the driver to transfer a single
// message while queuing transfers that arrive in the meantime. When the
// driver is finished with this message, it must call
// spi_finalize_current_message() so the subsystem can issue the next
// message
// @unprepare_transfer_hardware: there are currently no more messages on the
// queue so the subsystem notifies the driver that it may relax the
// hardware by issuing this call
//
// @set_cs: set the logic level of the chip select line.  May be called
// from interrupt context.
// @optimize_message: optimize the message for reuse
// @unoptimize_message: release resources allocated by optimize_message
// @prepare_message: set up the controller to transfer a single message,
// for example doing DMA mapping.  Called from threaded
// context.
// @transfer_one: transfer a single spi_transfer.
//
// - return 0 if the transfer is finished,
// - return 1 if the transfer is still in progress. When
// the driver is finished with this transfer it must
// call spi_finalize_current_transfer() so the subsystem
// can issue the next transfer. If the transfer fails, the
// driver must set the flag SPI_TRANS_FAIL_IO to
// spi_transfer->error first, before calling
// spi_finalize_current_transfer().
// Note: transfer_one and transfer_one_message are mutually
// exclusive; when both are set, the generic subsystem does
// not call your transfer_one callback.
// @handle_err: the subsystem calls the driver to handle an error that occurs
// in the generic implementation of transfer_one_message().
// @mem_ops: optimized/dedicated operations for interactions with SPI memory.
// This field is optional and should only be implemented if the
// controller has native support for memory like operations.
// @get_offload: callback for controllers with offload support to get matching
// offload instance. Implementations should return -ENODEV if no match is
// found.
// @put_offload: release the offload instance acquired by @get_offload.
// @mem_caps: controller capabilities for the handling of memory operations.
// @dtr_caps: true if controller has dtr(single/dual transfer rate) capability.
// QSPI based controller should fill this based on controller's capability.
// @unprepare_message: undo any work done by prepare_message().
// @target_abort: abort the ongoing transfer request on an SPI target controller
// @cs_gpiods: Array of GPIO descriptors to use as chip select lines; one per CS
// number. Any individual value may be NULL for CS lines that
// are not GPIOs (driven by the SPI controller itself).
// @use_gpio_descriptors: Turns on the code in the SPI core to parse and grab
// GPIO descriptors. This will fill in @cs_gpiods and SPI devices will have
// the cs_gpiod assigned if a GPIO line is found for the chipselect.
// @unused_native_cs: When cs_gpiods is used, spi_register_controller() will
// fill in this field with the first unused native CS, to be used by SPI
// controller drivers that need to drive a native CS when using GPIO CS.
// @max_native_cs: When cs_gpiods is used, and this field is filled in,
// spi_register_controller() will validate all native CS (including the
// unused native CS) against this value.
// @pcpu_statistics: statistics for the spi_controller
// @dma_tx: DMA transmit channel
// @dma_rx: DMA receive channel
// @dummy_rx: dummy receive buffer for full-duplex devices
// @dummy_tx: dummy transmit buffer for full-duplex devices
// @fw_translate_cs: If the boot firmware uses different numbering scheme
// what Linux expects, this optional hook can be used to translate
// between the two.
// @ptp_sts_supported: If the driver sets this to true, it must provide a
// time snapshot in @spi_transfer->ptp_sts as close as possible to the
// moment in time when @spi_transfer->ptp_sts_word_pre and
// @spi_transfer->ptp_sts_word_post were transmitted.
// If the driver does not set this, the SPI core takes the snapshot as
// close to the driver hand-over as possible.
// @irq_flags: Interrupt enable state during PTP system timestamping
// @queue_empty: signal green light for opportunistically skipping the queue
// for spi_sync transfers.
// @must_async: disable all fast paths in the core
// @defer_optimize_message: set to true if controller cannot pre-optimize messages
// and needs to defer the optimization step until the message is actually
// being transferred
// @userspace_clients: list of SPI devices instantiated from userspace via
// the sysfs new_device interface; protected by @add_lock
// @userspace_registered: true once the new_device/delete_device sysfs
// group has been added by spi_register_controller(); used by
// spi_unregister_controller() to know whether to remove it
//
// Each SPI controller can communicate with one or more @spi_device
// children.  These make a small bus, sharing MOSI, MISO and SCK signals
// but not chip select signals.  Each device may be configured to use a
// different clock rate, since those shared signals are ignored unless
// the chip is selected.
//
// The driver for an SPI controller manages access to those devices through
// a queue of spi_message transactions, copying data between CPU memory and
// an SPI target device.  For each such message it queues, it calls the
// message's completion function when the transaction completes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_controller {
    pub dev: device,
    pub list: list_head,
//
// Other than negative (== assign one dynamically), bus_num is fully
// board-specific. Usually that simplifies to being SoC-specific.
// example: one SoC has three SPI controllers, numbered 0..2,
// and one board's schematics might show it using SPI-2. Software
// would normally use bus_num=2 for that controller.
//
    pub bus_num: i16,
//
// Chipselects will be integral to many controllers; some others
// might use board-specific GPIOs.
//
    pub num_chipselect: u16,
//
// Some specialized SPI controllers can have more than one physical
// data lane interface per controller (each having it's own serializer).
// This specifies the number of data lanes in that case. Other
// controllers do not need to set this (defaults to 1).
//
    pub num_data_lanes: u16,
// Some SPI controllers pose alignment requirements on DMAable
// buffers; let protocol drivers know about these requirements.
//
    pub dma_alignment: u16,
// spi_device.mode flags understood by this controller driver
    pub mode_bits: u32,
// spi_device.mode flags override flags for this controller
    pub buswidth_override_bits: u32,
// Bitmask of supported bits_per_word for transfers
    pub bits_per_word_mask: u32,

// Limits on transfer speed
    pub min_speed_hz: u32,
    pub max_speed_hz: u32,
// Other constraints relevant to this driver
    pub flags: u16,

//
// The spi-controller has multi chip select capability and can
// assert/de-assert more than one chip select at once.
//

// Flag indicating this is an SPI slave controller
    pub slave: bool,
// Flag indicating this is an SPI target controller
    pub target: bool,
}

//
// On some hardware transfer / message size may be constrained
// the limit may depend on device transfer settings.
//
// I/O mutex
// Used to avoid adding the same CS twice
// Lock and mutex for SPI bus locking
// Flag indicating that the SPI bus is locked for exclusive use
//
// Setup mode and clock, etc (SPI driver may call many times).
//
// IMPORTANT:  this may be called when transfers to another
// device are active.  DO NOT UPDATE SHARED REGISTERS in ways
// which could break those transfers.
//
// set_cs_timing() method is for SPI controllers that supports
// configuring CS timing.
//
// This hook allows SPI client drivers to request SPI controllers
// to configure specific CS timing through spi_set_cs_timing() after
// spi_setup().
//
// Bidirectional bulk transfers
//
// + The transfer() method may not sleep; its main role is
// just to add the message to the queue.
// + For now there's no remove-from-queue operation, or
// any other request management
// + To a given spi_device, message queueing is pure FIFO
//
// + The controller's main job is to process its message queue,
// selecting a chip (for controllers), then transferring data
// + If there are multiple spi_device children, the i/o queue
// arbitration algorithm is unspecified (round robin, FIFO,
// priority, reservations, preemption, etc)
//
// + Chipselect stays active during the entire message
// (unless modified by spi_transfer.cs_change != 0).
// + The message transfers use clock and SPI mode parameters
// previously established by setup() for this device
//
// Called on deregistration to free memory provided by spi_controller
//
// Used to enable core support for DMA handling, if can_dma()
// exists and returns true then the transfer will be mapped
// prior to transfer_one() being called.  The driver should
// not modify or store xfer and dma_tx and dma_rx must be set
// while the device is prepared.
//
// These hooks are for drivers that want to use the generic
// controller transfer queueing mechanism. If these are used, the
// transfer() function above must NOT be specified by the driver.
// Over time we expect SPI drivers to be phased over to this API.
//
// These hooks are for drivers that use a generic implementation
// of transfer_one_message() provided by the core.
//
// Optimized handlers for SPI memory-like operations.
// SPI or QSPI controller can set to true if supports SDR/DDR transfer rate
// GPIO chip select
// Statistics
// DMA channels for use with core dmaengine helpers
// Dummy data for full duplex devices
//
// Driver sets this field to indicate it is able to snapshot SPI
// transfers (needed e.g. for reading the time of POSIX clocks)
//
// Interrupt enable state during PTP system timestamping
// Flag for enabling opportunistic skipping of the queue in spi_sync

// List of userspace-instantiated devices; protected by @add_lock
// True after new_device/delete_device sysfs group is created

extern "C" {
    pub fn dev_get_drvdata(_arg: &ctlr->dev) -> return;
}
// PM calls that need to be issued by the driver
extern "C" {
    pub fn spi_controller_suspend(ctlr: *mut spi_controller) -> c_int;
}
extern "C" {
    pub fn spi_controller_resume(ctlr: *mut spi_controller) -> c_int;
}
// Calls the driver make to interact with the message queue
extern "C" {
    pub fn spi_finalize_current_message(ctlr: *mut spi_controller);
}
extern "C" {
    pub fn spi_finalize_current_transfer(ctlr: *mut spi_controller);
}
// Helper calls for driver to timestamp transfer
// The SPI driver core manages memory for the spi_controller classdev
extern "C" {
    pub fn __spi_alloc_controller(_arg: dev, _arg: size, _arg: false) -> return;
}
extern "C" {
    pub fn __spi_alloc_controller(_arg: dev, _arg: size, _arg: true) -> return;
}
extern "C" {
    pub fn __devm_spi_alloc_controller(_arg: dev, _arg: size, _arg: false) -> return;
}
extern "C" {
    pub fn __devm_spi_alloc_controller(_arg: dev, _arg: size, _arg: true) -> return;
}
extern "C" {
    pub fn spi_register_controller(ctlr: *mut spi_controller) -> c_int;
}
extern "C" {
    pub fn spi_unregister_controller(ctlr: *mut spi_controller);
}

extern "C" {
    pub fn acpi_spi_count_resources(adev: *mut acpi_device) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

//
// SPI resource management while processing a SPI message
//
// struct spi_res - SPI resource management structure
// @entry:   list entry
// @release: release code called prior to freeing this resource
// @data:    extra data allocated for the specific use-case
//
// This is based on ideas from devres, but focused on life-cycle
// management during spi_message processing.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_res {
    pub entry: list_head,
    pub release: spi_res_release_t,
    pub /: *mut *mut unsigned long long data[]; / Guarantee ull alignment,
}

// ---------------------------------------------------------------------------
//
// I/O INTERFACE between SPI controller and protocol drivers
//
// Protocol drivers use a queue of spi_messages, each transferring data
// between the controller and memory buffers.
//
// The spi_messages themselves consist of a series of read+write transfer
// segments.  Those segments always read the same number of bits as they
// write; but one or the other is easily ignored by passing a NULL buffer
// pointer.  (This is unlike most types of I/O API, because SPI hardware
// is full duplex.)
//
// NOTE:  Allocation of spi_transfer and spi_message memory is entirely
// up to the protocol driver, which guarantees the integrity of both (as
// well as the data buffers) for as long as the message is queued.
//
// struct spi_transfer - a read/write buffer pair
// @tx_buf: data to be written (DMA-safe memory), or NULL
// @rx_buf: data to be read (DMA-safe memory), or NULL
// @tx_dma: DMA address of tx_buf, currently not for client use
// @rx_dma: DMA address of rx_buf, currently not for client use
// @tx_nbits: number of bits used for writing. If 0 the default
// (SPI_NBITS_SINGLE) is used.
// @rx_nbits: number of bits used for reading. If 0 the default
// (SPI_NBITS_SINGLE) is used.
// @multi_lane_mode: How to serialize data on multiple lanes. One of the
// SPI_MULTI_LANE_MODE_* values.
// @len: size of rx and tx buffers (in bytes)
// @speed_hz: Select a speed other than the device default for this
// transfer. If 0 the default (from @spi_device) is used.
// @bits_per_word: select a bits_per_word other than the device default
// for this transfer. If 0 the default (from @spi_device) is used.
// @dummy_data: indicates transfer is dummy bytes transfer.
// @cs_off: performs the transfer with chipselect off.
// @cs_change: affects chipselect after this transfer completes
// @cs_change_delay: delay between cs deassert and assert when
// @cs_change is set and @spi_transfer is not the last in @spi_message
// @delay: delay to be introduced after this transfer before
// (optionally) changing the chipselect status, then starting
// the next transfer or completing this @spi_message.
// @word_delay: inter word delay to be introduced after each word size
// (set by bits_per_word) transmission.
// @effective_speed_hz: the effective SCK-speed that was used to
// transfer this transfer. Set to 0 if the SPI bus driver does
// not support it.
// @transfer_list: transfers are sequenced through @spi_message.transfers
// @tx_sg_mapped: If true, the @tx_sg is mapped for DMA
// @rx_sg_mapped: If true, the @rx_sg is mapped for DMA
// @tx_sg: Scatterlist for transmit, currently not for client use
// @rx_sg: Scatterlist for receive, currently not for client use
// @offload_flags: Flags that are only applicable to specialized SPI offload
// transfers. See %SPI_OFFLOAD_XFER_* in spi-offload.h.
// @ptp_sts_word_pre: The word (subject to bits_per_word semantics) offset
// within @tx_buf for which the SPI device is requesting that the time
// snapshot for this transfer begins. Upon completing the SPI transfer,
// this value may have changed compared to what was requested, depending
// on the available snapshotting resolution (DMA transfer,
// @ptp_sts_supported is false, etc).
// @ptp_sts_word_post: See @ptp_sts_word_pre. The two can be equal (meaning
// that a single byte should be snapshotted).
// If the core takes care of the timestamp (if @ptp_sts_supported is false
// for this controller), it will set @ptp_sts_word_pre to 0, and
// @ptp_sts_word_post to the length of the transfer. This is done
// purposefully (instead of setting to spi_transfer->len - 1) to denote
// that a transfer-level snapshot taken from within the driver may still
// be of higher quality.
// @ptp_sts: Pointer to a memory location held by the SPI target device where a
// PTP system timestamp structure may lie. If drivers use PIO or their
// hardware has some sort of assist for retrieving exact transfer timing,
// they can (and should) assert @ptp_sts_supported and populate this
// structure using the ptp_read_system_*ts helper functions.
// The timestamp must represent the time at which the SPI target device has
// processed the word, i.e. the "pre" timestamp should be taken before
// transmitting the "pre" word, and the "post" timestamp after receiving
// transmit confirmation from the controller for the "post" word.
// @dtr_mode: true if supports double transfer rate.
// @timestamped: true if the transfer has been timestamped
// @error: Error status logged by SPI controller driver.
//
// SPI transfers always write the same number of bytes as they read.
// Protocol drivers should always provide @rx_buf and/or @tx_buf.
// In some cases, they may also want to provide DMA addresses for
// the data being transferred; that may reduce overhead, when the
// underlying driver uses DMA.
//
// If the transmit buffer is NULL, zeroes will be shifted out
// while filling @rx_buf.  If the receive buffer is NULL, the data
// shifted in will be discarded.  Only "len" bytes shift out (or in).
// It's an error to try to shift out a partial word.  (For example, by
// shifting out three bytes with word size of sixteen or twenty bits;
// the former uses two bytes per word, the latter uses four bytes.)
//
// In-memory data values are always in native CPU byte order, translated
// from the wire byte order (big-endian except with SPI_LSB_FIRST).  So
// for example when bits_per_word is sixteen, buffers are 2N bytes long
// (@len = 2N) and hold N sixteen bit words in CPU byte order.
//
// When the word size of the SPI transfer is not a power-of-two multiple
// of eight bits, those in-memory words include extra bits.  In-memory
// words are always seen by protocol drivers as right-justified, so the
// undefined (rx) or unused (tx) bits are always the most significant bits.
//
// All SPI transfers start with the relevant chipselect active.  Normally
// it stays selected until after the last transfer in a message.  Drivers
// can affect the chipselect signal using cs_change.
//
// (i) If the transfer isn't the last one in the message, this flag is
// used to make the chipselect briefly go inactive in the middle of the
// message.  Toggling chipselect in this way may be needed to terminate
// a chip command, letting a single spi_message perform all of group of
// chip transactions together.
//
// (ii) When the transfer is the last one in the message, the chip may
// stay selected until the next transfer.  On multi-device SPI busses
// with nothing blocking messages going to other devices, this is just
// a performance hint; starting a message to another device deselects
// this one.  But in other cases, this can be used to ensure correctness.
// Some devices need protocol transactions to be built from a series of
// spi_message submissions, where the content of one message is determined
// by the results of previous messages and where the whole transaction
// ends when the chipselect goes inactive.
//
// When SPI can transfer in 1x,2x or 4x. It can get this transfer information
// from device through @tx_nbits and @rx_nbits. In Bi-direction, these
// two should both be set. User can set transfer mode with SPI_NBITS_SINGLE(1x)
// SPI_NBITS_DUAL(2x) and SPI_NBITS_QUAD(4x) to support these three transfer.
//
// User may also set dtr_mode to true to use dual transfer mode if desired. if
// not, default considered as single transfer mode.
//
// The code that submits an spi_message (and its spi_transfers)
// to the lower layers is responsible for managing its memory.
// Zero-initialize every field you don't set up explicitly, to
// insulate against future API updates.  After you submit a message
// and its transfers, ignore them until its completion callback.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_transfer {
//
// It's okay if tx_buf == rx_buf (right?).
// For MicroWire, one buffer must be NULL.
// Buffers must work with dma_*map_single() calls.
//
    pub tx_buf: *const c_void,
    pub rx_buf: *mut c_void,
    pub len: unsigned,

    pub error: u16,
    pub tx_sg_mapped: bool,
    pub rx_sg_mapped: bool,
    pub tx_sg: sg_table,
    pub rx_sg: sg_table,
    pub tx_dma: dma_addr_t,
    pub rx_dma: dma_addr_t,
    pub dummy_data:1: unsigned,
    pub cs_off:1: unsigned,
    pub cs_change:1: unsigned,
    pub tx_nbits:4: unsigned,
    pub rx_nbits:4: unsigned,

    pub 2: unsigned multi_lane_mode:,
    pub timestamped:1: unsigned,
    pub dtr_mode: bool,
pub const SPI_NBITS_SINGLE: c_uint = 0x01 /* 1-bit transfer */;
pub const SPI_NBITS_DUAL: c_uint = 0x02 /* 2-bit transfer */;
pub const SPI_NBITS_QUAD: c_uint = 0x04 /* 4-bit transfer */;
pub const SPI_NBITS_OCTAL: c_uint = 0x08 /* 8-bit transfer */;
    pub bits_per_word: u8,
    pub delay: spi_delay,
    pub cs_change_delay: spi_delay,
    pub word_delay: spi_delay,
    pub speed_hz: u32,
    pub effective_speed_hz: u32,
// Use %SPI_OFFLOAD_XFER_* from spi-offload.h
    pub offload_flags: c_uint,
    pub ptp_sts_word_pre: c_uint,
    pub ptp_sts_word_post: c_uint,
    pub ptp_sts: *mut ptp_system_timestamp,
    pub transfer_list: list_head,
}

//
// struct spi_message - one multi-segment SPI transaction
// @transfers: list of transfer segments in this transaction
// @spi: SPI device to which the transaction is queued
// @pre_optimized: peripheral driver pre-optimized the message
// @optimized: the message is in the optimized state
// @prepared: spi_prepare_message was called for the this message
// @status: zero for success, else negative errno
// @complete: called to report transaction completions
// @context: the argument to complete() when it's called
// @frame_length: the total number of bytes in the message
// @actual_length: the total number of bytes that were transferred in all
// successful segments
// @queue: for use by whichever driver currently owns the message
// @state: for use by whichever driver currently owns the message
// @opt_state: for use by whichever driver currently owns the message
// @resources: for resource management when the SPI message is processed
// @offload: (optional) offload instance used by this message
//
// A @spi_message is used to execute an atomic sequence of data transfers,
// each represented by a struct spi_transfer.  The sequence is "atomic"
// in the sense that no other spi_message may use that SPI bus until that
// sequence completes.  On some systems, many such sequences can execute as
// a single programmed DMA transfer.  On all systems, these messages are
// queued, and might complete after transactions to other devices.  Messages
// sent to a given spi_device are always executed in FIFO order.
//
// The code that submits an spi_message (and its spi_transfers)
// to the lower layers is responsible for managing its memory.
// Zero-initialize every field you don't set up explicitly, to
// insulate against future API updates.  After you submit a message
// and its transfers, ignore them until its completion callback.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_message {
    pub transfers: list_head,
    pub spi: *mut spi_device,
// spi_optimize_message() was called for this message
    pub pre_optimized: bool,
// __spi_optimize_message() was called for this message
    pub optimized: bool,
// spi_prepare_message() was called for this message
    pub prepared: bool,
//
// REVISIT: we might want a flag affecting the behavior of the
// last transfer ... allowing things like "read 16 bit length L"
// immediately followed by "read L bytes".  Basically imposing
// a specific message scheduling algorithm.
//
// Some controller drivers (message-at-a-time queue processing)
// could provide that as their default scheduling algorithm.  But
// others (with multi-message pipelines) could need a flag to
// tell them about such special cases.
//
// Completion is reported through a callback
    pub status: c_int,
    pub context): *mut *mut void (complete)(void,
    pub context: *mut c_void,
    pub frame_length: unsigned,
    pub actual_length: unsigned,
//
// For optional use by whatever driver currently owns the
// spi_message ...  between calls to spi_async and then later
// complete(), that's the spi_controller controller driver.
//
    pub queue: list_head,
    pub state: *mut c_void,
//
// Optional state for use by controller driver between calls to
// __spi_optimize_message() and __spi_unoptimize_message().
//
    pub opt_state: *mut c_void,
//
// Optional offload instance used by this message. This must be set
// by the peripheral driver before calling spi_optimize_message().
//
    pub offload: *mut spi_offload,
// List of spi_res resources when the SPI message is processed
    pub resources: list_head,
}

extern "C" {
    pub fn spi_delay_exec(_arg: &t->delay, _arg: t) -> return;
}
//
// spi_message_init_with_transfers - Initialize spi_message and append transfers
// @m: spi_message to be initialized
// @xfers: An array of SPI transfers
// @num_xfers: Number of items in the xfer array
//
// This function initializes the given spi_message and adds each spi_transfer in
// the given array to the message.
//
// It's fine to embed message and transaction structures in other data
// structures so long as you don't free them while they're in use.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_message_with_transfers {
    pub m: spi_message,
    pub t: [spi_transfer; ],
    pub mwt: *mut },
    pub i: unsigned,
    pub flags): *mut *mut mwt = kzalloc_flex(mwt, t, ntrans,,
    pub NULL: return,
    pub i++): for (i = 0; i < ntrans;,
    pub &mwt->m): spi_message_add_tail(&mwt->t[i],,
    pub &mwt->m: return,
    pub msg): *mut *mut extern int spi_optimize_message(struct spi_device spi, struct spi_message,
    pub msg): *mut extern void spi_unoptimize_message(struct spi_message,
    pub msg): *mut spi_message,
    pub spi): *mut extern int spi_setup(struct spi_device,
    pub message): *mut *mut extern int spi_async(struct spi_device spi, struct spi_message,
    pub spi): *mut extern int spi_target_abort(struct spi_device,
    pub spi->controller: *mut *mut spi_controller ctlr =,
    pub SIZE_MAX: return,
    pub ctlr->max_message_size(spi): return,
    pub spi->controller: *mut *mut spi_controller ctlr =,
    pub SIZE_MAX: size_t tr_max =,
    pub spi_max_message_size(spi): size_t msg_max =,
    pub ctlr->max_transfer_size(spi): tr_max =,
// Transfer size limit must not be greater than message size limit
    pub msg_max): return min(tr_max,,
//
// spi_is_bpw_supported - Check if bits per word is supported
// @spi: SPI device
// @bpw: Bits per word
//
// This function checks to see if the SPI controller supports @bpw.
//
// Returns:
// True if @bpw is supported, false otherwise.
//
    pub spi->controller->bits_per_word_mask: u32 bpw_mask =,
    pub true: return,
    pub false: return,
//
// spi_bpw_to_bytes - Covert bits per word to bytes
// @bpw: Bits per word
//
// This function converts the given @bpw to bytes. The result is always
// power-of-two, e.g.,
//
// ===============    =================
// Input (in bits)    Output (in bytes)
// ===============    =================
// 5                   1
// 9                   2
// 21                  4
// 37                  8
// ===============    =================
//
// It will return 0 for the 0 input.
//
// Returns:
// Bytes for the given @bpw.
//
    pub roundup_pow_of_two(BITS_TO_BYTES(bpw)): return,
//
// spi_controller_xfer_timeout - Compute a suitable timeout value
// @ctlr: SPI device
// @xfer: Transfer descriptor
//
// Compute a relevant timeout value for the given transfer. We derive the time
// that it would take on a single data line and take twice this amount of time
// with a minimum of 500ms to avoid false positives on loaded systems.
//
// Returns: Transfer timeout value in milliseconds.
//
    pub 500U): *mut *mut *mut return max(xfer->len  8  2 / (xfer->speed_hz / 1000),,
// ---------------------------------------------------------------------------
// SPI transfer replacement methods which make use of spi_res
    pub spi_replaced_transfers: struct,
    pub res): *mut spi_replaced_transfers,
//
// struct spi_replaced_transfers - structure describing the spi_transfer
// replacements that have occurred
// so that they can get reverted
// @release:            some extra release code to get executed prior to
// releasing this structure
// @extradata:          pointer to some extra data if requested or NULL
// @replaced_transfers: transfers that have been replaced and which need
// to get restored
// @replaced_after:     the transfer after which the @replaced_transfers
// are to get re-inserted
// @inserted:           number of transfers inserted
// @inserted_transfers: array of spi_transfers of array-size @inserted,
// that have been replacing replaced_transfers
//
// Note: that @extradata will point to @inserted_transfers[@inserted]
// if some extra allocation is requested, so alignment will be the same
// as for spi_transfers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_replaced_transfers {
    pub release: spi_replaced_release_t,
    pub extradata: *mut c_void,
    pub replaced_transfers: list_head,
    pub replaced_after: *mut list_head,
    pub inserted: usize,
    pub inserted_transfers: [spi_transfer; ],
}

// ---------------------------------------------------------------------------
// SPI transfer transformation methods
// ---------------------------------------------------------------------------
//
// All these synchronous SPI transfer routines are utilities layered
// over the core async transfer primitive.  Here, "synchronous" means
// they will sleep uninterruptibly until the async transfer completes.
//
extern "C" {
    pub fn spi_sync(spi: *mut spi_device, message: *mut spi_message) -> c_int;
}
extern "C" {
    pub fn spi_sync_locked(spi: *mut spi_device, message: *mut spi_message) -> c_int;
}
extern "C" {
    pub fn spi_bus_lock(ctlr: *mut spi_controller) -> c_int;
}
extern "C" {
    pub fn spi_bus_unlock(ctlr: *mut spi_controller) -> c_int;
}
//
// spi_sync_transfer - synchronous SPI data transfer
// @spi: device with which data will be exchanged
// @xfers: An array of spi_transfers
// @num_xfers: Number of items in the xfer array
// Context: can sleep
//
// Does a synchronous SPI data transfer of the given spi_transfer array.
//
// For more specific semantics see spi_sync().
//
// Return: zero on success, else a negative error code.
//
extern "C" {
    pub fn spi_sync(_arg: spi, _arg: &msg) -> return;
}
//
// spi_write - SPI synchronous write
// @spi: device to which data will be written
// @buf: data buffer
// @len: data buffer size
// Context: can sleep
//
// This function writes the buffer @buf.
// Callable only from contexts that can sleep.
//
// Return: zero on success, else a negative error code.
//
extern "C" {
    pub fn spi_sync_transfer(_arg: spi, _arg: &t, _arg: 1) -> return;
}
//
// spi_read - SPI synchronous read
// @spi: device from which data will be read
// @buf: data buffer
// @len: data buffer size
// Context: can sleep
//
// This function reads the buffer @buf.
// Callable only from contexts that can sleep.
//
// Return: zero on success, else a negative error code.
//
extern "C" {
    pub fn spi_sync_transfer(_arg: spi, _arg: &t, _arg: 1) -> return;
}
// This copies txbuf and rxbuf data; for small transfers only!
//
// spi_w8r8 - SPI synchronous 8 bit write followed by 8 bit read
// @spi: device with which data will be exchanged
// @cmd: command to be written before data is read back
// Context: can sleep
//
// Callable only from contexts that can sleep.
//
// Return: the (unsigned) eight bit number returned by the
// device, or else a negative error code.
//
// Return negative errno or unsigned value
//
// spi_w8r16 - SPI synchronous 8 bit write followed by 16 bit read
// @spi: device with which data will be exchanged
// @cmd: command to be written before data is read back
// Context: can sleep
//
// The number is returned in wire-order, which is at least sometimes
// big-endian.
//
// Callable only from contexts that can sleep.
//
// Return: the (unsigned) sixteen bit number returned by the
// device, or else a negative error code.
//
// Return negative errno or unsigned value
//
// spi_w8r16be - SPI synchronous 8 bit write followed by 16 bit big-endian read
// @spi: device with which data will be exchanged
// @cmd: command to be written before data is read back
// Context: can sleep
//
// This function is similar to spi_w8r16, with the exception that it will
// convert the read 16 bit data word from big-endian to native endianness.
//
// Callable only from contexts that can sleep.
//
// Return: the (unsigned) sixteen bit number returned by the device in CPU
// endianness, or else a negative error code.
//
extern "C" {
    pub fn be16_to_cpu(_arg: result) -> return;
}
// ---------------------------------------------------------------------------
//
// INTERFACE between board init code and SPI infrastructure.
//
// No SPI driver ever sees these SPI device table segments, but
// it's how the SPI core (or adapters that get hotplugged) grows
// the driver model tree.
//
// As a rule, SPI devices can't be probed.  Instead, board init code
// provides a table listing the devices which are present, with enough
// information to bind and set up the device's driver.  There's basic
// support for non-static configurations too; enough to handle adding
// parport adapters, or microcontrollers acting as USB-to-SPI bridges.
//
// struct spi_board_info - board-specific template for a SPI device
// @modalias: Initializes spi_device.modalias; identifies the driver.
// @platform_data: Initializes spi_device.platform_data; the particular
// data stored there is driver-specific.
// @swnode: Software node for the device.
// @controller_data: Initializes spi_device.controller_data; some
// controllers need hints about hardware setup, e.g. for DMA.
// @irq: Initializes spi_device.irq; depends on how the board is wired.
// @max_speed_hz: Initializes spi_device.max_speed_hz; based on limits
// from the chip datasheet and board-specific signal quality issues.
// @bus_num: Identifies which spi_controller parents the spi_device; unused
// by spi_new_device(), and otherwise depends on board wiring.
// @chip_select: Initializes spi_device.chip_select; depends on how
// the board is wired.
// @mode: Initializes spi_device.mode; based on the chip datasheet, board
// wiring (some devices support both 3WIRE and standard modes), and
// possibly presence of an inverter in the chipselect path.
//
// When adding new SPI devices to the device tree, these structures serve
// as a partial device template.  They hold information which can't always
// be determined by drivers.  Information that probe() can establish (such
// as the default transfer wordsize) is not included here.
//
// These structures are used in two places.  Their primary role is to
// be stored in tables of board-specific device descriptors, which are
// declared early in board initialization and then used (much later) to
// populate a controller's device tree after the that controller's driver
// initializes.  A secondary (and atypical) role is as a parameter to
// spi_new_device() call, which happens after those controller drivers
// are active in some dynamic board configuration models.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_board_info {
//
// The device name and module name are coupled, like platform_bus;
// "modalias" is normally the driver name.
//
// platform_data goes to spi_device.dev.platform_data,
// controller_data goes to spi_device.controller_data,
// IRQ is copied too.
//
    pub modalias: [c_char; SPI_NAME_SIZE],
    pub platform_data: *const c_void,
    pub swnode: *const software_node,
    pub controller_data: *mut c_void,
    pub irq: c_int,
// Slower signaling on noisy or low voltage boards
    pub max_speed_hz: u32,
//
// bus_num is board specific and matches the bus_num of some
// spi_controller that will probably be registered later.
//
// chip_select reflects how this chip is wired to that controller;
// it's less than num_chipselect.
//
    pub bus_num: u16,
    pub chip_select: u16,
//
// mode becomes spi_device.mode, and is essential for chips
// where the default of SPI_CS_HIGH = 0 is wrong.
//
    pub mode: u32,
//
// ... may need additional spi_device chip config data here.
// avoid stuff protocol drivers can set; but include stuff
// needed to behave without being bound to a driver:
// - quirks like clock rate mattering when not selected
//
}

// Board init code may ignore whether SPI is configured or not

//
// If you're hotplugging an adapter with devices (parport, USB, etc)
// use spi_new_device() to describe each device.  You can also call
// spi_unregister_device() to start making that device vanish, but
// normally that would be handled by spi_unregister_controller().
//
// You can also use spi_alloc_device() and spi_add_device() to use a two
// stage registration sequence for each spi_device. This gives the caller
// some more control over the spi_device structure before it is registered,
// but requires that caller to initialize fields that would otherwise
// be defined using the board info.
//
extern "C" {
    pub fn spi_unregister_device(spi: *mut spi_device);
}
extern "C" {
    pub fn list_is_last(_arg: &xfer->transfer_list, _arg: &ctlr->cur_msg->transfers) -> return;
}
