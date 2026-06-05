# ternary-music

**Music theory operations in ternary — harmony, rhythm, and voice leading on {-1, 0, +1}.**

Music theory is already ternary if you squint. Tension and release. Strong beats and weak beats. Consonance and dissonance. This crate makes that implicit trichotomy *explicit* — every chord is classified as Tension, Neutral, or Resolution; every interval is consonant (+1), neutral (0), or dissonant (-1); every rhythmic position is strong, medium, or weak. The result is a music theory that's both simpler and more revealing than traditional analysis.

## Why This Matters

Western music theory has 24 major and minor keys, dozens of chord qualities, and interval classifications that range from "perfect" to "augmented" to "diminished" — it's a lot of vocabulary for what's ultimately a system of tension and release. `ternary-music` strips it down to the energetic core. A ii-V-I isn't a Roman numeral exercise — it's a tension curve: `[-1, 0, +1]`. The whole of jazz harmony distilled to three numbers.

This isn't dumbing down. It's *clarifying*. When you see the ternary classification of a chord progression, you see its emotional shape at a glance. That's useful for composers, for analysis, and for any system that needs to reason about music computationally without getting lost in accidentals.

## What's Inside

### TernaryChord

- **`TernaryChord::Tension`** / **`Neutral`** / **`Resolution`** — The three chord flavors.
- **`to_ternary() → i8`** — Map to {-1, 0, +1}.
- **`from_ternary(i8) → Option<Self>`** — Parse back.

### Interval

- **`Interval::new(semitones)`** — Create (modulo 12).
- **`consonance() → i8`** — Classify: unison/fifth = +1, thirds/sixths = 0, seconds/tritones/sevenths = -1.
- **`invert() → Interval`** — Complement to octave.
- **`name() → &'static str`** — Human-readable name ("minor third", "perfect fifth", etc.).

### Chord

- **`Chord::major(root)`** / **`minor(root)`** / **`diminished(root)`** / **`augmented(root)`** / **`dominant_seventh(root)`** — Standard chord constructors.
- **`ternary_classify() → TernaryChord`** — Is this chord tension, neutral, or resolution?
- **`pitches() → Vec<u8>`** — All pitch classes in the chord.

### Progression

- **`Progression::new(chords)`** — Build from a sequence of chords.
- **`tension_curve() → Vec<i8>`** — The emotional shape of the progression as ternary values.
- **`resolves() → bool`** — Does it end on a resolution chord?
- **`Progression::ii_v_i(key)`** — Classic ii-V-I in any key.
- **`ternary_balance() → (usize, usize, usize)`** — Count of tension/neutral/resolution chords.

### RhythmicPattern

- **`RhythmicPattern::new(steps)`** — Create from ternary values (asserts -1..=1).
- **`RhythmicPattern::four_four()`** — Standard 4/4: `[1, -1, 0, -1, 1, -1, 0, -1]`.
- **`RhythmicPattern::waltz()`** — 3/4: `[1, -1, -1]`.
- **`strong_beats()` / `weak_beats() → Vec<usize>`** — Indices of accented positions.
- **`net_emphasis() → i32`** — Sum of all ternary values (positive = strong-heavy).
- **`rotate(n) → Self`** — Rotate the pattern by n steps.

### VoiceLeading

- **`VoiceLeading::between(from, to) → Self`** — Compute semitone motion per voice (shortest path on circle).
- **`ternary_motions() → Vec<i8>`** — Each voice: up (+1), static (0), down (-1).
- **`distance() → u32`** — Total semitones moved.
- **`smoothness() → f64`** — Proportion of voices moving ≤2 semitones.

### Scale

- **`Scale::major(root)`** / **`Scale::minor(root)`** — Standard scales.
- **`pitches() → Vec<u8>`** — All pitch classes.
- **`degree_ternary(degree) → i8`** — Tonic/dominant = +1 (stable), mediant/submediant = 0 (passing), supertonic/subdominant/leading-tone = -1 (tendency).
- **`contains(pitch) → bool`** — Is this note in the scale?

## Quick Example

```rust
use ternary_music::{Chord, Progression, VoiceLeading, RhythmicPattern, Scale, Interval};

// Build a ii-V-I in C and examine its tension curve
let prog = Progression::ii_v_i(0); // C major
println!("Tension curve: {:?}", prog.tension_curve()); // [1, -1, 1]
println!("Resolves? {}", prog.resolves()); // true

// Voice leading between C major and G major
let c = Chord::major(0);
let g = Chord::major(7);
let vl = VoiceLeading::between(&c, &g);
println!("Motions: {:?}", vl.ternary_motions());
println!("Smoothness: {:.2}", vl.smoothness());

// Standard 4/4 rhythm
let rhythm = RhythmicPattern::four_four();
println!("Strong beats at: {:?}", rhythm.strong_beats()); // [0, 4]
println!("Net emphasis: {}", rhythm.net_emphasis()); // -2

// Scale degree ternary classification
let scale = Scale::major(0);
for degree in 0..7 {
    let t = scale.degree_ternary(degree);
    let label = match t { 1 => "stable", 0 => "passing", _ => "tendency" };
    println!("Degree {}: {}", degree, label);
}

// Interval consonance
let fifth = Interval::new(7);
let tritone = Interval::new(6);
println!("P5 consonance: {}", fifth.consonance()); // +1
println!("Tritone consonance: {}", tritone.consonance()); // -1
```

## The Deeper Truth

The `ternary_classify` method on chords is doing something quietly radical. It maps the entire universe of chord qualities onto three categories: major and minor are Resolution, diminished and augmented are Tension, anything with a seventh creates Tension, and everything else is Neutral. If you're a jazz theorist, you might bristle — isn't a minor chord sometimes tense? Yes, but *contextually*. The ternary classification captures the *inherent* energy of a chord in isolation, which is the right level of abstraction for computational reasoning.

The `tension_curve` of a `Progression` might be the single most useful thing in this crate. It reduces a chord progression to a sequence of {-1, 0, +1} that captures its emotional arc. A ii-V-I gives you `[1, -1, 1]` — it dips into tension and resolves. A I-IV-V-I gives you `[1, 0, -1, 1]` — a longer journey through neutral territory before tension and release. You can plot these curves, compare them, cluster them. It's a compact fingerprint of musical shape.

Voice leading in ternary is where theory meets practice. The `VoiceLeading::between` method computes the shortest-path semitone movement for each voice between two chords, then `ternary_motions` collapses those to direction only: up, down, or static. This is *exactly* what matters for smooth voice leading — not how far you move, but whether you're moving at all. A voice that stays static while its neighbors shift creates a completely different feel than one that moves in parallel. The `smoothness` metric (proportion of voices with ≤2 semitone movement) is a practical shortcut for "does this sound good?"

The `Scale::degree_ternary` classification reveals something about tonal gravity that textbooks often gloss over. The tonic and dominant are "stable" (+1) — they're home base. The mediant and submediant are "passing" (0) — they're colorful but not directional. The supertonic, subdominant, and leading tone are "tendency" (-1) — they *want* to go somewhere. This maps perfectly onto how melodies actually move. Good melodies spend time on stable degrees, pass through neutral ones, and use tendency tones to create forward motion.

Rhythmic patterns in ternary are beautifully reductive. A 4/4 pattern as `[1, -1, 0, -1, 1, -1, 0, -1]` tells you everything: beats 1 and 5 are strong, the offbeats are weak, and beats 3 and 7 are medium. The `net_emphasis` of -2 for standard 4/4 is a fun fact — there's more *weakness* than *strength* in the default groove. Waltz at `[1, -1, -1]` has net emphasis -1, even more weighted toward release. The rhythm breathes outward from the downbeat.

## Use Cases

1. **Generative composition** — Build a system that generates chord progressions by constraining the tension curve (e.g., "must dip to -1 before resolving to +1 in the last two chords").

2. **Music education tools** — Visualize the tension/resolve cycle of songs students are learning. Seeing `[1, 0, -1, 1]` mapped to color on a timeline makes theory tangible.

3. **Algorithmic analysis** — Compare songs by their ternary tension curves. Two songs with identical curves *feel* similar regardless of key, tempo, or instrumentation.

4. **Rhythmic pattern library** — Build a searchable database of rhythms indexed by ternary pattern. "Find me all patterns with emphasis on beats 0 and 3."

5. **Voice leading validator** — In a composition tool, flag chord transitions where `smoothness()` drops below a threshold — that's where awkward voice leading lives.

## See Also

- **[ternary-harmonic](https://github.com/clarkeressel/ternary-harmonic)** — Harmonic analysis in ternary space
- **[ternary-wave](https://github.com/clarkeressel/ternary-wave)** — Ternary signal waveforms and oscillators
- **[ternary-rhythm](https://github.com/clarkeressel/ternary-rhythm)** — Rhythmic pattern generation and manipulation
- **[ternary-polyrhythm](https://github.com/clarkeressel/ternary-polyrhythm)** — Polyrhythmic structures using ternary patterns
- **[ternary-fib](https://github.com/clarkeressel/ternary-fib)** — Fibonacci-based ternary sequences for natural-sounding patterns
- **[ternary-jam](https://github.com/clarkeressel/ternary-jam)** — Live jamming with ternary patterns
- **[ternary-ear](https://github.com/clarkeressel/ternary-ear)** — Ear training for ternary musical intuition

## Install

```toml
[dependencies]
ternary-music = "0.1"
```

## License

MIT
