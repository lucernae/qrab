use console::Term;

/// Represents a grid layout of QR codes
pub struct QrGrid {
    qr_codes: Vec<String>,
    terminal_width: usize,
}

impl QrGrid {
    /// Create a new QR grid with the given QR code strings
    pub fn new(qr_codes: Vec<String>) -> Self {
        let term = Term::stdout();
        let terminal_width = term.size().1 as usize;

        Self {
            qr_codes,
            terminal_width,
        }
    }

    /// Render the QR codes in a grid layout
    /// QR codes are arranged horizontally until terminal width is reached,
    /// then wrapped to the next row
    pub fn render(&self) -> String {
        if self.qr_codes.is_empty() {
            return String::new();
        }

        // Calculate QR code width from the first QR code
        let qr_width = self.qr_codes[0]
            .lines()
            .next()
            .map(|line| line.chars().count())
            .unwrap_or(0);

        if qr_width == 0 {
            return String::new();
        }

        // Calculate how many QR codes can fit per row
        // Add 2 spaces padding between QR codes
        let qr_codes_per_row =
            ((self.terminal_width + 2) / (qr_width + 2)).max(1);

        let mut result = Vec::new();

        // Process QR codes in chunks (rows)
        for chunk in self.qr_codes.chunks(qr_codes_per_row) {
            let merged_row = Self::merge_qr_codes_horizontally(chunk);
            result.push(merged_row);
        }

        result.join("\n\n")
    }

    /// Merge multiple QR codes side-by-side into a single string
    fn merge_qr_codes_horizontally(qr_codes: &[String]) -> String {
        if qr_codes.is_empty() {
            return String::new();
        }

        // Split each QR code into lines
        let qr_lines: Vec<Vec<&str>> = qr_codes
            .iter()
            .map(|qr| qr.lines().collect())
            .collect();

        // Find the maximum number of lines
        let max_lines = qr_lines
            .iter()
            .map(|lines| lines.len())
            .max()
            .unwrap_or(0);

        let mut merged = Vec::new();

        // For each line number, concatenate lines from all QR codes
        for line_num in 0..max_lines {
            let mut line_parts = Vec::new();

            for (i, qr) in qr_lines.iter().enumerate() {
                if line_num < qr.len() {
                    line_parts.push(qr[line_num].to_string());
                } else {
                    // Pad with spaces if this QR code has fewer lines
                    let width = qr.first().map(|l| l.chars().count()).unwrap_or(0);
                    line_parts.push(" ".repeat(width));
                }

                // Add spacing between QR codes (except after the last one)
                if i < qr_lines.len() - 1 {
                    line_parts.push("  ".to_string());
                }
            }

            merged.push(line_parts.join(""));
        }

        merged.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_single_qr() {
        let qr = "███\n█ █\n███".to_string();
        let result = QrGrid::merge_qr_codes_horizontally(&[qr.clone()]);
        assert_eq!(result, qr);
    }

    #[test]
    fn merge_two_qr_codes() {
        let qr1 = "███\n█ █\n███".to_string();
        let qr2 = "▄▄▄\n▄ ▄\n▄▄▄".to_string();

        let result = QrGrid::merge_qr_codes_horizontally(&[qr1, qr2]);

        let expected = "███  ▄▄▄\n█ █  ▄ ▄\n███  ▄▄▄";
        assert_eq!(result, expected);
    }

    #[test]
    fn merge_different_heights() {
        let qr1 = "███\n█ █".to_string();
        let qr2 = "▄▄▄\n▄ ▄\n▄▄▄".to_string();

        let result = QrGrid::merge_qr_codes_horizontally(&[qr1, qr2]);

        let expected = "███  ▄▄▄\n█ █  ▄ ▄\n     ▄▄▄";
        assert_eq!(result, expected);
    }

    #[test]
    fn empty_qr_codes() {
        let result = QrGrid::merge_qr_codes_horizontally(&[]);
        assert_eq!(result, "");
    }
}
