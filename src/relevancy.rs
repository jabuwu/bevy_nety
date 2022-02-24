use crate::{entity::NetworkEntity, player::NetworkPlayer, server::NetworkServerEntity};
use std::collections::HashMap;

pub(crate) enum NetworkRelevancyState {
    Spawn,
    Despawn,
    Relevant,
    Irrelevant,
}

#[derive(Default)]
pub(crate) struct NetworkRelevancy {
    // TODO: cleanup this hashmap
    relevancy: HashMap<NetworkPlayer, Vec<NetworkEntity>>,
}

impl NetworkRelevancy {
    pub(crate) fn update(
        &mut self,
        player: NetworkPlayer,
        entity: &NetworkServerEntity,
    ) -> NetworkRelevancyState {
        let vec = self.relevancy.entry(player).or_insert_with(|| Vec::new());
        if vec.contains(&entity.handle) {
            NetworkRelevancyState::Relevant
        } else {
            vec.push(entity.handle);
            NetworkRelevancyState::Spawn
        }
    }

    pub(crate) fn relevant(&self, player: NetworkPlayer, entity: NetworkEntity) -> bool {
        if let Some(vec) = self.relevancy.get(&player) {
            vec.contains(&entity)
        } else {
            false
        }
    }
}
