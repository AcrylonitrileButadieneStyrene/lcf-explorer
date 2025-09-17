use lcf::raw::RawLcf;

pub struct App {
    selected: Option<usize>,
    lcfs: Vec<(String, RawLcf)>,
    encoding: crate::code_page::CodePage,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            selected: None,
            lcfs: Vec::new(),
            encoding: Default::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("title bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                if ui.button("Open").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Lcf file", &["ldb", "lmt", "lmu", "lsd"])
                        .pick_file()
                    {
                        let bytes = std::fs::read(&path).unwrap();
                        let mut cursor = std::io::Cursor::new(bytes);
                        let lcf = RawLcf::read(&mut cursor).unwrap();
                        self.lcfs
                            .push((path.file_name().unwrap().to_str().unwrap().to_owned(), lcf));
                        self.selected = Some(self.lcfs.len() - 1);
                    }
                }

                ui.menu_button("Encoding", |ui| {
                    for encoding in crate::code_page::ALL {
                        if ui.button(encoding.to_str()).clicked() {
                            self.encoding = *encoding;
                        }
                    }
                });
            });
        });

        if !self.lcfs.is_empty() {
            egui::TopBottomPanel::top("tab bar").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    for (index, (name, _)) in self.lcfs.iter().enumerate() {
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
                let (_, lcf) = &self.lcfs[selected];
                egui::ScrollArea::both().show(ui, |ui| {
                    egui_ltreeview::TreeView::new("tree".into()).show(ui, |mut builder| {
                        match lcf {
                            RawLcf::RawDataBase(database) => crate::views::database::update(
                                database,
                                &mut builder,
                                self.encoding,
                            ),
                            RawLcf::RawMapTree(lcf_map_tree) => crate::views::map_tree::update(
                                lcf_map_tree,
                                &mut builder,
                                self.encoding,
                            ),
                            RawLcf::RawMapUnit(lcf_map_unit) => crate::views::map_unit::update(
                                lcf_map_unit,
                                &mut builder,
                                self.encoding,
                            ),
                            RawLcf::RawSaveData(lcf_save_data) => crate::views::save_data::update(
                                lcf_save_data,
                                &mut builder,
                                self.encoding,
                            ),
                        };
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
