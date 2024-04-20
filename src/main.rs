use eframe::egui;
use std::fs;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = native_options.viewport.with_inner_size([350.0,450.0]);
    let _ = eframe::run_native("To Do", native_options, Box::new(|cc| Box::new(ToDoList::new(cc))))
                .expect("Unexpected error");
    
}

#[derive(Default,Clone)]
struct ToDoList { 
    pub text:String,
    selected:i32,
    tasks:Vec<Task>
}

#[derive(Default,Clone)]
struct Task {
    title:String,
    finished:bool
}


impl ToDoList {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // egui_extras::install_image_loaders(&cc.egui_ctx);
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let tasks = Self::load();
        ToDoList {text:"".to_owned(), selected:-1, tasks}
    }

    fn load() -> Vec<Task> {
        let mut res = Vec::new();
        let file_contents = fs::read_to_string("todo.txt").expect("Couldn't read file todo.txt");
        for line in file_contents.lines() {
            if line.len() > 0 {
                res.push(Task {title:line.to_owned(),finished:false})
            }
        }
        res
    }

    fn save_tasks(&self) {
        let mut res:String = "".to_owned();
        for task in &self.tasks {
            if !task.finished {
                res += &task.title;
                res += "\n";
            }
        }
        fs::write("todo.txt",res).expect("Couldnt save to file todo.txt")
    }

}
impl eframe::App for ToDoList {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            //will cause a save_tasks call at the end of the function
            let mut should_save:bool = false;

            ui.horizontal(|ui| {
                egui::TextEdit::singleline(&mut self.text)
                    .hint_text("I will complete...")
                    .show(ui);
                if ui.button("+").clicked() && self.text.len() > 0 {
                    self.tasks.push(Task {title:self.text.to_owned(),finished:false});

                    self.text = "".to_owned();
                    should_save = true;
                }
            });
            ui.separator();

            //idx which will be removed after the for loop
            let mut remove_idx:i32 = -1;
            
            for (i,task) in self.tasks.iter_mut().enumerate() {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    let i:i32 = i32::try_from(i).unwrap();
                    if self.selected == i {
                        //edit mode
                        if ui.button("x").clicked() {remove_idx = i};
                        if ui.button("<").clicked() {should_save = true;}
                        ui.text_edit_singleline(&mut task.title);
                    } else {
                        //normal display
                        if ui.checkbox(&mut task.finished,"").changed() {should_save = true;}
                        if ui.button("edit").clicked() {
                            self.selected = i
                        }

                        if task.finished {
                            //crossed out text
                            let mut job = egui::epaint::text::LayoutJob::default();
                            job.append(&format!(" {} ", &task.title), 0.0,
                                egui::epaint::text::TextFormat {
                                    color: egui::ecolor::Color32::DARK_GRAY,
                                    strikethrough:egui::Stroke {color: egui::ecolor::Color32::GRAY, width: 2.0},
                                    ..Default::default()
                                }
                            );
                            ui.label(job);
                        } else {
                            //normal text
                            ui.label(&task.title);
                        }
                        
                    }
                });
                ui.separator();
            }
            if remove_idx >= 0 {
                let remove_idx:usize = usize::try_from(remove_idx).unwrap();
                self.tasks.remove(remove_idx);
                should_save = true;
            }
            if should_save {
                self.selected = -1;
                self.save_tasks();
            }
        });
    }
}