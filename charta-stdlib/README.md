# Charta Standard Library

Core blocks and patterns.

## Components

- Timer blocks (TON, TOF, TP, watchdog)
- Safety patterns (fail-safe gates, governance interlocks)
- HITL checkpoints
- Cost and resource governance blocks
- Explanation and contestability blocks
- Evidence normalization blocks

## Evidence Normalization Blocks

Blocks for transforming probabilistic inputs into structured evidence objects:

### NormalizeEvidence

Transforms raw probabilistic outputs (from LLMs, OCR, sensors) into structured evidence:

```charta
block NormalizeEvidence:
  inputs:
    raw: Any
    source: label  # "LLM", "OCR", "API", "user", "sensor"
    evidence_type: label  # "numeric_estimate", "categorical", "text_extraction"
    confidence: float  # Optional: explicit confidence, otherwise inferred
  outputs:
    evidence: Evidence[T]
  effect: Pure
```

### ExtractAndNormalize

Combines extraction and normalization for common patterns:

```charta
block ExtractAndNormalize:
  inputs:
    document: text
    field: label
    source: label
  outputs:
    evidence: Evidence[T]
  effect: Agent["llm.medium"]
```

### ConfidenceThreshold

Checks if evidence meets confidence threshold:

```charta
block ConfidenceThreshold:
  inputs:
    evidence: Evidence[T]
    threshold: float
  outputs:
    meets_threshold: Bool
    requires_verification: Bool
  effect: Pure
```

### DisambiguateEvidence

Handles ambiguous or contradictory evidence:

```charta
block DisambiguateEvidence:
  inputs:
    evidence_list: list[Evidence[T]]
  outputs:
    resolved: Evidence[T]
    ambiguous: Bool
    requires_hitl: Bool
  effect: Pure
```

