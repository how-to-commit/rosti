# Rust operating system on i686

## todo:  
- [x] gdt loading
- [x] interrupts 
- [ ] debug/test harness  
- [ ] paging  
- [ ] keyboard input? 
- [ ] rtc?  

## Project goals  
Learn more about operating systems and attempt to reimplement (nearly) 
everything without working with external libraries, minus the Rust core and 
alloc libraries. 


## run steps Windows
- wsl
- sudo apt install rustup build-essential gcc-i686-linux-gnu texinfo qemu-system-x86 libgmp-dev libmpfr-dev libmpc-dev
- cargo
- cargo run

i686 issue resolution:

# Choose a directory for your toolchain
mkdir -p ~/cross && cd ~/cross

# Download sources (replace with latest versions if needed)
wget https://ftp.gnu.org/gnu/binutils/binutils-2.40.tar.xz
wget https://ftp.gnu.org/gnu/gcc/gcc-13.2.0/gcc-13.2.0.tar.xz

tar -xf binutils-2.40.tar.xz
tar -xf gcc-13.2.0.tar.xz

# Build binutils first
mkdir build-binutils && cd build-binutils
../binutils-2.40/configure --target=i686-elf --prefix=$HOME/cross --disable-nls --disable-werror
make -j$(nproc)
make install
cd ..

# Build GCC (only C compiler)
mkdir build-gcc && cd build-gcc
../gcc-13.2.0/configure --target=i686-elf --prefix=$HOME/cross --disable-nls --enable-languages=c --without-headers
make all-gcc -j$(nproc)
make install-gcc
cd ..

# Add the toolchain to your PATH
Example: export PATH="$HOME/cross/bin:$PATH"
/home/sventan/cross/bin/i686-elf-gcc (which i686-elf-gcc)
export PATH="/home/sventan/cross/bin/i686-elf-gcc:$PATH"

# Confirm the cross compiler is available
i686-elf-gcc --version
