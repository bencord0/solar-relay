# Solar Relay
A raspberry pi can be used to monitor voltages, with the help of an ADC
such as the MCP3008.



## Toolchain setup

Get an ARM toolchain to cross-compile binaries for the raspberry pi.
This is a C compiler, but we still need the linker, aka. `arm-none-linux-gnueabihf-gc`.

    https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/downloads

Use the 32-bit toolchain, and install it to ~/Software/arm-none-linux-gnueabihf

    $ export PATH="${PATH}:/${HOME}/Software/arm-none-linux-gnueabihf/bin"

Get the rust ARM cross-compiler, so that we can create static binaries

    $ rustup target add arm-unknown-linux-musleabihf
    $ cargo build --release --target=arm-unknown-linux-musleabihf

## Raspberry Pi setup (headless)

Download the Raspberry Pi OS Lite (32-bit), extract and flash it to an SD Card.

https://www.raspberrypi.com/software/operating-systems/

Edit the boot directory, and add a `userconf.txt` to create a user and custom password.

    $ (echo -n 'pi:'; echo 'raspberry' | openssl passwd -6 -stdin) > userconf.txt

Enable SPI in the boot config.

    $ sed -i -e 's/#dtparam=spi=on/dtparam=spi=on/' config.txt

