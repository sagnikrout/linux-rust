//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_device.h
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
// Recovery methods for wedged device in order of less to more side-effects.
// To be used with drm_dev_wedged_event() as recovery @method. Callers can
// use any one, multiple (or'd) or none depending on their needs.
//
// Refer to "Device Wedging" chapter in Documentation/gpu/drm-uapi.rst for more
// details.
//

//
// struct drm_wedge_task_info - information about the guilty task of a wedge dev
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_wedge_task_info {
// @pid: pid of the task
    pub pid: pid_t,
// @comm: command name of the task
    pub comm: [c_char; TASK_COMM_LEN],
}

//
// enum switch_power_state - power state of drm device
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum switch_power_state {
// @DRM_SWITCH_POWER_ON: Power state is ON
    DRM_SWITCH_POWER_ON = 0,

// @DRM_SWITCH_POWER_OFF: Power state is OFF
    DRM_SWITCH_POWER_OFF = 1,

// @DRM_SWITCH_POWER_CHANGING: Power state is changing
    DRM_SWITCH_POWER_CHANGING = 2,

// @DRM_SWITCH_POWER_DYNAMIC_OFF: Suspended
    DRM_SWITCH_POWER_DYNAMIC_OFF = 3,
}

//
// struct drm_device - DRM device structure
//
// This structure represent a complete card that
// may contain multiple heads.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_device {
// @if_version: Highest interface version set
    pub if_version: c_int,
// @ref: Object ref-count
    pub ref: kref,
// @dev: Device structure of bus-device
    pub dev: *mut device,
//
// @dma_dev:
//
// Device for DMA operations. Only required if the device @dev
// cannot perform DMA by itself. Should be NULL otherwise. Call
// drm_dev_dma_dev() to get the DMA device instead of using this
// field directly. Call drm_dev_set_dma_dev() to set this field.
//
// DRM devices are sometimes bound to virtual devices that cannot
// perform DMA by themselves. Drivers should set this field to the
// respective DMA controller.
//
// Devices on USB and other peripheral busses also cannot perform
// DMA by themselves. The @dma_dev field should point the bus
// controller that does DMA on behalve of such a device. Required
// for importing buffers via dma-buf.
//
// If set, the DRM core automatically releases the reference on the
// device.
//
    pub dma_dev: *mut device,
//
// @managed:
//
// Managed resources linked to the lifetime of this &drm_device as
// tracked by @ref.
//
// @managed.resources: managed resources list
    pub resources: list_head,
// @managed.final_kfree: pointer for final kfree() call
    pub final_kfree: *mut c_void,
// @managed.lock: protects @managed.resources
    pub lock: spinlock_t,
    pub managed: },
// @driver: DRM driver managing the device
    pub driver: *const drm_driver,
//
// @dev_private:
//
// DRM driver private data. This is deprecated and should be left set to
// NULL.
//
// Instead of using this pointer it is recommended that drivers use
// devm_drm_dev_alloc() and embed struct &drm_device in their larger
// per-device structure.
//
    pub dev_private: *mut c_void,
//
// @primary:
//
// Primary node. Drivers should not interact with this
// directly. debugfs interfaces can be registered with
// drm_debugfs_add_file(), and sysfs should be directly added on the
// hardware (and not character device node) struct device @dev.
//
    pub primary: *mut drm_minor,
//
// @render:
//
// Render node. Drivers should not interact with this directly ever.
// Drivers should not expose any additional interfaces in debugfs or
// sysfs on this node.
//
    pub render: *mut drm_minor,
// @accel: Compute Acceleration node
    pub accel: *mut drm_minor,
//
// @registered:
//
// Internally used by drm_dev_register() and drm_connector_register().
//
    pub registered: bool,
//
// @master:
//
// Currently active master for this device.
// Protected by &master_mutex
//
    pub master: *mut drm_master,

//
// @huge_mnt:
//
// Huge tmpfs mountpoint used at GEM object initialization
// drm_gem_object_init(). Drivers can call drm_gem_huge_mnt_create() to
// create, mount and use it. The default tmpfs mountpoint (`shm_mnt`) is
// used if NULL.
//
    pub huge_mnt: *mut vfsmount,

//
// @driver_features: per-device driver features
//
// Drivers can clear specific flags here to disallow
// certain features on a per-device basis while still
// sharing a single &struct drm_driver instance across
// all devices.
//
    pub driver_features: u32,
//
// @unplugged:
//
// Flag to tell if the device has been unplugged.
// See drm_dev_enter() and drm_dev_is_unplugged().
//
    pub unplugged: bool,
// @anon_inode: inode for private address-space
    pub anon_inode: *mut inode,
// @unique: Unique name of the device
    pub unique: *mut c_char,
//
// @master_mutex:
//
// Lock for &drm_minor.master and &drm_file.is_master
//
    pub master_mutex: mutex,
//
// @open_count:
//
// Usage counter for outstanding files open,
// protected by drm_global_mutex
//
    pub open_count: core::sync::atomic::AtomicI32,
// @filelist_mutex: Protects @filelist.
    pub filelist_mutex: mutex,
//
// @filelist:
//
// List of userspace clients, linked through &drm_file.lhead.
//
    pub filelist: list_head,
//
// @filelist_internal:
//
// List of open DRM files for in-kernel clients.
// Protected by &filelist_mutex.
//
    pub filelist_internal: list_head,
//
// @clientlist_mutex:
//
// Protects &clientlist access.
//
    pub clientlist_mutex: mutex,
//
// @clientlist:
//
// List of in-kernel clients. Protected by &clientlist_mutex.
//
    pub clientlist: list_head,
//
// @client_sysrq_list:
//
// Entry into list of devices registered for sysrq. Allows in-kernel
// clients on this device to handle sysrq keys.
//
    pub client_sysrq_list: list_head,
//
// @vblank_disable_immediate:
//
// If true, vblank interrupt will be disabled immediately when the
// refcount drops to zero, as opposed to via the vblank disable
// timer.
//
// This can be set to true it the hardware has a working vblank counter
// with high-precision timestamping (otherwise there are races) and the
// driver uses drm_crtc_vblank_on() and drm_crtc_vblank_off()
// appropriately. Also, see @max_vblank_count,
// &drm_crtc_funcs.get_vblank_counter and
// &drm_vblank_crtc_config.disable_immediate.
//
    pub vblank_disable_immediate: bool,
//
// @vblank:
//
// Array of vblank tracking structures, one per &struct drm_crtc. For
// historical reasons (vblank support predates kernel modesetting) this
// is free-standing and not part of &struct drm_crtc itself. It must be
// initialized explicitly by calling drm_vblank_init().
//
    pub vblank: *mut drm_vblank_crtc,
//
// @vblank_time_lock:
//
// Protects vblank count and time updates during vblank enable/disable
//
    pub vblank_time_lock: spinlock_t,
//
// @vbl_lock: Top-level vblank references lock, wraps the low-level
// @vblank_time_lock.
//
    pub vbl_lock: spinlock_t,
//
// @max_vblank_count:
//
// Maximum value of the vblank registers. This value +1 will result in a
// wrap-around of the vblank register. It is used by the vblank core to
// handle wrap-arounds.
//
// If set to zero the vblank core will try to guess the elapsed vblanks
// between times when the vblank interrupt is disabled through
// high-precision timestamps. That approach is suffering from small
// races and imprecision over longer time periods, hence exposing a
// hardware vblank counter is always recommended.
//
// This is the statically configured device wide maximum. The driver
// can instead choose to use a runtime configurable per-crtc value
// &drm_vblank_crtc.max_vblank_count, in which case @max_vblank_count
// must be left at zero. See drm_crtc_set_max_vblank_count() on how
// to use the per-crtc value.
//
// If non-zero, &drm_crtc_funcs.get_vblank_counter must be set.
//
    pub max_vblank_count: u32,
// @vblank_event_list: List of vblank events
    pub vblank_event_list: list_head,
//
// @event_lock:
//
// Protects @vblank_event_list and event delivery in
// general. See drm_send_event() and drm_send_event_locked().
//
    pub event_lock: spinlock_t,
// @num_crtcs: Number of CRTCs on this device
    pub num_crtcs: c_uint,
// @mode_config: Current mode config
    pub mode_config: drm_mode_config,
// @object_name_lock: GEM information
    pub object_name_lock: mutex,
// @object_name_idr: GEM information
    pub object_name_idr: idr,
// @vma_offset_manager: GEM information
    pub vma_offset_manager: *mut drm_vma_offset_manager,
// @vram_mm: VRAM MM memory manager
    pub vram_mm: *mut drm_vram_mm,
//
// @switch_power_state:
//
// Power state of the client.
// Used by drivers supporting the switcheroo driver.
// The state is maintained in the
// &vga_switcheroo_client_ops.set_gpu_state callback
//
    pub switch_power_state: switch_power_state,
//
// @fb_helper:
//
// Pointer to the fbdev emulation structure.
// Set by drm_fb_helper_init() and cleared by drm_fb_helper_fini().
//
    pub fb_helper: *mut drm_fb_helper,
//
// @debugfs_root:
//
// Root directory for debugfs files.
//
    pub debugfs_root: *mut dentry,
//
// @gem_lru_mutex:
//
// Lock protecting movement of GEM objects between LRUs.
//
    pub gem_lru_mutex: mutex,
}

extern "C" {
    pub fn drm_dev_set_dma_dev(dev: *mut drm_device, dma_dev: *mut device);
}
//
// drm_dev_dma_dev - returns the DMA device for a DRM device
// @dev: DRM device
//
// Returns the DMA device of the given DRM device. By default, this
// the DRM device's parent. See drm_dev_set_dma_dev().
//
// Returns:
// A DMA-capable device for the DRM device.
//
