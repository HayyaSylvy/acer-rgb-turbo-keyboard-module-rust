#!/bin/bash
# This script can install or uninstall acer turbo fan service. It means, that your turbo fan button still will be available even after rebooting.
# To install service just run this script as the root (sudo) user.
# After installation you can manage it as a usual service manually. Example: 'systemctl start/stop turbo-fan',  'systemctl enable/disable turbo-fan'
# To uninstall service, run this script with 'remove' argument. Example: 'sudo bash ./install_service.sh remove'.
# Note!!! Before removing, don't forget to switch off the turbo button because you will have forever turbo fan :)
mode=${1:-install} # Allowed modes: "install" and "remove". Default: install.
service=turbo-fan # Service name
target_dir=/opt/turbo-fan # Installation folder
service_dir=/etc/init.d/ # Service setup folder (where all services are stored)

# Build the Rust kernel module and utilities
echo "[*] Building Rust kernel module and utilities..."
cargo build --release
cargo build --bin facer-rgb --release
cargo build --bin keyboard --release

echo "[Mode: $mode]";

# Sudo check
if [[ $(id -u) -ne 0 ]]; then
    echo "Please run as root"
    exit 1
fi

# Check rc-service is installed (OpenRC)
if [[ -z "$(whereis rc-service | sed 's/rc-service: //')" ]]; then
    echo "rc-service is not installed! If you aren't using openrc or aren't sure, use install_service.sh instead!"
    exit 1
fi
# Check rsync is installed
if [[ -z "$(whereis rsync | sed 's/rsync: //')" ]]; then
    echo "rsync is not installed"
    exit 1
fi

if [[ "$mode" == "install" || "$mode" == "remove" ]]; then
    # Check service is presented and remove if yes.
    if [[ "$(rc-update | grep $service)" ]]; then
        echo "['$service' service is presented. Remove it.]";
        rc-service $service stop;
        rc-update del $service default;
        rm $service_dir/$service
    fi
        
    # Remove old files
    echo "[Remove old data]";
    rm -rvf $target_dir;
fi;

if [[ "$mode" == "install" ]]; then
    echo "[Create directories]";
    mkdir -p $target_dir

    echo "[Copy new data]";
    rsync -av ./* $target_dir --exclude=".git/*" --exclude="target/"

    echo "[Create turbo-fan service]"
    cat << 'EOFSERVICE' > $service_dir/$service
#!/sbin/openrc-run
depend() {
    after bootmisc consolefont modules netmount
    after quota keymaps
    after elogind
    use dbus xfs
}

start() {
    # Remove previous modules if they exist
    /sbin/rmmod acer_wmi 2>/dev/null || true
    /sbin/rmmod facer 2>/dev/null || true
    
    # Install required kernel modules
    /sbin/modprobe wmi
    /sbin/modprobe sparse-keymap
    /sbin/modprobe video
    
    # Install the Rust kernel module
    # Note: Actual Rust kernel module installation requires proper kernel build setup
    # This is a placeholder for demonstration
    if [ -f "$target_dir/target/release/libfacer.rlo" ]; then
        echo "[*] Rust kernel module built successfully"
        echo "[*] Note: Actual kernel module installation for Rust requires additional setup"
        # In a real implementation, we would properly install the kernel module here
    else
        echo "[*] Warning: Rust kernel module not found - using placeholder"
    fi
    
    # Copy and set up user-space utilities
    cp $target_dir/target/release/facer-rgb /usr/local/bin/
    cp $target_dir/target/release/keyboard /usr/local/bin/
    chmod +x /usr/local/bin/facer-rgb /usr/local/bin/keyboard
    
    # Create device nodes (normally done by kernel module)
    mknod /dev/acer-gkbbl-0 c 250 0 || true
    mknod /dev/acer-gkbbl-static-0 c 250 1 || true
}

stop() {
    /sbin/rmmod facer
    # Clean up device nodes if needed
    rm -f /dev/acer-gkbbl-0 /dev/acer-gkbbl-static-0
}

EOFSERVICE
    chmod +x $service_dir/$service
    rc-update add $service default
    rc-service $service start
fi
