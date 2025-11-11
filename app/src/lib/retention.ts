/**
 * Retention Wave: Memory model for spaced repetition with emotional resonance
 *
 * This module implements the "wave" memory system where phrases decay
 * over time but can be reinforced through successful recall and real-world use.
 */

// ============================================================================
// Core Types
// ============================================================================

/**
 * Memory link: connection between a phrase and user's experience
 */
export type MemoryLink = {
  phrase: string;              // The actual phrase/content
  scriptId: string;            // Which scenario it came from
  stepIndex: number;           // Which step in the scenario
  lastSeen: number;            // Timestamp (ms) of last encounter
  wave: number;                // Current amplitude: 0 (forgotten) to 1 (strong)
  decayAlpha: number;          // Decay rate: 0.7 (fast) to 0.9 (slow)
  successCount: number;        // Number of successful recalls
  failCount: number;           // Number of failed recalls
  useInWildCount: number;      // Times marked as "said in real life"
  emotionalResonance?: number; // 0-1: how much it "clicked" for user
  contextTags?: string[];      // Associated contexts (cafe, work, etc)
};

/**
 * Outcome of a practice session
 */
export type PracticeOutcome =
  | 'success'      // Got it right
  | 'fail'         // Got it wrong
  | 'partial'      // Partially correct
  | 'skip';        // User skipped

/**
 * Ping schedule entry
 */
export type PingSchedule = {
  linkId: string;              // Which memory link
  scheduledFor: number;        // Timestamp when to ping
  priority: number;            // 0-1: higher = more urgent
  context?: string;            // Suggested context for ping
};

// ============================================================================
// Wave Dynamics
// ============================================================================

/**
 * Update wave amplitude based on practice outcome
 *
 * Algorithm:
 * - Success: wave += (1 - wave) * 0.3  (boost towards 1)
 * - Fail: wave *= decayAlpha            (decay)
 * - Partial: wave += (1 - wave) * 0.15  (small boost)
 * - Skip: wave *= 0.95                  (slight decay)
 *
 * @param link - Memory link to update
 * @param outcome - Result of practice
 * @returns Updated memory link
 *
 * @example
 * let link = { phrase: "in an hour", wave: 0.5, ... };
 * link = updateWave(link, 'success');
 * console.log(link.wave); // ~0.65
 */
export function updateWave(
  link: MemoryLink,
  outcome: PracticeOutcome
): MemoryLink {
  const updated = { ...link };
  updated.lastSeen = Date.now();

  switch (outcome) {
    case 'success':
      updated.wave += (1 - updated.wave) * 0.3;
      updated.successCount += 1;
      break;

    case 'fail':
      updated.wave *= updated.decayAlpha;
      updated.failCount += 1;
      break;

    case 'partial':
      updated.wave += (1 - updated.wave) * 0.15;
      updated.successCount += 0.5; // Partial credit
      break;

    case 'skip':
      updated.wave *= 0.95;
      break;
  }

  // Clamp wave between 0 and 1
  updated.wave = Math.max(0, Math.min(1, updated.wave));

  return updated;
}

/**
 * Calculate natural decay over time
 *
 * Formula: wave(t) = wave(0) * exp(-lambda * t)
 * where lambda = -ln(decayAlpha) / day_in_ms
 *
 * @param link - Memory link
 * @param currentTime - Current timestamp (default: now)
 * @returns Decayed wave amplitude
 */
export function calculateDecay(
  link: MemoryLink,
  currentTime: number = Date.now()
): number {
  const timeDiff = currentTime - link.lastSeen;
  const dayInMs = 24 * 60 * 60 * 1000;
  const lambda = -Math.log(link.decayAlpha) / dayInMs;

  const decayedWave = link.wave * Math.exp(-lambda * timeDiff);
  return Math.max(0, Math.min(1, decayedWave));
}

// ============================================================================
// Ping Scheduling
// ============================================================================

/**
 * Schedule next ping for a memory link
 *
 * Strategy:
 * - Lower wave → sooner ping (needs reinforcement)
 * - Higher wave → later ping (already strong)
 * - Add randomness to avoid predictability
 *
 * @param link - Memory link
 * @param minSec - Minimum seconds until ping (from rehearsal config)
 * @param maxSec - Maximum seconds until ping
 * @returns Timestamp for next ping
 *
 * @example
 * const nextPing = scheduleNextPing(link, 90, 3600);
 * console.log(new Date(nextPing)); // Some time in next 1-60 minutes
 */
export function scheduleNextPing(
  link: MemoryLink,
  minSec: number = 90,
  maxSec: number = 3600
): number {
  const currentWave = calculateDecay(link);

  // Inverse relationship: low wave → short delay
  // wave=0 → minSec, wave=1 → maxSec
  const baseDelay = minSec + (maxSec - minSec) * currentWave;

  // Add ±20% randomness
  const randomFactor = 0.8 + Math.random() * 0.4;
  const finalDelay = baseDelay * randomFactor;

  return Date.now() + finalDelay * 1000;
}

/**
 * Get priority for showing a ping
 *
 * Priority factors:
 * - Wave amplitude (lower = higher priority)
 * - Time since last seen (longer = higher priority)
 * - Emotional resonance (higher = higher priority)
 * - Use-in-wild count (more = higher priority)
 *
 * @param link - Memory link
 * @returns Priority score 0-1 (higher = more urgent)
 */
export function calculatePriority(link: MemoryLink): number {
  const currentWave = calculateDecay(link);
  const daysSinceLastSeen = (Date.now() - link.lastSeen) / (24 * 60 * 60 * 1000);

  // Base priority from inverse of wave (weak memories = high priority)
  let priority = 1 - currentWave;

  // Boost for phrases not seen in a while (up to 7 days)
  const timeBoost = Math.min(daysSinceLastSeen / 7, 1) * 0.3;
  priority += timeBoost;

  // Boost for emotionally resonant phrases
  if (link.emotionalResonance) {
    priority += link.emotionalResonance * 0.2;
  }

  // Boost for phrases used in wild (shows real value)
  const wildBoost = Math.min(link.useInWildCount / 5, 1) * 0.2;
  priority += wildBoost;

  // Clamp to 0-1
  return Math.max(0, Math.min(1, priority));
}

/**
 * Mark a phrase as "used in the wild" (real-world usage)
 *
 * This is a strong positive signal that boosts the wave significantly
 * and increases emotional resonance.
 *
 * @param link - Memory link
 * @returns Updated link
 */
export function markUsedInWild(link: MemoryLink): MemoryLink {
  return {
    ...link,
    useInWildCount: link.useInWildCount + 1,
    wave: Math.min(1, link.wave + 0.2), // Big boost
    emotionalResonance: Math.min(1, (link.emotionalResonance || 0.5) + 0.1),
    lastSeen: Date.now(),
  };
}

// ============================================================================
// Storage Integration (will use AsyncStorage)
// ============================================================================

/**
 * These functions will be implemented in Issue #10 with AsyncStorage
 */

export async function saveMemoryLink(link: MemoryLink): Promise<void> {
  // TODO: Save to AsyncStorage
  throw new Error('Not implemented yet - see Issue #10');
}

export async function loadMemoryLink(linkId: string): Promise<MemoryLink | null> {
  // TODO: Load from AsyncStorage
  throw new Error('Not implemented yet - see Issue #10');
}

export async function getAllMemoryLinks(): Promise<MemoryLink[]> {
  // TODO: Load all links from AsyncStorage
  throw new Error('Not implemented yet - see Issue #10');
}

export async function getUpcomingPings(limit: number = 5): Promise<MemoryLink[]> {
  // TODO: Get links that need pinging soon, sorted by priority
  throw new Error('Not implemented yet - see Issue #10');
}

// ============================================================================
// Exports
// ============================================================================

export default {
  updateWave,
  calculateDecay,
  scheduleNextPing,
  calculatePriority,
  markUsedInWild,
  saveMemoryLink,
  loadMemoryLink,
  getAllMemoryLinks,
  getUpcomingPings,
};
