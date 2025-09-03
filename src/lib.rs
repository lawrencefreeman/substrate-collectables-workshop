#![cfg_attr(not(feature = "std"), no_std)]

mod impls;
mod tests;

use frame::prelude::*;
use frame::traits::fungible::Inspect;
use frame::traits::fungible::Mutate;
pub use pallet::*;


#[frame::pallet(dev_mode)]
pub mod pallet {
	use super::*;


	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::config]
	pub trait Config: frame_system::Config { // this tightly couples our Kitties pallet to the frame_system pallet (we can + others too)
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type NativeBalance: Inspect<Self::AccountId> + Mutate<Self::AccountId>;

		#[pallet::constant]
		// TODO: Max number of kitties a single account can own = 100
		type MaxKittyOwned: Get<u32>;
	}

	#[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		pub dna: [u8; 32],
		pub owner: T::AccountId,
	}

	#[pallet::storage]
	pub(super) type CountForKitties<T: Config> = StorageValue<Value = u32>;

	//All owners and their Kitties
	#[pallet::storage]
	pub(super) type Kitties<T: Config> = StorageMap<Key = [u8; 32], Value = Kitty<T>>;

	/// Track the kitties owned by each account in an optimal way
	#[pallet::storage]
	pub(super) type KittiesOwned<T: Config> = StorageMap<
		Key = T::AccountId,
		Value = BoundedVec<[u8; 32], T::MaxKittyOwned>, 
		QueryKind = ValueQuery
	>;

/// The Events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created { owner: T::AccountId },
		Transferred { from: T::AccountId, to: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		TooManyKitties,
		DuplicateKitty,
		CannotTransferToSelf,
		KittyNotExist,
		NotKittyOwner,
	}

/// The Extrinsics
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let dna = [0u8; 32];
			Self::mint(who, dna)?;
			Ok(())
		}	

		pub fn transfer(origin: OriginFor<T>, to: T::AccountId, kitty_id: [u8; 32]) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_transfer(who, to, kitty_id)?;
			Ok(())
		}
	}
}
