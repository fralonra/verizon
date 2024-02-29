use anyhow::Result;
use self_update::{
    cargo_crate_version,
    update::{Release, ReleaseUpdate},
};

const REPO_OWNER: &'static str = "fralonra";
const REPO_NAME: &'static str = "verizon";

#[cfg(target_os = "linux")]
const BIN_NAME: &'static str = "verizon-linux";
#[cfg(target_os = "macos")]
const BIN_NAME: &'static str = "verizon-macos";
#[cfg(target_os = "windows")]
const BIN_NAME: &'static str = "verizon.exe";

pub fn check_releases() -> Result<Vec<Release>> {
    Ok(self_update::backends::github::ReleaseList::configure()
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .build()?
        .fetch()?)
}

pub fn update_to_latest_release() -> Result<()> {
    build_update(None)?.update()?;

    Ok(())
}

pub fn update_to_release(release: &Release) -> Result<()> {
    build_update(Some(&release.name))?.update()?;

    Ok(())
}

fn build_update(version_tag: Option<&str>) -> Result<Box<dyn ReleaseUpdate>> {
    let mut builder = self_update::backends::github::Update::configure();

    builder
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .bin_name(BIN_NAME)
        .current_version(cargo_crate_version!())
        .no_confirm(true);

    if let Some(version_tag) = version_tag {
        builder.target_version_tag(version_tag);
    }

    Ok(builder.build()?)
}
