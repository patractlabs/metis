#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod groth16 {
    use ink_prelude::vec::Vec;

    /// Groth16 exports
    #[ink(storage)]
    pub struct Groth16;

    impl Groth16 {
        #[ink(constructor)]
        pub fn default() -> Self {
            Groth16 {}
        }

        #[ink(message)]
        pub fn bls12_377_verify(
            &self,
            vk_gamma_abc: Vec<Vec<u8>>,
            vk: Vec<u8>,
            proof: Vec<u8>,
            public_inputs: Vec<Vec<u8>>,
        ) -> bool {
            if let Ok(result) = curve::verify(0x0, vk_gamma_abc, vk, proof, public_inputs) {
                result
            } else {
                false
            }
        }

        #[ink(message)]
        pub fn bls12_381_verify(
            &self,
            vk_gamma_abc: Vec<Vec<u8>>,
            vk: Vec<u8>,
            proof: Vec<u8>,
            public_inputs: Vec<Vec<u8>>,
        ) -> bool {
            if let Ok(result) = curve::verify(0x1, vk_gamma_abc, vk, proof, public_inputs) {
                result
            } else {
                false
            }
        }

        #[ink(message)]
        pub fn bn254_verify(
            &self,
            vk_gamma_abc: Vec<Vec<u8>>,
            vk: Vec<u8>,
            proof: Vec<u8>,
            public_inputs: Vec<Vec<u8>>,
        ) -> bool {
            if let Ok(result) = curve::verify(0x1, vk_gamma_abc, vk, proof, public_inputs) {
                result
            } else {
                false
            }
        }

        #[ink(message)]
        pub fn bw6_761(
            &self,
            vk_gamma_abc: Vec<Vec<u8>>,
            vk: Vec<u8>,
            proof: Vec<u8>,
            public_inputs: Vec<Vec<u8>>,
        ) -> bool {
            if let Ok(result) = curve::verify(0x1, vk_gamma_abc, vk, proof, public_inputs) {
                result
            } else {
                false
            }
        }
    }
}
