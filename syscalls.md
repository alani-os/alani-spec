---

## 📄 syscalls.md

```markdown
# Alani Cognitive Syscall Reference

> **Specification Version:** 0.1.0-draft
> **Last Updated:** April 2026
> **Status:** Open Specification

This document is the authoritative reference for the Alani cognitive syscall interface. All intelligence operations in Alani are mediated through kernel syscalls — permission-checked, auditable, and schedulable at the kernel boundary.

---

## Table of Contents

- [Overview](#overview)
- [Conventions](#conventions)
- [Type Definitions](#type-definitions)
- [Syscall Reference](#syscall-reference)
  - [sys_intent_create](#sys_intent_create)
  - [sys_intent_cancel](#sys_intent_cancel)
  - [sys_intent_status](#sys_intent_status)
  - [sys_context_resolve](#sys_context_resolve)
  - [sys_context_attach](#sys_context_attach)
  - [sys_model_infer](#sys_model_infer)
  - [sys_model_capabilities](#sys_model_capabilities)
  - [sys_memory_query](#sys_memory_query)
  - [sys_memory_write](#sys_memory_write)
  - [sys_memory_delete](#sys_memory_delete)
  - [sys_action_execute](#sys_action_execute)
  - [sys_action_validate](#sys_action_validate)
  - [sys_trace_emit](#sys_trace_emit)
  - [sys_trace_query](#sys_trace_query)
  - [sys_policy_check](#sys_policy_check)
- [Error Codes](#error-codes)
- [Permissions](#permissions)
- [Execution Semantics](#execution-semantics)
- [Tracing Requirements](#tracing-requirements)
- [Examples](#examples)

---

## Overview

The Alani cognitive syscall interface provides a formal, typed boundary between userspace agents and the kernel's cognitive subsystem. Every intelligence operation — from creating an intent to executing an action — passes through this interface.

### Design Goals

- **Uniformity** — All cognitive operations use the same syscall mechanism.
- **Auditability** — Every syscall invocation is logged in the execution trace.
- **Permission enforcement** — Capabilities are checked at the kernel boundary before dispatch.
- **Deterministic dispatch** — The kernel's handling of syscalls is deterministic; probabilistic behavior is confined to device execution (model inference).

### Syscall Flow

```
Userspace Agent
  │
  ├── sys_intent_create()        ─── Create a reasoning intent
  │     │
  │     ├── sys_context_resolve()  ─── Resolve context for the intent
  │     │
  │     ├── sys_model_infer()      ─── Invoke model device for inference
  │     │
  │     ├── sys_memory_write()     ─── Persist results to memory
  │     │
  │     └── sys_action_execute()   ─── Execute a decided action
  │
  └── sys_trace_emit()           ─── Emit trace for observability
```

---

## Conventions

### Naming

- All syscalls are prefixed with `sys_`.
- Syscall names follow the pattern `sys_<domain>_<operation>`.
- Domains: `intent`, `context`, `model`, `memory`, `action`, `trace`, `policy`.

### Return Values

- All syscalls return `int`.
- `0` indicates success.
- Negative values indicate errors (see [Error Codes](#error-codes)).
- Positive values are reserved for future use.

### Pointer Parameters

- All pointer parameters are non-null unless documented otherwise.
- Input pointers are marked `const` where applicable.
- Output pointers must reference valid, writable memory of sufficient size.
- The kernel does not take ownership of pointed-to memory; the caller retains responsibility.

### Thread Safety

- All syscalls are safe to invoke from any thread.
- The kernel provides internal synchronization.
- Concurrent syscalls on the same intent are serialized by the kernel.

---

## Type Definitions

```c
/* --- Identifiers --- */

typedef uint64_t intent_id_t;       // Unique intent identifier
typedef uint64_t context_id_t;      // Context block identifier
typedef uint64_t trace_id_t;        // Trace record identifier
typedef uint64_t memory_id_t;       // Memory entry identifier
typedef uint32_t device_id_t;       // Device identifier
typedef uint32_t capability_t;      // Capability token

/* --- Intent --- */

typedef struct {
    const char*     description;    // Human-readable intent description
    uint32_t        priority;       // 0 (lowest) to 255 (highest)
    uint32_t        flags;          // INTENT_FLAG_* bitmask
    uint64_t        timeout_ms;     // Maximum execution time (0 = no limit)
    capability_t    capability;     // Caller's capability token
} intent_t;

#define INTENT_FLAG_SYNC        0x01    // Synchronous execution
#define INTENT_FLAG_TRACE       0x02    // Enable detailed tracing
#define INTENT_FLAG_IDEMPOTENT  0x04    // Intent is safe to retry
#define INTENT_FLAG_PERSISTENT  0x08    // Persist intent across sessions

/* --- Context --- */

typedef struct {
    intent_id_t     intent_id;      // Associated intent
    const void*     data;           // Context payload
    size_t          data_len;       // Payload length in bytes
    const char*     content_type;   // MIME type of payload
    uint32_t        flags;          // CONTEXT_FLAG_* bitmask
} context_t;

#define CONTEXT_FLAG_IMMUTABLE  0x01    // Context cannot be modified after creation
#define CONTEXT_FLAG_SHARED     0x02    // Context is accessible by other agents
#define CONTEXT_FLAG_EPHEMERAL  0x04    // Context is discarded after intent completes

/* --- Task (Model Inference) --- */

typedef struct {
    intent_id_t     intent_id;      // Associated intent
    device_id_t     device_id;      // Target model device (0 = kernel-selected)
    const void*     input;          // Input payload
    size_t          input_len;      // Input length in bytes
    const char*     input_type;     // MIME type of input
    uint32_t        max_tokens;     // Maximum output tokens (0 = device default)
    uint32_t        flags;          // TASK_FLAG_* bitmask
} task_t;

#define TASK_FLAG_STREAM    0x01    // Stream results as they are produced
#define TASK_FLAG_CACHE     0x02    // Allow cached results

/* --- Result --- */

typedef struct {
    int             status;         // 0 = success, negative = error
    void*           data;           // Output payload (caller-allocated)
    size_t          data_len;       // Available buffer length (in), actual length (out)
    const char*     content_type;   // MIME type of output
    uint64_t        latency_us;     // Execution latency in microseconds
    device_id_t     device_used;    // Device that executed the task
} result_t;

/* --- Memory --- */

typedef struct {
    const char*     namespace;      // Memory namespace
    const char*     key;            // Entry key (null for auto-generated)
    const void*     value;          // Entry value
    size_t          value_len;      // Value length in bytes
    const char*     content_type;   // MIME type of value
    uint32_t        ttl_seconds;    // Time-to-live (0 = permanent)
    uint32_t        flags;          // MEMORY_FLAG_* bitmask
} memory_t;

#define MEMORY_FLAG_SEMANTIC    0x01    // Index for semantic retrieval
#define MEMORY_FLAG_STRUCTURED  0x02    // Index for structured queries
#define MEMORY_FLAG_APPEND      0x04    // Append to existing entry

/* --- Query --- */

typedef struct {
    const char*     namespace;      // Memory namespace to search
    const char*     query_text;     // Semantic query string
    const void*     query_vector;   // Query embedding (optional, null if unused)
    size_t          vector_len;     // Embedding length in bytes
    uint32_t        max_results;    // Maximum results to return
    uint32_t        flags;          // QUERY_FLAG_* bitmask
} query_t;

#define QUERY_FLAG_EXACT        0x01    // Exact key match only
#define QUERY_FLAG_SEMANTIC     0x02    // Semantic similarity search
#define QUERY_FLAG_STRUCTURED   0x04    // Structured query

/* --- Action --- */

typedef struct {
    intent_id_t     intent_id;      // Associated intent
    const char*     action_type;    // Action type identifier
    const void*     payload;        // Action payload
    size_t          payload_len;    // Payload length in bytes
    uint32_t        flags;          // ACTION_FLAG_* bitmask
    capability_t    capability;     // Capability token for the action
} action_t;

#define ACTION_FLAG_DRY_RUN     0x01    // Validate without executing
#define ACTION_FLAG_REVERSIBLE  0x02    // Action can be undone
#define ACTION_FLAG_CONFIRMED   0x04    // User confirmation obtained

/* --- Trace --- */

typedef struct {
    intent_id_t     intent_id;      // Associated intent
    const char*     event_type;     // Trace event type
    const void*     data;           // Trace payload
    size_t          data_len;       // Payload length in bytes
    uint64_t        timestamp_us;   // Microsecond timestamp (0 = kernel-assigned)
} trace_t;

/* --- Device Capabilities --- */

typedef struct {
    device_id_t     device_id;      // Device identifier
    const char*     device_type;    // "model" or "memory"
    const char*     device_name;    // Human-readable name
    const char*     capabilities;   // Comma-separated capability list
    uint32_t        max_input_len;  // Maximum input size in bytes
    uint32_t        max_output_len; // Maximum output size in bytes
    uint32_t        queue_depth;    // Current queue depth
    uint32_t        max_queue;      // Maximum queue capacity
} device_caps_t;

/* --- Policy --- */

typedef struct {
    const char*     action_type;    // Action type to check
    capability_t    capability;     // Caller's capability token
    intent_id_t     intent_id;      // Associated intent
} policy_request_t;

typedef struct {
    int             allowed;        // 1 = allowed, 0 = denied
    const char*     reason;         // Human-readable reason (if denied)
    uint32_t        constraints;    // Constraint bitmask applied
} policy_result_t;
```

---

## Syscall Reference

---

### sys_intent_create

Creates a new reasoning intent. An intent is the top-level unit of cognitive work in Alani.

```c
int sys_intent_create(intent_t* intent);
```

**Parameters:**

| Parameter | Direction | Description |
|-----------|-----------|-------------|
| `intent` | in/out | Intent descriptor. On success, the kernel assigns `intent_id` internally and the intent is registered for scheduling. |

**Returns:**

| Value | Meaning |
|-------|---------|
| `0` | Success. Intent created and queued for execution. |
| `-EPERM` | Caller lacks `CAP_INTENT_CREATE` capability. |
| `-EINVAL` | Invalid intent parameters (null description, invalid priority). |
| `-ENOMEM` | Insufficient kernel memory to allocate intent. |
| `-EQUOTA` | Caller has exceeded maximum concurrent intents. |

**Behavior:**
- The kernel validates the intent descriptor and checks the caller's capability token.
- A unique `intent_id` is assigned and the intent is registered in the kernel's intent table.
- The intent enters the `CREATED` state and is eligible for scheduling.
- If `INTENT_FLAG_SYNC` is set, the call blocks until the intent completes or times out.
- A trace event of type `intent.created` is emitted.

**Required Capability:** `CAP_INTENT_CREATE`

---

### sys_intent_cancel

Cancels a pending or running intent.

```c
int sys_intent_cancel(intent_id_t id);
```

**Parameters:**

| Parameter | Direction | Description |
|-----------|-----------|-------------|
| `id` | in | Identifier of the intent to cancel. |

**Returns:**

| Value | Meaning |
|-------|---------|
| `0` | Intent cancelled successfully. |
| `-EPERM` | Caller does not own this intent. |
| `-ENOENT` | Intent not found. |
| `-EINVAL` | Intent is already in a terminal state. |

**Behavior:**
- Running model inference tasks associated with the intent are aborted.
- The intent transitions to the `CANCELLED` state.
- Associated context and intermediate results are released.
- A trace event of type `intent.cancelled` is emitted.

**Required Capability:** `CAP_INTENT_CANCEL` or ownership of the intent.

---

### sys_intent_status

Queries the current status of an intent.

```c
int sys_intent_status(intent_id_t id, intent_status_t* status);
```

**Parameters:**

| Parameter | Direction | Description |
|-----------|-----------|-------------|
| `id` | in | Intent identifier. |
| `status` | out | Populated with the intent's current state, progress, and metadata. |

**Returns:**

| Value | Meaning |
|-------|---------|
| `0` | Success. |
| `-EPERM` | Caller lacks permission to query this intent. |
| `-ENOENT` | Intent not found. |

**Intent States:**

| State | Description |
|-------|-------------|
| `CREATED` | Intent registered, awaiting scheduling. |
| `SCHEDULED` | Intent assigned to a scheduling slot. |
| `RESOLVING` | Context resolution in progress. |
| `INFERRING` | Model inference in progress. |
| `ACTING` | Action execution in progress. |
| `COMPLETED` | Intent completed successfully. |
| `FAILED` | Intent failed. |
| `CANCELLED` | Intent cancelled by caller or policy. |

---

### sys_context_resolve

Resolves and assembles context for a given intent.

```c
int sys_context_resolve(intent_id_t id, context_t* ctx);
```

**Parameters:**

| Parameter | Direction | Description |
|-----------|-----------|-------------|
| `id` | in | Intent identifier. |
| `ctx` | out | Populated with the resolved context, including assembled data and metadata. |

**Returns:**

| Value | Meaning |
|-------|---------|
| `0` | Context resolved successfully. |
| `-EPERM` | Caller lacks `CAP_CONTEXT_READ` capability. |
| `-ENOENT` | Intent not found. |
| `-ENOMEM` | Insufficient memory for context assembly. |
| `-ETIMEOUT` | Context resolution timed out. |

**Behavior:**
- The kernel's context resolver gathers relevant data from memory devices, prior intents, and attached context blocks.
- Context is assembled according to the intent's requirements and the active policy.
- The assembled context is sized to fit within the target model's input constraints.
- A trace event of type `context.resolved` is emitted.

**Required Capability:** `CAP_CONTEXT_READ`

---

### sys_context_attach

Attaches additional context to an existing intent.

```c
int sys_context_attach(intent_id_t id, const context_t* ctx);
```

**Parameters:**

| Parameter | Direction | Description |
|-----------|-----------|-------------|
| `id` | in | Intent identifier. |
| `ctx` | in | Context block to attach. |

**Returns:**

| Value | Meaning |
|-------|---------|
| `0` | Context attached successfully. |
| `-EPERM` | Caller lacks `CAP_CONTEXT_WRITE` capability. |
| `-ENOENT` | Intent not found. |
| `-EINVAL` | Invalid context data or content type. |
| `-E2BIG` | Total context exceeds maximum size. |

**Required Capability:** `CAP_CONTEXT_WRITE`

---

### sys_model_infer

Submits a task to a model device for inference.

```c
int sys_model_infer(task_t* task, result_t* out);
```

**Parameters:**

| Parameter | Direction | Description |
|-----------|-----------|-------------|
| `task` | in | Task descriptor specifying input, target device, and constraints. |
| `out` | out | Populated with inference results, latency, and device metadata. |

**Returns:**

| Value | Meaning |
|-------|---------|
| `0` | Inference completed successfully. |
| `-EPERM` | Caller lacks `CAP_MODEL_INFER` capability. |
| `-ENODEV` | Specified model device not found or unavailable. |
| `-EBUSY` | Device queue is full. |
| `-ENOMEM` | Insufficient memory for inference. |
| `-ETIMEOUT` | Inference exceeded time limit. |
| `-ENOSPC` | Output buffer too small for result. |
| `-EFAULT` | Device reported an inference error. |

**Behavior:**
- If `device_id` is `0`, the kernel's router selects an appropriate device based on task requirements and scheduling policy.
- The task is queued on the selected device.
- For synchronous intents, the call blocks until inference completes.
- For streaming tasks (`TASK_FLAG_STREAM`), partial results are delivered via a callback mechanism.
- A trace event of type `model.infer` is emitted with device ID and latency.

**Required Capability:** `CAP_MODEL_INFER`

---

### sys_model_capabilities

Queries the capabilities of a model device.

```c
int sys_model_capabilities(device_id_t id, device_caps_t* caps);
```

**Parameters:**

| Parameter | Direction | Description |
|-----------|-----------|-------------|
| `id` | in | Device identifier. |
| `caps` | out | Populated with device capabilities and current status. |

**Returns:**

| Value | Meaning |
|-------|---------|
| `0` | Success. |
| `-EPERM` | Caller lacks `CAP_DEVICE_QUERY` capability. |
| `-ENODEV` | Device not found. |

**Required Capability:** `CAP_DEVICE_QUERY`

---

### sys_memory_query

Queries a memory device for stored information.

```c
int sys_memory_query(query_t* q, result_t* out);
```

**Parameters:**

| Parameter | Direction | Description |
|-----------|-----------|-------------|
| `q` | in | Query descriptor specifying search criteria. |
| `out` | out | Populated with query results. |

**Returns:**

| Value | Meaning |
|-------|---------|
| `0` | Query completed successfully. |
| `-EPERM` | Caller lacks `CAP_MEMORY_READ` capability for the specified namespace. |
| `-ENODEV` | Memory device unavailable. |
| `-EINVAL` | Invalid query parameters. |
| `-ENOSPC` | Output buffer too small for results. |

**Behavior:**
- The kernel routes the query to the appropriate memory device based on namespace and query type.
- For semantic queries (`QUERY_FLAG_SEMANTIC`), the memory device performs similarity search.
- For structured queries (`QUERY_FLAG_STRUCTURED`), the memory device performs graph or relational lookup.
- Results are ranked by relevance and truncated to `max_results`.
- A trace event of type `memory.query` is emitted.

**Required Capability:** `CAP_MEMORY_READ`

---

### sys_memory_write

Writes data to a memory device.

```c
int sys_memory_write(memory_t* mem);
```

**Parameters:**

| Parameter | Direction | Description |
|-----------|-----------|-------------|
| `mem` | in | Memory entry descriptor. |

**Returns:**

| Value | Meaning |
|-------|---------|
| `0` | Write completed successfully. |
| `-EPERM` | Caller lacks `CAP_MEMORY_WRITE` capability for the specified namespace. |
| `-ENODEV` | Memory device unavailable. |
| `-EINVAL` | Invalid memory entry parameters. |
| `-ENOMEM` | Insufficient storage on memory device. |
| `-EQUOTA` | Caller has exceeded memory quota. |

**Behavior:**
- If `MEMORY_FLAG_SEMANTIC` is set, the memory device generates and indexes an embedding.
- If `MEMORY_FLAG_STRUCTURED` is set, the entry is indexed for structured queries.
- If `MEMORY_FLAG_APPEND` is set, the value is appended to an existing entry with the same key.
- If `ttl_seconds` is non-zero, the entry is automatically removed after expiration.
- A trace event of type `memory.write` is emitted.

**Required Capability:** `CAP_MEMORY_WRITE`

---

### sys_memory_delete

Deletes an entry from a memory device.

```c
int sys_memory_delete(const char* namespace, const char* key);
```

**Parameters:**

| Parameter | Direction | Description |
|-----------|-----------|-------------|
| `namespace` | in | Memory namespace. |
| `key` | in | Entry key to delete. |

**Returns:**

| Value | Meaning |
|-------|---------|
| `0` | Entry deleted successfully. |
| `-EPERM` | Caller lacks `CAP_MEMORY_DELETE` capability. |
| `-ENOENT` | Entry not found. |

**Required Capability:** `CAP_MEMORY_DELETE`

---

### sys_action_execute

Executes a decided action.

```c
int sys_action_execute(action_t* action);
```

**Parameters:**

| Parameter | Direction | Description |
|-----------|-----------|-------------|
| `action` | in | Action descriptor specifying type, payload, and capabilities. |

**Returns:**

| Value | Meaning |
|-------|---------|
| `0` | Action executed successfully. |
| `-EPERM` | Caller lacks capability for this action type. |
| `-EINVAL` | Invalid action parameters. |
| `-EPOLICY` | Action denied by policy engine. |
| `-EFAULT` | Action execution failed. |

**Behavior:**
- The kernel's policy engine validates the action against active policies before execution.
- If `ACTION_FLAG_DRY_RUN` is set, the action is validated but not executed.
- If `ACTION_FLAG_CONFIRMED` is not set and the policy requires confirmation, the syscall returns `-EPOLICY` with a reason indicating confirmation is needed.
- A trace event of type `action.executed` or `action.denied` is emitted.

**Required Capability:** Action-type-specific capability.

---

### sys_action_validate

Validates an action against policies without executing it.

```c
int sys_action_validate(action_t* action, policy_result_t* result);
```

**Parameters:**

| Parameter | Direction | Description |
|-----------|-----------|-------------|
| `action` | in | Action to validate. |
| `result` | out | Policy evaluation result. |

**Returns:**

| Value | Meaning |
|-------|---------|
| `0` | Validation completed (check `result.allowed`). |
| `-EPERM` | Caller lacks `CAP_POLICY_QUERY` capability. |
| `-EINVAL` | Invalid action parameters. |

**Required Capability:** `CAP_POLICY_QUERY`

---

### sys_trace_emit

Emits a trace event to the execution trace log.

```c
int sys_trace_emit(trace_t* trace);
```

**Parameters:**

| Parameter | Direction | Description |
|-----------|-----------|-------------|
| `trace` | in | Trace event descriptor. |

**Returns:**

| Value | Meaning |
|-------|---------|
| `0` | Trace event recorded. |
| `-EPERM` | Caller lacks `CAP_TRACE_EMIT` capability. |
| `-EINVAL` | Invalid trace event parameters. |
| `-ENOMEM` | Trace buffer full. |

**Behavior:**
- If `timestamp_us` is `0`, the kernel assigns the current timestamp.
- Trace events are written to the execution trace memory tier.
- Trace events are not guaranteed to be delivered in real-time; they are batched for efficiency.

**Required Capability:** `CAP_TRACE_EMIT`

---

### sys_trace_query

Queries the execution trace log.

```c
int sys_trace_query(intent_id_t id, trace_t* buffer, size_t* count);
```

**Parameters:**

| Parameter | Direction | Description |
|-----------|-----------|-------------|
| `id` | in | Intent identifier to query traces for. |
| `buffer` | out | Array of trace events (caller-allocated). |
| `count` | in/out | On input, buffer capacity. On output, number of events returned. |

**Returns:**

| Value | Meaning |
|-------|---------|
| `0` | Query completed. |
| `-EPERM` | Caller lacks `CAP_TRACE_READ` capability. |
| `-ENOENT` | Intent not found or no traces available. |
| `-ENOSPC` | Buffer too small (partial results returned). |

**Required Capability:** `CAP_TRACE_READ`

---

### sys_policy_check

Checks whether an action is permitted under current policies.

```c
int sys_policy_check(policy_request_t* req, policy_result_t* result);
```

**Parameters:**

| Parameter | Direction | Description |
|-----------|-----------|-------------|
| `req` | in | Policy check request. |
| `result` | out | Policy evaluation result. |

**Returns:**

| Value | Meaning |
|-------|---------|
| `0` | Policy check completed. |
| `-EPERM` | Caller lacks `CAP_POLICY_QUERY` capability. |
| `-EINVAL` | Invalid request parameters. |

**Required Capability:** `CAP_POLICY_QUERY`

---

## Error Codes

| Code | Name | Description |
|------|------|-------------|
| `0` | `OK` | Success |
| `-1` | `-EPERM` | Permission denied |
| `-2` | `-ENOENT` | Entry or resource not found |
| `-3` | `-EINVAL` | Invalid parameters |
| `-4` | `-ENOMEM` | Insufficient memory |
| `-5` | `-ENODEV` | Device not found or unavailable |
| `-6` | `-EBUSY` | Resource busy or queue full |
| `-7` | `-ETIMEOUT` | Operation timed out |
| `-8` | `-ENOSPC` | Insufficient buffer or storage space |
| `-9` | `-EFAULT` | Device or execution fault |
| `-10` | `-EPOLICY` | Denied by policy engine |
| `-11` | `-EQUOTA` | Quota exceeded |
| `-12` | `-E2BIG` | Input exceeds maximum size |

---

## Permissions

All syscalls are gated by capability tokens. Capabilities are assigned to agents at creation time and can be refined by policy.

### Capability Table

| Capability | Description | Syscalls |
|------------|-------------|----------|
| `CAP_INTENT_CREATE` | Create new intents | `sys_intent_create` |
| `CAP_INTENT_CANCEL` | Cancel intents | `sys_intent_cancel` |
| `CAP_CONTEXT_READ` | Read resolved context | `sys_context_resolve` |
| `CAP_CONTEXT_WRITE` | Attach context to intents | `sys_context_attach` |
| `CAP_MODEL_INFER` | Invoke model inference | `sys_model_infer` |
| `CAP_DEVICE_QUERY` | Query device capabilities | `sys_model_capabilities` |
| `CAP_MEMORY_READ` | Query memory devices | `sys_memory_query` |
| `CAP_MEMORY_WRITE` | Write to memory devices | `sys_memory_write` |
| `CAP_MEMORY_DELETE` | Delete memory entries | `sys_memory_delete` |
| `CAP_TRACE_EMIT` | Emit trace events | `sys_trace_emit` |
| `CAP_TRACE_READ` | Query trace logs | `sys_trace_query` |
| `CAP_POLICY_QUERY` | Check policy permissions | `sys_policy_check`, `sys_action_validate` |

---

## Execution Semantics

### Ordering Guarantees

- Syscalls within a single intent are serialized by the kernel.
- Syscalls across different intents may execute concurrently.
- Memory writes are visible to subsequent reads within the same intent.
- Memory writes across intents follow eventual consistency unless `MEMORY_FLAG_SEMANTIC` forces synchronous indexing.

### Blocking Behavior

- `sys_model_infer` blocks until inference completes (for synchronous intents).
- `sys_memory_query` blocks until results are available.
- `sys_intent_create` with `INTENT_FLAG_SYNC` blocks until the intent completes.
- All other syscalls are non-blocking under normal conditions.

### Failure Semantics

- Failed syscalls do not modify kernel state.
- Partially completed intents can be retried if `INTENT_FLAG_IDEMPOTENT` is set.
- The kernel does not automatically retry failed operations.

---

## Tracing Requirements

Every syscall implementation must emit a trace event containing:

| Field | Description |
|-------|-------------|
| `intent_id` | Associated intent (if applicable) |
| `syscall` | Syscall name |
| `timestamp_us` | Kernel timestamp |
| `caller` | Agent identifier |
| `duration_us` | Execution duration |
| `result` | Return code |
| `device_id` | Device used (if applicable) |

Trace events are written to the **Execution Trace Memory** tier and are queryable via `sys_trace_query`.

---

## Examples

### Basic Intent Lifecycle

```c
// 1. Create an intent
intent_t intent = {
    .description = "Summarize recent research on cognitive architectures",
    .priority = 128,
    .flags = INTENT_FLAG_TRACE,
    .timeout_ms = 30000,
    .capability = agent_cap
};

int ret = sys_intent_create(&intent);
if (ret != 0) handle_error(ret);

// 2. Resolve context
context_t ctx;
ret = sys_context_resolve(intent_id, &ctx);
if (ret != 0) handle_error(ret);

// 3. Run inference
task_t task = {
    .intent_id = intent_id,
    .device_id = 0,  // kernel-selected
    .input = ctx.data,
    .input_len = ctx.data_len,
    .input_type = "text/plain",
    .max_tokens = 2048,
    .flags = 0
};

result_t result;
result.data = output_buffer;
result.data_len = sizeof(output_buffer);

ret = sys_model_infer(&task, &result);
if (ret != 0) handle_error(ret);

// 4. Persist result
memory_t mem = {
    .namespace = "research",
    .key = "cognitive-arch-summary",
    .value = result.data,
    .value_len = result.data_len,
    .content_type = "text/plain",
    .ttl_seconds = 0,  // permanent
    .flags = MEMORY_FLAG_SEMANTIC
};

ret = sys_memory_write(&mem);
if (ret != 0) handle_error(ret);
```

### Policy-Gated Action

```c
// Validate before executing
action_t action = {
    .intent_id = intent_id,
    .action_type = "email.send",
    .payload = email_payload,
    .payload_len = email_len,
    .flags = 0,
    .capability = agent_cap
};

policy_result_t policy;
ret = sys_action_validate(&action, &policy);

if (policy.allowed) {
    action.flags |= ACTION_FLAG_CONFIRMED;
    ret = sys_action_execute(&action);
} else {
    // Handle denial: policy.reason contains explanation
    log_denial(policy.reason);
}
```

### Device Introspection

```c
device_caps_t caps;
ret = sys_model_capabilities(0, &caps);  // device 0

printf("Device: %s\n", caps.device_name);
printf("Type: %s\n", caps.device_type);
printf("Capabilities: %s\n", caps.capabilities);
printf("Queue: %u / %u\n", caps.queue_depth, caps.max_queue);
```

---

## Revision History

| Version | Date | Changes |
|---------|------|---------|
| 0.1.0-draft | April 2026 | Initial specification |

---

<p align="center"><em>Intelligence is no longer an application feature — it is an operating system primitive.</em></p>
```