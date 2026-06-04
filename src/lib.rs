//! # ternary-music
//!
//! Musical theory with ternary harmony. TernaryChord (tension/neutral/resolution),
//! rhythmic patterns on {-1,0,+1}, harmonic progression rules, voice leading
//! with ternary motion, interval classification.

#![forbid(unsafe_code)]

/// Ternary chord classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TernaryChord {
    Tension,
    Neutral,
    Resolution,
}

impl TernaryChord {
    pub fn to_ternary(self) -> i8 {
        match self {
            TernaryChord::Tension => -1,
            TernaryChord::Neutral => 0,
            TernaryChord::Resolution => 1,
        }
    }

    pub fn from_ternary(v: i8) -> Option<Self> {
        match v {
            -1 => Some(TernaryChord::Tension),
            0 => Some(TernaryChord::Neutral),
            1 => Some(TernaryChord::Resolution),
            _ => None,
        }
    }
}

/// Musical interval in semitones with ternary classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval {
    pub semitones: u8,
}

impl Interval {
    pub fn new(semitones: u8) -> Self {
        Self { semitones: semitones % 12 }
    }

    /// Classify interval as consonant (+1), neutral (0), or dissonant (-1).
    pub fn consonance(&self) -> i8 {
        match self.semitones {
            0 | 7 => 1,             // unison, perfect fifth — consonant
            3 | 4 | 8 | 9 => 0,    // minor/major third, minor/major sixth — neutral
            1 | 2 | 5 | 6 | 10 | 11 => -1, // seconds, tritone, sevenths — dissonant
            _ => 0,
        }
    }

    /// Invert the interval (complement to octave).
    pub fn invert(&self) -> Self {
        Interval::new(12 - self.semitones)
    }

    /// Common name for the interval.
    pub fn name(&self) -> &'static str {
        match self.semitones {
            0 => "unison",
            1 => "minor second",
            2 => "major second",
            3 => "minor third",
            4 => "major third",
            5 => "perfect fourth",
            6 => "tritone",
            7 => "perfect fifth",
            8 => "minor sixth",
            9 => "major sixth",
            10 => "minor seventh",
            11 => "major seventh",
            _ => "octave",
        }
    }
}

/// A chord built from a root and intervals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chord {
    pub root: u8,
    pub intervals: Vec<u8>,
}

impl Chord {
    pub fn new(root: u8, intervals: Vec<u8>) -> Self {
        Self { root, intervals }
    }

    /// Major triad.
    pub fn major(root: u8) -> Self {
        Self::new(root, vec![0, 4, 7])
    }

    /// Minor triad.
    pub fn minor(root: u8) -> Self {
        Self::new(root, vec![0, 3, 7])
    }

    /// Diminished triad.
    pub fn diminished(root: u8) -> Self {
        Self::new(root, vec![0, 3, 6])
    }

    /// Augmented triad.
    pub fn augmented(root: u8) -> Self {
        Self::new(root, vec![0, 4, 8])
    }

    /// Dominant seventh.
    pub fn dominant_seventh(root: u8) -> Self {
        Self::new(root, vec![0, 4, 7, 10])
    }

    /// Classify chord's ternary character.
    pub fn ternary_classify(&self) -> TernaryChord {
        // Major/minor = resolution, diminished/augmented = tension, others = neutral
        if self.intervals == vec![0, 4, 7] || self.intervals == vec![0, 3, 7] {
            TernaryChord::Resolution
        } else if self.intervals == vec![0, 3, 6] || self.intervals == vec![0, 4, 8] {
            TernaryChord::Tension
        } else if self.intervals.contains(&10) {
            TernaryChord::Tension // sevenths generally create tension
        } else {
            TernaryChord::Neutral
        }
    }

    /// Get all pitch classes in the chord.
    pub fn pitches(&self) -> Vec<u8> {
        self.intervals.iter().map(|&i| (self.root + i) % 12).collect()
    }
}

/// Harmonic progression (sequence of chords).
#[derive(Debug, Clone)]
pub struct Progression {
    pub chords: Vec<Chord>,
}

impl Progression {
    pub fn new(chords: Vec<Chord>) -> Self {
        Self { chords }
    }

    /// Analyze tension curve of the progression.
    pub fn tension_curve(&self) -> Vec<i8> {
        self.chords.iter().map(|c| c.ternary_classify().to_ternary()).collect()
    }

    /// Check if progression resolves (ends on resolution).
    pub fn resolves(&self) -> bool {
        self.chords.last().map_or(false, |c| c.ternary_classify() == TernaryChord::Resolution)
    }

    /// Classical ii-V-I in the given key (root pitch class).
    pub fn ii_v_i(key: u8) -> Self {
        Self::new(vec![
            Chord::minor((key + 2) % 12),  // ii
            Chord::dominant_seventh((key + 7) % 12), // V7
            Chord::major(key),              // I
        ])
    }

    /// Ternary balance of the progression.
    pub fn ternary_balance(&self) -> (usize, usize, usize) {
        let (mut t, mut n, mut r) = (0, 0, 0);
        for c in &self.chords {
            match c.ternary_classify() {
                TernaryChord::Tension => t += 1,
                TernaryChord::Neutral => n += 1,
                TernaryChord::Resolution => r += 1,
            }
        }
        (t, n, r)
    }
}

/// Ternary rhythmic pattern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RhythmicPattern {
    /// Steps as ternary values: -1 (off-beat/weak), 0 (medium), +1 (strong).
    pub steps: Vec<i8>,
}

impl RhythmicPattern {
    pub fn new(steps: Vec<i8>) -> Self {
        for &s in &steps {
            assert!(s >= -1 && s <= 1, "Steps must be -1, 0, or +1");
        }
        Self { steps }
    }

    /// Length of the pattern.
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    /// Strong beats.
    pub fn strong_beats(&self) -> Vec<usize> {
        self.steps.iter().enumerate().filter(|(_, &v)| v == 1).map(|(i, _)| i).collect()
    }

    /// Weak beats.
    pub fn weak_beats(&self) -> Vec<usize> {
        self.steps.iter().enumerate().filter(|(_, &v)| v == -1).map(|(i, _)| i).collect()
    }

    /// Sum of ternary values (net emphasis).
    pub fn net_emphasis(&self) -> i32 {
        self.steps.iter().map(|&v| v as i32).sum()
    }

    /// Standard 4/4 pattern.
    pub fn four_four() -> Self {
        Self::new(vec![1, -1, 0, -1, 1, -1, 0, -1])
    }

    /// Waltz (3/4) pattern.
    pub fn waltz() -> Self {
        Self::new(vec![1, -1, -1])
    }

    /// Shift pattern by n steps (rotation).
    pub fn rotate(&self, n: usize) -> Self {
        if self.steps.is_empty() { return self.clone(); }
        let len = self.steps.len();
        let n = n % len;
        let rotated: Vec<i8> = self.steps[n..].iter().chain(self.steps[..n].iter()).copied().collect();
        Self::new(rotated)
    }
}

/// Voice leading analysis between two chords.
#[derive(Debug, Clone)]
pub struct VoiceLeading {
    pub motions: Vec<i8>, // semitone movement per voice: negative=down, 0=static, positive=up
}

impl VoiceLeading {
    /// Compute voice leading between two chords.
    pub fn between(from: &Chord, to: &Chord) -> Self {
        let fp = from.pitches();
        let tp = to.pitches();
        let len = fp.len().min(tp.len());
        let motions: Vec<i8> = (0..len).map(|i| {
            let diff = tp[i] as i16 - fp[i] as i16;
            // Shortest path on circle
            if diff > 6 {
                (diff - 12) as i8
            } else if diff < -6 {
                (diff + 12) as i8
            } else {
                diff as i8
            }
        }).collect();
        Self { motions }
    }

    /// Classify each voice motion as up (+1), static (0), or down (-1).
    pub fn ternary_motions(&self) -> Vec<i8> {
        self.motions.iter().map(|&m| {
            if m > 0 { 1 } else if m < 0 { -1 } else { 0 }
        }).collect()
    }

    /// Total voice leading distance.
    pub fn distance(&self) -> u32 {
        self.motions.iter().map(|&m| m.unsigned_abs() as u32).sum()
    }

    /// Smoothness: proportion of voices with minimal movement (0-2 semitones).
    pub fn smoothness(&self) -> f64 {
        if self.motions.is_empty() { return 1.0; }
        let smooth = self.motions.iter().filter(|&&m| m.abs() <= 2).count();
        smooth as f64 / self.motions.len() as f64
    }
}

/// Scale with ternary degree classification.
#[derive(Debug, Clone)]
pub struct Scale {
    pub root: u8,
    pub intervals: Vec<u8>,
}

impl Scale {
    /// Major scale.
    pub fn major(root: u8) -> Self {
        Self { root, intervals: vec![0, 2, 4, 5, 7, 9, 11] }
    }

    /// Minor (natural) scale.
    pub fn minor(root: u8) -> Self {
        Self { root, intervals: vec![0, 2, 3, 5, 7, 8, 10] }
    }

    /// Get the pitch classes in this scale.
    pub fn pitches(&self) -> Vec<u8> {
        self.intervals.iter().map(|&i| (self.root + i) % 12).collect()
    }

    /// Classify a degree (0-indexed) as stable (+1), passing (0), or tendency (-1).
    pub fn degree_ternary(&self, degree: usize) -> i8 {
        match degree % 7 {
            0 | 4 => 1,   // tonic, dominant — stable
            2 | 5 => 0,   // mediant, submediant — passing
            1 | 3 | 6 => -1, // supertonic, subdominant, leading tone — tendency
            _ => 0,
        }
    }

    /// Check if a pitch class is in the scale.
    pub fn contains(&self, pitch: u8) -> bool {
        self.pitches().contains(&(pitch % 12))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ternary_chord_roundtrip() {
        for v in [-1i8, 0, 1] {
            assert_eq!(TernaryChord::from_ternary(v).unwrap().to_ternary(), v);
        }
    }

    #[test]
    fn test_interval_consonance() {
        assert_eq!(Interval::new(0).consonance(), 1); // unison
        assert_eq!(Interval::new(7).consonance(), 1); // fifth
        assert_eq!(Interval::new(6).consonance(), -1); // tritone
        assert_eq!(Interval::new(1).consonance(), -1); // minor second
    }

    #[test]
    fn test_interval_invert() {
        assert_eq!(Interval::new(5).invert(), Interval::new(7)); // P4 inverts to P5
        assert_eq!(Interval::new(3).invert(), Interval::new(9)); // m3 inverts to M6
    }

    #[test]
    fn test_interval_name() {
        assert_eq!(Interval::new(0).name(), "unison");
        assert_eq!(Interval::new(7).name(), "perfect fifth");
        assert_eq!(Interval::new(6).name(), "tritone");
    }

    #[test]
    fn test_chord_major_ternary() {
        assert_eq!(Chord::major(0).ternary_classify(), TernaryChord::Resolution);
    }

    #[test]
    fn test_chord_diminished_ternary() {
        assert_eq!(Chord::diminished(0).ternary_classify(), TernaryChord::Tension);
    }

    #[test]
    fn test_chord_dominant_seventh_ternary() {
        assert_eq!(Chord::dominant_seventh(0).ternary_classify(), TernaryChord::Tension);
    }

    #[test]
    fn test_chord_pitches() {
        let pitches = Chord::major(0).pitches();
        assert_eq!(pitches, vec![0, 4, 7]);
    }

    #[test]
    fn test_progression_resolves() {
        let prog = Progression::ii_v_i(0);
        assert!(prog.resolves());
    }

    #[test]
    fn test_progression_tension_curve() {
        let prog = Progression::ii_v_i(0);
        let curve = prog.tension_curve();
        assert_eq!(curve.len(), 3);
        // ii=Resolution(1), V7=Tension(-1), I=Resolution(1)
        assert_eq!(curve[0], 1);
        assert_eq!(curve[1], -1);
        assert_eq!(curve[2], 1);
    }

    #[test]
    fn test_progression_ternary_balance() {
        let prog = Progression::ii_v_i(0);
        let (t, _n, r) = prog.ternary_balance();
        assert_eq!(t, 1); // V7
        assert_eq!(r, 2); // ii and I
    }

    #[test]
    fn test_rhythmic_pattern_four_four() {
        let p = RhythmicPattern::four_four();
        assert_eq!(p.len(), 8);
        assert_eq!(p.strong_beats(), vec![0, 4]);
    }

    #[test]
    fn test_rhythmic_pattern_waltz() {
        let p = RhythmicPattern::waltz();
        assert_eq!(p.len(), 3);
        assert_eq!(p.strong_beats(), vec![0]);
    }

    #[test]
    fn test_rhythmic_pattern_rotate() {
        let p = RhythmicPattern::new(vec![1, 0, -1]);
        let rotated = p.rotate(1);
        assert_eq!(rotated.steps, vec![0, -1, 1]);
    }

    #[test]
    fn test_rhythmic_net_emphasis() {
        let p = RhythmicPattern::four_four();
        assert_eq!(p.net_emphasis(), -2); // [1,-1,0,-1,1,-1,0,-1] = -2
    }

    #[test]
    fn test_voice_leading_distance() {
        let c_major = Chord::major(0);
        let f_major = Chord::major(5);
        let vl = VoiceLeading::between(&c_major, &f_major);
        assert!(vl.distance() > 0);
    }

    #[test]
    fn test_voice_leading_smoothness() {
        let c = Chord::major(0);
        let vl = VoiceLeading::between(&c, &c);
        assert!((vl.smoothness() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_voice_leading_ternary_motions() {
        let c = Chord::major(0);
        let g = Chord::major(7);
        let vl = VoiceLeading::between(&c, &g);
        let motions = vl.ternary_motions();
        assert_eq!(motions.len(), 3);
    }

    #[test]
    fn test_scale_major_pitches() {
        let scale = Scale::major(0);
        assert_eq!(scale.pitches(), vec![0, 2, 4, 5, 7, 9, 11]);
    }

    #[test]
    fn test_scale_contains() {
        let scale = Scale::major(0);
        assert!(scale.contains(0));  // C
        assert!(scale.contains(7));  // G
        assert!(!scale.contains(1)); // C#
    }

    #[test]
    fn test_scale_degree_ternary() {
        let scale = Scale::major(0);
        assert_eq!(scale.degree_ternary(0), 1);  // tonic
        assert_eq!(scale.degree_ternary(4), 1);  // dominant
        assert_eq!(scale.degree_ternary(6), -1); // leading tone
    }

    #[test]
    fn test_chord_minor_ternary() {
        assert_eq!(Chord::minor(0).ternary_classify(), TernaryChord::Resolution);
    }

    #[test]
    fn test_chord_augmented_ternary() {
        assert_eq!(Chord::augmented(0).ternary_classify(), TernaryChord::Tension);
    }
}
