#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod cyberpunk_anchor {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct CyberpunkAnchor {
        /// Maps a 32-byte manifest commitment to the block number it was anchored.
        /// This provides O(1) storage and verification.
        commitments: Mapping<[u8; 32], BlockNumber>,
    }

    #[ink(event)]
    pub struct ManifestAnchored {
        #[ink(topic)]
        commitment: [u8; 32],
        block_number: BlockNumber,
    }

    impl CyberpunkAnchor {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                commitments: Mapping::default(),
            }
        }

        /// Anchors a manifest commitment on-chain.
        /// Ensuring the manifest is unique and timestamped.
        #[ink(message)]
        pub fn anchor_manifest(&mut self, commitment: [u8; 32]) -> bool {
            if self.commitments.contains(commitment) {
                return false;
            }
            let current_block = self.env().block_number();
            self.commitments.insert(commitment, &current_block);
            
            self.env().emit_event(ManifestAnchored {
                commitment,
                block_number: current_block,
            });
            true
        }

        /// Verifies if a specific manifest commitment exists in the anchor layer.
        #[ink(message)]
        pub fn is_anchored(&self, commitment: [u8; 32]) -> bool {
            self.commitments.contains(commitment)
        }

        /// Returns the block number when a manifest was anchored.
        #[ink(message)]
        pub fn get_anchor_age(&self, commitment: [u8; 32]) -> Option<BlockNumber> {
            self.commitments.get(commitment)
        }
    }
}
