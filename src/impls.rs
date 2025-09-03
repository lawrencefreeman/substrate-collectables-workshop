use super::*;
use frame::prelude::*;
use frame::primitives::BlakeTwo256;
use frame::primitives::Hash;


/// Internal functions
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
		let kitty: Kitty<T> = Kitty {dna, owner: owner.clone(), price: None};
		//ensure the kittie does not already exist
		ensure!(!Kitties::<T>::contains_key(dna), Error::<T>::DuplicateKitty);
		let current_count: u32 = CountForKitties::<T>::get().unwrap_or(0);
		let new_kittie_count = current_count.checked_add(1).ok_or(Error::<T>::TooManyKitties)?;
		
		// must use try_append to avoid exceeding the max length of the bounded vec
		KittiesOwned::<T>::try_append(&owner, dna).map_err(|_| Error::<T>::TooManyKitties)?;

		Kitties::<T>::insert(dna, kitty);
		CountForKitties::<T>::set(Some(new_kittie_count));
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}

	pub fn do_transfer(from: T::AccountId, to: T::AccountId, kitty_id: [u8; 32]) -> DispatchResult {
		ensure!(from != to, Error::<T>::CannotTransferToSelf);
		let kitty = Kitties::<T>::get(kitty_id).ok_or(Error::<T>::KittyNotExist)?;
		ensure!(kitty.owner == from, Error::<T>::NotKittyOwner);
		// Go ahead wsafely with the mutations of the 3 storage items
		// First we will try and push into the new owner to ensure they do not exceed max bound
		let mut to_owned = KittiesOwned::<T>::get(&to);
		to_owned.try_push(kitty_id).map_err(|_| Error::<T>::TooManyKitties)?;
		KittiesOwned::<T>::insert(&to, to_owned);
		// Now we can remove from the old owner - will use swap_remove as lighter than remove (remove reorders)
		let mut from_owned = KittiesOwned::<T>::get(&from);
		if let Some(pos) = from_owned.iter().position(|&id| id == kitty_id) {
			from_owned.swap_remove(pos);
		} else {
			return Err(Error::<T>::NotKittyOwner.into());
		}

		Kitties::<T>::insert(kitty_id, Kitty { dna: kitty_id, owner: to.clone(), price: None });
		Self::deposit_event(Event::<T>::Transferred { from: from, to: to });
		Ok(())
	}
}
