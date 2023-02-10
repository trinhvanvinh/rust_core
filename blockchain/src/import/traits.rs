use std::error as stderror;
use crate::Block;

pub trait BlockImporter{
    type Block: Block;
    type Error: stderror::Error;
    fn import_block(&self, block: Self::Block)-> Result<(), Self::Error>;
}

pub trait SharedBlockImporter: BlockImporter{
    fn import_block(&self, block: Self::Block)-> Result<(), Self::Error>;
}

pub trait RawImporter{
    type Operation;
    type Error: stderror::Error;

    fn import_raw(&self, operation: Self::Operation)-> Result<(), Self::Error>;
}

pub trait SharedRawImporter: RawImporter{
    fn import_raw(&self, operation: Self::Operation)-> Result<(), Self::Error>;
}