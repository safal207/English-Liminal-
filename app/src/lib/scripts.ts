/**
 * Scripts Layer: Types and utilities for YAML scenario handling
 *
 * This module defines the core types for the English-Liminal script system
 * and provides functions for loading, validating, and managing scenarios.
 */

// ============================================================================
// Core Step Types
// ============================================================================

/**
 * Listen step: User listens to audio/text
 */
export type ListenStep = {
  type: 'listen';
  prompt: string;
  content: string;
  target?: string;
};

/**
 * Speak Check step: User repeats a phrase, gets pronunciation feedback
 */
export type SpeakCheckStep = {
  type: 'speak_check';
  prompt: string;
  content: string;
  ref_text: string;
  eval: 'pronunciation' | 'fluency' | 'both';
  hints?: string[];
};

/**
 * Contrast step: Multiple choice or comparison exercise
 */
export type ContrastStep = {
  type: 'contrast';
  prompt: string;
  question: string;
  answers: {
    text: string;
    correct: boolean;
    explanation: string;
  }[];
};

/**
 * Apply to Life step: User creates their own example
 */
export type ApplyToLifeStep = {
  type: 'apply_to_life';
  prompt: string;
  instruction: string;
  examples?: string[];
  recording: boolean;
};

/**
 * Union type for all step types
 */
export type Step =
  | ListenStep
  | SpeakCheckStep
  | ContrastStep
  | ApplyToLifeStep;

// ============================================================================
// Context and Metadata Types
// ============================================================================

/**
 * Context triggers: when and how to show this scenario
 */
export type ContextTrigger = {
  time?: 'morning' | 'afternoon' | 'evening' | 'night';
  location?: 'home' | 'work' | 'commute' | 'cafe' | 'gym';
  duration_max_sec?: number;
  user_level?: 'beginner' | 'intermediate' | 'advanced';
};

/**
 * Rehearsal configuration: how the scenario should repeat over time
 */
export type RehearsalConfig = {
  decay_alpha: number;           // 0.7-0.9: speed of memory decay
  next_ping_sec_min: number;     // Minimum seconds until next ping
  next_ping_sec_max: number;     // Maximum seconds until next ping
  importance?: 'low' | 'medium' | 'high';
  tags?: string[];
};

// ============================================================================
// Main Script Type
// ============================================================================

/**
 * Complete script/scenario definition
 */
export type Script = {
  id: string;
  title: string;
  description?: string;
  context_triggers?: ContextTrigger[];
  goals?: string[];
  steps: Step[];
  rehearsal?: RehearsalConfig;
};

/**
 * Script metadata (for listing scenarios without loading full content)
 */
export type ScriptMeta = {
  id: string;
  title: string;
  description?: string;
  duration_sec?: number;
  tags?: string[];
  difficulty?: 'beginner' | 'intermediate' | 'advanced';
};

// ============================================================================
// Script Loading and Management
// ============================================================================

/**
 * In-memory cache for loaded scripts
 */
const scriptCache = new Map<string, Script>();

/**
 * Load a script by ID from YAML files
 *
 * @param id - Script ID (e.g., 'morning-warmup-01')
 * @returns Parsed and validated Script object
 * @throws Error if script not found or invalid
 *
 * @example
 * const script = await loadScript('morning-warmup-01');
 * console.log(script.title); // "Morning Warmup: A/An & H-dropping"
 */
export async function loadScript(id: string): Promise<Script> {
  // Check cache first
  if (scriptCache.has(id)) {
    return scriptCache.get(id)!;
  }

  // TODO: Implementation in Issue #3
  // 1. Load YAML file from app/src/data/${id}.yaml
  // 2. Parse with js-yaml
  // 3. Validate with zod
  // 4. Cache result
  // 5. Return Script object

  throw new Error(`Script loading not yet implemented. Run Issue #3 first.`);
}

/**
 * Get list of all available scripts
 *
 * @returns Array of script metadata
 *
 * @example
 * const scripts = await listAvailableScripts();
 * scripts.forEach(s => console.log(s.title));
 */
export async function listAvailableScripts(): Promise<ScriptMeta[]> {
  // TODO: Implementation in Issue #3
  // 1. Scan app/src/data/ directory
  // 2. Load metadata from each YAML
  // 3. Return array of ScriptMeta

  return [];
}

/**
 * Preload a script into cache
 *
 * @param id - Script ID to preload
 */
export async function preloadScript(id: string): Promise<void> {
  await loadScript(id);
}

/**
 * Clear script cache (useful for testing or reload)
 */
export function clearScriptCache(): void {
  scriptCache.clear();
}

// ============================================================================
// Validation Schemas (for zod)
// ============================================================================

/**
 * Zod schemas will be implemented in Issue #3
 * Example structure:
 *
 * import { z } from 'zod';
 *
 * const StepSchema = z.union([
 *   z.object({ type: z.literal('listen'), ... }),
 *   z.object({ type: z.literal('speak_check'), ... }),
 *   ...
 * ]);
 *
 * const ScriptSchema = z.object({
 *   id: z.string(),
 *   title: z.string(),
 *   steps: z.array(StepSchema),
 *   ...
 * });
 */

// ============================================================================
// Exports
// ============================================================================

export default {
  loadScript,
  listAvailableScripts,
  preloadScript,
  clearScriptCache,
};
