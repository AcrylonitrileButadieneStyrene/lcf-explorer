use lcf::{
    helpers::ToChunkID as _,
    lmu::{LcfMapUnit, LcfMapUnitChunk},
};

pub fn update(
    map_unit: &LcfMapUnit,
    builder: &mut egui_ltreeview::TreeViewBuilder<'_, String>,
    _encoding: crate::code_page::CodePage,
) {
    for chunk in &map_unit.0.inner_vec {
        let node = format!("chunk-{}", chunk.data.id().0);
        match &chunk.data {
            LcfMapUnitChunk::ChipSet(val) => builder.leaf(node, format!("ChipSet: {}", val.0)),
            LcfMapUnitChunk::Width(val) => builder.leaf(node, format!("Width: {}", val.0)),
            LcfMapUnitChunk::Height(val) => builder.leaf(node, format!("Height: {}", val.0)),
            LcfMapUnitChunk::ScrollType(val) => builder.leaf(
                node,
                format!(
                    "Scroll Type: {}",
                    match val.0 {
                        0 => "No Loop",
                        1 => "Vertical Loop Only",
                        2 => "Horizontal Loop Only",
                        3 => "Vertical and Horizontal Loop",
                        _ => "Invalid",
                    }
                ),
            ),
            LcfMapUnitChunk::Lower(layer) => {
                builder.leaf(node, format!("Lower: {:?}", layer.inner_vec))
            }
            LcfMapUnitChunk::Upper(layer) => {
                builder.leaf(node, format!("Upper: {:?}", layer.inner_vec))
            }
            LcfMapUnitChunk::SaveTime(val) => builder.leaf(node, format!("Save Time: {}", val.0)),
            LcfMapUnitChunk::Unknown { id, bytes } => {
                builder.leaf(node, format!("Chunk {}: {bytes:?}", id.0));
            }
        }
    }
}
