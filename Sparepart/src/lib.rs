#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_error,decl_event, dispatch, traits::Get};
use frame_system::ensure_signed;

use frame_support::traits::Vec;
use codec::{Encode,Decode};


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Encode,Decode,Default,Clone,PartialEq)]
pub struct Sparepart<AccountId>{
	vendorbyid:AccountId,
	partid:u32,
	description:Vec<u8>,
	model:Vec<u8>,
	product:Vec<u8>,
	batch:Vec<u8>,
	manufacturingdate:Vec<u8>,
}

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}
pub type Sparepartof<T>= Sparepart<<T as frame_system::Config>::AccountId>;

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	
	trait Store for Module<T: Config> as SparepartsModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		 Something get(fn something): Option<u32>;
		 Spareparts get(fn spareparts):map hasher(blake2_128_concat) u32 => Option<Sparepart<T::AccountId>>
	
	}
}
decl_error!{
	pub enum Error for Module<T:Config>{
	
		NoneValue,
		StorageOverflow,
	  }
	}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where 
	 <T as frame_system::Config>::AccountId{
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32,AccountId),
		Insertsparepart(AccountId,u32,Vec<u8>,Vec<u8>,Vec<u8>,Vec<u8>,Vec<u8>),
	}
);



// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			Something::put(something);

			// Emit an event.
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			// Return a successful DispatchResult
			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn insert_sparepart(origin, 
			partid:u32,
			description:Vec<u8>,
			model:Vec<u8>,
			product:Vec<u8>,
			batch:Vec<u8>,
			manufacturingdate:Vec<u8>) -> dispatch::DispatchResult 
			{
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;
			
			let insert_sparepart=Sparepart{
				vendorbyid:who.clone(),
				partid,
				description:description.clone(),
				model:model.clone(),
				product:product.clone(),
				batch:batch.clone(),
				manufacturingdate:manufacturingdate.clone(),
			};
			//Sparepart::<T>::insert(&insert_sparepart, Some(&who))?;
			// Self::_add_sparepart_to_storage(&mut insert_sparepart,Some(&who))?;
			// // Emit an event.
			
			Spareparts::<T>::insert(&partid,insert_sparepart);
			
			Self::deposit_event(RawEvent::Insertsparepart(who,partid,description,model,product,batch,manufacturingdate));
			
			//Self::deposit_event(RawEvent::Insertsparepart(vendorbyid,partid,description,model,product,batch,manufacturingdate,who));
			// Return a successful DispatchResult
			Ok(())
		}
		
	}
	
 }
 


// fn _add_sparepart_to_storage(insert_sparepart:&mut Sparepart)-> dispatch::DispatchResult {

	
// Sparepart::insert(&insert_sparepart.clone());
	
// 	Ok(())
// }
