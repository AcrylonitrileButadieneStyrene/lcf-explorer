use lcf::{
    enums::Trigger,
    helpers::{Array, Chunk, Number, UnknownChunk},
    raw::ldb::{
        LcfDataBaseChunk, RawLcfDataBase, chipset::ChipSetChunk, common_event::CommonEventChunk,
        switch::SwitchChunk, variable::VariableChunk,
    },
};

pub fn update(
    database: &RawLcfDataBase,
    builder: &mut egui_ltreeview::TreeViewBuilder<'_, u64>,
    encoding: crate::code_page::CodePage,
) {
    for (index, chunk) in database.0.inner_vec.iter().enumerate() {
        let node = index as u64;
        match &chunk.data {
            LcfDataBaseChunk::Actors(bytes) => builder.leaf(node, format!("Actors: {bytes:?}")),
            LcfDataBaseChunk::Skills(bytes) => builder.leaf(node, format!("Skills: {bytes:?}")),
            LcfDataBaseChunk::Items(bytes) => builder.leaf(node, format!("Items: {bytes:?}")),
            LcfDataBaseChunk::Enemies(enemies) => {
                if builder.dir(node, "Enemies") {
                    for (index, (_, chunks)) in enemies.inner_vec.iter().enumerate() {
                        let node = node << 8;
                        if builder.dir(node, index.to_string()) {
                            draw_chunks(node + index as u64 + 1, &chunks, builder);
                        }
                        builder.close_dir();
                    }
                }
                builder.close_dir();
            }
            LcfDataBaseChunk::Troops(chunks) => {
                if builder.dir(node, "Troops") {
                    draw_chunks(node, &chunks, builder);
                }
                builder.close_dir();
            }
            LcfDataBaseChunk::Terrain(bytes) => builder.leaf(node, format!("Terrain: {bytes:?}")),
            LcfDataBaseChunk::Attributes(chunks) => {
                if builder.dir(node, "Attributes") {
                    draw_chunks(node, &chunks, builder);
                }
                builder.close_dir();
            }
            LcfDataBaseChunk::States(chunks) => {
                if builder.dir(node, "States") {
                    draw_chunks(node, &chunks, builder);
                }
                builder.close_dir();
            }
            LcfDataBaseChunk::Animations(bytes) => {
                builder.leaf(node, format!("Animations: {bytes:?}"))
            }
            LcfDataBaseChunk::ChipSet(chipset) => {
                if builder.dir(node, "Chipset") {
                    update_chipset(&chipset.inner_vec, builder, encoding, node);
                }
                builder.close_dir();
            }
            LcfDataBaseChunk::Terms(chunks) => {
                if builder.dir(node, "Terms") {
                    let node = node << 16;
                    builder.leaf(node, format!("Null terminated: {}", chunks.null_terminated));

                    for (index, chunk) in chunks.inner_vec.iter().enumerate() {
                        builder.leaf(
                            node + 1 + index as u64,
                            format!(
                                "{:?}: {}",
                                chunk.id,
                                encoding.to_encoding().decode(&chunk.bytes).0
                            ),
                        );
                    }
                }
                builder.close_dir();
            }
            LcfDataBaseChunk::System(chunks) => {
                if builder.dir(node, "System") {
                    draw_chunks(node, &chunks, builder);
                }
                builder.close_dir();
            }
            LcfDataBaseChunk::Switches(switches) => {
                if builder.dir(node, "Switches") {
                    let node = node << 16;
                    for (index, (id, chunks)) in switches.inner_vec.iter().enumerate() {
                        let node = (node + index as u64 + 1) << 8;
                        if builder.dir(node, id.to_string()) {
                            builder
                                .leaf(node, format!("Null terminated: {}", chunks.null_terminated));

                            for (index, chunk) in chunks.inner_vec.iter().enumerate() {
                                builder.leaf(
                                    node + index as u64 + 1,
                                    match &chunk.data {
                                        SwitchChunk::Name(bytes) => {
                                            encoding.to_encoding().decode(&bytes).0.to_string()
                                        }
                                        SwitchChunk::Unknown { id, bytes } => {
                                            format!("Field {id}: {bytes:?}")
                                        }
                                    },
                                )
                            }
                        }
                        builder.close_dir();
                    }
                }
                builder.close_dir();
            }
            LcfDataBaseChunk::Variables(variables) => {
                if builder.dir(node, "Variables") {
                    let node = node << 16;
                    for (index, (id, chunks)) in variables.inner_vec.iter().enumerate() {
                        let node = (node + index as u64 + 1) << 8;
                        if builder.dir(node, id.to_string()) {
                            builder
                                .leaf(node, format!("Null terminated: {}", chunks.null_terminated));

                            for (index, chunk) in chunks.inner_vec.iter().enumerate() {
                                builder.leaf(
                                    node + index as u64 + 1,
                                    match &chunk.data {
                                        VariableChunk::Name(bytes) => {
                                            encoding.to_encoding().decode(&bytes).0.to_string()
                                        }
                                        VariableChunk::Unknown { id, bytes } => {
                                            format!("Field {id}: {bytes:?}")
                                        }
                                    },
                                )
                            }
                        }
                        builder.close_dir();
                    }
                }
                builder.close_dir();
            }
            LcfDataBaseChunk::CommonEvents(events) => {
                if builder.dir(node, "Common Events") {
                    let node = node << 16;
                    for (index, (id, chunks)) in events.inner_vec.iter().enumerate() {
                        let node = (node + index as u64 + 1) << 8;
                        if builder.dir(node, id.to_string()) {
                            builder.leaf(
                                node + 1,
                                format!("Null terminated: {}", chunks.null_terminated),
                            );

                            for (index, chunk) in chunks.inner_vec.iter().enumerate() {
                                let node = node + index as u64 + 2;
                                builder.leaf(
                                    node,
                                    match &chunk.data {
                                        CommonEventChunk::Name(bytes) => {
                                            encoding.to_encoding().decode(&bytes).0.to_string()
                                        }
                                        CommonEventChunk::Trigger(val) => format!(
                                            "Trigger: {}",
                                            Trigger::try_from(val.0).map_or_else(
                                                |_| val.0.to_string(),
                                                |repr| {
                                                    match repr {
                                                        Trigger::ActionButton => "Action Button",
                                                        Trigger::PlayerTouch => "Player Touch",
                                                        Trigger::EventTouch => "Event Touch",
                                                        Trigger::Autorun => "Autorun",
                                                        Trigger::Parallel => "Parallel process",
                                                    }
                                                    .to_string()
                                                },
                                            )
                                        ),
                                        CommonEventChunk::SwitchState(val) => {
                                            format!("Switch state: {}", val.0 != 0)
                                        }
                                        CommonEventChunk::SwitchID(val) => {
                                            format!("Switch: {}", val.0)
                                        }
                                        CommonEventChunk::CommandsSize(val) => {
                                            format!("Commands size: {}", val.0)
                                        }
                                        CommonEventChunk::Commands(commands) => {
                                            builder.dir(node, "Commands");
                                            for (index, command) in commands.0.iter().enumerate() {
                                                builder.leaf(
                                                    (node << 8) + index as u64,
                                                    format!(
                                                        "{index}: {}{:?} {}",
                                                        "\t".repeat(command.indent as usize),
                                                        command.instruction,
                                                        encoding
                                                            .to_encoding()
                                                            .decode(&command.string)
                                                            .0,
                                                    ),
                                                );
                                            }
                                            builder.close_dir();
                                            continue;
                                        }
                                        CommonEventChunk::Unknown { id, bytes } => {
                                            format!("Field {id}: {bytes:?}")
                                        }
                                    },
                                )
                            }
                        }
                        builder.close_dir();
                    }
                }
                builder.close_dir();
            }
            LcfDataBaseChunk::Version(items) => {
                builder.leaf(node, format!("Version: {items:?}"));
            }
            LcfDataBaseChunk::Unknown { id, bytes } => {
                builder.leaf(node, format!("Chunk {}: {bytes:?}", id));
            }
        }
    }
}

fn update_chipset(
    chipsets: &[(Number, Array<Chunk<ChipSetChunk>>)],
    builder: &mut egui_ltreeview::TreeViewBuilder<'_, u64>,
    encoding: crate::code_page::CodePage,
    node: u64,
) {
    let node = node << 16;
    for (index, (id, chunks)) in chipsets.iter().enumerate() {
        let node = node + index as u64;
        builder.dir(node, format!("ChipSet {}", id.0));

        let node = node << 8;
        for (index, field) in chunks.inner_vec.iter().enumerate() {
            let label = match &field.data {
                ChipSetChunk::Name(bytes) => {
                    format!("Name: {}", encoding.to_encoding().decode(bytes).0)
                }
                ChipSetChunk::File(bytes) => {
                    format!("File: {}", encoding.to_encoding().decode(bytes).0)
                }
                ChipSetChunk::Terrain(items) => format!("Terrain: {items:?}"),
                ChipSetChunk::PassabilityLower(items) => format!("Passable (Lower): {items:?}"),
                ChipSetChunk::PassabilityUpper(items) => format!("Passable (Upper): {items:?}"),
                ChipSetChunk::AnimationType(val) => format!("Animation Type: {}", val.0),
                ChipSetChunk::AnimationSpeed(val) => format!("Animation Speed: {}", val.0),
                ChipSetChunk::Unknown { id, bytes } => {
                    format!("Field {}: {:?}", id, bytes)
                }
            };
            builder.leaf(node + index as u64, label);
        }
        builder.close_dir();
    }
}

fn draw_chunks(
    node: u64,
    chunks: &Array<Chunk<UnknownChunk>>,
    builder: &mut egui_ltreeview::TreeViewBuilder<'_, u64>,
) {
    let node = node << 16;
    builder.leaf(node, format!("Null terminated: {}", chunks.null_terminated));

    for (index, chunk) in chunks.inner_vec.iter().enumerate() {
        let UnknownChunk::Unknown { bytes, id } = &chunk.data;
        builder.leaf(node + 1 + index as u64, format!("Field {id}: {bytes:?}"));
    }
}
