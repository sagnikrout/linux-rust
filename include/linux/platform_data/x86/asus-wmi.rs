//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/x86/asus-wmi.h
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

// WMI Methods
pub const ASUS_WMI_METHODID_SPEC: c_uint = 0x43455053 /* BIOS SPECification */;
pub const ASUS_WMI_METHODID_SFBD: c_uint = 0x44424653 /* Set First Boot Device */;
pub const ASUS_WMI_METHODID_GLCD: c_uint = 0x44434C47 /* Get LCD status */;
pub const ASUS_WMI_METHODID_GPID: c_uint = 0x44495047 /* Get Panel ID?? (Resol) */;
pub const ASUS_WMI_METHODID_QMOD: c_uint = 0x444F4D51 /* Quiet MODe */;
pub const ASUS_WMI_METHODID_SPLV: c_uint = 0x4C425053 /* Set Panel Light Value */;
pub const ASUS_WMI_METHODID_AGFN: c_uint = 0x4E464741 /* Atk Generic FuNction */;
pub const ASUS_WMI_METHODID_SFUN: c_uint = 0x4E554653 /* FUNCtionalities */;
pub const ASUS_WMI_METHODID_SDSP: c_uint = 0x50534453 /* Set DiSPlay output */;
pub const ASUS_WMI_METHODID_GDSP: c_uint = 0x50534447 /* Get DiSPlay output */;
pub const ASUS_WMI_METHODID_DEVP: c_uint = 0x50564544 /* DEVice Policy */;
pub const ASUS_WMI_METHODID_OSVR: c_uint = 0x5256534F /* OS VeRsion */;
pub const ASUS_WMI_METHODID_DCTS: c_uint = 0x53544344 /* Device status (DCTS) */;
pub const ASUS_WMI_METHODID_DSTS: c_uint = 0x53545344 /* Device status (DSTS) */;
pub const ASUS_WMI_METHODID_BSTS: c_uint = 0x53545342 /* Bios STatuS ? */;
pub const ASUS_WMI_METHODID_DEVS: c_uint = 0x53564544 /* DEVice Set */;
pub const ASUS_WMI_METHODID_CFVS: c_uint = 0x53564643 /* CPU Frequency Volt Set */;
pub const ASUS_WMI_METHODID_KBFT: c_uint = 0x5446424B /* KeyBoard FilTer */;
pub const ASUS_WMI_METHODID_INIT: c_uint = 0x54494E49 /* INITialize */;
pub const ASUS_WMI_METHODID_HKEY: c_uint = 0x59454B48 /* Hot KEY ?? */;
pub const ASUS_WMI_METHODID_NOTIF: c_uint = 0x00100021 /* Notify method */;
pub const ASUS_WMI_UNSUPPORTED_METHOD: c_uint = 0xFFFFFFFE;
// Wireless
pub const ASUS_WMI_DEVID_HW_SWITCH: c_uint = 0x00010001;
pub const ASUS_WMI_DEVID_WIRELESS_LED: c_uint = 0x00010002;
pub const ASUS_WMI_DEVID_CWAP: c_uint = 0x00010003;
pub const ASUS_WMI_DEVID_WLAN: c_uint = 0x00010011;
pub const ASUS_WMI_DEVID_WLAN_LED: c_uint = 0x00010012;
pub const ASUS_WMI_DEVID_BLUETOOTH: c_uint = 0x00010013;
pub const ASUS_WMI_DEVID_GPS: c_uint = 0x00010015;
pub const ASUS_WMI_DEVID_WIMAX: c_uint = 0x00010017;
pub const ASUS_WMI_DEVID_WWAN3G: c_uint = 0x00010019;
pub const ASUS_WMI_DEVID_UWB: c_uint = 0x00010021;
// Leds
// 0x000200XX and 0x000400XX
pub const ASUS_WMI_DEVID_LED1: c_uint = 0x00020011;
pub const ASUS_WMI_DEVID_LED2: c_uint = 0x00020012;
pub const ASUS_WMI_DEVID_LED3: c_uint = 0x00020013;
pub const ASUS_WMI_DEVID_LED4: c_uint = 0x00020014;
pub const ASUS_WMI_DEVID_LED5: c_uint = 0x00020015;
pub const ASUS_WMI_DEVID_LED6: c_uint = 0x00020016;
pub const ASUS_WMI_DEVID_MICMUTE_LED: c_uint = 0x00040017;
// Disable Camera LED
pub const ASUS_WMI_DEVID_CAMERA_LED_NEG: c_uint = 0x00060078 /* 0 = on (unused) */;
pub const ASUS_WMI_DEVID_CAMERA_LED: c_uint = 0x00060079 /* 1 = on */;
// Backlight and Brightness
pub const ASUS_WMI_DEVID_ALS_ENABLE: c_uint = 0x00050001 /* Ambient Light Sensor */;
pub const ASUS_WMI_DEVID_BACKLIGHT: c_uint = 0x00050011;
pub const ASUS_WMI_DEVID_BRIGHTNESS: c_uint = 0x00050012;
pub const ASUS_WMI_DEVID_KBD_BACKLIGHT: c_uint = 0x00050021;
pub const ASUS_WMI_DEVID_LIGHT_SENSOR: c_uint = 0x00050022 /* ?? */;
pub const ASUS_WMI_DEVID_LIGHTBAR: c_uint = 0x00050025;
pub const ASUS_WMI_DEVID_OOBE: c_uint = 0x0005002F;
// This can only be used to disable the screen, not re-enable
pub const ASUS_WMI_DEVID_SCREENPAD_POWER: c_uint = 0x00050031;
// Writing a brightness re-enables the screen if disabled
pub const ASUS_WMI_DEVID_SCREENPAD_LIGHT: c_uint = 0x00050032;
pub const ASUS_WMI_DEVID_FAN_BOOST_MODE: c_uint = 0x00110018;
pub const ASUS_WMI_DEVID_THROTTLE_THERMAL_POLICY: c_uint = 0x00120075;
pub const ASUS_WMI_DEVID_THROTTLE_THERMAL_POLICY_VIVO: c_uint = 0x00110019;
// Misc
pub const ASUS_WMI_DEVID_PANEL_HD: c_uint = 0x0005001C;
pub const ASUS_WMI_DEVID_PANEL_OD: c_uint = 0x00050019;
pub const ASUS_WMI_DEVID_CAMERA: c_uint = 0x00060013;
pub const ASUS_WMI_DEVID_LID_FLIP: c_uint = 0x00060062;
pub const ASUS_WMI_DEVID_LID_FLIP_ROG: c_uint = 0x00060077;
pub const ASUS_WMI_DEVID_MINI_LED_MODE: c_uint = 0x0005001E;
pub const ASUS_WMI_DEVID_MINI_LED_MODE2: c_uint = 0x0005002E;
pub const ASUS_WMI_DEVID_SCREEN_AUTO_BRIGHTNESS: c_uint = 0x0005002A;
// Storage
pub const ASUS_WMI_DEVID_CARDREADER: c_uint = 0x00080013;
// Input
pub const ASUS_WMI_DEVID_TOUCHPAD: c_uint = 0x00100011;
pub const ASUS_WMI_DEVID_TOUCHPAD_LED: c_uint = 0x00100012;
pub const ASUS_WMI_DEVID_FNLOCK: c_uint = 0x00100023;
// Fan, Thermal
pub const ASUS_WMI_DEVID_THERMAL_CTRL: c_uint = 0x00110011;
pub const ASUS_WMI_DEVID_FAN_CTRL: c_uint = 0x00110012 /* deprecated */;
pub const ASUS_WMI_DEVID_CPU_FAN_CTRL: c_uint = 0x00110013;
pub const ASUS_WMI_DEVID_GPU_FAN_CTRL: c_uint = 0x00110014;
pub const ASUS_WMI_DEVID_MID_FAN_CTRL: c_uint = 0x00110031;
pub const ASUS_WMI_DEVID_CPU_FAN_CURVE: c_uint = 0x00110024;
pub const ASUS_WMI_DEVID_GPU_FAN_CURVE: c_uint = 0x00110025;
pub const ASUS_WMI_DEVID_MID_FAN_CURVE: c_uint = 0x00110032;
// Tunables for AUS ROG laptops
pub const ASUS_WMI_DEVID_PPT_PL2_SPPT: c_uint = 0x001200A0;
pub const ASUS_WMI_DEVID_PPT_PL1_SPL: c_uint = 0x001200A3;
pub const ASUS_WMI_DEVID_PPT_APU_SPPT: c_uint = 0x001200B0;
pub const ASUS_WMI_DEVID_PPT_PLAT_SPPT: c_uint = 0x001200B1;
pub const ASUS_WMI_DEVID_PPT_PL3_FPPT: c_uint = 0x001200C1;
pub const ASUS_WMI_DEVID_NV_DYN_BOOST: c_uint = 0x001200C0;
pub const ASUS_WMI_DEVID_NV_THERM_TARGET: c_uint = 0x001200C2;
// Power
pub const ASUS_WMI_DEVID_PROCESSOR_STATE: c_uint = 0x00120012;
// Deep S3 / Resume on LID open
pub const ASUS_WMI_DEVID_LID_RESUME: c_uint = 0x00120031;
// Maximum charging percentage
pub const ASUS_WMI_DEVID_RSOC: c_uint = 0x00120057;
// Keyboard dock
pub const ASUS_WMI_DEVID_KBD_DOCK: c_uint = 0x00120063;
// Charging mode - 1=Barrel, 2=USB
pub const ASUS_WMI_DEVID_CHARGE_MODE: c_uint = 0x0012006C;
// MCU powersave mode
pub const ASUS_WMI_DEVID_MCU_POWERSAVE: c_uint = 0x001200E2;
// epu is connected? 1 == true
pub const ASUS_WMI_DEVID_EGPU_CONNECTED: c_uint = 0x00090018;
// egpu on/off
pub const ASUS_WMI_DEVID_EGPU: c_uint = 0x00090019;
// dgpu on/off
pub const ASUS_WMI_DEVID_DGPU: c_uint = 0x00090020;
pub const ASUS_WMI_DEVID_APU_MEM: c_uint = 0x000600C1;
pub const ASUS_WMI_DEVID_DGPU_BASE_TGP: c_uint = 0x00120099;
pub const ASUS_WMI_DEVID_DGPU_SET_TGP: c_uint = 0x00120098;
// gpu mux switch, 0 = dGPU, 1 = Optimus
pub const ASUS_WMI_DEVID_GPU_MUX: c_uint = 0x00090016;
pub const ASUS_WMI_DEVID_GPU_MUX_VIVO: c_uint = 0x00090026;
// Keystone dongle insert/remove state.
// PRESENCE_BIT (0x00010000) encodes insert state:
// 0x00010000 = inserted, 0x00000000 = absent. STATUS_BIT is never set.
// 0xFFFFFFFE means no keystone slot on this machine.
//
pub const ASUS_WMI_DEVID_KEYSTONE: c_uint = 0x00120091;
// TUF laptop RGB modes/colours
pub const ASUS_WMI_DEVID_TUF_RGB_MODE: c_uint = 0x00100056;
pub const ASUS_WMI_DEVID_TUF_RGB_MODE2: c_uint = 0x0010005A;
// TUF laptop RGB power/state
pub const ASUS_WMI_DEVID_TUF_RGB_STATE: c_uint = 0x00100057;
// Bootup sound control
pub const ASUS_WMI_DEVID_BOOT_SOUND: c_uint = 0x00130022;
// DSTS masks
pub const ASUS_WMI_DSTS_STATUS_BIT: c_uint = 0x00000001;
pub const ASUS_WMI_DSTS_UNKNOWN_BIT: c_uint = 0x00000002;
pub const ASUS_WMI_DSTS_PRESENCE_BIT: c_uint = 0x00010000;
pub const ASUS_WMI_DSTS_USER_BIT: c_uint = 0x00020000;
pub const ASUS_WMI_DSTS_BIOS_BIT: c_uint = 0x00040000;
pub const ASUS_WMI_DSTS_BRIGHTNESS_MASK: c_uint = 0x000000FF;
pub const ASUS_WMI_DSTS_MAX_BRIGTH_MASK: c_uint = 0x0000FF00;
pub const ASUS_WMI_DSTS_LIGHTBAR_MASK: c_uint = 0x0000000F;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum asus_ally_mcu_hack {
    ASUS_WMI_ALLY_MCU_HACK_INIT,
    ASUS_WMI_ALLY_MCU_HACK_ENABLED,
    ASUS_WMI_ALLY_MCU_HACK_DISABLED,
}

// Used to notify hid-asus when asus-wmi changes keyboard backlight
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asus_hid_listener {
    pub list: list_head,
    pub brightness): *mut *mut *mut void (brightness_set)(struct asus_hid_listener listener, int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum asus_hid_event {
    ASUS_EV_BRTUP,
    ASUS_EV_BRTDOWN,
    ASUS_EV_BRTTOGGLE,
}

pub const ASUS_EV_MAX_BRIGHTNESS: c_int = 3;

extern "C" {
    pub fn set_ally_mcu_hack(status: asus_ally_mcu_hack);
}
extern "C" {
    pub fn set_ally_mcu_powersave(enabled: bool);
}
extern "C" {
    pub fn asus_wmi_get_devstate_dsts(dev_id: u32, retval: *mut u32) -> c_int;
}
extern "C" {
    pub fn asus_wmi_set_devstate(dev_id: u32, ctrl_param: u32, retval: *mut u32) -> c_int;
}
extern "C" {
    pub fn asus_wmi_evaluate_method(method_id: u32, arg0: u32, arg1: u32, retval: *mut u32) -> c_int;
}
extern "C" {
    pub fn asus_hid_register_listener(cdev: *mut asus_hid_listener) -> c_int;
}
extern "C" {
    pub fn asus_hid_unregister_listener(cdev: *mut asus_hid_listener);
}
extern "C" {
    pub fn asus_hid_event(event: asus_hid_event) -> c_int;
}
extern "C" {
    pub fn asus_wmi_custom_fan_curve_is_enabled() -> bool;
}

