# English-Liminal Architecture

Detailed technical architecture for the LIMINAL system.

## System Overview

```
┌─────────────────────────────────────────────────────────────┐
│                         USER LAYER                          │
│  (iOS/Android App - React Native + Expo)                   │
└────────────────────┬────────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────────┐
│                    PRESENTATION LAYER                        │
│  • WarmupScreen, PingScreen, RinseScreen                    │
│  • TTSButton, Recorder, ProgressBar                         │
│  • Navigation (Bottom Tabs)                                 │
└────────────────────┬────────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────────┐
│                    BUSINESS LOGIC LAYER                      │
│  • useScriptRunner (step execution)                         │
│  • useRetention (wave memory)                               │
│  • useNotifications (scheduling)                            │
└────────────────────┬────────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────────┐
│                      SERVICES LAYER                          │
│  • Scripts Service (YAML → Script objects)                  │
│  • Retention Service (wave calculations)                    │
│  • Storage Service (AsyncStorage wrapper)                   │
│  • Voice Service (TTS/STT)                                  │
│  • Notification Service (expo-notifications)                │
└────────────────────┬────────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────────┐
│                       DATA LAYER                             │
│  • AsyncStorage (local key-value store)                     │
│  • YAML files (scenarios in app/src/data/)                 │
│  • Future: Supabase/Neo4j for cloud sync                   │
└─────────────────────────────────────────────────────────────┘
```

## Core Modules

### 1. Scripts Layer (`app/src/lib/scripts.ts`)

**Purpose:** Load, parse, and validate YAML scenarios

**Key Types:**
- `Step`: Union type for all step types (listen, speak_check, contrast, apply_to_life)
- `Script`: Complete scenario definition
- `ScriptMeta`: Lightweight metadata for listings

**Key Functions:**
- `loadScript(id)`: Load and parse YAML → Script object
- `listAvailableScripts()`: Get all scenario metadata
- `preloadScript(id)`: Cache script in memory

**Dependencies:**
- `js-yaml`: YAML parsing
- `zod`: Schema validation

### 2. Retention Wave (`app/src/lib/retention.ts`)

**Purpose:** Memory model for spaced repetition with emotional resonance

**Key Concepts:**
- **Wave Amplitude** (0-1): Strength of memory
- **Decay Alpha** (0.7-0.9): Rate of forgetting
- **Priority**: Urgency for review based on wave + time + emotion

**Key Functions:**
- `updateWave(link, outcome)`: Adjust wave based on practice result
- `calculateDecay(link)`: Natural forgetting over time
- `scheduleNextPing(link)`: When to show next
- `markUsedInWild(link)`: Boost for real-world usage

**Algorithm:**
```
Success:  wave += (1 - wave) * 0.3
Fail:     wave *= decayAlpha
Decay:    wave(t) = wave(0) * exp(-λt)
Priority: f(wave, time, emotion, wild_use)
```

### 3. Script Runner Hook (`app/src/hooks/useScriptRunner.ts`)

**Purpose:** Manage state and flow through scenario steps

**State:**
- `currentStep`: Currently active step
- `stepIndex`: Position in steps array
- `progress`: Percentage complete (0-100)
- `isComplete`: Boolean flag

**Actions:**
- `next()`: Advance to next step
- `prev()`: Go back one step
- `reset()`: Start over

**Example Usage:**
```typescript
const { currentStep, next, progress } = useScriptRunner('morning-warmup-01');

// Render current step
{currentStep.type === 'listen' && <ListenStep step={currentStep} />}
{currentStep.type === 'speak_check' && <SpeakCheckStep step={currentStep} onComplete={next} />}
```

### 4. Storage Service (`app/src/lib/storage.ts`)

**Purpose:** Persist user data and events

**Data Models:**

```typescript
UserEvent {
  type: 'step_done' | 'said_in_wild' | 'score'
  timestamp: number
  scriptId: string
  stepIndex: number
  data: any  // score, phrase, etc.
}

DailySummary {
  date: string
  streak: number
  avgScore: number
  useInWildCount: number
}
```

**Key Functions:**
- `addEvent(event)`: Log a user action
- `getStats()`: Aggregate statistics
- `getStreak()`: Calculate current streak
- `getDailySummary(date)`: Get summary for specific day

### 5. Voice Service

**Components:**
- **TTSButton** (`app/src/components/TTSButton.tsx`)
  - Integrates `expo-speech`
  - States: idle, playing, paused
  - Props: `text`, `onComplete`

- **Recorder** (`app/src/components/Recorder.tsx`)
  - Integrates `expo-av` for audio recording
  - MVP: Manual text input for "recognized" text
  - Future: Real STT integration
  - Scoring via Levenshtein distance

**Scoring Algorithm (MVP):**
```typescript
function calculateScore(userText: string, refText: string): number {
  const distance = levenshtein(normalize(userText), normalize(refText));
  const maxLen = Math.max(userText.length, refText.length);
  return Math.round((1 - distance / maxLen) * 100);
}
```

### 6. Notification Service

**Purpose:** Schedule and handle local notifications

**Schedule:**
- Morning: Warmup reminder (default 08:00)
- Afternoon: Context ping (default 12:30)
- Evening: Rinse reminder (default 20:00)

**Deep Linking:**
- Notification → Open app → Navigate to specific screen
- URL scheme: `liminal://warmup`, `liminal://ping`, `liminal://rinse`

**Implementation:**
```typescript
import * as Notifications from 'expo-notifications';

// Schedule notification
await Notifications.scheduleNotificationAsync({
  content: {
    title: "Morning Warmup",
    body: "3 minutes to sharpen your English",
    data: { screen: 'warmup' }
  },
  trigger: { hour: 8, minute: 0, repeats: true }
});
```

## Data Flow Examples

### Example 1: Complete a Warmup Session

```
User opens WarmupScreen
  ↓
useScriptRunner('morning-warmup-01')
  ↓
Load script via loadScript()
  ↓
Parse YAML → Script object
  ↓
Initialize state: step 0, progress 0%
  ↓
Render ListenStep
  User taps "Play" → TTSButton plays audio
  User taps "Next"
  ↓
Advance to step 1 (speak_check)
  ↓
Render SpeakCheckStep
  User records audio via Recorder
  System calculates score
  User sees feedback + hint
  User taps "Next"
  ↓
Advance to step 2 (contrast)
  ↓
Render ContrastStep
  User selects answer
  System shows explanation
  User taps "Next"
  ↓
Advance to step 3 (apply_to_life)
  ↓
Render ApplyToLifeStep
  User creates own sentence
  System records/saves
  User taps "Done"
  ↓
Complete session
  ↓
Log events to storage:
  - step_done × 4
  - score (from speak_check)
  - custom phrase (from apply_to_life)
  ↓
Update retention wave for key phrases
  ↓
Schedule next ping
  ↓
Show completion summary
```

### Example 2: Retention Wave Update

```
User completes speak_check step with score 85
  ↓
Storage: addEvent({ type: 'score', score: 85, ... })
  ↓
Identify phrase: "I'll be there in an hour"
  ↓
Load/create MemoryLink for phrase
  ↓
updateWave(link, 'success')
  wave = 0.6 + (1 - 0.6) * 0.3 = 0.72
  successCount++
  lastSeen = now
  ↓
scheduleNextPing(link, 90, 3600)
  delay = 90 + (3600 - 90) * 0.72 = ~2600 sec
  nextPing = now + 2600s
  ↓
Save updated MemoryLink to AsyncStorage
  ↓
Later: Notification service checks pending pings
  Current time > nextPing?
  ↓
Send notification: "Quick practice: 'I'll be there in an hour'"
  ↓
User taps notification → PingScreen with this phrase
```

## Technology Stack

### Frontend (React Native + Expo)

**Core:**
- React Native (via Expo)
- TypeScript
- Expo SDK 50+

**Navigation:**
- @react-navigation/native
- @react-navigation/bottom-tabs

**Voice:**
- expo-speech (TTS)
- expo-av (audio recording)

**Storage:**
- @react-native-async-storage/async-storage

**Data:**
- js-yaml (YAML parsing)
- zod (validation)

**Notifications:**
- expo-notifications

### Future Backend (Optional)

**Real-time:**
- Supabase Realtime OR
- Phoenix Channels (Elixir) OR
- Socket.io (Node.js)

**Database:**
- Postgres (user data, events)
- Neo4j (resonance graph)
- Weaviate (semantic search)

**AI/LLM:**
- OpenAI API (GPT-4) for personalization
- Whisper API for STT
- ElevenLabs for high-quality TTS

## Performance Considerations

### Local-First Architecture

**MVP:** Everything runs locally on device
- No network required (offline-first)
- Fast response times
- Privacy by default

**Trade-offs:**
- No cloud sync (yet)
- Limited analytics
- No collaborative features

### Caching Strategy

**Scripts:**
- Preload all YAML on app start
- Keep in memory for session
- Reload on app restart

**Memory Links:**
- Lazy load from AsyncStorage
- Cache active links in memory
- Batch writes (every 5 events)

### Optimization Tips

**YAML Loading:**
- Bundle YAMLs with app (not remote fetch)
- Parse once, cache result
- Consider pre-compiling to JSON for faster load

**Voice Operations:**
- TTS: Queue phrases, don't interrupt
- Recording: Limit to 10 seconds max
- Scoring: Run in background thread (future)

## Security & Privacy

**MVP Security:**
- All data stored locally (AsyncStorage)
- No cloud transmission
- No personal identifiable info collected

**Future Considerations:**
- End-to-end encryption for cloud sync
- Anonymized analytics (PostHog self-hosted)
- GDPR compliance for EU users
- Audio recordings: local only, never uploaded (unless user opts in)

## Deployment

**Development:**
```bash
pnpm expo start
# Scan QR with Expo Go app
```

**Preview Builds:**
```bash
eas build --profile preview --platform ios
eas build --profile preview --platform android
```

**Production:**
```bash
eas build --profile production --platform all
eas submit --platform ios
eas submit --platform android
```

## Testing Strategy

**Unit Tests:**
- `scripts.ts`: YAML parsing, validation
- `retention.ts`: Wave calculations, scheduling
- `storage.ts`: CRUD operations

**Integration Tests:**
- Full scenario flow (Warmup end-to-end)
- Notification scheduling and deep-linking
- Storage persistence across app restarts

**Manual Testing:**
- Voice recording quality
- TTS naturalness
- UI/UX smoothness
- Performance on low-end devices

## Monitoring & Analytics

**MVP (Local Only):**
- Event logging to AsyncStorage
- Export logs as JSON
- Manual analysis

**Future (Cloud):**
- PostHog for product analytics
- Sentry for error tracking
- Custom dashboards for retention metrics

---

For implementation details, see [CODEX_RUNBOOK.md](../CODEX_RUNBOOK.md).
For task breakdown, see [ISSUES.md](../ISSUES.md).
