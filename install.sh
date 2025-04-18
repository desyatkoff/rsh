#!/usr/bin/env bash


###########################################
#                                         #
#  INSTALLER SCRIPT FOR RSH (RUST SHELL)  #
#                                         #
###########################################


# 0. Before installation

# Warning!
# You can only continue if you already have Rust and Cargo installed


# 1. Compile the Rust project

cargo build --release


# 2. Copy Rsh binary file to the `/bin/` directory

sudo cp -v \
    ./target/release/rsh \
    /bin/


# 3. Launch Rsh

rsh


# 4. After installation

# Success!
# Enjoy your new blazingly fast and very unuseful trashy shell

