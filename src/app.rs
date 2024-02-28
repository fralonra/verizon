mod update;

use std::rc::Rc;

use anyhow::Result;
use self_update::update::Release;
use slint::{SharedString, VecModel};

use crate::app::update::update_to_release;

use self::update::{check_releases, update_to_latest_release};

pub struct App {
    release_list: Rc<Vec<Release>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            release_list: Rc::new(vec![]),
        }
    }

    pub fn init(&mut self) -> Result<()> {
        self.release_list = check_releases()?.into();

        log::info!("Release Count: {}", self.release_list.len());

        Ok(())
    }
}

slint::include_modules!();
pub fn run(app: &App) -> Result<()> {
    let ui = MainWindow::new()?;

    update_release_list(&ui, &app.release_list);

    ui.on_refresh_button_clicked({
        let ui_handle = ui.as_weak();

        let mut release_list = app.release_list.clone();

        move || match check_releases() {
            Ok(releases) => {
                release_list = releases.into();

                update_release_list(&ui_handle.unwrap(), &release_list);
            }
            Err(err) => {
                log::error!("Failed to fetch release list: {}", err);
            }
        }
    });

    ui.on_update_button_clicked({
        let release_list = app.release_list.clone();

        move |index| {
            if let Some(release) = &release_list.iter().nth(index as usize) {
                match update_to_release(release) {
                    Ok(()) => {
                        log::info!("Updated to release: {}", release.name);
                    }
                    Err(err) => {
                        log::error!("Failed to update release: {}", err);
                    }
                }
            }
        }
    });

    ui.on_update_latest_button_clicked({
        || match update_to_latest_release() {
            Ok(()) => {
                log::info!("Updated to latest release.");
            }
            Err(err) => {
                log::error!("Failed to update release: {}", err);
            }
        }
    });

    ui.run()?;

    return Ok(());

    fn update_release_list(ui: &MainWindow, release_list: &[Release]) {
        let release_list = Rc::new(VecModel::<SharedString>::from(
            release_list
                .iter()
                .map(|release| release.name.as_str().into())
                .collect::<Vec<SharedString>>(),
        ));

        ui.set_release_list(release_list.into());
    }
}
