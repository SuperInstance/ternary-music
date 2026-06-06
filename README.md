# ternary-music

Musical theory with ternary harmony — chord classification (tension/neutral/resolution), interval consonance, rhythmic patterns on {-1, 0, +1}, harmonic progression, voice leading, and scale degree analysis.

## Background

Every culture that has developed harmony has discovered the tension-resolution cycle. Indian raga organizes notes around a vadi (most important) and samvadi (second most). West African music balances call and response. Western tonal music built an elaborate syntax around dominant-to-tonic resolution.

`ternary-music` maps this universal principle onto three-valued logic. Every musical element — chord quality, interval consonance, rhythmic emphasis, scale degree stability — is classified as one of three states: tension (−1), neutral (0), or resolution (+1). This is not a simplification but a different lens: by reducing music theory to its ternary skeleton, the essential dynamics become visible.

The crate works with the standard 12-tone chromatic system but classifies every element into ternary categories. This is the bridge between the rich chromatic universe and the minimal ternary representation used by the rest of the Oxide stack.

## How It Works

### TernaryChord Classification

Every chord is classified into one of three categories:

| Classification | Ternary | Chord Types                    |
|---------------|---------|--------------------------------|
| Resolution    | +1      | Major triads, minor triads     |
| Neutral       | 0       | Suspended chords, power chords |
| Tension       | −1      | Diminished, augmented, 7ths    |

The classification is based on interval content: major and minor triads contain consonant thirds and fifths; diminished and augmented triads contain dissonant intervals; dominant sevenths add the tritone-bearing seventh.

### Interval Consonance

Each of the 12 chromatic intervals is mapped to a ternary consonance value:

| Ternary | Intervals                         | Category     |
|---------|-----------------------------------|--------------|
| +1      | Unison, perfect fifth             | Consonant    |
| 0       | Minor 3rd, major 3rd, minor 6th, major 6th | Neutral |
| −1      | Seconds, tritone, sevenths        | Dissonant    |

### Harmonic Progression

A `Progression` is a sequence of chords. The crate provides:

- **Tension curve** — ternary classification at each position, revealing the harmonic narrative
- **Resolution check** — whether the progression ends on a resolution chord
- **Classical ii-V-I** — the most common jazz turnaround, pre-built
- **Ternary balance** — count of tension/neutral/resolution chords

The ii-V-I in ternary produces the curve [Resolution, Tension, Resolution] — a micro-tension arc that mirrors the traditional dominant preparation-resolution syntax.

### Rhythmic Patterns

Ternary rhythmic patterns use signed intensity: +1 (strong), 0 (medium), −1 (weak). Pre-built patterns include:

- **4/4** — [1, −1, 0, −1, 1, −1, 0, −1] with strong beats on 1 and 5
- **Waltz (3/4)** — [1, −1, −1] with the characteristic oom-pah-pah

### Voice Leading

`VoiceLeading::between(from, to)` computes the semitone movement for each voice, taking the shortest path around the chromatic circle. Key metrics:

- **Ternary motions** — each voice classified as up (+1), static (0), or down (−1)
- **Distance** — total semitone movement across all voices
- **Smoothness** — proportion of voices moving ≤ 2 semitones

### Scale Degree Classification

Each degree of the major/minor scale is classified by stability:

| Ternary | Degrees                    | Character    |
|---------|---------------------------|--------------|
| +1      | Tonic (1), Dominant (5)   | Stable       |
| 0       | Mediant (3), Submediant (6)| Passing     |
| −1      | Supertonic (2), Subdominant (4), Leading tone (7) | Tendency |

## Experimental Results

- **The ii-V-I maps cleanly to ternary.** The progression produces [Resolution, Tension, Resolution], confirming that the most important chord progression in Western music has a simple ternary structure: a brief tension spike followed by resolution.
- **Voice leading smoothness is high for diatonic progressions.** Moving between chords within a key typically produces smoothness > 0.6, meaning most voices move by ≤ 2 semitones.
- **Major and minor triads are both "resolution."** Unlike traditional theory where major is "bright" and minor is "dark," ternary classification treats them identically — both are consonant, stable sonorities. The distinction is lost at ternary resolution.
- **Net emphasis of 4/4 is negative.** The pattern [1, −1, 0, −1, 1, −1, 0, −1] sums to −2, reflecting the inherent asymmetry of common-time meter: weak beats outnumber strong beats.

## Impact

`ternary-music` demonstrates that the core concepts of Western music theory — consonance, resolution, tension curves, voice leading — can be faithfully represented in ternary without losing their essential character. The classification system provides a universal "checksum" for harmonic content: any chord or progression can be reduced to its ternary skeleton and compared.

This has practical implications for music information retrieval, algorithmic composition, and music education: ternary classification provides a lossy but meaningful compression of harmonic information.

## Use Cases

1. **Harmonic analysis** — Reduce complex chord progressions to ternary tension curves for comparison, clustering, and similarity search.
2. **Generative composition** — Use ternary voice leading constraints to generate smooth chord progressions with guaranteed consonance properties.
3. **Music education** — Teach tension-resolution dynamics using a simple three-valued system that students can internalize quickly.
4. **Music information retrieval** — Index and search chord progressions by their ternary tension profile rather than exact chord quality.

## Open Questions

1. **Non-Western harmony.** How well does the ternary classification map to non-Western harmonic systems? Would Indian raga, Japanese pentatonic, or Arabic maqam produce different ternary profiles?
2. **Ternary rhythm interaction.** How do ternary rhythmic patterns interact with ternary harmonic tension? Does aligning strong beats with tension chords (or vice versa) produce perceptible effects?
3. **Higher-order ternary music.** Could ternary classification be extended to form, phrase structure, or even large-scale formal architecture?

## Connection to Oxide Stack

`ternary-music` is the central music theory crate. It consumes `ternary-temperament` for chord construction in pure ternary pitch space, feeds `ternary-counterpoint` with interval and voice leading data, provides rhythmic patterns to `ternary-rhythm`, and connects to `ternary-ear` for pattern recognition in harmonic sequences. The ternary chord classification framework is the same pattern used by `ternary-color` for temperature classification.
