use flate2::{bufread::GzEncoder, Compression};
use ignore::Walk;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use tar::Builder;
use tempfile::tempfile;

pub fn get_artifact_bytes(path: &str) -> Vec<u8> {
  // note(itay): The ignore crate uses the .gitignore file and also a .ignore file
  // if specified by default, so basically like walkdir but already baked with the
  // logic we had in mind.
  let artifact_files = ignore::Walk::new(path);
  let tar = create_tar(artifact_files, Path::new(path).to_path_buf());
  get_compressed_tar(tar)
}

pub fn save_artifact_to_path(path: &str, output_path: &str) {
  let compressed_tar = get_artifact_bytes(path);
  save_buffer_to_file(compressed_tar, output_path);
}

fn create_tar(files_to_tar: Walk, tar_path_root: PathBuf) -> File {
  let mut tar_file = tempfile().unwrap();
  {
    let mut tar_builder = Builder::new(&tar_file);
    for file in files_to_tar {
      let entry = file.unwrap();
      let path = entry.path();
      let stripped_path = path.strip_prefix(tar_path_root.as_path()).unwrap();
      if stripped_path.to_str().unwrap().len() == 0 {
        tar_builder.append_path_with_name(path, "./").unwrap()
      } else {
        tar_builder
          .append_path_with_name(path, stripped_path)
          .unwrap()
      }
    }
    tar_builder.into_inner().unwrap();
  }
  &mut tar_file.seek(SeekFrom::Start(0)).unwrap();
  return tar_file;
}

fn get_compressed_tar(tar: File) -> Vec<u8> {
  let buf_reader = BufReader::new(tar);
  let mut encoder = GzEncoder::new(buf_reader, Compression::default());
  let mut buffer = Vec::new();
  encoder.read_to_end(&mut buffer).unwrap();
  return buffer;
}

fn save_buffer_to_file(buffer: Vec<u8>, path: &str) {
  let mut output_file = File::create(path).unwrap();
  output_file.write_all(&buffer).unwrap();
}

#[test]
fn should_save_artifact_to_path() {
  let output_path = "/tmp/test_artifact.tar.gz";
  save_artifact_to_path("tests/fixtures/test_artifact", output_path);
  assert!(std::path::Path::new(output_path).exists());
  let file = File::open(output_path).unwrap();
  assert_ne!(file.metadata().unwrap().len(), 0);
  std::fs::remove_file(output_path).unwrap();
}
