pub fn update(
    database: &lcf::ldb::LcfDataBase,
    builder: &mut egui_ltreeview::TreeViewBuilder<'_, u64>,
    encoding: crate::code_page::CodePage,
) {
    if builder.dir(10, "ChipSets") {
        let node = 10 << 16;
        for (index, chipset) in database.chipsets.iter().enumerate() {
            let node = node + index as u64;
            if builder.dir(node, (index + 1).to_string()) {
                let node = node << 3;

                if !chipset.name.is_empty() {
                    builder.leaf(
                        node,
                        format!("Name: {}", encoding.to_encoding().decode(&chipset.name).0),
                    );
                }

                if !chipset.file.is_empty() {
                    builder.leaf(
                        node + 1,
                        format!("File: {}", encoding.to_encoding().decode(&chipset.file).0),
                    );
                }

                if chipset.animation_type != 0 {
                    builder.leaf(
                        node + 5,
                        format!("Animation Type: {}", chipset.animation_type),
                    );
                }

                if chipset.animation_speed != 0 {
                    builder.leaf(
                        node + 6,
                        format!("Animation Speed: {}", chipset.animation_speed),
                    );
                }
            }
            builder.close_dir();
        }
    }
    builder.close_dir();
}
