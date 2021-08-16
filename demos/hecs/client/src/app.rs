use std::{collections::HashMap, time::Duration};

use log::{info, warn};

use hecs::{Entity as HecsEntityKey, EntityBuilder as HecsEntityBuilder, World};

use naia_client::{
    Client, ClientConfig, Event, LocalEntityKey as NaiaEntityKey, NaiaKey, Ref, Replicate,
};

use naia_hecs_demo_shared::{
    get_server_address, get_shared_config,
    protocol::{Auth, Protocol, StringMessage},
};

pub struct App {
    client: Client<Protocol>,
    world: World,
    entity_builder: HecsEntityBuilder,
    entity_key_map: HashMap<NaiaEntityKey, HecsEntityKey>,
    message_count: u32,
}

impl App {
    pub fn new() -> Self {
        info!("Naia Hecs Client Demo started");

        let mut client_config = ClientConfig::default();
        client_config.server_address = get_server_address();
        client_config.heartbeat_interval = Duration::from_secs(2);
        // Keep in mind that the disconnect timeout duration should always be at least
        // 2x greater than the server's heartbeat interval, to make it so that at the
        // worst case, the client would need to miss 2 server heartbeats before
        // disconnecting from them
        client_config.disconnection_timeout_duration = Duration::from_secs(5);

        // This will be evaluated in the Server's 'on_auth()' method
        let auth = Auth::new("charlie", "12345");

        App {
            client: Client::new(
                Protocol::load(),
                Some(client_config),
                get_shared_config(),
                Some(Protocol::AuthConvert(auth)),
            ),
            world: World::new(),
            entity_builder: HecsEntityBuilder::new(),
            entity_key_map: HashMap::new(),
            message_count: 0,
        }
    }

    // Currently, this will call every frame.
    // On Linux it's called in a loop.
    // On Web it's called via request_animation_frame()
    pub fn update(&mut self) {
        loop {
            if let Some(result) = self.client.receive() {
                match result {
                    Ok(event) => match event {
                        Event::Connection => {
                            info!("Client connected to: {}", self.client.server_address());
                        }
                        Event::Disconnection => {
                            info!("Client disconnected from: {}", self.client.server_address());
                        }
                        Event::Message(Protocol::StringMessage(_recv_message_ref)) => {
                            //let recv_message = _recv_message_ref.borrow();
                            //let recv_message_contents = recv_message.contents.get();
                            //info!("Client received message: {}", recv_message_contents);

                            let send_message_contents =
                                format!("Client Packet ({})", self.message_count);
                            //info!("Client send: {}", send_message_contents);

                            let send_message = StringMessage::new(send_message_contents);
                            self.client.send_message(&Protocol::StringMessageConvert(send_message), true);
                            self.message_count += 1;
                        }
                        Event::CreateEntity(naia_entity_key, component_keys) => {
                            info!("creation of entity: {}", naia_entity_key.to_u16());

                            // initialize w/ starting components
                            for component_key in component_keys {
                                info!(
                                    "init component: {}, to entity: {}",
                                    component_key.to_u16(),
                                    naia_entity_key.to_u16()
                                );

                                match self.client.get_component(&component_key).cloned() {
                                    Some(Protocol::Position(position_ref)) => {
                                        self.entity_builder.add(position_ref);
                                    }
                                    Some(Protocol::Name(name_ref)) => {
                                        self.entity_builder.add(name_ref);
                                    }
                                    Some(Protocol::Marker(marker_ref)) => {
                                        self.entity_builder.add(marker_ref);
                                    }
                                    _ => {}
                                }
                            }

                            let hecs_entity_key = self.world.spawn(self.entity_builder.build());
                            self.entity_key_map.insert(naia_entity_key, hecs_entity_key);
                        }
                        Event::DeleteEntity(naia_entity_key) => {
                            info!("deletion of entity: {}", naia_entity_key.to_u16());
                            if let Some(hecs_entity_key) =
                                self.entity_key_map.remove(&naia_entity_key)
                            {
                                self.world
                                    .despawn(hecs_entity_key)
                                    .expect("unsuccessful despawn of entity");
                            } else {
                                warn!("attempted deletion of non-existent entity");
                            }
                        }
                        Event::AddComponent(naia_entity_key, component_key) => {
                            info!(
                                "add component: {}, to entity: {}",
                                component_key.to_u16(),
                                naia_entity_key.to_u16()
                            );

                            let hecs_entity_key = *self
                                .entity_key_map
                                .get(&naia_entity_key)
                                .expect("attempting to add new component to non-existent entity");

                            match self.client.get_component(&component_key).cloned() {
                                Some(Protocol::Position(position_ref)) => {
                                    self.world
                                        .insert_one(hecs_entity_key, position_ref)
                                        .expect("error inserting component");
                                }
                                Some(Protocol::Name(name_ref)) => {
                                    self.world
                                        .insert_one(hecs_entity_key, name_ref)
                                        .expect("error inserting component");
                                }
                                Some(Protocol::Marker(marker_ref)) => {
                                    self.world
                                        .insert_one(hecs_entity_key, marker_ref)
                                        .expect("error inserting component");
                                }
                                _ => {}
                            }
                        }
                        Event::RemoveComponent(naia_entity_key, component_key, protocol) => {
                            info!(
                                "remove component: {}, from entity: {}",
                                component_key.to_u16(),
                                naia_entity_key.to_u16()
                            );
                            if let Some(hecs_entity_key) =
                                self.entity_key_map.get(&naia_entity_key).copied()
                            {
                                match protocol {
                                    Protocol::Position(position_ref) => {
                                        self.remove_component(&hecs_entity_key, &position_ref);
                                    }
                                    Protocol::Name(name_ref) => {
                                        self.remove_component(&hecs_entity_key, &name_ref);
                                    }
                                    Protocol::Marker(marker_ref) => {
                                        self.remove_component(&hecs_entity_key, &marker_ref);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        Event::Tick => {
                            //info!("tick event");
                        }
                        _ => {}
                    },
                    Err(err) => {
                        info!("Client Error: {}", err);
                        return;
                    }
                }
            } else {
                break;
            }
        }
    }

    fn remove_component<T: 'static + Replicate<Protocol>>(
        &mut self,
        hecs_entity_key: &HecsEntityKey,
        _component_ref: &Ref<T>,
    ) {
        self.world
            .remove_one::<Ref<T>>(*hecs_entity_key)
            .expect("error removing component");
    }
}
