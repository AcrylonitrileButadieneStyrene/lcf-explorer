use lcf::raw::lsd::{LcfSaveDataChunk, RawLcfSaveData, SaveSystemChunk};

pub fn update(
    save_data: &RawLcfSaveData,
    builder: &mut egui_ltreeview::TreeViewBuilder<'_, u64>,
    _encoding: crate::code_page::CodePage,
) {
    for (index, chunk) in save_data.0.inner_vec.iter().enumerate() {
        match &chunk.data {
            LcfSaveDataChunk::Title { bytes } => {
                builder.leaf(index as u64, format!("Title: {bytes:?}"))
            }
            LcfSaveDataChunk::System(chunks) => {
                if builder.dir(index as u64, "System") {
                    let node = (index as u64) << 8;
                    for (index, chunk) in chunks.iter().enumerate() {
                        let node = node + index as u64;
                        match &chunk.data {
                            SaveSystemChunk::SwitchesSize(val) => {
                                builder.leaf(node, format!("Switches Size: {}", val.0))
                            }
                            SaveSystemChunk::Switches(numbers) => {
                                if builder.dir(node, "Switches") {
                                    for (index, number) in numbers.iter().enumerate() {
                                        builder.leaf(
                                            (node << 16) + index as u64,
                                            format!("{index}: {}", number.0 != 0),
                                        );
                                    }
                                }
                                builder.close_dir();
                            }
                            SaveSystemChunk::VariablesSize(val) => {
                                builder.leaf(node, format!("Variables Size: {}", val.0))
                            }
                            SaveSystemChunk::Variables(numbers) => {
                                if builder.dir(node, "Variables") {
                                    for (index, number) in numbers.iter().enumerate() {
                                        builder.leaf(
                                            (node << 16) + index as u64,
                                            format!("{index}: {}", number),
                                        );
                                    }
                                }
                                builder.close_dir();
                            }
                            SaveSystemChunk::Unknown { id, bytes } => {
                                builder.leaf(node, format!("Chunk {}: {bytes:?}", id))
                            }
                        }
                    }
                }
                builder.close_dir();
            }
            LcfSaveDataChunk::Screen { bytes } => {
                builder.leaf(index as u64, format!("Screen: {bytes:?}"))
            }
            LcfSaveDataChunk::Pictures { bytes } => {
                builder.leaf(index as u64, format!("Pictures: {bytes:?}"))
            }
            LcfSaveDataChunk::PartyLocation { bytes } => {
                builder.leaf(index as u64, format!("Party Location: {bytes:?}"))
            }
            LcfSaveDataChunk::BoatLocation { bytes } => {
                builder.leaf(index as u64, format!("Boat Location: {bytes:?}"))
            }
            LcfSaveDataChunk::ShipLocation { bytes } => {
                builder.leaf(index as u64, format!("Ship Location: {bytes:?}"))
            }
            LcfSaveDataChunk::AirshipLocation { bytes } => {
                builder.leaf(index as u64, format!("Airship Location: {bytes:?}"))
            }
            LcfSaveDataChunk::Actors { bytes } => {
                builder.leaf(index as u64, format!("Actors: {bytes:?}"))
            }
            LcfSaveDataChunk::Inventory { bytes } => {
                builder.leaf(index as u64, format!("Inventory: {bytes:?}"))
            }
            LcfSaveDataChunk::Targets { bytes } => {
                builder.leaf(index as u64, format!("Targets: {bytes:?}"))
            }
            LcfSaveDataChunk::MapInfo { bytes } => {
                builder.leaf(index as u64, format!("Map Info: {bytes:?}"))
            }
            LcfSaveDataChunk::Panorama { bytes } => {
                builder.leaf(index as u64, format!("Panorama: {bytes:?}"))
            }
            LcfSaveDataChunk::ExecutionState { bytes } => {
                builder.leaf(index as u64, format!("Execution State: {bytes:?}"))
            }
            LcfSaveDataChunk::CommonEvents { bytes } => {
                builder.leaf(index as u64, format!("Common Events: {bytes:?}"))
            }
            LcfSaveDataChunk::EasyRPG { bytes } => {
                builder.leaf(index as u64, format!("EasyRPG: {bytes:?}"))
            }
            LcfSaveDataChunk::Unknown { id, bytes } => {
                builder.leaf(index as u64, format!("Chunk {}: {bytes:?}", id))
            }
        };
    }
}
