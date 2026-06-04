# Future Integration: ternary-music

## Current State
Provides ternary chord classification (tension/neutral/resolution), rhythmic patterns on {-1, 0, +1}, harmonic progression rules, voice leading with ternary motion, and interval classification with consonance ratings.

## Integration Opportunities

### With ternary-cell (Rhythmic Tick Cycles)
The 6-phase cell tick (acquire → predict → update → surprise → vibe → gc) is a rhythmic pattern. `ternary-music` provides the formal framework: each phase maps to a beat, `TernaryChord::Tension` maps to the `surprise` phase, `TernaryChord::Resolution` maps to `gc`. Voice leading rules ensure smooth transitions between cell states — no jarring jumps.

### With ternary-reservoir
Echo state networks are temporal pattern recognizers. Musical rhythmic patterns on {-1, 0, +1} are exactly the input domain. A reservoir trained on ternary music patterns could recognize temporal patterns in room state sequences — is this room "in rhythm" or "syncopated"?

### With flux-algebra (PLATO music rooms)
The PLATO music room uses flux-algebra for PLR group operations and tuning fields. `ternary-music` provides the higher-level music theory that flux-algebra implements at the algebraic level. Together: music theory + music algebra = complete music room.

## Potential in Mature Systems
In room-as-codespace, rooms have "rhythms" — periodic patterns of activity. Music theory provides the vocabulary: a room in 4/4 time has regular predictable cycles; syncopated rooms need attention. Harmonic progression rules model how rooms interact: consonant rooms reinforce each other, dissonant rooms interfere.

## Cross-Pollination Ideas
- Rhythmic patterns as scheduling templates — schedule tasks on the beat, rest on the off-beat
- Chord progressions as room transition sequences — I-IV-V-I is a common room navigation pattern
- Voice leading as smooth state transitions in ternary-cell — minimize "melodic" distance between states

## Dependencies for Next Steps
- ternary-cell needs temporal pattern recognition hooks
- Integration with ternary-reservoir for temporal pattern matching
- flux-algebra bridge for the PLATO music room domain
