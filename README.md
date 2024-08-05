# Hakifi-smart-contract (solana)

## Required environment
1. ### Install Rust 
   The Rust programming language is a multi-paradigm, general-purpose programming language that emphasizes performance, type safety, and concurrency.

   Using rustup, the official Rust version installer and manager, we will install rustc (the compiler for rust) and cargo (the package manager for rust) all at once.

   Install Rust for macOS, Linux, WSL or another Unix-like OS #
   Using the following command, we can install and configure the Rust tooling on your local system. The following command will automatically download the correct binaries needed for your specific operating system:

   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
   As part of this Rust installer, Rustup will also configure your terminal's PATH to include the rust toolchain.

   After the installation is complete, restart your terminal or run the following command to manually refresh your new PATH settings to make the rust tooling (like cargo) available:

   source ~/.bashrc
2. ### Install the Solana CLI 
   For local development, including compiling your Solana programs, you will need to install the Solana CLI. This command-line tool suite provides all the commands needed to perform common tasks, like:

   creating and managing file-system Solana wallets/keypairs,
   connecting to Solana clusters,
   building Solana programs,
   and deploying your programs to the blockchain
   For Linux, macOS, WSL or other Unix-like systems: #
   Install the Solana CLI tool suite using the official install command:

   sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
   You can replace stable with the release tag matching the software version of your desired release (i.e. v1.18.1), or use one of the three symbolic channel names: stable, beta, or edge.

   Depending on your specific operating system, the Solana CLI installer messaging may prompt you to update the PATH environment.

   Please update your PATH environment variable to include the Solana programs:
   If you get the above message, simply copy and paste the command recommended by the Solana CLI installer to update your PATH environment variable.

   After running this command. restart your terminal to make sure your Solana binaries are accessible in all the terminal sessions you open afterwards.

   To check if your installation was successful, check the Solana CLI version:

   solana --version
   You can see more versions and releases according to the target solana/releases

   UPDATING THE SOLANA CLI
   In the future, you can use the Solana CLI to update itself based on which latest version is available: solana-install update

3. ### Install Anchor for Solana 
   Anchor is a framework for the Solana runtime providing several convenient developer tools for writing onchain programs. It helps you write programs with less code since it has abstracted away a lot of security checks and common boilerplate using Rust's macros.

   To install and manage anchor versions, we will use avm, the anchor version manager. Since avm is installed via cargo (the Rust package manager), the installation steps will be the same for all the operating systems.

   We can then use avm to install the desired version of the Anchor framework.

   Install avm #
   cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
   Install Anchor using avm #
   To install the latest version of anchor using avm:

   avm install latest
   avm use latest
   After the anchor installation is complete, you can verify anchor was installed by checking the installed version:

   anchor --version
   If you do not see an output or receive an error, you may need to restart your terminal.

## Stack details
1. [anchor v0.24.2](https://github.com/project-serum/anchor)
2. [solana v1.19.3](https://github.com/solana-labs/solana)
3. [@solana/web3.js v1.39.1](https://github.com/solana-labs/solana-web3.js)
4. [@solana/spl-token v0.2.0](https://github.com/solana-labs/solana-program-library)

## Deployment
1. Setup a devnet blockchain cluster
   1. solana config set --url https://api.devnet.solana.com
   2. solana-keygen new
   3. solana-keygen new
   4. solana airdrop 5
2. Build
   1. anchor build
3. Deploy
   1. anchor deploy --provider.cluster devnet