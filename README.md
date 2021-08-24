# Metis

Metis is inspired by the [OpenZeppelin-Contracts](https://github.com/OpenZeppelin/openzeppelin-contracts) from the Ethereum ecosystem. 

Metis will be the Wasm contract standard library developed by Patract Labs. Patract Labs will work with community forces to formulate various Wasm contract standards, develop corresponding implementations of ink! and Ask! versions, and conduct audits by professional institutions.

**At now, Metis just provide support for ink! contract.** It will develop the corresponding version of the ink! implementation for each newly generated ink! contract standard. And Metis will be audited at the same time to ensure the security of the contract. Metis will also be able to be easily searched and downloaded just like Solidity development, and integrated into the developer's project as a Redspot plug-in.

To use the Metis, can see the [documentation](https://patractlabs.github.io/metis/#/./en-us/overview).

Proposal(M1)： https://polkadot.polkassembly.io/post/469

## Use
Since ink! is under development, developers need to use metis according to their own situation:

1. developers use ink! dependencies from `crates.io`:

    e.g.:
    ```toml
    [dependencies]
    ink_lang = { version = "3.0.0-rc4", default-features = false }
    ```
    For this situation, Metis provides different branch/tag to support different ink! version. Like here ink! version
    is `3.0.0-rc4`, then Metis provides branch `ink/3.0.0-rc4` to allow developer to use the same ink! source. In this 
    branch, Metis' dependencies for ink! will point to `ink_lang = { version = "3.0.0-rc4", default-features = false }` as well:

    ```toml
    [dependencies]
    ink_lang = { version = "3.0.0-rc4", default-features = false }
    metis_lang = { github = "https://github.com/patractlabs/metis", branch = "ink/3.0.0-rc4", default-features = false }
    ```
    
    For now, the relationship between Metis branch/tag and ink! release version are following:
    
    * ink!: `3.0.0-rc4` | Metis branch: `ink/3.0.0-rc4`

2. developers use ink! dependencies from github repo:

    e.g.:
    ```toml
    [dependencies]
    ink_lang = { version = "3.0.0-rc4", git = "https://github.com/paritytech/ink", default-features = false }
    ```
    For this situation, developer should use Metis master branch directly. In master, Metis' dependencies for ink! will be github repo as well.

    ```toml
    [dependencies]
    ink_lang = { version = "3.0.0-rc4", default-features = false }
    metis_lang = { github = "https://github.com/patractlabs/metis", default-features = false }
    ```
