use super::*;
use frame::prelude::*;
use frame::primitives::BlakeTwo256;
use frame::primitives::Hash;


impl<T: Config> Pallet<T> {
	pub fn gen_dna() -> [u8; 32] {
			let parent_hash = frame_system::Pallet::<T>::parent_hash();
			let block_number = frame_system::Pallet::<T>::block_number();
			let extrinsic_index = frame_system::Pallet::<T>::extrinsic_index().unwrap_or(0);
			let count = CountForKitties::<T>::get().unwrap_or(0);

			let unique_payload = (
				parent_hash,
				block_number,
				extrinsic_index,
				count,
			);

			let hash = BlakeTwo256::hash_of(&unique_payload);
			hash.into()
		}

	pub fn mint(owner: T::AccountId, dna: [u8; 32]) -> DispatchResult {
		let kitty: Kitty<T> = Kitty {dna, owner: owner.clone()};
		//ensure the kittie does not already exist
		ensure!(!Kitties::<T>::contains_key(dna), Error::<T>::DuplicateKitty);
		let current_count: u32 = CountForKitties::<T>::get().unwrap_or(0);
		let new_kittie_count = current_count.checked_add(1).ok_or(Error::<T>::TooManyKitties)?;
		
		/* 🚧 TODO 🚧: `append` the `dna` to the `KittiesOwned` storage for the `owner`. */
		KittiesOwned::<T>::try_append(&owner, dna).map_err(|_| Error::<T>::TooManyKitties)?;

		Kitties::<T>::insert(dna, kitty);
		CountForKitties::<T>::set(Some(new_kittie_count));
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
}
