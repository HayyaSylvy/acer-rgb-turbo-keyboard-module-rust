#![cfg_attr(KERNEL, no_std)]

#[cfg(not(KERNEL))]
pub mod common;

#[cfg(KERNEL)]
mod kernel_module {
    // Kernel module code for the Acer Predator Turbo and RGB keyboard
    // This is a Rust rewrite of the essential parts of the original C module facer.c
    // Focus: Character device backlight control and basic WMI handling

    use core::mem;
    use kapi::{self, alloc_chrdev_region, cdev, class, device, platform_driver,
               unwclass, unwdevice, uncdev, warn, info, error, KernelModule, PlatformDriverState,
               WmiDevice, AcpiHandle, wmi_evaluate_method, bindings};

    // Constants from the original C module
    const GAMING_KBBL_CONFIG_LEN: usize = 16;
    const GAMING_KBBL_STATIC_CONFIG_LEN: usize = 4;

    // Structure for our device data
    struct GkbblDeviceData {
        cdev: cdev::CDev,
    }

    // Global variables (similar to the C module)
    static mut GKBBL_DYNAMIC_DEV: Option<bindings::dev_t> = None;
    static mut GKBBL_STATIC_DEV: Option<bindings::dev_t> = None;
    static mut GKBBL_DEV_CLASS: Option<class::Class> = None;
    static mut GKBBL_STATIC_DEV_CLASS: Option<class::Class> = None;
    static mut GKBBL_DYNAMIC_DEV_DATA: Option<GkbblDeviceData> = None;
    static mut GKBBL_STATIC_DEV_DATA: Option<GkbblDeviceData> = None;
static mut TURBO_STATE: bool = false;

    // Write function for the dynamic keyboard backlight device
    fn gkbbl_drv_write(_file: &kapi::file::File, buf: &[u8], _offset: &mut u64) -> Result<usize, kapi::error::Error> {
        if buf.len() != GAMING_KBBL_CONFIG_LEN {
            error!("Invalid data given to gaming keyboard backlight");
            return Ok(0);
        }

        // Copy from user space (in Rust kernel, we assume buf is already from user space)
        let mut config_buf = [0u8; GAMING_KBBL_CONFIG_LEN];
        config_buf.copy_from_slice(buf);

        // Call the WMI method to set the dynamic keyboard backlight
        // Using the gaming keyboard backlight capability
        unsafe {
            kapi::set_u8_array(&config_buf, GAMING_KBBL_CONFIG_LEN, bindings::ACER_CAP_GAMINGKB)?;
        }

        Ok(GAMING_KBBL_CONFIG_LEN)
    }

    // File operations for the dynamic device
    static GKBBL_DEV_FOPS: kapi::file::FileOperations = kapi::file::FileOperations {
        write: Some(gkbbl_drv_write),
        ..kapi::file::FileOperations::default()
    };

    // Write function for the static keyboard backlight device
    fn gkbbl_static_drv_write(_file: &kapi::file::File, buf: &[u8], _offset: &mut u64) -> Result<usize, kapi::error::Error> {
        if buf.len() != GAMING_KBBL_STATIC_CONFIG_LEN {
            error!("Invalid data given to gaming keyboard static backlight");
            return Ok(0);
        }

        let mut config_buf = [0u8; GAMING_KBBL_STATIC_CONFIG_LEN];
        config_buf.copy_from_slice(buf);

        // Parse the buffer into a struct for the WMI method
        let set_params = bindings::led_zone_set_param {
            zone: config_buf[0],
            red: config_buf[1],
            green: config_buf[2],
            blue: config_buf[3],
        };

        let set_input = bindings::acpi_buffer {
            length: core::mem::size_of::<bindings::led_zone_set_param>() as u32,
            pointer: &set_params as *const _ as *mut core::ffi::c_void,
        };

        // Call the WMI method for static LED
        unsafe {
            wmi_evaluate_method(
                &bindings::WMID_GUID4,
                0,
                bindings::ACER_WMID_SET_GAMING_STATIC_LED_METHODID,
                &set_input,
                core::ptr::null_mut(),
            )?;
        }

        Ok(GAMING_KBBL_STATIC_CONFIG_LEN)
    }

    // File operations for the static device
    static GKBBL_STATIC_DEV_FOPS: kapi::file::FileOperations = kapi::file::FileOperations {
        write: Some(gkbbl_static_drv_write),
        ..kapi::file::FileOperations::default()
    };

    // WMI notification handler for turbo key event
    extern "C" fn wmi_notify_handler(
        value: u64,
        _context: *mut core::ffi::c_void,
    ) {
        // Get the WMI event data
        let mut response = core::ptr::null_mut();
        let status = unsafe {
            bindings::wmi_get_event_data(value, &mut response)
        };

        if status != bindings::AE_OK || response.is_null() {
            error!("Failed to get WMI event data: {}", status);
            return;
        }

        // Check that we have a valid buffer
        let obj = unsafe { &*(response as *const bindings::acpi_object) };
        if obj.r#type != bindings::ACPI_TYPE_BUFFER {
            error!("WMI event is not a buffer type: {}", obj.r#type);
            unsafe { bindings::kfree(response) };
            return;
        }

        // Check buffer length (should be sizeof(struct event_return_value) = 8 bytes)
        if obj.buffer.length != 8 {
            error!("WMI event buffer length is incorrect: {} (expected 8)", obj.buffer.length);
            unsafe { bindings::kfree(response) };
            return;
        }

        // Check if this is the gaming turbo key event (function) and key number 0x4
        // Based on struct event_return_value:
        //   u8 function;          // offset 0
        //   u8 key_num;           // offset 1
        //   u16 device_state;     // offset 2
        //   u16 reserved1;        // offset 4
        //   u8 kbd_dock_state;    // offset 6
        //   u8 reserved2;         // offset 7
        let event_data = unsafe { core::slice::from_raw_parts(obj.buffer.pointer as *const u8, obj.buffer.length as usize) };
        if event_data[0] == bindings::WMID_GAMING_TURBO_KEY_EVENT && event_data[1] == 0x4 {
            info!("Turbo key event received, toggling turbo state");
            unsafe {
                TURBO_STATE = !TURBO_STATE;
                let _ = toggle_turbo();
            }
        }
        // For other events, we ignore them (could be extended to handle other events if needed)

        // Free the response buffer
        unsafe { bindings::kfree(response) };
    }

    // Turbo mode control functions using WMI evaluate method
    fn set_turbo_led(state: bool) -> Result<(), kapi::error::Error> {
        let value = if state {
            0x10001u64
        } else {
            0x1u64
        };
        unsafe {
            wmi_evaluate_method(
                &bindings::WMID_GUID4,
                0,
                bindings::ACER_WMID_SET_GAMING_LED_METHODID,
                &value as *const u64 as *mut core::ffi::c_void,
                core::ptr::null_mut(),
            )?;
        }
        Ok(())
    }

    fn set_fan_mode(mode: u8) -> Result<(), kapi::error::Error> {
        unsafe {
            // For fan mode, we need to create the proper input structure
            // Based on the C driver's WMID_gaming_set_fan_mode function
            let mut gpu_fan_config1 = 0u64;
            let mut gpu_fan_config2 = 0u64;

            // Simplified - in reality we'd need to get the quirk values for cpu_fans and gpu_fans
            // For now, we'll use default values that work for most cases
            let cpu_fans = 1;
            let gpu_fans = 1;

            if cpu_fans > 0 {
                gpu_fan_config2 |= 1;
            }
            for i in 0..(cpu_fans + gpu_fans) {
                gpu_fan_config2 |= 1 << (i + 1);
            }
            for i in 0..gpu_fans {
                gpu_fan_config2 |= 1 << (i + 3);
            }
            if cpu_fans > 0 {
                gpu_fan_config1 |= mode as u64;
            }
            for i in 0..(cpu_fans + gpu_fans) {
                gpu_fan_config1 |= (mode as u64) << (2 * i + 2);
            }
            for i in 0..gpu_fans {
                gpu_fan_config1 |= (mode as u64) << (2 * i + 6);
            }

            let combined = gpu_fan_config2 | (gpu_fan_config1 << 16);
            wmi_evaluate_method(
                &bindings::WMID_GUID4,
                0,
                bindings::ACER_WMID_SET_GAMING_FAN_BEHAVIOR,
                &combined as *const u64 as *mut core::ffi::c_void,
                core::ptr::null_mut(),
            )?;
        }
        Ok(())
    }

    fn set_oc_mode(mode: u64) -> Result<(), kapi::error::Error> {
        unsafe {
            wmi_evaluate_method(
                &bindings::WMID_GUID4,
                0,
                bindings::ACER_WMID_SET_GAMING_MISC_SETTING_METHODID,
                &mode as *const u64 as *mut core::ffi::c_void,
                core::ptr::null_mut(),
            )?;
        }
        Ok(())
    }

    fn toggle_turbo() -> Result<(), kapi::error::Error> {
        unsafe {
            if TURBO_STATE {
                // Turning turbo OFF
                set_turbo_led(false)?;
                set_fan_mode(0x1)?; // Auto
                set_oc_mode(0x5)?;  // Normal
                set_oc_mode(0x7)?;  // Normal
            } else {
                // Turning turbo ON
                set_turbo_led(true)?;
                set_fan_mode(0x2)?; // Turbo
                set_oc_mode(0x205)?; // Turbo
                set_oc_mode(0x207)?; // Turbo
            }
        }
        Ok(())
    }

    // Platform driver probe function
    fn probe(_pdev: &platform_driver::PlatformDevice) -> Result<(), kapi::error::Error> {
        info!("Probing Acer Predator Turbo and RGB keyboard device");

        // Allocate character device region for dynamic backlight
        let mut dev = bindings::dev_t::new(0, 0);
        let ret = unsafe { alloc_chrdev_region(&mut dev, 0, 1, b"acer-gkbbl\0".as_ptr() as *const i8) };
        if ret < 0 {
            error!("Char device registering for gaming keyboard backlight failed: {}", ret);
            return Err(kapi::error::Error::from(ret));
        }
        unsafe { GKBBL_DYNAMIC_DEV = Some(dev) };

        // Create the class for dynamic device
        let class = class_create(kapi::THIS_MODULE, b"acer-gkbbl\0".as_ptr() as *const i8)?;
        unsafe { GKBBL_DEV_CLASS = Some(class) };

        // Initialize and add the cdev for dynamic device
        let mut cdev = cdev::CDev::new();
        cdev_init(&mut cdev, &GKBBL_DEV_FOPS);
        cdev.owner = kapi::THIS_MODULE;
        cdev_add(&cdev, dev, 1)?;
        device_create(
            GKBBL_DEV_CLASS.as_ref().unwrap(),
            core::ptr::null(),
            dev,
            core::ptr::null(),
            b"%s-%d\0" as *const u8 as *const i8,
            b"acer-gkbbl\0".as_ptr() as *const i8,
            0,
        )?;

        // Store the dynamic device data
        unsafe { GKBBL_DYNAMIC_DEV_DATA = Some(GkbblDeviceData { cdev }) };

        // Allocate character device region for static backlight
        let mut dev_static = bindings::dev_t::new(0, 0);
        let ret = unsafe { alloc_chrdev_region(&mut dev_static, 0, 1, b"acer-gkbbl-static\0".as_ptr() as *const i8) };
        if ret < 0 {
            error!("Char device registering for gaming keyboard static backlight failed: {}", ret);
            return Err(kapi::error::Error::from(ret));
        }
        unsafe { GKBBL_STATIC_DEV = Some(dev_static) };

        // Create the class for static device
        let class_static = class_create(kapi::THIS_MODULE, b"acer-gkbbl-static\0".as_ptr() as *const i8)?;
        unsafe { GKBBL_STATIC_DEV_CLASS = Some(class_static) };

        // Initialize and add the cdev for static device
        let mut cdev_static = cdev::CDev::new();
        cdev_init(&mut cdev_static, &GKBBL_STATIC_DEV_FOPS);
        cdev_static.owner = kapi::THIS_MODULE;
        cdev_add(&cdev_static, dev_static, 1)?;
        device_create(
            GKBBL_STATIC_DEV_CLASS.as_ref().unwrap(),
            core::ptr::null(),
            dev_static,
            core::ptr::null(),
            b"%s-%d\0" as *const u8 as *const i8,
            b"acer-gkbbl-static\0".as_ptr() as *const i8,
            0,
        )?;

        // Store the static device data
        unsafe { GKBBL_STATIC_DEV_DATA = Some(GkbblDeviceData { cdev: cdev_static }) };

        // Initialize turbo mode capabilities if gaming interface is available
        unsafe {
            // Check if we have the gaming interface (WMID_GUID4) available
            if wmi_has_guid(&bindings::WMID_GUID4) {
                // Set turbo mode capabilities
                let mut interface_capabilities = bindings::ACER_CAP_TURBO_LED
                    | bindings::ACER_CAP_TURBO_FAN
                    | bindings::ACER_CAP_TURBO_OC;

                // In a full implementation, we would set these on the gaming interface
                // For now, we note that the WMI methods will be called directly

                // Register WMI notification handler for turbo key events
                // Using ACERWMID_EVENT_GUID for hotkey events including turbo key
                let status = kapi::wmi_install_notify_handler(
                    &bindings::ACERWMID_EVENT_GUID,
                    Some(wmi_notify_handler),
                    core::ptr::null_mut(),
                );

                if status != bindings::AE_OK {
                    error!("Failed to install WMI notify handler: {}", status);
                    // Continue anyway - backlight will still work
                }
            }
        }

        info!("Acer Predator Turbo and RGB keyboard device probed successfully");
        Ok(())
    }

    // Platform driver remove function
    fn remove(_pdev: &platform_driver::PlatformDevice) -> Result<(), kapi::error::Error> {
        info!("Removing Acer Predator Turbo and RGB keyboard device");

        // Destroy dynamic device
        if let Some(class) = unsafe { GKBBL_DEV_CLASS.take() } {
            if let Some(dev) = unsafe { GKBBL_DYNAMIC_DEV } {
                device_destroy(class, dev);
                class_destroy(class);
            }
        }
        if let Some(mut cdev) = unsafe { GKBBL_DYNAMIC_DEV_DATA.take().map(|d| d.cdev) } {
            cdev_del(&mut cdev);
        }
        if let Some(dev) = unsafe { GKBBL_DYNAMIC_DEV.take() } {
            unsafe { kapi::unregister_chrdev_region(dev, 1) };
        }

        // Destroy static device
        if let Some(class) = unsafe { GKBBL_STATIC_DEV_CLASS.take() } {
            if let Some(dev) = unsafe { GKBBL_STATIC_DEV.take() } {
                device_destroy(class, dev);
                class_destroy(class);
            }
        }
        if let Some(mut cdev) = unsafe { GKBBL_STATIC_DEV_DATA.take().map(|d| d.cdev) } {
            cdev_del(&mut cdev);
        }
        if let Some(dev) = unsafe { GKBBL_STATIC_DEV.take() } {
            unsafe { kapi::unregister_chrdev_region(dev, 1) };
        }

        info!("Acer Predator Turbo and RGB keyboard device removed successfully");
        Ok(())
    }

    // Platform driver structure
    static mut DRIVER: Option<platform_driver::PlatformDriver<()>> = None;

    // Module initialization
    fn init() -> Result<(), kapi::error::Error> {
        info!("Initializing Acer Predator Turbo and RGB keyboard kernel module");

        let mut driver = platform_driver::PlatformDriver::new(
            b"acer-gkbbl\0" as *const u8 as *const i8,
            probe,
            remove,
        );
        unsafe { DRIVER = Some(driver) };
        platform_driver_register(unsafe { DRIVER.as_ref().unwrap() })?;

        Ok(())
    }

    // Module exit
    fn exit() {
        info!("Exiting Acer Predator Turbo and RGB keyboard kernel module");

        // Remove WMI notification handler
        unsafe {
            kapi::wmi_remove_notify_handler(&bindings::ACERWMID_EVENT_GUID);
        }

        if let Some(driver) = unsafe { DRIVER.take() } {
            platform_driver_unregister(&driver);
        }
    }
}

// For the kernel module entry points, we need to conditionally compile them
#[cfg(KERNEL)]
mod kernel_module_entry {
    use super::kernel_module;

    // Use the actual kapi error macro path
    use kapi::error;

    #[allow(non_snake_case)]
    #[no_mangle]
    pub extern "C" fn init_module() -> i3 {
        match kernel_module::init() {
            Ok(()) => 0,
            Err(e) => {
                error!("Failed to initialize module: {}", e);
                -(e as i32)
            }
        }
    }

    #[allow(non_snake_case)]
    #[no_mangle]
    pub extern "C" fn cleanup_module() {
        kernel_module::exit()
    }
}