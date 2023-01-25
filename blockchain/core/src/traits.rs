#[cfg(feature="std")]
use std::error as stderror;
use alloc::vec::Vec;
use core::hash;

pub trait Block: Clone{
    type Identifier: Clone + Eq + hash::Hash;
    fn id(&self)-> Self::Identifier;
    fn parent_id(&self)-> Option<Self::Identifier>;
}

pub trait Auxiliary<B: Block>: Clone{
    type Key: Clone + Eq + hash::Hash;

    fn key(&self)-> Self::Key;

    fn associated(&self)-> Vec<B::Identifier>{
        Vec::new()
    }
}

impl<B: Block> Auxiliary<B> for (){
    type Key = ();

    fn key(&self)-> () {
        ()
    }
}

pub trait AsExternalities<E: ?Sized>{
    fn as_externalities(&self)-> &E;
}

pub trait NullExternalities{}

impl NullExternalities for () {}

impl AsExternalities<dyn NullExternalities> for (){
    fn as_externalities(&self)-> &(dyn NullExternalities + 'static ){
        self
    }
}

pub trait StorageExternalities<Error>{
    fn read_storage(&self, key: &[u8])-> Result<Option<Vec<u8>> ,Error>;
    fn write_storage(&mut self, key: Vec<u8>, value: Vec<u8>);
    fn remove_storage(&mut self, key: &[u8]);
}

pub trait BlockExecutor{
    #[cfg(feature="std")]
    type Error: stderror::Error;
    #[cfg(not(feature="std"))]
    type Error: 'static;
    type Block: Block;
    type Externalities: Sized;
    fn execute_block(&self, block: &Self::Block, state: &Self::Externalities)-> Result<(), Self::Error>;
}

pub trait ExtrinsicBuilder: BlockExecutor{
    type BuildBlock;
    type Inherent;
    type Extrinsic;

    fn initialize_block(&self, parent_block: &Self::Block, state: &Self::Externalities, inherent: Self::Inherent )-> Result<Self::BuildBlock, Self::Error>;
    fn apply_extrinsic(&self, block: &Self::BuildBlock, extrinsic: Self::Extrinsic, state: &Self::Externalities) -> Result<(), Self::Error>;
    fn finalize_block(&self, block: &Self::BuildBlock, state: &Self::Externalities)-> Result<(), Self::Error>;
}

