use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use anyhow::{bail, Context};

/// Load vectors from an fvecs file.
///
/// Format: per vector, [dim: i32 LE] [v[0]: f32 LE] ... [v[dim-1]: f32 LE]
/// No global header. Dimension is repeated per vector.
///
/// Returns (dimension, flat_vectors) where flat_vectors has n*dim elements.
pub fn load_fvecs(path: &Path) -> anyhow::Result<(usize, Vec<f32>)> {
    let file = File::open(path).with_context(|| format!("opening {}", path.display()))?;
    let file_len = file.metadata()?.len() as usize;
    let mut reader = BufReader::new(file);

    // Read first dimension
    let mut dim_buf = [0u8; 4];
    reader.read_exact(&mut dim_buf)?;
    let dim = i32::from_le_bytes(dim_buf) as usize;

    if dim == 0 {
        bail!("fvecs dimension is 0");
    }

    let vec_byte_size = 4 + dim * 4; // 4 bytes dim + dim*4 bytes data
    if !file_len.is_multiple_of(vec_byte_size) {
        bail!(
            "file size {} not divisible by vector byte size {} (dim={})",
            file_len,
            vec_byte_size,
            dim
        );
    }
    let n = file_len / vec_byte_size;

    let mut flat = Vec::with_capacity(n * dim);

    // We already read the first dim; read first vector's data
    let mut float_buf = vec![0u8; dim * 4];
    reader.read_exact(&mut float_buf)?;
    for chunk in float_buf.chunks_exact(4) {
        flat.push(f32::from_le_bytes(chunk.try_into().unwrap()));
    }

    // Read remaining vectors
    for i in 1..n {
        reader
            .read_exact(&mut dim_buf)
            .with_context(|| format!("reading dim for vector {i}"))?;
        let d = i32::from_le_bytes(dim_buf) as usize;
        if d != dim {
            bail!("dimension mismatch at vector {i}: expected {dim}, got {d}");
        }
        reader.read_exact(&mut float_buf)?;
        for chunk in float_buf.chunks_exact(4) {
            flat.push(f32::from_le_bytes(chunk.try_into().unwrap()));
        }
    }

    Ok((dim, flat))
}

/// Load integer vectors from an ivecs file.
///
/// Same format as fvecs but with i32 values instead of f32.
/// Returns (dimension, flat_indices) where flat_indices has n*dim elements.
pub fn load_ivecs(path: &Path) -> anyhow::Result<(usize, Vec<i32>)> {
    let file = File::open(path).with_context(|| format!("opening {}", path.display()))?;
    let file_len = file.metadata()?.len() as usize;
    let mut reader = BufReader::new(file);

    let mut dim_buf = [0u8; 4];
    reader.read_exact(&mut dim_buf)?;
    let dim = i32::from_le_bytes(dim_buf) as usize;

    if dim == 0 {
        bail!("ivecs dimension is 0");
    }

    let vec_byte_size = 4 + dim * 4;
    if !file_len.is_multiple_of(vec_byte_size) {
        bail!(
            "file size {} not divisible by vector byte size {} (dim={})",
            file_len,
            vec_byte_size,
            dim
        );
    }
    let n = file_len / vec_byte_size;

    let mut flat = Vec::with_capacity(n * dim);
    let mut int_buf = vec![0u8; dim * 4];

    // First vector data (dim already read)
    reader.read_exact(&mut int_buf)?;
    for chunk in int_buf.chunks_exact(4) {
        flat.push(i32::from_le_bytes(chunk.try_into().unwrap()));
    }

    for i in 1..n {
        reader
            .read_exact(&mut dim_buf)
            .with_context(|| format!("reading dim for vector {i}"))?;
        let d = i32::from_le_bytes(dim_buf) as usize;
        if d != dim {
            bail!("dimension mismatch at vector {i}: expected {dim}, got {d}");
        }
        reader.read_exact(&mut int_buf)?;
        for chunk in int_buf.chunks_exact(4) {
            flat.push(i32::from_le_bytes(chunk.try_into().unwrap()));
        }
    }

    Ok((dim, flat))
}

/// Save vectors to an fvecs file.
pub fn save_fvecs(path: &Path, dim: usize, flat: &[f32]) -> anyhow::Result<()> {
    assert_eq!(flat.len() % dim, 0);
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    let dim_bytes = (dim as i32).to_le_bytes();

    for vec in flat.chunks_exact(dim) {
        writer.write_all(&dim_bytes)?;
        for &v in vec {
            writer.write_all(&v.to_le_bytes())?;
        }
    }
    Ok(())
}

/// Save integer vectors to an ivecs file.
pub fn save_ivecs(path: &Path, dim: usize, flat: &[i32]) -> anyhow::Result<()> {
    assert_eq!(flat.len() % dim, 0);
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    let dim_bytes = (dim as i32).to_le_bytes();

    for vec in flat.chunks_exact(dim) {
        writer.write_all(&dim_bytes)?;
        for &v in vec {
            writer.write_all(&v.to_le_bytes())?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn write_fvecs_data(dim: usize, vectors: &[Vec<f32>]) -> NamedTempFile {
        let mut tmp = NamedTempFile::new().unwrap();
        let dim_bytes = (dim as i32).to_le_bytes();
        for vec in vectors {
            tmp.write_all(&dim_bytes).unwrap();
            for &v in vec {
                tmp.write_all(&v.to_le_bytes()).unwrap();
            }
        }
        tmp.flush().unwrap();
        tmp
    }

    #[test]
    fn test_fvecs_round_trip() {
        let dim = 3;
        let vecs = vec![vec![1.0f32, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let tmp = write_fvecs_data(dim, &vecs);

        let (d, flat) = load_fvecs(tmp.path()).unwrap();
        assert_eq!(d, 3);
        assert_eq!(flat.len(), 6);
        assert_eq!(&flat[..3], &[1.0, 2.0, 3.0]);
        assert_eq!(&flat[3..], &[4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_fvecs_save_load() {
        let dim = 2;
        let flat = vec![1.0f32, 2.0, 3.0, 4.0];
        let tmp = NamedTempFile::new().unwrap();

        save_fvecs(tmp.path(), dim, &flat).unwrap();
        let (d, loaded) = load_fvecs(tmp.path()).unwrap();
        assert_eq!(d, dim);
        assert_eq!(loaded, flat);
    }

    #[test]
    fn test_ivecs_save_load() {
        let dim = 2;
        let flat = vec![10i32, 20, 30, 40];
        let tmp = NamedTempFile::new().unwrap();

        save_ivecs(tmp.path(), dim, &flat).unwrap();
        let (d, loaded) = load_ivecs(tmp.path()).unwrap();
        assert_eq!(d, dim);
        assert_eq!(loaded, flat);
    }
}
