use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::*;

#[account]
#[derive(Default)]
pub struct GlobalState {
    pub admin: PubKey
}


#[account]
#[derive(Default)]
pub struct BreedingNftInfo {

}

#[account]
#[derive(Default)]
pub struct EvolvingNftInfo {

}