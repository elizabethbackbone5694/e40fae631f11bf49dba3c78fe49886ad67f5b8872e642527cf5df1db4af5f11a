use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// File processor for project edce77
struct Processor_edce77 {
    base_dir: PathBuf,
    project_id: String,
}

impl Processor_edce77 {
    fn new(base_dir: impl AsRef<Path>) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
            project_id: "edce77".to_string(),
        }
    }

    fn process_file(&self, path: impl AsRef<Path>) -> io::Result<usize> {
        let content = fs::read_to_string(&path)?;
        let lines = content.lines().count();
        println!(
            "[{}] processed: {} ({} lines)",
            self.project_id,
            path.as_ref().display(),
            lines
        );
        Ok(lines)
    }

    fn scan_dir(&self) -> io::Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        if self.base_dir.is_dir() {
            for entry in fs::read_dir(&self.base_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    files.push(path);
                }
            }
        }
        Ok(files)
    }

    fn write_report(&self, results: &[(PathBuf, usize)]) -> io::Result<()> {
        let report_path = self.base_dir.join(format!("report_{}.txt", self.project_id));
        let mut f = fs::File::create(&report_path)?;
        writeln!(f, "Report for project: {}", self.project_id)?;
        writeln!(f, "Generated: {:?}", SystemTime::now())?;
        for (path, lines) in results {
            writeln!(f, "  {} -> {} lines", path.display(), lines)?;
        }
        println!("Report written to: {}", report_path.display());
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let dir = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let processor = Processor_edce77::new(&dir);
    let files = processor.scan_dir()?;
    println!("Found {} files in {}", files.len(), dir);
    let mut results = Vec::new();
    for file in &files {
        if let Ok(lines) = processor.process_file(file) {
            results.push((file.clone(), lines));
        }
    }
    processor.write_report(&results)?;
    Ok(())
}
