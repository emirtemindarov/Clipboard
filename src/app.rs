use crate::*;

impl App for Clipboard {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        // println!("{}", self.clip_ctx.clip.get_contents().unwrap_or_else(|err| err.to_string()));
        get_text_every_loop(self, ctx);
        // println!("{}", self.clip_ctx.clip.get_contents().unwrap_or_else(|err| err.to_string()));
        CentralPanel::default().show(ctx, |ui| {
            TopBottomPanel::top("main").show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(10.0);
                    render_header(self, ctx, ui);
                    ui.add_space(20.0);
                    render_scroll(self, ui, ctx);
                });
            });
        });
    }
    fn save(&mut self, _storage: &mut dyn Storage) {

    }
}

fn render_header(clipboard: &mut Clipboard, ctx: &Context, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.add_space(10.0);
        ui.heading(RichText::new("Буфер обмена").color(Color32::from_rgb(168, 168, 172)).size(16.0));
        ui.add_space(90.0);

        // let rect = Rect::from_min_max(Pos2::new(200.0, 30.0), Pos2::new(240.0, 60.0));

        let rich_text = RichText::new("Очистить все").color(Color32::from_rgb(0, 0, 0));
        let delete_unpinned = ui.add(Button::new(rich_text).fill(Color32::from_rgb(244, 244, 244)));
        if delete_unpinned.hovered() && delete_unpinned.clicked() {
            clipboard.delete_unpinned();
        }
    });
}

fn render_scroll(clipboard: &mut Clipboard, ui: &mut Ui, ctx: &Context) {
    // пришли с ui.vertical()
    ui.horizontal(|ui| {
        ScrollArea::vertical().min_scrolled_height(350.0).show(ui, |ui| {
            ui.with_layout(Layout::top_down(Align::Min), |ui| {
                render_blocks(clipboard, ui, ctx);
            });
        });
    });
}

fn render_blocks(clipboard: &mut Clipboard, ui: &mut Ui, ctx: &Context) {
    // пришли с Layout::top_down()
    let h_c_blocks = clipboard.blocks.clone();
    println!("----");
    for (number, _) in h_c_blocks.iter().rev() {     // узнаем ключи блоков
        println!("{:?}", number);
        // match clipboard.blocks.get_mut(&(number as &u32)) {
        //     None => {println!("fries"); continue},
        //     block => {
        if appropriated(&clipboard.blocks.get(&number).unwrap().text) {
            let block_panel = TopBottomPanel::top(format!("number{}", &number)).show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.allocate_ui_with_layout(Vec2::default().clamp(Vec2::new(600.0, 200.0), Vec2::new(600.0, 200.0)), Layout::left_to_right(), |ui| {
                        ui.vertical(|ui| {
                            ui.add_space(5.0);
                            render_block_top_panel(&clipboard, ui, &number);
                            ui.add_space(5.0);
                            ui.allocate_ui_with_layout(Vec2::default().clamp(Vec2::new(40.0, 30.0), Vec2::new(40.0, 30.0)), Layout::left_to_right(), |ui| {
                                let non_preview = StringWrapper::new(format!("{}", clipboard.blocks.get_mut(&(number as &u32)).unwrap().text));             // не знаю где ошибки CAN YOU FIXME?  ggg unwrap
                                let preview = make_preview(&non_preview);
                                ui.monospace(&preview.src[..]);      // ну и где monospace?  pppppppppjjjjjjjjjjjjjjjjj
                            });
                            ui.label(format!("{}", get_time()));
                        });
                        ui.add_space(30.0);
                    });
                    render_block_sidebar(clipboard, ui, ctx, &(number as &u32));
                });
            });
            println!("{:?}", clipboard.blocks);
            let mut block = get_block(clipboard, number);
            match block {
                None => println!("get_block() вернул None"),
                Some(block) => {
                    if block_panel.response.hovered() {
                        block.hovered = true;
                        //ui.button(format!("{:?}", clipboard.blocks));  // TODO remove
                    } else {
                        block.hovered = false;
                    }
                }
            }



            // TODO    if block_panel.clicked() {
            /*if block_panel.response.clicked() {
                println!("click!!");
                clipboard.clip_ctx.clip.set_contents(clipboard.blocks.get_mut(&number).unwrap().text.to_owned()).unwrap_or_default();
            }*/

            // println!("{}", clipboard.blocks.get_mut(number).unwrap().clicked);
            // TODO    if clipboard.block.clicked() {
            /*if clipboard.blocks.get_mut(number).unwrap().clicked {
                clipboard.blocks.get_mut(number).unwrap().clicked = false;
                clipboard.clip_ctx.clip.set_contents(
                    format!("{}", clipboard.blocks.get_mut(&number).unwrap().text)
                ).unwrap_or_default();
                println!("ttttttt");
            }*/

        }
    };
    ui.add_space(15.0);
}

fn get_block<'a>(clipboard: &'a mut Clipboard, number: &'a u32) -> Option<&'a mut Block> {
    clipboard.blocks.get_mut(number)
}

// какая-то магия
/*
fn get_keys<'a, T>(keys_vector: T) -> Cow<'a, [T]>
    where T: Into<Cow<'a, [T]>>,
          [T]: ToOwned
{
    keys_vector.into()
}
*/

fn appropriated(text: &StringWrapper) -> bool {
    // TODO match
    if text != &StringWrapper::new("".to_string()) &&
        text != &StringWrapper::new("The operation completed successfully. (os error 0)".to_string()) &&
        text != &StringWrapper::new("The handle is invalid (os error 6)".to_string()) &&
        text != &StringWrapper::new("Access is denied. (os error 5)".to_string()) {
        true
    } else {
        false
    }
}

/*
impl<'a> PartialEq<str> for Cower<'a> {
    fn eq(&self, other: &Cower) -> bool {
        self == other
    }
}
*/

/*
impl<'a, 'b, 'c> PartialEq<String> for Cower<'c> {
    fn eq(&self, other: &String) -> bool {
        self == other
    }
}
*/

/*
impl PartialEq for StringWrapper {
    fn eq(&self, other: &Self) -> bool {
        if self.0 == other.0 {
            false
        } else {
            true
        }
    }
}
*/

// опять магия, но чуть понятнее
/*
impl<T: PartialEq, U: PartialEq> PartialEq for Either<T, U> {
    fn eq(&self, other: &Self) -> bool {
        use Either::*;
        match (self, other) {
            (Left(a), Left(b)) => a == b,
            (Right(a), Right(b)) => a == b,
            _ => false,
        }
    }
}
*/

fn get_text_every_loop(clipboard: &mut Clipboard, ctx: &Context) {
    // сохранить выделенный текст в буфер с помощью self.clip_ctx.set_contents().unwrap_or_else(|err| err.to_string());
    let text = StringWrapper::new(clipboard.clip_ctx.clip.get_contents().unwrap_or_else(|err| err.to_string()));
    // TODO разобраться .clone()
    if text.src != clipboard.clip_ctx.previous.src {
        clipboard.clip_ctx.previous = text.clone();
        clipboard.clip_ctx.clip.set_contents(text.src.clone().to_owned()).unwrap_or_default();
        let block1 = Block::new(text.clone());   // достать этот же текст из буфера и отправить в конструктор нового блока
        clipboard.add(block1);   // добавить созданный блок
    }
}

fn render_block_top_panel (clipboard: &Clipboard, ui: &mut Ui, number: &u32) {
    if clipboard.blocks.get(number).unwrap().hovered {
        ui.horizontal(|ui| {
            let eye_label = ui.label("👁");
            eye_label.on_hover_ui(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        format!("{}", show_block_text(&clipboard.blocks.get(number).unwrap().text))
                    );
                });
            });
        });
    } else {
        ui.add_space(20.0);
    }
}

fn make_preview(non_preview: &StringWrapper) -> StringWrapper {
    let mut preview = StringWrapper::default();
    let mut cx = non_preview.src.len() / 35;
    if cx == 0 {
        preview = StringWrapper::new(format!("{}", &non_preview.src[..]));
    }
    if cx > 3 { cx = 3 };
    for char in 1..=cx {
        preview = StringWrapper::new(format!("{}{}\n", preview, &non_preview.src[(char - 1) * 35..char * 35]));
    }
    StringWrapper::new(format!("{}", preview))
}

fn show_block_text(non_preview: &StringWrapper) -> StringWrapper {
    let mut preview = StringWrapper::default();
    let mut cx = non_preview.src.len() / 35;
    for char in 1..=cx {
        preview = StringWrapper::new(format!("{}{}\n", preview, &non_preview.src[(char - 1) * 35..char * 35]));
    }
    println!("9999999999{}{}", preview, &non_preview.src[cx*35..]);
    StringWrapper::new(format!("{}{}", preview, &non_preview.src[cx*35..]))
}

fn render_block_sidebar(clipboard: &mut Clipboard, ui: &mut Ui, ctx: &Context, number: &u32) {
    // пришли с ui.horizontal()
    ui.vertical(|ui| {
        ui.with_layout(Layout::top_down(Align::Max), |ui| {
            ui.add_space(10.0);
            let h_c_block = clipboard.blocks.get(number).unwrap().clone();
            if h_c_block.hovered {
                let delete_button = ui.small_button(
                    "✖"
                ).on_hover_text("Удалить (Delete)\n\nОкончательно удалить элемент\nиз журнала буфера обмена.");
                if delete_button.clicked() {
                    clipboard.delete(number);
                    ctx.request_repaint();     // надо?
                }
                let pin_button = ui.small_button(
                    match h_c_block.pinned {
                        true => "📍",
                        false => "📌",
                    }).on_hover_text(
                    match h_c_block.pinned {
                        true => "Открепить (U)\n\nРазрешить удалять элемент при\nочистке журнала буфераобмена\nили перезагрузке компьютера.",
                        false => "Закрепить (P)\n\nСохранить элемент даже при\nочистке журнала буфера обмена\nили перезагрузке компьютера.",
                    });
                if pin_button.clicked() {
                    clipboard.pin(number);
                    ctx.request_repaint();    // надо?
                }
                ui.add_space(30.0);
                let copy_to_clipboard_button = ui.button("📋").on_hover_text("Нажмите, чтобы скопировать этот текст");
                if copy_to_clipboard_button.clicked() {
                    clipboard.stage(number);
                    ctx.request_repaint();    // надо?
                }
            }
        });
    });
}

fn render_preview_text_block_and_time(ui: &mut Ui) {

}

fn get_time() -> String {
    let utc: DateTime<Utc> = Utc::now();       // e.g. `2014-11-28T12:45:59.324310806Z`
    // let local: DateTime<Local> = Local::now(); // e.g. `2014-11-28T21:45:59.324310806+09:00`

    let hour = utc.time().hour();
    let mut minute: String = utc.time().minute().to_string();
    if minute.parse::<u32>().unwrap() < 10 {
        minute = format!("0{}", minute);
    }
    let time = format!("{}:{}", hour, minute);
    time
}