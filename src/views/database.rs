use lcf::{
    helpers::ToChunkID as _,
    ldb::{
        LcfDataBaseChunk,
        chipset::{ChipSet, ChipSetChunk},
    },
};

pub fn update(
    database: &lcf::ldb::LcfDataBase,
    builder: &mut egui_ltreeview::TreeViewBuilder<'_, String>,
    encoding: crate::code_page::CodePage,
) {
    for chunk in &database.0.inner_vec {
        let node = format!("chunk-{}", chunk.data.id().0);
        match &chunk.data {
            lcf::ldb::LcfDataBaseChunk::ChipSet(chipset) => {
                builder.dir(node, "Chipset");
                update_chipset(chipset, builder, encoding);
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
    builder: &mut egui_ltreeview::TreeViewBuilder<'_, String>,
    encoding: crate::code_page::CodePage,
) {
    for chipsets in &chipset.data {
        builder.dir(
            format!("chipset-{}", chipsets.index.0),
            format!("ChipSet {}", chipsets.index.0),
        );
        for field in &chipsets.chunks.inner_vec {
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
            builder.leaf(
                format!("chipset-{}-field-{}", chipsets.index.0, field.data.id().0),
                label,
            );
        }
        builder.close_dir();
    }
}
