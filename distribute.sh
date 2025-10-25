#!/bin/bash

# This script builds a distributable AppImage
# of the term.everything application using Podman.

PODMAN_ROOT="./.podman"
PODMAN_RUNROOT="./.podman-run"
PODMAN="podman --root $PODMAN_ROOT --runroot $PODMAN_RUNROOT"
APP_NAME="term.everything❗mmulet.com-dont_forget_to_chmod_+x_this_file"

get_distro() {
    # Try to detect the distro
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        DISTRO=$ID
    else
        DISTRO="unknown"
    fi
    
    case $DISTRO in
        ubuntu|debian)
            echo "sudo apt update && sudo apt install -y "
            ;;
        fedora)
            echo "sudo dnf install -y "
            ;;
        centos|rhel|rocky|almalinux)
            echo "sudo yum install -y "
            ;;
        arch|manjaro)
            echo "sudo pacman -S "
            ;;
        opensuse*)
            echo "sudo zypper install "
            ;;
        alpine)
            echo "sudo apk add "
            ;;
        *)
            echo "Please install podman using your distribution's package manager"
            ;;
    esac
}

if ! command -v podman >/dev/null 2>&1; then
    INSTALL_CMD=$(get_distro)
    echo "Warning: podman is not installed or not in PATH."
    echo "To install on your system, try: $INSTALL_CMD podman"
    echo "Please install podman to proceed, it's literally all you need. Don't even need attention. Just podman. Just get podman. What are you waiting for? Stop reading this and install podman."
    exit 1
fi

# Check if git is available and update submodules if so, unless SKIP_SUBMODULE_CHECK is set
if [ -z "$SKIP_SUBMODULE_CHECK" ]; then
    if command -v git >/dev/null 2>&1; then
        git submodule update --init --recursive
    else
        INSTALL_CMD=$(get_distro)
        echo "Git is not available, I know I said you only need podman, which is technically true. But, git is the easiest way to download third_party dependencies. Either install git (perhaps with $INSTALL_CMD git) or download the submodules manually. If you already downloaded the submodule, rerun this script with SKIP_SUBMODULE_CHECK=1."
        exit 1
    fi
else
    echo "Skipping submodule update due to SKIP_SUBMODULE_CHECK being set."
fi

# Create a distributable appimage using podman
$PODMAN build -t term.everything:appimage -f ./resources/ContainerFile .
$PODMAN create --name term-appimg term.everything:appimage
$PODMAN cp term-appimg:/out ./dist
$PODMAN rm term-appimg
$PODMAN rmi -f term.everything:appimage
$PODMAN image prune -f
$PODMAN unmount -a || true
$PODMAN system reset -f || true
chmod -R u+rwX $PODMAN_ROOT || true
rm -rf $PODMAN_ROOT || true
rm -rf $PODMAN_RUNROOT || true


mv ./dist/*.AppImage ./dist/$APP_NAME

echo ""
echo "Output is ./dist/$APP_NAME "


chmod +x ./third_party/chafa
./third_party/chafa ./resources/icon.png

echo "Output is ./dist/$APP_NAME "
