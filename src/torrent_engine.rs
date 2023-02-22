//! TODO: after the web designing, we should change all into the tauri interface support.
//! 
//! transfer `FileDetails` -> `Metainfo`
//! 
//! the data: Vec<u8> could be extracted.
//! 
//! the file_type: used to limit file type which only allow .torrent extensions.
//! 
//! And then should support the `TorrentStats`, `PieceStats` and `ThruputStats`.
use std::collections::HashMap;

use gloo::file::callbacks::FileReader;
use gloo::file::File;
use web_sys::{DragEvent, Event, FileList, HtmlInputElement};
use yew::html::TargetCast;
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct FileDetails {
  name: String,
  file_type: String,
  data: Vec<u8>,
}

pub enum EngineMsg {
  FileLoaded(String, String, Vec<u8>),
  Files(Vec<File>),
}

pub struct TorrentEngine {
  readers: HashMap<String, FileReader>,
  files: Vec<FileDetails>,
  show_active: bool,
}

impl Component for TorrentEngine {
  type Message = EngineMsg;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self {
      readers: HashMap::default(),
      files: Vec::default(),
      show_active: false,
    }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      EngineMsg::FileLoaded(file_name, file_type, data) => {
        self.files.push(FileDetails {
          data,
          file_type,
          name: file_name.clone(),
        });
        self.readers.remove(&file_name);
        if !self.files.is_empty() {
          self.show_active = true;
        }
        true
      }
      EngineMsg::Files(files) => {
        for file in files.into_iter() {
          let file_name = file.name();
          let file_type = file.raw_mime_type();

          let task = {
            let link = ctx.link().clone();
            let file_name = file_name.clone();

            gloo::file::callbacks::read_as_bytes(&file, move |res| {
              link.send_message(EngineMsg::FileLoaded(
                file_name,
                file_type,
                res.expect("failed to read file"),
              ))
            })
          };
          self.readers.insert(file_name, task);
        }
        if !self.files.is_empty() {
          self.show_active = true;
        }
        true
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let Self { show_active, .. } = *self;
    html! {
      <div id="wrapper">
        <input type="file"
          id="title"
          class="file-input file-input-warning w-full max-w-xs"
          ondrop={ctx.link().callback(|event: DragEvent| {
            event.prevent_default();
            let files = event.data_transfer().unwrap().files();
            Self::upload_files(files)
          })}
          ondragover={Callback::from(|event: DragEvent| {
            event.prevent_default();
          })}
          ondragenter={Callback::from(|event: DragEvent| {
            event.prevent_default();
          })}
          accept="image/*,video/*"
          multiple={true}
          onchange={ctx.link().callback(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Self::upload_files(input.files())
          })}
        />
      
          <div class="overflow-x-auto" style="overflow-y: auto;height: 70vh;margin: 15px 0;">
              if show_active {
                  <table class="table w-full">
                      <tbody>
                          { for self.files.iter().map(Self::view_file) }
                      </tbody>
                  </table>
              }
          </div>
      </div>
    }
  }
}

impl TorrentEngine {
  fn view_file(file: &FileDetails) -> Html {
    html! {
        <tr>
            <th>
                <DownloadCard file={file.clone()}/>
            </th>
        </tr>
    }
  }
  fn upload_files(files: Option<FileList>) -> EngineMsg {
    let mut result = Vec::new();

    if let Some(files) = files {
      let files = js_sys::try_iter(&files)
        .unwrap()
        .unwrap()
        .map(|v| web_sys::File::from(v.unwrap()))
        .map(File::from);
      result.extend(files);
    }
    EngineMsg::Files(result)
  }
}

pub struct DownloadCard;

#[derive(PartialEq, Properties)]
pub struct DownloadCardProps {
  pub file: FileDetails,
}

impl Component for DownloadCard {
  type Message = ();
  type Properties = DownloadCardProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let FileDetails {
      name,
      file_type,
      data,
    } = &ctx.props().file;
    html! {
        <div>
            <p style="width: 80vw;overflow-x: auto;">{name.clone()}</p>
            <progress class="progress progress-error w-full" value="50" max="100"></progress>
            <div class="collapse">
                <input type="checkbox" />
                <div class="btn btn-outline collapse-title text-xl font-medium">
                    {"Check Details"}
                </div>
                <div class="collapse-content">
                    <p>{"Details"}</p>
                </div>
            </div>
            <div class="form-control">
                <label class="label cursor-pointer">
                    <span class="label-text">{"Start Download"}</span>
                    <input type="checkbox" class="toggle"/>
                </label>
            </div>
        </div>

    }
  }
}
