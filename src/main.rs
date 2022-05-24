mod app;

use std::error::Error;
use std::collections::btree_map::BTreeMap;
use std::fmt::{Debug, Display, Formatter};

// safe_unwrap
/*#[macro_use]
extern crate safe_unwrap;
use safe_unwrap::*;*/

// image
/*extern crate image;
use image::io::Reader as ImageReader;
use image::error;*/

// keybind
/*extern crate keybind;
use keybind::{Keybind, Keycode};*/

extern crate chrono;
use chrono::{Timelike, DateTime, FixedOffset, Local, Utc};

extern crate clipboard;
use clipboard::{ClipboardProvider, ClipboardContext};
use clipboard::windows_clipboard::WindowsClipboardContext;

use eframe::{App, Storage, CreationContext, Frame, IconData, NativeOptions};
use eframe::emath::Vec2;
use eframe::egui::{Ui, Context, Modifiers, FontData, FontFamily, FontDefinitions, TextStyle, Direction, Layout,
                   CentralPanel, TopBottomPanel, Window, Area, ScrollArea, SidePanel,
                   Button, PointerButton, FontSelection::Style,
                   Align, Shape, Rect, WidgetInfo, Color32, Pos2, RichText, Event, Event::Key, Key::H,
                   NumExt, show_tooltip, show_tooltip_at, show_tooltip_text, show_tooltip_at_pointer, show_tooltip_for, };

//----------------------------------------------------------------------------------------------------------------------------------------------------------------------

#[derive(Clone)]
struct Clipboard {
    blocks: BTreeMap<u32, Block>,
    clip_ctx: Clip,
}

impl Clipboard {
    fn new(cc: &eframe::CreationContext<'_>, clip_ctx: Clip) -> Self {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "Calibri".to_owned(),
            FontData::from_static(include_bytes!("../fonts/Calibri/Calibri.ttf"))
        );
        fonts.families.get_mut(&FontFamily::Monospace).unwrap()
            .insert(0, "Calibri".to_owned());
        cc.egui_ctx.set_fonts(fonts);
        Self {
            blocks: BTreeMap::default(),
            clip_ctx,
        }
    }
}

impl Clipboard {
    fn add(&mut self, block: Block) {
        let mut counter: u32 = 0;
        loop {
            counter += 1;
            match self.blocks.get(&counter) {
                None => break,
                Some(_) => continue,
            };
        }
        self.blocks.insert(counter, block);
    }
    // не создает новый блок, а подготавливает к вставке текст из выбранного блока
    fn stage(&mut self, id: &u32) {
        self.clip_ctx.clip.set_contents(self.blocks.get_mut(id).unwrap().text.src.to_owned()).unwrap_or_default();
    }
    fn pin(&mut self, id: &u32) {
        if let Some(needed) = self.blocks.get_mut(&id) {
            needed.pinned = !needed.pinned;
        }
    }
    fn delete(&mut self, id: &u32) {
        println!("before---{:?} {:?}", self.blocks.len(), self.blocks);
        println!("take {}", id);
        self.blocks.retain(|k, _| k != id);
        println!("after---{:?} {:?}", self.blocks.len(), self.blocks);
        // self.blocks.retain(|k, _| k != id);
    }
    fn delete_unpinned(&mut self) {
        self.blocks.retain(|_, v| v.pinned)
    }
}

//----------------------------------------------------------------------------------------------------------------------------------------------------------------------

#[derive(Clone)]
struct Block {
    text: StringWrapper,
    pinned: bool,
    hovered: bool,
    clicked: bool,
}

impl Block {
    fn new(text: StringWrapper) -> Self {
        Self {
            text,
            pinned: false,
            hovered: false,
            clicked: false,
        }
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text.src)
    }
}

//----------------------------------------------------------------------------------------------------------------------------------------------------------------------

struct Clip {
    clip: ClipboardContext,
    previous: StringWrapper,
}

impl Clone for Clip {
    fn clone(&self) -> Self {
        Self {
            clip: WindowsClipboardContext,
            previous: Default::default()
        }
    }
}

impl Default for Clip {
    fn default() -> Self {
        Self {
            clip: ClipboardProvider::new().unwrap(),
            previous: StringWrapper {
                src: "It's Your Clipboard, here will be stored copied data".to_string()
            },
        }
    }
}

//----------------------------------------------------------------------------------------------------------------------------------------------------------------------

#[derive(PartialEq, Clone)]
struct StringWrapper {
    src: String
}

impl StringWrapper {
    fn new(text: String) -> Self {
        Self {
            src: text
        }
    }
}

impl Default for StringWrapper {
    fn default() -> Self {
        Self {
            src: "".to_string()
        }
    }
}

impl Display for StringWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.src)
    }
}

//----------------------------------------------------------------------------------------------------------------------------------------------------------------------

fn main() {
    let mut clip_ctx = Clip::default();
    //clip_ctx.set_contents("something new".to_owned()).unwrap();
    //println!("{:?}", clip_ctx.get_contents());

    let mut native_options = NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(300.0, 400.0));
    native_options.decorated = false;
    // native_options.icon_data = Some(load_icon("../icon/hamburg-removebg-preview.png").unwrap());
    eframe::run_native("Clipboard", native_options, Box::new(|cc| Box::new(Clipboard::new(cc, clip_ctx))));
}

// icon_data
/*
fn load_icon(path: &str) -> Result<IconData, Box<dyn Error>> {
    let (icon_rgba, icon_width, icon_height) = {
        let image = ImageReader::open(path)?.decode()?
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    Ok(IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    })
}
*/