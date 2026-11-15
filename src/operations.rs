use anyhow::{Context, Result, bail};
use flate2::{write::GzEncoder, Compression};
use std::{fs::{self, File}, io::{Read, Write}, path::{Path, PathBuf}};
use walkdir::WalkDir;

fn collect(inputs: &[PathBuf]) -> Result<Vec<(PathBuf, PathBuf)>> {
    let mut entries = Vec::new();
    for input in inputs {
        if input.is_file() {
            let name = input.file_name().context("Input has no file name")?;
            entries.push((input.clone(), PathBuf::from(name)));
        } else if input.is_dir() {
            let base = input.parent().unwrap_or(input);
            for item in WalkDir::new(input).follow_links(false) {
                let item = item?;
                if item.file_type().is_file() {
                    let rel = item.path().strip_prefix(base)?;
                    entries.push((item.path().to_path_buf(), rel.to_path_buf()));
                }
            }
        } else {
            bail!("Input does not exist: {}", input.display());
        }
    }
    Ok(entries)
}

pub fn compress(inputs: &[PathBuf], output: &Path, format: usize, level: usize) -> Result<PathBuf> {
    fs::create_dir_all(output)?;
    let entries = collect(inputs)?;
    if entries.is_empty() { bail!("No files found in selected inputs"); }
    let stem = inputs.first().and_then(|p| p.file_stem()).unwrap_or_default().to_string_lossy();
    let ext = match format { 1 => "tar.gz", 2 => "tar.zst", _ => "zip" };
    let target = output.join(format!("{}.{}", if stem.is_empty() { "Archive" } else { &stem }, ext));
    let file = File::create(&target).with_context(|| format!("Cannot create {}", target.display()))?;
    match format {
        1 => {
            let enc = GzEncoder::new(file, Compression::new([1, 6, 9][level]));
            let mut builder = tar::Builder::new(enc);
            for (src, name) in entries { builder.append_path_with_name(src, name)?; }
            builder.finish()?;
        }
        2 => {
            let enc = zstd::Encoder::new(file, [1, 8, 19][level])?;
            let mut builder = tar::Builder::new(enc);
            for (src, name) in entries { builder.append_path_with_name(src, name)?; }
            builder.finish()?;
        }
        _ => {
            let mut zip = zip::ZipWriter::new(file);
            let options = zip::write::FileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated)
                .compression_level(Some([1, 6, 9][level] as i32));
            for (src, name) in entries {
                zip.start_file(name.to_string_lossy().replace('\\', "/"), options)?;
                let mut f = File::open(src)?;
                std::io::copy(&mut f, &mut zip)?;
            }
            zip.finish()?;
        }
    }
    Ok(target)
}

pub fn extract(archive: &Path, destination: &Path) -> Result<()> {
    fs::create_dir_all(destination)?;
    let name = archive.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
    if name.ends_with(".zip") {
        let f = File::open(archive)?;
        let mut zip = zip::ZipArchive::new(f)?;
        for i in 0..zip.len() {
            let mut item = zip.by_index(i)?;
            let Some(rel) = item.enclosed_name().map(Path::to_path_buf) else { continue };
            let out = destination.join(rel);
            if item.is_dir() { fs::create_dir_all(&out)?; }
            else {
                if let Some(parent) = out.parent() { fs::create_dir_all(parent)?; }
                let mut f = File::create(out)?;
                std::io::copy(&mut item, &mut f)?;
            }
        }
    } else if name.ends_with(".tar.gz") || name.ends_with(".tgz") {
        let f = File::open(archive)?;
        let dec = flate2::read::GzDecoder::new(f);
        safe_unpack(tar::Archive::new(dec), destination)?;
    } else if name.ends_with(".tar.zst") || name.ends_with(".tzst") {
        let f = File::open(archive)?;
        let dec = zstd::Decoder::new(f)?;
        safe_unpack(tar::Archive::new(dec), destination)?;
    } else {
        bail!("Unsupported archive. Use ZIP, TAR.GZ, or TAR.ZST.");
    }
    Ok(())
}

fn safe_unpack<R: Read>(mut archive: tar::Archive<R>, destination: &Path) -> Result<()> {
    // Reject absolute paths and parent traversal before extraction.
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.into_owned();
        if path.is_absolute() || path.components().any(|c| matches!(c, std::path::Component::ParentDir)) {
            bail!("Unsafe path in archive: {}", path.display());
        }
        entry.unpack_in(destination)?;
    }
    Ok(())
}
