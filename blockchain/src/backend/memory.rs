use std::{fmt, error as stderror };
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::{Block, Auxiliary};
use crate::{Store, BlockData, ChainQuery, ChainSettlement, Operation, Committable, ShareCommittable, OperationError};

pub enum Error{
    InvalidOperation,
    IsGenesis,
    NotExist
}

impl OperationError for Error{
    fn invalid_operation()-> Self{
        Error::InvalidOperation
    }
    fn block_is_genesis()-> Self{
        Error::IsGenesis
    }

}

impl stderror::Error for Error{}

pub struct MemoryDatabase<B: Block, A: Auxiliary<B>, S>{
    blocks_and_states: HashMap<B::Identifier, BlockData<B, S>>,
    head: B::Identifier,
    genesis: B::Identifier,
    canon_depth_mappings: HashMap<usize, B::Identifier>,
    auxiliaries: HashMap<A::Key, A>
}

impl<B: Block, A: Auxiliary<B>, S: Clone > Store for MemoryDatabase<B, A, S>{
    type Block = B;
    type State = S;
    type Auxiliary = A;
    type Error = Error;
}

impl <B: Block, A: Auxiliary<B>, S: Clone>  ChainQuery for MemoryDatabase<B, A, S>{
    fn head(&self)->B::Identifier{
        self.head
    }

    fn genesis(&self)->B::Identifier{
        self.genesis
    }


}

impl <B: Block, A: Auxiliary<B>, S: Clone> ChainSettlement for MemoryDatabase<B, A, S>{
    fn insert_block(){

    }
}

pub struct MemoryBackend<B: Block, A: Auxiliary<B>, S> (MemoryDatabase<B, A, S>);

impl <B: Block, A: Auxiliary, S: Clone> MemoryBackend <B, A, S>{
    pub fn new_with_genesis(block: B, genesis_state: S)-> Self{
        assert!(block.parent_id(), "with_genesis must be provided with a genesis block");
    }
}