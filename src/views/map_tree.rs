use lcf::{
    helpers::{Array, Chunk, Number, ToChunkID},
    lmt::{bgm::MapBGM, map::MapChunk, start::StartChunk},
};

pub fn update(
    map_tree: &lcf::lmt::LcfMapTree,
    builder: &mut egui_ltreeview::TreeViewBuilder<'_, String>,
    encoding: crate::code_page::CodePage,
) {
    update_maps(&map_tree.maps, builder, encoding);

    builder.dir("order".to_string(), "Order");
    for (index, id) in map_tree.order.iter().enumerate() {
        builder.leaf(format!("order-{}", id.0), format!("{index}: {}", id.0));
    }
    builder.close_dir();

    builder.leaf(
        "active".to_string(),
        format!("Active: {}", map_tree.active.0),
    );

    builder.dir("start".to_string(), "Start");
    for chunk in &map_tree.start.inner_vec {
        let label = match &chunk.data {
            StartChunk::PartyMapID(val) => format!("Party Map ID: {}", val.0),
            StartChunk::PartyX(val) => format!("Party X: {}", val.0),
            StartChunk::PartyY(val) => format!("Party Y: {}", val.0),
            StartChunk::BoatMapID(val) => format!("Boat Map ID: {}", val.0),
            StartChunk::BoatX(val) => format!("Boat X: {}", val.0),
            StartChunk::BoatY(val) => format!("Boat Y: {}", val.0),
            StartChunk::ShipMapID(val) => format!("Ship Map ID: {}", val.0),
            StartChunk::ShipX(val) => format!("Ship X: {}", val.0),
            StartChunk::ShipY(val) => format!("Ship Y: {}", val.0),
            StartChunk::AirshipMapID(val) => format!("Airship Map ID: {}", val.0),
            StartChunk::AirshipX(val) => format!("AirshipX: {}", val.0),
            StartChunk::AirshipY(val) => format!("AirshipY: {}", val.0),
            StartChunk::Unknown { id, bytes } => {
                format!("Field {}: [{:?}]", id.0, bytes)
            }
        };
        builder.leaf(format!("start-{}", chunk.data.id().0), label);
    }
    builder.close_dir();
}

pub fn update_maps(
    maps: &[(Number, Array<Chunk<MapChunk>>)],
    builder: &mut egui_ltreeview::TreeViewBuilder<'_, String>,
    encoding: crate::code_page::CodePage,
) {
    builder.dir("maps".to_string(), "Maps");
    for (id, chunks) in maps {
        builder.dir(format!("map-{}", id.0), format!("Map {}", id.0));
        for chunk in &chunks.inner_vec {
            let node = move || format!("map-{}-chunk-{}", id.0, chunk.data.id().0);
            match &chunk.data {
                MapChunk::AreaRange {
                    begin_x,
                    begin_y,
                    end_x,
                    end_y,
                } => {
                    builder.dir(node(), "Area Range");
                    let node = node();
                    builder.leaf(format!("{node}-1"), format!("Begin X: {begin_x}"));
                    builder.leaf(format!("{node}-2"), format!("Begin Y: {begin_y}"));
                    builder.leaf(format!("{node}-3"), format!("End X: {end_x}"));
                    builder.leaf(format!("{node}-4"), format!("End Y: {end_y}"));
                    builder.close_dir();
                }
                MapChunk::BGMData(chunks) => {
                    builder.dir(node(), "BGM Data");
                    for chunk in &chunks.inner_vec {
                        let label = match &chunk.data {
                            MapBGM::FileName(bytes) => {
                                format!("File Name: {}", encoding.to_encoding().decode(bytes).0)
                            }
                            MapBGM::FadeInTime(val) => format!("Fade-in Time: {}", val.0),
                            MapBGM::Volume(val) => {
                                format!("Volume: {}", val.0)
                            }
                            MapBGM::Tempo(val) => {
                                format!("Tempo: {}", val.0)
                            }
                            MapBGM::Balance(val) => {
                                format!("Balance: {}", val.0)
                            }
                            MapBGM::Unknown { id, bytes } => {
                                format!("Field {}: {:?}", id.0, bytes)
                            }
                        };
                        builder.leaf(format!("{}-{}", node(), chunk.data.id().0), label)
                    }
                    builder.close_dir();
                }
                other => {
                    let label = match other {
                        MapChunk::Name(bytes) => {
                            format!("Name: {}", encoding.to_encoding().decode(bytes).0)
                        }
                        MapChunk::Parent(val) => format!("Parent: {}", val.0),
                        MapChunk::Type(val) => format!("Type: {}", val.0),
                        MapChunk::HorizontalScrollBar(val) => {
                            format!("Horizontal Scroll Bar: {}", val.0)
                        }
                        MapChunk::VerticalScrollBar(val) => {
                            format!("Vertical Scroll Bar: {}", val.0)
                        }
                        MapChunk::ExtractedNode(val) => format!("Extracted Node: {}", val.0),
                        MapChunk::BGM(val) => format!("BGM: {}", val.0),
                        MapChunk::Backdrop(val) => format!("Backdrop: {}", val.0),
                        MapChunk::BackdropFile(bytes) => format!("Backdrop File: {:?}", bytes),
                        MapChunk::Teleport(val) => format!("Teleport: {}", val.0),
                        MapChunk::Escape(val) => format!("Escape: {}", val.0),
                        MapChunk::Save(val) => format!("Save: {}", val.0),
                        MapChunk::EncounterEnemyGroup(bytes) => {
                            format!("Encounter Enemy Group: {:?}", bytes)
                        }
                        MapChunk::EnemyAppearStep(val) => {
                            format!("Enemy Appear Step: {}", val.0)
                        }
                        MapChunk::Unknown { id, bytes } => {
                            format!("Field {}: {:?}", id.0, bytes)
                        }
                        MapChunk::BGMData(_) | MapChunk::AreaRange { .. } => unreachable!(),
                    };
                    builder.leaf(node(), label);
                }
            };
        }
        builder.close_dir();
    }
    builder.close_dir();
}
