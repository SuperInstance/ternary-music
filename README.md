# ternary-music

Musical theory with ternary harmony classification (tension/neutral/resolution) and balanced ternary rhythmic patterns.

## Why This Exists

Music theory tools give you intervals, chords, and scales, but they don't answer the question "is this tense or resolved?" in a computable way. This crate classifies every chord and interval into a ternary tension space: tension (−1), neutral (0), or resolution (+1). You can compute tension curves over progressions, build ternary rhythmic patterns, and analyze voice leading smoothness — all in a framework that maps naturally to {-1, 0, +1} for downstream ternary processing.

## Core Concepts

- **Ternary chord classification** — Every chord maps to Tension (−1), Neutral (0), or Resolution (+1). Major and minor triads resolve; diminished and augmented triads create tension; seventh chords generally create tension.
- **Interval consonance** — Each semitone interval classified as consonant (+1: unison, perfect fifth), neutral (0: thirds, sixths), or dissonant (−1: seconds, tritone, sevenths).
- **Voice leading** — The movement of individual pitches between two chords. "Smoothness" is the proportion of voices moving 0–2 semitones. Distance is total semitone movement across all voices.
- **Rhythmic ternary pattern** — A sequence of beat strengths: +1 (strong), 0 (medium), −1 (weak/off-beat). Standard 4/4 and waltz patterns provided.
- **Scale degree ternary** — Each degree of a major scale classified as stable (+1: tonic, dominant), passing (0: mediant, submediant), or tendency (−1: supertonic, subdominant, leading tone).

## Quick Start

```toml
# Cargo.toml
[dependencies]
ternary-music = "0.1"
```

```rust
use ternary_music::*;

fn main() {
    // Analyze a ii-V-I progression
    let prog = Progression::ii_v_i(0); // key of C
    println!("Resolves: {}", prog.resolves()); // true

    let curve = prog.tension_curve(); // [1, -1, 1]
    println!("Tension curve: {:?}", curve);

    // Voice leading between C major and F major
    let c = Chord::major(0);
    let f = Chord::major(5);
    let vl = VoiceLeading::between(&c, &f);
    println!("Distance: {}, Smoothness: {:.2}", vl.distance(), vl.smoothness());

    // Ternary rhythm
    let rhythm = RhythmicPattern::four_four();
    println!("Strong beats: {:?}", rhythm.strong_beats()); // [0, 4]
}
```

## API Overview

| Type | Description |
|------|-------------|
| `TernaryChord` | Tension/Neutral/Resolution classification |
| `Interval` | Semitone interval with consonance classification and inversion |
| `Chord` | Chord from root + intervals, with factory methods for triads/sevenths |
| `Progression` | Sequence of chords with tension curve analysis |
| `RhythmicPattern` | Ternary beat pattern (strong/medium/weak) |
| `VoiceLeading` | Analysis of pitch movement between two chords |
| `Scale` | Major or minor scale with ternary degree classification |

## How It Works

Chord classification uses a lookup approach: the intervals vector is matched against known patterns. Major `[0,4,7]` and minor `[0,3,7]` triads resolve; diminished `[0,3,6]` and augmented `[0,4,8]` create tension; anything containing a seventh (interval 10) is tension. Everything else falls to neutral.

Voice leading computes the shortest-path semitone distance between corresponding pitches of two chords, wrapping around the 12-tone octave. If the raw difference exceeds 6 semitones, it wraps (e.g., moving up 11 semitones becomes moving down 1). Ternary motions collapse each voice's movement to up (+1), static (0), or down (−1).

The `Progression::ii_v_i` factory builds the jazz standard ii-V-I cadence: supertonic minor, dominant seventh, tonic major. The tension curve reads [Resolution, Tension, Resolution] — a textbook tension-release arc.

## Known Limitations

- **No rhythm-to-ternary conversion for arbitrary time signatures.** Only 4/4 and 3/4 factory patterns are provided. You can construct custom patterns manually via `RhythmicPattern::new()`.
- **Voice leading assumes voice pairs by position.** It matches the i-th pitch of the source chord to the i-th pitch of the target. It doesn't find the globally optimal voice assignment that minimizes total movement.
- **No chord spelling or enharmonic handling.** Everything is pitch classes 0–11. No distinction between C♯ and D♭.

## Use Cases

- **Generative music systems** — Use tension curves to automatically create progressions that build and release tension in controlled patterns.
- **Music education tools** — Classify and visualize why a progression "works" by showing its ternary tension/resolution structure.
- **Algorithmic composition** — Build ternary rhythmic patterns and constrain melody generation by scale degree stability.

## Ecosystem Context

Part of the SuperInstance ternary crate family. `ternary-music` is a standalone leaf crate. Its ternary outputs (chord classification, interval consonance, beat strength) use the standard {-1, 0, +1} encoding and compose naturally with `ternary-diff` for comparing progressions, `ternary-visualization` for rendering tension curves, or `ternary-cell` for cellular rhythm simulation.

## License

MIT
