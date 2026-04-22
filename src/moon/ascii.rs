//! ASCII and Unicode art moon renderer.

use super::phase::{illumination_percent, phase_name, PhaseGranularity};

/// Render an ASCII art moon at the given character dimensions.
///
/// `width` and `height` should be odd for best centering. Typical sizes:
/// 11×11, 15×15, 21×11 (wider to compensate for character aspect ratio).
///
/// `fraction`: 0.0 = new moon, 0.5 = full moon, 1.0 = new moon.
///
/// Characters used:
/// - `O` — lit surface interior
/// - `@` — lit limb edge
/// - `.` — shadow surface
/// - ` ` — outside the disc
pub fn ascii_moon(fraction: f64, width: usize, height: usize) -> String {
    render_moon(fraction, width, height, 'O', '@', '.', '.')
}

/// Render an ASCII art moon with default dimensions (21×11).
pub fn ascii_moon_default(fraction: f64) -> String {
    ascii_moon(fraction, 21, 11)
}

/// Render a Unicode block moon at the given dimensions.
///
/// Uses denser block characters for a more visual result:
/// - `█` — lit surface
/// - `░` — shadow surface
/// - `▓` — terminator boundary
/// - ` ` — outside the disc
pub fn ascii_moon_unicode(fraction: f64, width: usize, height: usize) -> String {
    render_moon(fraction, width, height, '█', '▓', '░', '▓')
}

/// Compact single-line moon status.
///
/// Returns a string like `"🌒 Waxing Crescent (23%)"` or `"◐ First Quarter (50%)"`.
pub fn moon_status_line(fraction: f64, use_emoji: bool) -> String {
    let name = phase_name(fraction, PhaseGranularity::Standard);
    let icon: &str = if use_emoji { name.emoji() } else { name.symbol() };
    let illum = illumination_percent(fraction);
    format!("{} {} ({:.0}%)", icon, name.as_str(), illum)
}

// ---------------------------------------------------------------------------
// Core renderer
// ---------------------------------------------------------------------------

fn render_moon(
    fraction: f64,
    width: usize,
    height: usize,
    lit_fill: char,
    lit_edge: char,
    shadow_fill: char,
    term_char: char,
) -> String {
    if width == 0 || height == 0 {
        return String::new();
    }

    let phase = fraction.rem_euclid(1.0);

    // Characters are ~2× taller than wide; stretch x to appear circular.
    let aspect_ratio = 2.0_f64;
    let cx = width as f64 / 2.0;
    let cy = height as f64 / 2.0;
    let ry = (cy - 0.5).max(0.5);
    let rx = ry * aspect_ratio;

    // Terminator x-radius (in stretched character units).
    //
    // Waxing (0→0.5): terminator moves from right limb (+rx) to left limb (−rx).
    //   Lit region = x ≥ terminator.
    // Waning (0.5→1.0): terminator moves from right limb (+rx) back to right (−rx).
    //   Lit region = x ≤ terminator.
    //
    // At phase=0.25: term_rx=0 → right half lit (first quarter). ✓
    // At phase=0.5:  waxing term_rx=−rx → all lit (full moon). ✓
    // At phase=0.75: waning term_rx=0 → left half lit (last quarter). ✓
    let term_rx: f64 = if phase <= 0.5 {
        rx * (1.0 - phase * 4.0).clamp(-1.0, 1.0)
    } else {
        rx * (3.0 - phase * 4.0).clamp(-1.0, 1.0)
    };

    let mut lines = Vec::with_capacity(height);

    for row in 0..height {
        let mut line = String::with_capacity(width);
        let y = row as f64 - cy;
        let yn = y / ry;

        if yn.abs() > 1.0 {
            line.extend(std::iter::repeat(' ').take(width));
            lines.push(line);
            continue;
        }

        let chord = (1.0 - yn * yn).sqrt();
        let circle_x = rx * chord; // half-width of disc at this row
        let term_x = term_rx * chord; // terminator x at this row

        for col in 0..width {
            let x = col as f64 - cx;

            if x.abs() > circle_x {
                line.push(' ');
                continue;
            }

            let lit = if phase <= 0.5 {
                x >= term_x
            } else {
                x <= term_x
            };

            let near_edge = circle_x - x.abs() < 1.0;
            let near_term = (x - term_x).abs() < 1.2 && term_x.abs() < circle_x;

            let ch = if near_edge {
                if lit { lit_edge } else { shadow_fill }
            } else if near_term {
                term_char
            } else if lit {
                lit_fill
            } else {
                shadow_fill
            };

            line.push(ch);
        }

        lines.push(line);
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_moon_default_dimensions() {
        let art = ascii_moon_default(0.5);
        let lines: Vec<&str> = art.lines().collect();
        assert_eq!(lines.len(), 11);
        assert!(lines[0].len() <= 21);
    }

    #[test]
    fn ascii_new_moon_mostly_shadow() {
        let art = ascii_moon(0.0, 21, 11);
        let lit = art.chars().filter(|&c| c == 'O' || c == '@').count();
        let shadow = art.chars().filter(|&c| c == '.').count();
        assert!(shadow > lit, "new moon should be mostly shadow (lit={lit}, shadow={shadow})");
    }

    #[test]
    fn ascii_full_moon_mostly_lit() {
        let art = ascii_moon(0.5, 21, 11);
        let lit = art.chars().filter(|&c| c == 'O' || c == '@').count();
        let shadow = art.chars().filter(|&c| c == '.').count();
        assert!(lit > shadow, "full moon should be mostly lit (lit={lit}, shadow={shadow})");
    }

    #[test]
    fn ascii_moon_no_panic_edge_cases() {
        ascii_moon(0.0, 1, 1);
        ascii_moon(0.5, 3, 3);
        ascii_moon(0.75, 51, 25);
        ascii_moon(-0.5, 11, 11);
        ascii_moon(2.5, 11, 11);
    }

    #[test]
    fn ascii_moon_unicode_produces_output() {
        let art = ascii_moon_unicode(0.5, 21, 11);
        assert!(art.contains('█'));
    }

    #[test]
    fn moon_status_line_contains_name_and_percent() {
        let s = moon_status_line(0.5, true);
        assert!(s.contains("Full Moon"), "got: {s}");
        assert!(s.contains("100%") || s.contains("99%"), "got: {s}");
    }
}
