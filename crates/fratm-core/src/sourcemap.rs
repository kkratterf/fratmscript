//! Source Map generation for FratmScript
//!
//! Implements Source Map v3 specification for debugging support.

use serde::{Serialize, Deserialize};

/// A source map following the v3 specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMap {
    /// Version (always 3)
    pub version: u8,
    /// Generated file name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// Source file name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_root: Option<String>,
    /// List of source files
    pub sources: Vec<String>,
    /// Source contents (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources_content: Option<Vec<String>>,
    /// Symbol names
    pub names: Vec<String>,
    /// VLQ encoded mappings
    pub mappings: String,
}

impl Default for SourceMap {
    fn default() -> Self {
        Self {
            version: 3,
            file: None,
            source_root: None,
            sources: vec!["input.fratm".to_string()],
            sources_content: None,
            names: vec![],
            mappings: String::new(),
        }
    }
}

impl SourceMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_source(mut self, source: &str) -> Self {
        self.sources = vec![source.to_string()];
        self
    }

    pub fn with_content(mut self, content: &str) -> Self {
        self.sources_content = Some(vec![content.to_string()]);
        self
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    /// Convert to JSON string (pretty)
    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    /// Get as data URL for inline source maps
    pub fn to_data_url(&self) -> String {
        let json = self.to_json();
        let encoded = base64_encode(&json);
        format!("//# sourceMappingURL=data:application/json;base64,{}", encoded)
    }
}

/// Source map builder for incremental construction
#[derive(Debug, Default, Clone)]
pub struct SourceMapBuilder {
    /// Mapping segments
    segments: Vec<Vec<Segment>>,
    /// Current generated line
    gen_line: usize,
    /// Current generated column
    gen_col: usize,
    /// Previous generated column (for delta encoding)
    prev_gen_col: usize,
    /// Previous source line
    prev_src_line: usize,
    /// Previous source column
    prev_src_col: usize,
    /// Symbol names
    names: Vec<String>,
}

/// A single mapping segment
#[derive(Debug, Clone)]
struct Segment {
    gen_col: usize,
    src_line: usize,
    src_col: usize,
    name_idx: Option<usize>,
}

impl SourceMapBuilder {
    pub fn new() -> Self {
        Self {
            segments: vec![vec![]],
            ..Default::default()
        }
    }

    /// Add a mapping from generated position to source position
    pub fn add_mapping(&mut self, gen_line: usize, gen_col: usize, src_line: usize, src_col: usize) {
        // Ensure we have enough lines
        while self.segments.len() <= gen_line {
            self.segments.push(vec![]);
        }

        self.segments[gen_line].push(Segment {
            gen_col,
            src_line,
            src_col,
            name_idx: None,
        });
    }

    /// Add a named mapping
    pub fn add_named_mapping(
        &mut self,
        gen_line: usize,
        gen_col: usize,
        src_line: usize,
        src_col: usize,
        name: &str,
    ) {
        let name_idx = self.names.iter().position(|n| n == name).unwrap_or_else(|| {
            self.names.push(name.to_string());
            self.names.len() - 1
        });

        while self.segments.len() <= gen_line {
            self.segments.push(vec![]);
        }

        self.segments[gen_line].push(Segment {
            gen_col,
            src_line,
            src_col,
            name_idx: Some(name_idx),
        });
    }

    /// Notify that we moved to a new generated line
    pub fn new_line(&mut self) {
        self.gen_line += 1;
        self.gen_col = 0;
        self.prev_gen_col = 0;
    }

    /// Build the final source map
    pub fn build(mut self, source_file: Option<&str>) -> SourceMap {
        let mappings = self.encode_mappings();

        SourceMap {
            version: 3,
            file: None,
            source_root: None,
            sources: vec![source_file.unwrap_or("input.fratm").to_string()],
            sources_content: None,
            names: self.names,
            mappings,
        }
    }

    /// Encode all mappings to VLQ string
    fn encode_mappings(&mut self) -> String {
        let mut result = String::new();
        let mut prev_gen_col: i64 = 0;
        let mut prev_src_line: i64 = 0;
        let mut prev_src_col: i64 = 0;
        let mut prev_name: i64 = 0;

        for (line_idx, line_segments) in self.segments.iter().enumerate() {
            if line_idx > 0 {
                result.push(';');
            }

            prev_gen_col = 0; // Reset column for each line

            for (seg_idx, segment) in line_segments.iter().enumerate() {
                if seg_idx > 0 {
                    result.push(',');
                }

                // Generated column (delta)
                let gen_col_delta = segment.gen_col as i64 - prev_gen_col;
                result.push_str(&vlq_encode(gen_col_delta));
                prev_gen_col = segment.gen_col as i64;

                // Source index (always 0 for single source)
                result.push_str(&vlq_encode(0));

                // Source line (delta)
                let src_line_delta = segment.src_line as i64 - prev_src_line;
                result.push_str(&vlq_encode(src_line_delta));
                prev_src_line = segment.src_line as i64;

                // Source column (delta)
                let src_col_delta = segment.src_col as i64 - prev_src_col;
                result.push_str(&vlq_encode(src_col_delta));
                prev_src_col = segment.src_col as i64;

                // Name index (delta, if present)
                if let Some(name_idx) = segment.name_idx {
                    let name_delta = name_idx as i64 - prev_name;
                    result.push_str(&vlq_encode(name_delta));
                    prev_name = name_idx as i64;
                }
            }
        }

        result
    }
}

// ============== VLQ Encoding ==============

const VLQ_BASE_SHIFT: u8 = 5;
const VLQ_BASE: i64 = 1 << VLQ_BASE_SHIFT;
const VLQ_BASE_MASK: i64 = VLQ_BASE - 1;
const VLQ_CONTINUATION_BIT: i64 = VLQ_BASE;

const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Encode a number as VLQ
fn vlq_encode(value: i64) -> String {
    let mut encoded = String::new();
    let mut vlq = if value < 0 {
        ((-value) << 1) + 1
    } else {
        value << 1
    };

    loop {
        let mut digit = vlq & VLQ_BASE_MASK;
        vlq >>= VLQ_BASE_SHIFT;

        if vlq > 0 {
            digit |= VLQ_CONTINUATION_BIT;
        }

        encoded.push(BASE64_CHARS[digit as usize] as char);

        if vlq == 0 {
            break;
        }
    }

    encoded
}

/// Simple base64 encoding
fn base64_encode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut result = String::new();

    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as usize;
        let b1 = chunk.get(1).copied().unwrap_or(0) as usize;
        let b2 = chunk.get(2).copied().unwrap_or(0) as usize;

        result.push(BASE64_CHARS[b0 >> 2] as char);
        result.push(BASE64_CHARS[((b0 & 0x03) << 4) | (b1 >> 4)] as char);

        if chunk.len() > 1 {
            result.push(BASE64_CHARS[((b1 & 0x0f) << 2) | (b2 >> 6)] as char);
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(BASE64_CHARS[b2 & 0x3f] as char);
        } else {
            result.push('=');
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vlq_encode() {
        assert_eq!(vlq_encode(0), "A");
        assert_eq!(vlq_encode(1), "C");
        assert_eq!(vlq_encode(-1), "D");
        assert_eq!(vlq_encode(16), "gB");
    }

    #[test]
    fn test_source_map_builder() {
        let mut builder = SourceMapBuilder::new();
        builder.add_mapping(0, 0, 0, 0);
        builder.add_mapping(0, 6, 0, 8);
        builder.new_line();
        builder.add_mapping(1, 0, 1, 0);

        let map = builder.build(Some("test.fratm"));
        assert_eq!(map.version, 3);
        assert!(!map.mappings.is_empty());
    }

    #[test]
    fn test_source_map_json() {
        let map = SourceMap::default();
        let json = map.to_json();
        assert!(json.contains("\"version\":3"));
    }
}
