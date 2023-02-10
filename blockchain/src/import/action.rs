use std::sync::MutexGuard;
use crate::backend::{SharedCommittable, Store, Operation, ImportOperation};
use crate::{Block, Auxiliary};

pub struct ImportAction< Ba: Store>{
    backend: Ba,
    pending: Operation<Ba::Block, Ba::State, Ba::Auxiliary>,
    _guard: MutexGuard<'a, ()>
}

impl<Ba: Store > From<ImportAction<Ba>> for Operation<Ba::Block, Ba:: State, Ba: Auxiliary>{
    fn from(action: ImportAction<Ba>)-> Operation<Ba::Block, Ba::State, Ba::Auxiliary>{
        action.pending
    }
}

impl<Ba: Store> ImportAction<Ba> where Ba: SharedCommittable<Operation= Operation<<Ba as Store>::Block,<Ba as Store>::State, <Ba as Store>::Auxiliary>>{

    pub fn new(backend: Ba, import_guard: MutexGuard<()>)-> Self{
        Self{
            backend,
            pending: Default::default(),
            _guard: import_guard
        }
    }

    pub fn backend(&self)-> Ba{
        self.backend
    }

    pub fn import_block(&self, block: Ba::Block, state: Ba::State){
        self.import_raw(ImportOperation{block, state});
    }

    pub fn import_raw(&self, raw: ImportOperation<Ba::Block, Ba::State>){
        self.pending.import_block.push(raw);
    }

    pub fn set_head(&self, head: <Ba::Block as Block>::Identifier){
        self.pending.set_head = Some(head);
    }

    pub fn insert_auxiliary(&self, aux: Ba::Auxiliary){
        self.pending.insert_auxiliaries.push(aux);
    }

    pub fn remove_auxiliary(&self, aux_key:<Ba::Auxiliary as Auxiliary<Ba::Block>>::Key){
        self.pending.remove_auxiliaries.push(aux_key);
    }

    pub fn commit(self)-> Result<(), Ba::Error>{
        Ok(self.backend.commit(self.into())?)
    }

}