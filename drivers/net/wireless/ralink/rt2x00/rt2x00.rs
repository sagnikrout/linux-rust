//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ralink/rt2x00/rt2x00.h
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

//
// Module information.
//

// Debug definitions.
// Debug output has to be enabled during compile time.
//

// Macro flag: #define DEBUG

// Utility printing macros
// rt2x00_probe_err is for messages when rt2x00_dev is uninitialized
//

// Various debug levels

//
// Duration calculations
// The rate variable passed is: 100kbs.
// To convert from bytes to bits we multiply size with 8,
// then the size is multiplied with 10 to make the
// real rate -> rate argument correction.
//

//
// Determine the number of L2 padding bytes required between the header and
// the payload.
//

//
// Determine the alignment requirement,
// to make sure the 802.11 payload is padded to a 4-byte boundrary
// we must determine the address of the payload and calculate the
// amount of bytes needed to move the data.
//

//
// Constants for extra TX headroom for alignment purposes.
//

//
// Standard timing and size defines.
// These values should follow the ieee80211 specifications.
//
pub const ACK_SIZE: c_int = 14;
pub const IEEE80211_HEADER: c_int = 24;
pub const PLCP: c_int = 48;
pub const BEACON: c_int = 100;
pub const PREAMBLE: c_int = 144;
pub const SHORT_PREAMBLE: c_int = 72;
pub const SLOT_TIME: c_int = 20;
pub const SHORT_SLOT_TIME: c_int = 9;
pub const SIFS: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt2x00_chip_intf {
    RT2X00_CHIP_INTF_PCI,
    RT2X00_CHIP_INTF_PCIE,
    RT2X00_CHIP_INTF_USB,
    RT2X00_CHIP_INTF_SOC,
}

//
// Chipset identification
// The chipset on the device is composed of a RT and RF chip.
// The chipset combination is important for determining device capabilities.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt2x00_chip {
    pub rt: u16,
pub const RT2460: c_uint = 0x2460;
pub const RT2560: c_uint = 0x2560;
pub const RT2570: c_uint = 0x2570;
pub const RT2661: c_uint = 0x2661;
pub const RT2573: c_uint = 0x2573;
pub const RT2860: c_uint = 0x2860	/* 2.4GHz */;
pub const RT2872: c_uint = 0x2872	/* WSOC */;
pub const RT2883: c_uint = 0x2883	/* WSOC */;
pub const RT3070: c_uint = 0x3070;
pub const RT3071: c_uint = 0x3071;
pub const RT3090: c_uint = 0x3090	/* 2.4GHz PCIe */;
pub const RT3290: c_uint = 0x3290;
pub const RT3352: c_uint = 0x3352  /* WSOC */;
pub const RT3390: c_uint = 0x3390;
pub const RT3572: c_uint = 0x3572;
pub const RT3593: c_uint = 0x3593;
pub const RT3883: c_uint = 0x3883	/* WSOC */;
pub const RT5350: c_uint = 0x5350  /* WSOC 2.4GHz */;
pub const RT5390: c_uint = 0x5390  /* 2.4GHz */;
pub const RT5392: c_uint = 0x5392  /* 2.4GHz */;
pub const RT5592: c_uint = 0x5592;
pub const RT6352: c_uint = 0x6352  /* WSOC 2.4GHz */;
    pub rf: u16,
    pub rev: u16,
    pub intf: rt2x00_chip_intf,
}

//
// RF register values that belong to a particular channel.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rf_channel {
    pub channel: c_int,
    pub rf1: u32,
    pub rf2: u32,
    pub rf3: u32,
    pub rf4: u32,
}

//
// Information structure for channel survey.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt2x00_chan_survey {
    pub time_idle: u64,
    pub time_busy: u64,
    pub time_ext_busy: u64,
}

//
// Channel information structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct channel_info {
    pub flags: c_uint,
pub const GEOGRAPHY_ALLOWED: c_uint = 0x00000001;
    pub max_power: c_short,
    pub default_power1: c_short,
    pub default_power2: c_short,
    pub default_power3: c_short,
}

//
// Antenna setup values.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct antenna_setup {
    pub rx: antenna,
    pub tx: antenna,
    pub rx_chain_num: u8,
    pub tx_chain_num: u8,
}

//
// Quality statistics about the currently active link.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_qual {
//
// Statistics required for Link tuning by driver
// The rssi value is provided by rt2x00lib during the
// link_tuner() callback function.
// The false_cca field is filled during the link_stats()
// callback function and could be used during the
// link_tuner() callback function.
//
    pub rssi: c_int,
    pub false_cca: c_int,
//
// VGC levels
// Hardware driver will tune the VGC level during each call
// to the link_tuner() callback function. This vgc_level is
// determined based on the link quality statistics like
// average RSSI and the false CCA count.
//
// In some cases the drivers need to differentiate between
// the currently "desired" VGC level and the level configured
// in the hardware. The latter is important to reduce the
// number of BBP register reads to reduce register access
// overhead. For this reason we store both values here.
//
    pub vgc_level: u8,
    pub vgc_level_reg: u8,
//
// Statistics required for Signal quality calculation.
// These fields might be changed during the link_stats()
// callback function.
//
    pub rx_success: c_int,
    pub rx_failed: c_int,
    pub tx_success: c_int,
    pub tx_failed: c_int,
}

//
// Antenna settings about the currently active link.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_ant {
//
// Antenna flags
//
    pub flags: c_uint,
pub const ANTENNA_RX_DIVERSITY: c_uint = 0x00000001;
pub const ANTENNA_TX_DIVERSITY: c_uint = 0x00000002;
pub const ANTENNA_MODE_SAMPLE: c_uint = 0x00000004;
//
// Currently active TX/RX antenna setup.
// When software diversity is used, this will indicate
// which antenna is actually used at this time.
//
    pub active: antenna_setup,
//
// RSSI history information for the antenna.
// Used to determine when to switch antenna
// when using software diversity.
//
    pub rssi_history: c_int,
//
// Current RSSI average of the currently active antenna.
// Similar to the avg_rssi in the link_qual structure
// this value is updated by using the walking average.
//
    pub rssi_ant: ewma_rssi,
}

//
// To optimize the quality of the link we need to store
// the quality of received frames and periodically
// optimize the link.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct link {
//
// Link tuner counter
// The number of times the link has been tuned
// since the radio has been switched on.
//
    pub count: u32,
//
// Quality measurement values.
//
    pub qual: link_qual,
//
// TX/RX antenna setup.
//
    pub ant: link_ant,
//
// Currently active average RSSI value
//
    pub avg_rssi: ewma_rssi,
//
// Work structure for scheduling periodic link tuning.
//
    pub work: delayed_work,
//
// Work structure for scheduling periodic watchdog monitoring.
// This work must be scheduled on the kernel workqueue, while
// all other work structures must be queued on the mac80211
// workqueue. This guarantees that the watchdog can schedule
// other work structures and wait for their completion in order
// to bring the device/driver back into the desired state.
//
    pub watchdog_work: delayed_work,
    pub watchdog_interval: c_uint,
    pub watchdog: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt2x00_delayed_flags {
    DELAYED_UPDATE_BEACON,
}

//
// Interface structure
// Per interface configuration details, this structure
// is allocated as the private data for ieee80211_vif.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt2x00_intf {
//
// beacon->skb must be protected with the mutex.
//
    pub beacon_skb_mutex: mutex,
//
// Entry in the beacon queue which belongs to
// this interface. Each interface has its own
// dedicated beacon entry.
//
    pub beacon: *mut queue_entry,
    pub enable_beacon: bool,
//
// Actions that needed rescheduling.
//
    pub delayed_flags: c_ulong,
//
// Software sequence counter, this is only required
// for hardware which doesn't support hardware
// sequence counting.
//
    pub seqno: core::sync::atomic::AtomicI32,
}

//
// struct hw_mode_spec: Hardware specifications structure
//
// Details about the supported modes, rates and channels
// of a particular chipset. This is used by rt2x00lib
// to build the ieee80211_hw_mode array for mac80211.
//
// @supported_bands: Bitmask contained the supported bands (2.4GHz, 5.2GHz).
// @supported_rates: Rate types which are supported (CCK, OFDM).
// @num_channels: Number of supported channels. This is used as array size
// for @tx_power_a, @tx_power_bg and @channels.
// @channels: Device/chipset specific channel values (See &struct rf_channel).
// @channels_info: Additional information for channels (See &struct channel_info).
// @ht: Driver HT Capabilities (See &ieee80211_sta_ht_cap).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_mode_spec {
    pub supported_bands: c_uint,
pub const SUPPORT_BAND_2GHZ: c_uint = 0x00000001;
pub const SUPPORT_BAND_5GHZ: c_uint = 0x00000002;
    pub supported_rates: c_uint,
pub const SUPPORT_RATE_CCK: c_uint = 0x00000001;
pub const SUPPORT_RATE_OFDM: c_uint = 0x00000002;
    pub num_channels: c_uint,
    pub channels: *const rf_channel,
    pub channels_info: *const channel_info,
    pub ht: ieee80211_sta_ht_cap,
}

//
// Configuration structure wrapper around the
// mac80211 configuration structure.
// When mac80211 configures the driver, rt2x00lib
// can precalculate values which are equal for all
// rt2x00 drivers. Those values can be stored in here.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt2x00lib_conf {
    pub conf: *mut ieee80211_conf,
    pub rf: rf_channel,
    pub channel: channel_info,
}

//
// Configuration structure for erp settings.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt2x00lib_erp {
    pub short_preamble: c_int,
    pub cts_protection: c_int,
    pub basic_rates: u32,
    pub slot_time: c_int,
    pub sifs: c_short,
    pub pifs: c_short,
    pub difs: c_short,
    pub eifs: c_short,
    pub beacon_int: u16,
    pub ht_opmode: u16,
}

//
// Configuration structure for hardware encryption.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt2x00lib_crypto {
    pub cipher: cipher,
    pub cmd: set_key_cmd,
    pub address: *const u8,
    pub bssidx: u32,
    pub key: [u8; 16],
    pub tx_mic: [u8; 8],
    pub rx_mic: [u8; 8],
    pub wcid: c_int,
}

//
// Configuration structure wrapper around the
// rt2x00 interface configuration handler.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt2x00intf_conf {
//
// Interface type
//
    pub type: nl80211_iftype,
//
// TSF sync value, this is dependent on the operation type.
//
    pub sync: tsf_sync,
//
// The MAC and BSSID addresses are simple array of bytes,
// these arrays are little endian, so when sending the addresses
// to the drivers, copy the it into a endian-signed variable.
//
// Note that all devices (except rt2500usb) have 32 bits
// register word sizes. This means that whatever variable we
// pass _must_ be a multiple of 32 bits. Otherwise the device
// might not accept what we are sending to it.
// This will also make it easier for the driver to write
// the data to the device.
//
    pub mac: [__le32; 2],
    pub bssid: [__le32; 2],
}

//
// Private structure for storing STA details
// wcid: Wireless Client ID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt2x00_sta {
    pub wcid: c_int,
}

//
// rt2x00lib callback functions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt2x00lib_ops {
//
// Interrupt handlers.
//
    pub irq_handler: irq_handler_t,
//
// TX status tasklet handler.
//
    pub t): *mut *mut void (txstatus_tasklet) (struct tasklet_struct,
    pub t): *mut *mut void (pretbtt_tasklet) (struct tasklet_struct,
    pub t): *mut *mut void (tbtt_tasklet) (struct tasklet_struct,
    pub t): *mut *mut void (rxdone_tasklet) (struct tasklet_struct,
    pub t): *mut *mut void (autowake_tasklet) (struct tasklet_struct,
//
// Device init handlers.
//
    pub rt2x00dev): *mut *mut int (probe_hw) (struct rt2x00_dev,
    pub rt2x00dev): *mut *mut *mut char (get_firmware_name) (struct rt2x00_dev,
    pub len): *const *const u8 data, size_t,
    pub len): *const *const u8 data, size_t,
//
// Device initialization/deinitialization handlers.
//
    pub rt2x00dev): *mut *mut int (initialize) (struct rt2x00_dev,
    pub rt2x00dev): *mut *mut void (uninitialize) (struct rt2x00_dev,
//
// queue initialization handlers
//
    pub entry): *mut *mut bool (get_entry_state) (struct queue_entry,
    pub entry): *mut *mut void (clear_entry) (struct queue_entry,
//
// Radio control handlers.
//
    pub state): dev_state,
    pub rt2x00dev): *mut *mut int (rfkill_poll) (struct rt2x00_dev,
    pub qual): *mut link_qual,
    pub qual): *mut link_qual,
    pub count): *const *const link_qual qual, u32,
    pub rt2x00dev): *mut *mut void (gain_calibration) (struct rt2x00_dev,
    pub rt2x00dev): *mut *mut void (vco_calibration) (struct rt2x00_dev,
//
// Data queue handlers.
//
    pub rt2x00dev): *mut *mut void (watchdog) (struct rt2x00_dev,
    pub queue): *mut *mut void (start_queue) (struct data_queue,
    pub queue): *mut *mut void (kick_queue) (struct data_queue,
    pub queue): *mut *mut void (stop_queue) (struct data_queue,
    pub drop): *mut *mut *mut void (flush_queue) (struct data_queue queue, bool,
    pub entry): *mut *mut void (tx_dma_done) (struct queue_entry,
//
// TX control handlers
//
    pub txdesc): *mut txentry_desc,
    pub txdesc): *mut txentry_desc,
    pub txdesc): *mut txentry_desc,
    pub entry): *mut *mut void (clear_beacon) (struct queue_entry,
    pub entry): *mut *mut int (get_tx_data_len) (struct queue_entry,
//
// RX control handlers
//
    pub rxdesc): *mut rxdone_entry_desc,
//
// Configuration handlers.
//
    pub key): *mut ieee80211_key_conf,
    pub key): *mut ieee80211_key_conf,
    pub filter_flags): c_uint,
    pub flags): c_uint,

    pub changed): u32,
    pub ant): *mut antenna_setup,
    pub changed_flags): c_uint,
    pub rt2x00dev): *mut *mut void (pre_reset_hw) (struct rt2x00_dev,
    pub sta): *mut ieee80211_sta,
    pub sta): *mut ieee80211_sta,
}

//
// rt2x00 driver callback operation structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt2x00_ops {
    pub name: *const c_char,
    pub drv_data_size: c_uint,
    pub max_ap_intf: c_uint,
    pub eeprom_size: c_uint,
    pub rf_size: c_uint,
    pub tx_queues: c_uint,
    pub queue): *mut *mut void (queue_init)(struct data_queue,
    pub lib: *const rt2x00lib_ops,
    pub drv: *const c_void,
    pub hw: *const ieee80211_ops,

    pub debugfs: *const rt2x00debug,

}

//
// rt2x00 state flags
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt2x00_state_flags {
//
// Device flags
//
    DEVICE_STATE_PRESENT,
    DEVICE_STATE_REGISTERED_HW,
    DEVICE_STATE_INITIALIZED,
    DEVICE_STATE_STARTED,
    DEVICE_STATE_ENABLED_RADIO,
    DEVICE_STATE_SCANNING,
    DEVICE_STATE_FLUSHING,
    DEVICE_STATE_RESET,

//
// Driver configuration
//
    CONFIG_CHANNEL_HT40,
    CONFIG_POWERSAVING,
    CONFIG_HT_DISABLED,
    CONFIG_MONITORING,

//
// Mark we currently are sequentially reading TX_STA_FIFO register
// FIXME: this is for only rt2800usb, should go to private data
//
    TX_STATUS_READING,
}

//
// rt2x00 capability flags
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt2x00_capability_flags {
//
// Requirements
//
    REQUIRE_FIRMWARE,
    REQUIRE_BEACON_GUARD,
    REQUIRE_ATIM_QUEUE,
    REQUIRE_DMA,
    REQUIRE_COPY_IV,
    REQUIRE_L2PAD,
    REQUIRE_TXSTATUS_FIFO,
    REQUIRE_TASKLET_CONTEXT,
    REQUIRE_SW_SEQNO,
    REQUIRE_HT_TX_DESC,
    REQUIRE_PS_AUTOWAKE,
    REQUIRE_DELAYED_RFKILL,

//
// Capabilities
//
    CAPABILITY_HW_BUTTON,
    CAPABILITY_HW_CRYPTO,
    CAPABILITY_POWER_LIMIT,
    CAPABILITY_CONTROL_FILTERS,
    CAPABILITY_CONTROL_FILTER_PSPOLL,
    CAPABILITY_PRE_TBTT_INTERRUPT,
    CAPABILITY_LINK_TUNING,
    CAPABILITY_FRAME_TYPE,
    CAPABILITY_RF_SEQUENCE,
    CAPABILITY_EXTERNAL_LNA_A,
    CAPABILITY_EXTERNAL_LNA_BG,
    CAPABILITY_DOUBLE_ANTENNA,
    CAPABILITY_BT_COEXIST,
    CAPABILITY_VCO_RECALIBRATION,
    CAPABILITY_EXTERNAL_PA_TX0,
    CAPABILITY_EXTERNAL_PA_TX1,
    CAPABILITY_RESTART_HW,
}

//
// Interface combinations
//
// rt2x00 device structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt2x00_dev {
//
// Device structure.
// The structure stored in here depends on the
// system bus (PCI or USB).
// When accessing this variable, the rt2x00dev_{pci,usb}
// macros should be used for correct typecasting.
//
    pub dev: *mut device,
//
// Callback functions.
//
    pub ops: *const rt2x00_ops,
//
// Driver data.
//
    pub drv_data: *mut c_void,
//
// IEEE80211 control structure.
//
    pub hw: *mut ieee80211_hw,
    pub bands: [ieee80211_supported_band; NUM_NL80211_BANDS],
    pub chan_survey: *mut rt2x00_chan_survey,
    pub curr_band: nl80211_band,
    pub curr_freq: c_int,
//
// If enabled, the debugfs interface structures
// required for deregistration of debugfs.
//

    pub debugfs_intf: *mut rt2x00debug_intf,

//
// LED structure for changing the LED status
// by mac8011 or the kernel.
//

    pub led_radio: rt2x00_led,
    pub led_assoc: rt2x00_led,
    pub led_qual: rt2x00_led,
    pub led_mcu_reg: u16,

//
// Device state flags.
// In these flags the current status is stored.
// Access to these flags should occur atomically.
//
    pub flags: c_ulong,
//
// Device capabiltiy flags.
// In these flags the device/driver capabilities are stored.
// Access to these flags should occur non-atomically.
//
    pub cap_flags: c_ulong,
//
// Device information, Bus IRQ and name (PCI, SoC)
//
    pub irq: c_int,
    pub name: *const c_char,
//
// Chipset identification.
//
    pub chip: rt2x00_chip,
//
// hw capability specifications.
//
    pub spec: hw_mode_spec,
//
// This is the default TX/RX antenna setup as indicated
// by the device's EEPROM.
//
    pub default_ant: antenna_setup,
//
// Register pointers
// csr.base: CSR base register address. (PCI)
// csr.cache: CSR cache for usb_control_msg. (USB)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union csr {
    pub base: *mut void __iomem,
    pub cache: *mut c_void,
    pub csr: },
//
// Mutex to protect register accesses.
// For PCI and USB devices it protects against concurrent indirect
// register access (BBP, RF, MCU) since accessing those
// registers require multiple calls to the CSR registers.
// For USB devices it also protects the csr_cache since that
// field is used for normal CSR access and it cannot support
// multiple callers simultaneously.
//
    pub csr_mutex: mutex,
//
// Mutex to synchronize config and link tuner.
//
    pub conf_mutex: mutex,
//
// Current packet filter configuration for the device.
// This contains all currently active FIF_* flags send
// to us by mac80211 during configure_filter().
//
    pub packet_filter: c_uint,
//
// Interface details:
// - Open ap interface count.
// - Open sta interface count.
// - Association count.
// - Beaconing enabled count.
//
    pub intf_ap_count: c_uint,
    pub intf_sta_count: c_uint,
    pub intf_associated: c_uint,
    pub intf_beaconing: c_uint,
//
// Interface combinations
//
    pub if_limits_ap: ieee80211_iface_limit,
    pub if_combinations: [ieee80211_iface_combination; NUM_IF_COMB],
//
// Link quality
//
    pub link: link,
//
// EEPROM data.
//
    pub eeprom: *mut __le16,
//
// Active RF register values.
// These are stored here so we don't need
// to read the rf registers and can directly
// use this value instead.
// This field should be accessed by using
// rt2x00_rf_read() and rt2x00_rf_write().
//
    pub rf: *mut u32,
//
// LNA gain
//
    pub lna_gain: c_short,
//
// Current TX power value.
//
    pub tx_power: u16,
//
// Current retry values.
//
    pub short_retry: u8,
    pub long_retry: u8,
//
// Rssi <-> Dbm offset
//
    pub rssi_offset: u8,
//
// Frequency offset.
//
    pub freq_offset: u8,
//
// Association id.
//
    pub aid: u16,
//
// Beacon interval.
//
    pub beacon_int: u16,
// Rx/Tx DMA busy watchdog counter
    pub txdma_busy: u16 rxdma_busy,,
//
// Timestamp of last received beacon
//
    pub last_beacon: c_ulong,
//
// Low level statistics which will have
// to be kept up to date while device is running.
//
    pub low_level_stats: ieee80211_low_level_stats,
//
// Work queue for all work which should not be placed
// on the mac80211 workqueue (because of dependencies
// between various work structures).
//
    pub workqueue: *mut workqueue_struct,
//
// Scheduled work.
// NOTE: intf_work will use ieee80211_iterate_active_interfaces()
// which means it cannot be placed on the hw->workqueue
// due to RTNL locking requirements.
//
    pub intf_work: work_struct,
//
// Scheduled work for TX/RX done handling (USB devices)
//
    pub rxdone_work: work_struct,
    pub txdone_work: work_struct,
//
// Powersaving work
//
    pub autowakeup_work: delayed_work,
    pub sleep_work: work_struct,
//
// Data queue arrays for RX, TX, Beacon and ATIM.
//
    pub data_queues: c_uint,
    pub rx: *mut data_queue,
    pub tx: *mut data_queue,
    pub bcn: *mut data_queue,
    pub atim: *mut data_queue,
//
// Firmware image.
//
    pub fw: *const firmware,
//
// FIFO for storing tx status reports between isr and tasklet.
//
    pub u32): DECLARE_KFIFO_PTR(txstatus_fifo,,
//
// Timer to ensure tx status reports are read (rt2800usb).
//
    pub txstatus_timer: hrtimer,
//
// Tasklet for processing tx status reports (rt2800pci).
//
    pub txstatus_tasklet: tasklet_struct,
    pub pretbtt_tasklet: tasklet_struct,
    pub tbtt_tasklet: tasklet_struct,
    pub rxdone_tasklet: tasklet_struct,
    pub autowake_tasklet: tasklet_struct,
//
// Used for VCO periodic calibration.
//
    pub rf_channel: c_int,
//
// Protect the interrupt mask register.
//
    pub irqmask_lock: spinlock_t,
//
// List of BlockAckReq TX entries that need driver BlockAck processing.
//
    pub bar_list: list_head,
    pub bar_list_lock: spinlock_t,
// Extra TX headroom required for alignment purposes.
    pub extra_tx_headroom: c_uint,
    pub num_proto_errs: c_uint,
// Clock for System On Chip devices.
    pub clk: *mut clk,
    pub anchor: [usb_anchor; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt2x00_bar_list_entry {
    pub list: list_head,
    pub head: rcu_head,
    pub entry: *mut queue_entry,
    pub block_acked: c_int,
// Relevant parts of the IEEE80211 BAR header
    pub ra: [__u8; 6],
    pub ta: [__u8; 6],
    pub control: __le16,
    pub start_seq_num: __le16,
}

//
// Register defines.
// Some registers require multiple attempts before success,
// in those cases REGISTER_BUSY_COUNT attempts should be
// taken with a REGISTER_BUSY_DELAY interval. Due to USB
// bus delays, we do not have to loop so many times to wait
// for valid register value on that bus.
//
pub const REGISTER_BUSY_COUNT: c_int = 100;
pub const REGISTER_USB_BUSY_COUNT: c_int = 20;
pub const REGISTER_BUSY_DELAY: c_int = 100;
//
// Generic RF access.
// The RF is being accessed by word index.
//
// Generic EEPROM access. The EEPROM is being accessed by word or byte index.
//
extern "C" {
    pub fn le16_to_cpu(_arg: rt2x00dev->eeprom[word]) -> return;
}
//
// Chipset handlers
//
extern "C" {
    pub fn rt2x00_intf(_arg: rt2x00dev, _arg: RT2X00_CHIP_INTF_PCIE) -> return;
}
extern "C" {
    pub fn rt2x00_intf(_arg: rt2x00dev, _arg: RT2X00_CHIP_INTF_USB) -> return;
}
extern "C" {
    pub fn rt2x00_intf(_arg: rt2x00dev, _arg: RT2X00_CHIP_INTF_SOC) -> return;
}
// Helpers for capability flags
extern "C" {
    pub fn test_bit(_arg: cap_flag, _arg: &rt2x00dev->cap_flags) -> return;
}
extern "C" {
    pub fn rt2x00_has_cap_flag(_arg: rt2x00dev, _arg: CAPABILITY_HW_CRYPTO) -> return;
}
extern "C" {
    pub fn rt2x00_has_cap_flag(_arg: rt2x00dev, _arg: CAPABILITY_POWER_LIMIT) -> return;
}
extern "C" {
    pub fn rt2x00_has_cap_flag(_arg: rt2x00dev, _arg: CAPABILITY_CONTROL_FILTERS) -> return;
}
extern "C" {
    pub fn rt2x00_has_cap_flag(_arg: rt2x00dev, _arg: CAPABILITY_CONTROL_FILTER_PSPOLL) -> return;
}
extern "C" {
    pub fn rt2x00_has_cap_flag(_arg: rt2x00dev, _arg: CAPABILITY_PRE_TBTT_INTERRUPT) -> return;
}
extern "C" {
    pub fn rt2x00_has_cap_flag(_arg: rt2x00dev, _arg: CAPABILITY_LINK_TUNING) -> return;
}
extern "C" {
    pub fn rt2x00_has_cap_flag(_arg: rt2x00dev, _arg: CAPABILITY_FRAME_TYPE) -> return;
}
extern "C" {
    pub fn rt2x00_has_cap_flag(_arg: rt2x00dev, _arg: CAPABILITY_RF_SEQUENCE) -> return;
}
extern "C" {
    pub fn rt2x00_has_cap_flag(_arg: rt2x00dev, _arg: CAPABILITY_EXTERNAL_LNA_A) -> return;
}
extern "C" {
    pub fn rt2x00_has_cap_flag(_arg: rt2x00dev, _arg: CAPABILITY_EXTERNAL_LNA_BG) -> return;
}
extern "C" {
    pub fn rt2x00_has_cap_flag(_arg: rt2x00dev, _arg: CAPABILITY_EXTERNAL_PA_TX0) -> return;
}
extern "C" {
    pub fn rt2x00_has_cap_flag(_arg: rt2x00dev, _arg: CAPABILITY_DOUBLE_ANTENNA) -> return;
}
extern "C" {
    pub fn rt2x00_has_cap_flag(_arg: rt2x00dev, _arg: CAPABILITY_BT_COEXIST) -> return;
}
extern "C" {
    pub fn rt2x00_has_cap_flag(_arg: rt2x00dev, _arg: CAPABILITY_VCO_RECALIBRATION) -> return;
}
extern "C" {
    pub fn rt2x00_has_cap_flag(_arg: rt2x00dev, _arg: CAPABILITY_RESTART_HW) -> return;
}
//
// rt2x00queue_map_txskb - Map a skb into DMA for TX purposes.
// @entry: Pointer to &struct queue_entry
//
// Returns -ENOMEM if mapping fail, 0 otherwise.
//
extern "C" {
    pub fn rt2x00queue_map_txskb(entry: *mut queue_entry) -> c_int;
}
//
// rt2x00queue_unmap_skb - Unmap a skb from DMA.
// @entry: Pointer to &struct queue_entry
//
extern "C" {
    pub fn rt2x00queue_unmap_skb(entry: *mut queue_entry);
}
//
// rt2x00queue_get_tx_queue - Convert tx queue index to queue pointer
// @rt2x00dev: Pointer to &struct rt2x00_dev.
// @queue: rt2x00 queue index (see &enum data_queue_qid).
//
// Returns NULL for non tx queues.
//
// rt2x00queue_get_entry - Get queue entry where the given index points to.
// @queue: Pointer to &struct data_queue from where we obtain the entry.
// @index: Index identifier for obtaining the correct index.
//
// rt2x00queue_pause_queue - Pause a data queue
// @queue: Pointer to &struct data_queue.
//
// This function will pause the data queue locally, preventing
// new frames to be added to the queue (while the hardware is
// still allowed to run).
//
extern "C" {
    pub fn rt2x00queue_pause_queue(queue: *mut data_queue);
}
//
// rt2x00queue_unpause_queue - unpause a data queue
// @queue: Pointer to &struct data_queue.
//
// This function will unpause the data queue locally, allowing
// new frames to be added to the queue again.
//
extern "C" {
    pub fn rt2x00queue_unpause_queue(queue: *mut data_queue);
}
//
// rt2x00queue_start_queue - Start a data queue
// @queue: Pointer to &struct data_queue.
//
// This function will start handling all pending frames in the queue.
//
extern "C" {
    pub fn rt2x00queue_start_queue(queue: *mut data_queue);
}
//
// rt2x00queue_stop_queue - Halt a data queue
// @queue: Pointer to &struct data_queue.
//
// This function will stop all pending frames in the queue.
//
extern "C" {
    pub fn rt2x00queue_stop_queue(queue: *mut data_queue);
}
//
// rt2x00queue_flush_queue - Flush a data queue
// @queue: Pointer to &struct data_queue.
// @drop: True to drop all pending frames.
//
// This function will flush the queue. After this call
// the queue is guaranteed to be empty.
//
extern "C" {
    pub fn rt2x00queue_flush_queue(queue: *mut data_queue, drop: bool);
}
//
// rt2x00queue_start_queues - Start all data queues
// @rt2x00dev: Pointer to &struct rt2x00_dev.
//
// This function will loop through all available queues to start them
//
extern "C" {
    pub fn rt2x00queue_start_queues(rt2x00dev: *mut rt2x00_dev);
}
//
// rt2x00queue_stop_queues - Halt all data queues
// @rt2x00dev: Pointer to &struct rt2x00_dev.
//
// This function will loop through all available queues to stop
// any pending frames.
//
extern "C" {
    pub fn rt2x00queue_stop_queues(rt2x00dev: *mut rt2x00_dev);
}
//
// rt2x00queue_flush_queues - Flush all data queues
// @rt2x00dev: Pointer to &struct rt2x00_dev.
// @drop: True to drop all pending frames.
//
// This function will loop through all available queues to flush
// any pending frames.
//
extern "C" {
    pub fn rt2x00queue_flush_queues(rt2x00dev: *mut rt2x00_dev, drop: bool);
}
//
// Debugfs handlers.
//
// rt2x00debug_dump_frame - Dump a frame to userspace through debugfs.
// @rt2x00dev: Pointer to &struct rt2x00_dev.
// @type: The type of frame that is being dumped.
// @entry: The queue entry containing the frame to be dumped.
//

//
// Utility functions.
//
extern "C" {
    pub fn rt2x00lib_set_mac_address(rt2x00dev: *mut rt2x00_dev, eeprom_mac_addr: *mut u8) -> c_int;
}
//
// Interrupt context handlers.
//
extern "C" {
    pub fn rt2x00lib_beacondone(rt2x00dev: *mut rt2x00_dev);
}
extern "C" {
    pub fn rt2x00lib_pretbtt(rt2x00dev: *mut rt2x00_dev);
}
extern "C" {
    pub fn rt2x00lib_dmastart(entry: *mut queue_entry);
}
extern "C" {
    pub fn rt2x00lib_dmadone(entry: *mut queue_entry);
}
extern "C" {
    pub fn rt2x00lib_txdone_noinfo(entry: *mut queue_entry, status: u32);
}
extern "C" {
    pub fn rt2x00lib_rxdone(entry: *mut queue_entry, gfp: gfp_t);
}
//
// mac80211 handlers.
//
extern "C" {
    pub fn rt2x00mac_start(hw: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn rt2x00mac_stop(hw: *mut ieee80211_hw, suspend: bool);
}
extern "C" {
    pub fn rt2x00mac_config(hw: *mut ieee80211_hw, radio_idx: c_int, changed: u32) -> c_int;
}

extern "C" {
    pub fn rt2x00mac_rfkill_poll(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rt2x00mac_tx_frames_pending(hw: *mut ieee80211_hw) -> bool;
}
//
// Driver allocation handlers.
//
extern "C" {
    pub fn rt2x00lib_probe_dev(rt2x00dev: *mut rt2x00_dev) -> c_int;
}
extern "C" {
    pub fn rt2x00lib_remove_dev(rt2x00dev: *mut rt2x00_dev);
}
extern "C" {
    pub fn rt2x00lib_suspend(rt2x00dev: *mut rt2x00_dev) -> c_int;
}
extern "C" {
    pub fn rt2x00lib_resume(rt2x00dev: *mut rt2x00_dev) -> c_int;
}
