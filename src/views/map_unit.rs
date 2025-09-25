use lcf::enums::{AnimationType, Priority, Trigger};

pub fn update(
    map_unit: &lcf::lmu::LcfMapUnit,
    builder: &mut egui_ltreeview::TreeViewBuilder<'_, u64>,
    encoding: crate::code_page::CodePage,
) {
    map_unit
        .chipset
        .inspect(|chipset| builder.leaf(0, format!("ChipSet: {}", chipset)));
    builder.leaf(1, format!("Width: {}", map_unit.width));
    builder.leaf(2, format!("Height: {}", map_unit.height));
    builder.leaf(
        3,
        format!(
            "Scroll Type: {}",
            match map_unit.scroll_type {
                lcf::enums::ScrollType::None => "No Loop",
                lcf::enums::ScrollType::Vertical => "Vertical Loop Only",
                lcf::enums::ScrollType::Horizontal => "Horizontal Loop Only",
                lcf::enums::ScrollType::Both => "Vertical and Horizontal Loop",
            }
        ),
    );
    if builder.dir(4, "Panorama") {
        builder.leaf(5, format!("Enabled: {}", map_unit.panorama.enabled));
        builder.leaf(
            6,
            format!(
                "File: {}",
                encoding
                    .to_encoding()
                    .decode(map_unit.panorama.file.as_deref().unwrap_or_default())
                    .0
            ),
        );
        if builder.dir(7, "Horizontal") {
            builder.leaf(8, format!("Looping: {}", map_unit.panorama.horizontal_loop));
            builder.leaf(
                9,
                format!("Auto Scroll: {}", map_unit.panorama.horizontal_auto_scroll),
            );
            builder.leaf(
                10,
                format!(
                    "Auto Scroll Speed: {}",
                    map_unit.panorama.horizontal_auto_scroll_speed
                ),
            );
        }
        builder.close_dir();
        if builder.dir(11, "Vertical") {
            builder.leaf(12, format!("Looping: {}", map_unit.panorama.vertical_loop));
            builder.leaf(
                13,
                format!("Auto Scroll: {}", map_unit.panorama.vertical_auto_scroll),
            );
            builder.leaf(
                14,
                format!(
                    "Auto Scroll Speed: {}",
                    map_unit.panorama.vertical_auto_scroll_speed
                ),
            );
        }
        builder.close_dir();
        builder.close_dir();
        if builder.dir(15, "Events") {
            for event in &map_unit.events {
                let node = (event.id as u64) << 8;
                if builder.dir(
                    node,
                    format!(
                        "E{:04}: {}",
                        event.id,
                        encoding.to_encoding().decode(&event.name).0
                    ),
                ) {
                    builder.leaf(node + 1, format!("X: {}", event.x));
                    builder.leaf(node + 2, format!("Y: {}", event.y));
                    if builder.dir(node + 3, "Pages") {
                        let node = node << 2;
                        for page in &event.pages {
                            // TODO: trigger term

                            if builder.dir(node + 2, "Graphic") {
                                builder.leaf(
                                    node + 3,
                                    format!(
                                        "File: {}",
                                        encoding.to_encoding().decode(&page.graphic.file).0
                                    ),
                                );
                                builder.leaf(node + 4, format!("Index: {}", page.graphic.index));
                                builder.leaf(
                                    node + 5,
                                    format!("Direction: {}", page.graphic.direction),
                                );
                                builder
                                    .leaf(node + 6, format!("Pattern: {}", page.graphic.pattern));
                                builder.leaf(
                                    node + 7,
                                    format!("Transparent: {}", page.graphic.transparent),
                                );
                            }
                            builder.close_dir();

                            if builder.dir(node + 8, "Movement") {
                                builder.leaf(node + 9, format!("Type: {}", page.movement.r#type));
                                builder.leaf(
                                    node + 10,
                                    format!("Frequency: {}", page.movement.frequency),
                                );
                                builder.leaf(node + 11, format!("Speed: {}", page.movement.speed));
                                // builder.leaf(node + 12, format!("Route: {}", page.movement.route));
                            }
                            builder.close_dir();

                            builder.leaf(
                                node + 13,
                                format!(
                                    "Trigger: {}",
                                    match page.trigger {
                                        Trigger::ActionButton => "Action Button",
                                        Trigger::PlayerTouch => "Player Touch",
                                        Trigger::EventTouch => "Event Touch",
                                        Trigger::Autorun => "Autorun",
                                        Trigger::Parallel => "Parallel process",
                                    }
                                ),
                            );
                            builder.leaf(
                                node + 14,
                                format!(
                                    "Priority: {}",
                                    match page.priority {
                                        Priority::BelowCharacters => "Below Characters",
                                        Priority::SameAsCharacters => "Same as Characters",
                                        Priority::AboveCharacters => "Above Characters",
                                    }
                                ),
                            );
                            builder.leaf(
                                node + 15,
                                format!("Forbid Event Overlap: {}", page.forbid_event_overlap),
                            );
                            builder.leaf(
                                node + 16,
                                format!(
                                    "Animation Type: {}",
                                    match page.animation_type {
                                        AnimationType::Standing => "Standing Animation",
                                        AnimationType::Walking => "Walking Animation",
                                        AnimationType::DirectionFixInanimated =>
                                            "Direction Fix/Inanimated",
                                        AnimationType::DirectionFixAnimated =>
                                            "Direction Fix/Animated",
                                        AnimationType::FixedGraphic => "Fixed Graphic",
                                        AnimationType::Spin => "Spin",
                                    }
                                ),
                            );
                            if builder.dir(node + 17, "Commands") {
                                let node = node << 16;
                                for (index, command) in page.commands.iter().enumerate() {
                                    builder.leaf(
                                        node + index as u64,
                                        format!(
                                            "{index}: {}{:?} {}",
                                            "\t".repeat(command.indent as usize),
                                            command.instruction,
                                            encoding.to_encoding().decode(&command.string).0,
                                        ),
                                    )
                                }
                            }
                            builder.close_dir();
                        }
                    }
                    builder.close_dir();
                }
                builder.close_dir();
            }
        }
        builder.close_dir();
        builder.leaf(18, format!("Lower: {:?}", map_unit.lower));
        builder.leaf(19, format!("Upper: {:?}", map_unit.upper));
        builder.leaf(20, format!("Save Time: {}", map_unit.save_time));
    }
}
