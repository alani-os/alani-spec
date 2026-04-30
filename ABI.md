First-pass Alani Kernel ABI in Rust: syscall contracts, cognitive device traits, IPC types, and kernel/user boundary structures.

#![no_std]
pub mod abi {
    pub type Pid = u64;
    pub type Tid = u64;
    pub type IntentId = u64;
    pub type TaskId = u64;
    pub type MemoryId = u64;
    pub type DeviceId = u64;
    pub type TraceId = u64;
    pub type CapabilityMask = u64;
    #[repr(u32)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AbiStatus {
        Ok = 0,
        InvalidArg = 1,
        PermissionDenied = 2,
        NotFound = 3,
        Busy = 4,
        Timeout = 5,
        DeviceError = 6,
        InternalError = 7,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct AbiResult<T> {
        pub status: AbiStatus,
        pub value: T,
    }
}

1. Core Cognitive Types

pub mod cognition {
    use super::abi::*;
    pub const MAX_OBJECTIVE_LEN: usize = 512;
    pub const MAX_CONTEXT_REFS: usize = 32;
    pub const MAX_PAYLOAD_LEN: usize = 4096;
    #[repr(u32)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IntentKind {
        Query = 0,
        Task = 1,
        Action = 2,
        AgentRequest = 3,
        SystemSignal = 4,
    }
    #[repr(u32)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Priority {
        Low = 0,
        Normal = 1,
        High = 2,
        Critical = 3,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct IntentDescriptor {
        pub kind: IntentKind,
        pub priority: Priority,
        pub caller_pid: Pid,
        pub capability_mask: CapabilityMask,
        pub objective_ptr: *const u8,
        pub objective_len: usize,
        pub payload_ptr: *const u8,
        pub payload_len: usize,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct IntentHandle {
        pub id: IntentId,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct ContextDescriptor {
        pub intent_id: IntentId,
        pub memory_refs_ptr: *const MemoryId,
        pub memory_refs_len: usize,
        pub payload_ptr: *mut u8,
        pub payload_capacity: usize,
        pub payload_len_out: *mut usize,
    }
    #[repr(u32)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CognitiveTaskKind {
        Planning = 0,
        Inference = 1,
        Evaluation = 2,
        Summarization = 3,
        ToolSelection = 4,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct CognitiveTask {
        pub kind: CognitiveTaskKind,
        pub intent_id: IntentId,
        pub input_ptr: *const u8,
        pub input_len: usize,
        pub required_capabilities: CapabilityMask,
        pub timeout_ms: u64,
        pub max_output_len: usize,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct CognitiveResult {
        pub task_id: TaskId,
        pub output_ptr: *mut u8,
        pub output_capacity: usize,
        pub output_len_out: *mut usize,
        pub confidence_out: *mut u32, // fixed-point 0..10000
    }
}

2. Syscall ABI

The kernel exposes intelligence through syscall numbers. Userspace never directly controls model devices or memory engines.

pub mod syscall {
    use super::abi::*;
    use super::cognition::*;
    #[repr(usize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SyscallNumber {
        IntentCreate = 0x1000,
        ContextResolve = 0x1001,
        ModelInfer = 0x1002,
        MemoryQuery = 0x1003,
        MemoryWrite = 0x1004,
        ActionExecute = 0x1005,
        TraceEmit = 0x1006,
    }
    pub type SyscallRet = AbiStatus;
    pub trait CognitiveSyscalls {
        unsafe fn sys_intent_create(
            desc: *const IntentDescriptor,
            out: *mut IntentHandle,
        ) -> SyscallRet;
        unsafe fn sys_context_resolve(
            desc: *mut ContextDescriptor,
        ) -> SyscallRet;
        unsafe fn sys_model_infer(
            task: *const CognitiveTask,
            result: *mut CognitiveResult,
        ) -> SyscallRet;
        unsafe fn sys_memory_query(
            query: *const MemoryQuery,
            result: *mut MemoryQueryResult,
        ) -> SyscallRet;
        unsafe fn sys_memory_write(
            write: *const MemoryWrite,
            out_id: *mut MemoryId,
        ) -> SyscallRet;
        unsafe fn sys_trace_emit(
            trace: *const TraceEvent,
        ) -> SyscallRet;
    }
}

3. Memory ABI

pub mod memory {
    use super::abi::*;
    #[repr(u32)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MemoryKind {
        Episodic = 0,
        Semantic = 1,
        Procedural = 2,
        Policy = 3,
        Trace = 4,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct MemoryQuery {
        pub caller_pid: Pid,
        pub kind: MemoryKind,
        pub query_ptr: *const u8,
        pub query_len: usize,
        pub required_capabilities: CapabilityMask,
        pub limit: usize,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct MemoryQueryResult {
        pub result_ptr: *mut u8,
        pub result_capacity: usize,
        pub result_len_out: *mut usize,
        pub match_count_out: *mut usize,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct MemoryWrite {
        pub caller_pid: Pid,
        pub kind: MemoryKind,
        pub content_ptr: *const u8,
        pub content_len: usize,
        pub capability_mask: CapabilityMask,
        pub confidence: u32, // fixed-point 0..10000
    }
}

4. Cognitive Device ABI

Models, memory engines, and tools are kernel-managed devices.

pub mod device {
    use super::abi::*;
    use super::cognition::*;
    use super::memory::*;
    #[repr(u32)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DeviceKind {
        Model = 0,
        Memory = 1,
        Tool = 2,
        Policy = 3,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct DeviceDescriptor {
        pub id: DeviceId,
        pub kind: DeviceKind,
        pub capability_mask: CapabilityMask,
        pub max_input_len: usize,
        pub max_output_len: usize,
        pub queue_depth: usize,
    }
    pub trait KernelDevice {
        fn descriptor(&self) -> DeviceDescriptor;
        fn open(&mut self, caller_pid: Pid) -> AbiStatus;
        fn close(&mut self, caller_pid: Pid) -> AbiStatus;
    }
    pub trait ModelDevice: KernelDevice {
        fn infer(
            &mut self,
            task: &CognitiveTask,
            output: &mut [u8],
        ) -> Result<ModelInferenceMeta, AbiStatus>;
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct ModelInferenceMeta {
        pub output_len: usize,
        pub confidence: u32,
        pub tokens_in: u32,
        pub tokens_out: u32,
        pub latency_ms: u64,
    }
    pub trait MemoryDevice: KernelDevice {
        fn query(
            &mut self,
            query: &MemoryQuery,
            output: &mut [u8],
        ) -> Result<MemoryQueryMeta, AbiStatus>;
        fn write(
            &mut self,
            write: &MemoryWrite,
        ) -> Result<MemoryId, AbiStatus>;
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct MemoryQueryMeta {
        pub output_len: usize,
        pub match_count: usize,
        pub latency_ms: u64,
    }
}

5. Policy ABI

pub mod policy {
    use super::abi::*;
    use super::device::*;
    #[repr(u32)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PolicyDecision {
        Allow = 0,
        Deny = 1,
        RequireEscalation = 2,
        RequireAudit = 3,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct PolicyCheck {
        pub caller_pid: Pid,
        pub requested_capabilities: CapabilityMask,
        pub target_device: DeviceId,
        pub operation: u32,
    }
    pub trait PolicyEngine {
        fn check(&self, check: &PolicyCheck) -> PolicyDecision;
    }
}

6. Trace ABI

pub mod trace {
    use super::abi::*;
    #[repr(u32)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TraceKind {
        IntentCreated = 0,
        ContextResolved = 1,
        ModelInferenceStarted = 2,
        ModelInferenceCompleted = 3,
        MemoryQueried = 4,
        MemoryWritten = 5,
        ActionExecuted = 6,
        PolicyDenied = 7,
        Error = 8,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct TraceEvent {
        pub kind: TraceKind,
        pub caller_pid: Pid,
        pub intent_id: IntentId,
        pub task_id: TaskId,
        pub timestamp_ns: u64,
        pub payload_ptr: *const u8,
        pub payload_len: usize,
    }
}

7. Kernel-Side Syscall Dispatcher

pub mod kernel {
    use super::abi::*;
    use super::cognition::*;
    use super::memory::*;
    use super::trace::*;
    pub struct CognitiveKernel {
        pub next_intent_id: IntentId,
        pub next_task_id: TaskId,
    }
    impl CognitiveKernel {
        pub const fn new() -> Self {
            Self {
                next_intent_id: 1,
                next_task_id: 1,
            }
        }
        pub unsafe fn syscall_dispatch(
            &mut self,
            num: usize,
            arg0: usize,
            arg1: usize,
            _arg2: usize,
            _arg3: usize,
        ) -> AbiStatus {
            match num {
                0x1000 => {
                    let desc = arg0 as *const IntentDescriptor;
                    let out = arg1 as *mut IntentHandle;
                    self.handle_intent_create(desc, out)
                }
                0x1001 => {
                    let desc = arg0 as *mut ContextDescriptor;
                    self.handle_context_resolve(desc)
                }
                0x1002 => {
                    let task = arg0 as *const CognitiveTask;
                    let result = arg1 as *mut CognitiveResult;
                    self.handle_model_infer(task, result)
                }
                0x1003 => {
                    let query = arg0 as *const MemoryQuery;
                    let result = arg1 as *mut MemoryQueryResult;
                    self.handle_memory_query(query, result)
                }
                0x1004 => {
                    let write = arg0 as *const MemoryWrite;
                    let out = arg1 as *mut MemoryId;
                    self.handle_memory_write(write, out)
                }
                0x1006 => {
                    let trace = arg0 as *const TraceEvent;
                    self.handle_trace_emit(trace)
                }
                _ => AbiStatus::InvalidArg,
            }
        }
        unsafe fn handle_intent_create(
            &mut self,
            desc: *const IntentDescriptor,
            out: *mut IntentHandle,
        ) -> AbiStatus {
            if desc.is_null() || out.is_null() {
                return AbiStatus::InvalidArg;
            }
            let intent_id = self.next_intent_id;
            self.next_intent_id += 1;
            (*out).id = intent_id;
            AbiStatus::Ok
        }
        unsafe fn handle_context_resolve(
            &mut self,
            desc: *mut ContextDescriptor,
        ) -> AbiStatus {
            if desc.is_null() {
                return AbiStatus::InvalidArg;
            }
            AbiStatus::Ok
        }
        unsafe fn handle_model_infer(
            &mut self,
            task: *const CognitiveTask,
            result: *mut CognitiveResult,
        ) -> AbiStatus {
            if task.is_null() || result.is_null() {
                return AbiStatus::InvalidArg;
            }
            AbiStatus::Ok
        }
        unsafe fn handle_memory_query(
            &mut self,
            query: *const MemoryQuery,
            result: *mut MemoryQueryResult,
        ) -> AbiStatus {
            if query.is_null() || result.is_null() {
                return AbiStatus::InvalidArg;
            }
            AbiStatus::Ok
        }
        unsafe fn handle_memory_write(
            &mut self,
            write: *const MemoryWrite,
            out: *mut MemoryId,
        ) -> AbiStatus {
            if write.is_null() || out.is_null() {
                return AbiStatus::InvalidArg;
            }
            *out = 1;
            AbiStatus::Ok
        }
        unsafe fn handle_trace_emit(
            &mut self,
            trace: *const TraceEvent,
        ) -> AbiStatus {
            if trace.is_null() {
                return AbiStatus::InvalidArg;
            }
            AbiStatus::Ok
        }
    }
}

8. Proposed Crate Layout

alani/
  Cargo.toml
  crates/
    alani-abi/
      src/
        lib.rs
        syscall.rs
        cognition.rs
        memory.rs
        device.rs
        policy.rs
        trace.rs
    alani-kernel/
      src/
        main.rs
        arch/
        boot/
        memory/
        sched/
        ipc/
        cognition/
          mod.rs
          intent.rs
          context.rs
          router.rs
          planner.rs
          trace.rs
    alani-device-model/
      src/
        lib.rs
        model_device.rs
        queue.rs
    alani-device-memory/
      src/
        lib.rs
        vector.rs
        graph.rs
    alani-userspace/
      src/
        lib.rs
        syscall_client.rs
        agent.rs
        shell.rs

9. ABI Rule Set

The ABI should obey these rules:

1. All kernel boundary structs use #[repr(C)].
2. No heap-owned Rust types cross the boundary.
3. No String, Vec, Box, HashMap, or trait objects cross the ABI.
4. Userspace passes raw pointers and lengths.
5. Kernel validates every pointer and capability before use.
6. Models do not run in kernel space.
7. The kernel schedules model access as a cognitive device operation.
8. Traces are mandatory for all cognitive syscalls.
9. Memory writes require explicit capability grants.
10. ABI versioning must be explicit.

Add this:

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AbiVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}
pub const ALANI_ABI_VERSION: AbiVersion = AbiVersion {
    major: 0,
    minor: 1,
    patch: 0,
};

10. Core Design Decision

The most important boundary is this:

Kernel owns:
  - intent lifecycle
  - capability checks
  - routing policy
  - scheduling
  - trace integrity
Userspace owns:
  - model execution
  - agents
  - workflows
  - external tools
  - large memory stores

