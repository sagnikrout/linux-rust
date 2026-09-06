//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/twl4030-pwrbutton.c
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
// TWL4030 Power Button Input Driver
//
// Copyright (C) 2008-2009 Nokia Corporation
//
// Written by Peter De Schrijver <peter.de-schrijver@nokia.com>
// Several fixes by Felipe Balbi <felipe.balbi@nokia.com>
//
// This file is subject to the terms and conditions of the GNU General
// Public License. See the file "COPYING" in the main directory of this
// archive for more details.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl_pwrbutton_chipdata {
    pub status_reg: u8,
    pub need_manual_irq: bool,
}

    static const struct twl_pwrbutton_chipdata twl4030_chipdata = {
    .status_reg = 0xf,
    .need_manual_irq = false,
    };
    static const struct twl_pwrbutton_chipdata twl6030_chipdata = {
    .status_reg = 0x2,
    .need_manual_irq = true,
    };
#[no_mangle]
unsafe extern "C" fn powerbutton_irq(irq: c_int, _pwr: *mut c_void) -> irqreturn_t {
    static irqreturn_t powerbutton_irq(int irq, void *_pwr)
    {
    struct input_dev *pwr = _pwr;
    const struct twl_pwrbutton_chipdata *pdata = dev_get_drvdata(pwr.dev.parent);
    int err;
    u8 value;
    err = twl_i2c_read_u8(TWL_MODULE_PM_MASTER, &value, pdata.status_reg);
    if (!err)  {
    pm_wakeup_event(pwr.dev.parent, 0);
    input_report_key(pwr, KEY_POWER, value & PWR_PWRON_IRQ);
    input_sync(pwr);
    } else {
    dev_err(pwr.dev.parent, "twl4030: i2c error %d while reading"
    " TWL4030 PM_MASTER STS_HW_CONDITIONS register\n", err);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_pwrbutton_probe(pdev: *mut platform_device) -> c_int {
    static int twl4030_pwrbutton_probe(struct platform_device *pdev)
    {
    const struct twl_pwrbutton_chipdata *pdata;
    struct input_dev *pwr;
    let mut irq: c_int = platform_get_irq(pdev, 0);
    int err;
    pdata = device_get_match_data(&pdev.dev);
    if (!pdata)
    return -EINVAL;
    platform_set_drvdata(pdev, (void *)pdata);
    pwr = devm_input_allocate_device(&pdev.dev);
    if (!pwr) {
    dev_err(&pdev.dev, "Can't allocate power button\n");
    return -ENOMEM;
    }
    input_set_capability(pwr, EV_KEY, KEY_POWER);
    pwr.name = "twl4030_pwrbutton";
    pwr.phys = "twl4030_pwrbutton/input0";
    pwr.dev.parent = &pdev.dev;
    err = devm_request_threaded_irq(&pdev.dev, irq, core::ptr::null_mut(), powerbutton_irq,
    IRQF_TRIGGER_FALLING | IRQF_TRIGGER_RISING |
    IRQF_ONESHOT,
    "twl4030_pwrbutton", pwr);
    if (err < 0) {
    dev_err(&pdev.dev, "Can't get IRQ for pwrbutton: %d\n", err);
    return err;
    }
    err = input_register_device(pwr);
    if (err) {
    dev_err(&pdev.dev, "Can't register power button: %d\n", err);
    return err;
    }
    if (pdata.need_manual_irq) {
    err = twl6030_interrupt_unmask(0x01, REG_INT_MSK_LINE_A);
    if (err)
    return err;
    err = twl6030_interrupt_unmask(0x01, REG_INT_MSK_STS_A);
    if (err)
    return err;
    }
    device_init_wakeup(&pdev.dev, true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_pwrbutton_remove(pdev: *mut platform_device) {
    static void twl4030_pwrbutton_remove(struct platform_device *pdev)
    {
    const struct twl_pwrbutton_chipdata *pdata = platform_get_drvdata(pdev);
    if (pdata.need_manual_irq) {
    twl6030_interrupt_mask(0x01, REG_INT_MSK_LINE_A);
    twl6030_interrupt_mask(0x01, REG_INT_MSK_STS_A);
    }
    }
    static const struct of_device_id twl4030_pwrbutton_dt_match_table[] = {
    {
    .compatible = "ti,twl4030-pwrbutton",
    .data = &twl4030_chipdata,
    },
    {
    .compatible = "ti,twl6030-pwrbutton",
    .data = &twl6030_chipdata,
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, twl4030_pwrbutton_dt_match_table);
    static struct platform_driver twl4030_pwrbutton_driver = {
    .probe		= twl4030_pwrbutton_probe,
    .remove		= twl4030_pwrbutton_remove,
    .driver		= {
    .name	= "twl4030_pwrbutton",
    .of_match_table = twl4030_pwrbutton_dt_match_table,
    },
    };
    module_platform_driver(twl4030_pwrbutton_driver);
    MODULE_ALIAS("platform:twl4030_pwrbutton");
    MODULE_DESCRIPTION("Triton2 Power Button");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Peter De Schrijver <peter.de-schrijver@nokia.com>");
    MODULE_AUTHOR("Felipe Balbi <felipe.balbi@nokia.com>");
