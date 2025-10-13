use lcf::{
    helpers::{Array, Chunk, Number},
    raw::lmt::{RawLcfMapTree, bgm::MapBGMChunk, map::MapChunk, start::StartChunk},
};

pub fn update(
    map_tree: &RawLcfMapTree,
    builder: &mut egui_ltreeview::TreeViewBuilder<'_, u64>,
    encoding: crate::code_page::CodePage,
) {
    builder.leaf(0, format!("Active: {}", map_tree.active.0));
    builder.dir(1, "Order");
    for (index, id) in map_tree.order.iter().enumerate() {
        builder.leaf(2 + index as u64, format!("{index}: {}", id.0));
    }
    builder.close_dir();

    let node = 1 << 16;
    builder.dir(node, "Start");
    for (index, chunk) in map_tree.start.inner_vec.iter().enumerate() {
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
                format!("Field {}: [{:?}]", id, bytes)
            }
        };
        builder.leaf(node + 1 + index as u64, label);
    }
    builder.close_dir();

    update_maps(&map_tree.maps, builder, encoding);
}

pub fn update_maps(
    maps: &[(Number, Array<Chunk<MapChunk>>)],
    builder: &mut egui_ltreeview::TreeViewBuilder<'_, u64>,
    encoding: crate::code_page::CodePage,
) {
    let node = 2 << 16;
    builder.dir(node, "Maps");
    for (index, (id, chunks)) in maps.iter().enumerate() {
        let node = node + 1 + index as u64;
        builder.dir(node, format!("Map {}", id.0));
        let node = node << 8;
        for (index, chunk) in chunks.inner_vec.iter().enumerate() {
            let node = node + index as u64;
            match &chunk.data {
                MapChunk::AreaRange {
                    begin_x,
                    begin_y,
                    end_x,
                    end_y,
                } => {
                    builder.dir(node, "Area Range");
                    let node = node << 2;
                    builder.leaf(node, format!("Begin X: {begin_x}"));
                    builder.leaf(node + 1, format!("Begin Y: {begin_y}"));
                    builder.leaf(node + 2, format!("End X: {end_x}"));
                    builder.leaf(node + 3, format!("End Y: {end_y}"));
                    builder.close_dir();
                }
                MapChunk::BGMData(chunks) => {
                    builder.dir(node, "BGM Data");
                    let node = node << 4;
                    for (index, chunk) in chunks.inner_vec.iter().enumerate() {
                        let label = match &chunk.data {
                            MapBGMChunk::FileName(bytes) => {
                                format!("File Name: {}", encoding.to_encoding().decode(bytes).0)
                            }
                            MapBGMChunk::FadeInTime(val) => format!("Fade-in Time: {}", val.0),
                            MapBGMChunk::Volume(val) => {
                                format!("Volume: {}", val.0)
                            }
                            MapBGMChunk::Tempo(val) => {
                                format!("Tempo: {}", val.0)
                            }
                            MapBGMChunk::Balance(val) => {
                                format!("Balance: {}", val.0)
                            }
                            MapBGMChunk::Unknown { id, bytes } => {
                                format!("Field {}: {:?}", id, bytes)
                            }
                        };
                        builder.leaf(node + index as u64, label)
                    }
                    builder.close_dir();
                }
                other => {
                    let label = match other {
                        MapChunk::Name(bytes) => {
                            format!("Name: {}", encoding.to_encoding().decode(bytes).0)
                        }
                        MapChunk::Parent(val) => format!("Parent: {}", val.0),
                        MapChunk::Indentation(val) => format!("Indentation: {}", val.0),
                        MapChunk::Type(val) => format!("Type: {}", val.0),
                        MapChunk::HorizontalScrollBar(val) => {
                            format!("Horizontal Scroll Bar: {}", val.0)
                        }
                        MapChunk::VerticalScrollBar(val) => {
                            format!("Vertical Scroll Bar: {}", val.0)
                        }
                        MapChunk::Expanded(val) => format!("Expanded: {}", val.0 != 0),
                        MapChunk::BGM(val) => format!("BGM: {}", val.0),
                        MapChunk::Background(val) => format!("Background: {}", val.0),
                        MapChunk::BackgroundFile(bytes) => format!("Background File: {:?}", bytes),
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
                            format!("Field {}: {:?}", id, bytes)
                        }
                        MapChunk::BGMData(_) | MapChunk::AreaRange { .. } => unreachable!(),
                    };
                    builder.leaf(node, label);
                }
            };
        }
        builder.close_dir();
    }
    builder.close_dir();
}
