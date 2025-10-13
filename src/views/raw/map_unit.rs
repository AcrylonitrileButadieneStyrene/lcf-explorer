use lcf::raw::lmu::{
    LcfMapUnitChunk, RawLcfMapUnit,
    event::{
        EventChunk, condition::EventPageConditionChunk, move_route::EventMoveRouteChunk,
        page::EventPageChunk,
    },
};

pub fn update(
    map_unit: &RawLcfMapUnit,
    builder: &mut egui_ltreeview::TreeViewBuilder<'_, u64>,
    encoding: crate::code_page::CodePage,
) {
    for (index, chunk) in map_unit.0.inner_vec.iter().enumerate() {
        let node = index as u64;
        let label = match &chunk.data {
            LcfMapUnitChunk::ChipSet(val) => format!("ChipSet: {}", val.0),
            LcfMapUnitChunk::Width(val) => format!("Width: {}", val.0),
            LcfMapUnitChunk::Height(val) => format!("Height: {}", val.0),
            LcfMapUnitChunk::ScrollType(val) => format!(
                "Scroll Type: {}",
                match val.0 {
                    0 => "No Loop",
                    1 => "Vertical Loop Only",
                    2 => "Horizontal Loop Only",
                    3 => "Vertical and Horizontal Loop",
                    _ => "Invalid",
                }
            ),
            LcfMapUnitChunk::PanoramaEnabled(val) => {
                format!("Panorama Enabled: {}", val.0 != 0)
            }
            LcfMapUnitChunk::PanoramaFile(bytes) => {
                format!("Panorama File: {}", encoding.to_encoding().decode(bytes).0)
            }
            LcfMapUnitChunk::PanoramaHorizontalLoop(val) => {
                format!("Panorama Horizontal Loop: {}", val.0 != 0)
            }
            LcfMapUnitChunk::PanoramaVerticalLoop(val) => {
                format!("Panorama Vertical Loop: {}", val.0 != 0)
            }
            LcfMapUnitChunk::PanoramaHorizontalAutoScroll(val) => {
                format!("Panorama Horizontal Auto Scroll: {}", val.0 != 0)
            }
            LcfMapUnitChunk::PanoramaHorizontalAutoScrollSpeed(val) => {
                format!("Panorama Horizontal Auto Scroll Speed: {}", val.0)
            }
            LcfMapUnitChunk::PanoramaVerticalAutoScroll(val) => {
                format!("Panorama Vertical Auto Scroll: {}", val.0 != 0)
            }
            LcfMapUnitChunk::PanoramaVerticalAutoScrollSpeed(val) => {
                format!("Panorama Vertical Auto Scroll Speed: {}", val.0)
            }
            LcfMapUnitChunk::Events { chunks } => {
                builder.dir(node, "Events");
                let node = node << 4;
                for (index, (id, events)) in chunks.into_iter().enumerate() {
                    let node = node + index as u64;
                    builder.dir(node, format!("Event {}", id.0));
                    let node = node << 14;
                    for (index, event) in events.iter().enumerate() {
                        let node = node + index as u64;
                        let label = match &event.data {
                            EventChunk::Name(bytes) => {
                                format!("Name: {}", encoding.to_encoding().decode(bytes).0)
                            }
                            EventChunk::PositionX(val) => format!("X: {}", val.0),
                            EventChunk::PositionY(val) => format!("Y: {}", val.0),
                            EventChunk::Pages { chunks } => {
                                builder.dir(node, "Pages");
                                let node = node << 7;
                                for (index, (id, page)) in chunks.iter().enumerate() {
                                    builder.dir(node, format!("Page {}", id.0));
                                    let node = node << 8 + index as u64;
                                    for (index, chunk) in page.inner_vec.iter().enumerate() {
                                        let node = node + index as u64;
                                        let label = match &chunk.data {
                                            EventPageChunk::Condition(chunks) => {
                                                builder.dir(node, "Condition");
                                                for (index, chunk) in
                                                    chunks.inner_vec.iter().enumerate()
                                                {
                                                    let node = node << 8 + index as u64;
                                                    let label = match &chunk.data {
                                                        EventPageConditionChunk::Flags(x) => {
                                                            format!("Flags: {}", x.0)
                                                        }
                                                        EventPageConditionChunk::SwitchA(x) => {
                                                            format!("Switch A: {}", x.0)
                                                        }
                                                        EventPageConditionChunk::SwitchB(x) => {
                                                            format!("Switch B: {}", x.0)
                                                        }
                                                        EventPageConditionChunk::Variable(x) => {
                                                            format!("Variable: {}", x.0)
                                                        }
                                                        EventPageConditionChunk::Value(x) => {
                                                            format!("Value: {}", x.0)
                                                        }
                                                        EventPageConditionChunk::Item(x) => {
                                                            format!("Item: {}", x.0)
                                                        }
                                                        EventPageConditionChunk::Actor(x) => {
                                                            format!("Actor: {}", x.0)
                                                        }
                                                        EventPageConditionChunk::Timer(x) => {
                                                            format!("Timer: {}", x.0)
                                                        }
                                                        EventPageConditionChunk::Unknown {
                                                            id,
                                                            bytes,
                                                        } => format!("Field {}: {bytes:?}", id),
                                                    };
                                                    builder.leaf(node, label);
                                                }
                                                builder.close_dir();
                                                continue;
                                            }
                                            EventPageChunk::GraphicFile(bytes) => format!(
                                                "Graphic: {}",
                                                encoding.to_encoding().decode(bytes).0
                                            ),
                                            EventPageChunk::GraphicIndex(val) => {
                                                format!("Graphic Index: {}", val.0)
                                            }
                                            EventPageChunk::GraphicDirection(val) => format!(
                                                "Graphic Direction: {}",
                                                match val.0 {
                                                    0 => "Up",
                                                    1 => "Right",
                                                    2 => "Down",
                                                    3 => "Left",
                                                    _ => "Unknown",
                                                }
                                            ),
                                            EventPageChunk::GraphicPattern(val) => {
                                                format!("Graphic Pattern: {}", val.0)
                                            }
                                            EventPageChunk::GraphicTransparent(val) => {
                                                format!("Graphic Transparent: {}", val.0 != 0)
                                            }
                                            EventPageChunk::MovementType(val) => format!(
                                                "Movement Type: {}",
                                                match val.0 {
                                                    0 => "Fixed",
                                                    1 => "Random",
                                                    2 => "Vertical",
                                                    3 => "Horizontal",
                                                    4 => "Approach Player",
                                                    5 => "Away from Player",
                                                    6 => "Custom",
                                                    _ => "Unknown",
                                                }
                                            ),
                                            EventPageChunk::MovementFrequency(val) => {
                                                format!("Movement Frequency: {}", val.0)
                                            }
                                            EventPageChunk::MovementRoute(chunks) => {
                                                builder.dir(node, "Move Route");
                                                for (index, chunk) in
                                                    chunks.inner_vec.iter().enumerate()
                                                {
                                                    let node = node << 8 + index as u64;
                                                    let label = match &chunk.data {
                                                        EventMoveRouteChunk::CommandsSize(val) => {
                                                            format!("CommandsSize: {}", val.0)
                                                        }
                                                        EventMoveRouteChunk::Commands(val) => {
                                                            format!("Commands: {:?}", val)
                                                        }
                                                        EventMoveRouteChunk::Repeat(val) => {
                                                            format!("Repeat: {}", val.0)
                                                        }
                                                        EventMoveRouteChunk::Skippable(val) => {
                                                            format!("Skippable: {}", val.0)
                                                        }
                                                        EventMoveRouteChunk::Unknown {
                                                            id,
                                                            bytes,
                                                        } => format!("Field {}: {bytes:?}", id),
                                                    };
                                                    builder.leaf(node, label);
                                                }
                                                builder.close_dir();
                                                continue;
                                            }
                                            EventPageChunk::Trigger(val) => format!(
                                                "Trigger: {}",
                                                match val.0 {
                                                    0 => "Action Button",
                                                    1 => "Player Touch",
                                                    2 => "Event Touch",
                                                    3 => "Autorun",
                                                    4 => "Parallel process",
                                                    _ => "Unknown",
                                                }
                                            ),
                                            EventPageChunk::Priority(val) => format!(
                                                "Priority: {}",
                                                match val.0 {
                                                    0 => "Below Characters",
                                                    1 => "Same as Characters",
                                                    2 => "Above Characters",
                                                    _ => "Unknown",
                                                }
                                            ),
                                            EventPageChunk::PriorityForbidEventOverlap(val) => {
                                                format!("Forbid Event Overlap: {}", val.0)
                                            }
                                            EventPageChunk::AnimationType(val) => format!(
                                                "Animation Type: {}",
                                                match val.0 {
                                                    0 => "Standing Animation",
                                                    1 => "Walking Animation",
                                                    2 => "Direction Fix/Inanimated",
                                                    3 => "Direction Fix/Animated",
                                                    4 => "Fixed Graphic",
                                                    5 => "Spin",
                                                    _ => "Unknown",
                                                }
                                            ),
                                            EventPageChunk::MoveSpeed(val) => {
                                                format!("Movement Speed: {}", val.0)
                                            }
                                            EventPageChunk::CommandsSize(val) => {
                                                format!("Commands size: {}", val.0)
                                            }
                                            EventPageChunk::Commands(commands) => {
                                                builder.dir(node, "Commands");
                                                for (index, command) in
                                                    commands.0.iter().enumerate()
                                                {
                                                    builder.leaf(
                                                        node.unbounded_shl(8) + index as u64,
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
                                            EventPageChunk::Unknown { id, bytes } => {
                                                format!("Field {}: {bytes:?}", id)
                                            }
                                        };
                                        builder.leaf(node, label);
                                    }
                                    builder.close_dir();
                                }
                                builder.close_dir();
                                continue;
                            }
                            EventChunk::Unknown { id, bytes } => {
                                format!("Field {}: {bytes:?}", id)
                            }
                        };
                        builder.leaf(node, label);
                    }
                    builder.close_dir();
                }
                builder.close_dir();
                continue;
            }
            LcfMapUnitChunk::Lower(layer) => {
                format!("Lower: {layer:?}")
            }
            LcfMapUnitChunk::Upper(layer) => {
                format!("Upper: {layer:?}")
            }
            LcfMapUnitChunk::SaveTime(val) => format!("Save Time: {}", val.0),
            LcfMapUnitChunk::Unknown { id, bytes } => {
                format!("Chunk {}: {bytes:?}", id)
            }
        };
        builder.leaf(node, label);
    }
}
