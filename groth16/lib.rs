#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod groth16 {
    // use curve::result::Result;
    use ink_prelude::{string::String, vec::Vec};

    /// Groth16 exports
    #[ink(storage)]
    pub struct Groth16 {
        value: bool,
    }

    impl Groth16 {
        #[ink(constructor)]
        pub fn default() -> Self {
            Groth16 { value: false }
        }

        #[ink(message)]
        pub fn debug(&self, parcel: Vec<u8>) -> Result<Vec<u8>, String> {
            curve::call(0x2a, &parcel).map_err(|e| e.debug())
        }

        #[ink(message)]
        pub fn bls12_377_verify(
            &self,
            vk_gamma_abc: Vec<Vec<u8>>,
            vk: Vec<u8>,
            proof: Vec<u8>,
            public_inputs: Vec<Vec<u8>>,
        ) -> Result<bool, String> {
            curve::verify(0x00, vk_gamma_abc, vk, proof, public_inputs).map_err(|e| e.debug())
        }

        #[ink(message)]
        pub fn bls12_381_verify(
            &self,
            vk_gamma_abc: Vec<Vec<u8>>,
            vk: Vec<u8>,
            proof: Vec<u8>,
            public_inputs: Vec<Vec<u8>>,
        ) -> Result<bool, String> {
            curve::verify(0x10, vk_gamma_abc, vk, proof, public_inputs).map_err(|e| e.debug())
        }

        #[ink(message)]
        pub fn bn254_verify(
            &self,
            vk_gamma_abc: Vec<Vec<u8>>,
            vk: Vec<u8>,
            proof: Vec<u8>,
            public_inputs: Vec<Vec<u8>>,
        ) -> Result<bool, String> {
            curve::verify(0x20, vk_gamma_abc, vk, proof, public_inputs).map_err(|e| e.debug())
        }

        #[ink(message)]
        pub fn bw6_761(
            &self,
            vk_gamma_abc: Vec<Vec<u8>>,
            vk: Vec<u8>,
            proof: Vec<u8>,
            public_inputs: Vec<Vec<u8>>,
        ) -> Result<bool, String> {
            curve::verify(0x30, vk_gamma_abc, vk, proof, public_inputs).map_err(|e| e.debug())
        }
    }
}
