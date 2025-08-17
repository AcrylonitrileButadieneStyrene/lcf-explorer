use lcf::ldb::{
    LcfDataBaseChunk,
    chipset::{ChipSet, ChipSetChunk},
};

pub fn update(
    database: &lcf::ldb::LcfDataBase,
    builder: &mut egui_ltreeview::TreeViewBuilder<'_, u64>,
    encoding: crate::code_page::CodePage,
) {
    for (index, chunk) in database.0.inner_vec.iter().enumerate() {
        let node = index as u64;
        match &chunk.data {
            lcf::ldb::LcfDataBaseChunk::ChipSet(chipset) => {
                builder.dir(node, "Chipset");
                update_chipset(chipset, builder, encoding, node);
                builder.close_dir();
            }
            LcfDataBaseChunk::Unknown { id, .. } => {
                builder.leaf(node, format!("Chunk {}", id.0));
            }
        }
    }
}

fn update_chipset(
    chipset: &ChipSet,
    builder: &mut egui_ltreeview::TreeViewBuilder<'_, u64>,
    encoding: crate::code_page::CodePage,
    node: u64,
) {
    let node = node << 16;
    for (index, chipsets) in chipset.data.iter().enumerate() {
        let node = node + index as u64;
        builder.dir(node, format!("ChipSet {}", chipsets.index.0));

        let node = node << 8;
        for (index, field) in chipsets.chunks.inner_vec.iter().enumerate() {
            let label = match &field.data {
                ChipSetChunk::Name(bytes) => {
                    format!("Name: {}", encoding.to_encoding().decode(bytes).0)
                }
                ChipSetChunk::File(bytes) => {
                    format!("File: {}", encoding.to_encoding().decode(bytes).0)
                }
                ChipSetChunk::Unknown { id, bytes } => {
                    format!("Field {}: [{:?}]", id.0, bytes)
                }
            };
            builder.leaf(node + index as u64, label);
        }
        builder.close_dir();
    }
}
