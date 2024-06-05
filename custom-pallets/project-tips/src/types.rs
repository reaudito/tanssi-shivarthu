use super::*;
use parity_scale_codec::{Decode, Encode, EncodeLike, MaxEncodedLen};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;

pub const PROJECT_ID: ProjectId = 1;

#[derive(Encode, Decode, Clone, Copy, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub enum TippingName {
	SmallTipper,
	BigTipper,
	SmallSpender,
	MediumSpender,
	BigSpender,
}

#[derive(Encode, Decode, Clone, Copy, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct TippingValue<Balance> {
	pub max_tipping_value: Balance,
	pub stake_required: Balance,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Project<T: Config> {
	pub created: WhoAndWhenOf<T>,
	pub project_id: ProjectId,
	pub department_id: DepartmentId,
	pub content: Content,
	pub tipping_name: TippingName,
	pub funding_needed: BalanceOf<T>,
	pub project_leader: T::AccountId,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Incentives<T: Config> {
	pub number_of_games: u32,
	pub winner: u32,
	pub loser: u32,
	pub start: WhoAndWhenOf<T>,
	pub update: WhoAndWhenOf<T>,
}


#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct IncentivesMetaData<T: Config> {
    pub total_number: u32,
	pub disincentive_times: u32, 
    pub total_block: BlockNumberOf<T>,
}

impl<T: Config> Default for IncentivesMetaData<T> {
    fn default() -> Self {
        Self {
            total_number: 20,
            disincentive_times: 15, // its 1.5
            total_block: 432000u64.saturated_into::<BlockNumberOf<T>>() // 30 days = (24*60*60)/6 * 30
        }
    }
}