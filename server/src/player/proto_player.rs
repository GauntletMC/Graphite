use crate::{
    entity::position::Position,
    universe::{EntityId, UniverseService},
    world::World,
};
use net::network_buffer::WriteBuffer;

use super::{
    player::{Player, PlayerService}, player_connection::AbstractConnectionReference,
};

// Proto player

pub struct ProtoPlayer<U: UniverseService> {
    connection: U::ConnectionReferenceType,
    pub hardcore: bool,

    pub(crate) write_buffer: WriteBuffer,
    pub(crate) entity_id: EntityId,
    // username
    // uuid
}

impl<U: UniverseService> ProtoPlayer<U> {
    pub fn new(connection: U::ConnectionReferenceType, entity_id: EntityId) -> Self {
        Self {
            hardcore: false,

            write_buffer: WriteBuffer::new(),
            entity_id,

            connection,
        }
    }

    pub(crate) fn create_player<P: PlayerService<UniverseServiceType = U>>(
        mut self,
        service: P,
        world: &mut World<P::WorldServiceType>,
        position: Position,
    ) -> anyhow::Result<Player<P>> {
        // Fill write buffer with required initial packets

        // todo: dont send all these packets if the player is in the same world
        // i.e. the player had it's PlayerService changed
        // holdup: implementing dimension ids to be able to differentiate worlds

        // todo: if new, send join game
        world.write_game_join_packet(&mut self)?;
        world
            .get_universe()
            .write_brand_packet(&mut self.write_buffer)?;

        // todo: if dim changed, send dimension changed
        // todo: else, don't send

        let view_position = world.initialize_view_position(&mut self, position)?;

        // Write the necessary packets to the TCP stream
        self.connection.write_bytes(self.write_buffer.get_written());

        let player = Player::new(
            service,
            world,
            self.entity_id,
            position,
            view_position,
            self.connection,
        );

        Ok(player)
    }
}
