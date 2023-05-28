pub mod user;

use uuid::Uuid;

use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub Id<T> {
    pub id: Uuid,
    _maker: PhantomData<T>,
}

impl Id<T> {
    pub fn new(id: Uuid) -> Self {
        Self {
            id: id,
            _maker: PhantomData,
        }
    }

    pub fn gen() -> Self {
        Self {
            id: Uuid::new_v4(),
            _maker: PhantomData,
        }
    }
}
