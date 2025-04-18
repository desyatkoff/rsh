# Rsh

## Description

**Rsh** stands for **Rust shell**. And yes, yet another shell like Bash, Zsh, Fish, etc but worse

## Installation

### Auto method

1. **Clone the repository**
    ```Shell
    $ git clone https://github.com/desyatkoff/rsh.git
    ```
2. **Go to the repository directory**
    ```Shell
    $ cd rsh/
    ```
3. **Launch `install.sh` script**
    ```Shell
    sh install.sh
    ```

### Manual method

1. **Clone the repository**
    ```Shell
    $ git clone https://github.com/desyatkoff/rsh.git
    ```
2. **Go to the repository directory**
    ```Shell
    $ cd rsh/
    ```
3. **Compile the Rust project**
    ```Shell
    $ cargo build --release
    ```
4. **OPTIONAL (but still recommended): Copy Rsh to the `/bin/` directory**
    ```Shell
    $ cp ./target/release/rsh /bin/
    ```

## Usage

Firstly, you have to [install Rsh](#installation) and then type one of these commands:
1. **If you have NOT moved or copied Rsh binary to `/bin/`**
    ```Shell
    $ ./target/release/rsh
    ```
2. **If you have moved or copied Rsh binary to `/bin/`**
    ```Shell
    $ rsh
    ```
After that, you will be moved to Rsh and you can start executing some commands with Rsh

