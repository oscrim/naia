use std::marker::PhantomData;

use bevy::ecs::{
    system::{SystemParamFetch, SystemParamState, SystemState},
    world::World,
};

use naia_client::ProtocolType;

use super::client::Client;
use bevy::ecs::system::SystemMeta;

// State

pub struct State<P: ProtocolType> {
    phantom_p: PhantomData<P>,
}

// SAFE: only local state is accessed
unsafe impl<P: ProtocolType> SystemParamState for State<P> {
    type Config = ();

    fn init(_world: &mut World, _system_state: &mut SystemMeta, _config: Self::Config) -> Self {
        State {
            phantom_p: PhantomData,
        }
    }

    fn apply(&mut self, _world: &mut World) {}

    fn default_config() {}
}

impl<'w, 's, P: ProtocolType> SystemParamFetch<'w, 's> for State<P> {
    type Item = Client<'w, 's, P>;

    #[inline]
    unsafe fn get_param(
        _state: &'s mut Self,
        _system_state: & SystemMeta,
        world: &'w  World,
        _change_tick: u32,
    ) -> Self::Item {
        Client::new(world)
    }
}
