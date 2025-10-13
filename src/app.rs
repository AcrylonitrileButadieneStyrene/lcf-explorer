struct Instance {
    name: String,
    raw: lcf::raw::RawLcf,
    converted: Result<lcf::Lcf, lcf::LcfReadError>,
}

pub struct App {
    selected: Option<usize>,
    instances: Vec<Instance>,
    encoding: crate::code_page::CodePage,
    using_raw: bool,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            selected: None,
            instances: Vec::new(),
            encoding: Default::default(),
            using_raw: false,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("title bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                if ui.button("Open").clicked()
                    && let Some(path) = rfd::FileDialog::new()
                        .add_filter("Lcf file", &["ldb", "lmt", "lmu", "lsd"])
                        .pick_file()
                {
                    let bytes = std::fs::read(&path).unwrap();
                    let mut cursor = std::io::Cursor::new(bytes);
                    let lcf = lcf::raw::RawLcf::read(&mut cursor).unwrap();
                    self.instances.push(Instance {
                        name: path.file_name().unwrap().to_str().unwrap().to_owned(),
                        converted: lcf.clone().try_into(),
                        raw: lcf,
                    });
                    self.selected = Some(self.instances.len() - 1);
                }

                ui.menu_button("Encoding", |ui| {
                    for encoding in crate::code_page::ALL {
                        if ui.button(encoding.to_str()).clicked() {
                            self.encoding = *encoding;
                        }
                    }
                });

                ui.toggle_value(&mut self.using_raw, "Raw");
            });
        });

        if !self.instances.is_empty() {
            egui::TopBottomPanel::top("tab bar").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    for (index, Instance { name, .. }) in self.instances.iter().enumerate() {
                        if ui
                            .radio(
                                self.selected.map_or_default(|selected| selected == index),
                                name,
                            )
                            .clicked()
                        {
                            self.selected = Some(index);
                        }
                    }
                });
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(selected) = self.selected {
                let Instance { raw, converted, .. } = &self.instances[selected];

                egui::ScrollArea::both().show(ui, |ui| {
                    egui_ltreeview::TreeView::new("tree".into()).show(ui, |builder| {
                        if self.using_raw {
                            match raw {
                                lcf::raw::RawLcf::RawDataBase(database) => {
                                    crate::views::raw::database::update(
                                        database,
                                        builder,
                                        self.encoding,
                                    )
                                }
                                lcf::raw::RawLcf::RawMapTree(map_tree) => {
                                    crate::views::raw::map_tree::update(
                                        map_tree,
                                        builder,
                                        self.encoding,
                                    )
                                }
                                lcf::raw::RawLcf::RawMapUnit(map_unit) => {
                                    crate::views::raw::map_unit::update(
                                        map_unit,
                                        builder,
                                        self.encoding,
                                    )
                                }
                                lcf::raw::RawLcf::RawSaveData(save_data) => {
                                    crate::views::raw::save_data::update(
                                        save_data,
                                        builder,
                                        self.encoding,
                                    )
                                }
                            };
                        } else {
                            match converted {
                                Ok(lcf::Lcf::DataBase(_database)) => todo!(),
                                Ok(lcf::Lcf::MapTree(_map_tree)) => todo!(),
                                Ok(lcf::Lcf::MapUnit(map_unit)) => {
                                    crate::views::map_unit::update(map_unit, builder, self.encoding)
                                }
                                Ok(lcf::Lcf::SaveData(_save_data)) => todo!(),
                                Err(_) => todo!(),
                            };
                        }
                    });
                });
            } else {
                ui.centered_and_justified(|ui| {
                    ui.heading("No files opened. Add a file from the top bar.");
                });
            }
        });
    }
}
