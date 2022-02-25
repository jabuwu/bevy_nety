use crate::{entity::NetworkEntity, player::NetworkPlayer, server::NetworkServerEntity};
use std::collections::HashMap;

struct NetworkRelevancyEntry {
    spawned: bool,
    despawned: bool,
    relevant: bool,
    manual_relevancy: bool,
}

pub(crate) enum NetworkRelevancyState {
    Spawn,
    Despawn,
    Relevant,
    Irrelevant,
}

#[derive(Default)]
pub(crate) struct NetworkRelevancy {
    // TODO: cleanup this hashmap
    relevancy: HashMap<NetworkPlayer, HashMap<NetworkEntity, NetworkRelevancyEntry>>,
}

impl NetworkRelevancy {
    fn get_or_insert_entry(
        &mut self,
        player: NetworkPlayer,
        entity: NetworkEntity,
    ) -> &mut NetworkRelevancyEntry {
        let entity_map = self
            .relevancy
            .entry(player)
            .or_insert_with(|| HashMap::new());
        entity_map
            .entry(entity)
            .or_insert_with(|| NetworkRelevancyEntry {
                spawned: false,
                despawned: false,
                relevant: true,
                manual_relevancy: true,
            })
    }

    pub(crate) fn update(
        &mut self,
        player: NetworkPlayer,
        entity: &NetworkServerEntity,
        force_relevant: bool,
    ) -> NetworkRelevancyState {
        let entry = self.get_or_insert_entry(player, entity.handle);
        entry.relevant = entry.manual_relevancy || force_relevant;
        if entry.relevant || force_relevant {
            if entry.spawned {
                NetworkRelevancyState::Relevant
            } else {
                entry.spawned = true;
                entry.despawned = false;
                NetworkRelevancyState::Spawn
            }
        } else {
            if entry.despawned {
                NetworkRelevancyState::Despawn
            } else {
                entry.despawned = true;
                entry.spawned = false;
                NetworkRelevancyState::Irrelevant
            }
        }
    }

    pub(crate) fn relevant(&mut self, player: NetworkPlayer, entity: NetworkEntity) -> bool {
        self.get_or_insert_entry(player, entity).relevant
    }

    pub(crate) fn set_relevant(
        &mut self,
        player: NetworkPlayer,
        entity: NetworkEntity,
        relevant: bool,
    ) {
        self.get_or_insert_entry(player, entity).manual_relevancy = relevant;
    }
}
