//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/cx88/cx88-input.c
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
// Device driver for GPIO attached remote control interfaces
// on Conexant 2388x based TV/DVB cards.
//
// Copyright (c) 2003 Pavel Machek
// Copyright (c) 2004 Gerd Knorr
// Copyright (c) 2004, 2005 Chris Pascoe
//

// ----------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx88_IR {
    pub core: *mut cx88_core,
    pub dev: *mut rc_dev,
    pub users: c_int,
    pub name: [c_char; 32],
    pub phys: [c_char; 32],
// sample from gpio pin 16
    pub sampling: u32,
// poll external decoder
    pub polling: c_int,
    pub timer: hrtimer,
    pub gpio_addr: u32,
    pub last_gpio: u32,
    pub mask_keycode: u32,
    pub mask_keydown: u32,
    pub mask_keyup: u32,
}

    let mut ir_samplerate: static unsigned int = 4;
    module_param(ir_samplerate, uint, 0444);
    MODULE_PARM_DESC(ir_samplerate, "IR samplerate in kHz, 1 - 20, default 4");
    static int ir_debug;
    module_param(ir_debug, int, 0644);	/* debug level [IR] */
    MODULE_PARM_DESC(ir_debug, "enable debug messages [IR]");

    if (ir_debug)							\
    printk(KERN_DEBUG "%s IR: " fmt, ir.core.name, ##arg);\
    } while (0)

    if (ir_debug)							\
    printk(KERN_DEBUG "cx88 IR: " fmt, ##arg);		\
    } while (0)
// ----------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn cx88_ir_handle_key(ir: *mut cx88_IR) {
    static void cx88_ir_handle_key(struct cx88_IR *ir)
    {
    struct cx88_core *core = ir.core;
    u32 gpio, data, auxgpio;
// read gpio value
    gpio = cx_read(ir.gpio_addr);
    switch (core.boardnr) {
    case CX88_BOARD_NPGTECH_REALTV_TOP10FM:
//
// This board apparently uses a combination of 2 GPIO
// to represent the keys. Additionally, the second GPIO
// can be used for parity.
//
// Example:
//
// for key "5"
// gpio = 0x758, auxgpio = 0xe5 or 0xf5
// for key "Power"
// gpio = 0x758, auxgpio = 0xed or 0xfd
//
    auxgpio = cx_read(MO_GP1_IO);
// Take out the parity part
    gpio = (gpio & 0x7fd) + (auxgpio & 0xef);
    break;
    case CX88_BOARD_WINFAST_DTV1000:
    case CX88_BOARD_WINFAST_DTV1800H:
    case CX88_BOARD_WINFAST_DTV1800H_XC4000:
    case CX88_BOARD_WINFAST_DTV2000H_PLUS:
    case CX88_BOARD_WINFAST_TV2000_XP_GLOBAL:
    case CX88_BOARD_WINFAST_TV2000_XP_GLOBAL_6F36:
    case CX88_BOARD_WINFAST_TV2000_XP_GLOBAL_6F43:
    gpio = (gpio & 0x6ff) | ((cx_read(MO_GP1_IO) << 8) & 0x900);
    auxgpio = gpio;
    break;
    default:
    auxgpio = gpio;
    }
    if (ir.polling) {
    if (ir.last_gpio == auxgpio)
    return;
    ir.last_gpio = auxgpio;
    }
// extract data
    data = ir_extract_bits(gpio, ir.mask_keycode);
    ir_dprintk("irq gpio=0x%x code=%d | %s%s%s\n",
    gpio, data,
    ir.polling ? "poll" : "irq",
    (gpio & ir.mask_keydown) ? " down" : "",
    (gpio & ir.mask_keyup) ? " up" : "");
    if (ir.core.boardnr == CX88_BOARD_NORWOOD_MICRO) {
    let mut gpio_key: u32 = cx_read(MO_GP0_IO);
    data = (data << 4) | ((gpio_key & 0xf0) >> 4);
    rc_keydown(ir.dev, RC_PROTO_UNKNOWN, data, 0);
    } else if (ir.core.boardnr == CX88_BOARD_PROLINK_PLAYTVPVR ||
    ir.core.boardnr == CX88_BOARD_PIXELVIEW_PLAYTV_ULTRA_PRO) {
// bit cleared on keydown, NEC scancode, 0xAAAACC, A = 0x866b
    u16 addr;
    u8 cmd;
    u32 scancode;
    addr = (data >> 8) & 0xffff;
    cmd  = (data >> 0) & 0x00ff;
    scancode = RC_SCANCODE_NECX(addr, cmd);
    if (0 == (gpio & ir.mask_keyup))
    rc_keydown_notimeout(ir.dev, RC_PROTO_NECX, scancode,
    0);
    else
    rc_keyup(ir.dev);
    } else if (ir.mask_keydown) {
// bit set on keydown
    if (gpio & ir.mask_keydown)
    rc_keydown_notimeout(ir.dev, RC_PROTO_UNKNOWN, data,
    0);
    else
    rc_keyup(ir.dev);
    } else if (ir.mask_keyup) {
// bit cleared on keydown
    if (0 == (gpio & ir.mask_keyup))
    rc_keydown_notimeout(ir.dev, RC_PROTO_UNKNOWN, data,
    0);
    else
    rc_keyup(ir.dev);
    } else {
// can't distinguish keydown/up :-/
    rc_keydown_notimeout(ir.dev, RC_PROTO_UNKNOWN, data, 0);
    rc_keyup(ir.dev);
    }
    }
#[no_mangle]
unsafe extern "C" fn cx88_ir_work(timer: *mut hrtimer) -> enum hrtimer_restart {
    static enum hrtimer_restart cx88_ir_work(struct hrtimer *timer)
    {
    u64 missed;
    struct cx88_IR *ir = container_of(timer, struct cx88_IR, timer);
    cx88_ir_handle_key(ir);
    missed = hrtimer_forward_now(&ir.timer,
    ktime_set(0, ir.polling * 1000000));
    if (missed > 1)
    ir_dprintk("Missed ticks %llu\n", missed - 1);
    return HRTIMER_RESTART;
    }
#[no_mangle]
unsafe extern "C" fn __cx88_ir_start(priv: *mut c_void) -> c_int {
    static int __cx88_ir_start(void *priv)
    {
    struct cx88_core *core = priv;
    struct cx88_IR *ir;
    if (!core || !core.ir)
    return -EINVAL;
    ir = core.ir;
    if (ir.polling) {
    hrtimer_setup(&ir.timer, cx88_ir_work, CLOCK_MONOTONIC, HRTIMER_MODE_REL);
    hrtimer_start(&ir.timer,
    ktime_set(0, ir.polling * 1000000),
    HRTIMER_MODE_REL);
    }
    if (ir.sampling) {
    core.pci_irqmask |= PCI_INT_IR_SMPINT;
    cx_write(MO_DDS_IO, 0x33F286 * ir_samplerate); /* samplerate */
    cx_write(MO_DDSCFG_IO, 0x5); /* enable */
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __cx88_ir_stop(priv: *mut c_void) {
    static void __cx88_ir_stop(void *priv)
    {
    struct cx88_core *core = priv;
    struct cx88_IR *ir;
    if (!core || !core.ir)
    return;
    ir = core.ir;
    if (ir.sampling) {
    cx_write(MO_DDSCFG_IO, 0x0);
    core.pci_irqmask &= ~PCI_INT_IR_SMPINT;
    }
    if (ir.polling)
    hrtimer_cancel(&ir.timer);
    }
#[no_mangle]
pub unsafe extern "C" fn cx88_ir_start(core: *mut cx88_core) -> c_int {
    int cx88_ir_start(struct cx88_core *core)
    {
    if (core.ir.users)
    return __cx88_ir_start(core);
    return 0;
    }
    EXPORT_SYMBOL(cx88_ir_start);
#[no_mangle]
pub unsafe extern "C" fn cx88_ir_stop(core: *mut cx88_core) {
    void cx88_ir_stop(struct cx88_core *core)
    {
    if (core.ir.users)
    __cx88_ir_stop(core);
    }
    EXPORT_SYMBOL(cx88_ir_stop);
#[no_mangle]
unsafe extern "C" fn cx88_ir_open(rc: *mut rc_dev) -> c_int {
    static int cx88_ir_open(struct rc_dev *rc)
    {
    struct cx88_core *core = rc.priv;
    core.ir.users++;
    return __cx88_ir_start(core);
    }
#[no_mangle]
unsafe extern "C" fn cx88_ir_close(rc: *mut rc_dev) {
    static void cx88_ir_close(struct rc_dev *rc)
    {
    struct cx88_core *core = rc.priv;
    core.ir.users--;
    if (!core.ir.users)
    __cx88_ir_stop(core);
    }
// ----------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn cx88_ir_init(core: *mut cx88_core, pci: *mut pci_dev) -> c_int {
    int cx88_ir_init(struct cx88_core *core, struct pci_dev *pci)
    {
    struct cx88_IR *ir;
    struct rc_dev *dev;
    char *ir_codes = core::ptr::null_mut();
    let mut rc_proto: u64 = RC_PROTO_BIT_OTHER;
    let mut err: c_int = -ENOMEM;
    u32 hardware_mask = 0;	/* For devices with a hardware mask, when
// used with a full-code IR table
//
    ir = kzalloc_obj(*ir);
    dev = rc_allocate_device(RC_DRIVER_IR_RAW);
    if (!ir || !dev)
    goto err_out_free;
    ir.dev = dev;
// detect & configure
    switch (core.boardnr) {
    case CX88_BOARD_DNTV_LIVE_DVB_T:
    case CX88_BOARD_KWORLD_DVB_T:
    case CX88_BOARD_KWORLD_DVB_T_CX22702:
    ir_codes = RC_MAP_DNTV_LIVE_DVB_T;
    ir.gpio_addr = MO_GP1_IO;
    ir.mask_keycode = 0x1f;
    ir.mask_keyup = 0x60;
    ir.polling = 50; /* ms */
    break;
    case CX88_BOARD_TERRATEC_CINERGY_1400_DVB_T1:
    ir_codes = RC_MAP_CINERGY_1400;
    ir.sampling = 0xeb04; /* address */
    break;
    case CX88_BOARD_HAUPPAUGE:
    case CX88_BOARD_HAUPPAUGE_DVB_T1:
    case CX88_BOARD_HAUPPAUGE_NOVASE2_S1:
    case CX88_BOARD_HAUPPAUGE_NOVASPLUS_S1:
    case CX88_BOARD_HAUPPAUGE_HVR1100:
    case CX88_BOARD_HAUPPAUGE_HVR3000:
    case CX88_BOARD_HAUPPAUGE_HVR4000:
    case CX88_BOARD_HAUPPAUGE_HVR4000LITE:
    case CX88_BOARD_PCHDTV_HD3000:
    case CX88_BOARD_PCHDTV_HD5500:
    case CX88_BOARD_HAUPPAUGE_IRONLY:
    ir_codes = RC_MAP_HAUPPAUGE;
    ir.sampling = 1;
    break;
    case CX88_BOARD_WINFAST_DTV2000H:
    case CX88_BOARD_WINFAST_DTV2000H_J:
    case CX88_BOARD_WINFAST_DTV1800H:
    case CX88_BOARD_WINFAST_DTV1800H_XC4000:
    case CX88_BOARD_WINFAST_DTV2000H_PLUS:
    ir_codes = RC_MAP_WINFAST;
    ir.gpio_addr = MO_GP0_IO;
    ir.mask_keycode = 0x8f8;
    ir.mask_keyup = 0x100;
    ir.polling = 50; /* ms */
    break;
    case CX88_BOARD_WINFAST2000XP_EXPERT:
    case CX88_BOARD_WINFAST_DTV1000:
    case CX88_BOARD_WINFAST_TV2000_XP_GLOBAL:
    case CX88_BOARD_WINFAST_TV2000_XP_GLOBAL_6F36:
    case CX88_BOARD_WINFAST_TV2000_XP_GLOBAL_6F43:
    ir_codes = RC_MAP_WINFAST;
    ir.gpio_addr = MO_GP0_IO;
    ir.mask_keycode = 0x8f8;
    ir.mask_keyup = 0x100;
    ir.polling = 1; /* ms */
    break;
    case CX88_BOARD_IODATA_GVBCTV7E:
    ir_codes = RC_MAP_IODATA_BCTV7E;
    ir.gpio_addr = MO_GP0_IO;
    ir.mask_keycode = 0xfd;
    ir.mask_keydown = 0x02;
    ir.polling = 5; /* ms */
    break;
    case CX88_BOARD_PROLINK_PLAYTVPVR:
    case CX88_BOARD_PIXELVIEW_PLAYTV_ULTRA_PRO:
//
// It seems that this hardware is paired with NEC extended
// address 0x866b. So, unfortunately, its usage with other
// IR's with different address won't work. Still, there are
// other IR's from the same manufacturer that works, like the
// 002-T mini RC, provided with newer PV hardware
//
    ir_codes = RC_MAP_PIXELVIEW_MK12;
    rc_proto = RC_PROTO_BIT_NECX;
    ir.gpio_addr = MO_GP1_IO;
    ir.mask_keyup = 0x80;
    ir.polling = 10; /* ms */
    hardware_mask = 0x3f;	/* Hardware returns only 6 bits from command part */
    break;
    case CX88_BOARD_PROLINK_PV_8000GT:
    case CX88_BOARD_PROLINK_PV_GLOBAL_XTREME:
    ir_codes = RC_MAP_PIXELVIEW_NEW;
    ir.gpio_addr = MO_GP1_IO;
    ir.mask_keycode = 0x3f;
    ir.mask_keyup = 0x80;
    ir.polling = 1; /* ms */
    break;
    case CX88_BOARD_KWORLD_LTV883:
    ir_codes = RC_MAP_PIXELVIEW;
    ir.gpio_addr = MO_GP1_IO;
    ir.mask_keycode = 0x1f;
    ir.mask_keyup = 0x60;
    ir.polling = 1; /* ms */
    break;
    case CX88_BOARD_ADSTECH_DVB_T_PCI:
    ir_codes = RC_MAP_ADSTECH_DVB_T_PCI;
    ir.gpio_addr = MO_GP1_IO;
    ir.mask_keycode = 0xbf;
    ir.mask_keyup = 0x40;
    ir.polling = 50; /* ms */
    break;
    case CX88_BOARD_MSI_TVANYWHERE_MASTER:
    ir_codes = RC_MAP_MSI_TVANYWHERE;
    ir.gpio_addr = MO_GP1_IO;
    ir.mask_keycode = 0x1f;
    ir.mask_keyup = 0x40;
    ir.polling = 1; /* ms */
    break;
    case CX88_BOARD_AVERTV_303:
    case CX88_BOARD_AVERTV_STUDIO_303:
    ir_codes         = RC_MAP_AVERTV_303;
    ir.gpio_addr    = MO_GP2_IO;
    ir.mask_keycode = 0xfb;
    ir.mask_keydown = 0x02;
    ir.polling      = 50; /* ms */
    break;
    case CX88_BOARD_OMICOM_SS4_PCI:
    case CX88_BOARD_SATTRADE_ST4200:
    case CX88_BOARD_TBS_8920:
    case CX88_BOARD_TBS_8910:
    case CX88_BOARD_PROF_7300:
    case CX88_BOARD_PROF_7301:
    case CX88_BOARD_PROF_6200:
    ir_codes = RC_MAP_TBS_NEC;
    ir.sampling = 0xff00; /* address */
    break;
    case CX88_BOARD_TEVII_S464:
    case CX88_BOARD_TEVII_S460:
    case CX88_BOARD_TEVII_S420:
    ir_codes = RC_MAP_TEVII_NEC;
    ir.sampling = 0xff00; /* address */
    break;
    case CX88_BOARD_DNTV_LIVE_DVB_T_PRO:
    ir_codes         = RC_MAP_DNTV_LIVE_DVBT_PRO;
    ir.sampling     = 0xff00; /* address */
    break;
    case CX88_BOARD_NORWOOD_MICRO:
    ir_codes         = RC_MAP_NORWOOD;
    ir.gpio_addr    = MO_GP1_IO;
    ir.mask_keycode = 0x0e;
    ir.mask_keyup   = 0x80;
    ir.polling      = 50; /* ms */
    break;
    case CX88_BOARD_NPGTECH_REALTV_TOP10FM:
    ir_codes         = RC_MAP_NPGTECH;
    ir.gpio_addr    = MO_GP0_IO;
    ir.mask_keycode = 0xfa;
    ir.polling      = 50; /* ms */
    break;
    case CX88_BOARD_PINNACLE_PCTV_HD_800i:
    ir_codes         = RC_MAP_PINNACLE_PCTV_HD;
    ir.sampling     = 1;
    break;
    case CX88_BOARD_POWERCOLOR_REAL_ANGEL:
    ir_codes         = RC_MAP_POWERCOLOR_REAL_ANGEL;
    ir.gpio_addr    = MO_GP2_IO;
    ir.mask_keycode = 0x7e;
    ir.polling      = 100; /* ms */
    break;
    case CX88_BOARD_TWINHAN_VP1027_DVBS:
    ir_codes         = RC_MAP_TWINHAN_VP1027_DVBS;
    ir.sampling     = 0xff00; /* address */
    break;
    }
    if (!ir_codes) {
    err = -ENODEV;
    goto err_out_free;
    }
//
// The usage of mask_keycode were very convenient, due to several
// reasons. Among others, the scancode tables were using the scancode
// as the index elements. So, the less bits it was used, the smaller
// the table were stored. After the input changes, the better is to use
// the full scancodes, since it allows replacing the IR remote by
// another one. Unfortunately, there are still some hardware, like
// Pixelview Ultra Pro, where only part of the scancode is sent via
// GPIO. So, there's no way to get the full scancode. Due to that,
// hardware_mask were introduced here: it represents those hardware
// that has such limits.
//
    if (hardware_mask && !ir.mask_keycode)
    ir.mask_keycode = hardware_mask;
// init input device
    snprintf(ir.name, sizeof(ir.name), "cx88 IR (%s)", core.board.name);
    snprintf(ir.phys, sizeof(ir.phys), "pci-%s/ir0", pci_name(pci));
    dev.device_name = ir.name;
    dev.input_phys = ir.phys;
    dev.input_id.bustype = BUS_PCI;
    dev.input_id.version = 1;
    if (pci.subsystem_vendor) {
    dev.input_id.vendor = pci.subsystem_vendor;
    dev.input_id.product = pci.subsystem_device;
    } else {
    dev.input_id.vendor = pci.vendor;
    dev.input_id.product = pci.device;
    }
    dev.dev.parent = &pci.dev;
    dev.map_name = ir_codes;
    dev.driver_name = MODULE_NAME;
    dev.priv = core;
    dev.open = cx88_ir_open;
    dev.close = cx88_ir_close;
    dev.scancode_mask = hardware_mask;
    if (ir.sampling) {
    dev.timeout = MS_TO_US(10); /* 10 ms */
    } else {
    dev.driver_type = RC_DRIVER_SCANCODE;
    dev.allowed_protocols = rc_proto;
    }
    ir.core = core;
    core.ir = ir;
// all done
    err = rc_register_device(dev);
    if (err)
    goto err_out_free;
    return 0;
    err_out_free:
    rc_free_device(dev);
    core.ir = core::ptr::null_mut();
    kfree(ir);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn cx88_ir_fini(core: *mut cx88_core) -> c_int {
    int cx88_ir_fini(struct cx88_core *core)
    {
    struct cx88_IR *ir = core.ir;
// skip detach on non attached boards
    if (!ir)
    return 0;
    rc_unregister_device(ir.dev);
    cx88_ir_stop(core);
    rc_free_device(ir.dev);
    kfree(ir);
// done
    core.ir = core::ptr::null_mut();
    return 0;
    }
// ----------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn cx88_ir_irq(core: *mut cx88_core) {
    void cx88_ir_irq(struct cx88_core *core)
    {
    struct cx88_IR *ir = core.ir;
    u32 samples;
    unsigned int todo, bits;
    let mut ev: ir_raw_event = {};
    if (!ir || !ir.sampling)
    return;
//
// Samples are stored in a 32 bit register, oldest sample in
// the msb. A set bit represents space and an unset bit
// represents a pulse.
//
    samples = cx_read(MO_SAMPLE_IO);
    if (samples == 0xff && ir.dev.idle)
    return;
    for (todo = 32; todo > 0; todo -= bits) {
    ev.pulse = samples & 0x80000000 ? false : true;
    bits = min(todo, 32U - fls(ev.pulse ? samples : ~samples));
    ev.duration = (bits * (USEC_PER_SEC / 1000)) / ir_samplerate;
    ir_raw_event_store_with_filter(ir.dev, &ev);
    samples <<= bits;
    }
    ir_raw_event_handle(ir.dev);
    }
    static int get_key_pvr2000(struct IR_i2c *ir, enum rc_proto *protocol,
    u32 *scancode, u8 *toggle)
    {
    int flags, code;
// poll IR chip
    flags = i2c_smbus_read_byte_data(ir.c, 0x10);
    if (flags < 0) {
    dprintk("read error\n");
    return 0;
    }
// key pressed ?
    if (0 == (flags & 0x80))
    return 0;
// read actual key code
    code = i2c_smbus_read_byte_data(ir.c, 0x00);
    if (code < 0) {
    dprintk("read error\n");
    return 0;
    }
    dprintk("IR Key/Flags: (0x%02x/0x%02x)\n",
    code & 0xff, flags & 0xff);
// protocol = RC_PROTO_UNKNOWN;
// scancode = code & 0xff;
// toggle = 0;
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn cx88_i2c_init_ir(core: *mut cx88_core) {
    void cx88_i2c_init_ir(struct cx88_core *core)
    {
    struct i2c_board_info info;
    static const unsigned short default_addr_list[] = {
    0x18, 0x33, 0x6b, 0x71,
    I2C_CLIENT_END
    };
    static const unsigned short pvr2000_addr_list[] = {
    0x18, 0x1a,
    I2C_CLIENT_END
    };
    const unsigned short *addr_list = default_addr_list;
    const unsigned short *addrp;
// Instantiate the IR receiver device, if present
    if (core.i2c_rc != 0)
    return;
    memset(&info, 0, sizeof(struct i2c_board_info));
    strscpy(info.type, "ir_video", I2C_NAME_SIZE);
    switch (core.boardnr) {
    case CX88_BOARD_LEADTEK_PVR2000:
    addr_list = pvr2000_addr_list;
    core.init_data.name = "cx88 Leadtek PVR 2000 remote";
    core.init_data.type = RC_PROTO_BIT_UNKNOWN;
    core.init_data.get_key = get_key_pvr2000;
    core.init_data.ir_codes = RC_MAP_EMPTY;
    break;
    }
//
// We can't call i2c_new_scanned_device() because it uses
// quick writes for probing and at least some RC receiver
// devices only reply to reads.
// Also, Hauppauge XVR needs to be specified, as address 0x71
// conflicts with another remote type used with saa7134
//
    for (addrp = addr_list; *addrp != I2C_CLIENT_END; addrp++) {
    info.platform_data = core::ptr::null_mut();
    memset(&core.init_data, 0, sizeof(core.init_data));
    if (*addrp == 0x71) {
// Hauppauge Z8F0811
    strscpy(info.type, "ir_z8f0811_haup", I2C_NAME_SIZE);
    core.init_data.name = core.board.name;
    core.init_data.ir_codes = RC_MAP_HAUPPAUGE;
    core.init_data.type = RC_PROTO_BIT_RC5 |
    RC_PROTO_BIT_RC6_MCE | RC_PROTO_BIT_RC6_6A_32;
    core.init_data.internal_get_key_func = IR_KBD_GET_KEY_HAUP_XVR;
    info.platform_data = &core.init_data;
    }
    if (i2c_smbus_xfer(&core.i2c_adap, *addrp, 0,
    I2C_SMBUS_READ, 0,
    I2C_SMBUS_QUICK, core::ptr::null_mut()) >= 0) {
    info.addr = *addrp;
    i2c_new_client_device(&core.i2c_adap, &info);
    break;
    }
    }
    }
// ----------------------------------------------------------------------
    MODULE_AUTHOR("Gerd Knorr, Pavel Machek, Chris Pascoe");
    MODULE_DESCRIPTION("input driver for cx88 GPIO-based IR remote controls");
    MODULE_LICENSE("GPL");
