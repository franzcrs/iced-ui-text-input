# iced-ui-text-input 
Hands-on test of "iced" for Rust, displaying and styling an input box. 
Library official repository: [github.com/iced-rs/iced](https://github.com/iced-rs/iced)

## Prerequisites 

- **Rust**  
  Install Rust for your OS from [Rust-lang.org](https://www.rust-lang.org/learn/get-started). 
  Verify if Rust is installed by running in your terminal:
   ```
   rustc --version
   ```

- **Cargo**  
  When you install Rustup you’ll also get the latest stable version of Cargo (the Rust build tool and package manager). 
  Verify if Cargo is installed by running in your terminal:
   ```bash
   cargo --version
   ```

## Execution 

1. Open a terminal window and navigate to the directory where you want to save this repository. 
   ```bash
   cd 'YOUR_FOLDER'
   ```

2. Clone this repository: 
   ```bash
   git clone https://github.com/franzcrs/iced-ui-text-input.git
   ```

3. Make sure the iced dependency is defined in Cargo.toml 
   ```bash
   [dependencies]
   iced = "0.13.1"
   ```

4. In the terminal, navigate to the repository folder: 
   ```bash
   cd iced-ui-text-input
   ```

5. Compile and run 
   ```bash
   cargo run
   ```

## Consult examples before the v13.0

After version 13, iced had a big change in the way of building gui components. To see the previous source and examples in the cloned repository of iced, one can follow these steps:

1. Clone the repository
   ```bash
   git clone https://github.com/iced-rs/iced.git
   ```

2. List all the release tags
   ```bash
   git tag
   ```

3. Checkout to the version of your interest
   ```bash
   git checkout tags/0.xx.x
   ```
